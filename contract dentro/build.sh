#!/bin/bash
set -e
export NFT_CONTRACT_ID="t.jeknowledge.testnet"
export MAIN_CONTRACT_ID="main.jeknowledge.testnet"
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near deploy --wasmFile target/wasm32-unknown-unknown/release/nftjek.wasm --accountId $MAIN_CONTRACT_ID