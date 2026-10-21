# Verify a local Lunsaran integration

Use this procedure to verify the handoff from USB Camera Taytay to a separately
running Taytay uploader and a local Lunsaran platform. It uses deterministic
fixtures, so it does not require a USB camera. USB Camera Taytay publishes
artifacts to the local spool; Taytay performs the remote upload.

## Prerequisites

- A running local Lunsaran environment with its required services.
- The [Taytay repository](https://github.com/kanutocd/taytay) checked out and
  its Lunsaran uploader configured for that environment.
- A test organization and project, plus a valid device credential enrolled
  for that project.
- A credential file readable only by its owner. Do not put credentials in
  TOML, command arguments, shell history, logs, or test evidence.
- The same Taytay version and spool directory configured for both processes.

This procedure requires platform credentials and configuration that this
repository cannot provide. Follow the local platform's operator instructions
to create the test project and enroll a credential.

## 1. Publish fixtures to the spool

Create a private USB Camera Taytay configuration. The directories below are
examples; choose locations writable by your user.

```toml
input_dir = "/tmp/usb-camera-live/input"
spool_dir = "/tmp/usb-camera-live/spool"
archive_dir = "/tmp/usb-camera-live/archive"
source_id = "usb-camera-live-check"
spool_quota_bytes = 1073741824
```

Save it as `/path/to/usb-camera.toml`, then publish and inspect the fixtures:

```sh
cargo run -- check-config /path/to/usb-camera.toml
cargo run -- fixture /path/to/usb-camera.toml
cargo run -- run-filesystem /path/to/usb-camera.toml
cargo run -- status /path/to/usb-camera.toml
```

The first status output should list two artifacts in `Published` state. The
CLI output does not include source filenames or credentials.

## 2. Point Taytay at the same spool

Create a separate Taytay configuration. Set `spool_dir` to the exact directory
used above and use the local platform's actual URL and test project IDs.

```toml
spool_dir = "/tmp/usb-camera-live/spool"
quota_bytes = 1073741824
upload_workers = 1
lunsaran_base_url = "http://127.0.0.1:8080"
organization_id = "YOUR_TEST_ORGANIZATION_UUID"
project_id = "YOUR_TEST_PROJECT_UUID"
token_file = "/secure/path/device-token"
```

The URL and UUIDs are examples, not defaults. Protect the token file with
owner-only permissions and use the credential enrolled for this project.

## 3. Run the uploader and check both ledgers

From the Taytay repository, build and run its uploader with the configuration
you created:

```sh
cargo build -p lunsaran-taytay --bin taytay
target/debug/taytay /path/to/taytay-live.toml
target/debug/taytay --status /path/to/taytay-live.toml
```

On success, Taytay reports completed jobs without failures. Check the USB
Camera Taytay ledger as well:

```sh
cargo run -- status /path/to/usb-camera.toml
```

The same artifact IDs should show `Completed` or `Retained`. Run ingestion
again to archive only completed source files:

```sh
cargo run -- run-filesystem /path/to/usb-camera.toml
```

## If an upload fails

Check that both processes use the same spool, Taytay version, local base URL,
organization, project, and currently valid enrolled credential. A local
platform may reject upload-session authorization when service signing
configuration or device enrollment is inconsistent. A failed artifact should
remain in the input directory; do not treat fixture publication to the spool
as proof of a successful upload.

The [delivery status](../stakeholder/delivery.md) records the latest verified
acceptance state. Do not include tokens, raw server responses containing
identifiers, or private diagnostic paths in public issue reports.
