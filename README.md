# GymGate

A simulation-first gym access-control system: a Rust device agent drives a simulated bidirectional
turnstile, a Spring Boot backend manages members, memberships, credentials and devices, and a React
dashboard shows access events and live occupancy.

> **Status: early development (Milestone 0 — repository foundation).**
> Only the repository skeleton and the architecture documentation exist so far.
> See the [roadmap](docs/roadmap.md) for what is planned and what is done.

## Repository layout

| Path | Contents | Status |
|---|---|---|
| `device/access-core` | Pure, `no_std` turnstile state machine | Crate skeleton |
| `device/device-agent` | Tokio device agent | Planned (M2) |
| `device/hardware-simulator` | Simulated peripherals + CLI | Planned (M2) |
| `backend/` | Spring Boot backend | Planned (M3) |
| `frontend/` | React admin dashboard | Planned (M7) |
| `docs/` | Architecture, ADRs, development docs | In progress |

## Documentation

- [Architecture proposal](docs/architecture/proposal.md)
- [Architecture decision records](docs/architecture/decisions/README.md)
- [Development setup](docs/development/setup.md)
- [Roadmap](docs/roadmap.md)
