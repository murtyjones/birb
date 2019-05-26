#!/bin/bash

cd $(git rev-parse --show-toplevel)

mkdir -p build/
mkdir -p dist/

wasm-pack build --dev --target web --no-typescript --out-dir ./build
