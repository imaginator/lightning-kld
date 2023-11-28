CREATE TABLE wallet_version (
    version INTEGER
);

INSERT INTO wallet_version VALUES (1);

CREATE TABLE wallet_script_pubkeys (
    keychain TEXT,
    child INTEGER,
    script BYTEA
);

CREATE INDEX idx_wallet_script_pubkeys_keychain_child ON wallet_script_pubkeys (keychain, child);
CREATE INDEX idx_wallet_script_pubkeys_script ON wallet_script_pubkeys (script);

CREATE TABLE wallet_utxos (
    value INTEGER,
    keychain TEXT,
    vout INTEGER,
    txid BYTEA,
    script BYTEA,
    is_spent BOOLEAN,
    PRIMARY KEY (txid, vout)
);

CREATE TABLE wallet_transactions (
    txid BYTEA,
    raw_tx BYTEA
);

CREATE TABLE wallet_transaction_details (
    txid BYTEA,
    timestamp INT,
    received INT,
    sent INT,
    fee INT,
    height INT
);

CREATE INDEX idx_wallet_transaction_details_txid ON wallet_transaction_details (txid);

CREATE TABLE wallet_last_derivation_indices (
    keychain TEXT PRIMARY KEY,
    value INT
);

CREATE TABLE wallet_checksums (
    keychain TEXT,
    checksum BYTEA
);

CREATE INDEX idx_wallet_checksums_keychain ON wallet_checksums (keychain);

CREATE TABLE wallet_sync_time (
    id INT PRIMARY KEY,
    height INT,
    timestamp INT
);
