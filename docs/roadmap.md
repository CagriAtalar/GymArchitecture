# GymGate — Milestone Roadmap

> **Status: ACCEPTED.** Current milestone: **M0**. Each milestone ends with passing checks, updated docs and a list of suggested commits.

| # | Milestone | Outcome |
|---|---|---|
| M0 | Repository foundation | Folder skeleton, Cargo workspace, `rust-toolchain.toml`, `.editorconfig`, `.gitignore`, ADRs for approved decisions, CI running `cargo fmt/clippy/test` |
| M1 | access-core state machine | Pure `no_std` bidirectional turnstile state machine (one rotation per grant, rotation window) with unit tests for every transition, timeout and failure; bare-metal build check in CI |
| M2 | Hardware simulator + agent skeleton | Hardware traits, simulator process with CLI, agent driving the core locally with a stub authorizer |
| M3 | Backend foundation | Spring Boot + PostgreSQL + Flyway; members, memberships, credentials; admin auth; Testcontainers tests |
| M4 | Device API | Device registration & tokens, `/authorize`, idempotent `/events`, heartbeat, OpenAPI |
| M5 | Online agent ↔ backend | Online authorization, SQLite outbox, sync worker, restart recovery |
| M6 | Offline operation | Credential snapshot cache, expiry, fail-closed policy, link-proxy fault injection |
| M7 | Admin dashboard | Overview, members, memberships, devices, events |
| M8 | End-to-end scenarios | The 8 required system scenarios, reproducible via one script |
| M9 | ARM64 / QEMU | Cross-compilation, QEMU user-mode tests in CI, optional QEMU system-mode VM with systemd + watchdog |
| M10 | Hardening & polish | Threat model, troubleshooting, operations docs, README demo |

## M1 broken into small steps

M1 is delivered in small reviewable steps, each discussed before the next:

1. Crate setup and core types (`CardUid`, `Millis`, `AuthDecision`, `DenyReason`).
2. States and inputs; the happy path `Idle → … → PassageConfirmed → Idle` for both directions.
3. Denials and the "busy" rule.
4. Timeouts (`Tick` + `now`), including the 30 s rotation window.
5. Turnstile faults, unauthorized rotation, and the "no false passage" guarantee.
6. Offline policy as a pure function.
7. `no_std` bare-metal build check.
