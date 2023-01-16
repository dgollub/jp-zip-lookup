#!/usr/bin/env bash
#
# This script starts the PostgreSQL service in a Docker container, with a locally mounted volume "./pgdata" for
# the data.
#

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
PG_DATA_DIR="${SCRIPTPATH}/pgdata"
DOCKER_COMPOSE_FILE="${SCRIPTPATH}/compose.yml"
GROUP=${GROUP}
SUDO=
echo "OSTYPE: $OSTYPE"
[ -z $GROUP ] && GROUP=`groups $(whoami) | cut -d' ' -f1`
[ -z $GROUP ] && GROUP=`id -g`
[ "$OSTYPE" != "darwin"* ] && SUDO="sudo"

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

echofig "Building and starting PostgreSQL Docker container"

mkdir -p "${PG_DATA_DIR}"
$SUDO chown -R $USER:$GROUP "${PG_DATA_DIR}"
$SUDO chmod -R 0777 "${PG_DATA_DIR}"

docker-compose -f "${DOCKER_COMPOSE_FILE}" stop
docker-compose -f "${DOCKER_COMPOSE_FILE}" up -d --build || abort "Failed to build or start the Docker container."

echo "Done: Docker PostgreSQL container will be up and running shortly. It usually takes a couple of seconds."
echo "Use 'docker logs -f postgres14-jpzip' to see the logs of the database."
echo "Use 'PGPASSWORD=jpzip psql --host=localhost --port 5432 --user=jpzip' to connect to the database."