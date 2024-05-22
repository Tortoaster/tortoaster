FROM curlimages/curl:latest AS tailwindcss_build

WORKDIR /app

RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
RUN chmod +x tailwindcss-linux-x64

FROM debian:bookworm-slim AS tailwindcss

COPY --from=tailwindcss_build /app/tailwindcss-linux-x64 /usr/local/bin/tailwindcss-linux-x64

ENTRYPOINT ["/usr/local/bin/tailwindcss-linux-x64"]

FROM tailwindcss AS style

WORKDIR /app

COPY ./tailwind /app
COPY ./templates /app/templates

RUN tailwindcss-linux-x64 -i ./input.css -o ./output.css -m

FROM rust:latest AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=build /app/target/release/tortoaster /usr/local/bin
COPY --from=build /app/static /app/static
COPY --from=style /app/output.css /app/static/style.css

RUN apt-get update
RUN apt-get install -y ca-certificates

ENTRYPOINT ["/usr/local/bin/tortoaster"]
