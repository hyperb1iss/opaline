# Opaline — Development Tasks
# Install just: cargo install just

set dotenv-load := false

# Show available recipes
default:
    @just --list --unsorted

# ── Quality ───────────────────────────────────────────────────

# Run format check + clippy
check:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings

# Format all Rust code
fmt:
    cargo fmt --all

# Run tests with cargo-nextest
test:
    cargo nextest run --all-features

# Run doc tests
test-doc:
    cargo test --doc --all-features

# Run all tests (nextest + doc)
test-all: test test-doc

# ── Build ─────────────────────────────────────────────────────

# Build library
build:
    cargo build --all-features

# Build in release mode
build-release:
    cargo build --release --all-features --locked

# Generate documentation
doc:
    cargo doc --all-features --open

# ── Demo ──────────────────────────────────────────────────────

# Run the interactive theme showcase
demo:
    cargo run --example theme-showcase

# ── Security ──────────────────────────────────────────────────

# Run cargo-deny supply-chain audit
deny:
    cargo deny check

# ── Docs Site ─────────────────────────────────────────────────

# Start VitePress dev server
docs-dev:
    cd docs && pnpm dev

# Build docs for deployment
docs-build:
    cd docs && pnpm build

# ── Full Pipeline ─────────────────────────────────────────────

# Run the full CI pipeline locally
ci: check test-all deny
