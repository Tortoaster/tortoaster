# Tortoaster

![](./static/turtle-back.png)

## Set-up for Development

### Install dependencies

```shell
# Tailwind CLI
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
sudo chmod +x tailwindcss-linux-x64
sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwind

# Rust CLI tools
cargo install sqlx-cli --no-default-features --features rustls,postgres
cargo install cargo-watch
```

### After any database changes

```shell
sqlx migrate run
```

### After any database changes or creating new queries

```shell
export DATABASE_URL=postgres://tortoaster:password@localhost/tortoaster
cargo sqlx prepare
```

### Run locally

```shell
docker compose up
```
