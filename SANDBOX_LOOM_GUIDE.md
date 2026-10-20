# USB Camera Taytay: Sandbox Loom Guide

This guide describes the intended path from project scoping to governed autonomous development for "USB Camera Taytay".

It assumes the private Sandbox Loom repository is available locally at
`$MY_PROJECTS_HOME/sandbox-loom`
`$MY_PROJECTS_HOME/usb-camera-taytay`.

## 1. Prepare the project

```bash
cd $MY_PROJECTS_HOME/usb-camera-taytay
export SANDBOX_LOOM_REPO="$HOME$MY_PROJECTS_HOME/sandbox-loom"
export SANDBOX_LOOM="$SANDBOX_LOOM_REPO/bin/sandbox-loom"
```

Keep the project-scope statement under `project-requirements/`. It is stakeholder-readable source material; it is not itself the runtime workflow.

## 2. Install the Sandbox Loom intake skill

Sandbox Loom can generate the same intake skill for different coding-agent environments. First inspect the installed catalog:

```bash
"$SANDBOX_LOOM" intake skills
"$SANDBOX_LOOM" intake skill show sandbox-loom-intake
"$SANDBOX_LOOM" intake skill roles
```

The current catalog provides `sandbox-loom-intake`. The planned `requirements-analyst` skill is described in Sandbox Loom’s planning notes, but is not yet a current catalog entry. Do not use `requirements-analyst` as the value of `--skill` until it appears in `intake skills`.

Generate only the targets that will actually be used:

```bash
# Provider-neutral copy for humans or standalone agents.
"$SANDBOX_LOOM" intake skill generate . \
  --skill sandbox-loom-intake \
  --target standalone \
  --output .sandbox-loom/skills/sandbox-loom-intake/SKILL.md

# OpenCode project skill.
"$SANDBOX_LOOM" intake skill generate . \
  --skill sandbox-loom-intake \
  --target opencode \
  --output .opencode/skills/sandbox-loom-intake/SKILL.md

# Codex project skill/context artifact.
"$SANDBOX_LOOM" intake skill generate . \
  --skill sandbox-loom-intake \
  --target codex \
  --output .codex/skills/sandbox-loom-intake/SKILL.md

# Hermes project skill.
"$SANDBOX_LOOM" intake skill generate . \
  --skill sandbox-loom-intake \
  --target hermes \
  --output .hermes/skills/sandbox-loom-intake/SKILL.md
```

The generated skill contains the project context and intake procedure. It does not contain credentials. Commit the generated skill when it is part of the project’s reproducible agent harness.

For a minimal showcase, generate `standalone` plus the provider you will demonstrate. Generating all targets is also valid when the project intentionally supports Codex, OpenCode, and Hermes.

## 3. Add the USB Camera Taytay project skill

`sandbox-loom-intake` teaches the agent how to create and validate the
project brief. It does not teach the agent the USB Camera Taytay domain. Keep
that project-specific context separate from Sandbox Loom policy and credentials.

When a provider-specific agent skill is needed, add a project-owned skill such
as:

```text
.codex/skills/usb-camera-taytay/SKILL.md
```

It should explain that this is a Rust CLI reference application with two
bounded MVPs:

- MVP1 watches stable filesystem/file-segment output and publishes it through
  Taytay's durable spool and upload boundary.
- MVP2 captures directly from one configured V4L2 device and emits artifacts
  through the same durable boundary.

The skill may describe commands, test fixtures, V4L2 inspection, and safety
boundaries. It must not contain device credentials, captured media, unrestricted
network instructions, or authority to approve its own changes.

## 4. Create the high-level intake brief

Convert the stakeholder-readable scope statement into the schema-backed brief
before running the Sandbox Loom intake commands. Start the coding agent from
the "USB Camera Taytay" project root and load the generated intake skill.

For Codex, first generate the project-local Codex artifact:

