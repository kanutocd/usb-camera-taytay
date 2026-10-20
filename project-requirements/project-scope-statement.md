# USB Camera Taytay — Project Scope Statement

## Document purpose

USB Camera Taytay is a small reference application that demonstrates how a
Linux machine with a USB camera can capture local media and deliver it to
Lunsaran through the Taytay edge bridge. It is also a reference application
for the private Sandbox Loom project in the `agent-platform` repository,
showing how Sandbox Loom can take a concrete hardware-integrating project from
scoped intent through bounded implementation, quality evidence, and reviewed
delivery.

This document defines two deliberately staged MVPs:

- **MVP1 — filesystem/file-segment ingestion:** use completed files produced by
  an existing camera or media tool and exercise Taytay's durable spool and
  upload path.
- **MVP2 — pure Rust V4L2 capture:** replace the external producer with a Rust
  capture loop that reads directly from `/dev/video*`.

The project is a local reference application and integration example. It is
not a hosted camera platform, surveillance product, video management system,
mobile application, or replacement for Lunsaran authorization and storage
services.

This scope statement is source material for a later requirements-analysis and
implementation-planning step. It distinguishes confirmed project intent from
proposals and open decisions. A proposal must not be treated as an approved
architecture choice until the project owner accepts it.

### Sandbox Loom reference-app role

Sandbox Loom remains separate from this application repository. The private
`agent-platform` repository supplies the Sandbox Loom control plane, policy,
workflow, intake, harness, and evidence conventions. USB Camera Taytay owns
its application code, public-facing documentation, source fixtures, Taytay
integration, and hardware-specific acceptance evidence.

The relationship is intentionally bounded:

```text
USB Camera Taytay scope and repository
        ↓ reviewed intake and proposed harness
private Sandbox Loom / agent-platform
        ↓ policy-governed, bounded execution and evidence
USB Camera Taytay implementation and release
```

Sandbox Loom may orchestrate approved work in the USB Camera Taytay checkout,
run the project's declared quality gates, preserve redacted evidence, and
require approval at configured boundaries. It does not replace the project's
technical ownership, Taytay's runtime contracts, Linux/V4L2 behavior,
Lunsaran authorization, code review, or release governance. Credentials,
hardware access, network access, and commit or publication authority remain
external capabilities that must be explicitly configured and approved.

## Intake authoring contract

The requirements analyst should convert this statement into the project's
normalized requirements or implementation brief. Preserve the source and
classify statements as:

- **facts:** constraints or capabilities explicitly stated here;
- **confirmed decisions:** choices required by this scope;
- **proposals:** candidate implementation choices requiring review;
- **assumptions:** working beliefs requiring validation; and
- **open questions:** unresolved decisions with an owner and review state.

Every open question should retain a stable identifier, owner, blocking status,
status, answer, and answer suggestion. An answer suggestion is not an answer;
keep the question open until the project owner or delegated technical owner
accepts it.

## 1. Project intent

### Problem

Developers need a small, reproducible way to verify that Taytay can turn media
produced at a Linux edge into durable, checksum-bound, resumable Lunsaran
uploads. A USB camera is available for local validation, but integrating camera
capture and cloud delivery in one first step would mix two different sources of
complexity.

### Desired outcome

Provide a runnable reference application that can:

- receive completed camera media segments from a local filesystem;
- identify only stable and complete files;
- publish them into Taytay's durable spool;
- preserve artifact identity, capture metadata, size, and checksum;
- upload through the supported `lunsaran-entregar` integration;
- survive temporary network failure, process restart, and repeated execution;
- expose safe operator diagnostics without leaking credentials or media; and
- later replace the filesystem producer with a pure Rust V4L2 capture loop
  without changing the durable upload boundary.

### Demonstration story

```text
USB camera or media producer
        ↓
completed file segment
        ↓
filesystem stability detector
        ↓
Taytay spool and durable ledger
        ↓
checksum-bound Entregar upload
        ↓
Lunsaran session and Brutus/TUS transfer
```

