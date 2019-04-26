#!/bin/bash

# Establishes a (secure?) connection to the production RDS
# instance via the EC2 bastion host.
# You can use this like so:
#   1. Provision all terraform infrastructure, then
#   2. Run this script, then
#   3. Open up the SQL client of your choice and
#      connect to 127.0.0.1:5433 using your production
#      RDS's production username and password.
# Once you stop this script, the connection will be lost.


# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

# Get variables needed to make the tunnel

# We'll connect to 127.0.0.1:5433 locally:
LOCAL_PORT=$(<scripts/local_port)

# User to ssh into in AWS
BASTION_USER="ec2-user"

# Bastion's public IP Address
BASTION_IP=$(<terraform/out/bastion_ip_address)

# RDS's private address (accessible from bastion):
RDS_DB_ADDRESS=$(<terraform/out/rds_db_address)

# RDS's port (probably 5432)
RDS_DB_PORT=$(<terraform/out/rds_db_port)

# Private key path (provided or default)
PRIVATE_KEY_PATH=${1:-~/.ssh/id_rsa}

echo "$BASTION_IP"
echo "$RDS_DB_ADDRESS"
echo "$RDS_DB_PORT"

# Establish tunnel and listen for connections locally
ssh -N \
    -L $LOCAL_PORT:$RDS_DB_ADDRESS:$RDS_DB_PORT \
    $BASTION_USER@$BASTION_IP \
    -i $PRIVATE_KEY_PATH -v