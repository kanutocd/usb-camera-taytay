# Glossary

| Term | Meaning in this project |
| --- | --- |
| Artifact | A complete media item published through Taytay with stable identity, available capture metadata, size, and checksum. |
| Brutus | The service boundary responsible for TUS data transfer and provider execution. |
| Entregar | The supported upload client integration used with Taytay; it creates and transfers upload sessions through the supported path. |
| Lunsaran | The platform that owns identity, organization/project authorization, and upload-session policy. |
| MVP1 | Filesystem/file-segment ingestion of completed files from a manual drop directory, optional external producer, or synthetic fixtures. |
| MVP2 | Pure Rust capture from one configured V4L2 device, publishing through the same artifact and upload boundary as MVP1. |
| Spool | Taytay's durable local storage boundary for published artifacts and upload work. |
| TUS | Resumable upload protocol used by the Brutus data plane; the remote offset is authoritative for transfer resumption. |
| Taytay | The edge integration and runtime that owns the Artifact publication, durable spool and ledger, retries, resume, cancellation, bounded workers, and upload orchestration. |
| V4L2 | Linux Video4Linux2 interface used by MVP2 to inspect and capture from a configured camera device. |
| Sandbox Loom | Private project workflow for intake, policy, bounded execution, quality evidence, review, approvals, and commit boundaries. It is separate from the application's runtime. |
