#!/bin/bash

# Exit immediately if any step errors:
set -e

# crate to build a binary from:
PACKAGE=$1

# Go to project root
cd $(git rev-parse --show-toplevel)

## get base image
docker pull clux/muslrust:nightly

# build binary
docker run \
    -v cargo-cache:/root/.cargo/registry \
    -v "$PWD:/volume" \
    --rm \
    -it clux/muslrust \
    cargo build -p ${PACKAGE} --release
