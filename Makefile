.PHONY: test

test:
	cargo test --release --features include-data,serde