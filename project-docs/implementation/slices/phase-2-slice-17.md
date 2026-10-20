# usb-camera-taytay — phase-2-slice-17: Implement MVP2-V4L2-007

**Phase:** [phase-2: Functional requirements](../phases/phase-2.md)

Hardware acceptance evidence and troubleshooting guide.

Live status is recorded in `.sandbox-loom/implementation-status.yml`; Sandbox Loom marks the runtime-selected slice in progress before implementation and records completion only after verification, approval, and a verified commit.

- Source: `requirements`
- Requirement: `MVP2-V4L2-007`
- Acceptance criteria: Record a hardware capture run with Linux version, camera model, negotiated mode, segment settings, artifact checksums, and disconnect/reconnect results. Verify that the captured artifact uses MVP1's Taytay durable upload path, maintain a hardware matrix entry for the available camera, and document troubleshooting guidance.
- Quality gates: declared_cargo_fmt, declared_cargo_check, declared_cargo_test, declared_cargo_llvm_cov, declared_cargo_clippy, declared_quality, declared_acceptance_mock, documentation_links, documentation_navigation, documentation_redaction, mermaid_validation, zensical_build
- Commit required: `true`
