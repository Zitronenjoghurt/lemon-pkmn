.PHONY: test data

data:
	cargo run --release --bin lemon-pkmn-codegen

test:
	cargo test --release --features include-data,serde