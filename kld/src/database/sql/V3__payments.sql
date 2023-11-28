CREATE TYPE payment_status AS ENUM ('pending', 'succeeded', 'recipient_rejected', 'user_abandoned', 'retries_exhausted', 'expired', 'route_not_found', 'error');

CREATE TYPE payment_direction AS ENUM ('inbound', 'outbound');

CREATE TABLE payments (
    id              BYTEA NOT NULL,
    hash            BYTEA NOT NULL,
    preimage        BYTEA,
    secret          BYTEA,
    status          payment_status NOT NULL,
    amount          INT NOT NULL,
    fee             INT,
    metadata        BYTEA,
    direction       payment_direction NOT NULL,
    channel_id      BYTEA,
    counterparty_id BYTEA,
    timestamp       TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY ( id )
);

CREATE INDEX idx_payments_hash ON payments (hash);
