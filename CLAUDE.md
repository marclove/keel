# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Keel is a composable SaaS architecture project using the WASI Component Model to create reusable business capabilities. The goal is to build a suite of infrastructure and business domain components that can be composed to rapidly develop new SaaS applications.

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

**Current Phase**: Phase 1 (Foundation) - Recently completed with Phase 2 (Infrastructure Layer) in progress.

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
