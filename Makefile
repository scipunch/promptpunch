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

example-web:
	cargo run --features web --example web

watch-web:
	cargo watch --quiet --watch templates --exec 'run --features web --bin web'
