#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-target}"
set -e
cd "`dirname $0`"
RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release
cp $TARGET/wasm32-unknown-unknown/release/approval_receiver.wasm ./res/
cp $TARGET/wasm32-unknown-unknown/release/wrapped_appchain_nft.wasm ./res/
cp $TARGET/wasm32-unknown-unknown/release/token_receiver.wasm ./res/
