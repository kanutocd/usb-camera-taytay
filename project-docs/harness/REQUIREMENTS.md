# usb-camera-taytay — Requirements

## Objective

Provide a small, reproducible Linux Rust CLI reference application that captures or receives local USB-camera media and delivers durable, checksum-bound, resumable uploads to Lunsaran through Taytay, while keeping filesystem ingestion and pure Rust V4L2 capture as deliberately staged MVPs.

## Requirements

```yaml
---
functional:
- id: MVP1-FS-001
  source_feature: MVP1-FS-001
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
  description: Configurable input, spool, and archive directories.
  acceptance_criteria: []
- id: MVP1-FS-002
  source_feature: MVP1-FS-002
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
  description: Atomic-rename and stable-file completion detection.
  acceptance_criteria: []
- id: MVP1-FS-003
  source_feature: MVP1-FS-003
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  description: Deterministic artifact-ID derivation and duplicate handling.
  acceptance_criteria: []
- id: MVP1-FS-004
  source_feature: MVP1-FS-004
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
  description: Media-type and capture-time detection from filename or sidecar.
  acceptance_criteria: []
- id: MVP1-FS-005
  source_feature: MVP1-FS-005
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
  description: Taytay spool publication and checksum verification.
  acceptance_criteria: []
- id: MVP1-FS-006
  source_feature: MVP1-FS-006
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  description: Bounded observation and upload scheduling.
  acceptance_criteria: []
- id: MVP1-FS-007
  source_feature: MVP1-FS-007
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  description: Completed-file cleanup only after durable upload completion.
  acceptance_criteria: []
- id: MVP1-FS-008
  source_feature: MVP1-FS-008
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-003
  description: Status, pause, resume, and redacted failure diagnostics.
  acceptance_criteria: []
- id: MVP1-FS-009
  source_feature: MVP1-FS-009
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-003
  description: Synthetic fixture producer for camera-independent acceptance.
  acceptance_criteria: []
- id: MVP1-FS-010
  source_feature: MVP1-FS-010
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-003
  description: Taytay durable-ledger recovery, bounded retry/resume, and repeated-execution
    deduplication.
  acceptance_criteria: []
- id: MVP2-V4L2-001
  source_feature: MVP2-V4L2-001
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-005
  description: Configured device discovery and capability inspection.
  acceptance_criteria: []
- id: MVP2-V4L2-002
  source_feature: MVP2-V4L2-002
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-005
  description: Format, resolution, and frame-interval negotiation.
  acceptance_criteria: []
- id: MVP2-V4L2-003
  source_feature: MVP2-V4L2-003
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  description: Memory-mapped or explicitly approved V4L2 buffer lifecycle.
  acceptance_criteria: []
- id: MVP2-V4L2-004
  source_feature: MVP2-V4L2-004
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  - milestone-007
  description: Frame timestamping and bounded segment writing that emits only complete,
    readable artifacts.
  acceptance_criteria: []
- id: MVP2-V4L2-005
  source_feature: MVP2-V4L2-005
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  - milestone-007
  description: Safe malformed-frame handling, capture-loop shutdown, and device disconnect/reconnect
    behavior.
  acceptance_criteria: []
- id: MVP2-V4L2-006
  source_feature: MVP2-V4L2-006
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  description: Capture-to-spool backpressure.
  acceptance_criteria: []
- id: MVP2-V4L2-007
  source_feature: MVP2-V4L2-007
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-007
  description: Hardware acceptance evidence and troubleshooting guide.
  acceptance_criteria: []
- id: MVP2-V4L2-008
  source_feature: MVP2-V4L2-008
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  - milestone-007
  description: Reuse MVP1's Taytay Artifact publication, durable spool, and upload
    boundary without application-specific upload logic.
  acceptance_criteria: []
- id: DEV-001
  source_feature: DEV-001
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
  description: One-command local setup with no production credentials.
  acceptance_criteria: []
- id: DEV-002
  source_feature: DEV-002
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
  description: Configuration example and protected-token instructions.
  acceptance_criteria: []
- id: DEV-003
  source_feature: DEV-003
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
  description: Deterministic mock acceptance against the published client.
  acceptance_criteria: []
- id: DEV-004
  source_feature: DEV-004
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
  description: Documentation build and local-link drift checks.
  acceptance_criteria: []
- id: DEV-005
  source_feature: DEV-005
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
  description: Rust formatting, check, test, clippy, and coverage guidance.
  acceptance_criteria: []
- id: DEV-006
  source_feature: DEV-006
  mvp_ids:
  - mvp-001
  milestone_ids: []
  description: Release and compatibility notes for the Taytay crate version.
  acceptance_criteria: []
- id: LOOM-001
  source_feature: LOOM-001
  mvp_ids: []
  milestone_ids:
  - milestone-008
  description: Sandbox Loom intake and normalized-scope traceability.
  acceptance_criteria: []
- id: LOOM-002
  source_feature: LOOM-002
  mvp_ids: []
  milestone_ids:
  - milestone-008
  description: Reviewed project-specific harness and bounded execution plan.
  acceptance_criteria: []
- id: LOOM-003
  source_feature: LOOM-003
  mvp_ids: []
  milestone_ids:
  - milestone-008
  description: Quality-gate and evidence correlation for each implementation slice.
  acceptance_criteria: []
- id: LOOM-004
  source_feature: LOOM-004
  mvp_ids: []
  milestone_ids:
  - milestone-008
  description: Explicit approval before consequential repository or release operations.
  acceptance_criteria: []
non_functional:
- id: tenancy.model
  requirement: Lunsaran organization and project authorization; acceptance uses a
    provisioned test project and scoped device credential when available.
  source: tenancy.model
  acceptance_criteria: []
- id: operations.recovery
  requirement: Preserve durable jobs across restart; handle temporary network failure
    and repeated execution; hardware reconnect behavior is covered for MVP2.
  source: operations.recovery
  acceptance_criteria: []
```

