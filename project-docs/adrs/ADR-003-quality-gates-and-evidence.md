# ADR-003: Quality gates and evidence

- Status: **proposed**
- Source: `quality_expectations/qa`

## Context

This decision proposal was derived from the normalized scope for `usb-camera-taytay`.

## Decision proposal

Treat the declared quality gates, coverage thresholds, contract tests, and E2E expectations as required acceptance evidence.

## Rationale and consequences

Autonomous changes must be reviewable through deterministic, recorded verification results.

Review and approve this record before autonomous execution. Update this ADR
when changing the boundary, persistence strategy, tenancy model, generation
workflow, or quality expectations it governs.
