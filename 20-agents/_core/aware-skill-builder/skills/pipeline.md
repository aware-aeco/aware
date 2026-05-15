---
name: aware-skill-builder-pipeline
description: This skill should be used as the entry point when authoring, porting, or refining any AWARE agent skill. Describes the six-step pipeline that produces an AWARE-spec-compliant skill from a source (existing markdown, contributor notes, vendor docs). Apply whenever a contributor adds a new skill to an agent, ports an external skill into AWARE, or updates an existing skill's content. The pipeline composes Anthropic's `skill-creator` (the engine) with AWARE-specific post-processing (folder placement, manifest update, FloLess-decoupling, runtime-scoping).
---

# AWARE skill-builder pipeline

**Every AWARE skill goes through six steps. Skipping any step risks a skill that doesn't trigger correctly, doesn't fit the target agent's folder, or smuggles in FloLess-runtime assumptions.**

```
┌─ INPUT ──────────────────────────────────────────────────────────────────┐
│  • New skill: topic + initial notes from contributor                     │
│  • Port: existing markdown file (from FloLess, vendor docs, etc.)        │
│  • Modify: existing AWARE skill + issue description                      │
└─────────────────────────────┬────────────────────────────────────────────┘
                              ▼
┌─ STEP 1 · Invoke Anthropic skill-creator ────────────────────────────────┐
│  Mode: create | modify | optimize-description | eval                     │
│  Engine handles: frontmatter shape, imperative voice, progressive        │
│  disclosure, description trigger phrasing, example coverage.             │
│  See: skill-creator-invocation.md                                        │
└─────────────────────────────┬────────────────────────────────────────────┘
                              ▼
┌─ STEP 2 · Apply AWARE frontmatter conventions ───────────────────────────┐
│  • Frontmatter `name`: agent-prefixed (e.g. `tekla-drawing-identity`)    │
│  • `description`: third-person, enumerates trigger phrases               │
│  • Drop FloLess-only fields (`group:`)                                   │
│  See: frontmatter-conventions.md                                         │
└─────────────────────────────┬────────────────────────────────────────────┘
                              ▼
┌─ STEP 3 · Decouple FloLess-runtime assumptions ──────────────────────────┐
│  Seven categories of FloLess-isms to strip or rephrase                   │
│  (token injection mechanism, Settings UI references, "Smart Node"        │
│  naming, etc.). See: decouple-floless-isms.md                            │
└─────────────────────────────┬────────────────────────────────────────────┘
                              ▼
┌─ STEP 4 · Scope runtime guidance correctly ──────────────────────────────┐
│  C#-only when the target API mandates .NET (Tekla, Revit, AutoCAD).      │
│  REST-only agents are runtime-agnostic; mark C# examples as ".NET note." │
│  See: runtime-scoping.md                                                 │
└─────────────────────────────┬────────────────────────────────────────────┘
                              ▼
┌─ STEP 5 · Place in correct folder + update manifest ─────────────────────┐
│  Path: 20-agents/<vertical>/<agent>/skills/<short-name>.md               │
│  Append `<short-name>.md` to the agent's manifest.yaml `skills:` list.   │
│  See: folder-placement-and-manifest.md                                   │
└─────────────────────────────┬────────────────────────────────────────────┘
                              ▼
┌─ STEP 6 · Run skill-creator eval ────────────────────────────────────────┐
│  Test trigger accuracy with positive + negative prompts.                 │
│  Surface false-positives, false-negatives, suggested fixes.              │
│  Mandatory before commit.                                                │
└─────────────────────────────┬────────────────────────────────────────────┘
                              ▼
┌─ OUTPUT ─────────────────────────────────────────────────────────────────┐
│  • Skill markdown at correct path                                        │
│  • Agent manifest updated                                                │
│  • Eval report (accuracy %, false-positives, false-negatives)            │
│  • Ready-to-commit diff                                                  │
└──────────────────────────────────────────────────────────────────────────┘
```

## When to skip steps

- **Step 3** is no-op when porting from a non-FloLess source (vendor docs, contributor notes from scratch).
- **Step 4** is no-op when the target agent is .NET-mandated (Tekla, Revit, AutoCAD) — the C# examples stay as-is.
- **Step 6** is **never** skipped. Eval is the cheapest catch for description drift.

## When to invoke

- `aware skill create` → starts at Step 1 with "create" mode
- `aware skill port` → starts at Step 1 with "create" mode + Source param; Step 3 is fully active
- `aware skill modify` → starts at Step 1 with "modify" mode
- `aware skill eval` → jumps to Step 6 only

## Pre-flight check before invoking

- Is the target agent's `manifest.yaml` clean (no syntax errors)?
- Does the target agent's `skills/` folder exist?
- For ports: does the source file actually exist?
- For ports from FloLess: have I read [decouple-floless-isms.md](./decouple-floless-isms.md)?
