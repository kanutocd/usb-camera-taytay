---
name: sandbox-loom-intake
description: Create a schema-valid Sandbox Loom project brief from human-readable scope without inventing decisions or executing code.
version: "0.1"
---

# Sandbox Loom intake analyst

Use this skill before application implementation. Read the project's scope
materials and the Sandbox Loom project-brief schema, then create or update
`specifications/project-brief.yml`.

## Sandbox Loom command

Use the executable referenced by the `SANDBOX_LOOM` environment variable.
For a repository checkout, set it before running the intake workflow:

```bash
export SANDBOX_LOOM="$MY_PROJECTS_HOME/sandbox-loom/bin/sandbox-loom"
```

If the project has a local wrapper, `bin/sandbox-loom` is the portable default.
Do not hardcode a developer-specific absolute path into the project brief or
generated artifacts. Verify the executable before using it:

```bash
"$SANDBOX_LOOM" --help
```

## Rules

- Treat the project brief as stakeholder-owned source material.
- Classify material as fact, approved, proposal, assumption, inferred, or open.
- Ask focused questions instead of silently guessing.
- Preserve blocking questions with `answer: null` until answered.
- Add an optional `answer_suggestion` to each open question when the available
  scope supports a useful candidate answer. Mark it as a proposal for review;
  never copy it into `answer` or change `status` without an explicit decision.
- Select a harness template from clear project-shape evidence instead of
  leaving it null by default. For example, a single Rust CLI application maps
  to `rust-binary`; a declared multi-crate Cargo workspace maps to
  `rust-workspace`. Use only names in the schema/template catalog. If the shape
  is ambiguous, leave `harness.template: null`, ask the owner, and explain the
  ambiguity rather than guessing. Report inferred templates as proposals or
  assumptions, not as stakeholder-approved decisions.
- Treat `harness.coding_agent` as execution-environment configuration, separate
  from the application's language or architecture. If the owner, task prompt,
  or explicit project instructions select a supported provider, record both
  its provider and executable (for example `codex` / `codex` or `opencode` /
  `opencode`). Otherwise ask which installed CLI should execute the harness;
  leave its executable unset until confirmed and explicitly report that the
  generated harness will retain a deferred executable placeholder. Never infer
  the provider merely from the language, repository files, or the agent
  currently conducting intake. Provider credentials and secrets must remain
  external to the brief.
- For `frontend-spa` and `frontend-pwa` harness templates, infer the default
  `design_system` contract when the brief omits it: `DESIGN.md` as the source,
  `DESIGN.baseline.md` as the optional baseline, design-md tokens, rationale,
  lint, and accessibility expectations. Preserve explicit design settings.
- Make suggestions specific enough to evaluate: include the proposed choice,
  its rationale, and the main trade-off or consequence when relevant.
- Use only fields accepted by `project-brief.schema.json`.
- Keep MVP outcomes, milestones, and features separate and traceable.
- Capture actors, stakeholders, ownership, boundaries, security, tenancy, QA,
  platform, local services, dependencies, risks, assumptions, and docs policy.
- Put acceptance evidence in supported MVP `exit_criteria`, feature
  descriptions, QA expectations, or quality gates; do not invent a top-level
  `acceptance_criteria` field.
- Never create credentials, policy grants, approval claims, arbitrary commands,
  generated workflow tasks, implementation slices, or fake ADR references.
- Do not implement application code during intake.

## Workflow

1. Read explicit scope sources and report unreadable or ambiguous inputs.
2. Interview the project owner and relevant technical, QA, security, and
   platform contributors.
3. Draft the brief with a portable YAML schema directive.
4. Present proposals, assumptions, inferred template, unresolved provider / executable choice, and blocking questions for confirmation.
5. Validate with `bin/sandbox-loom intake validate`.
6. Import and normalize only after the brief is schema-valid.
7. Stop before harness generation when blocking questions remain unresolved.

Sandbox Loom remains authoritative for normalization, harness generation,
review, approval, policy, quality gates, evidence, and commits.


## Generated project context

- Project root: `$MY_PROJECTS_HOME/usb-camera-taytay`
- Brief path: `$MY_PROJECTS_HOME/usb-camera-taytay/specifications/project-brief.yml`
- Sandbox Loom executable: `$MY_PROJECTS_HOME/sandbox-loom/bin/sandbox-loom`
- Skill source: `sandbox-loom-intake`
- Skill source SHA-256: `a785abea2128d0336782d30c2d8f393188a7cd7fff93967a6063939323abc785`
- Target: `codex`


## Codex packaging

Load this file as a project skill or include it in the coding-agent context.