```bash
"$SANDBOX_LOOM" intake skill generate . \
  --skill sandbox-loom-intake \
  --target codex \
  --output .codex/skills/sandbox-loom-intake/SKILL.md
```

Then start Codex in the project root. `$sandbox-loom-intake` may be used in
the prompt only when that skill has been installed and discovered by the
Codex environment. It is not a shell command and must not be typed as
`$sandbox-loom-intake` in the terminal. The path-based form is unambiguous.
This project also has a project-owned Codex skill at
`.codex/skills/usb-camera-taytay/SKILL.md` (the project skill is under
`.codex/skills/`, not directly under `.codex/`):

```text
Read .codex/skills/sandbox-loom-intake/SKILL.md and follow it exactly.
Read .codex/skills/usb-camera-taytay/SKILL.md and use it as the project
domain, architecture, security, MVP, and validation context.
```

Use this Codex-specific prompt from the USB Camera Taytay project root:

```text
You are Codex working in the USB Camera Taytay project root.

First read and follow these two project artifacts:

1. .codex/skills/sandbox-loom-intake/SKILL.md — the Sandbox Loom intake
   procedure and project-brief schema rules.
2. .codex/skills/usb-camera-taytay/SKILL.md — the USB Camera Taytay Rust CLI
   domain, MVP1/MVP2 boundaries, Taytay integration, security rules, and
   validation expectations.

Treat the Sandbox Loom intake skill as the authority for the intake workflow
and schema. Treat the USB Camera Taytay skill as the authority for project
context, subject to repository instructions and Sandbox Loom policy. Do not
invent decisions, grant capabilities, implement code, or generate harness
artifacts during this intake task.

Read project-requirements/project-scope-statement.md and convert it into
specifications/project-brief.yml.

Use the Sandbox Loom project-brief JSON Schema in
`$MY_PROJECTS_HOME/sandbox-loom/specifications/intake/v0.1/project-brief.schema.json`
as the structural authority.
Preserve stakeholder language for objectives, actors, MVP1/MVP2 outcomes,
milestones, features, assumptions, exclusions, and open questions. Separate
confirmed decisions from proposals. Do not invent answers for unresolved
decisions; preserve them as explicit open_questions with an answer marker,
owner, and blocking status. When the source material supports a candidate,
add an `answer_suggestion` with the proposed choice, rationale, and relevant
trade-off. An `answer_suggestion` is only a proposal: keep `answer: null` and
`status: open` until an authorized stakeholder accepts it.

Include the Rust CLI, Linux filesystem/V4L2, Taytay crate, security, QA,
hardware-acceptance, documentation, and deployment constraints that are
explicitly present in the scope statement. Preserve the boundary between
MVP1's external file producer and MVP2's pure Rust V4L2 capture loop.
Preserve the private Sandbox Loom / sandbox-loom relationship without
copying credentials or private policy details into the brief. Do not
implement application code or generate the runtime harness yet.

After writing specifications/project-brief.yml, report:
- the source facts captured;
- assumptions and proposals;
- unresolved open questions and their owners;
- any information that could not be represented in the schema; and
- whether the brief preserves the Rust CLI shape, MVP1/MVP2 boundary, Taytay
  upload boundary, and Sandbox Loom authority boundary.
```

The prompt is intentionally Codex-specific. If another provider is used,
adapt only the provider invocation and skill-discovery wording; retain the two
skill paths, the schema rules, and the project-boundary instructions.

The coding agent must create `specifications/project-brief.yml` from the scope
statement; do not substitute a hand-written minimal draft or a summary. Use the
prompt above as the single source of instructions. Afterward, inspect the diff
and confirm the brief represents the source before moving to section 5.

An unresolved question may look like this:

```yaml
open_questions:
  - id: identity-provider
    question: Which identity provider should be used for the MVP?
    owner: project_owner
    blocking: true
    status: open
    answer: null
    answer_suggestion: Use tenant-scoped demo identities for the MVP because production identity is out of scope.
```

The suggestion helps stakeholders review a concrete option; it does not
resolve the question and must not be treated as an approval or an ADR.

