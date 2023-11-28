CREATE TABLE channel_manager (
    id              BYTEA PRIMARY KEY,
    manager         BYTEA NOT NULL,
    timestamp       TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE channel_monitors (
    out_point       BYTEA NOT NULL,
    update_id       INT NOT NULL,
    monitor         BYTEA NOT NULL,
    timestamp       TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY ( out_point )
);

CREATE TABLE channel_monitor_updates (
    out_point       BYTEA NOT NULL,
    update          BYTEA NOT NULL,
    update_id       INT NOT NULL,
    timestamp       TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY ( out_point, update_id )
);

CREATE TABLE scorer (
    id              BYTEA PRIMARY KEY,
    scorer          BYTEA NOT NULL,
    timestamp       TIMESTAMP NOT NULL DEFAULT current_timestamp
);
