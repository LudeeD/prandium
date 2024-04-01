.PHONY: build run check test clean

# Build the project.
build:
	npx tailwindcss -i ./src/theme/input.css -o ./src/theme/output.css
	cargo build

# Run the project.
run: build
	cargo run

# Compile the project without producing an executable.
check:
	cargo check

# Run tests.
test:
	cargo test

# Clean up the project.
clean:
	cargo clean
