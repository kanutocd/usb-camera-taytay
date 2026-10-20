# Solution Strategy

```yaml
---
milestones:
- id: milestone-001
  title: Confirm scope and resolve initial MVP decisions
  exit_criteria: Confirm this scope and resolve MVP1 format and source-file retention
    decisions before implementation proceeds on those choices.
- id: milestone-002
  title: Build MVP1 filesystem ingestion and Taytay adapter
  exit_criteria: Build the MVP1 fixture producer, filesystem observer, and Taytay
    adapter, including stable artifact publication, checksum verification, and durable-ledger
    integration.
  mvp_ids:
  - mvp-001
  feature_ids:
  - MVP1-FS-001
  - MVP1-FS-002
  - MVP1-FS-003
  - MVP1-FS-004
  - MVP1-FS-005
  - MVP1-FS-006
  - MVP1-FS-007
  - MVP1-FS-008
  - MVP1-FS-009
  - MVP1-FS-010
- id: milestone-003
  title: Prove deterministic mock acceptance and document the local runbook
  exit_criteria: Demonstrate deterministic fixture and mock acceptance, including
    incomplete-file suppression, duplicate-observation handling, checksum and quota
    behavior, pause/resume, retry and resume across restart, cancellation, and credential/media
    redaction. Document reproducible local setup and operation without production
    credentials. When a local Lunsaran/Brutus environment is provisioned, verify service
    readiness and a successful end-to-end upload using a test project and scoped device
    credential; retain only redacted readiness and result evidence, with no credentials,
    signed URLs, or media.
  mvp_ids:
  - mvp-001
  feature_ids:
  - MVP1-FS-008
  - MVP1-FS-009
  - MVP1-FS-010
  - DEV-001
  - DEV-002
  - DEV-003
  - DEV-004
  - DEV-005
- id: milestone-004
  title: Demonstrate USB-camera-to-file-segment ingestion
  exit_criteria: Run and record a real USB-camera-to-file-segment demonstration using
    an external producer.
  mvp_ids:
  - mvp-001
  feature_ids:
  - MVP1-FS-001
  - MVP1-FS-002
  - MVP1-FS-004
  - MVP1-FS-005
- id: milestone-005
  title: Inspect and select a V4L2 capture mode
  exit_criteria: Inspect available camera capabilities and record the Linux version,
    camera model, and one supported pixel format, resolution, and frame interval selected
    for MVP2.
  mvp_ids:
  - mvp-002
  feature_ids:
  - MVP2-V4L2-001
  - MVP2-V4L2-002
- id: milestone-006
  title: Implement the MVP2 V4L2 capture loop
  exit_criteria: Implement pure Rust capture for the selected V4L2 mode, configure
    and safely return streaming buffers, and write bounded timestamped segments that
    are complete and readable before publication. Handle malformed frames, shutdown,
    disconnect, and reconnect without corrupting completed artifacts. Apply backpressure
    at the spool boundary and publish through the same Taytay Artifact, spool, and
    upload path used by MVP1, without application-specific upload logic.
  mvp_ids:
  - mvp-002
  feature_ids:
  - MVP2-V4L2-003
  - MVP2-V4L2-004
  - MVP2-V4L2-005
  - MVP2-V4L2-006
  - MVP2-V4L2-008
- id: milestone-007
  title: Add hardware acceptance and operational guidance
  exit_criteria: Record a hardware capture run with Linux version, camera model, negotiated
    mode, segment settings, artifact checksums, and disconnect/reconnect results.
    Verify that the captured artifact uses MVP1's Taytay durable upload path, maintain
    a hardware matrix entry for the available camera, and document troubleshooting
    guidance.
  mvp_ids:
  - mvp-002
  feature_ids:
  - MVP2-V4L2-004
  - MVP2-V4L2-005
  - MVP2-V4L2-007
  - MVP2-V4L2-008
- id: milestone-008
  title: Release after applicable quality and governance evidence
  exit_criteria: Release only after applicable quality gates and documentation checks
    pass, including approved Sandbox Loom evidence and approval boundaries when that
    workflow is used.
features:
- id: MVP1-FS-001
  description: Configurable input, spool, and archive directories.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
- id: MVP1-FS-002
  description: Atomic-rename and stable-file completion detection.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
- id: MVP1-FS-003
  description: Deterministic artifact-ID derivation and duplicate handling.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
- id: MVP1-FS-004
  description: Media-type and capture-time detection from filename or sidecar.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
- id: MVP1-FS-005
  description: Taytay spool publication and checksum verification.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-004
- id: MVP1-FS-006
  description: Bounded observation and upload scheduling.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
- id: MVP1-FS-007
  description: Completed-file cleanup only after durable upload completion.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
- id: MVP1-FS-008
  description: Status, pause, resume, and redacted failure diagnostics.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-003
- id: MVP1-FS-009
  description: Synthetic fixture producer for camera-independent acceptance.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-003
- id: MVP1-FS-010
  description: Taytay durable-ledger recovery, bounded retry/resume, and repeated-execution
    deduplication.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-002
  - milestone-003
- id: MVP2-V4L2-001
  description: Configured device discovery and capability inspection.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-005
- id: MVP2-V4L2-002
  description: Format, resolution, and frame-interval negotiation.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-005
- id: MVP2-V4L2-003
  description: Memory-mapped or explicitly approved V4L2 buffer lifecycle.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
- id: MVP2-V4L2-004
  description: Frame timestamping and bounded segment writing that emits only complete,
    readable artifacts.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  - milestone-007
- id: MVP2-V4L2-005
  description: Safe malformed-frame handling, capture-loop shutdown, and device disconnect/reconnect
    behavior.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  - milestone-007
- id: MVP2-V4L2-006
  description: Capture-to-spool backpressure.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
- id: MVP2-V4L2-007
  description: Hardware acceptance evidence and troubleshooting guide.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-007
- id: MVP2-V4L2-008
  description: Reuse MVP1's Taytay Artifact publication, durable spool, and upload
    boundary without application-specific upload logic.
  mvp_ids:
  - mvp-002
  milestone_ids:
  - milestone-006
  - milestone-007
- id: DEV-001
  description: One-command local setup with no production credentials.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
- id: DEV-002
  description: Configuration example and protected-token instructions.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
- id: DEV-003
  description: Deterministic mock acceptance against the published client.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
- id: DEV-004
  description: Documentation build and local-link drift checks.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
- id: DEV-005
  description: Rust formatting, check, test, clippy, and coverage guidance.
  mvp_ids:
  - mvp-001
  milestone_ids:
  - milestone-003
- id: DEV-006
  description: Release and compatibility notes for the Taytay crate version.
  mvp_ids:
  - mvp-001
- id: LOOM-001
  description: Sandbox Loom intake and normalized-scope traceability.
  milestone_ids:
  - milestone-008
- id: LOOM-002
  description: Reviewed project-specific harness and bounded execution plan.
  milestone_ids:
  - milestone-008
- id: LOOM-003
  description: Quality-gate and evidence correlation for each implementation slice.
  milestone_ids:
  - milestone-008
- id: LOOM-004
  description: Explicit approval before consequential repository or release operations.
  milestone_ids:
  - milestone-008
```
