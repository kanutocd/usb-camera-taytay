# usb-camera-taytay — phase-2-slice-3: Implement MVP1-FS-003

**Phase:** [phase-2: Functional requirements](../phases/phase-2.md)

Deterministic artifact-ID derivation and duplicate handling.

Live status is recorded in `.sandbox-loom/implementation-status.yml`; Sandbox Loom marks the runtime-selected slice in progress before implementation and records completion only after verification, approval, and a verified commit.

- Source: `requirements`
- Requirement: `MVP1-FS-003`
- Acceptance criteria: Build the MVP1 fixture producer, filesystem observer, and Taytay adapter, including stable artifact publication, checksum verification, and durable-ledger integration.
- Quality gates: declared_cargo_fmt, declared_cargo_check, declared_cargo_test, declared_cargo_llvm_cov, declared_cargo_clippy, declared_quality, declared_acceptance_mock, documentation_links, documentation_navigation, documentation_redaction, mermaid_validation, zensical_build
- Commit required: `true`
