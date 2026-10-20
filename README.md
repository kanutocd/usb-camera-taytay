# usb-camera-taytay

Provide a small, reproducible Linux Rust CLI reference application that captures or receives local USB-camera media and delivers durable, checksum-bound, resumable uploads to Lunsaran through Taytay, while keeping filesystem ingestion and pure Rust V4L2 capture as deliberately staged MVPs.

This project is being developed under an approved Sandbox Loom harness
using the `rust-binary` template.

## Project guidance

- [Generated harness documentation](project-docs/harness/README.md)
- [Architecture](project-docs/harness/ARCHITECTURE.md)
- [Requirements](project-docs/harness/REQUIREMENTS.md)
- [Architecture decisions](project-docs/adrs/README.md)
- [Implementation plan](project-docs/implementation/README.md)
- [Backlog](project-docs/backlog/README.md)
- [Changelog](CHANGELOG.md)
- [Quality gates](project-docs/harness/QUALITY_GATES.md)
- [Executable harness configuration](.sandbox-loom/)

## Development

Run `bin/setup-dev` to install the pinned documentation tools and the
project-local coverage tool. Run `scripts/quality.sh` for Rust formatting,
build, tests, and linting; run `scripts/acceptance-mock.sh` for the camera-free
CLI smoke check. The individual commands and documentation gates are listed in
[Quality gates](project-docs/harness/QUALITY_GATES.md).

## Baseline constraints

The generated harness applies these baseline constraints where applicable:

```yaml
---
application: One Linux reference application; one configured MVP1 input directory;
  one configured MVP2 V4L2 device.
cli: Rust CLI; command names and behavior should be consumer-oriented and documented
  before being treated as stable.
mvp_boundary: MVP1 consumes completed files from an external producer or fixtures;
  MVP2 replaces that producer with a pure Rust V4L2 capture loop.
artifact_contract: Filesystem and V4L2 source boundaries produce the same Taytay Artifact
  contract; only complete, stable, readable artifacts are emitted.
durability: Publication is atomic and checksum-bound before upload; the durable ledger
  is authoritative for local job state; the remote TUS offset is authoritative for
  resumable transfer.
retry: Retries are bounded and classified; permanent authorization or protocol failures
  are not hidden as transport retries.
upload_boundary: Use published lunsaran-taytay and lunsaran-entregar integrations
  and Taytay ArtifactUploader; no application-specific HTTP or TUS upload path.
local_tests: Synthetic fixtures and camera-independent tests are required; camera
  hardware is not required for ordinary unit and mock acceptance.
hardware: MVP2 targets one configured /dev/video* device and initially supports one
  verified pixel format, resolution, and frame interval.
operations: Keep the application unprivileged and use a narrowly scoped writable spool;
  bound memory, segment size, spool, and capture-to-upload pressure.
deployment: Local reference application and integration example; no hosted control
  plane or production deployment scope is defined.
```

The generated documents are review artifacts. The harness and policy
configuration remain subject to explicit review and approval.
