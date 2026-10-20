---
name: usb-camera-taytay
description: Build and maintain the USB Camera Taytay Rust CLI reference application across its filesystem and V4L2 MVPs.
version: "0.1"
---

# USB Camera Taytay project skill

Use this skill when working in the
`/home/ken/Documents/mi-proyekto/usb-camera-taytay` repository. USB Camera
Taytay is a Rust CLI reference application for capturing or receiving local
camera media and delivering it to Lunsaran through the Taytay edge bridge.

This project is also a reference application for the private Sandbox Loom
project in `/home/ken/Documents/mi-proyekto/agent-platform`. Sandbox Loom
governs intake, policy, bounded execution, quality evidence, approvals, and
commits. This skill provides project context; it does not grant authority or
override repository instructions, Sandbox Loom policy, or human approval.

## Project shape

The application has two deliberately bounded MVPs:

- **MVP1 — filesystem/file-segment ingestion:** watch for stable, completed
  files produced by an external camera or media tool, publish them through
  Taytay's durable spool, and upload them through the supported Entregar path.
- **MVP2 — pure Rust V4L2 capture:** open one configured `/dev/video*` device,
  negotiate one verified capture mode, capture frames into bounded segments,
  and emit artifacts through the same durable spool and upload boundary.

The application is not a hosted camera platform, surveillance product, mobile
application, PWA, video-management system, analytics service, or replacement
for Lunsaran or Brutus.

## Ownership boundaries

```text
USB camera or external media producer
        ↓
USB Camera Taytay Rust CLI
        ↓
Taytay Artifact + Spool + durable ledger
        ↓
EntregarUploader / ArtifactUploader
        ↓
Lunsaran authorization and Brutus/TUS transfer
```

- USB Camera Taytay owns source observation/capture, local configuration, and
  reference-app orchestration.
- Taytay owns durable publication, checksums, ledger state, retries, resume,
  cancellation, bounded workers, and upload integration.
- Lunsaran owns identity, organization/project authorization, and upload-session
  policy.
- Brutus owns the TUS data plane and provider execution.
- Sandbox Loom owns governed execution, policy decisions, approvals, quality
  evidence, and commit boundaries.

Do not bypass Taytay's `ArtifactUploader` boundary with application-specific
HTTP or TUS code.

## Project workflow

When implementing work:

1. Read `project-requirements/project-scope-statement.md` and the current
   project brief or harness artifacts before changing code.
2. Identify whether the work belongs to MVP1, MVP2, shared infrastructure, or
   documentation/tooling.
3. Preserve the separation between file ingestion and V4L2 capture.
4. Inspect the relevant implementation path, tests, and invariants.
5. Make the smallest coherent change.
6. Update consumer and operator documentation when behavior or contracts
   change.
7. Add a changelog entry for user-visible or architectural changes.
8. Run the narrowest relevant checks, then the declared quality gates.
9. Preserve evidence and request approval at the configured Sandbox Loom
   boundary before consequential commit or release operations.

Do not silently resolve open questions from the scope statement. Record a
proposal and request an authorized decision instead.

## MVP1 rules

The filesystem producer must:

- observe only complete, readable files;
- use an atomic rename, completion marker, or repeated stable observations;
- derive stable artifact IDs that survive restarts and retries;
- preserve media type, capture time, source identity, and bounded metadata;
- publish through Taytay's `Spool` and ledger;
- leave source files untouched until the configured completion policy permits
  cleanup after durable upload completion; and
- remain testable with synthetic fixtures without a camera or live Lunsaran.

MVP1 acceptance should cover incomplete-file suppression, duplicate
observations, checksum verification, quota pressure, retry/resume, restart
recovery, pause/resume, cleanup, cancellation, and output redaction.

An external producer such as FFmpeg or GStreamer may be used for a real-camera
demonstration, but it is not part of Taytay's upload boundary and must not be
required for ordinary tests.

## MVP2 rules

The V4L2 implementation must:

- target one configured `/dev/video*` device initially;
- inspect capabilities before selecting a mode;
- support one verified pixel format, resolution, and frame interval first;
- make buffer ownership and return-to-driver behavior explicit;
- bound memory, segment size, and capture-to-spool pressure;
- emit only complete, readable artifacts;
- handle shutdown, malformed frames, disconnect, and reconnect safely; and
- reuse MVP1's artifact publication and Taytay upload path.

Do not expand MVP2 into universal camera, codec, audio, PTZ, synchronization,
or video-analytics support without an approved scope change.

Hardware acceptance must record the Linux version, camera model, negotiated
mode, segment settings, reconnect result, and artifact checksum. Never record
credentials or raw customer media in committed evidence.

## CLI expectations

The reference app should remain a Rust CLI. Keep command names and behavior
consumer-oriented and document them before treating them as stable. The
intended command shape is:

```text
usb-camera-taytay check-config CONFIG
usb-camera-taytay status CONFIG
usb-camera-taytay run-filesystem CONFIG
usb-camera-taytay run-v4l2 CONFIG
usb-camera-taytay pause ARTIFACT_ID CONFIG
usb-camera-taytay resume ARTIFACT_ID CONFIG
```

Do not assume this command surface is approved merely because it is listed
here; reconcile it with the current brief and implementation plan.

## Security boundaries

- Never commit or print device credentials, Lunsaran tokens, signed URLs,
  authorization headers, camera passwords, or captured media.
- Read credentials from protected local files and require no group/other access.
- Keep credentials out of artifact metadata, filenames, metrics, diagnostics,
  fixtures, and Sandbox Loom evidence.
- Do not grant unrestricted network, device, filesystem, publication, or commit
  access through project documentation.
- Treat V4L2 device access and live Lunsaran acceptance as explicit optional
  capabilities, not defaults.
- Keep the application unprivileged and use a narrowly scoped writable spool.

## Quality and validation

Run the narrowest relevant test first. For the Rust workspace or shared Taytay
integration, use:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
scripts/quality.sh
scripts/acceptance-mock.sh
```

For documentation changes, also run the project's documented Zensical and
Markdown checks. For MVP2, add hardware acceptance only when the configured
camera and device capability are explicitly available. A hardware failure
must not be misreported as a code-quality failure.

## Documentation and evidence

Keep the following aligned:

- `project-requirements/project-scope-statement.md` — stakeholder scope;
- `specifications/project-brief.yml` — schema-backed intake brief;
- `.sandbox-loom/` — executable Sandbox Loom harness and status;
- `SANDBOX_LOOM_GUIDE.md` — project workflow guidance; and
- consumer/operator documentation and changelog.

Generated harness artifacts are proposals until reviewed and approved. Public
documentation must not publish private harness instructions, raw evidence,
credentials, captured media, or provider-specific secrets.

## Sandbox Loom authority

Sandbox Loom remains authoritative for normalization, harness generation,
review, approval, policy, quality gates, evidence, and commits. This skill
must not:

- approve its own changes;
- invent answers to blocking intake questions;
- grant itself credentials or external capabilities;
- bypass a failed quality gate or missing verified commit;
- combine MVP1 and MVP2 into an unbounded autonomous run; or
- publish a crate, binary, documentation site, or captured artifact without
  the applicable approval.

When a request conflicts with this skill, repository instructions or Sandbox
Loom policy take precedence.

