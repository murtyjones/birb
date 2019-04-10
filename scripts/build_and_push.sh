#!/bin/bash

# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

# Set variables to use in the tagging process
NAME=api
VERSION=$(git rev-parse HEAD)
SEMVER_VERSION=$(grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)
REPO=birb


# docker login to ECR registry
aws ecr get-login --region $AWS_DEFAULT_REGION --no-include-email | sh

# Build docker image for production and push to ECR
docker build -t $REPO/$NAME:$VERSION -t $REPO/$NAME:latest -f ./out/Dockerfile-prod ./out

# Tag image
docker tag $REPO/$NAME:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/$NAME:latest

# Push to ECR
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/$NAME:latest

# Force cluster to restart with new image
# aws ecs update-service --cluster cluster-name-here --service service-name-here --force-new-deployment