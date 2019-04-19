#!/bin/bash

# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

# Set variables to use in the tagging process
NAME_UNDER=birb_api
NAME_HYPHEN=birb-api
VERSION=$(git rev-parse HEAD)
REPO=murtyjones

# docker login to ECR registry
aws ecr get-login --region $AWS_DEFAULT_REGION --no-include-email | sh

# Build docker image for production and push to ECR
docker build -t $REPO/$NAME_UNDER:$VERSION -t $REPO/$NAME_UNDER:latest -f ./out/Dockerfile-prod ./out

# Tag image
docker tag $REPO/$NAME_UNDER:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/$NAME_UNDER:latest

# Push to ECR
# Disable until infrastructure is done:
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/$NAME_UNDER:latest

# Force cluster to restart with new image
# Disable until infrastructure is done:
aws ecs update-service --cluster $NAME_UNDER-cluster --service $NAME_HYPHEN-service --force-new-deployment
