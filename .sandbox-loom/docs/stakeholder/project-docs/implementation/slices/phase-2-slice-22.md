# usb-camera-taytay — phase-2-slice-22: Implement DEV-004

**Phase:** [phase-2: Functional requirements](../phases/phase-2.md)

Documentation build and local-link drift checks.

Live status is recorded in `.sandbox-loom/implementation-status.yml`; Sandbox Loom marks the runtime-selected slice in progress before implementation and records completion only after verification, approval, and a verified commit.

- Source: `requirements`
- Requirement: `DEV-004`
- Acceptance criteria: Demonstrate deterministic fixture and mock acceptance, including incomplete-file suppression, duplicate-observation handling, checksum and quota behavior, pause/resume, retry and resume across restart, cancellation, and credential/media redaction. Document reproducible local setup and operation without production credentials. When a local Lunsaran/Brutus environment is provisioned, verify service readiness and a successful end-to-end upload using a test project and scoped device credential; retain only redacted readiness and result evidence, with no credentials, signed URLs, or media.
- Quality gates: declared_cargo_fmt, declared_cargo_check, declared_cargo_test, declared_cargo_llvm_cov, declared_cargo_clippy, declared_quality, declared_acceptance_mock, documentation_links, documentation_navigation, documentation_redaction, mermaid_validation, zensical_build
- Commit required: `true`
