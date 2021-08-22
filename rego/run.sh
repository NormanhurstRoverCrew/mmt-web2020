#!/bin/sh

build(){
    wasm-pack build --target web --dev -- --features app
}

pack(){
   rollup ./main.js --format iife --file ./pkg/bundle.js
}

minify(){
	minifier ./pkg/bundle.js
}

less(){
	lessc less/main.less pkg/style.min.css
}

run(){
	cargo run --bin server
}

less
build
pack
minify
# run