## Assumptions

- Reference machine runs Linux and exposes a USB camera through V4L2.
- MVP1 may use an external producer such as FFmpeg or GStreamer, but remains valid with synthetic files.
- Taytay and the published Entregar crates remain the upload integration boundary.
- A Lunsaran organization, project, and scoped device credential can be provisioned for live acceptance; they are not required for ordinary local tests.
- Private Sandbox Loom / sandbox-loom remains available to review and execute the project's approved harness workflow.
- Available USB camera has at least one capture mode verifiable with standard V4L2 inspection tools.
- Reference app is single-device and single-process for both MVPs.


## Open questions

- None recorded


## Resolved decisions

- Which file format should MVP1 treat as the primary reference artifact: JPEG, MP4, or both? — Support both JPEG and MP4 as MVP1 reference artifacts, while using small
      synthetic fixtures for deterministic acceptance. This exercises both
      still-image and video-segment handling without requiring a camera; the
      trade-off is added fixture and media-type coverage compared with choosing
      one format.
- Which capture format and resolution should MVP2 support first on the available USB camera? — Select one format and resolution from the available camera's
    reported V4L2 capabilities, favoring a mode that can be captured and segmented
    reliably with the reference hardware. This follows the scope's one-verified-mode
    MVP boundary; the specific mode remains undecided until the camera is inspected,
    and supporting only that mode limits initial hardware compatibility.
- Should MVP1 invoke an external producer, document a manual drop directory, or support both? — Support a documented manual drop directory and document an external
    producer command as an optional demonstration path. This keeps ordinary MVP1 setup
    simple while retaining a real-camera demonstration; the trade- off is that producer
    lifecycle management is not automated.
- Should MVP2 write encoded camera frames directly or depend on a Rust encoder? — Prefer writing a camera-provided encoded format directly when the
    available device exposes a suitable mode; otherwise evaluate a Rust
    encoder only after confirming the required artifact format. This avoids
    adding encoding complexity when the camera can provide usable encoded
    frames; the trade-off is that supported formats and resulting artifacts
    depend on the selected camera mode.
- What retention policy applies to successfully uploaded source files? — Retain successfully uploaded source files in the configured archive
    until an operator removes them, rather than deleting immediately. This preserves
    a local recovery copy; the trade-off is additional disk usage and a need for a
    later cleanup policy.
- Which live Lunsaran/Brutus environment and credential lifecycle will be used for hardware acceptance? — Use a local provisioned Lunsaran/Brutus environment with a dedicated
    test project and narrowly scoped device credential, and define rotation or revocation
    after acceptance. This keeps live validation separate from production access;
    the trade-off is environment and credential setup overhead.


> Generation status: proposed. No unresolved open questions remain; facts, template defaults, and proposals still require review.
