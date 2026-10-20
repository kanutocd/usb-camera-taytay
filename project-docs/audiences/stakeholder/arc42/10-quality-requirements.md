# Quality Requirements

```yaml
---
quality_expectations:
  commands:
  - cargo fmt --all -- --check
  - cargo check --workspace --all-targets --all-features
  - cargo test --workspace --all-features
  - cargo llvm-cov --workspace --all-features --fail-under-lines 90
  - cargo clippy --workspace --all-targets --all-features -- -D warnings
  - scripts/quality.sh
  - scripts/acceptance-mock.sh
  documentation: Run documented Zensical and Markdown checks for documentation changes.
  release: Run applicable quality gates and documentation checks before release.
  scope_note: Commands apply as relevant to the application/workspace; they are transcribed
    from scope and do not assert that the current repository checkout contains each
    script or a Cargo workspace.
qa:
  strategy: Deterministic synthetic and mock acceptance by default; live platform
    and hardware acceptance add evidence when provisioned and available.
  mvp1:
  - Deterministic fixture run showing file stabilization, spool publication, and mock
    upload completion.
  - Tests suppress incomplete files and prevent duplicate logical jobs from repeated
    observations.
  - Restart coverage proves durable jobs resume without data loss.
  - Cover checksum, quota, pause/resume, retry, cancellation, and redaction.
  - Record a real USB-camera-produced file-segment command sequence.
  - Validate successful end-to-end upload against a locally running Lunsaran platform
    using a provisioned test project and scoped device credential; record service
    readiness and result without retaining credentials, signed URLs, or media.
  mvp2:
  - Record device capability and negotiated format without credential leakage.
  - Record capture run with artifact checksums and segment metadata.
  - Test buffer ownership, shutdown, malformed input, disconnect, and reconnect behavior.
  - Maintain hardware matrix entry for available USB camera.
  - Verify capture loop uses same Taytay durable upload path as MVP1.
  test_data: Synthetic media fixtures; no captured media or secrets in committed evidence.
  hardware: Optional for MVP2-specific acceptance; hardware failures are not code-quality
    failures.
```
