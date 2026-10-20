# usb-camera-taytay — phase-2-slice-16: Implement MVP2-V4L2-006

**Phase:** [phase-2: Functional requirements](../phases/phase-2.md)

Capture-to-spool backpressure.

Live status is recorded in `.sandbox-loom/implementation-status.yml`; Sandbox Loom marks the runtime-selected slice in progress before implementation and records completion only after verification, approval, and a verified commit.

- Source: `requirements`
- Requirement: `MVP2-V4L2-006`
- Acceptance criteria: Implement pure Rust capture for the selected V4L2 mode, configure and safely return streaming buffers, and write bounded timestamped segments that are complete and readable before publication. Handle malformed frames, shutdown, disconnect, and reconnect without corrupting completed artifacts. Apply backpressure at the spool boundary and publish through the same Taytay Artifact, spool, and upload path used by MVP1, without application-specific upload logic.
- Quality gates: declared_cargo_fmt, declared_cargo_check, declared_cargo_test, declared_cargo_llvm_cov, declared_cargo_clippy, declared_quality, declared_acceptance_mock, documentation_links, documentation_navigation, documentation_redaction, mermaid_validation, zensical_build
- Commit required: `true`
