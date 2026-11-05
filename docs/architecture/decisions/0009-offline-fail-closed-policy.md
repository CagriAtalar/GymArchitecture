# ADR-0009: Offline authorization: fail-closed with a bounded cache age

- Status: Accepted
- Date: 2026-11-05

## Context
When the backend is unreachable the device must still decide. Its cached credential snapshot
becomes stale: revocations and expirations after the last refresh are unknown.

## Decision
- The device refreshes a credential snapshot periodically (`ETag`-based).
- Offline **entry** is granted only if the snapshot is younger than `max_offline_age`
  (default 24 h, configurable) and the cached entry's `validUntil` is in the future.
  Otherwise entry is denied (`OFFLINE_CACHE_STALE` or `UNKNOWN_CREDENTIAL`).
- Offline **exit** is allowed for any card present in the snapshot, regardless of its age (ADR-0012).
- Every decision records its source: `ONLINE` or `OFFLINE_CACHE`.
- The snapshot is bounded by a configurable maximum number of entries.

## Alternatives considered
- **Fail-open** (grant all known cards offline indefinitely) — revoked cards keep working.
- **No offline mode** — any outage blocks all members.
- **Unlimited cache age** — same risk as fail-open for long outages.

## Consequences
- Revocations made while a device is offline are not enforced for up to `max_offline_age`.
- Outages longer than `max_offline_age` block entry; staff override is not implemented.
