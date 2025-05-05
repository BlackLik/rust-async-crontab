install:
	cargo install cargo-deny
	cargo install cargo-watch

lint:
	cargo fmt --all
	cargo check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo deny check

test:
	cargo test --all

check: lint test

.PHONY: generate-openapi
generate-openapi:
	cargo run -p cmd --bin openapi --release

run:
	docker-compose -f ./deployment/docker-compose.local.yaml up -d --build --force-recreate
