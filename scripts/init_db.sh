#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo > "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo > "Error : sqlx is not installed."
  echo > "Use:"
  echo > "    cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres"
  echo > "to install it."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postres using Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    # ^ Increased maximum number of connections for testing purposes
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  echo "Postgres is still unavailable - sleeping"
  sleep 1
done

echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"


DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

echo "Postgres has been migrated, ready to go!"
