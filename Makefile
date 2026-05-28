.PHONY: all build test bench bench-kit examples-kit example-kit-simple example-kit-complex example-kit-svg example-kit-math example-kit-native example-renderer-pipeline doc clean hooks fmt clippy docs-serve changelog changelog-preview release-dry release-patch release-minor release-major

all: build test

build:
	@echo "Building workspace..."
	cargo build --workspace

test:
	@echo "Testing workspace..."
	cargo test --workspace

bench:
	@echo "Benchmarking workspace..."
	cargo bench --workspace

bench-kit:
	@echo "Benchmarking graphitepdf-kit..."
	cargo bench -p graphitepdf-kit --bench benchmarks

example-kit-simple:
	@echo "Running graphitepdf-kit simple example..."
	cargo run -p graphitepdf-kit --example simple

example-kit-complex:
	@echo "Running graphitepdf-kit complex example..."
	cargo run -p graphitepdf-kit --example complex

example-kit-svg:
	@echo "Running graphitepdf-kit SVG example..."
	cargo run -p graphitepdf-kit --example svg

example-kit-math:
	@echo "Running graphitepdf-kit math example..."
	cargo run -p graphitepdf-kit --example math

example-kit-native:
	@echo "Running graphitepdf-kit native example..."
	cargo run -p graphitepdf-kit --example native

example-renderer-pipeline:
	@echo "Running graphitepdf-renderer pipeline example..."
	cargo run -p graphitepdf-renderer --example pipeline

examples-kit: example-kit-simple example-kit-complex example-kit-svg example-kit-math example-kit-native

doc:
	@echo "Generating documentation..."
	cargo doc --workspace --no-deps

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# ── Dev tooling setup ─────────────────────────────────────────────────────────

hooks:
	git config core.hooksPath .githooks
	chmod +x .githooks/pre-push
	@echo "✓ Pre-push hook installed (fmt → check → clippy → wasm-build)."

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

docs-serve:
	cd docs && trunk serve

# ── CHANGELOG ─────────────────────────────────────────────────────────────────

changelog:
	git cliff --output CHANGELOG.md
	@echo "✓ CHANGELOG.md regenerated."

changelog-preview:
	git cliff --unreleased --strip header

# ── Release ───────────────────────────────────────────────────────────────────

release-dry:
	cargo release minor --workspace --no-publish --no-push --no-tag

release-patch:
	cargo release patch --workspace --execute

release-minor:
	cargo release minor --workspace --execute

release-major:
	cargo release major --workspace --execute
