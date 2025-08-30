# Keel: Frequently Asked Questions

> **Note**: This FAQ is actively maintained and updated as the project evolves. If you have questions not covered here, please [open a GitHub issue](https://github.com/marclove/keel/issues) or contribute to our documentation.

This document addresses common questions about the Keel composable SaaS architecture. For detailed technical specifications, see our [Architecture Documentation](ARCHITECTURE.md).

## Table of Contents

- [Architecture & Design](#architecture--design)
- [Edge Deployment](#edge-deployment)
- [Component Model](#component-model)
- [Development & Testing](#development--testing)
- [Comparison to Alternatives](#comparison-to-alternatives)

**See also:** [Scaling Architecture](SCALING.md) for comprehensive coverage of performance, scale, and global coordination.

---

## Architecture & Design

### Q: How is this different from traditional microservices?

Keel provides **microservices architecture with monolithic deployment**. You get the modularity, team autonomy, and technology diversity of microservices with the performance and operational simplicity of a single binary.

**Traditional Microservices:**
```
┌─────────────┐    HTTP    ┌─────────────┐
│ User Service│◄──────────►│Auth Service │
│             │            │             │
└─────────────┘            └─────────────┘
  Container A                Container B
```

**Keel Components:**
```
┌─────────────────────────────────────┐
│        Single Binary              │
│  ┌─────────────┐  ┌─────────────┐   │
│  │user.wasm    │  │auth.wasm    │   │
│  │             │  │             │   │
│  └─────────────┘  └─────────────┘   │
└─────────────────────────────────────┘
   Function calls (nanoseconds)
```

**Benefits:**
- No network latency between components
- No service mesh complexity
- Easier debugging and observability
- Better resource utilization
- Simpler deployment and operations

**You still get:**
- Component isolation via WASM sandboxing
- Independent team development
- Technology diversity (any WASI-compatible language)
- Hot-swappable components via configuration

### Q: Why layer the architecture this way instead of just having business logic components?

The layered architecture prevents business logic contamination and enables true reusability:

**Without layers (problematic):**
```rust
// Business logic contains database details
impl EmailService {
    fn send_email(&self, to: String) -> Result<()> {
        // 🚫 SQL in business logic!
        let user = sqlx::query!("SELECT * FROM users WHERE email = ?", to)
            .fetch_one(&self.db).await?;

        // 🚫 Provider-specific code!
        sendgrid::send_email(user.email, template).await?;
    }
}
```

**With layers (clean):**
```rust
// Business logic uses abstract operations
impl EmailService {
    fn send_email(&self, to: String, template: String) -> Result<()> {
        // ✅ Business operations only
        let user = user_repository::find_by_email(&to)?;
        let template_data = template_repository::get_template(&template)?;

        // ✅ Abstract email sending
        email_provider::send(user.email, template_data)?;
    }
}
```

This separation means:
- Email service works with SQLite, PostgreSQL, or any SQL adapter
- Email service works with SendGrid, Mailgun, or any email provider
- Same email service can be reused in team-collab, e-learning, and streaming products

---

## Edge Deployment

### Q: Isn't edge deployment supposed to be lots of small microservices?

No! This is a common misconception. **Edge computing** and **microservices** solve different problems:

**Edge Computing Goal:** Reduce latency by running code closer to users
**Microservices Goal:** Enable team autonomy and independent scaling

Traditional "edge microservices" actually work against edge constraints:

**Edge Constraints:**
- Limited compute and memory resources
- Network unreliability between services
- Operational complexity of running distributed systems

**Keel's Edge-Native Approach:**
- Single binary deploys to hundreds of edge locations
- All "services" are components running in one process
- No network calls between business logic
- Operational simplicity (one process to monitor)

**Real-World Analogy:** A smartphone contains multiple "services" (camera, GPS, messaging) with clear interfaces and isolation, but they run in one device, not distributed across multiple phones. This makes it reliable, fast, and battery-efficient.

### Q: How do you handle different scaling requirements?

Keel components scale together as a unit, which is actually ideal for most SaaS applications:

**When components should scale together (Keel's approach):**
- User activity drives email, billing, and storage needs proportionally
- Components share user sessions and context
- Geographic distribution is more important than independent scaling

**When you need independent scaling (traditional microservices):**
- AI inference workloads vs web serving
- Different companies owning different services
- Vastly different resource requirements

For most SaaS products, the scaling patterns are coupled, making Keel's approach more efficient.

### Q: What about fault isolation between services?

WASM components provide excellent fault isolation without network boundaries:

**Memory Isolation:** Each component has its own linear memory space
**Capability Security:** Components can only access explicitly granted interfaces
**Resource Limits:** WASM runtime can enforce CPU and memory limits per component
**Crash Isolation:** Component failure doesn't crash other components

Example:
```rust
// If billing.wasm crashes, user.wasm continues working
let user = user_service::get_user(id)?;  // ✅ Still works

// billing_service::charge() might return an error, but won't crash the process
match billing_service::charge(user_id, amount) {
    Ok(charge_id) => { /* success */ },
    Err(BillingError::ServiceUnavailable) => {
        // Handle gracefully, other services unaffected
    }
}
```

---

## Component Model

### Q: Why WASI Component Model instead of just microservices or libraries?

The Component Model provides the best of both approaches:

**vs Libraries:**
- **Runtime composability:** Swap implementations without recompiling
- **Language independence:** Mix Rust, Go, Python components
- **Security isolation:** Components can't access each other's memory
- **Interface contracts:** WIT provides strong typing and versioning

**vs Microservices:**
- **Performance:** Function calls instead of HTTP requests
- **Simplicity:** No network configuration, service discovery, or load balancing
- **Resource efficiency:** Shared memory and single process
- **Easier debugging:** Standard stack traces instead of distributed tracing

**Example of runtime composability:**
```toml
# Development environment
[infrastructure]
sql = "components/sql-sqlite.wasm"
email = "components/email-console.wasm"  # Prints to console

# Production environment
[infrastructure]
sql = "components/sql-postgres.wasm"
email = "components/email-sendgrid.wasm"

# Same business logic, different adapters
```

### Q: What happens when the WASI specification changes?

We follow the same approach as the [Bytecode Alliance](https://bytecodealliance.org/):

**Stability Strategy:**
- Build on stable WASI features (currently WASI 0.2)
- Test against multiple WASI runtimes (wasmtime, wasmer, etc.)
- Follow semantic versioning for WIT interface changes
- Provide migration guides for breaking changes

**Current Status:**
- WASI 0.2 is stable and production-ready
- Component Model is in active standardization
- Major browser and runtime vendors are committed to compatibility

**Risk Mitigation:**
- Components can be transpiled to JavaScript via jco for broader compatibility
- WIT interfaces provide an abstraction layer above WASI internals
- Community-driven development reduces vendor lock-in risk

---

For detailed information about Keel's scaling architecture, including global coordination for billions of users, see **[Scaling Architecture](SCALING.md)**.

---

## Development & Testing

### Q: How do teams work independently with shared components?

Keel enables team autonomy through interface-driven development:

**Team Boundaries:**
```
Team A owns: user-repository, user-lifecycle components
Team B owns: billing-repository, subscription-billing components
Team C owns: email-sendgrid, email-mailgun adapters
```

**Development Process:**
1. **Interface First:** Teams agree on WIT interfaces
2. **Independent Development:** Each team builds their components
3. **Integration Testing:** BDD tests verify interface contracts
4. **Composition:** Product teams assemble components via configuration

**Example Workflow:**
```rust
// Team A defines the interface
interface user-repository {
    find-by-email: func(email: string) -> result<user, user-error>
}

// Team B depends on the interface, not implementation
impl EmailService {
    fn send_welcome_email(email: String) -> Result<()> {
        let user = user_repository::find_by_email(&email)?; // Uses interface
        // ... business logic
    }
}

// Team A can change implementation without affecting Team B
```

### Q: How do you test components in isolation?

Keel uses comprehensive BDD testing at every layer:

**Infrastructure Components:** Test with real databases
```rust
#[test]
fn test_sql_sqlite_with_real_database() {
    let db = TestDatabase::sqlite();
    // Test against actual SQLite database
}
```

**Repository Components:** Test business operations
```gherkin
Feature: User Repository
  Scenario: Find user by email
    Given a user exists with email "test@example.com"
    When I search for user by email "test@example.com"
    Then I should get the user details
```

**Business Components:** Test with mock repositories
```rust
#[test]
fn test_email_service_with_mock() {
    let mock_repo = MockUserRepository::new();
    mock_repo.expect_find_by_email().returning(|_| Ok(test_user()));

    let email_service = EmailService::new(mock_repo);
    // Test business logic without database
}
```

**Integration Tests:** Full component stack
```rust
#[test]
fn test_full_email_flow() {
    let config = TestConfig {
        sql: "sql-sqlite.wasm",
        email: "email-console.wasm",
    };

    let app = TestApp::new(config);
    // Test end-to-end functionality
}
```

---

## Comparison to Alternatives

### Q: How does this compare to serverless functions?

**Serverless Functions** (AWS Lambda, Vercel Functions):
- ✅ Auto-scaling and pay-per-use
- ❌ Cold start latency
- ❌ Vendor lock-in
- ❌ Limited execution time
- ❌ Stateless (need external storage for everything)

**Keel Components:**
- ✅ Fast startup (WASM)
- ✅ Stateful (can maintain connections, caches)
- ✅ Long-running processes
- ✅ No vendor lock-in
- ❌ Need to manage scaling (but edge deployment helps)

**Best of Both:** Deploy Keel binaries to serverless edge platforms (Cloudflare Workers, Deno Deploy) for auto-scaling with better performance.

### Q: How does this compare to Kubernetes and containers?

**Kubernetes + Containers:**
- ✅ Mature ecosystem and tooling
- ✅ Independent scaling of services
- ❌ Operational complexity
- ❌ Resource overhead (OS per container)
- ❌ Network latency between services
- ❌ Not suitable for edge deployment

**Keel:**
- ✅ Operational simplicity (single binary)
- ✅ Perfect for edge deployment
- ✅ No network overhead
- ✅ Efficient resource usage
- ❌ Components scale together
- ❌ Newer ecosystem

**Use Keel when:**
- Building SaaS applications for global edge deployment
- Team size is small-to-medium
- Performance and simplicity are priorities

**Use Kubernetes when:**
- Very large organization with dedicated platform team
- Need independent scaling patterns
- Existing investment in cloud-native tooling

### Q: What about service mesh (Istio, Linkerd)?

Service mesh solves problems that Keel avoids entirely:

**Service Mesh Problems:**
- Network encryption between services → **Solved:** No network calls
- Load balancing between services → **Solved:** Function calls
- Circuit breakers and retries → **Solved:** No network failures
- Distributed tracing → **Solved:** Standard stack traces
- Service discovery → **Solved:** Direct component imports

**Service Mesh Benefits We Keep:**
- Security policies → **Keel:** WASM capability security
- Traffic splitting → **Keel:** Component-level feature flags
- Observability → **Keel:** Built-in observability component

Service mesh adds complexity to solve distributed systems problems. Keel eliminates the distributed systems, eliminating the problems.

---

## Getting Started

### Q: Where should I start if I want to try this?

1. **Read the [Architecture Documentation](ARCHITECTURE.md)** to understand the design
2. **Check the [Roadmap](ROADMAP.md)** to see current progress
3. **Review [Contributing Guidelines](CONTRIBUTING.md)** for development setup
4. **Start with Phase 1 components:** SQL and KV adapters are working
5. **Join the discussion:** Open GitHub issues with questions or ideas

### Q: Is this ready for production?

**Current Status (Phase 1):** Early development, not production-ready

**Production Readiness Timeline:**
- **Stage 1:** Infrastructure adapters complete
- **Stage 2:** Repository layer complete
- **Stage 3:** First production SaaS applications

**What works today:**
- WIT interface definitions
- Basic SQL/KV components
- BDD testing framework
- Component compilation

**What's coming:**
- Complete adapter implementations
- Platform services (observability, security)
- Business domain components
- Production tooling

---

> **Have more questions?** [Open a GitHub issue](https://github.com/marclove/keel/issues) or contribute to our documentation!

> **Last Updated:** December 2024
> **Contributors:** See our [Contributing Guide](CONTRIBUTING.md)
