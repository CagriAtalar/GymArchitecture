# ADR-0012: Card-controlled exit with anti-passback

- Status: Accepted
- Date: 2026-11-05

## Context
The gym wants to know current occupancy and to prevent a card being passed to another person after entry.

## Decision
- Both directions require a card. Each `PassageConfirmed` updates the member's presence (`INSIDE` / `OUTSIDE`).
- **Anti-passback:** entry is denied (`ANTI_PASSBACK`) while the member is `INSIDE`.
- **Exit is never denied for membership reasons**; only unknown or revoked cards are refused.
  Real installations must also provide an emergency release.
- Presence can drift (tailgating, emergency exits): admins can reset a member's presence, and all
  members are reset to `OUTSIDE` at a configured closing time.
- Presence is owned by the backend; offline, the device tracks presence locally (best-effort) and
  reconciles through event sync.

## Alternatives considered
- **Free exit (push button)** — simpler, but no occupancy data and no anti-passback.
- **Soft anti-passback** (log violation but allow) — weaker; could be added as a configuration option later.

## Consequences
- Live occupancy is available to the dashboard and to a future mobile app.
- Wrong presence states can block a legitimate entry until corrected; the reset mechanisms exist for this reason.
