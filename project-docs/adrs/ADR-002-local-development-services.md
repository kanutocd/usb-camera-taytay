# ADR-002: Local development services

- Status: **approved**
- Source: `development_environment/services`

## Context

This decision was derived from the normalized scope for `usb-camera-taytay`.

## Decision

Use the declared local orchestration and dependency services as the reproducible development boundary.

## Rationale and consequences

Health checks and disposable local dependencies make setup and verification repeatable without production credentials.

This decision is approved in the decision ledger. The harness must also be
approved before autonomous execution. Update this ADR when changing the boundary,
persistence strategy, tenancy model, generation workflow, or quality expectations
it governs.
