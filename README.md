# Tortoaster

![Logo](bucket_data/thumbnails/d410d185-f372-43e4-bc4b-888bada43d83)

## Launch Project

```shell
docker compose --profile full up -d
```

## Navigation

* The website itself can be found at http://localhost:8000
* MinIO, which hosts all images and markdown files, can be accessed at http://localhost:8001
* A MailCrab instance, useful for intercepting account confirmation emails, runs on http://localhost:8002
* The KeyCloak admin console is at http://localhost:8003

## Set-Up for Local Development or Debugging

### Install tools

#### sqlx

```shell
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### Run dependencies

```shell
docker compose up -d
```

### Run backend

> See `Config.toml` for build options (these can be overridden with environment variables)

```shell
cargo run
```

## Useful commands

#### Prepare compile-time checked queries

Should be run when:

* New compile-time checked queries are created
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

## License

Tortoaster website\
Copyright (C) 2025 Rick van der Wal

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
