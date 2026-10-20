# Quality Requirements

The project brief defines behavioral acceptance but no numerical latency, availability, or throughput targets. The scenarios below are testable with the stated evidence without inventing service-level objectives.

| ID | Quality concern | Stimulus and context | Required response | Evidence |
| --- | --- | --- | --- | --- |
| QS-01 | Integrity | A file is still being written in the MVP1 input directory. | Do not publish it until atomic completion, a completion marker, or the configured stable-file condition is met; publish a complete readable artifact with checksum. | Incomplete-file suppression and checksum acceptance. |
| QS-02 | Idempotency and recovery | The same source is observed repeatedly or the process restarts with a pending job. | Preserve stable artifact identity and one logical upload; recover durable job state through Taytay. | Duplicate-observation and restart tests. |
| QS-03 | Resumability | Network transfer is interrupted temporarily. | Keep the job inspectable and retryable, and resume through Taytay using durable local state and the remote TUS offset. | Mock retry/resume coverage; locally provisioned end-to-end upload when available. |
| QS-04 | Confidentiality | A failure includes sensitive configuration or media context. | Diagnostics and evidence exclude credentials, signed URLs, authorization headers, camera passwords, and media. | Redaction tests and review of retained evidence. |
| QS-05 | Testability | MVP1 runs in a developer or CI environment without a camera or production credentials. | Synthetic fixtures and deterministic mock acceptance exercise the source-to-upload flow. | Fixture run and mock acceptance. |
| QS-06 | Resource control | Capture or observation outpaces spool/upload capacity. | Bound memory, scheduling, segment size, and spool use; apply MVP2 backpressure at the spool boundary. | Quota and pressure tests; hardware run evidence for MVP2. |
| QS-07 | Hardware resilience | MVP2 receives malformed frames, shutdown, or device disconnect/reconnect. | Avoid corrupting completed artifacts and recover safely according to the capture behavior. | V4L2 tests and recorded hardware acceptance. |

Applicable Rust formatting, check, test, coverage, and clippy gates, project quality scripts, mock acceptance, and documentation checks are part of release evidence. Hardware acceptance supplements deterministic checks and is required only for the hardware-specific MVP2 evidence.
