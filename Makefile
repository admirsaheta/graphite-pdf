.PHONY: all build test bench bench-kit examples-kit example-kit-simple example-kit-complex doc clean

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

examples-kit: example-kit-simple example-kit-complex

doc:
	@echo "Generating documentation..."
	cargo doc --workspace --no-deps

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
