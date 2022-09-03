check: fmt lint build test
	@echo ""
	@echo "✨ Good to go!"

lint:
	@echo ""
	@echo "🔬 Lint"
	@cargo clippy --all-targets --all-features -- -D warnings

test: install-deps
	@echo ""
	@echo "🧪 Test"
	@cargo nextest run

build: build-readmes fmt
	@echo ""
	@echo "🚧 Build"
	@cargo build

fmt:
	@echo ""
	@echo "💎 Format"
	@cargo fmt --all
	@echo "    Done!"

build-readmes: install-deps
	@echo ""
	@echo "📖 Generate readmes"
	@cargo readme -r server -i src/main.rs -o README.md
	@echo "    Done!"

# https://lib.rs/crates/cargo-readme
# https://lib.rs/crates/cargo-nextest
install-deps:
	@echo ""
	@echo "📦 Install tools"
	@cargo install cargo-readme --locked
	@cargo install cargo-nextest --locked

.PHONY: test build check fmt lint build-readmes install-deps
