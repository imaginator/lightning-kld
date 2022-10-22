use std::fs::File;
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

const NETWORK: &str = "regtest";

#[allow(dead_code)]
pub struct BitcoinManager {
    process: Option<Child>,
    data_dir: String,
    pub p2p_port: u16,
    pub rpc_port: u16,
    pub network: String,
}

impl BitcoinManager {
    pub fn start(&mut self) {
        if self.process.is_none() {
            self.clean();
            let child = Command::new("bitcoind")
                .arg("-daemon")
                .arg("-server")
                .arg("-noconnect")
                .arg(format!("-chain={}", NETWORK))
                .arg(format!("-datadir={}", &self.data_dir))
                .arg(format!("-port={}", &self.p2p_port.to_string()))
                .arg(format!("-rpcport={}", &self.rpc_port.to_string()))
                .stdout(Stdio::null())
                .spawn()
                .unwrap();
            sleep(Duration::from_secs(1));
            self.process = Some(child)
        }
    }

    pub fn kill(&mut self) {
        if let Some(mut process) = self.process.take() {
            process.kill().unwrap_or_default();
            process.wait().unwrap();
            self.process = None
        }
        if let Ok(mut pid_file) = File::open(format!("{}/bitcoind.pid", self.data_dir())) {
            let mut pid = String::new();
            pid_file.read_to_string(&mut pid).unwrap();
            pid = pid.trim().to_string();
            Command::new("kill")
                .arg(&pid)
                .output()
                .expect("failed to terminate bitcoind");
        }
    }

    pub fn cookie_path(&self) -> String {
        format!("{}/.cookie", self.data_dir())
    }

    pub fn test_bitcoin(test_name: &str, node_index: u16) -> BitcoinManager {
        let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let test_number = std::fs::read_dir(format!("{}/tests", dir))
            .unwrap()
            .position(|f| f.unwrap().file_name().to_str().unwrap() == format!("{}.rs", test_name))
            .unwrap() as u16;

        let p2p_port = 20000u16 + (test_number * 1000u16) + node_index * 10;
        let rpc_port = 30000u16 + (test_number * 1000u16) + node_index * 10;
        let data_dir = format!("{}/output/{}/bitcoind_{}", dir, test_name, node_index);

        BitcoinManager {
            process: None,
            data_dir,
            p2p_port,
            rpc_port,
            network: NETWORK.to_string(),
        }
    }

    fn clean(&self) {
        if let Ok(mut file) = File::open(format!("{}/bitcoind.pid", &self.data_dir())) {
            let mut pid = String::new();
            file.read_to_string(&mut pid).unwrap();
            pid = pid.trim().to_string();
            Command::new("kill").arg("-9").arg(pid).output().unwrap();
        }
        std::fs::remove_dir_all(&self.data_dir()).unwrap_or_default();
        std::fs::create_dir_all(&self.data_dir()).unwrap();
    }

    fn data_dir(&self) -> String {
        if NETWORK == "mainnet" {
            self.data_dir.clone()
        } else {
            format!("{}/{}", self.data_dir, NETWORK)
        }
    }
}

impl Drop for BitcoinManager {
    fn drop(&mut self) {
        self.kill()
    }
}

#[macro_export]
macro_rules! bitcoin {
    () => {
        test_utils::bitcoin_manager::BitcoinManager::test_bitcoin(env!("CARGO_CRATE_NAME"), 0)
    };
    ($n:literal) => {
        test_utils::bitcoin_manager::BitcoinManager::test_bitcoin(env!("CARGO_CRATE_NAME"), $n)
    };
}