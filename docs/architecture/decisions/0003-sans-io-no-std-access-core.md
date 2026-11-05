# ADR-0003: Sans-IO, `no_std` access core

- Status: Accepted
- Date: 2026-11-05

## Context
The access-control logic must be independent of Tokio, Linux, networking and hardware, and be
portable to a future microcontroller. It must also be thoroughly testable, including timeouts and
failures.

## Decision
`access-core` is a **sans-IO state machine**: it receives an input plus the current monotonic time
and returns commands (`Release`, `Lock`, `Buzz`, `RequestAuthorization`, `Emit(event)`). It never
performs I/O, never reads a clock and never sleeps.

The crate is `#![no_std]`, uses no heap allocation (`alloc`), forbids `unsafe`, and has no
dependencies. Fixed-size types are used (e.g. a card UID is at most 10 bytes, per ISO/IEC 14443).
From Milestone 1, CI builds it for the bare-metal target `thumbv7em-none-eabihf` to prove portability.

## Alternatives considered
- **Async core calling hardware traits** (`turnstile.release().await`) — ties the core to an async
  executor and makes timeout tests depend on real or mocked time.
- **Core with `alloc`** — more convenient collections, but requires a heap allocator on an MCU.
- **Logic embedded in the agent** — fastest to write, but untestable without hardware mocks and not portable.

## Consequences
- Every transition is testable deterministically with plain unit tests, without mocks or real time.
- The agent must faithfully execute commands and feed confirmations back as inputs.
- Portability is proven only at the build level: no firmware exists and none is claimed.
