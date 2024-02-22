# Tortoaster

## Install dependencies

```shell
# Tailwind CLI
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
sudo chmod +x tailwindcss-linux-x64
sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwind

# Rust CLI tools
cargo install sqlx-cli --no-default-features --features rustls,postgres
cargo install cargo-watch
```

## Run locally

```shell
cargo sqlx prepare
docker compose up
```
