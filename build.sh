#!/usr/bin/env bash

rm -rf ./pkg

cargo build --target wasm32-unknown-unknown --release

wasm-pack build --target web

cp -P pkg/* ../hankruiger.com/app/modules/behave_blog_demo
