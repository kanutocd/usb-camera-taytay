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

Install Rust with Cargo, then run `bin/setup-dev` to install the pinned
documentation tools and project-local coverage tool. Run
`scripts/quality.sh` for Rust formatting, build, tests, and linting; run
`scripts/acceptance-mock.sh` for the camera-free CLI smoke check. MVP1
development and mock acceptance require no camera, production credentials, or
local Lunsaran/Brutus services. A provisioned local platform may be used for
optional end-to-end acceptance. The individual commands and documentation
gates are listed in [Quality gates](project-docs/harness/QUALITY_GATES.md).

## MVP1 filesystem ingestion

Copy [config.example.toml](config.example.toml) and set the input, spool, and
archive directories to paths writable by the current user. The CLI creates
these directories when they do not exist. `spool_quota_bytes` bounds durable
spool usage; individual input artifacts are limited to 64 MiB.

```sh
cargo run -- check-config config.toml
cargo run -- fixture config.toml
cargo run -- run-filesystem config.toml
cargo run -- status config.toml
cargo run -- pause ARTIFACT_ID config.toml
cargo run -- resume ARTIFACT_ID config.toml
```

`fixture` atomically places deterministic JPEG and MP4 fixtures in the input
directory. The MP4 is a minimal synthetic container sample for ingestion and
media-type acceptance; it is not a camera recording. External producers can
write to a `.part` filename and atomically rename it to the final
filename when complete; `.part` files are ignored. `run-filesystem` scans three
times with one second intervals so files that appear during the scan can still
receive two stable observations. It publishes stable files through Taytay's
durable spool and ledger, and verifies each published checksum. Artifact IDs
are content-derived; a file already represented in the ledger with the same
full checksum is skipped as a verified duplicate on later runs. A conflicting
checksum for an existing ID is reported as an error. Once the Taytay ledger
reports an artifact as `Completed` or `Retained`, a later run moves its
checksum-verified source file and optional JSON sidecar from input to the
configured archive directory. Pending, failed, paused, and uploading artifacts
remain in input. The archive keeps the source copy. This command does not start
an uploader or require camera hardware or Lunsaran credentials.

## MVP2 V4L2 MJPEG segments

On Linux, `run-v4l2` opens the configured device, requests the configured
capture mode, maps four driver-owned buffers, queues them for streaming, copies
each dequeued frame, and returns that buffer to the driver. Set
`v4l2_device`, `v4l2_width`, `v4l2_height`, and the four-character
`v4l2_pixel_format` in the config after inspecting the device's capabilities.
This increment writes complete MJPEG frames as timestamped JPEG files into the
configured input directory, with matching capture-time sidecars. Files are
synced and atomically renamed from `.part` paths, so the existing filesystem
ingestion command can publish them through Taytay. Configure
`v4l2_segment_frames` (default 30, maximum 300) and
`v4l2_segment_max_bytes` (default 64 MiB) to bound each run. Only `MJPG` is
currently written; other capture formats are rejected. Capture also checks
Taytay spool usage and pending JPEGs in the input directory before admitting
each frame. It stops at the configured spool quota, leaving completed frames
available for `run-filesystem` to publish through Taytay.

```sh
cargo run -- run-v4l2 config.toml
```

The command allows up to three reconnect cycles, with up to three open attempts
per cycle using the configured device and capture mode. `Ctrl-C` and `SIGTERM`
stop capture after the current frame is returned; completed JPEGs remain
published and incomplete frames are discarded. Mappings and streaming state are
released on shutdown and frame errors. Camera-independent tests cover frame
validation, disconnect classification, JPEG completeness, timestamp sidecars,
atomic file publication, and stopping capture when spool capacity is exhausted.

### Hardware acceptance matrix

| Linux host | Camera | Reported mode | Segment settings | Capture and delivery result |
| --- | --- | --- | --- | --- |
| Linux Mint 22.3, kernel 7.0.0-28-generic, x86_64 | A4ech FHD 1080P PC Camera (`/dev/video0`) | FFmpeg reports MJPEG at 1920x1080, 1280x720, 1024x768, 800x600, 640x480, and 320x240; frame interval not verified | Attempted 640x480 MJPEG, one frame, 64 MiB maximum | Capture could not initialize; the current user is not in the `video` group that owns the device. No artifact, checksum, reconnect result, or Taytay publication was verified. |

Before recording a successful hardware run, inspect the configured device with
`v4l2-ctl --all --list-formats-ext`, choose a reported mode and frame interval,
and ensure the operator can open the device. Then run `run-v4l2`, record the
Linux version, camera model, negotiated mode, segment settings, and SHA-256 of
each completed JPEG. Run `run-filesystem` and confirm the artifact appears in
the Taytay ledger before marking delivery verified. Keep captured media and
credentials out of committed evidence.

If capture reports that it could not initialize, check `ls -l /dev/video*`
and the current user's device permissions, then confirm no other process has
exclusive access to the camera. On systems that restrict the node to the
`video` group, arrange access through the host's normal device policy and start
a new login session before retrying. If the requested mode is rejected, use a
format, resolution, and frame interval reported by the device. Record reconnect
as unverified until a physical disconnect and reconnect has been observed.

`status` reads artifact IDs and lifecycle states from the durable ledger.
`pause` and `resume` persist state changes through Taytay's ledger; resume makes
the artifact eligible for a later uploader run. Command errors omit underlying
paths, credentials, and provider diagnostics.

MVP1 accepts `.jpg`/`.jpeg` images as `image/jpeg` and `.mp4` videos as
`video/mp4`; other file types are ignored. To attach a capture time, either
name a file with a UTC timestamp suffix such as
`front-door_20241021T123456Z.mp4`, or add a same-stem JSON sidecar such as
`front-door.mp4` with `front-door.json` containing
`{"captured_at":"2024-10-21T12:34:56Z"}`. The sidecar value takes precedence
over the filename suffix. Timestamps must use `YYYY-MM-DDTHH:MM:SSZ`; invalid
sidecars fail the scan rather than silently publishing incorrect metadata.

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