The reference demonstration should make the ownership boundaries visible:
the application owns local capture and buffering, Taytay owns local durability
and transfer orchestration, Lunsaran owns identity and project authorization,
and Brutus owns the TUS data plane and provider execution.

## 2. Users and stakeholders

### Primary users

- **Reference-app developer:** runs the application against a USB camera or
  file producer and changes one integration boundary at a time.
- **Rust integrator:** uses the project as a concrete example of composing a
  source adapter, `Spool`, scheduler, and uploader.
- **Edge operator:** configures the local spool, credential file, device source,
  and status checks.
- **Taytay maintainer:** verifies that application behavior remains aligned
  with the public Taytay crate API and security boundaries.
- **Sandbox Loom maintainer:** verifies that the reference-app workflow remains
  policy-governed, bounded, auditable, and separate from application authority.

### Ownership

- **Project owner:** approves scope, MVP acceptance, assumptions, and open
  questions.
- **Technical owner:** owns the application architecture, source boundary,
  runtime behavior, and release quality.
- **Security owner:** reviews credential handling, local permissions, logs,
  media exposure, and least-privilege operation.
- **Test owner:** owns deterministic fixtures, camera-independent tests, and
  optional hardware acceptance evidence.

## 3. MVP outcomes

### MVP1 — Filesystem/file-segment ingestion

MVP1 is complete when:

1. A documented command or producer can create synthetic or real media segment
   files in an input directory.
2. The application detects files only after an atomic rename, completion marker,
   or configured stability interval.
3. A file is published once into Taytay's spool with a stable artifact ID,
   source ID, media type, capture time when available, and metadata.
4. The application does not mutate or delete a source file before durable
   publication and successful upload completion.
5. Uploads use Taytay's checksum, retry, resume, bounded-worker, and durable
   ledger behavior rather than a second application-specific upload path.
6. Restarting the application does not lose a published file or duplicate its
   logical upload.
7. Offline or failed uploads remain inspectable and retryable without exposing
   credentials, signed URLs, or media payloads in diagnostics.
8. A new developer can run the reference application using synthetic fixtures
   without owning a camera or a live Lunsaran environment.

### MVP2 — Pure Rust V4L2 capture loop

MVP2 is complete when:

1. The application discovers and opens a configured `/dev/video*` device.
2. It negotiates a supported capture format, resolution, and frame interval.
3. It configures V4L2 streaming buffers and safely returns buffers to the
   driver after processing.
4. It writes frames into bounded, timestamped segments or another explicitly
   approved artifact format.
5. It emits an artifact only after the segment is complete and readable.
6. It feeds the same MVP1 filesystem/spool-to-upload boundary or an equivalent
   `Artifact` publication boundary without duplicating upload logic.
7. It handles camera disconnect, reconnect, malformed frames, and shutdown
   without corrupting completed artifacts.
8. It applies backpressure when the spool is full or upload throughput falls
   below capture throughput.
9. Hardware acceptance records the Linux version, camera model, negotiated
   format, segment settings, disconnect/reconnect result, and checksum.

MVP2 does not require support for every USB camera, arbitrary codecs, audio,
multi-camera synchronization, or production-grade video analytics.

## 4. Scope boundaries

### In scope

- One Linux reference application.
- One configured input directory for MVP1.
- One configured V4L2 device for MVP2.
- Synthetic file fixtures and deterministic local tests.
- Integration with the published `lunsaran-taytay` crate and
  `lunsaran-entregar` client.
- Durable local spool, ledger, checksums, retries, resume, cancellation, and
  operator pause/resume where provided by Taytay.
- Safe configuration, protected device credentials, redacted diagnostics, and
  a concise operator/developer guide.
- Optional local Lunsaran/Brutus acceptance when a provisioned environment is
  available.
- Sandbox Loom intake, proposed harness, bounded execution, quality evidence,
  and approval workflow artifacts maintained in the private `agent-platform`
  repository.

### Out of scope

- Building or changing Lunsaran, Brutus, or their authorization model.
- Building or changing Sandbox Loom itself; this project consumes its private
  workflow and harness capabilities as a reference application.
