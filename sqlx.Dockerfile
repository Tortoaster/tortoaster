FROM rust:latest

RUN cargo install --git https://github.com/Tortoaster/sqlx.git --rev 6c70021 sqlx-cli --no-default-features --features rustls,postgres

ENTRYPOINT [ "sqlx" ]
