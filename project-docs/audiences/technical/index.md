# USB Camera Taytay — Reference App Guide

This guide explains how this Linux Rust application composes camera and file
inputs with Taytay. It is written for developers who want to learn from or
adapt the reference application. It documents this application's behavior;
Taytay's own documentation remains authoritative for its reusable crate API
and upload contracts.

## What this application demonstrates

- **MVP1:** observe completed JPEG and MP4 files, validate stable content, and
  publish checksum-bound artifacts to Taytay's spool and durable ledger.
- **MVP2:** capture MJPEG frames from one configured V4L2 device, write
  complete JPEG files and timestamp sidecars, then reuse the MVP1 filesystem
  publication path.
- **Shared boundary:** both sources produce Taytay artifacts. This project
  does not implement a separate HTTP or TUS uploader.

The current USB Camera Taytay commands do not start an uploader or connect to
Lunsaran. For a live transfer, a separate Taytay uploader can use the same
spool and ledger with provisioned Lunsaran project credentials. The
[architecture summary](architecture.md) describes the component boundaries
and local artifact lifecycle.

## Taytay documentation

Use Taytay's documentation for generic integration contracts and behavior:

- [Quick start](https://github.com/kanutocd/taytay/blob/main/docs/quickstart.md)
- [Rust crate guide](https://github.com/kanutocd/taytay/blob/main/docs/crate-guide.md)
- [Source adapters](https://github.com/kanutocd/taytay/blob/main/docs/sources.md)
- [Durability and recovery](https://github.com/kanutocd/taytay/blob/main/docs/durability.md)
- [Uploads and TUS](https://github.com/kanutocd/taytay/blob/main/docs/uploads.md)
- [Security](https://github.com/kanutocd/taytay/blob/main/docs/security.md)

These guides cover reusable `SourceAdapter`, `Artifact`, `Spool`, and
`ArtifactUploader` contracts, along with Entregar upload behavior. This guide
focuses on how USB Camera Taytay composes and exercises those published
boundaries.

## Development and acceptance

Run camera-independent checks with `scripts/acceptance-mock.sh`. To inspect
what the application published, run its `status` command against the same
configuration used for ingestion. Hardware acceptance additionally records
the Linux version, camera model, negotiated mode, segment settings, and
checksums. A successful fixture or mock run does not establish physical
camera capture or a live Lunsaran upload.

## Project relationship

USB Camera Taytay is also a reference application for the private Sandbox Loom
project. The [Sandbox Loom Guide](../stakeholder/sandbox-loom-guide.md) is
published in the Project Team section and shows this project's reference-app
workflow. Credentials, raw evidence, and secrets are not included in the guide.
