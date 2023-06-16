# Japan ZIP Code Lookup

This is a simple REST-based webservice that allows users to lookup Japanese ZIP codes and get the address for it.

## Dataset

The dataset used is the official [住所の郵便番号（CSV形式）](https://www.post.japanpost.jp/zipcode/download.html) data provided by Japan Post.
The service uses the [読み仮名データの促音・拗音を小書きで表記するもの](https://www.post.japanpost.jp/zipcode/dl/kogaki-zip.html) version of the data.


# Build Instructions

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/dgollub/jp-zip-lookup/actions/workflows/rust.yml?query=branch%3Amain">
    <img src="https://img.shields.io/github/actions/workflow/status/dgollub/jp-zip-lookup/rust.yml?branch=main&style=flat-square"
      alt="github actions status" />
  </a>
</div>

## Requirements

You will need the following software/tools installed on your system

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/) (optional, provides the [PostgreSQL database](docker/))
- [PostgreSQL](https://www.postgresql.org/) (not needed if you have Docker and use the provided [docker-compose file](docker/compose.yml))

## Building and running

Run `cargo build --release` to build a release build of the service.

Run `cargo run` to run a debug build of the service.

Then try one of the following curl examples:

```bash
# general healthcheck endpoint
curl http://localhost:8900/healthcheck

# get a list of postcodes
curl http://localhost:8900/postcodes

# get a list of cities in "postcode" (e.g. 3430114)
# you can use incomplete postcodes, e.g. only the first 3 digits
curl http://localhost:8900/postcodes/:postcode

# get a list of prefectures
curl http://localhost:8900/prefectures

# get a list of cities for a prefecture by providing the prefecture code
curl http://localhost:8900/prefectures/:prefecture_code/cities
# example for Okinawa
curl http://localhost:8900/prefectures/47/cities
```

## Database

The database must be up and running for the service to work. 

The following environment variables are read to connect to the database:

```bash
# showing default values here
JPZIP_DB_HOST="localhost"
JPZIP_DB_PORT=5432
JPZIP_DB_NAME="jpzip"
JPZIP_DB_USER="jpzip"
JPZIP_DB_PASSWORD="jpzip"
```

The service _can_ [automatically download](data/README.md) the data and import it into the
dabase, but the schema must exists prior to that.

You can use a `.env` environment file to set those variables. See [.env.example](.env.example) for an example.

## Docker

You can use Docker to run the PostgreSQL database service.
Check [README.md](docker/README.md) for details.

# Copyright

Copyright (c) 2022, 2023 by Daniel Kurashige-Gollub <daniel@kurashige-gollub.de>


# License

[MIT 2.0](LICENSE.md)
