build:
	cargo build

test:
	cargo test

test_examples:
	cargo run --example basic

lint:
	cargo clippy --all-targets --all-features -- -D warnings

publish:
	cargo login $(CRATES_TOKEN)
	cargo package
	cargo publish

setup:
	rustup toolchain install stable
	rustup default stable
	rustup update
	rustup component add clippy

clean:
	cargo clean