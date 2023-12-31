use std::ops::Deref;
pub mod bitcoin_manager;
pub mod cockroach_manager;
pub mod electrs_manager;
pub mod kld_manager;
pub mod ports;

use anyhow::Result;
use kld::database::DurableConnection;
use kld::settings::Settings;
use std::{fs::File, io::Read, path::Path};

use bitcoin::secp256k1::{PublicKey, SecretKey};
pub use bitcoin_manager::BitcoinManager;
pub use cockroach_manager::CockroachManager;
pub use electrs_manager::ElectrsManager;
pub use kld_manager::KldManager;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::{Certificate, Client};

// https://mempool.space/tx/b9deb5e0aaf6d80fe156e64b3a339b7d5f853bcf9993a8183e1eec4b6f26cf86
pub const TEST_TX_ID: &str = "b9deb5e0aaf6d80fe156e64b3a339b7d5f853bcf9993a8183e1eec4b6f26cf86";

pub const TEST_TX: &str = "02000000000101f127bd21d5188f21f623bca61cdefc443d2f8b1ffc208945321a1af0096b4bb11400000000fdffffff01220200000000000022512019bd1296ec4e4f75f75f4cd409e19811a59e0e9877c6135dc310210916dc0072034021e1157d9aa76cbc86602656c5d504957c40694182bb18bff749f6007950aab07c0915f750ea5782a69b206379387bfae97f9ec8954b1c5613e95082c376dba18320117f692257b2331233b5705ce9c682be8719ff1b2b64cbca290bd6faeb54423eac0663c05b218801750063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800357b2270223a226272632d3230222c226f70223a226d696e74222c227469636b223a22574f4c56222c22616d74223a2231303030227d6821c1117f692257b2331233b5705ce9c682be8719ff1b2b64cbca290bd6faeb54423e00000000";

pub const TEST_ADDRESS: &str = "2N4eQYCbKUHCCTUjBJeHcJp9ok6J2GZsTDt";

pub const TEST_SHORT_CHANNEL_ID: u64 = 0x0102030405060708;

pub const TEST_ALIAS: &str = "test node";

pub const TEST_WPKH: &str = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";

// https://mempool.space/block/0000000000000000000590fc0f3eba193a278534220b2b37e9849e1a770ca959
pub const TEST_BLOCK_HASH: &str =
    "0000000000000000000590fc0f3eba193a278534220b2b37e9849e1a770ca959";

pub const TEST_PRIVATE_KEY: [u8; 32] = [
    0xe1, 0x26, 0xf6, 0x8f, 0x7e, 0xaf, 0xcc, 0x8b, 0x74, 0xf5, 0x4d, 0x26, 0x9f, 0xe2, 0x06, 0xbe,
    0x71, 0x50, 0x00, 0xf9, 0x4d, 0xac, 0x06, 0x7d, 0x1c, 0x04, 0xa8, 0xca, 0x3b, 0x2d, 0xb7, 0x34,
];

// Public key of the above private key.
pub const TEST_PUBLIC_KEY: &str =
    "03e7156ae33b0a208d0744199163177e909e80176e55d97a2f221ede0f934dd9ad";

pub fn test_settings(tmp_dir: &tempfile::TempDir, name: &str) -> kld::settings::Settings {
    let mut settings = kld::settings::Settings::default();
    settings.certs_dir = format!("{}/certs", env!("CARGO_MANIFEST_DIR"));
    settings.database_ca_cert_path =
        format!("{}/certs/cockroach/ca.crt", env!("CARGO_MANIFEST_DIR"));
    settings.database_client_cert_path = format!(
        "{}/certs/cockroach/client.root.crt",
        env!("CARGO_MANIFEST_DIR")
    );
    settings.database_client_key_path = format!(
        "{}/certs/cockroach/client.root.key",
        env!("CARGO_MANIFEST_DIR")
    );
    settings.database_name = name.to_string();
    settings.node_id = name.to_string();
    settings.data_dir = format!("{}/test_{name}", tmp_dir.path().display());
    settings.mnemonic_path = format!("{}/mnemonic", settings.data_dir);
    std::fs::create_dir_all(&settings.data_dir).unwrap();
    settings
}

