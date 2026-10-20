# Runtime View

```yaml
---
operations:
  runbook: Provide a concise operator/developer guide and local runbook.
  diagnostics: Expose safe operator diagnostics; failed or offline uploads remain
    inspectable and retryable without leaking secrets or media.
  recovery: Preserve durable jobs across restart; handle temporary network failure
    and repeated execution; hardware reconnect behavior is covered for MVP2.
  source_retention: Do not mutate or delete a source file before durable publication
    and successful upload completion; retention after success remains an open decision.
services:
  local_acceptance:
    lunsaran: Locally running platform used for provisioned end-to-end acceptance
      when available.
    brutus: Lunsaran/Brutus local acceptance may be used when provisioned.
  relationships:
  - USB Camera Taytay publishes artifacts to Taytay's durable spool and ledger.
  - Taytay/Entregar creates checksum-bound upload sessions through Lunsaran and transfers
    through Brutus/TUS.
```
