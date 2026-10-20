# Risks and Technical Debt

```yaml
---
- risk: Files are observed before they are complete.
  impact: Corrupt or partial uploads.
  mitigation: Require atomic rename, completion marker, or repeated stable observations.
- risk: Capture rate exceeds upload or disk capacity.
  impact: Backlog and eventual data loss.
  mitigation: Bound spool, expose pressure, and define retention/backpressure behavior.
- risk: USB camera format is unsupported.
  impact: MVP2 cannot capture.
  mitigation: Inspect capabilities first and support one verified format initially.
- risk: Camera disconnects during a segment.
  impact: Corrupt segment or stalled loop.
  mitigation: Close incomplete output safely and reconnect with bounded backoff.
- risk: Credential or URL leakage.
  impact: Security incident.
  mitigation: Strict file permissions, redaction tests, and secret-free fixtures.
- risk: Local and remote state diverge.
  impact: Duplicate or lost uploads.
  mitigation: Preserve artifact identity, checksum-bound resume state, and durable
    ledger records.
- risk: Hardware-only tests become required for every change.
  impact: Slow or unavailable CI.
  mitigation: Keep MVP1 fixtures and mock acceptance as the default test layer.
- risk: Codec implementation expands scope.
  impact: Delayed MVP2.
  mitigation: Start with one camera-supported format and defer transcoding.
```
