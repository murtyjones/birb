#!/bin/bash

set -e

# Go to project root
cd $(git rev-parse --show-toplevel)

cd ./crates/client

./build-wasm.sh
OUTPUT_CSS="$(pwd)/build/app.css" cargo +nightly run -p server
