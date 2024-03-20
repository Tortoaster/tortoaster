FROM rust:latest

RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

CMD [ "sqlx" ]
