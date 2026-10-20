# Crosscutting Concepts

## Source-independent artifact boundary

MVP1 filesystem input and MVP2 V4L2 capture produce the same Taytay `Artifact` contract. Source-specific logic ends before durable publication. An artifact is emitted only when complete, stable, readable, and checksum-bound.

## Durability and recovery

Taytay's ledger is the source of truth for local job state, and the remote TUS offset is the source of truth for resumed transfer. Stable identity and repeated-execution deduplication prevent duplicate logical uploads. Retry behavior is bounded and classifies permanent authorization or protocol failures.

## Resource bounds

Bound observation and upload scheduling, memory, segment size, spool capacity, and capture-to-upload pressure. MVP2 must respond to spool pressure with backpressure rather than unbounded buffering. The project does not guarantee lossless capture under arbitrary hardware, disk, or network overload.

## Security and privacy

- Read credentials from protected local files with restrictive permissions.
- Do not put credentials, authorization headers, signed URLs, camera passwords, or captured media in logs, metadata, filenames, metrics, diagnostics, fixtures, acceptance output, or evidence.
- Keep the process unprivileged and grant no implicit device, network, filesystem, publication, or commit capabilities.
- Use a provisioned test project and scoped device credential for live acceptance; define rotation or revocation after acceptance.

## Ownership boundaries

USB Camera Taytay owns source observation/capture, local configuration, and orchestration. Taytay owns durable publication and transfer orchestration. Lunsaran owns identity and authorization. Brutus owns TUS transfer and provider execution. Sandbox Loom owns its separate governed work process; it is not part of application runtime or upload authorization.

## Validation and evidence

Synthetic fixtures and deterministic mock acceptance are the default. Live-platform and camera evidence are additional checks when the required environment and hardware are available. Retain redacted results only; do not commit raw media, credentials, or unverifiable hardware logs.
