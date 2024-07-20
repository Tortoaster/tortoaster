FROM curlimages/curl:latest AS tailwindcss_build

ARG TARGETARCH

WORKDIR /app

RUN if [ "$TARGETARCH" = "amd64" ]; then \
        export RELEASE=tailwindcss-linux-x64; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        export RELEASE=tailwindcss-linux-arm64; \
    else \
        echo "Unsupported architecture: $TARGETARCH"; \
        exit 1; \
    fi

RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/$RELEASE
RUN mv $RELEASE tailwindcss
RUN chmod +x tailwindcss

FROM debian:bookworm-slim AS tailwindcss

COPY --from=tailwindcss_build /app/tailwindcss /usr/local/bin/tailwindcss

ENTRYPOINT ["/usr/local/bin/tailwindcss"]

FROM tailwindcss AS style

WORKDIR /app

COPY ./tailwind /app
COPY ./templates /app/templates

RUN tailwindcss -i ./input.css -o ./output.css -m

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
