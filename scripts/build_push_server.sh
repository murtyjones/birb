#!/bin/bash

# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

# Remove current output folder and recreate it
rm -rf out
mkdir out

# Copy built binary and Dockerfile to output folder
cp ./crates/server/Dockerfile-prod out
cp ./target/x86_64-unknown-linux-musl/release/server out

# Set variables to use in the tagging process
VERSION=$(git rev-parse HEAD)
REPO=murtyjones

# docker login to ECR registry
aws ecr get-login --region $AWS_DEFAULT_REGION --no-include-email | sh

# Build docker image for production and push to ECR
docker build -t $REPO/birb_server:$VERSION -t $REPO/birb_server:latest -f ./out/Dockerfile-prod ./out

# Tag image
docker tag $REPO/birb_server:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/birb_server:latest

# Push to ECR
# Disable until infrastructure is done:
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/birb_server:latest

# Force cluster to restart with new image
# Disable until infrastructure is done:
# aws ecs update-service --cluster birb-server-cluster --service birb-server-service --force-new-deployment
