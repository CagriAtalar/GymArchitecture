# ADR-0011: Backend stack and structure

- Status: Accepted
- Date: 2026-11-05

## Context
The backend provides admin and device APIs over a relational model with integrity constraints.

## Decision
Java 21 (LTS), Spring Boot, Maven, PostgreSQL, Flyway migrations, springdoc-openapi, and
Testcontainers for integration tests. A **modular monolith packaged by feature** (`member`,
`membership`, `credential`, `device`, `event`, `security`), each with controller / service /
repository / domain / dto layers.

## Alternatives considered
- **Gradle** — equally capable; Maven chosen for familiarity and simplicity.
- **H2 for tests** — fast, but SQL dialect differences hide real PostgreSQL behaviour (e.g. `ON CONFLICT`).
- **Microservices** — no requirement justifies the operational cost.

## Consequences
- Docker is required to run backend integration tests.
- Feature packages keep boundaries clear and allow later extraction if ever needed.