If you need a fresh scaffold before agent authoring, create it at a temporary
path first so it cannot overwrite the brief:

```bash
"$SANDBOX_LOOM" intake init \
  --guided \
  --template rust-binary \
  --output /tmp/usb-camera-project-brief-starter.yml
```

Merge useful scaffold structure into the source-grounded brief; do not replace
the brief with the starter.

## 5. Resolve and validate intake

Review each question with its named owner. Apply an answer only after that
owner has decided; a suggestion is not an answer. Use the question ID shown
by `intake review` (the example below is illustrative):

```bash
"$SANDBOX_LOOM" intake review specifications/project-brief.yml --json

"$SANDBOX_LOOM" intake answer specifications/project-brief.yml \
  --question identity \
  --answer "Local demo identity with tenant-scoped fixture accounts; production identity remains out of scope." \
  --answered-by "Project owner"
```

### Audit the brief after applying answers

`intake answer` updates the selected question and rewrites the YAML file, but
it does not repair unrelated authoring drift or update document-level review
metadata. Before importing an answered brief, run a bounded coding-agent audit
from the project root:

```text
Audit specifications/project-brief.yml against:
- project-requirements/project-scope-statement.md
- the Sandbox Loom project-brief schema
- the current intake review rules

Fix only intake-document drift. Do not implement application code or generate
harness artifacts yet.

Preserve all answered questions and answer_suggestion fields. Repair malformed
inline YAML mappings, incomplete stakeholder roles, obvious answered_by typos,
and stale authoring.completion_status. Set completion_status to
ready_for_normalization only when all blocking questions are answered. Do not
invent decisions or change answer content except for obvious transcription
errors, and report every correction.

Then run:

$SANDBOX_LOOM intake validate specifications/project-brief.yml --json
$SANDBOX_LOOM intake review specifications/project-brief.yml --json

Stop and report the diff and both results. Do not run intake import, normalize,
harness generate, or harness docs.
```

Proceed only when `intake validate` passes and `intake review` reports
`ready_for_normalization`, no blocking questions, and no missing required
ownership. If a required role is genuinely unassigned, stop and ask the
project owner rather than fabricating a name. Non-blocking questions may
remain open only when their owner and deferral are explicit and accepted.

Then run the authoritative intake pipeline:

```bash
INTAKE_DIR=".sandbox-loom/intake-$(date +%Y%m%d-%H%M%S)"

"$SANDBOX_LOOM" intake validate specifications/project-brief.yml
"$SANDBOX_LOOM" intake import specifications/project-brief.yml \
  --output "$INTAKE_DIR"
"$SANDBOX_LOOM" intake normalize "$INTAKE_DIR" \
  --output "$INTAKE_DIR/normalized-scope.yml"
```

Review the normalized scope and provenance before using that exact
`normalized-scope.yml` in section 6. Keep earlier imports for comparison;
do not delete them as a routine regeneration step.

## 6. Generate and approve the harness

Generate the runtime manifest, workflow, implementation plan, harness documentation, and project-facing documentation from the normalized scope:

```bash
"$SANDBOX_LOOM" harness generate . \
  --scope "$INTAKE_DIR/normalized-scope.yml" \
  --template rust-binary

"$SANDBOX_LOOM" harness docs . \
  --scope "$INTAKE_DIR/normalized-scope.yml"

"$SANDBOX_LOOM" harness review .
```

The `--template` option is optional when the normalized scope contains enough
signals for inference. For this project, `rust-binary` is the explicit fallback
because the reference application is a Rust CLI. Omitting the option is useful
when validating that intake inference selects the same project shape.

Read the complete review output and inspect the generated diff before
approving. Resolve review errors, missing ownership, or scope drift first.
Proposed architecture decisions are separate from harness approval; approve
them individually with `harness approve-decision` only when the authorized
decision owner accepts them. Then record harness approval:

