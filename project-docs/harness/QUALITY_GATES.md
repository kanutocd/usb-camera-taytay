# usb-camera-taytay — Quality gates

## Configured gates

- `declared_cargo_fmt` — `cargo fmt --all -- --check`
- `declared_cargo_check` — `cargo check --workspace --all-targets --all-features`
- `declared_cargo_test` — `cargo test --workspace --all-features`
- `declared_cargo_llvm_cov` — `bin/ci declared_cargo_llvm_cov`
- `declared_cargo_clippy` — `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `declared_quality` — `scripts/quality.sh`
- `declared_acceptance_mock` — `scripts/acceptance-mock.sh`
- `documentation_links` — `bin/ci documentation_links`
- `documentation_navigation` — `bin/ci documentation_navigation`
- `documentation_redaction` — `bin/ci documentation_redaction`
- `mermaid_validation` — `bin/ci mermaid_validation`
- `zensical_build` — `bin/ci zensical_build`

These commands are copied from `.sandbox-loom/quality-gates.yml`; update the harness and regenerate documentation if they change.

> Harness approval: approved. No unresolved open questions remain; recorded decisions are approved.
