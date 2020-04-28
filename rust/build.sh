#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/rust_example.wasm ./res/
#wasm-opt -Oz --output ./res/cross_contract_low_level.wasm ./res/cross_contract_low_level.wasm
 
