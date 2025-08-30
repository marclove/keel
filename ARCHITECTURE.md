# Keel: Composable SaaS Architecture

> **Note**: This project is very much a work-in-progress. The architecture described here is being actively developed and refined. We welcome contributions via GitHub issues and pull requests.

## Introduction

Keel is a composable SaaS architecture built on the [WASI Component Model](https://component-model.bytecodealliance.org/) to create reusable business capabilities. The system enables rapid development of new SaaS applications through component composition, following the principles established by the WebAssembly Component Model standardization effort.

### Who is this for?
- **SaaS developers** who want to build applications faster by reusing business capabilities
- **Platform engineers** who need to manage infrastructure complexity across multiple products
- **Component developers** who want to contribute reusable business domain components

## Core Design Principles

1. **Composable Business Objects**: Focus on reusable business capabilities rather than just infrastructure
2. **Strong Separation of Concerns**: Infrastructure, repositories, and business logic developed independently with WIT interface contracts
3. **Runtime Composability**: Components can be hot-swapped through configuration without recompilation
4. **Edge-First Deployment**: Single binary deployments to edge locations, avoiding distributed systems complexity
5. **Language Agnostic**: Components can be written in any WASI-compatible language
6. **Provider Independence**: Business logic never contains provider-specific implementation details

## Component Hierarchy

```
Product Layer (SaaS Applications)
├─ team-collaboration.wasm
├─ live-streaming.wasm
└─ e-learning.wasm
    │
    └── Business Domain Layer
        ├─ email-service.wasm
        ├─ subscription-billing.wasm
        ├─ user-lifecycle.wasm
        └─ collaborative-docs.wasm
            │
            └── Repository Layer
                ├─ user-repository.wasm
                ├─ template-repository.wasm
                ├─ billing-repository.wasm
                └─ document-repository.wasm
                    │
                    └── Platform Services Layer
                        ├─ observability.wasm
                        ├─ security-context.wasm
                        ├─ rate-limiting.wasm
                        └─ feature-flags.wasm
                            │
                            └── Infrastructure Layer
                                ├─ sql-sqlite.wasm
                                ├─ sql-postgres.wasm
                                ├─ kv-rocksdb.wasm
                                ├─ kv-redis.wasm
                                ├─ email-sendgrid.wasm
                                └─ auth-okta.wasm
```

## Layer Definitions

### 1. Infrastructure Layer
**Purpose**: Pure adapters to external services and databases.
**Characteristics**:
- Provider-specific implementations (sql-sqlite, kv-redis, etc.)
- No business logic
- Single responsibility per component
- Hot-swappable via configuration

**Examples**:
```
components/infrastructure/
├── sql-sqlite/          # SQLite database adapter
├── sql-postgres/        # PostgreSQL database adapter
├── kv-rocksdb/          # RocksDB key-value adapter
├── kv-redis/            # Redis key-value adapter
├── email-sendgrid/      # SendGrid email provider
├── email-mailgun/       # Mailgun email provider
├── auth-okta/           # Okta authentication provider
└── storage-s3/          # AWS S3 storage adapter
```

**WIT Interface Example**:
```wit
interface sql {
    query: func(sql: string, params: list<sql-value>) -> result<query-result, sql-error>
    execute: func(sql: string, params: list<sql-value>) -> result<u64, sql-error>
    begin-transaction: func() -> result<transaction, sql-error>
}
```

### 2. Platform Services Layer
**Purpose**: Cross-cutting concerns that all business components may need.
**Characteristics**:
- Observability, security, rate limiting, feature flags
- Used by multiple business domains
- Provider-agnostic (may use infrastructure layer internally)

**Examples**:
```
components/platform-services/
├── observability/       # Tracing, metrics, logging
├── security-context/    # Auth, permissions, audit
├── rate-limiting/       # Request rate control
└── feature-flags/       # A/B testing, rollouts
```

### 3. Repository Layer
**Purpose**: Translate between business concepts and data persistence.
**Characteristics**:
- Abstract business operations (no SQL strings in business logic)
- Domain-focused interfaces
- Uses infrastructure layer internally
- Shields business logic from persistence details

**Examples**:
```
components/repositories/
├── user-repository/     # User CRUD operations
├── template-repository/ # Email template management
├── billing-repository/  # Subscription data access
└── document-repository/ # Document storage/retrieval
```

**WIT Interface Example**:
```wit
interface user-repository {
    find-by-email: func(email: string) -> result<user, user-error>
    create-user: func(registration: user-registration) -> result<user-id, user-error>
    update-preferences: func(user-id: user-id, prefs: user-preferences) -> result<_, user-error>
}
```

**Implementation Pattern**:
```rust
// Repository component uses SQL infrastructure internally
impl UserRepository for UserRepositoryImpl {
    fn find_by_email(email: String) -> Result<User, UserError> {
        // Abstract business operation -> SQL translation happens here
        let result = sql::query(
            "SELECT id, email, name, created_at FROM users WHERE email = $1", 
            vec![SqlValue::Text(email)]
        )?;
        
        // Convert SQL result to business domain object
        Ok(User::from_sql_row(result.rows.first().ok_or(UserError::NotFound)?))
    }
}
```

### 4. Business Domain Layer
**Purpose**: Core business capabilities that can be reused across products.
**Characteristics**:
- Pure business logic and workflows
- Uses repository layer for data access
- Uses platform services for cross-cutting concerns
- Product-agnostic and composable

**Examples**:
```
components/business-domains/
├── email-service/           # Email sending and templating
├── subscription-billing/    # Usage and subscription billing
├── user-lifecycle/          # Onboarding, activation, retention
├── collaborative-docs/      # Real-time document editing
├── live-streaming/          # Video streaming capabilities
└── notification-orchestration/  # Multi-channel notifications
```

**WIT Interface Example**:
```wit
interface email-service {
    send-email: func(to: string, template: string, vars: template-vars) -> result<message-id, email-error>
    send-transactional: func(to: string, template-id: string, vars: template-vars) -> result<message-id, email-error>
    get-delivery-status: func(message-id: message-id) -> delivery-status
}
```

**Implementation Pattern**:
```rust
// Business component uses repositories and services
impl EmailService for EmailServiceImpl {
    fn send_email(to: String, template: String, vars: TemplateVars) -> Result<MessageId, EmailError> {
        // Use repository layer (no SQL in business logic)
        let user = user_repository::find_by_email(&to)?;
        let template_data = template_repository::get_template(&template)?;
        
        // Use platform services
        security_context::validate_permission(&user, "send-email")?;
        rate_limiting::check_rate_limit(&format!("email:{}", user.id), 10, Duration::minutes(1))?;
        
        // Create business domain objects
        let message = Message::new(&user, &template_data, vars);
        let message_id = message_repository::create_outbound_message(&message)?;
        
        // Use infrastructure (via platform service)
        email_provider::send(message.to, message.subject, message.body)?;
        
        // Track metrics
        observability::record_metric("emails.sent", 1.0, vec![("template", &template)]);
        
        Ok(message_id)
    }
}
```

### 5. Product Layer
**Purpose**: Complete SaaS applications composed of business domain components.
**Characteristics**:
- Orchestration and UX-specific logic
- HTTP routing, API design, user interface adaptation
- Minimal business logic (mostly composition)

**Examples**:
```
products/
├── team-collaboration/  # Slack/Discord competitor
├── live-streaming/      # Twitch competitor  
├── e-learning/          # Teachable competitor
└── project-management/  # Asana competitor
```

## Component Composition Pattern

### Configuration-Driven Assembly
Components are assembled through TOML configuration files:

```toml
# products/team-collaboration/components.toml
[infrastructure]
sql = "components/sql-postgres.wasm"
kv = "components/kv-redis.wasm"
email-provider = "components/email-sendgrid.wasm"
auth-provider = "components/auth-okta.wasm"

[platform-services]
observability = "components/observability.wasm"
security-context = "components/security-context.wasm"
rate-limiting = "components/rate-limiting.wasm"

[repositories]
user-repository = "components/user-repository.wasm"
document-repository = "components/document-repository.wasm"
workspace-repository = "components/workspace-repository.wasm"

[business-domains]
email-service = "components/email-service.wasm"
user-lifecycle = "components/user-lifecycle.wasm"
collaborative-docs = "components/collaborative-docs.wasm"
notification-orchestration = "components/notification-orchestration.wasm"

[product]
http-handler = "components/team-collab-api.wasm"
websocket-handler = "components/team-collab-realtime.wasm"
```

### Hot-Swapping Providers
Changing from PostgreSQL to SQLite requires only configuration changes:

```toml
[infrastructure]
sql = "components/sql-sqlite.wasm"  # Changed from sql-postgres.wasm
# Everything else stays the same - no code changes needed
```

## Development Workflow

### 1. Infrastructure First
Build provider-specific infrastructure adapters:
- `sql-sqlite`, `sql-postgres`
- `kv-rocksdb`, `kv-redis`
- `email-sendgrid`, `email-mailgun`

### 2. Platform Services
Develop cross-cutting concerns:
- `observability`, `security-context`
- `rate-limiting`, `feature-flags`

### 3. Repository Layer
Create domain-focused data access:
- `user-repository`, `document-repository`
- Abstract business operations
- Use infrastructure adapters internally

### 4. Business Domains
Build reusable business capabilities:
- `email-service`, `user-lifecycle`
- Pure business logic
- Use repositories and platform services

### 5. Product Composition
Assemble business domains into complete products:
- Focus on UX and API design
- Minimal business logic
- Configuration-driven component loading

## Testing Strategy

### BDD at Every Layer
- **Infrastructure**: Test against real databases (SQLite, RocksDB)
- **Repositories**: Test business operations with test databases
- **Business Domains**: Test with mock repositories
- **Products**: Integration tests with full component stack

### Test Isolation
Each layer can be tested independently:
```rust
// Repository tests use real databases
#[test]
fn test_user_repository_with_sqlite() {
    let db = TestDatabases::new().unwrap();
    let repo = UserRepository::new(&db.sqlite_connection_string());
    // Test business operations
}

// Business domain tests use mock repositories
#[test] 
fn test_email_service_with_mock_repo() {
    let mock_repo = MockUserRepository::new();
    let email_service = EmailService::new(mock_repo);
    // Test business logic
}
```

## Deployment Model

### Edge-First Architecture
Each product deploys as a single, self-contained binary:

```
team-collaboration-edge-binary
├─ Product layer (team-collab-api.wasm)
├─ Business domains (email-service.wasm, collaborative-docs.wasm)
├─ Repositories (user-repository.wasm, document-repository.wasm)
├─ Platform services (observability.wasm, security-context.wasm)
└─ Infrastructure (sql-sqlite.wasm, kv-rocksdb.wasm, email-sendgrid.wasm)
```

### Benefits
- **Zero network latency** between components
- **Atomic deployments** - entire stack works or fails together
- **No service mesh complexity** - clean internal boundaries
- **Edge scalability** - deploy anywhere without dependencies

## Migration and Evolution

### Provider Migration
Switch database providers without touching business logic:
1. Deploy new infrastructure adapter (e.g., `sql-postgres.wasm`)
2. Update configuration to use new adapter
3. Restart application (hot-swap in future versions)

### Business Logic Evolution
Add new business capabilities by composing existing components:
1. Build new business domain component
2. Add to product configuration
3. Update product layer to expose new capabilities

### Cross-Product Reuse
Business domains can be shared across products:
```toml
# Both team-collaboration and project-management can use:
[business-domains]
email-service = "components/email-service.wasm"
user-lifecycle = "components/user-lifecycle.wasm" 
notification-orchestration = "components/notification-orchestration.wasm"
```

## Implementation Roadmap

### Phase 1: Foundation (Completed)
- [x] Project structure and build system
- [x] BDD testing framework
- [x] Generic WIT interfaces for SQL and KV
- [x] Basic infrastructure components (sql-sqlite, kv-rocksdb)

### Phase 2: Infrastructure Layer
- [ ] Complete SQL adapter implementations
- [ ] Complete KV adapter implementations  
- [ ] Email provider adapters (SendGrid, Mailgun)
- [ ] Authentication provider adapters (Okta, Auth0)

### Phase 3: Platform Services Layer
- [ ] Observability component (tracing, metrics, logging)
- [ ] Security context component (auth, permissions, audit)
- [ ] Rate limiting component
- [ ] Feature flags component

### Phase 4: Repository Layer
- [ ] User repository component
- [ ] Template repository component
- [ ] Document repository component
- [ ] Billing repository component

### Phase 5: Business Domain Layer
- [ ] Email service component
- [ ] User lifecycle component
- [ ] Subscription billing component
- [ ] Collaborative docs component

### Phase 6: Product Layer
- [ ] First product: Team collaboration SaaS
- [ ] Component composition framework
- [ ] Hot-swapping mechanism
- [ ] Edge deployment pipeline

### Future Phases
- [ ] Live streaming business domain
- [ ] E-learning business domain
- [ ] Additional product compositions
- [ ] Advanced runtime features (hot-swapping, A/B testing)

## Success Metrics

### Development Velocity
- **Time to new product**: <2 weeks from concept to MVP
- **Provider switching**: <1 day to swap infrastructure providers
- **Feature reuse**: 80% of business logic reused across products

### Operational Excellence
- **Edge deployment**: Single binary, <100MB
- **Startup time**: <100ms cold start
- **Memory efficiency**: <50MB runtime footprint per product

### Architecture Quality
- **Test coverage**: >90% at each layer
- **Component isolation**: Zero business logic in infrastructure
- **Interface stability**: Backward compatibility across versions