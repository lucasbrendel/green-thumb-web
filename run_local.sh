#!/bin/bash

./build_all.sh

# cp client/release/green-thumb.js server/static/green-thumb.js
# cp client/release/green-thumb.wasm server/static/ui.wasm
# cp client/release/green-thumb.css server/static/styles.css
cp -R client/release/. server/static/

(
  echo "running server"
  cd server
  export ROCKET_CLI_COLORS=off
  cargo run --release
)