//! GymGate access-control core.
//!
//! A pure ("sans-IO") state machine for a bidirectional tripod turnstile. It receives inputs
//! (card presented, turnstile confirmations, rotations, clock ticks) together with the current
//! monotonic time and returns commands for the device agent to execute. It never performs I/O.
//!
//! See `docs/architecture/decisions/0003-sans-io-no-std-access-core.md`.
//! The state machine itself is implemented in Milestone 1.

#![no_std]