pub fn random_public_key() -> PublicKey {
    let rand: [u8; 32] = rand::random();
    let secp_ctx = bitcoin::secp256k1::Secp256k1::new();
    let secret_key = &SecretKey::from_slice(&rand).unwrap();
    PublicKey::from_secret_key(&secp_ctx, secret_key)
}

pub fn https_client(macaroon: Option<Vec<u8>>) -> Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(macaroon) = macaroon {
        headers.insert("macaroon", HeaderValue::from_bytes(&macaroon)?);
    }

    // Rustls does not support IP addresses (hostnames only) so we need to use native tls (openssl). Also turn off SNI as this requires host names as well.
    Ok(reqwest::ClientBuilder::new()
        .tls_sni(false)
        .add_root_certificate(test_cert())
        .use_native_tls()
        .default_headers(headers)
        .build()?)
}

fn test_cert() -> Certificate {
    let mut buf = Vec::new();
    File::open(format!("{}/certs/kld.crt", env!("CARGO_MANIFEST_DIR")))
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    Certificate::from_pem(&buf).unwrap()
}

#[macro_export]
macro_rules! poll {
    ($count: expr, $func: expr) => {
        let mut ct = 0;
        while ct < $count {
            if $func {
                break;
            };
            tokio::time::sleep(std::time::Duration::from_secs(1 + 3 * ct)).await;
            ct += 1;
        }
        if ct == $count {
            anyhow::bail!("Fail {ct:} times on polling for result");
        }
    };
}

pub mod fake_fs {
    use std::{io, path::Path};

    pub fn read<P: AsRef<Path>>(_path: P) -> io::Result<Vec<u8>> {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
    pub fn read_to_string<P: AsRef<Path>>(_path: P) -> io::Result<String> {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
    pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(_path: P, _contents: C) -> io::Result<()> {
        Ok(())
    }
    pub fn create_dir_all<P: AsRef<Path>>(_path: P) -> io::Result<()> {
        Ok(())
    }
}

/// A wrapper of tempfile::TempDir to keep temp files if env `KEEP_TEST_ARTIFACTS_IN` is set
pub struct TempDir {
    inner: Option<tempfile::TempDir>,
}

impl TempDir {
    pub fn new() -> std::io::Result<Self> {
        if let Ok(path) = std::env::var("KEEP_TEST_ARTIFACTS_IN") {
            let dir = Path::new(&path);
            if !dir.is_dir() {
                std::fs::create_dir(dir)?;
            }
            Ok(Self {
                inner: Some(tempfile::TempDir::new_in(dir)?),
            })
        } else {
            Ok(Self {
                inner: Some(tempfile::TempDir::new()?),
            })
        }
    }
}

impl Deref for TempDir {
    type Target = tempfile::TempDir;

    fn deref(&self) -> &Self::Target {
        self.inner
            .as_ref()
            .expect("TmpDir should be create with `new()`")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if std::env::var("KEEP_TEST_ARTIFACTS_IN").is_ok() {
            if let Some(inner) = self.inner.take() {
                let path = inner.into_path();
                println!("test artifacts keep in {:?}", path);
            }
        }
    }
}

pub async fn init_db_test_context(
    temp_dir: &tempfile::TempDir,
) -> Result<(Settings, CockroachManager, DurableConnection)> {
    let mut settings = test_settings(temp_dir, "integration");
    let cockroach = CockroachManager::builder(temp_dir, &mut settings)
        .await?
        .build()
        .await?;
    cockroach_manager::create_database(&settings).await;
    let durable_connection = DurableConnection::new_migrate(settings.clone().into()).await;
    Ok((settings, cockroach, durable_connection))
}
