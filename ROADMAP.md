# Keel Roadmap

> **Status**: This roadmap is actively maintained and updated as the project evolves. The Keel project is in early development (Phase 1).

This roadmap outlines the planned development phases for the Keel composable SaaS architecture. Like the [WASI roadmap](https://wasi.dev/), our development is milestone-driven with incremental deliveries.

## Current Milestone: Foundation (Phase 1)

**Goal**: Establish the core component model architecture and prove it works with real databases.

### Completed
- [x] Project structure and Cargo workspace setup
- [x] BDD testing framework using cucumber-rs
- [x] Generic WIT interfaces for `sql` and `kv` adapters
- [x] Component compilation and WIT binding generation
- [x] SQLite and RocksDB integration as test dependencies
- [x] Architecture documentation and roadmap

### In Progress
- [ ] Real SQLite integration in BDD tests
- [ ] Complete sql-sqlite component implementation
- [ ] Complete kv-rocksdb component implementation

### Target Completion
Foundation Phase complete with working infrastructure adapters.

---

## Next Milestone: Infrastructure Layer (Phase 2)

**Goal**: Complete infrastructure adapter ecosystem with multiple provider options.

### Planned Components

#### SQL Adapters
- [ ] `sql-sqlite` - SQLite database adapter (in progress)
- [ ] `sql-postgres` - PostgreSQL database adapter
- [ ] `sql-mysql` - MySQL database adapter

#### Key-Value Adapters
- [ ] `kv-rocksdb` - RocksDB adapter (in progress)
- [ ] `kv-redis` - Redis adapter
- [ ] `kv-memory` - In-memory adapter for testing

#### Communication Adapters
- [ ] `email-sendgrid` - SendGrid email provider
- [ ] `email-mailgun` - Mailgun email provider
- [ ] `email-ses` - AWS SES email provider

#### Authentication Adapters
- [ ] `auth-okta` - Okta authentication provider
- [ ] `auth-auth0` - Auth0 authentication provider
- [ ] `auth-local` - Local authentication for development

### Success Criteria
- All adapters pass comprehensive BDD test suites
- Hot-swapping between providers works via configuration
- Components compile to efficient WASM modules (<1MB each)
- Documentation and examples for each adapter

### Target Completion
Complete infrastructure adapter ecosystem.

---

## Future Milestone: Platform Services Layer (Phase 3)

**Goal**: Cross-cutting concerns that provide observability, security, and operational capabilities.

### Planned Components

#### Observability
- [ ] `observability` - Unified tracing, metrics, and logging
  - Integration with OpenTelemetry standards
  - Support for multiple backends (Jaeger, Prometheus, etc.)

#### Security & Compliance
- [ ] `security-context` - Authentication, authorization, and audit
  - Role-based access control (RBAC)
  - Audit logging and compliance reporting
- [ ] `rate-limiting` - Request throttling and abuse prevention

#### Operational
- [ ] `feature-flags` - A/B testing and gradual rollouts
- [ ] `configuration` - Dynamic configuration management

### Success Criteria
- Platform services integrate seamlessly with business components
- Security auditing meets compliance requirements (SOC 2, GDPR)
- Observability provides production-ready monitoring
- Feature flags enable safe deployments

### Target Completion
Production-ready platform services.

---

## Future Milestone: Repository Layer (Phase 4)

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

### Success Criteria
- Business logic contains zero SQL strings or database-specific code
- Repositories are database-agnostic (work with any SQL adapter)
- Comprehensive test coverage with both real and mock databases
- Performance benchmarks meet SaaS application requirements

### Target Completion
Complete repository abstraction layer.

---

## Future Milestone: Business Domain Layer (Phase 5)

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

## Future Milestone: Product Composition (Phase 6)

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

| Phase | Focus | Target | Status |
|-------|-------|--------|---------|
| 1 | Foundation & Architecture | Q1 2025 | 🚧 In Progress |
| 2 | Infrastructure Adapters | Q2 2025 | 📋 Planned |
| 3 | Platform Services | Q3 2025 | 📋 Planned |
| 4 | Repository Layer | Q4 2025 | 📋 Planned |
| 5 | Business Domains | Q1 2026 | 📋 Planned |
| 6 | Product Composition | Q2 2026 | 📋 Planned |
| 7+ | Advanced Features | 2026+ | 💭 Vision |

---

> **Last Updated**: August 2025
> **Maintainers**: [Project Team](https://github.com/marclove/keel/blob/main/MAINTAINERS.md)
