# Keel Roadmap

> **Status**: This roadmap is actively maintained and updated as the project evolves. The Keel project is in early development (Phase 2).

This roadmap outlines the planned development phases for the Keel composable SaaS architecture. Like the [WASI roadmap](https://wasi.dev/), our development is milestone-driven with incremental deliveries.

## Foundation (Phase 1) - COMPLETE
- [x] Project structure and Cargo workspace setup
- [x] BDD testing framework using cucumber-rs
- [x] Generic WIT interfaces for `sql` and `kv` adapters
- [x] Component compilation and WIT binding generation
- [x] Architecture documentation and roadmap

---

## Infrastructure Layer (Phase 2)

- [ ] Complete sql-spin-sqlite adapter implementing sql.wit interface via Spin Framework
- [ ] Complete kv-rocksdb component implementation
- [ ] Spin Framework integration and configuration
- [ ] Full WASM compilation pipeline (wasm32-wasip2 target)
- [ ] JavaScript transpilation via jco toolchain
- [ ] TypeScript definitions for edge deployment
- [ ] BDD test integration with WASM components
- [ ] Single binary deployment capability via Spin applications
- [ ] Fermyon Cloud deployment pipeline

### Planned Components

#### SQL Adapters
- [ ] `sql-spin-sqlite` - Spin Framework SQLite adapter implementing sql.wit interface
- [ ] `sql-postgres` - PostgreSQL database adapter (deferred to Phase 3+)
- [ ] `sql-mysql` - MySQL database adapter (deferred to Phase 3+)

#### Key-Value Adapters
- [ ] `kv-memory` - In-memory adapter for testing
- [ ] `kv-rocksdb` - RocksDB adapter
- [ ] `kv-redis` - Redis adapter

#### Communication Adapters
- [ ] `email-sendgrid` - SendGrid email provider
- [ ] `email-mailgun` - Mailgun email provider
- [ ] `email-ses` - AWS SES email provider

#### Authentication Adapters
- [ ] `auth-okta` - Okta authentication provider
- [ ] `auth-auth0` - Auth0 authentication provider
- [ ] `auth-local` - Local authentication for development

### Technical Approach
Phase 2 focuses on implementing WIT interface adapters that leverage Spin Framework's production-ready capabilities rather than building custom WASI components from scratch. The sql-spin-sqlite adapter provides immediate production capability with proven performance characteristics.

### Target Completion
Infrastructure Phase complete with Spin Framework integration and production-ready WASM components.

---

## Platform Services Layer (Phase 3)

### Planned Components

#### Observability
- [ ] `spin-observability` - Unified tracing, metrics, and logging using Spin Framework's built-in telemetry APIs
  - Integration with Spin's OpenTelemetry support
  - Leverage Spin's native observability hooks
  - Support for Spin-compatible backends

#### Security & Compliance
- [ ] `spin-security-context` - Authentication, authorization, and audit using Spin's security patterns
  - Role-based access control (RBAC) via Spin middleware
  - Audit logging through Spin's logging framework
  - Integration with Spin's authentication triggers
- [ ] `spin-rate-limiting` - Request throttling and abuse prevention via Spin middleware patterns

#### Operational
- [ ] `spin-feature-flags` - A/B testing and gradual rollouts through Spin configuration and variables
- [ ] `spin-configuration` - Dynamic configuration management using Spin's variable system

#### Additional Spin-Native Services
- [ ] `spin-http-client` - HTTP client abstraction using Spin's outbound HTTP capabilities
- [ ] `spin-messaging` - Message queuing via Spin's Redis and MQTT support

### Technical Approach
Platform services leverage Spin Framework's mature middleware patterns, configuration system, and built-in APIs rather than implementing custom WASI solutions. This provides immediate production readiness and operational best practices.

### Target Completion
Production-ready platform services built on Spin Framework foundations.

---

## Repository Layer (Phase 4)

**Goal**: Abstract business data operations away from infrastructure concerns.

### Planned Components

#### Core Repositories
- [ ] `user-repository` - User management and profiles
- [ ] `workspace-repository` - Multi-tenant workspace data
- [ ] `template-repository` - Email and notification templates
- [ ] `billing-repository` - Subscription and usage data

### WIT Interface Design
Each repository will expose business-domain operations:

```wit
interface user-repository {
    // Business operations, not SQL
    find-by-email: func(email: string) -> result<user, user-error>
    create-user: func(registration: user-registration) -> result<user-id, user-error>
    update-preferences: func(user-id: user-id, prefs: user-preferences) -> result<_, user-error>
    deactivate-user: func(user-id: user-id, reason: deactivation-reason) -> result<_, user-error>
}
```

### Technical Approach
Repositories implement business-domain operations using the sql.wit interface, with persistence provided by the sql-spin-sqlite adapter. This maintains database-agnostic business logic while leveraging Spin Framework's production-ready SQLite capabilities.

### Success Criteria
- Business logic contains zero SQL strings or database-specific code
- Repositories work with any SQL adapter implementing sql.wit interface
- Comprehensive test coverage with both Spin SQLite and mock databases
- Repositories leverage Spin's connection pooling and transaction management

### Target Completion
Complete repository abstraction layer with Spin SQLite integration.

---

## Business Domain Layer (Phase 5)

**Goal**: Reusable business capabilities that can be composed into different SaaS products.

### Core Business Domains

#### Communication & Engagement
- [ ] `email-service` - Transactional and marketing email
- [ ] `notification-orchestration` - Multi-channel notifications
- [ ] `real-time-messaging` - Chat and messaging capabilities

#### User & Workspace Management
- [ ] `user-lifecycle` - Onboarding, activation, retention
- [ ] `workspace-management` - Multi-tenant workspace operations
- [ ] `team-collaboration` - Shared workspaces and permissions

#### Commerce & Billing
- [ ] `subscription-billing` - Recurring subscription management
- [ ] `usage-billing` - Metered usage and billing
- [ ] `payment-processing` - Payment method and transaction handling

#### Content & Media
- [ ] `document-management` - File storage and collaboration
- [ ] `live-streaming` - Video streaming and recording
- [ ] `content-moderation` - Automated and manual content review

### Success Criteria
- Business domains are product-agnostic and reusable
- Each domain has comprehensive API documentation
- Performance meets production SaaS requirements
- Business domains integrate cleanly with platform services

### Target Completion
Core business domain library complete.

---

## Product Composition (Phase 6)

**Goal**: Demonstrate the architecture by building complete SaaS products.

### Target Products

#### Team Collaboration SaaS
A Slack/Discord competitor demonstrating:
- Real-time messaging using `real-time-messaging`
- User management using `user-lifecycle` and `workspace-management`
- File sharing using `document-management`
- Notifications using `notification-orchestration`

#### E-learning Platform
A Teachable competitor demonstrating:
- Course content using `document-management`
- Live classes using `live-streaming`
- Student progress using custom domain logic
- Billing using `subscription-billing`

#### Live Streaming Platform
A Twitch competitor demonstrating:
- Video streaming using `live-streaming`
- Chat integration using `real-time-messaging`
- Content moderation using `content-moderation`
- Creator monetization using `usage-billing`

### Success Criteria
- Products launch with <2 weeks development time
- 80%+ code reuse between products
- Single binary edge deployment (<100MB)
- Production performance (sub-100ms response times)

### Target Completion
First product suite launched.

---

## Technical Decisions

### Runtime Framework: Spin Framework

Keel leverages the **Spin Framework** as the primary runtime for building and deploying WASM components. This decision provides:

- **Production-ready infrastructure**: Spin's mature SQLite, HTTP, and middleware capabilities
- **Edge deployment**: Single binary applications deployable to edge locations
- **Standards compliance**: Built on WASI Component Model with portability guarantees
- **Operational simplicity**: Unified development, testing, and deployment workflows

### Deployment Architecture: SpinKube + Linode LKE

- **SpinKube**: Kubernetes operator for Spin applications, enabling enterprise deployment patterns
- **Linode LKE**: Kubernetes clusters providing regional deployment with transparent pricing and clear CLI
- **Edge distribution**: Multiple regional clusters with geo-aware DNS routing

### Database Strategy: Spin Framework SQLite

- **Primary database**: Spin's native SQLite support via `spin:sqlite` interface
- **Managed service**: Fermyon Cloud's managed SQLite with Turso integration
- **Edge optimization**: Co-located database access eliminating network latency
- **Global sync**: Distributed SQLite with eventual consistency for multi-region deployments

### Component Implementation Approach

- **WIT interfaces**: All components implement standard WIT contracts for interoperability
- **Hybrid architecture**: Leverage Spin's native capabilities where available, implement WIT adapters where needed
- **Runtime composability**: Components configured via Spin's configuration system
- **Single binary deployment**: All components packaged into Spin applications

---

## Long-term Vision

### Advanced Runtime Features
- **Hot Component Swapping** - Update business logic without restarts
- **A/B Testing Integration** - Component-level feature flags
- **Multi-Region Deployment** - Edge-optimized component distribution
- **Component Marketplace** - Third-party business domain components

### Ecosystem Growth
- **Community Contributions** - Open-source business domain components
- **Enterprise Features** - Advanced security, compliance, and governance
- **Developer Tools** - IDE integrations, debugging, and profiling
- **Training & Certification** - Component development best practices

### Performance & Scale
- **Sub-10ms Cold Starts** - Optimized WASM runtime performance
- **Automatic Scaling** - Component-aware horizontal scaling
- **Edge Intelligence** - AI-driven component optimization
- **Cost Optimization** - Pay-per-component usage models

---

## Contributing to the Roadmap

We welcome community input on this roadmap! Here's how to get involved:

### Feedback & Discussion
- **GitHub Issues**: Report bugs, request features, or ask questions
- **Architecture Discussions**: Open issues tagged with `architecture`
- **Component Proposals**: Suggest new business domain components

### Development Contributions
- **Component Development**: Help build infrastructure and business components
- **Testing**: Improve BDD test coverage and integration testing
- **Documentation**: Enhance architecture docs and component examples
- **Performance**: Optimize component compilation and runtime performance

### Community Guidelines
- Follow the [WASI Community Group](https://www.w3.org/community/webassembly/) standards for respectful collaboration
- All contributions are reviewed via pull requests
- Component designs should be discussed in GitHub issues before implementation
- Maintain backward compatibility in WIT interface changes

---

## Milestones Summary

| Phase | Focus | Status |
|-------|-------|--------|
| 1 | Foundation & Architecture | ✅ Complete |
| 2 | Infrastructure Adapters (Spin Framework Integration) | 🚧 In Progress |
| 3 | Platform Services (Spin-Native) | 📋 Planned |
| 4 | Repository Layer (Spin SQLite) | 📋 Planned |
| 5 | Business Domains | 📋 Planned |
| 6 | Product Composition | 📋 Planned |
| 7+ | Advanced Features | 💭 Vision |

---

> **Last Updated**: August 2025
> **Maintainers**: [Project Team](https://github.com/marclove/keel/blob/main/MAINTAINERS.md)
