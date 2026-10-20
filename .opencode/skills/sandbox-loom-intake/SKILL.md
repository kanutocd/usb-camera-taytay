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
export SANDBOX_LOOM="/path/to/agent-platform/bin/sandbox-loom"
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
4. Present proposals, assumptions, and blocking questions for confirmation.
5. Validate with `bin/sandbox-loom intake validate`.
6. Import and normalize only after the brief is schema-valid.
7. Stop before harness generation when blocking questions remain unresolved.

Sandbox Loom remains authoritative for normalization, harness generation,
review, approval, policy, quality gates, evidence, and commits.


## Generated project context

- Project root: `/home/ken/Documents/mi-proyekto/usb-camera-taytay`
- Brief path: `/home/ken/Documents/mi-proyekto/usb-camera-taytay/specifications/project-brief.yml`
- Sandbox Loom executable: `/home/ken/Documents/mi-proyekto/agent-platform/bin/sandbox-loom`
- Skill source: `sandbox-loom-intake`
- Skill source SHA-256: `4043eaf1c812b537740d0330309ead43eed2ae1e12b1be215774dd9dd77fdd09`
- Target: `opencode`


## OpenCode packaging

Install this file under `.opencode/skills/sandbox-loom-intake/SKILL.md`.