```bash
"$SANDBOX_LOOM" harness approve . \
  --reviewer "Project owner" \
  --rationale "Approved USB Camera Taytay scope, quality gates, and execution policy."
```

Do not regenerate approved artifacts casually. If regeneration is necessary,
review the diff and run `harness review` again before approval.

### Generate GitHub Actions workflows

After the harness and quality gates have been reviewed, compose CI and
documentation workflows from the project's current normalized scope and
`.sandbox-loom/quality-gates.yml`:

```bash
"$SANDBOX_LOOM" repo workflows generate .
```

Sandbox Loom infers GitHub from the selected local Git remote and reports that
as a proposal; it does not create or modify remote repository settings. If
there is no remote or its host cannot be identified, generation stops. Select
the provider explicitly when needed:

```bash
"$SANDBOX_LOOM" repo workflows generate . --provider github
"$SANDBOX_LOOM" repo workflows generate . --provider github --remote upstream
```

The command generates `.github/workflows/ci.yml` from the declared toolchain
and quality-gate `argv` commands, and `.github/workflows/docs.yml` for strict
Zensical validation. The docs workflow deploys to GitHub Pages only when
`github-pages` is an approved documentation publication target. Review both
files before committing them. Existing customized workflows are preserved;
generation stops on a differing file unless replacement is explicitly
requested with `--force`.

Review the generated files before approval, especially:

- `.sandbox-loom/project.yml`;
- `.sandbox-loom/workflow.yml`;
- `.sandbox-loom/implementation-plan.yml`;
- `.sandbox-loom/intake/normalized-scope.yml`;
- `project-docs/harness/AGENTS.md`;
- `project-docs/implementation/`;
- `README.md`.

For documentation-enabled projects, also review `zensical.toml`,
`project-docs/zensical/`, and the generated audience/redaction policy. The
generated Zensical `docs_dir` is `project-docs`; internal harness, ledger, and
stakeholder-only material must not be published to a consumer or technical
build.

## 7. Run local development and bounded slices

Start with the local-runtime path. It uses the configured coding-agent CLI,
so provider credentials or connectivity may still be required; local runtime
does not mean provider-free. This path does not require USB camera hardware.

```bash
"$SANDBOX_LOOM" project check .
"$SANDBOX_LOOM" run . \
  --runtime local \
  --approve \
  --json-summary
```

For bounded multi-slice execution, use the project policy and an explicit limit.
Run MVP1 before MVP2; do not let one autonomous run silently combine the
filesystem and V4L2 delivery slices:

```bash
"$SANDBOX_LOOM" run . \
  --until-complete \
  --approve \
  --max-slices 2 \
  --json-summary
```

The runtime reports the active slice and evidence location. If it is
interrupted or fails, inspect the final summary, checkpoints, and
`implementation-status.yml`; fix the underlying blocker and invoke the same
command again. The next invocation should resume from persisted status. Do not
manually reset a slice to pending or mark work complete. Hardware access and
live Lunsaran acceptance are optional capabilities; they are never implied by
this guide.

After a slice reports completion, verify that its code changes were committed
with a descriptive subject, its coherent change is recorded in `CHANGELOG.md`,
and the implementation status projection marks that slice completed. If any
of those disagree with the run summary or checkpoint evidence, stop and
investigate before starting the next slice.

Preview and verify the generated documentation independently of a development
run:

```bash
"$SANDBOX_LOOM" docs preview . --audience consumer --check
"$SANDBOX_LOOM" docs preview . --audience technical --check

# One-time setup, using the version pinned/declared by this project:
python3 -m venv .venv-zensical
source .venv-zensical/bin/activate
python -m pip install zensical

# Run for each build (activate the project environment first):
source .venv-zensical/bin/activate && zensical build --clean --strict

# Optional interactive preview after the checks pass:
"$SANDBOX_LOOM" docs preview . --audience consumer
```

`--check` performs a preview build without starting the server. Use the
project-local virtual environment, not a global Zensical installation. If the
project pins a Zensical version, install that exact version instead of the
unversioned example above. Do not recreate the virtual environment on every
build.

