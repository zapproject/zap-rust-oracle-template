#!/bin/bash
cd ./testserver
node  websocket.js &

cd ../
cargo run --release ./devnet_config.json ws://127.0.0.1:8545 127.0.0.1:3007