- A hosted control plane, user management, billing, or tenant administration.
- Mobile, Android, iOS, PWA, or browser camera clients.
- Cloud storage-provider credentials or direct object-storage uploads.
- Video analytics, object detection, facial recognition, or event inference.
- Audio capture, multi-camera synchronization, PTZ control, or camera fleet
  management.
- Supporting every V4L2 pixel format and codec in the first hardware MVP.
- A guarantee of lossless capture under arbitrary hardware, disk, or network
  overload.

## 5. Candidate features

Feature IDs are proposals until accepted in the implementation brief.

### MVP1 features

- `MVP1-FS-001`: configurable input, spool, and archive directories;
- `MVP1-FS-002`: atomic-rename and stable-file completion detection;
- `MVP1-FS-003`: deterministic artifact-ID derivation and duplicate handling;
- `MVP1-FS-004`: media-type and capture-time detection from filename or sidecar;
- `MVP1-FS-005`: Taytay spool publication and checksum verification;
- `MVP1-FS-006`: bounded observation and upload scheduling;
- `MVP1-FS-007`: completed-file cleanup only after durable upload completion;
- `MVP1-FS-008`: status, pause, resume, and redacted failure diagnostics; and
- `MVP1-FS-009`: synthetic fixture producer for camera-independent acceptance.

### MVP2 features

- `MVP2-V4L2-001`: configured device discovery and capability inspection;
- `MVP2-V4L2-002`: format, resolution, and frame-interval negotiation;
- `MVP2-V4L2-003`: memory-mapped or explicitly approved V4L2 buffer lifecycle;
- `MVP2-V4L2-004`: frame timestamping and bounded segment writing;
- `MVP2-V4L2-005`: capture-loop shutdown and device-reconnect behavior;
- `MVP2-V4L2-006`: capture-to-spool backpressure; and
- `MVP2-V4L2-007`: hardware acceptance evidence and troubleshooting guide.

### Developer experience

- `DEV-001`: one-command local setup with no production credentials;
- `DEV-002`: configuration example and protected-token instructions;
- `DEV-003`: deterministic mock acceptance against the published client;
- `DEV-004`: documentation build and local-link drift checks;
- `DEV-005`: Rust formatting, check, test, clippy, and coverage guidance; and
- `DEV-006`: release and compatibility notes for the Taytay crate version.
- `LOOM-001`: Sandbox Loom intake and normalized-scope traceability;
- `LOOM-002`: reviewed, project-specific harness and bounded execution plan;
- `LOOM-003`: quality-gate and evidence correlation for each implementation
  slice; and
- `LOOM-004`: explicit approval before consequential repository or release
  operations.

## 6. Technical principles and invariants

- The filesystem and V4L2 source boundaries must produce the same Taytay
  `Artifact` contract.
- A source must emit only complete, stable, readable artifacts.
- Artifact IDs remain stable across restarts and retries.
- Publication is atomic and checksum-bound before upload begins.
- The durable ledger is authoritative for local job state.
- The remote TUS offset is authoritative for resumable byte transfer.
- Retries are bounded and classified; permanent authorization or protocol
  failures must not be hidden as transport retries.
- Credentials are read from protected local files and never placed in logs,
  artifact metadata, filenames, fixtures, or acceptance output.
- The application must not bypass Taytay's `ArtifactUploader` boundary to make
  ad hoc HTTP or TUS requests.
- Camera hardware behavior must not be required for ordinary unit and mock
  acceptance tests.
- Sandbox Loom must not be granted implicit credentials, unrestricted network,
  publication, or commit authority by this scope statement.

## 7. Acceptance evidence

### MVP1 evidence

- A deterministic fixture run showing a file becoming stable, entering the
  spool, and completing a mock upload.
- Tests proving incomplete files are ignored and duplicate observations do not
  create duplicate logical jobs.
- Restart coverage proving durable jobs resume without data loss.
- Checksum, quota, pause/resume, retry, cancellation, and redaction coverage.
- A recorded command sequence for a real USB-camera-produced file segment.

### MVP2 evidence

- Device capability and negotiated-format output with no credential leakage.
- A recorded capture run with artifact checksums and segment metadata.
- Tests for buffer ownership, shutdown, malformed input, disconnect, and
  reconnect behavior.
