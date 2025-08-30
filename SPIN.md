# Spin + Keel

Spin is our runtime framework for building and running event‑driven microservice applications using WebAssembly (Wasm) components. It aligns with Keel’s WASI Component Model approach and lets us compose business capabilities behind portable, sandboxed components.

- Sandboxed, portable, fast: Wasm isolates components with millisecond cold starts.
- Language‑agnostic: Build components in Rust (our default) or other Wasm‑targeting languages.
- Open and portable: Spin is open source under the CNCF and runs locally, self‑hosted, on Kubernetes, and in managed clouds.

Useful references:
- Spin docs: https://spinframework.dev
- Spin v2 guide: https://developer.fermyon.com/spin/v2/index

---

## What Spin Enables

- Triggers: HTTP, Redis, Cron for event‑driven entry points.
- Feature APIs: HTTP client, Key‑Value store, SQLite DB, MQTT, Redis storage, relational databases, Serverless AI, variables.
- Dev workflow: Creating apps, application structure, building, running, testing, observability, troubleshooting, runtime configuration.

These map cleanly onto Keel’s layer model: expose business capabilities via triggers, use Feature APIs as platform services, and keep infrastructure swappable behind WIT contracts.

---

## Local Setup

Prerequisites:
- Rust toolchain with WASI target: `just init` (adds `wasm32-wasip2`, installs JS deps)
- Spin CLI

Install Spin CLI (choose one; see official guide for your OS):
- macOS (Homebrew):
  ```bash
  brew tap fermyon/tap
  brew install spin
  ```
- Linux (APT/DNF/YUM/Pacman/Zypper):
  ```bash
  # Our just task will install curl (via your package manager) and run the official installer
  just spin-install
  # or manually:
  curl -fsSL https://developer.fermyon.com/downloads/spin/install.sh | bash
  ```
- Windows:
  - Native Windows: run `just spin-install-windows` to print options. Use winget or Chocolatey if available, or follow the official guide.
  - WSL: follow the Linux instructions above (APT/DNF/etc. + installer script).
  - Docs: https://developer.fermyon.com/spin/v2/install
- Official installer and other OS options: https://developer.fermyon.com/spin/v2/install

Verify installation:
```bash
just spin-check    # prints Spin version if installed
```

All-in-one setup:
```bash
# Installs Rust WASI target, JS deps, and Spin (via Homebrew if available)
just setup
```

---

## Create and Run a Spin App

Scaffold a new app (example uses the HTTP Rust template):
```bash
just spin-new http-rust my-app
cd my-app
```

Build and run locally:
```bash
just spin-build .
just spin-up .
# or watch for changes
just spin-watch .
```

Deploy to Fermyon Cloud:
```bash
just spin-cloud-login
just spin-cloud-deploy .
```

---

## Example spin.toml (Minimal HTTP)

```toml
spin_version = "2"
name = "example"
version = "0.1.0"
trigger = { type = "http", base = "/" }

[[component]]
id = "example"
source = "target/wasm32-wasip2/release/example.wasm"
[component.trigger]
route = "/..."
[component.build]
# For Rust components, a common pattern is using cargo build steps
command = "cargo build --target wasm32-wasip2 --release"
```

Notes:
- For Keel, components should implement WIT contracts. The `source` should point at the built Wasm artifact for that component.
- Use the `trigger` section to bind HTTP/Redis/Cron entry points to the component’s exported operations.

---

## Using Spin with Keel

- WIT contracts: Keep business interfaces stable; map trigger inputs to WIT world functions.
- Feature APIs: Prefer Spin’s built‑ins (HTTP client, KV, SQLite, Redis, etc.) over bespoke plumbing when possible.
- Configuration: Use Spin variables and runtime configuration for environment‑specific values and secrets.
- Observability: Leverage Spin’s logging/metrics hooks to feed into Keel’s Platform Services layer.
- Testing: Combine `cargo test` for logic with `spin up` for endpoint/integration checks.

---

## Deployment Options

- Fermyon Cloud: Managed hosting for Spin apps (fast path to production). Use `just spin-cloud-login` and `just spin-cloud-deploy .`.
- Fermyon Wasm Functions: Function‑level deployment aligned with event‑driven models. See https://developer.fermyon.com/wasm-functions/index
- Other runtimes: Spin apps are portable; self‑host or run on Kubernetes as needed.

Keep portability in mind: avoid provider‑specific assumptions in business components; encapsulate them behind WIT + Spin Feature APIs.

---

## Troubleshooting

- Spin not found: Install via the Spin docs, then run `just spin-check`.
- Build errors: Ensure `wasm32-wasip2` target is installed (`just init`) and your Rust toolchain is up‑to‑date.
- Port conflicts: `spin up` defaults to port 3000; set `SPIN_HTTP_LISTEN_ADDR` or adjust your config if needed.
- Environment: Use Spin variables and `.env` files per the docs for local secrets.

If issues persist, capture `spin up` logs and open a ticket with steps to reproduce.