## 8. Optional provider integration

Provider integration is optional. The selected agent should follow the
generated intake skill during scoping and the project-owned USB Camera Taytay
skill during implementation. Keep provider credentials outside the project
brief, workflow, root filesystem, and evidence. Do not place secrets in saved
shell commands or committed files.

Verify the selected provider independently before attempting provider-backed
Sandbox Loom execution. For Hermes, for example:

```bash
hermes chat -q "Reply with exactly: Hermes is ready" --format stream-json
```

Run provider acceptance only after the local-runtime workflow is understood
and the provider/model are configured and authenticated. Replace the example
values with a provider and model supported by the installed Hermes setup:

```bash
SANDBOX_LOOM_RUN_LIVE_HERMES=1 \
SANDBOX_LOOM_HERMES_PROVIDER=your-provider \
SANDBOX_LOOM_HERMES_MODEL=your-model \
SANDBOX_LOOM_HERMES_EXECUTABLE=hermes \
(cd "$SANDBOX_LOOM_REPO" && \
  bundle exec ruby -I src examples/17_provider_acceptance/hermes_live.rb)
```

Provider acceptance must exercise the same disposable-project workflow:
inspect, implement, verify, approval, commit, and checkpoint. A successful
provider chat is not evidence that the full Sandbox Loom workflow succeeded.

## 9. MVP and quality-gate acceptance

MVP1 is the primary hardware-independent demonstration (the harness still uses
the configured coding-agent provider). Validate stable-file detection,
duplicate handling, Taytay publication,
checksum propagation, retry/resume, restart recovery, quota behavior,
pause/resume, cleanup, and redaction.

MVP2 is an opt-in hardware layer. Validate V4L2 capability inspection, one
verified capture format, buffer ownership, segment completion, shutdown,
disconnect/reconnect behavior, backpressure, and artifact checksums. Record
the Linux version, camera model, negotiated mode, and test result without
capturing credentials or customer media in evidence.

Use the commands and thresholds in the approved
`.sandbox-loom/quality-gates.yml` as the authoritative automated checks.
Confirm the run summary and evidence files show each required gate passing.
The acceptance coverage should include:

- Rust formatting, compilation, linting, and documentation warnings;
- unit and integration tests;
- deterministic filesystem fixtures and mock upload acceptance;
- Taytay API and published Entregar compatibility;
- checksum, retry, resume, cancellation, and redaction behavior;
- hardware acceptance for the selected V4L2 mode when MVP2 is enabled;
- coverage thresholds;
- evidence redaction and retention;
- approval and commit boundaries.

## 10. Publish the reference application and documentation

Publish only after the approved release checks pass and the publication
audience/redaction rules have been reviewed. Use the deployment target actually
configured for this project (do not assume GitHub Pages is enabled). Retain
the brief revision, implementation status, quality-gate results, and Sandbox
Loom evidence reference with the release. Never publish captured media,
credentials, raw hardware logs, or private harness material.

Live Lunsaran/Brutus and provider integrations remain optional acceptance
layers. The primary portfolio demonstration should use deterministic fixtures
and mock services where possible, and must not require access to private
credentials or camera hardware.

## Current readiness checklist

- [ ] `project-requirements/project-scope-statement.md` reviewed by the project owner.
- [ ] `specifications/project-brief.yml` generated and all blocking questions answered.
- [ ] `sandbox-loom-intake` skill generated for each intended agent.
- [ ] Intake validated, imported, and normalized.
- [ ] Harness generated, reviewed, and approved.
- [ ] Deterministic MVP1 local run passes.
- [ ] USB-camera file-segment demonstration passes.
- [ ] MVP2 V4L2 hardware acceptance passes, if enabled.
- [ ] Selected provider acceptance passes, if enabled.
- [ ] Documentation preview/build passes.
- [ ] Deployment configuration reviewed before publishing.
