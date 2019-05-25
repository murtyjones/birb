#!/bin/bash

# Exit immediately if any step errors:
set -e

# crate to build a binary from:
PACKAGE=$1

# Go to project root
cd $(git rev-parse --show-toplevel)

# get base image
docker pull clux/muslrust:nightly

# build binary
docker run --rm \
    -v cargo-cache:/usr/local/cargo \
    -v target-cache:$$PWD/target \
    -v mac-cargo-cache:$$HOME/.cargo/registry \
    -v $PWD:/volume \
    -w /volume \
    -it clux/muslrust:nightly \
    cargo build -p ${PACKAGE} --release
