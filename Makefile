check: fmt lint build test
	@echo ""
	@echo "โจ Good to go!"

check-static: fmt lint
	@echo ""
	@echo "โจ Good to go!"

check-dynamic: build test
	@echo ""
	@echo "โจ Good to go!"

lint:
	@echo ""
	@echo "๐ฌ Lint"
	@cargo clippy --all-targets --all-features -- -D warnings

test:
	@echo ""
	@echo "๐งช Test"
	@cargo nextest run

build: build-readmes
	@echo ""
	@echo "๐ง Build"
	@cargo build

fmt:
	@echo ""
	@echo "๐ Format"
	@cargo fmt --all
	@echo "    Done!"

build-readmes:
	@echo ""
	@echo "๐ Generate readmes"
	@cargo readme -r server -i src/main.rs -o README.md
	@echo "    Done!"

# https://lib.rs/crates/cargo-readme
# https://lib.rs/crates/cargo-nextest
install-deps:
	@echo ""
	@echo "๐ฆ Install tools"
	@cargo install cargo-readme cargo-nextest --locked

.PHONY: test build check check-static fmt lint build-readmes install-deps
