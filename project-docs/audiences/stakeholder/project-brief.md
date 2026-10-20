# Schema-backed Project Brief

The canonical, schema-backed brief is
[`specifications/project-brief.yml`](https://github.com/kanutocd/usb-camera-taytay/blob/main/specifications/project-brief.yml).
This page gives product-team readers a short orientation; the YAML remains the
authoritative structured intake record.

## Objectives and milestones

The brief records the project's dual role: demonstrate USB-camera media
delivery to Lunsaran through Taytay and serve as a private Sandbox Loom
reference application. Its milestones cover MVP1 filesystem ingestion and
Taytay publication, local and hardware acceptance, MVP2 capability inspection
and V4L2 capture, and reviewed delivery.

## Confirmed scope decisions

- MVP1 supports both JPEG and MP4 with deterministic synthetic fixtures.
- MVP1 documents a manual drop directory and an optional external-producer
  demonstration.
- Successfully uploaded source files are retained in the configured archive
  until an operator removes them.
- Optional live acceptance uses a provisioned local Lunsaran/Brutus environment,
  a dedicated test project, and a narrowly scoped credential that is rotated or
  revoked after acceptance.
- MVP2 prefers a camera-provided encoded format when suitable; its exact
  supported mode depends on inspecting the available camera.

Questions Q-001 through Q-006 have recorded answers. The precise MVP2 format
and resolution remain dependent on hardware capability inspection. Suggestions
and proposed feature additions in the brief retain their proposal status until
approved.

## Ownership and constraints

The application owns local capture, buffering, and orchestration. Taytay owns
the durable spool, ledger, checksums, retry, resume, cancellation, and upload
boundary. Lunsaran owns identity and project authorization; Brutus owns TUS
transfer and provider execution. Keep credentials, signed URLs, raw media, and
private Sandbox Loom policy out of public documentation and evidence.
