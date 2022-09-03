check: fmt build test
	@cargo clippy

test: install-deps
	@cargo nextest run

build: build-readmes fmt
	@cargo build

fmt:
	@cargo fmt --all

build-readmes: install-deps
	@cargo readme -r server -i src/main.rs -o README.md

# https://lib.rs/crates/cargo-readme
# https://lib.rs/crates/cargo-nextest
install-deps:
	@cargo install cargo-readme --locked
	@cargo install cargo-nextest --locked

.PHONY: test build check fmt build-readmes install-deps
