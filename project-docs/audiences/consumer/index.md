# USB Camera Taytay — User Guide

USB Camera Taytay is a local Linux Rust CLI reference application. It accepts
completed JPEG and MP4 files (MVP1), and can capture MJPEG frames from one
configured V4L2 camera into JPEG files (MVP2). Both paths publish artifacts to
Taytay's durable local spool and ledger.

## Set up local directories

Install Rust with Cargo. Copy the repository's
[`config.example.toml`](https://github.com/kanutocd/usb-camera-taytay/blob/main/config.example.toml)
to a private working location and set `input_dir`, `spool_dir`, and
`archive_dir` to directories writable by your user. Set `spool_quota_bytes` to
the maximum space Taytay may use. The CLI creates these directories when
needed. Keep credentials out of this configuration file.

Check the configuration and create deterministic sample inputs:

```sh
cargo run -- check-config /path/to/config.toml
cargo run -- fixture /path/to/config.toml
```

The fixture command writes a synthetic JPEG and minimal MP4 sample. The MP4 is
for ingestion checks, not a camera recording.

## Ingest completed files (MVP1)

Run the filesystem scan and inspect the local ledger:

```sh
cargo run -- run-filesystem /path/to/config.toml
cargo run -- status /path/to/config.toml
```

The scan observes files over three polls and publishes only stable, readable
`.jpg`, `.jpeg`, and `.mp4` files. An external producer should write to a
`.part` filename and atomically rename it to its final supported name when
writing is complete. Files with matching content already in the ledger are
skipped as verified duplicates. The CLI does not start an uploader or connect
to Lunsaran; a separate, provisioned Taytay uploader is required for live
transfer.

After a later run sees `Completed` or `Retained` in the Taytay ledger, it moves
the source file and optional JSON sidecar to the configured archive directory.
Pending or failed artifacts remain in the input directory.

## Capture from V4L2 (MVP2)

Inspect the camera's supported modes with `v4l2-ctl --all --list-formats-ext`.
Set `v4l2_device`, `v4l2_width`, `v4l2_height`, and
`v4l2_pixel_format` in the configuration. This version writes MJPEG (`MJPG`)
frames as complete JPEG files with timestamp sidecars. Capture is bounded by
`v4l2_segment_frames` and `v4l2_segment_max_bytes`; the resulting files use the
same filesystem ingestion path as MVP1.

```sh
cargo run -- run-v4l2 /path/to/config.toml
cargo run -- run-filesystem /path/to/config.toml
```

Camera access depends on Linux device permissions and a mode supported by the
camera. Disconnect/reconnect behavior should be considered unverified until
observed on the target hardware.

## Local artifact controls

`status` reads states from the durable Taytay ledger. `pause` and `resume`
persist state changes but do not start an uploader. Fixture and camera capture
checks do not need credentials or a running Lunsaran platform.

For optional live acceptance, use a dedicated test project and a narrowly
scoped credential in a separate owner-only file. Never put the credential in
TOML, command arguments, shell history, logs, or evidence. The current USB
Camera Taytay CLI does not read a token file; configure credentials only in the
separate Taytay upload process.

See the [Reference App Guide](../technical/index.md) for the integration
boundaries and links to Taytay's reusable crate documentation.
