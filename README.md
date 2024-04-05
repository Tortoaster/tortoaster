# Tortoaster

![](./static/turtle-back-cool.png)

My personal website!

## Launch Project

```shell
docker compose --profile full up
```

## Set-Up for Local Development or Debugging

### Install dependencies

#### Tailwind CSS CLI

```shell
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
sudo chmod +x tailwindcss-linux-x64
sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwind
```

#### sqlx

```shell
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

#### cargo-watch

```shell
cargo install cargo-watch
```

### Useful commands

#### Run dependencies

```shell
docker compose up
```

#### Run database migrations

Should be run when:

* First deploying the project
* New database migrations are created

> `compose.yml` performs this step automatically

```shell
export DATABASE_URL=postgres://tortoaster:password@localhost/tortoaster
sqlx migrate run
```

#### Run Tailwind CSS

Should be run when:

* First deploying the project
* New templates are created, edited, or deleted
* `input.css` is updated
* `tailwind.config.js` is updated

> `build.rs` performs this step automatically

```shell
tailwind -i ./input.css -o ./static/style.css --minify
```

#### Run the project

> See `Config.toml` for build options (these can be overridden with environment variables)

```shell
cargo run
```

Alternatively, to re-run automatically on change:

```shell
cargo watch -x run -w templates -w input.css -w tailwind.config.js
```

#### Prepare compile-time checked queries

Should be run when:

* Creating new compile-time checked queries
* New database migrations are created

```shell
export DATABASE_URL=postgres://tortoaster:password@localhost/tortoaster
cargo sqlx prepare
```

#### Create a database dump

Useful for creating fixtures. Run in the database container:

```shell
pg_dump \
	-d tortoaster \
	-U tortoaster \
	--data-only \
	--inserts
```
