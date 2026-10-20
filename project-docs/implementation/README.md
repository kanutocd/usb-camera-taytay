# usb-camera-taytay implementation plan

This directory is the human-readable projection of the executable
`.sandbox-loom/implementation-plan.yml`.

- [`roadmap.md`](roadmap.md) — phase-level delivery sequence.
- [`status.md`](status.md) — current phase, slice, and runtime status.
- `phases/` — objectives, dependencies, and exit criteria for each phase.
- `slices/` — independently reviewable implementation units, acceptance
  criteria, quality gates, and commit boundaries.

The YAML plan remains authoritative for workflow execution. Update the
plan first, then regenerate this documentation. Do not treat this
directory as a place for unapproved ideas; use
[`../backlog/README.md`](../backlog/README.md) for those.

Runtime and coding-agent progress is shared through
`.sandbox-loom/implementation-status.yml`. The plan defines phases and
slices; the status ledger records their lifecycle, commit metadata, and
Sandbox Loom run/task history.

Regenerate `status.md` after a run with `sandbox-loom harness docs . --scope .sandbox-loom/intake/normalized-scope.yml --force` before
publishing this project documentation through Zensical.
