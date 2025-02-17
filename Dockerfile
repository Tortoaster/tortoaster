FROM --platform=$BUILDPLATFORM rust:latest AS build

ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN apt-get update
RUN apt-get install -y musl-tools
RUN apt-get clean

WORKDIR /app

COPY backend .

RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        wget https://musl.cc/aarch64-linux-musl-cross.tgz && \
        tar -xzf aarch64-linux-musl-cross.tgz -C /usr/local && \
        export PATH="/usr/local/aarch64-linux-musl-cross/bin:$PATH" && \
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-musl-gcc && \
        export TARGET=aarch64-unknown-linux-musl; \
    else \
        export TARGET=x86_64-unknown-linux-musl; \
    fi && \
    rustup target add $TARGET && \
    cargo build --release --target $TARGET && \
    mv target/$TARGET/release/toast target/release/toast

FROM alpine:latest

COPY --from=build /app/target/release/toast /usr/local/bin

ENTRYPOINT ["/usr/local/bin/toast"]
