# Japan ZIP Code Lookup

This is a simple REST-based webservice that allows users to lookup Japanese ZIP codes and get the address for it.

## Dataset

The dataset used is the official [住所の郵便番号（CSV形式）](https://www.post.japanpost.jp/zipcode/download.html) data provided by Japan Post.
The service uses the [読み仮名データの促音・拗音を小書きで表記するもの](https://www.post.japanpost.jp/zipcode/dl/kogaki-zip.html) version of the data.


*TODO* 
- Tools to download and import data are provided in this repository. - how to use examples
- replace sqlx with ormx or SeaORM

# Build Instructions

## Requirements

You will need the following software/tools installed on your system

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/) (optional)
- [PostgreSQL](https://www.postgresql.org/) or alternatively [SQLite](https://www.sqlite.org/)

## Building and running

Run `cargo build --release` to build a release build of the service.

Run `cargo run` to run a debug build of the service.

## Database

_TBD_

## Docker

_TBD_

# Copyright

Copyright (c) 2022, 2023 by Daniel Kurashige-Gollub <daniel@kurashige-gollub.de>


# License

[MIT 2.0](LICENSE.md)
