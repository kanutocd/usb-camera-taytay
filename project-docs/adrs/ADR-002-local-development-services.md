# ADR-002: Local development services

- Status: **proposed**
- Source: `development_environment/services`

## Context

This decision proposal was derived from the normalized scope for `usb-camera-taytay`.

## Decision proposal

Use the declared local orchestration and dependency services as the reproducible development boundary.

## Rationale and consequences

Health checks and disposable local dependencies make setup and verification repeatable without production credentials.

Review and approve this record before autonomous execution. Update this ADR
when changing the boundary, persistence strategy, tenancy model, generation
workflow, or quality expectations it governs.
