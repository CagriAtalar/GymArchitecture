# ADR-0005: Hardware simulator as a separate process

- Status: Accepted
- Date: 2026-11-05

## Context
The first release has no physical hardware. Peripherals must be controllable for demos and
end-to-end tests, including failures such as motor faults and network loss.

## Decision
`hardware-simulator` is a separate process. It talks to the agent's `SimulatedHardware` adapter over
localhost TCP using newline-delimited JSON, and provides an interactive CLI plus a scripted scenario
mode. It also contains a small TCP **link proxy** placed between the agent and the backend, which can
be cut or delayed to simulate network loss.

## Alternatives considered
- **In-process mock adapters only** — still used for agent unit tests, but cannot demonstrate a real
  process boundary, restarts, or an agent running in QEMU.
- **Simulator as a library linked into the agent** — simpler, but lets the agent bypass the hardware boundary.
- **Network fault flag inside the agent** — fakes the failure instead of producing it.

## Consequences
- An extra wire protocol to define, version and test.
- The agent can run inside QEMU while the simulator runs natively (port forwarding).
- Network and backend failures seen by the agent are real socket failures.
