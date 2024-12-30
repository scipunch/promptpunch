format:
	cargo fmt

clippy:
	cargo clippy --all-features

check:
	cargo check --all-features

tests:
	cargo test --all-features

example-base:
	cargo run --example base
