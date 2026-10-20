# ADR-001: Tenant isolation

- Status: **approved**
- Source: `tenancy/isolation`

## Context

This decision was derived from the normalized scope for `usb-camera-taytay`.

## Decision

Enforce tenant boundaries in application authorization and the persistence layer.

## Rationale and consequences

Cross-tenant access must fail closed even if one enforcement layer is bypassed or misconfigured.

This decision is approved in the decision ledger. The harness must also be
approved before autonomous execution. Update this ADR when changing the boundary,
persistence strategy, tenancy model, generation workflow, or quality expectations
it governs.
