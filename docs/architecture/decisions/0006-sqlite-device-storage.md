# ADR-0006: SQLite for device-local storage

- Status: Accepted
- Date: 2026-11-05

## Context
The device must persist its event outbox and credential cache across crashes and restarts.

## Decision
Use SQLite through `rusqlite` with the bundled SQLite build, in WAL mode, with `synchronous=FULL`
for outbox writes. Each emitted event is committed before it is considered emitted.

## Alternatives considered
- **Hand-written append-only log file** — needs custom crash-safety and compaction code.
- **`redb`** (pure-Rust embedded key-value store) — avoids a C dependency; less convenient to inspect and query.
- **JSON files** — no atomicity guarantees.

## Consequences
- Transactions and crash safety come from a mature, widely deployed engine.
- The bundled C code requires a C cross-compiler when building for ARM64.
- The `sqlite3` CLI can be used to inspect device state while debugging.
