# Project Scope Statement

This page summarizes the scope for product-team readers. The canonical source
remains [`project-requirements/project-scope-statement.md`](https://github.com/kanutocd/usb-camera-taytay/blob/main/project-requirements/project-scope-statement.md).

## Purpose

USB Camera Taytay is a Linux reference application that demonstrates how
camera media can reach Lunsaran through the Taytay edge bridge. It is also a
reference application for the private Sandbox Loom project, showing a
hardware-integrating project from scoped intent through bounded implementation,
quality evidence, and reviewed delivery.

## Staged outcomes

- **MVP1 — filesystem/file-segment ingestion:** accept completed JPEG and MP4
  files from an external producer or synthetic fixtures, identify stable
  content, and publish through Taytay's durable spool and ledger.
- **MVP2 — pure Rust V4L2 capture:** capture from one configured Linux device,
  produce complete artifacts, and reuse the same Taytay publication boundary.

Taytay owns durable local publication and upload orchestration; this app must
not create a parallel HTTP or TUS upload path. Ordinary tests remain independent
of camera hardware and live services. Optional live acceptance uses a provisioned
local Lunsaran/Brutus environment, test project, and scoped credential.

See the canonical scope statement for detailed requirements, exclusions,
acceptance evidence, risks, assumptions, and open-question records.
