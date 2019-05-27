#!/bin/bash

cd $(dirname $0)

set -e

rm -rf dist/
mkdir -p dist/

wasm-pack build --target web --no-typescript --out-dir ./dist --release
