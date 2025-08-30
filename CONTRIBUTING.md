# Contributing to Keel

Thank you for your interest in contributing to Keel! This document provides guidelines for contributing to the Keel composable SaaS architecture project.

## Code of Conduct

This project follows the same collaborative principles as the [WebAssembly Community Group](https://www.w3.org/community/webassembly/). We are committed to providing a welcoming and inclusive environment for all contributors.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Node.js (for jco transpilation tools)
- Git
- Familiarity with the [WASI Component Model](https://component-model.bytecodealliance.org/)

### Development Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-org/keel.git
   cd keel
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   npm install
   ```

3. **Run tests**:
   ```bash
   cargo test --workspace
   ```

4. **Check the architecture documentation**:
   ```bash
   # Review ARCHITECTURE.md and ROADMAP.md
   ```

## How to Contribute

### Reporting Issues

We use GitHub Issues to track bugs, feature requests, and architecture discussions:

- **Bug Reports**: Include steps to reproduce, expected vs actual behavior
- **Feature Requests**: Describe the use case and proposed solution
- **Architecture Discussions**: Use the `architecture` label for design discussions

### Contributing Code

1. **Check existing issues** to see if your contribution is already being discussed
2. **Open an issue** for new features or significant changes before starting work
3. **Fork the repository** and create a feature branch
4. **Follow the development patterns** established in the codebase
5. **Write tests** following our BDD approach
6. **Submit a pull request** with a clear description

### Component Development Guidelines

#### Infrastructure Components
Infrastructure components should be pure adapters with these characteristics:

- **Single Responsibility**: Each component handles exactly one external service
- **No Business Logic**: Pure translation between WIT interfaces and provider APIs  
- **Provider-Specific**: Named clearly (e.g., `sql-sqlite`, `email-sendgrid`)
- **Comprehensive Testing**: BDD tests with real provider integration

Example structure:
```
components/infrastructure/sql-postgres/
├── Cargo.toml
├── src/lib.rs              # WIT bindings and adapter implementation
├── features/               # BDD feature files
│   └── sql-operations.feature
└── tests/
    └── bdd_tests.rs        # Cucumber test implementations
```

#### Repository Components
Repository components abstract business data operations:

- **Business-Focused**: Operations reflect business concepts, not database schema
- **Database-Agnostic**: Work with any SQL/KV infrastructure component
- **Rich Domain Models**: Return business objects, not raw database rows
- **Abstract Operations**: No SQL strings or database-specific code

Example interface:
```wit
interface user-repository {
    find-by-email: func(email: string) -> result<user, user-error>
    create-user: func(registration: user-registration) -> result<user-id, user-error>
    // Business operations, not CRUD
}
```

#### Business Domain Components
Business domain components implement reusable business capabilities:

- **Product-Agnostic**: Can be used across multiple SaaS products
- **Pure Business Logic**: Focus on business rules and workflows
- **Service Integration**: Use repository and platform service components
- **Rich APIs**: Expose business capabilities, not technical operations

### Testing Standards

We follow a comprehensive BDD (Behavior-Driven Development) approach:

#### Test Structure
```
components/your-component/
├── features/
│   └── component-behavior.feature    # Gherkin scenarios
└── tests/
    └── bdd_tests.rs                  # Cucumber step implementations
```

#### Writing BDD Tests

1. **Start with failing tests** - Write feature scenarios first
2. **Test behavior, not implementation** - Focus on business outcomes
3. **Use real dependencies** - Test with actual databases, not mocks (for infrastructure)
4. **Test edge cases** - Error conditions, boundary values, failure scenarios

Example feature file:
```gherkin
Feature: User Registration
  As a SaaS application
  I want to register new users
  So that they can access the system

  Scenario: Successful user registration
    Given a clean user database
    When I register a user with email "test@example.com"
    Then the user should be created successfully
    And the user should receive a welcome email
    And the user should be marked as "pending verification"
```

#### Test Categories

- **Unit Tests**: Component-level functionality
- **Integration Tests**: Component interactions with real dependencies  
- **BDD Tests**: User-facing behavior scenarios
- **Performance Tests**: Latency and throughput requirements

### WIT Interface Design

When designing WIT interfaces, follow these principles:

#### Interface Naming
- Use kebab-case for interface names: `user-repository`, `email-service`
- Use descriptive function names: `find-by-email`, `send-transactional`
- Avoid technical jargon in business domain interfaces

#### Error Handling
```wit
variant user-error {
    not-found(string),
    invalid-email(string), 
    duplicate-email(string),
    validation-failed(list<validation-error>),
}
```

#### Versioning
- Follow semantic versioning for interface changes
- Maintain backward compatibility when possible
- Document breaking changes in pull requests

### Documentation Standards

#### Code Documentation
- **WIT Interfaces**: Document all functions, types, and error conditions
- **Implementation**: Focus on business logic, not obvious technical details
- **Architecture Decisions**: Document design choices in ARCHITECTURE.md

#### Examples
Include working examples for:
- Component configuration
- Basic usage patterns
- Integration with other components
- Error handling

### Pull Request Guidelines

#### Before Submitting
- [ ] Tests pass locally (`cargo test --workspace`)
- [ ] Code follows existing patterns and style
- [ ] Documentation is updated if needed
- [ ] BDD tests cover new functionality
- [ ] No breaking changes to existing WIT interfaces (unless discussed)

#### PR Description Template
```markdown
## Summary
Brief description of the change and motivation.

## Type of Change
- [ ] Bug fix
- [ ] New component
- [ ] Interface change
- [ ] Documentation update
- [ ] Architecture improvement

## Testing
- [ ] Existing tests pass
- [ ] New tests added for new functionality
- [ ] BDD scenarios cover the change

## Breaking Changes
List any breaking changes and migration steps.

## Related Issues
Closes #123
```

## Architecture Discussions

For significant architectural decisions, we follow this process:

1. **Open an Issue** with the `architecture` label
2. **Describe the Problem** and proposed solution
3. **Community Discussion** - gather feedback and alternatives
4. **Document Decision** in ARCHITECTURE.md
5. **Implementation** following the agreed approach

### Design Principles Review

All contributions should align with our core principles:
- **Composable Business Objects**: Reusable business capabilities
- **Strong Separation of Concerns**: Clear layer boundaries
- **Runtime Composability**: Configuration-driven component loading
- **Edge-First Deployment**: Single binary, no distributed systems
- **Provider Independence**: Business logic never knows about specific providers

## Release Process

We follow milestone-driven releases aligned with our [roadmap](ROADMAP.md):

1. **Feature Complete**: All planned components implemented
2. **Testing**: Comprehensive BDD and integration testing
3. **Documentation**: Architecture and usage documentation complete
4. **Performance**: Benchmarks meet target requirements
5. **Community Review**: Public review period for feedback

## Getting Help

- **GitHub Issues**: For bugs, features, and questions
- **Architecture Discussions**: Use the `architecture` label
- **Component Design**: Open an issue before significant new components

## Recognition

We maintain a [CONTRIBUTORS.md](CONTRIBUTORS.md) file to recognize all contributors to the project. Contributions include:

- Code contributions (components, tests, fixes)
- Documentation improvements
- Architecture feedback and design input
- Issue reporting and triage
- Community support and discussion

---

Thank you for contributing to Keel! Together we're building the future of composable SaaS architecture.

> **Questions?** Open a GitHub issue or start a discussion in our repository.