- A hardware matrix entry for the available USB camera.
- Evidence that the capture loop uses the same Taytay durable upload path as
  MVP1.

## 8. Risks and mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Files are observed before they are complete | Corrupt or partial uploads | Require atomic rename, marker, or repeated stable observations |
| Capture rate exceeds upload or disk capacity | Backlog and eventual data loss | Bound the spool, expose pressure, and define retention/backpressure behavior |
| USB camera format is unsupported | MVP2 cannot capture | Inspect capabilities first and support one verified format initially |
| Camera disconnects during a segment | Corrupt segment or stalled loop | Close incomplete output safely and reconnect with bounded backoff |
| Credential or URL leakage | Security incident | Strict file permissions, redaction tests, and secret-free fixtures |
| Local and remote state diverge | Duplicate or lost uploads | Preserve artifact identity, checksum-bound resume state, and durable ledger records |
| Hardware-only tests become required for every change | Slow or unavailable CI | Keep MVP1 fixtures and mock acceptance as the default test layer |
| Codec implementation expands scope | Delayed MVP2 | Start with one camera-supported format and defer transcoding |

## 9. Assumptions

- The reference machine runs Linux and exposes the USB camera through V4L2.
- The project can use an external producer such as FFmpeg or GStreamer for
  MVP1 demonstrations, but MVP1 must remain valid with synthetic files.
- Taytay and the published Entregar crates remain the upload integration
  boundary.
- A Lunsaran organization, project, and scoped device credential can be
  provisioned for live acceptance, but they are not required for local tests.
- The private Sandbox Loom `agent-platform` repository remains available to
  review and execute the project's approved harness workflow.
- The available USB camera has at least one capture mode that can be verified
  with standard V4L2 inspection tools.
- The reference app is single-device and single-process for the MVPs.

## 10. Open questions

| ID | Question | Owner | Blocking | Status | Answer |
| --- | --- | --- | --- | --- | --- |
| `Q-001` | Which file format should MVP1 treat as the primary reference artifact: JPEG, MP4, or both? | Technical owner | Yes | Open | — |
| `Q-002` | Which capture format and resolution should MVP2 support first on the available USB camera? | Technical owner | Yes | Open | — |
| `Q-003` | Should MVP1 invoke an external producer, document a manual drop directory, or support both? | Project owner | No | Open | — |
| `Q-004` | Should MVP2 write encoded camera frames directly or depend on a Rust encoder? | Technical owner | Yes | Open | — |
| `Q-005` | What retention policy applies to successfully uploaded source files? | Project owner | Yes | Open | — |
| `Q-006` | Which live Lunsaran/Brutus environment and credential lifecycle will be used for hardware acceptance? | Security owner | No | Open | — |

## 11. Delivery sequence

1. Confirm this scope and resolve the MVP1 format and retention decisions.
2. Build the MVP1 fixture producer, filesystem observer, and Taytay adapter.
3. Prove deterministic mock acceptance and document the local runbook.
4. Run a real USB-camera-to-file-segment demonstration using an external
   producer.
5. Inspect the camera's V4L2 capabilities and select one supported mode.
6. Implement the MVP2 capture loop behind the same artifact publication and
   upload boundary.
7. Add hardware acceptance, reconnect evidence, and operational guidance.
8. Release the reference application only after the applicable quality gates
   and documentation checks pass, including the approved Sandbox Loom evidence
   and approval boundaries where that workflow is used.

## 12. Release and quality expectations

Every coherent change should update user-facing documentation and the
changelog where behavior or scope changes. The project should run, as
applicable:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
scripts/quality.sh
scripts/acceptance-mock.sh
```

Hardware acceptance is an additional evidence layer, not a substitute for
deterministic tests. When Sandbox Loom is used, its run, checkpoint, approval,
quality-gate, and commit evidence should be correlated with the corresponding
USB Camera Taytay slice without copying secrets or raw media into the evidence.
Do not commit credentials, captured media, device-specific secrets, generated
site output, or unverifiable hardware logs.

