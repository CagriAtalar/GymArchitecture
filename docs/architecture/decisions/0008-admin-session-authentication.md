# ADR-0008: Admin authentication with server-side sessions

- Status: Accepted
- Date: 2026-11-05

## Context
The admin dashboard is a single-page app talking to a single backend.

## Decision
Use Spring Security form login with server-side sessions (`HttpOnly`, `SameSite` cookie) and CSRF
protection. Passwords are hashed with bcrypt. The frontend is served from the same origin as the
API (Vite proxy in development, reverse proxy in Docker Compose).

## Alternatives considered
- **JWT access + refresh tokens** — useful when several services must validate tokens independently;
  here it adds refresh, storage and revocation complexity with no benefit.
- **HTTP Basic** — credentials sent with every request; no logout.

## Consequences
- Logout and account disabling take effect immediately.
- Session state is held by the backend, which is acceptable for a single-instance monolith; horizontal
  scaling would need a shared session store.
