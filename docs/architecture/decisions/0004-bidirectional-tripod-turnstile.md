# ADR-0004: Bidirectional tripod turnstile model

- Status: Accepted
- Date: 2026-11-05

## Context
The state machine depends on the physical device being modelled.

## Decision
Model a **bidirectional tripod turnstile**:
- Entry and exit directions are released independently.
- A grant allows exactly **one rotation** in the granted direction within a configurable
  **rotation window** (default 30 s); otherwise the grant expires.
- After one rotation the turnstile locks again.
- The rotation sensor is the passage sensor; the controller confirms release and lock.
- A passage is recorded only after confirmed release *and* a completed rotation in the granted direction.
- A rotation without an active grant, or in the wrong direction, raises `UnauthorizedRotation`.

## Alternatives considered
- **Door / swing gate** opened per direction — needs door-position sensing and allows several
  people per opening (tailgating).
- **Separate infrared beam as passage sensor** — more realistic for speed gates; unnecessary for a tripod.

## Consequences
- One person per grant is enforced by the mechanism, simplifying the state machine.
- Jumping over or crawling under the turnstile is not detected.
- Emergency free-rotation (fire alarm) is a real-installation requirement that is documented but not simulated.
