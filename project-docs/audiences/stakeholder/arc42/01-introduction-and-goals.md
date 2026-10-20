# Introduction and Goals

## Purpose

USB Camera Taytay is a local Linux Rust CLI reference application. It demonstrates how camera media reaches Lunsaran through Taytay's durable edge upload path, and provides a concrete hardware-integrating project for the private Sandbox Loom workflow.

The architecture separates two sources of media complexity. **MVP1** observes completed files supplied through a manual drop directory, an optional external producer, or synthetic fixtures. **MVP2** replaces the external producer with a pure Rust V4L2 capture loop. Both use the same Taytay Artifact publication and upload boundary.

## Goals

| Goal | Acceptance evidence |
| --- | --- |
| Publish only complete, stable, readable media as durable artifacts. | Incomplete-file suppression, checksum verification, and artifact readability checks. |
| Preserve a logical upload across duplicate observations, restart, and transient failure. | Deterministic duplicate, recovery, retry, and resume acceptance through Taytay. |
| Keep capture and upload responsibilities at their intended ownership boundaries. | Both source paths publish through Taytay; no application-specific HTTP or TUS upload path. |
| Make ordinary MVP1 validation reproducible without camera hardware or live credentials. | Synthetic fixture and mock acceptance. |
| Protect credentials and media while keeping the local application operable. | Redacted diagnostics and evidence, protected credentials, status and recovery behavior. |
| Add direct camera capture in MVP2 without changing the durable upload boundary. | Hardware evidence for the selected V4L2 mode, bounded segments, and reuse of MVP1's Taytay path. |

These goals come from the approved scope and project brief. The brief defines acceptance conditions but no numerical latency, availability, or throughput targets.

## Stakeholders

- **Project owner:** scope, MVP acceptance, assumptions, and open questions.
- **Technical lead:** application architecture, source boundary, runtime, and release quality.
- **Security lead:** credentials, permissions, logs, media exposure, and least privilege.
- **QA lead:** deterministic fixtures and optional hardware acceptance.
- **Reference-app developer and Rust integrator:** run and study the source-to-Taytay composition.
- **Edge operator:** configure local input, spool, credentials, device, and status checks.
- **Taytay maintainer:** verify use of the public crate contracts.
- **Sandbox Loom maintainer:** verify the separate governed workflow and its evidence boundaries.
