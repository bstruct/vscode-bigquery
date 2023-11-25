#!/usr/bin/env bash

# https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/continuous-integration.html
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# npm install -g chromedriver

wasm-pack test --headless --chrome 
# wasm-pack test --chrome
