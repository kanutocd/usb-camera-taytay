# ADR-003: Quality gates and evidence

- Status: **approved**
- Source: `quality_expectations/qa`

## Context

This decision was derived from the normalized scope for `usb-camera-taytay`.

## Decision

Treat the declared quality gates, coverage thresholds, contract tests, and E2E expectations as required acceptance evidence.

## Rationale and consequences

Autonomous changes must be reviewable through deterministic, recorded verification results.

This decision is approved in the decision ledger. The harness must also be
approved before autonomous execution. Update this ADR when changing the boundary,
persistence strategy, tenancy model, generation workflow, or quality expectations
it governs.
