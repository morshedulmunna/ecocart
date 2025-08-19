########## Makefile for the Rust API crate ##########
# Usage examples:
#  make dev               # hot-reload server (requires cargo-watch)
#  make run               # run default command (apis)
#  make run CMD=cli       # run with a specific subcommand
#  make test              # run tests
#  make fmt               # format code
#  make clippy            # lint with clippy (deny warnings)

SHELL := /bin/sh

# Binary/crate name from Cargo.toml
BIN ?= ecocart

# Default subcommand for `cargo run -- <CMD>`
CMD ?= ecocart-apis

.PHONY: help dev run build build-release test fmt fmt-check clippy check clean migrate migrate-down sqlx-install

help:
	@echo "Available targets:"
	@echo "  dev             - Hot-reload run (cargo-watch)"
	@echo "  run [CMD=...]   - Run the binary with a subcommand (default: ecocart-apis)"
	@echo "  build           - Build in debug mode"
	@echo "  build-release   - Build in release mode"
	@echo "  test            - Run tests"
	@echo "  fmt             - Format code with rustfmt"
	@echo "  fmt-check       - Check formatting (no changes)"
	@echo "  clippy          - Run clippy with warnings as errors"
	@echo "  check           - Type-check without building artifacts"
	@echo "  clean           - Clean target directory"
	@echo "  migrate         - Run SQLx database migrations (requires DATABASE_URL)"
	@echo "  migrate-down    - Revert the last migration (requires sqlx-cli)"
	@echo "  sqlx-install    - Install sqlx-cli (rustls, postgres)"
	@echo "  docs            - Build rustdoc and open in browser"

# Hot-reload dev server (requires cargo-watch: cargo install cargo-watch)
dev:
	cargo watch -q -c -w src -x 'run -- $(CMD)'

# Run the binary with a subcommand (default CMD=apis)
run:
	cargo run -- $(CMD)

# Run SQLx migrations using the built-in command
migrate:
	cargo run -- migrate

# Revert the last applied migration using sqlx-cli
migrate-down:
	sqlx migrate revert -y

# Helper to install sqlx-cli if you don't have it
sqlx-install:
	cargo install sqlx-cli --no-default-features --features rustls,postgres

build:
	cargo build

build-release:
	cargo build --release

test:
	cargo test --all --all-features

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo check --all-targets --all-features

clean:
	cargo clean

.PHONY: docs
docs:
	cargo doc --no-deps --open