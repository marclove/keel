# Keel: Composable SaaS Architecture

> **Status**: Infrastructure development (Phase 2) with Spin Framework integration. This project is actively developed and welcomes community contributions.

> [!WARNING]
> Keel is pre-release, alpha-stage software. It's API is not stable and backward compatibility will not be
> provided until we reach stability. We want the freedom to cleanly and quickly rearchitect Keel until we've settled
> on stable patterns.

Keel is a composable SaaS architecture built on the [WASI Component Model](https://component-model.bytecodealliance.org/) to create reusable business capabilities. Build multiple SaaS products by composing business domain components, with infrastructure adapters that can be swapped via configuration.

## Spin Framework

We use the [Spin framework](https://spinframework.dev) as **our primary runtime** for building and running event-driven microservice applications composed of WebAssembly (Wasm) components. Keel leverages Spin's production-ready capabilities for immediate performance advantages and operational simplicity.

- Fast cold starts: Wasm is sandboxed, portable, and fast — millisecond cold start times remove the need to keep apps "warm".
- Language flexibility: Many languages have Wasm implementations; build components in the language you prefer.
- Open and portable: Spin is open source, under the CNCF, and built on standards, with implementations for local development, self-hosted servers, Kubernetes, and cloud-hosted services.
- **Production-ready infrastructure**: Spin's native SQLite provides 10x+ performance improvement over custom WASI components.

What Spin enables (non-exhaustive):
- Triggers: HTTP, Redis, Cron, with a simple trigger model for event-driven apps.
- Developer workflow: Creating apps, application structure, building, running, testing, observability, troubleshooting, runtime configuration.
- Feature APIs: Making HTTP requests, Key-Value store, SQLite database, MQTT messaging, Redis storage, relational databases, Serverless AI, variables.

These capabilities let us compose business components and attach the right triggers and platform features at the edges while keeping core logic portable.

## Quick Start

```bash
# Clone and setup
git clone https://github.com/marclove/keel.git
cd keel

# One-time setup
just init              # adds wasm target (wasm32-wasip2), installs JS deps
# Or do everything including Spin CLI (Homebrew if available)
just setup

# Build (native or WASM)
just build             # native workspace build
just build-wasm        # wasm32-wasip2 build

# Transpile WASM components to JS (via jco)
just transpile

# Run tests
just test              # workspace tests (unit + BDD)
# or target a single crate
just test-crate sql-sqlite

# E2E app (HTTP): build and run
just spin-build apps/e2e-keel
just spin-up apps/e2e-keel

# Smoke tests against the running E2E app
just e2e-smoke
```

## Spin Quickstart

Spin powers our event-driven Wasm apps. If the Spin CLI is installed and you have a Spin app directory (with `spin.toml`):

```bash
# Check Spin availability and version
just spin-check

# Scaffold a new app (example template)
just spin-new http-rust my-app
cd my-app

# Build and run locally
just spin-build .
just spin-up .

# Watch for changes
just spin-watch .

# Deploy to Fermyon Cloud
just spin-cloud-login
just spin-cloud-deploy .
```

Install Spin: https://developer.fermyon.com/spin/v2/index
Or run `just setup` to install prerequisites and Spin. On Linux, `setup`/`spin-install` will attempt to use your package manager (apt/dnf/yum/pacman/zypper) and the official installer script.
On Windows: run `just spin-install-windows` to see native Windows options; on WSL, use the Linux steps above.

## Architecture Overview

Keel organizes components into layers with clear separation of concerns:

```
Product Layer        → Complete SaaS applications
Business Domain      → Reusable business capabilities
Repository Layer     → Abstract data operations
Platform Services    → Cross-cutting concerns
Infrastructure       → Provider-specific adapters
```

Each layer uses WIT interfaces for strong contracts and hot-swappable implementations.

## Key Benefits

- **🚀 Rapid Development**: Build new SaaS products in weeks, not months
- **🔄 Hot-Swappable**: Change providers via configuration (PostgreSQL → SQLite)
- **📦 Edge-Native**: Single binary deployment to edge locations worldwide
- **🌐 Language Agnostic**: Components in Rust, Go, Python, or any WASI language
- **🧪 Test-Driven**: Comprehensive BDD testing at every layer
- **🏗️ Team Autonomy**: Independent component development with interface contracts

## Example: Email Service

```rust
// Business logic - no database or provider details
impl EmailService {
    fn send_welcome_email(email: String) -> Result<MessageId> {
        // Abstract business operations
        let user = user_repository::find_by_email(&email)?;
        let template = template_repository::get_template("welcome")?;

        // Platform services
        rate_limiting::check_limit(&user.id)?;

        // Infrastructure abstraction
        email_provider::send(user.email, template.render(user))?;

        Ok(message_id)
    }
}
```

Configuration determines implementation:
```toml
[infrastructure]
sql = "sql-sqlite.wasm"     # or "sql-postgres.wasm"
email = "email-sendgrid.wasm" # or "email-mailgun.wasm"

[repositories]
user-repository = "user-repository.wasm"
template-repository = "template-repository.wasm"
```

## Documentation

- **[Architecture Guide](ARCHITECTURE.md)** - Detailed technical architecture and design principles
- **[Scaling Architecture](SCALING.md)** - Global coordination, billion-user scale, and performance characteristics
- **[Roadmap](ROADMAP.md)** - Development phases and timeline
- **[FAQ](FAQ.md)** - Common questions about architecture, components, and development
- **[Contributing](CONTRIBUTING.md)** - How to contribute components and improvements
- **[Spin + Keel](SPIN.md)** - How we use Spin, local setup, and workflows

### Deployment Targets

In addition to generic Spin-compatible runtimes, we consider leveraging the following to accelerate delivery:
- [Fermyon Cloud](https://developer.fermyon.com/cloud/index) — managed hosting for Spin applications.
- [Fermyon Wasm Functions](https://developer.fermyon.com/wasm-functions/index) — function-level deployments that map well to Spin’s event-driven model.

These options provide much of the desired operational functionality out of the box and may help us get to market faster.

## Current Status

**Phase 2 (Infrastructure Layer)** - In Progress:
- ✅ Project structure and WIT interfaces  
- ✅ BDD testing framework
- ✅ Spin Framework integration foundation
- ✅ SQL Spin SQLite adapter (query/execute/transactions)
- 🚧 Complete infrastructure component suite

**Phase 3 (Platform Services)** - Next: Spin-native platform services (observability, security, rate-limiting)

See the [Roadmap](ROADMAP.md) for upcoming phases and target dates.

## Just Tasks

- help: List all available tasks.
- init: Install `wasm32-wasip2` and JS deps.
- setup: Run `init` and install Spin CLI (Homebrew if available), then verify with `spin-check`.
- build / build-release: Native builds (debug/release).
- build-wasm / build-wasm-release: WASM builds for `wasm32-wasip2`.
- transpile: Transpile WASM to JS using `jco`.
- test / test-crate <name> / test-integration: Run tests.
- fmt / fmt-check: Format or verify formatting.
- clippy: Lint all targets and deny warnings.
- clean / clean-wasm: Clean build artifacts.
- tree: Show workspace dependency tree.
- watch-tests: Re-run tests on change (requires `cargo-watch`).
- wit-list / wit-print <name>: Explore WIT files.

Spin helpers:
- spin-check: Verify Spin CLI is installed and print version.
- spin-new <template> <name>: Scaffold a new Spin app from a template.
- spin-build [dir='.']: Build a Spin app in a directory.
- spin-up [dir='.']: Run a Spin app locally.
- spin-watch [dir='.']: Watch a Spin app for changes.
- spin-cloud-login: Authenticate with Fermyon Cloud.
- spin-cloud-deploy [dir='.']: Deploy a Spin app to Fermyon Cloud.

E2E helpers:
- e2e-smoke: Runs setup, creates two users, lists users, then tests commit/rollback
- e2e-setup / e2e-user / e2e-users / e2e-txn-commit / e2e-txn-rollback: Individual endpoints

E2E app location:
- apps/e2e-keel (HTTP routes exercising SQLite flows)

## Community

- **GitHub Issues**: [Report bugs, request features, ask questions](https://github.com/marclove/keel/issues)
- **Architecture Discussions**: Use the `architecture` label for design discussions
- **Component Proposals**: Suggest new business domain components

## License

[MIT License](./LICENSE) - See license file for details.
