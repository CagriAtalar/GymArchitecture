# Development setup

GymGate is developed on **Windows + WSL2 (Ubuntu)** (see ADR-0002). All commands below run in a WSL2 shell.

## Where to keep the repository

Clone the repository into the WSL2 filesystem, for example `~/projects/gymgate`.
Building from a Windows or cloud-synced folder (e.g. OneDrive) through `/mnt/c/...` is much slower,
and the sync client will try to upload large build directories such as `device/target/`.

## Device code (Rust)

1. Install rustup: <https://rustup.rs>
2. The exact Rust version is pinned in `device/rust-toolchain.toml`; rustup installs it automatically
   the first time a `cargo` command runs inside `device/`.
3. Run the same checks as CI:

```bash
cd device
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Backend, frontend, Docker Compose and QEMU instructions will be added in the milestones that introduce them.
