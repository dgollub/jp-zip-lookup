#!/usr/bin/env bash
#
# This script starts the PostgreSQL service in a Docker container, with a locally mounted volume "./pgdata" for
# the data.
#

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
PG_DATA_DIR="${SCRIPTPATH}/pgdata"
DOCKER_COMPOSE_FILE="${SCRIPTPATH}/docker-compose.yml"
GROUP=${GROUP}
SUDO=

DB_USER=${JPZIP_DB_USER:=jpzip}
DB_PASSWORD=${JPZIP_DB_PASSWORD:=jpzip}
DB_NAME=${JPZIP_DB_NAME:=jpzip}
DB_PORT=${JPZIP_DB_PORT:=5432}
DB_HOST=${JPZIP_DB_HOST:=localhost}

[ -z $GROUP ] && GROUP=`groups $(whoami) | cut -d' ' -f1`
[ -z $GROUP ] && GROUP=`id -g`
[ "$OSTYPE" != "darwin"* ] || SUDO="sudo"

cleanup() {
    err=$?
    exit $err
}
trap cleanup INT TERM

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

export PGPASSWORD="${DB_PASSWORD}"

TEST=`psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c 'SELECT NOW();'`
if [ ! -z "${TEST}" ]; then
  echo "PostgreSQL database service already up and running."
  exit 0;
fi

echofig "Building and starting PostgreSQL Docker container"

mkdir -p "${PG_DATA_DIR}"
$SUDO chown -R $USER:$GROUP "${PG_DATA_DIR}"
$SUDO chmod -R 0777 "${PG_DATA_DIR}"

docker-compose -f "${DOCKER_COMPOSE_FILE}" stop
docker-compose -f "${DOCKER_COMPOSE_FILE}" up -d --build || abort "Failed to build or start the Docker container."

echo "Done: Docker PostgreSQL container will be up and running shortly. It usually takes a couple of seconds."
echo "Waiting for container to start up ..."
echo ""

until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' > /dev/null 2>&1; do
    echo "Postgres is still unavailable - sleeping ... press CTRL-C to stop this script"
    sleep 1
done

echo "Use 'docker logs -f postgres14-jpzip' to see the logs of the database."
echo "Use 'PGPASSWORD=jpzip psql --host=localhost --port 5432 --user=jpzip' to connect to the database."
echo ""
echo "Done"
