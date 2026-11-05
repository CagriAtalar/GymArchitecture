# ADR-0001: Record architecture decisions

- Status: Accepted
- Date: 2026-11-05

## Context
GymGate spans embedded, backend and frontend code. Decisions such as "why is the core `no_std`" or
"why sessions instead of JWT" are easy to forget and hard to reconstruct from code alone.

## Decision
Record each significant decision as a numbered Markdown ADR in `docs/architecture/decisions/`
with the sections Context, Decision, Alternatives considered and Consequences.

## Alternatives considered
- **Commit messages only** — decisions get scattered and lack alternatives.
- **One large design document** — drifts and hides when and why something changed.
- **External wiki** — separated from the code it describes.

## Consequences
- A small writing cost per decision.
- Decision history is versioned with the code and reviewable in pull requests.
