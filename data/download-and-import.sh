#!/usr/bin/env bash
#
# This script downloads the data ZIP file and imports it into the database (must be up and running).
#

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
ZIP_NAME="ken_all.zip"
UNZIPPED_NAME="KEN_ALL.CSV"
DOWNLOAD_URL="https://www.post.japanpost.jp/zipcode/dl/kogaki/zip/${ZIP_NAME}"

DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}
DB_USER=${DB_USER:-jpzip}
DB_PASS=${DB_PASS:-jpzip}
DB_NAME=${DB_NAME:-jpzip}

echofig() {
  WIDTH=`tput cols`; [ $? -eq 0 ] || WIDTH=120
  type figlet > /dev/null && figlet -w $WIDTH $*
  echo $*
}

abort() {
  [ "$1x" != "x" ] && echo $1
  echo "Aborted."
  exit 1;
}

echofig "Downloading data file from japanpost.jp"

# TODO(dkg): ask to overwrite if file exists already?!
# curl -q -o "${ZIP_NAME}" "${DOWNLOAD_URL}" || abort "Could not download data file from ${DOWNLOAD_URL}"
[ -f "${UNZIPPED_NAME}" ] && rm -rf "${UNZIPPED_NAME}" > /dev/null
unzip "${ZIP_NAME}"

echofig "DB Import"
echo "Importing data file into DB at ${DB_HOST}:${DB_PORT}/${DB_NAME}"

COLUMNS="gov_code, zip_code_old, zip_code, prefecture_kana, muncipalities_kana, town_kana, prefecture_kanji, muncipalities_kanji, town_kanji, two_or_more_zip, address_numbered, with_chome, two_or_more_area, updated, change_reason"
PGPASSWORD="${DB_PASS}" psql \
  -h ${DB_HOST} -p ${DB_PORT} -d ${DB_NAME} -U ${DB_USER} \
  -c "\copy addresses ($COLUMNS) from './${UNZIPPED_NAME}' with (format csv, delimiter ',', header true, encoding 'shift-jis')" 
[ $? -ne 0 ] && abort "Import failed"

echo "Done"
