# ADR-0007: Device authentication with per-device bearer tokens

- Status: Accepted
- Date: 2026-11-05

## Context
Only registered devices may request authorizations and submit events. Secrets must not live in source code.

## Decision
On registration the backend generates a random 256-bit token, returns it **once**, and stores only
its SHA-256 hash. The device stores the token in a permission-restricted file outside the
repository and sends it as `Authorization: Bearer <token>`. Tokens can be rotated by an administrator.
A fast hash is sufficient because the token is high-entropy random data, unlike a human password.

## Alternatives considered
- **Mutual TLS (mTLS)** — strongest option, but requires certificate provisioning and a small PKI;
  documented as the production path.
- **HMAC request signing** — integrity without TLS, but needs nonce/timestamp replay protection.
- **JWT for devices** — adds token issuance and expiry handling with no benefit for a single backend.

## Consequences
- A stolen token allows impersonation of that device until it is rotated.
- Confidentiality depends on TLS; the local Docker Compose setup uses plain HTTP and says so.
