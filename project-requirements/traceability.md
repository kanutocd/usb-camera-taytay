# Sandbox Loom intake traceability

This record connects the source brief to the normalized scope, requirements,
and selected implementation slice. Sandbox Loom's executable YAML artifacts
remain authoritative for execution; this page records their lineage and review
state without granting approval.

## Artifact chain

| Stage | Artifact | SHA-256 | Trace |
| --- | --- | --- | --- |
| Source brief snapshot | `.sandbox-loom/intake-20261021-021359/source-materials/project-brief.yml` | `a09fdfcf35f9555f60ec1a66de336c57039f6a5649d104211cc2f4e05f09f292` | Contains feature `LOOM-001`. |
| Intake provenance | `.sandbox-loom/intake-20261021-021359/provenance.json` | `c9f7bc488e03563c016d2ffb65f3e622ca4def12b4af7972c5d9c36b8efe7b0d` | Records the source brief snapshot and its original source digest. |
| Normalized scope | `.sandbox-loom/intake-20261021-021359/normalized-scope.yml` | `3a7ebf08641145d8f68ad94d25e11829cbf7b7855e19338d94760c65f1ce3232` | Carries `LOOM-001` into the approved project scope. |
| Requirements projection | `.sandbox-loom/requirements.yml` | `4cf7982e0308bb8be6ba21b7b58e5573fd31d4be4d436fd147faa003ce5ded64` | Maps `LOOM-001` to milestone 008. This digest is recorded in the project-owner approval. |
| Executable plan | `.sandbox-loom/implementation-plan.yml` | `9b73c5727c6d529fc6c22afaf399a37cdf55e2b7438f795f45a0b7686f635241` | Places `LOOM-001` in `phase-2-slice-25`, with the configured quality gates. The plan remains marked `proposed`. |

## Review state and drift

The project-owner approval records the current normalized-scope and requirements
digests above. It records `implementation-plan.yml` with digest
`152a50c56158406bf09eada04d94320517a57d9fb9a02264ace9f3e4e1e116c3`, which
does not match the current plan digest. The current plan and requirements files
also remain marked `proposed`. The approved harness and scope do not silently
approve those artifacts; only an authorized plan review can change their review
state or approval digest.

Approved ADRs remain the binding architecture decisions. Proposed ADRs do not
become binding through this traceability record.

## Slice mapping

`LOOM-001` maps from the normalized scope into requirement `LOOM-001`, then to
`phase-2/phase-2-slice-25` in the executable plan and its generated slice
projection. The slice's gates are defined by `.sandbox-loom/quality-gates.yml`;
verification evidence and commit approval remain Sandbox Loom runtime
responsibilities.
