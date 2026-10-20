# ADR-001: Tenant isolation

- Status: **proposed**
- Source: `tenancy/isolation`

## Context

This decision proposal was derived from the normalized scope for `usb-camera-taytay`.

## Decision proposal

Enforce tenant boundaries in application authorization and the persistence layer.

## Rationale and consequences

Cross-tenant access must fail closed even if one enforcement layer is bypassed or misconfigured.

Review and approve this record before autonomous execution. Update this ADR
when changing the boundary, persistence strategy, tenancy model, generation
workflow, or quality expectations it governs.
