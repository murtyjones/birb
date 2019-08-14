#!/usr/bin/env bash

# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

# Build
npm run build

# Upload to S3
aws s3 sync ./build s3://birb.io/

cloudfront_id=$(<terraform/out/www_cloudfront_id)

# Invalidate cache
aws cloudfront create-invalidation --distribution-id ${cloudfront_id} --paths /index.html
