# usb-camera-taytay — phase-3-slice-2: Harden operations.recovery

**Phase:** [phase-3: Non-functional requirements and acceptance](../phases/phase-3.md)

Preserve durable jobs across restart; handle temporary network failure and repeated execution; hardware reconnect behavior is covered for MVP2.

Live status is recorded in `.sandbox-loom/implementation-status.yml`; Sandbox Loom marks the runtime-selected slice in progress before implementation and records completion only after verification, approval, and a verified commit.

- Source: `requirements`
- Requirement: `operations.recovery`
- Acceptance criteria: Automated tests or reproducible verification evidence demonstrate: Preserve durable jobs across restart; handle temporary network failure and repeated execution; hardware reconnect behavior is covered for MVP2.
- Quality gates: declared_cargo_fmt, declared_cargo_check, declared_cargo_test, declared_cargo_llvm_cov, declared_cargo_clippy, declared_quality, declared_acceptance_mock, documentation_links, documentation_navigation, documentation_redaction, mermaid_validation, zensical_build
- Commit required: `true`
