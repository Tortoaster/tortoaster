CREATE TABLE users
(
    id                   SERIAL PRIMARY KEY,
    username             VARCHAR(32) UNIQUE       NOT NULL,
    email_address        VARCHAR(254) UNIQUE      NOT NULL,
    email_verified       BOOLEAN                  NOT NULL DEFAULT FALSE,
    password_hash        VARCHAR(82)              NOT NULL,
    account_created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    last_online_date     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
