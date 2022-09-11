#!/bin/bash
set -e

cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near deploy --wasmFile target/wasm32-unknown-unknown/release/ehr.wasm --accountId jeknowledge.testnet 