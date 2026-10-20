# Architecture Constraints

## Technical constraints

- The product is one local Linux Rust CLI. MVP1 uses one configured input directory; MVP2 initially uses one configured `/dev/video*` device.
- MVP1 accepts completed files from a manual drop directory, an optional external producer, or synthetic fixtures. MVP2 uses a pure Rust V4L2 capture loop. Keep these as staged source implementations.
- Both sources emit the same Taytay `Artifact` contract. Publish only complete, stable, readable artifacts; publication is atomic and checksum-bound.
- Taytay's durable ledger is authoritative for local job state. The remote TUS offset is authoritative for resumable transfer.
- Use the published `lunsaran-taytay` and `lunsaran-entregar` integrations and Taytay's uploader. Do not add application-specific HTTP or TUS upload logic.
- Retries are bounded and classified. Permanent authorization or protocol failures must not be treated as transient transport failures.
- MVP2 begins with one camera-verified pixel format, resolution, and frame interval. Bound memory, segment size, spool use, and capture-to-upload pressure.

## Security and operational constraints

- Run unprivileged with a narrowly scoped writable spool. Keep device credentials in protected local files with no group or other access.
- Do not expose credentials, authorization headers, signed URLs, camera passwords, or captured media in logs, metadata, filenames, metrics, diagnostics, fixtures, acceptance output, or evidence.
- Device, network, filesystem, credential, publication, and commit capabilities are explicit external capabilities, not defaults granted by the application or its documentation.
- Synthetic fixtures and camera-independent tests are the default local acceptance path. A camera and live Lunsaran environment are not required for ordinary MVP1 tests.

## Scope constraints

This is a local reference application and integration example. A hosted control plane, user management, billing, tenant administration, analytics, audio, multi-camera operation, PTZ, and universal camera/codec support are outside the approved scope. The project does not replace Lunsaran, Brutus, Taytay, or Sandbox Loom.
