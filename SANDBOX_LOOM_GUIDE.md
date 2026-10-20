# USB Camera Taytay: Sandbox Loom Guide

This guide describes the intended path from project scoping to governed autonomous development for "USB Camera Taytay".

It assumes the private Sandbox Loom repository is available locally at
`~/path/to/sandbox-loom`
`~/path/to/usb-camera-taytay`.

## 1. Prepare the project

```bash
cd ~/path/to/usb-camera-taytay
export SANDBOX_LOOM_REPO="$HOME/path/to/sandbox-loom"
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
`$sandbox-loom-intake` in the terminal. The path-based form is unambiguous:

```text
Read .codex/skills/sandbox-loom-intake/SKILL.md and follow it exactly.
```

For Codex, OpenCode, or another coding agent, use this prompt:

```text
Read and follow the project intake skill at
.codex/skills/sandbox-loom-intake/SKILL.md exactly.

Read project-requirements/project-scope-statement.md and convert it into
specifications/project-brief.yml.

Use the Sandbox Loom project-brief JSON Schema as the structural authority.
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
Do not implement application code or generate the runtime harness yet.

After writing specifications/project-brief.yml, report:
- the source facts captured;
- assumptions and proposals;
- unresolved open questions and their owners;
- any information that could not be represented in the schema.
```

If the generated skill is installed for a provider, use its provider-specific
path instead, such as `.opencode/skills/sandbox-loom-intake/SKILL.md` or
`.hermes/skills/sandbox-loom-intake/SKILL.md`.

The agent should create the brief, not merely summarize the scope statement.
The expected output is the actual file:

```text
specifications/project-brief.yml
```

Use the project-scope statement as the source for a stakeholder-readable brief. A coding agent can use the generated intake skill to preserve facts, distinguish proposals from decisions, and keep unresolved decisions explicit.

```bash
mkdir -p specifications

# Have the selected coding agent create this file from:
# project-requirements/project-scope-statement.md
${EDITOR:-vi} specifications/project-brief.yml
```

The brief should contain the approved MVP, milestones, features, stakeholders, ownership, constraints, quality expectations, local services, security/tenancy expectations, and `open_questions`. Do not silently convert an unresolved decision into a final architecture choice.

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

For guided authoring, Sandbox Loom can create a documented starting point. This
project is a Rust binary, not a frontend or PWA, so use the `rust-binary`
template:

```bash
"$SANDBOX_LOOM" intake init \
  --guided \
  --template rust-binary \
  --output specifications/project-brief.yml
```

## 5. Resolve and validate intake

Review blocking questions with the project owner and technical stakeholders. Answer them in the brief or with the intake answer command:

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

The audit must finish with `status: ready_for_normalization`, no blocking
questions, and no missing ownership before the brief enters the import step.

Then run the authoritative intake pipeline:

```bash
rm -rf .sandbox-loom/intake

"$SANDBOX_LOOM" intake validate specifications/project-brief.yml
"$SANDBOX_LOOM" intake import specifications/project-brief.yml \
  --output .sandbox-loom/intake
"$SANDBOX_LOOM" intake normalize .sandbox-loom/intake \
  --output .sandbox-loom/intake/normalized-scope.yml
```

Do not proceed to harness generation while blocking questions remain unresolved
or normalization reports a drift. Non-blocking questions may remain open only
when the brief records an owner, status, and answer suggestion and the project
owner explicitly accepts the deferral.

## 6. Generate and approve the harness

Generate the runtime manifest, workflow, implementation plan, harness documentation, and project-facing documentation from the normalized scope:

```bash
"$SANDBOX_LOOM" harness generate . \
  --scope .sandbox-loom/intake/normalized-scope.yml \
  --template rust-binary

"$SANDBOX_LOOM" harness docs . \
  --scope .sandbox-loom/intake/normalized-scope.yml

"$SANDBOX_LOOM" harness review .
"$SANDBOX_LOOM" harness approve . \
  --reviewer "Project owner" \
  --rationale "Approved USB Camera Taytay scope, tenancy model, local environment, quality gates, and bounded execution policy."
```

The `--template` option is optional when the normalized scope contains enough
signals for inference. For this project, `rust-binary` is the explicit fallback
because the reference application is a Rust CLI. Omitting the option is useful
when validating that intake inference selects the same project shape.

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

## 7. Run deterministic local development

Start with the provider-free path. It proves the harness, quality gates,
approvals, evidence, and commit handling without depending on external model
credentials or camera hardware.

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

The runtime must stop on a failed gate, an approval boundary, a missing
verified commit, an unavailable provider, or an unavailable hardware device.
A later invocation should resume from persisted implementation status rather
than requiring manual edits. Hardware access and live Lunsaran acceptance are
optional capabilities; they are never implied by this guide.

Preview and verify the generated documentation independently of a development
run:

```bash
"$SANDBOX_LOOM" docs preview . --audience consumer --check
"$SANDBOX_LOOM" docs preview . --audience technical --check
zensical build --strict

# Optional interactive preview after the checks pass:
"$SANDBOX_LOOM" docs preview . --audience consumer
```

The preview command builds from `project-docs/`; `--check` performs the build
without starting the preview server. The project-local Zensical installation
and its pinned dependencies are part of bootstrap, so use the repository's
documented toolchain rather than an unrelated global executable.

## 8. Optional provider integration

If Hermes, Codex, OpenCode, or another provider is used, it should consume the
generated intake skill plus the project-owned USB Camera Taytay skill. Keep
provider credentials outside the project brief, workflow, root filesystem, and
evidence.

Verify the selected provider independently before attempting provider-backed
Sandbox Loom execution. For Hermes, for example:

```bash
hermes chat -q "Reply with exactly: Hermes is ready" --format stream-json
```

Then run the Sandbox Loom provider acceptance only after the deterministic path works:

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

MVP1 is the primary provider-free and hardware-independent demonstration.
Validate stable-file detection, duplicate handling, Taytay publication,
checksum propagation, retry/resume, restart recovery, quota behavior,
pause/resume, cleanup, and redaction.

MVP2 is an opt-in hardware layer. Validate V4L2 capability inspection, one
verified capture format, buffer ownership, segment completion, shutdown,
disconnect/reconnect behavior, backpressure, and artifact checksums. Record
the Linux version, camera model, negotiated mode, and test result without
capturing credentials or customer media in evidence.

At minimum, the quality harness should cover:

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

The Rust CLI reference application and consumer documentation can be released
from the approved scope and committed harness artifacts. Zensical documentation
may be deployed to GitHub Pages. Each release should retain the brief revision,
implementation status, quality-gate results, and Sandbox Loom evidence
reference. Do not publish captured media, credentials, raw hardware logs, or
private harness material.

Live Lunsaran/Brutus and provider integrations remain optional acceptance
layers. The provider-free MVP1 workflow is the primary portfolio demonstration
because it is deterministic and can be run by another person without access to
your credentials or camera hardware.

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
