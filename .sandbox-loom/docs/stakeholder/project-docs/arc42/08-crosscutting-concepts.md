# Crosscutting Concepts

```yaml
---
security:
  credential_storage: Read credentials from protected local files; require no group/other
    access.
  secret_redaction: Never commit or expose device credentials, Lunsaran tokens, signed
    URLs, authorization headers, camera passwords, or captured media in logs, artifact
    metadata, filenames, metrics, diagnostics, fixtures, acceptance output, or evidence.
  least_privilege: Do not grant implicit or unrestricted credentials, network, device,
    filesystem, publication, or commit capabilities; these remain external and explicitly
    configured/approved where needed.
  upload_auth: Lunsaran owns identity, organization/project authorization, and upload-session
    policy.
  evidence: Preserve redacted evidence without credentials, raw media, device-specific
    secrets, generated site output, or unverifiable hardware logs.
tenancy:
  model: Lunsaran organization and project authorization; acceptance uses a provisioned
    test project and scoped device credential when available.
  application_scope: No user management, billing, tenant administration, or replacement
    for Lunsaran authorization.
isolation:
  boundaries:
  - USB Camera Taytay owns local capture, buffering, configuration, and reference-app
    orchestration.
  - Taytay owns durable local publication, checksums, ledger, retries, resume, cancellation,
    bounded workers, and upload integration.
  - Lunsaran owns identity and project authorization.
  - Brutus owns TUS data transfer and provider execution.
  - Sandbox Loom owns governed execution, policy, approvals, quality evidence, and
    commit boundaries.
```
