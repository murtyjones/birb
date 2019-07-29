#!/bin/bash

set -e

SEED_DIRECTORY=db/seeders

# if less than 1 args, exit
if [ $# -lt 1 ]; then
    echo "Please provide the environment you want to seed ("local" or "prod")."
    exit 2;
fi

ENV=$1

if [ $ENV = "prod" ]; then
    echo "This is will seed the production database! If you are sure you want to do this please type 'yes'"
    read -p "> "  confirmation
    if [ $confirmation != "yes" ]; then
        echo "Exiting."
        exit 2;
    fi
    PORT=$(<scripts/local_port)
    DB_USERNAME=$(<terraform/out/rds_db_username)
    DB_PASSWORD=$(<terraform/out/rds_db_password)
    DB_NAME=$(<terraform/out/rds_db_name)
elif [ $ENV = "local" ]; then
    PORT=5432
    DB_USERNAME=postgres
    DB_PASSWORD=develop
    DB_NAME=postgres
else
    echo "Unrecognized environment specified!"
    exit 2;
fi

# Go to project root
cd $(git rev-parse --show-toplevel)

for i in `/bin/ls -1 $SEED_DIRECTORY/*.sql;`; do
    # We always connect to localhost because regardless of whether
    # we're seeding the local or production environment, we are
    # connecting to a local instance. In production, localhost is
    # an SSH tunnel to RDS via our ec2 bastion.
    PG_PASSWORD=$DB_PASSWORD psql --host=localhost \
         --port=$PORT \
         --username=$DB_USERNAME \
         --dbname=$DB_NAME \
         --file=$i
done