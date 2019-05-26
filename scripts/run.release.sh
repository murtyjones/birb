#!/bin/bash

# Go to project root
cd $(git rev-parse --show-toplevel)

cd ./crates/client

../../target/x86_64-unknown-linux-musl/release/api
