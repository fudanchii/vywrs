all: dist/index.html dist/assets/style.css dist/vywrs.wasm dist/vywrs.js dist/vyw.config.js

clean:
	rm -rf dist
	rm -rf target

dist:
	mkdir -p dist

dist/assets: dist
	mkdir -p dist/assets

dist/index.html: dist static/index.html
	cp static/index.html dist

dist/assets/style.css: dist/assets static/assets/main.scss
	scss --unix-newlines static/assets/main.scss dist/assets/style.css

target/wasm32-unknown-unknown/release/vywrs.wasm:
	cargo web build --target-webasm

target/wasm32-unknown-unknown/release/vywrs.js: target/wasm32-unknown-unknown/release/vywrs.wasm

dist/vywrs.wasm: dist target/wasm32-unknown-unknown/release/vywrs.wasm
	cp target/wasm32-unknown-unknown/release/vywrs.wasm dist/vywrs.wasm

dist/vywrs.js: dist target/wasm32-unknown-unknown/release/vywrs.js
	cp target/wasm32-unknown-unknown/release/vywrs.js dist/vywrs.js

dist/vyw.config.js: dist static/vyw.config.js
	cp static/vyw.config.js dist/vyw.config.js

.phony: all clean
