.PHONY: serve build build-pages test lint clean

serve:
	trunk serve --open=false

build: test
	trunk build --release

build-pages: test
	trunk build --release --public-url /portfolio/

test:
	cargo test

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo clippy --target wasm32-unknown-unknown -- -D warnings

clean:
	rm -rf dist
