# Japan ZIP Code Lookup

This is a simple REST-based webservice that allows users to lookup Japanese ZIP codes and get the address for it.

## Dataset

The dataset used is the official [住所の郵便番号（CSV形式）](https://www.post.japanpost.jp/zipcode/download.html) data provided by Japan Post.
The service uses the [読み仮名データの促音・拗音を小書きで表記するもの](https://www.post.japanpost.jp/zipcode/dl/kogaki-zip.html) version of the data.


# Build Instructions

## Requirements

You will need the following software/tools installed on your system

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/) (optional)
- [PostgreSQL](https://www.postgresql.org/)

## Building and running

Run `cargo build --release` to build a release build of the service.

Run `cargo run` to run a debug build of the service.

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
