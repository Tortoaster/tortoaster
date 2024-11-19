FROM rust:latest AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=build /app/target/release/tortoaster /usr/local/bin

RUN apt-get update
RUN apt-get install -y ca-certificates

ENTRYPOINT ["/usr/local/bin/tortoaster"]
