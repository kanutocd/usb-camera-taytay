# Solution Strategy

The architecture reduces risk by proving the durable upload path with completed files before adding direct camera capture.

| Strategy | Reason |
| --- | --- |
| Start with MVP1 file ingestion and synthetic fixtures. | Exercise artifact publication and upload without requiring a camera or live platform. A manual drop directory is the ordinary path; an external producer is an optional demonstration path. |
| Use one artifact contract for filesystem and V4L2 sources. | Keep source-specific capture separate from shared publication and upload behavior. |
| Delegate durability and transfer to Taytay. | Reuse its spool, ledger, checksums, bounded retry, resume, cancellation, and uploader rather than duplicating them in the app. |
| Add MVP2 only after inspecting camera capabilities. | Select one supported mode and use a pure Rust capture loop with bounded segments and explicit buffer lifecycle. |
| Keep local/mock checks as the normal test layer. | Make MVP1 acceptance reproducible without hardware or production credentials; add live-platform and hardware evidence when provisioned. |
| Keep governance separate from runtime. | Sandbox Loom reviews and governs project work; it is not in the application's media path. |

## Delivery sequence

1. Implement the MVP1 filesystem observer and Taytay adapter.
2. Prove deterministic mock acceptance and document local operation; record the real-camera-to-file demonstration and locally running Lunsaran upload when the environment is provisioned.
3. Inspect the camera, record its supported mode, and then implement MVP2 capture behind the same artifact boundary.
4. Add hardware acceptance and troubleshooting guidance.
5. Release after applicable quality, documentation, and Sandbox Loom review requirements are met.

## Accepted scope answers

The brief records these answers: MVP1 covers both JPEG and MP4 fixtures; MVP1 uses a manual drop directory with an optional external-producer demonstration; successful source files stay in the configured archive until an operator removes them; MVP2 prefers a suitable camera-provided encoded mode and evaluates a Rust encoder only if needed; live acceptance uses a provisioned local Lunsaran/Brutus environment, dedicated test project, and scoped credential with rotation or revocation defined after acceptance. The particular MVP2 mode is selected only after inspecting the available camera.
