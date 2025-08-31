set shell := ["zsh", "-eu", "-o", "pipefail", "-c"]

# Default base URL for the E2E Spin app

e2e_url := "http://127.0.0.1:3000"

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

# Install Spin CLI if missing (prefers Homebrew on macOS)
spin-install:
    if command -v spin >/dev/null 2>&1; then \
      echo "Spin already installed: $$(spin --version)"; \
    elif command -v brew >/dev/null 2>&1; then \
      brew tap fermyon/tap && brew install spin; \
    elif command -v apt-get >/dev/null 2>&1; then \
      sudo apt-get update && sudo apt-get install -y curl ca-certificates; \
      curl -fsSL https://developer.fermyon.com/downloads/spin/install.sh | bash; \
    elif command -v dnf >/dev/null 2>&1; then \
      sudo dnf install -y curl; \
      curl -fsSL https://developer.fermyon.com/downloads/spin/install.sh | bash; \
    elif command -v yum >/dev/null 2>&1; then \
      sudo yum install -y curl; \
      curl -fsSL https://developer.fermyon.com/downloads/spin/install.sh | bash; \
    elif command -v pacman >/dev/null 2>&1; then \
      sudo pacman -Sy --noconfirm curl; \
      curl -fsSL https://developer.fermyon.com/downloads/spin/install.sh | bash; \
    elif command -v zypper >/dev/null 2>&1; then \
      sudo zypper install -y curl; \
      curl -fsSL https://developer.fermyon.com/downloads/spin/install.sh | bash; \
    else \
      echo "Spin CLI not found and Homebrew unavailable." >&2; \
      echo "If on Linux/WSL, install curl and run: curl -fsSL https://developer.fermyon.com/downloads/spin/install.sh | bash" >&2; \
      echo "On native Windows, use winget/choco or see the official guide: https://developer.fermyon.com/spin/v2/install" >&2; \
      exit 1; fi

# One-shot setup: toolchains, JS deps, and Spin CLI
setup:
    just init
    just spin-install
    just spin-check
    just spin-install-plugins

# Print Windows install instructions (native Windows / PowerShell)
spin-install-windows:
    echo "Spin installation on Windows (native):"; \
    echo "- Recommended: follow the official guide: https://developer.fermyon.com/spin/v2/install"; \
    echo "- If you use winget, try:   winget install Fermyon.Spin   (or search: winget search Spin)"; \
    echo "- If you use Chocolatey, try:   choco install spin   (package availability may vary)"; \
    echo "- Alternatively, download the latest MSI from the releases linked in the docs."; \
    echo "WSL users: run 'just spin-install' from WSL to use the Linux path."

spin-install-plugins:
    echo "Installing the Fermyon Wasm Functions for Akamai Spin plugin to let us"; \
    echo "to interact with Fermyon Wasm Functions (Login, Deploy, etc.)"
    spin plugin install aka
    spin plugin install pluginify
    spin plugin install -u https://github.com/spinframework/spin-test/releases/download/canary/spin-test.json

spin-login:
    spin login

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

# Generates JavaScript + TypeScript definitions for edge deployment
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

# -------------------------
# Spin framework helpers
# -------------------------

# Verify Spin CLI is installed
spin-check:
    if command -v spin >/dev/null 2>&1; then spin --version; \
    else echo "Spin CLI not found. Install from https://developer.fermyon.com/spin/v2/index" >&2; exit 1; fi

# -------------------------
# E2E helper commands
# -------------------------

# Run all E2E curls in sequence (requires the app running via `just spin-up apps/e2e-keel` in another terminal)
e2e-smoke:
    echo "POST /setup";
    curl -fsS -X POST "{{ e2e_url }}/setup" | sed 's/.*/\n&\n/'
    echo "POST /users (Alice)";
    curl -fsS -X POST "{{ e2e_url }}/users" -H 'content-type: application/json' -d '{"name":"Alice","email":"a@example.com"}' | sed 's/.*/\n&\n/'
    echo "POST /users (Bob)";
    curl -fsS -X POST "{{ e2e_url }}/users" -H 'content-type: application/json' -d '{"name":"Bob","email":"b@example.com"}' | sed 's/.*/\n&\n/'
    echo "GET /users";
    curl -fsS "{{ e2e_url }}/users" | sed 's/.*/\n&\n/'
    echo "POST /txn/commit";
    curl -fsS -X POST "{{ e2e_url }}/txn/commit" | sed 's/.*/\n&\n/'
    echo "POST /txn/rollback";
    curl -fsS -X POST "{{ e2e_url }}/txn/rollback" | sed 's/.*/\n&\n/'

# POST /setup only
e2e-setup:
    curl -fsS -X POST "{{ e2e_url }}/setup"

# POST /users with params: just e2e-user "Alice" "a@example.com"
e2e-user name email:
    curl -fsS -X POST "{{ e2e_url }}/users" -H 'content-type: application/json' -d '{"name":"{{ name }}","email":"{{ email }}"}'

# GET /users
e2e-users:
    curl -fsS "{{ e2e_url }}/users"

# POST /txn/commit
e2e-txn-commit:
    curl -fsS -X POST "{{ e2e_url }}/txn/commit"

# POST /txn/rollback
e2e-txn-rollback:
    curl -fsS -X POST "{{ e2e_url }}/txn/rollback"

# Create a new Spin app from a template

# Usage: just spin-new http-rust my-app
spin-new template name:
    if command -v spin >/dev/null 2>&1; then \
      spin new {{ template }} {{ name }}; \
    else echo "Spin CLI not found. Install from https://developer.fermyon.com/spin/v2/index" >&2; exit 1; fi

# Build a Spin app in a directory (default '.')
spin-build dir='.':
    if command -v spin >/dev/null 2>&1; then \
      (cd {{ dir }} && spin build); \
    else echo "Spin CLI not found. Install from https://developer.fermyon.com/spin/v2/index" >&2; exit 1; fi

# Run a Spin app locally (default '.')
spin-up dir='.':
    if command -v spin >/dev/null 2>&1; then \
      (cd {{ dir }} && spin up); \
    else echo "Spin CLI not found. Install from https://developer.fermyon.com/spin/v2/index" >&2; exit 1; fi

# Watch a Spin app for changes and rebuild/restart
spin-watch dir='.':
    if command -v spin >/dev/null 2>&1; then \
      (cd {{ dir }} && spin watch); \
    else echo "Spin CLI not found. Install from https://developer.fermyon.com/spin/v2/index" >&2; exit 1; fi

# Login to Fermyon Cloud (tries modern and legacy commands)
spin-cloud-login:
    if command -v spin >/dev/null 2>&1; then \
      if spin help login >/dev/null 2>&1; then spin login; \
      elif spin cloud --help >/dev/null 2>&1; then spin cloud login; \
      else echo "Spin installed but no cloud login command found." >&2; exit 1; fi; \
    else echo "Spin CLI not found. Install from https://developer.fermyon.com/spin/v2/index" >&2; exit 1; fi

# Deploy a Spin app to Fermyon Cloud (tries modern and legacy commands)
spin-cloud-deploy dir='.':
    if command -v spin >/dev/null 2>&1; then \
      if spin help deploy >/dev/null 2>&1; then (cd {{ dir }} && spin deploy); \
      elif spin cloud --help >/dev/null 2>&1; then (cd {{ dir }} && spin cloud deploy); \
      else echo "Spin installed but no deploy command found." >&2; exit 1; fi; \
    else echo "Spin CLI not found. Install from https://developer.fermyon.com/spin/v2/index" >&2; exit 1; fi
