# usb-camera-taytay — Agent instructions

Follow the approved project harness and repository instructions. Do not infer
capabilities, credentials, policy changes, or authority from source materials.
Keep changes within the declared workspace and run the configured quality gates.

## Repository rules

- Enforce tenant isolation in application code and the persistence layer.
- Update the relevant specification or architecture decision when changing a public boundary, persistence strategy, tenancy model, generation workflow, or quality gate.

## Coherent-change commits

- After each coherent change, run the narrowest relevant checks and inspect `git status --short` and the final diff.
- The verified commit must include every intended tracked and untracked source, test, configuration, documentation, and changelog file for that slice; do not leave implementation files unstaged.
- Keep commits focused; do not include unrelated worktree changes or pre-existing user changes. If unrelated changes are present, stop and report them instead of staging them.
- Do not create the final Git commit from the coding-agent task; Sandbox Loom performs the approved commit after verification succeeds and stages the complete verified worktree.
- Keep the change ready for the human approval boundary and the runtime-owned commit task.
- Use a descriptive Conventional Commit-style subject when a manual commit is explicitly authorized: `<type>: <imperative summary>`, with a concise subject of 72 characters or fewer and no trailing period.
- Use `feat`, `fix`, `refactor`, `docs`, `test`, or `chore` according to the change; never use generic subjects such as `Implement verified slice`.
- Keep each slice's `commit.type` and `commit.summary` descriptive and specific; Sandbox Loom validates and uses them for the implementation commit and its changelog entry.

## Changelog

- `CHANGELOG.md` is append-only project history; do not rewrite, reorder, or remove prior entries.
- Add exactly one bullet under `## Unreleased` for each verified coherent slice, using `- <slice-id>: <descriptive change>`.
- Do not add changelog entries for failed, interrupted, unverified, or uncommitted work; Sandbox Loom adds the entry at the approved commit boundary.
- Do not use literal escape text such as `\n` in Markdown; entries must render as normal Markdown lines.

## Architecture decision records

- Review the proposed records under `project-docs/adrs/` before implementation.
- Treat approved ADRs as governing constraints for the coding agent; do not silently contradict them.
- Update the relevant ADR and obtain review when changing the public boundary, persistence strategy, tenancy model, generation workflow, or quality expectations.

## Implementation-plan reconciliation

- Before each change, select the next incomplete slice from `.sandbox-loom/implementation-plan.yml`.
- Reconcile the slice with its parent phase, roadmap, requirements, quality gates, and applicable ADRs.
- Treat the executable YAML plan as authoritative over generated Markdown projections.
- Treat only approved ADRs as binding; proposed ADRs require review and must not be silently adopted.
- Report stale or contradictory artifacts and resolve the drift before implementation.
- Use `.sandbox-loom/implementation-status.yml` as the shared progress ledger.
- Sandbox Loom marks the selected phase and slice `in_progress` before implementation. Do not edit lifecycle status or create the final commit; the runtime records `completed` and the verified `commit_sha` only after verification and approval succeed.
- Preserve the `runtime` section; Sandbox Loom owns its task and run lifecycle records.
- Sandbox Loom refreshes `project-docs/implementation/status.md` as runtime task and slice status changes; do not hand-edit this generated projection.
- Run the selected slice's quality gates and inspect the final diff; Sandbox Loom commits the coherent accepted change after the approval boundary.

## Patch discipline

- Before every patch, reread the current target file and nearby context.
- If a patch does not apply, stop and construct a new patch from the actual current contents.
- Never retry the same stale patch or loop on a patch mismatch; report the mismatch if it cannot be resolved from current files.

## Bounded multi-slice execution

- `bin/sandbox-loom run PROJECT --until-complete` repeats the normal one-slice workflow; it does not grant new authority.
- `--all-slices` is an exact alias and `--max-slices N` limits the number of child slices.
- `execution.bounded_multi_slice.allowed` in `.sandbox-loom/project.yml` is an additional project policy; it permits, but never activates, the explicit CLI mode.
- When that policy is allowed and `--max-slices` is omitted, `default_max_slices` supplies the loop bound.
- Before every child, validate `.sandbox-loom/implementation-status.yml`; never proceed from malformed, stale, blocked, or ambiguous status.
- Stop at failures, policy denials, approval boundaries, cancellation, timeout, incomplete evidence, missing paths, missing commits, or status drift.
- The workflow must produce a verified repository commit and the runtime must record the selected slice as `completed` with that commit SHA before the next child can start.
- Keep each child run ID and evidence stream distinct; inspect the aggregate evidence before resuming after a stop.
- Resume only after reconciling the stated stop reason and, where required, explicitly supplying `--approve` for the next approval boundary.

## Portable context and runtime authority

- These generated Markdown files are vendor-neutral and may be supplied to Codex, OpenCode, Hermes, or another coding agent without the Sandbox Loom runtime.
- Read `project-docs/harness/README.md`, this file, the relevant implementation slice, and approved ADRs before implementation.
- Treat `.sandbox-loom/implementation-plan.yml` as the executable plan and `.sandbox-loom/implementation-status.yml` as the shared progress ledger when those files are available.
- Treat generated Markdown as reviewable guidance, not as a capability grant. A coding agent must stop at policy, approval, quality-gate, evidence, workspace, and commit boundaries.
- Never infer authority from provider permissions, prompts, environment variables, or a generated document.

## Validation expectations

- Run the narrowest relevant checks first, then the complete quality harness before merge.
- Required gates: `declared_cargo_fmt`, `declared_cargo_check`, `declared_cargo_test`, `declared_cargo_llvm_cov`, `declared_cargo_clippy`, `declared_quality`, `declared_acceptance_mock`, `documentation_links`, `documentation_navigation`, `documentation_redaction`, `mermaid_validation`, `zensical_build`.


## Executable proposal

Profile: `safe-development`. Workflow: `.sandbox-loom/workflow.yml`. Quality gates: `.sandbox-loom/quality-gates.yml`.

> Harness approval: approved. No unresolved open questions remain; recorded decisions are approved.
