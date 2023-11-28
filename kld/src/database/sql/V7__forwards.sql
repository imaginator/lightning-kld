CREATE TYPE forward_status AS ENUM ('succeeded', 'failed');

CREATE TABLE forwards (
    id                   UUID NOT NULL,
    inbound_channel_id   BYTEA NOT NULL,
    outbound_channel_id  BYTEA,
    amount               INT,
    fee                  INT,
    status               forward_status NOT NULL,
    htlc_destination     BYTEA,
    timestamp            TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY ( id )
);
