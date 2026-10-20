# Risks and Technical Debt

The risks below are carried from the approved scope. The source material does not assign probability or severity, so this page does not invent priority ratings. No separate technical-debt items are identified in the scope brief.

| Risk | Effect | Mitigation or evidence |
| --- | --- | --- |
| A file is observed before it is complete. | Partial or corrupt upload. | Require atomic rename, completion marker, or repeated stable observations; test incomplete-file suppression. |
| Capture rate exceeds upload or disk capacity. | Growing backlog and eventual data loss. | Bound spool and buffers, expose pressure, and apply MVP2 backpressure; test quota behavior. |
| The camera does not support a usable mode. | MVP2 cannot capture with the available hardware. | Inspect capabilities before selecting one verified format, resolution, and frame interval. |
| The camera disconnects during a segment. | Corrupt segment or stalled capture. | Safely close incomplete output and test bounded disconnect/reconnect behavior. |
| A credential or signed URL leaks. | Security incident. | Restrict credential files, redact output, and use secret-free fixtures and evidence. |
| Local and remote upload state diverge. | Duplicate or lost uploads. | Preserve stable artifact identity, checksum-bound resume state, Taytay ledger records, and the remote committed offset. |
| Hardware-only checks become required for ordinary changes. | Slow or unavailable CI and developer validation. | Keep MVP1 fixture and mock acceptance as the default layer. |
| Codec implementation expands MVP2. | Delayed capture MVP. | Prefer a suitable camera-provided encoded mode; evaluate encoding only after the required artifact format is confirmed. |

The scope does not guarantee lossless capture under arbitrary hardware, disk, or network overload. That limitation should remain visible in operator-facing material.
