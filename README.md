# Keel: Composable SaaS Architecture

> **Status**: Early development (Phase 1). This project is actively developed and welcomes community contributions.

Keel is a composable SaaS architecture built on the [WASI Component Model](https://component-model.bytecodealliance.org/) to create reusable business capabilities. Build multiple SaaS products by composing business domain components, with infrastructure adapters that can be swapped via configuration.

## Quick Start

```bash
# Clone and setup
git clone https://github.com/your-org/keel.git
cd keel

# One-time setup
just init              # adds wasm target (wasm32-wasip2), installs JS deps

# Build (native or WASM)
just build             # native workspace build
just build-wasm        # wasm32-wasip2 build

# Transpile WASM components to JS (via jco)
just transpile

# Run tests
just test              # workspace tests (unit + BDD)
# or target a single crate
just test-crate sql-sqlite
```

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
sql = "sql-postgres.wasm"     # or "sql-sqlite.wasm" 
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

## Current Status

**Phase 1 (Foundation)** - In Progress:
- ✅ Project structure and WIT interfaces
- ✅ BDD testing framework
- ✅ Basic SQL and KV components
- 🚧 SQLite and RocksDB integration

See the [Roadmap](ROADMAP.md) for upcoming phases and target dates.

## Just Tasks

- help: List all available tasks.
- init: Install `wasm32-wasip2` and JS deps.
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

## Community

- **GitHub Issues**: [Report bugs, request features, ask questions](https://github.com/your-org/keel/issues)
- **Architecture Discussions**: Use the `architecture` label for design discussions
- **Component Proposals**: Suggest new business domain components

## License

[MIT License](LICENSE) - See license file for details.

---

Built with ❤️ using the [WASI Component Model](https://component-model.bytecodealliance.org/).
