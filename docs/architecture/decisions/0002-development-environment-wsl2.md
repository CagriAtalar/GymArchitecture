# ADR-0002: Development environment: Windows + WSL2

- Status: Accepted
- Date: 2026-11-05

## Context
The primary development machine runs Windows. Rust cross-compilation to ARM64 Linux, Docker and
QEMU are best supported on Linux, and CI runs on Linux.

## Decision
Use WSL2 (Ubuntu) as the primary development environment. All scripts are Bash. Line endings are
normalized to LF via `.gitattributes`. The repository should live on the WSL2 filesystem
(e.g. `~/projects/gymgate`) rather than on a Windows or cloud-synced drive, for build performance.

## Alternatives considered
- **Native Windows toolchains** — workable for Rust, but QEMU, shell scripts and musl cross-builds add friction.
- **Dedicated Linux VM** — heavier than WSL2 with no benefit here.
- **Dev containers** — reproducible, but adds a layer the project does not yet need.

## Consequences
- Windows-native builds are not supported or tested.
- Development and CI share the same OS family, reducing "works on my machine" issues.
