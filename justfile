# Digital K-Hole justfile

# default recipe: list available commands
default:
    @just --list

# run dev server with hot reload
serve: build
    ~/.cargo/bin/trunk serve

# run dev server on specific port
serve-port port="8080":
    ~/.cargo/bin/trunk serve --port {{port}}

# build for development
build:
    ~/.cargo/bin/trunk build

# build for release (optimized, smaller wasm)
release:
    ~/.cargo/bin/trunk build --release

# clean build artifacts
clean:
    cargo clean
    rm -rf dist/

# check code without building
check:
    cargo check --target wasm32-unknown-unknown

# format code
fmt:
    cargo fmt

# run clippy lints
lint:
    cargo clippy --target wasm32-unknown-unknown

# watch for changes and rebuild
watch:
    ~/.cargo/bin/trunk watch

# validate all tanka yaml files against schema
validate:
    cargo run --bin validate
