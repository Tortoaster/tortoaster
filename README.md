# Tortoaster

![Logo](bucket_data/thumbnails/d410d185-f372-43e4-bc4b-888bada43d83)

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
sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss
```

#### sqlx

```shell
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

#### SeaORM

```shell
cargo install sea-orm-cli
```

#### cargo-watch

(Optional)

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

> `compose.yml` performs this step automatically

```shell
tailwindcss -i ./input.css -o ./static/style.css -m
```

Alternatively, to regenerate CSS automatically on change:

```shell
tailwindcss -i ./input.css -o ./static/style.css -m -w
```

#### Run the project

> See `Config.toml` for build options (these can be overridden with environment variables)

```shell
cargo run
```

Alternatively, to re-run automatically on change:

```shell
cargo watch -x run -w templates
```

#### Prepare compile-time checked queries

Should be run when:

* Creating new compile-time checked queries
* New database migrations are created

```shell
export DATABASE_URL=postgres://tortoaster:password@localhost/tortoaster
cargo sqlx prepare
```

#### Generate entities

Should be run when:

* New database migrations are created

```shell
sea-orm-cli generate entity -o src/model --date-time-crate time
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
