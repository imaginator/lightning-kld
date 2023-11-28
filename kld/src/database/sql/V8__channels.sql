CREATE TABLE channels (
    id                   BYTEA NOT NULL,
    scid                 INT NOT NULL,
    user_channel_id      INT NOT NULL,
    counterparty         BYTEA NOT NULL,
    funding_txo          BYTEA NOT NULL,
    is_public            BOOLEAN NOT NULL,
    is_outbound          BOOLEAN NOT NULL,
    value                INT NOT NULL,
    type_features        BYTEA NOT NULL,
    open_timestamp       TIMESTAMP NOT NULL DEFAULT current_timestamp,
    close_timestamp      TIMESTAMP,
    closure_reason       BYTEA,
    PRIMARY KEY ( id )
);
