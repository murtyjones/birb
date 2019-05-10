#!/bin/bash

# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

# Set variables to use in the tagging process
VERSION=$(git rev-parse HEAD)
REPO=murtyjones

# docker login to ECR registry
aws ecr get-login --region $AWS_DEFAULT_REGION --no-include-email | sh

# Build docker image for production and push to ECR
docker build -t $REPO/birb_edgar_worker:$VERSION -t $REPO/birb_edgar_worker:latest -f ./out/Dockerfile-prod ./out

# Tag image
docker tag $REPO/birb_edgar_worker:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/birb_edgar_worker:latest

# Push to ECR
# Disable until infrastructure is done:
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_DEFAULT_REGION.amazonaws.com/birb_edgar_worker:latest

# Force cluster to restart with new image
# Disable until infrastructure is done:
aws ecs update-service --cluster edgar-worker-cluster --service birb-edgar-worker-service --force-new-deployment
