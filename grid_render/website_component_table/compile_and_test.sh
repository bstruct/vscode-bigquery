#!/usr/bin/env bash

set -e  # Exit on error

echo "cargo t"
cargo t

echo "cargo clippy"
cargo clippy -- -D warnings

echo ""
echo "Running WASM tests in Chrome..."
wasm-pack test --headless --chrome
# echo ""
# echo "Running WASM tests in Safari..."
# wasm-pack test --headless --safari

echo ""
echo "✅ All tests passed!"