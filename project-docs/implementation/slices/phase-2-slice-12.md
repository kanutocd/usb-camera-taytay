# usb-camera-taytay — phase-2-slice-12: Implement MVP2-V4L2-002

**Phase:** [phase-2: Functional requirements](../phases/phase-2.md)

Format, resolution, and frame-interval negotiation.

Live status is recorded in `.sandbox-loom/implementation-status.yml`; Sandbox Loom marks the runtime-selected slice in progress before implementation and records completion only after verification, approval, and a verified commit.

- Source: `requirements`
- Requirement: `MVP2-V4L2-002`
- Acceptance criteria: Inspect available camera capabilities and record the Linux version, camera model, and one supported pixel format, resolution, and frame interval selected for MVP2.
- Quality gates: declared_cargo_fmt, declared_cargo_check, declared_cargo_test, declared_cargo_llvm_cov, declared_cargo_clippy, declared_quality, declared_acceptance_mock, documentation_links, documentation_navigation, documentation_redaction, mermaid_validation, zensical_build
- Commit required: `true`
