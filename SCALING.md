# Keel: Scaling Architecture

> **Note**: This document is actively maintained as the project evolves. For questions not covered here, please [open a GitHub issue](https://github.com/your-org/keel/issues) or see our [FAQ](FAQ.md).

This document provides comprehensive coverage of how Keel scales from prototype to billions of users while maintaining its edge-native architecture. For foundational concepts, see our [Architecture Documentation](ARCHITECTURE.md).

## Table of Contents

- [Edge Deployment Philosophy](#edge-deployment-philosophy)
- [Performance & Scale](#performance--scale)  
- [Global Coordination Architecture](#global-coordination-architecture)
- [Scaling to Billions of Users](#scaling-to-billions-of-users)
- [Comparison to Alternatives](#comparison-to-alternatives)

---

## Edge Deployment Philosophy

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


## Performance & Scale

### Q: Can a single binary really handle SaaS-scale traffic?

Yes! Modern single-process applications can handle enormous scale:

**Examples of high-scale single binaries:**
- WhatsApp: 2 billion users on Erlang (single VM)
- Discord: Millions of concurrent users on Elixir
- Stackoverflow: Handles massive traffic with minimal servers

**Keel's Performance Advantages:**
- **No serialization overhead** between components
- **No network latency** for inter-component calls
- **Efficient memory usage** via WASM linear memory
- **CPU cache locality** from single-process execution

**Scaling Strategy:**
```
Regional Scaling:
├── US-West: keel-binary (handles millions of users)
├── US-East: keel-binary (handles millions of users)
├── Europe: keel-binary (handles millions of users)
└── Asia: keel-binary (handles millions of users)

NOT:
├── US-West: 50 microservices
├── US-East: 50 microservices
└── ...network complexity...
```

**When you need to scale out:**
- Deploy the same binary to more edge locations
- Use load balancers to distribute traffic geographically
- Each binary handles local traffic with shared global state (databases)

### Q: What about cold start performance?

Edge deployments prioritize cold start performance, and Keel excels here:

**Target Metrics:**
- **<100ms cold start** (vs seconds for container-based microservices)
- **<50MB memory footprint** per instance
- **<100MB binary size** including all components

**Optimization Strategies:**
- WASM's ahead-of-time compilation
- Component lazy loading
- Shared dependencies between components
- Optimized WASM runtimes (wasmtime, wasmer)

---

## Global Coordination Architecture

### Q: How does Keel handle global coordination for billions of users?

Keel's architecture naturally supports a **hierarchical coordination model** that matches how global systems actually scale, while maintaining strong data residency and regulatory compliance alignment.

### Three-Tier Coordination Model

**1. Edge Tier (Regional Autonomy)**
- Each edge location operates autonomously for regional traffic
- Handles 99% of requests with local state and regional databases
- Perfect for data residency (GDPR, data sovereignty requirements)
- Sub-10ms response times for local users

**2. Regional Tier (Cross-Edge Coordination)**
- Regional coordinators aggregate state from multiple edge locations
- Handle cross-edge operations (user migrations, regional analytics)
- Implement eventual consistency patterns for non-critical global state
- Use CRDT-based components for conflict-free replicated data

**3. Global Tier (Planet-Scale Coordination)**
- Global services for truly centralized operations (billing reconciliation, global leaderboards)
- Event streaming infrastructure (Kafka/Pulsar) for global event propagation
- Strong consistency only where absolutely required (payment processing)
- Can leverage existing cloud infrastructure (Spanner, DynamoDB Global Tables)

### Implementation Within Keel's Component Model

This scales **within** our existing architecture through specialized components:

```toml
# Edge deployment configuration
[infrastructure]
sql = "sql-sqlite.wasm"              # Local edge database
kv = "kv-rocksdb.wasm"               # Local cache
event-stream = "event-nats.wasm"     # Regional event bus

[platform-services]
state-sync = "state-crdt.wasm"       # CRDT-based state synchronization
global-coordinator = "coordinator-regional.wasm"  # Regional coordination logic

[business-domains]
user-presence = "presence-local.wasm"     # Local presence (online/offline)
global-presence = "presence-global.wasm"  # Global presence aggregation
```

### New WIT Interfaces for Scale

```wit
interface global-coordination {
    // Local operations (synchronous, fast)
    record-local-event: func(event: event-data) -> result<event-id, error>
    get-local-state: func(key: string) -> result<state-value, error>
    
    // Regional operations (async, eventual consistency)
    sync-to-region: func(events: list<event-data>) -> result<sync-id, error>
    subscribe-regional-updates: func(filter: update-filter) -> result<stream<update>, error>
    
    // Global operations (async, relaxed consistency)
    publish-global-event: func(event: global-event) -> result<_, error>
    query-global-state: func(query: global-query) -> result<global-result, error>
}
```

---

## Scaling to Billions of Users

### Real-World Example: Global Chat Application

Consider a Slack/Discord competitor at billion-user scale:

**Edge Level (Mumbai):**
- All Indian users' messages stored locally
- Real-time messaging within India is instant
- User presence updated locally
- 5ms latency for local operations

**Regional Level (Asia-Pacific):**
- Aggregates presence from Mumbai, Singapore, Sydney edges
- Handles user traveling between cities
- Cross-region direct messages via regional coordinator
- 50ms latency for regional operations

**Global Level:**
- Global user search and discovery
- Compliance and audit logging
- Billing aggregation
- Analytics and reporting
- 200ms latency acceptable for these operations

### Scaling Patterns for Billion Users

**1. Sharding by Geography**
- Natural sharding based on user location
- Each edge handles its regional users completely
- Cross-region operations are rare and async

**2. Event Sourcing for Global State**
- Local events captured at edge
- Streamed to regional aggregators
- Eventually consistent global view
- Can replay events for debugging/recovery

**3. Smart Caching with Write-Through**
- Edge caches user data aggressively
- Writes go through to regional/global stores
- Cache invalidation via event streams
- TTL-based consistency for non-critical data

**4. Hybrid Consistency Models**
- **Strong consistency**: Payment processing, account security
- **Eventual consistency**: User profiles, social graphs
- **Local consistency**: Session data, user preferences
- **CRDT consistency**: Collaborative features, counters

### Data Residency and Compliance

Keel's edge-native architecture provides exceptional compliance capabilities:

**Geographic Data Residency:**
- European users' data never leaves Europe
- Chinese users comply with data localization laws
- Healthcare data stays within required jurisdictions
- Financial data meets regional banking requirements

**Regulatory Compliance:**
- GDPR: Right to be forgotten implemented at edge level
- CCPA: Data export happens from local edge storage
- HIPAA: PHI processing contained within compliant regions
- SOC 2: Audit trails maintained locally and globally

### Addressing Due Diligence Concerns

**"How does this scale to billions of users?"**

1. **Horizontal scaling at the edge** - Add more edge locations as you grow
2. **Natural geographic sharding** - Users naturally partition by location
3. **Hierarchical aggregation** - Not every operation needs global coordination
4. **Event-driven architecture** - Async processing for non-critical paths
5. **Proven patterns** - Similar to how CDNs, DNS, and other internet-scale systems work

**"What about global consistency?"**

- Most operations don't need it (local consistency is enough)
- When needed, use purpose-built global databases (Spanner, CockroachDB)
- Event sourcing provides audit trail and recovery
- CRDTs handle distributed counters and collaborative features

**"How do you handle network partitions?"**

- Edge locations continue operating independently
- Local-first architecture means users aren't blocked
- Sync resumes when partition heals
- Event sourcing ensures no data loss

### Performance Characteristics

**Target Metrics at Scale:**
- **Edge operations**: <10ms (99% of requests)
- **Regional operations**: <50ms (cross-edge coordination)
- **Global operations**: <200ms (acceptable for admin/analytics)
- **Cold start**: <100ms (edge deployment)
- **Memory footprint**: <50MB per edge instance
- **Binary size**: <100MB including all components

**Proven Scale Examples:**
- WhatsApp: 2 billion users on Erlang (single VM approach)
- Discord: Millions of concurrent users on Elixir
- Cloudflare: Edge-first architecture serving billions
- DNS root servers: Hierarchical coordination at internet scale

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
- **Q2 2025:** Infrastructure adapters complete
- **Q4 2025:** Repository layer complete  
- **Q1 2026:** First production SaaS applications

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

> **Have more questions?** [Open a GitHub issue](https://github.com/your-org/keel/issues) or contribute to our documentation!

> **Last Updated:** December 2024  
> **Contributors:** See our [Contributing Guide](CONTRIBUTING.md)