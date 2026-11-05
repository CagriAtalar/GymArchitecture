# ADR-0010: At-least-once event delivery with idempotent ingestion

- Status: Accepted
- Date: 2026-11-05

## Context
Events are produced on the device and must reach the backend despite network loss and restarts,
without creating duplicate records.

## Decision
- The device assigns each event a UUIDv7 `event_id` and a per-device monotonic `device_seq`, and
  commits it to its SQLite outbox (ADR-0006).
- A sync worker sends batches and deletes rows only after acknowledgement (**at-least-once**).
- The backend inserts with a unique constraint on `event_id` (`ON CONFLICT DO NOTHING`) and returns
  which IDs were duplicates, so the device can delete them too.

The resulting guarantee is *at-least-once delivery with at-most-once storage*. It is not exactly-once delivery.

## Alternatives considered
- **Server-generated IDs** — a retry after a lost acknowledgement creates a duplicate.
- **Message broker (e.g. Kafka)** — solves nothing the outbox does not, and adds infrastructure.
- **Fire-and-forget** — loses events on any outage.

## Consequences
- Events can arrive late and out of order; consumers (history, presence) order by
  `occurred_at` / `device_seq` and store `received_at` as well.
- Device clock skew is visible; the heartbeat returns server time for diagnostics.
