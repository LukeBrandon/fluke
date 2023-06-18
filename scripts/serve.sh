#!/bin/bash

cd ../frontend

rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
trunk build --dist ../backend/static

cd ../backend
cargo build
