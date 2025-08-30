set shell := ["zsh", "-eu", "-o", "pipefail", "-c"]

# Show all available tasks
help:
    @just --list

# Quick commit via llmc, then push (existing behavior)
commit:
    llmc
    git push

# Install toolchains and JS deps (Rust + Node)

# - Adds wasm32-wasip2 target and installs Node packages
init:
    rustup target add wasm32-wasip2
    if command -v pnpm >/dev/null 2>&1; then pnpm install; \
    elif command -v npm >/dev/null 2>&1; then npm install; \
    else echo "No Node package manager found (pnpm/npm)" >&2; exit 1; fi

# Build all Rust crates (native)
build:
    cargo build --workspace

# Build all crates in release mode (native)
build-release:
    cargo build --workspace --release

# Build all crates for WASI preview2 target
build-wasm:
    cargo build --workspace --target wasm32-wasip2

# Build WASM in release mode
build-wasm-release:
    cargo build --workspace --target wasm32-wasip2 --release

# Transpile built WASM components to JS via jco
transpile:
    if command -v pnpm >/dev/null 2>&1; then pnpm run build:js; \
    elif command -v npm >/dev/null 2>&1; then npm run build:js; \
    else echo "No Node package manager found (pnpm/npm)" >&2; exit 1; fi

# Clean Rust build artifacts
clean:
    cargo clean

# Remove WASM build outputs
clean-wasm:
    rm -rf components/target/wasm32-wasip2

# Run all workspace tests (unit + BDD where present)
test:
    cargo test --workspace

# Run integration tests (if any are defined)
test-integration:
    cargo test --test integration

# Run tests for a single crate: just test-crate sql-sqlite
test-crate crate:
    cargo test -p {{ crate }}

# Format code
fmt:
    cargo fmt

# Check formatting (CI-friendly)
fmt-check:
    cargo fmt -- --check

# Lint with clippy and fail on warnings
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Validate before merging/releasing (format, lint, tests, wasm build, transpile)
release-check:
    just fmt-check
    just clippy
    just test
    just build-wasm
    just transpile

# Show dependency tree (useful for diagnosing workspace deps)
tree:
    cargo tree --workspace

# Watch and test on change (requires cargo-watch)
watch-tests:
    if command -v cargo-watch >/dev/null 2>&1; then cargo watch -x 'test'; \
    else echo "cargo-watch not installed. Install with: cargo install cargo-watch" >&2; exit 1; fi

# List WIT interfaces in the repo
wit-list:
    find wit -maxdepth 1 -name '*.wit' -print | sort || true

# Print a WIT file (example: just wit-print world)
wit-print name:
    if [ -f "wit/{{ name }}.wit" ]; then cat "wit/{{ name }}.wit"; else echo "wit/{{ name }}.wit not found" >&2; exit 1; fi
