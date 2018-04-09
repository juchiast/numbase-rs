build:
	cargo web build
	mkdir dist -p
	cp ./target/wasm32-unknown-unknown/release/numbase.wasm dist
	cp ./target/wasm32-unknown-unknown/release/numbase.js dist
