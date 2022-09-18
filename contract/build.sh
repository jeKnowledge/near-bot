#!/bin/bash
set -e
export NFT_CONTRACT_ID="jeknowledge.testnet"
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near deploy --wasmFile target/wasm32-unknown-unknown/release/nftjek.wasm --accountId $NFT_CONTRACT_ID
# near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID
# near view $NFT_CONTRACT_ID nft_metadata