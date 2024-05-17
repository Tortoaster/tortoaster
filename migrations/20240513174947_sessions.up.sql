CREATE SCHEMA sessions;

CREATE TABLE sessions.sessions
(
    id          TEXT PRIMARY KEY NOT NULL,
    data        BYTEA            NOT NULL,
    expiry_date TIMESTAMPTZ      NOT NULL
);
