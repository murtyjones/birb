#!/bin/bash

set -e

MIGRATE_DIRECTORY=db/migrations
ENV=$1
ACTION=$2

# if less than 2 args (environment, action), exit
if [ $# -lt 2 ]; then
    echo "Please provide the environment you want to seed ("local" or "prod")."
    exit 2;
fi

if [ $ENV = "prod" ]; then
    echo "This is will migrate the production database! If you are sure you want to do this please type 'yes'"
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

# Connection string
CONNECTION_STRING=postgres://$DB_USERNAME:$DB_PASSWORD@localhost:$PORT/$DB_NAME

# Perform migration action
dbmigrate --url $CONNECTION_STRING --path ./db/migrations $ACTION
