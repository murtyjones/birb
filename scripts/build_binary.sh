#!/bin/bash

# Exit immediately if any step errors:
set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

# get base image
docker pull clux/muslrust:nightly

# build binary
docker run --rm \
    -v cargo-cache:/usr/local/cargo \
    -v cargo-bin-cache:$$HOME/.cargo/bin \
    -v $$PWD:/volume \
    -w /volume \
    -it clux/muslrust:nightly \
    cargo build -p $(package) --release
