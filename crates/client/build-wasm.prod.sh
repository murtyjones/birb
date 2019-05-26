#!/bin/bash

cd $(git rev-parse --show-toplevel)

rm -rf dist/
mkdir -p dist/

wasm-pack build --target web --no-typescript --out-dir ./dist --release
