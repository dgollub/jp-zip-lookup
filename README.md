# Japan ZIP Code Lookup

This is a simple REST-based webservice that allows users to lookup Japanese ZIP codes and get the address for it.

## Dataset

The dataset used is the official [住所の郵便番号（CSV形式）](https://www.post.japanpost.jp/zipcode/download.html) data provided by Japan Post.
The service uses the [読み仮名データの促音・拗音を小書きで表記するもの](https://www.post.japanpost.jp/zipcode/dl/kogaki-zip.html) version of the data.


# Build Instructions

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/dgollub/jp-zip-lookup/actions/workflows/rust.yml?query=branch%3Amaster">
    <img src="https://img.shields.io/github/actions/workflow/status/dgollub/jp-zip-lookup/rust.yml?branch=master&style=flat-square"
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
curl -s http://localhost:8900/healthcheck | jq
{
  "message": "ok"
}

# get a list of postcodes
curl -s http://localhost:8900/postcodes | jq
[
  "0010000",
  "0010010",
  "0010011",
  "0010012",
  "0010013",
  "0010014",
  "0010015",
  ...
]

# get a list of cities in "postcode" (e.g. 1130034)
# you can use incomplete postcodes, e.g. only the first 3 digits
curl -s http://localhost:8900/postcodes/:postcode | jq

# example for 113-0034
curl -s http://localhost:8900/postcodes/1130034 | jq
[
  {
    "zipCode": "1130034",
    "muncipalitiesKanji": "文京区",
    "muncipalitiesKana": "ﾌﾞﾝｷｮｳｸ",
    "muncipalitiesKanaFull": "ブンキョウク",
    "muncipalitiesHiragana": "ぶんきょうく",
    "muncipalitiesRomaji": "bunkyouku",
    "townKanji": "湯島",
    "townKana": "ﾕｼﾏ",
    "townKanaFull": "ユシマ",
    "townHiragana": "ゆしま",
    "townRomaji": "yushima"
  },
  ...
]

# get a list of prefectures
curl -s http://localhost:8900/prefectures | jq
[
  {
    "prefCode": "01",
    "halfWidthKana": "ﾎｯｶｲﾄﾞｳ",
    "fullWidthKana": "ホッカイドウ",
    "hiragana": "ほっかいどう",
    "pref": "北海道",
    "romaji": "hokkaidou"
  },
  ...
]

# get a list of cities for a prefecture by providing the prefecture code
curl -s http://localhost:8900/prefectures/:prefecture_code/cities | jq
# example for Hokkaido
curl -s http://localhost:8900/prefectures/01/cities | jq
[
  {
    "govCode": "01101",
    "zipCode": "0600001",
    "halfWidthKana": "ｷﾀ1ｼﾞｮｳﾆｼ(1-19ﾁｮｳﾒ)",
    "fullWidthKana": "キタ1ジョウニシ(1-19チョウメ)",
    "hiragana": "きた1じょうにし（1ー19ちょうめ）",
    "city": "北一条西（１～１９丁目）",
    "romaji": "kita1jounishi(1-19choume)"
  },
  ...
]
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
