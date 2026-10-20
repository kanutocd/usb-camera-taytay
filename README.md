# USB Camera Taytay

[![CI](https://github.com/kanutocd/usb-camera-taytay/actions/workflows/ci.yml/badge.svg)](https://github.com/kanutocd/usb-camera-taytay/actions/workflows/ci.yml)
[![Documentation](https://github.com/kanutocd/usb-camera-taytay/actions/workflows/docs.yml/badge.svg)](https://github.com/kanutocd/usb-camera-taytay/actions/workflows/docs.yml)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-2563eb)](https://kanutocd.github.io/usb-camera-taytay/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust)](https://www.rust-lang.org/)

USB Camera Taytay is a Linux Rust CLI reference application for publishing
local camera media to Taytay's durable artifact spool. It demonstrates two
staged input paths: MVP1 ingests completed files, and MVP2 captures MJPEG from
a configured V4L2 camera and feeds the resulting JPEG files through the same
filesystem ingestion path.

The CLI writes artifacts to Taytay; a separately provisioned Taytay uploader
handles transfer to Lunsaran. This application does not start that uploader
or implement its own HTTP or TUS upload path. The repository also serves as a
public reference application for the private Sandbox Loom project.

## Documentation

- [User Guide](project-docs/audiences/consumer/index.md) — configure the CLI,
  ingest files, capture from V4L2, and inspect artifact status.
- [Reference App Guide](project-docs/audiences/technical/index.md) — understand
  the application structure, Taytay integration boundary, and acceptance
  approach.
- [Project Team](project-docs/audiences/stakeholder/index.md) — project scope
  and brief, Arc42 architecture, delivery plan, and Sandbox Loom guide.
- [Project Scope Statement](project-requirements/project-scope-statement.md)

## Quick start

On Linux with Rust and Cargo installed, copy `config.example.toml` to a local
configuration file and adjust the input, spool, and archive paths. These
commands create deterministic sample inputs, ingest completed files, and show
their local Taytay ledger state; they do not need a camera, credentials, or a
running Lunsaran platform.

```sh
cp config.example.toml config.toml
cargo run -- check-config config.toml
cargo run -- fixture config.toml
cargo run -- run-filesystem config.toml
cargo run -- status config.toml
```

Keep credentials out of `config.toml`. The current CLI does not consume a
device-token file. For optional live transfer, configure credentials in the
separate Taytay uploader process using the provisioned environment's guidance.

## MVP boundaries

- **MVP1 — filesystem ingestion:** observe stable, completed JPEG and MP4 files
  in one input directory, then publish checksum-verified artifacts through
  Taytay's spool and ledger. Synthetic fixtures support camera-free checks.
- **MVP2 — V4L2 capture:** capture MJPEG from one configured Linux camera and
  write bounded, complete JPEG files and timestamp sidecars. A subsequent
  `run-filesystem` command publishes them through the MVP1 path.

For MVP2, inspect the device's supported modes with
`v4l2-ctl --all --list-formats-ext`, configure the V4L2 fields in
`config.toml`, then run:

```sh
cargo run -- run-v4l2 config.toml
cargo run -- run-filesystem config.toml
```

Capture requires permission to access the camera device. Physical camera
capture and live Lunsaran transfer require separate hardware and platform
acceptance; fixture or mock success does not verify either. See the User Guide
for camera setup, artifact lifecycle, and troubleshooting details.

## Development checks

Run `bin/setup-dev` once to install the pinned documentation tools and local
coverage tool. The main checks are:

```sh
scripts/quality.sh
scripts/acceptance-mock.sh
bin/ci declared_cargo_llvm_cov
```

The mock acceptance uses no camera or live credentials. See the
[quality gates](project-docs/harness/QUALITY_GATES.md) for the full CI command
set.
