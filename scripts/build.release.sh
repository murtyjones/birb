#!/bin/bash

# Go to project root
cd $(git rev-parse --show-toplevel)

cd ./crates/client

./build-wasm.prod.sh
OUTPUT_CSS="$(pwd)/dist/app.css" cargo +nightly build -p server --release --target x86_64-unknown-linux-musl
