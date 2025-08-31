# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Keel is a composable SaaS architecture project using the WASI Component Model to create reusable business capabilities. The goal is to build a suite of infrastructure and business domain components that can be composed to rapidly develop new SaaS applications.

> [!WARNING]
> Keel is pre-release, alpha-stage software. It's API is not stable and backward compatibility will not be
> provided until we reach stability. We want the freedom to cleanly and quickly rearchitect Keel until we've settled
> on stable patterns.

## Spin Framework

We use the [Spin framework](https://spinframework.dev) to build and run event-driven microservice applications composed of WebAssembly (Wasm) components.

- Wasm advantages: sandboxed, portable, and fast; millisecond cold starts remove the need to keep applications “warm”.
- Language choice: many languages have Wasm implementations so developers don’t need to learn new languages or libraries.
- Open and portable: Spin is open source under the CNCF and built on standards, with implementations for local development, self-hosted servers, Kubernetes, and cloud-hosted services.

When designing or modifying components, consider how Spin’s capabilities map to Keel’s layers:
- Triggers: HTTP, Redis, Cron for event-driven entry points.
- Developer workflow: creating apps, application structure, building, running, testing, observability, troubleshooting, runtime configuration.
- Feature APIs: HTTP requests, Key-Value store, SQLite, MQTT, Redis storage, relational databases, Serverless AI, variables.

**Keel leverages Spin Framework as our primary runtime**, using Spin's production-ready SQLite, middleware patterns, and deployment capabilities rather than building custom WASI components from scratch. Prefer Spin's native capabilities over bespoke implementations while maintaining WIT interface contracts for component composability.

## Architecture Philosophy

The project follows a hierarchical component model with multiple layers:

1. **Product Layer** - Individual SaaS applications (team-collaboration, live-streaming, e-learning)
2. **Business Domain Layer** - Reusable business capabilities (subscription-billing, notification-orchestration, user-lifecycle, collaborative-docs)
3. **Platform Services Layer** - Cross-cutting concerns (observability, security-context, rate-limiting, feature-flags)
4. **Infrastructure Layer** - Provider adapters (sql-sqlite, kv-rocksdb, email-sendgrid, auth-okta)

## Key Design Principles

- **Composable Business Objects**: Focus on reusable business capabilities rather than just infrastructure
- **Strong Separation of Concerns**: Infrastructure and business logic developed independently with WIT interface contracts
- **Runtime Composability**: Components can be hot-swapped through configuration without recompilation
- **Edge-First Deployment**: Single binary deployments to edge locations, avoiding distributed systems complexity
- **Language Agnostic**: Components can be written in any WASI-compatible language

## Component Organization

Components are organized by business domain rather than technical function:
- Business exports should represent business capabilities (order-processing, customer-management)
- Infrastructure imports provide platform services (auth, sql, email, logging)
- Platform services handle cross-cutting concerns (observability, security, rate-limiting)

## WIT Interface Design

Interfaces should:
- Represent business capabilities, not technical services
- Provide strong contracts that can be versioned
- Enable provider swapping through configuration
- Support both synchronous and asynchronous operations where appropriate

## Development Status

**Current Phase**: Phase 2 (Infrastructure Layer) - In progress with Spin Framework integration focus.

This project uses WASI 0.2 and Component Model for production-ready composable architecture. The toolchain is stable and components can be built today. Components can be transpiled to JavaScript using jco for deployment to platforms like Cloudflare Workers.

**Timeline**: Targeting production readiness with multiple SaaS products demonstrating the architecture.

## Documentation

The project has comprehensive documentation organized by topic:

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Primary technical specification with detailed component hierarchy, layer definitions, and implementation patterns
- **[SCALING.md](SCALING.md)** - Global coordination architecture for billion-user scale, edge deployment philosophy, and performance characteristics
- **[ROADMAP.md](ROADMAP.md)** - Milestone-driven development plan with current status and timeline
- **[FAQ.md](FAQ.md)** - Common questions about architecture, components, and development approach
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guidelines, testing standards, and WIT interface design principles

**Key Focus Areas:**
- Component-based architecture using WASI Component Model
- Edge-native deployment (single binary with WASM components)
- Provider abstraction through Repository Layer pattern
- BDD testing at every architectural layer
- Global coordination while maintaining edge performance

## Deployment Targets

In addition to generic Spin-compatible runtimes, we consider using:
- [Fermyon Cloud](https://developer.fermyon.com/cloud/index) for managed Spin app hosting.
- [Fermyon Wasm Functions](https://developer.fermyon.com/wasm-functions/index) for function-level deployments aligned with Spin’s event-driven model.

Agents should keep these targets in mind when making trade-offs (e.g., storage choices, trigger selection) to shorten time-to-market.

## Dev Workflow (For Agents)

- Use `just` to run common tasks. Start with `just help` to list tasks.
- Setup: `just init` (adds `wasm32-wasip2` target, installs JS deps).
- Build: `just build` (native) or `just build-wasm` (WASI preview2).
- Transpile: `just transpile` to generate JS from built WASM via `jco`.
- Test: `just test` for the whole workspace or `just test-crate <name>`.
- Lint/Format: `just clippy`, `just fmt`, `just fmt-check`.
- Release sanity: `just release-check` (fmt-check, clippy, tests, wasm build, transpile).

Notes for code changes:
- Keep changes minimal and focused on requested scopes.
- Align with WIT contracts and layer boundaries.
- Prefer adding tasks to `justfile` for repeatable workflows.
