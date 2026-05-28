.PHONY: all build test bench bench-kit examples-kit example-kit-simple example-kit-complex example-kit-svg example-kit-math example-kit-native example-renderer-pipeline doc clean

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
