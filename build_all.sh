#!/bin/bash

set -e
echo "Building Green Thumb"
echo "Build Client"
pushd client 
yarn build
popd
echo "Build Client Complete"
echo "Build Server"
pushd server 
cargo build --release
popd
echo "Build Server Complete"