#!/usr/bin/env bash
is_mac() {
  [[ "$OSTYPE" == "darwin"* ]]
}


echo "Building the rust library"

cd .. 

wasm-pack build --target=web

PACKAGEJSON=./pkg/package.json
IMPORTFILE=./pkg/ridb_rust.js

if is_mac; then
  sed -i '' 's/"module": "ridb_rust.js",/"main": "ridb_rust.js",/' $PACKAGEJSON
else
  sed -i  's/"module": "ridb_rust.js",/"main": "ridb_rust.js",/' $PACKAGEJSON
fi
