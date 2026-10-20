# Building Block View

```yaml
---
- name: USB Camera Taytay CLI
  responsibility: Local filesystem observation or V4L2 capture, configuration, and
    application orchestration.
- name: Taytay spool and ledger
  responsibility: Durable local artifact publication, checksum state, and upload job
    state.
- name: Entregar uploader
  responsibility: Supported upload-session and artifact upload integration through
    Taytay.
- name: Lunsaran
  responsibility: Identity and project authorization.
- name: Brutus
  responsibility: TUS data plane and provider execution.
- name: Sandbox Loom
  responsibility: Separate policy-governed workflow, approvals, quality evidence,
    and commits.
```
