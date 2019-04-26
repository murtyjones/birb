#!/bin/bash

set -e

SEED_DIRECTORY=db/seeders

# Go to project root
cd $(git rev-parse --show-toplevel)

for i in `/bin/ls -1 $SEED_DIRECTORY/*.sql;`; do
    PG_PASSWORD=develop psql --host=localhost \
         --port=5432 \
         --username=postgres \
         --dbname=postgres \
         --file=$i
done