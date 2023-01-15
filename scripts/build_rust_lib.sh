#!/bin/sh

# Go to root project
cd $(dirname "$(readlink $(test $(uname -s) = 'Linux' && echo "-f") "$0" || echo "$(echo "$0" | sed -e 's,\\,/,g')")")/..

cd rust_lib

# Should be installed wasm-pack - cargo install wasm-pack
wasm-pack build --target web
