CREATE TABLE invoices (
    payment_hash    BYTEA NOT NULL,
    label           VARCHAR,
    expiry          INT,
    payee_pub_key   BYTEA,
    amount          INT,
    bolt11          VARCHAR NOT NULL,
    timestamp       TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY ( payment_hash )
);

CREATE INDEX idx_invoices_label ON invoices (label);

ALTER TABLE payments ADD COLUMN label VARCHAR;
