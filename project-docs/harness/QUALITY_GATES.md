# usb-camera-taytay — Quality gates

## Configured gates

- `fmt` — `bin/ci fmt`
- `clippy` — `bin/ci clippy`
- `unit_tests` — `bin/ci unit_tests`
- `code_coverage` — `bin/ci code_coverage`
- `documentation_links` — `bin/ci documentation_links`
- `documentation_navigation` — `bin/ci documentation_navigation`
- `documentation_redaction` — `bin/ci documentation_redaction`
- `mermaid_validation` — `bin/ci mermaid_validation`
- `zensical_build` — `bash -lc source .venv-zensical/bin/activate && zensical build --clean --strict`

These commands are copied from `.sandbox-loom/quality-gates.yml`; update the harness and regenerate documentation if they change.

> Generation status: proposed. No unresolved open questions remain; facts, template defaults, and proposals still require review.
