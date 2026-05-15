# `aware-skill-builder.port` — port an existing skill from an external source

Stateless. Takes an existing skill markdown (FloLess production skill, vendor doc, contributor's rough draft, etc.) and produces an AWARE-compliant version in a target agent.

This is the highest-traffic command for the substrate's growth — every existing AECO skill someone has written elsewhere arrives via this command.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `source` | string | Path to the source markdown file (e.g. `D:\Repos\floless-app\src\FloLess\Core\Skills\Resources\Tekla\2025\tekla-events-clash-detection.md`). |
| `target-agent` | string | Which AWARE agent receives the skill. |
| `target-name` | string (optional) | Override the derived filename. Default = derived from source basename. |
| `source-kind` | enum | `floless`, `vendor-doc`, `contributor-notes`, `other`. Default `other`. |

## Outputs

```yaml
path:                  string
manifest-updated:      bool
decouplings-applied:   array         # which of the seven FloLess-ism categories fired
runtime-scoping-mode:  enum          # tier-a | tier-b | tier-c (see runtime-scoping.md)
eval-passed:           bool
eval-accuracy:         number
```

## CLI form

```bash
aware skill port \
  --source D:/Repos/floless-app/src/FloLess/Core/Skills/Resources/Tekla/2025/tekla-events-clash-detection.md \
  --target-agent tekla \
  --source-kind floless
```

## What runs internally

Same six-step pipeline as [`create`](./create.md), with these differences:

| Step | `create` | `port` |
|---|---|---|
| 1 — skill-creator | Mode: `create`, with topic + notes | Mode: `create`, with the source file as input |
| 2 — frontmatter | Generate from scratch | Derive from source's existing frontmatter; drop FloLess-only fields |
| 3 — decouple FloLess-isms | No-op | **Fully active.** All seven categories scanned. |
| 4 — runtime scoping | Based on `tier` input | Auto-detected: read agent manifest's `transport`, classify Tier A/B/C |
| 5 — folder + manifest | Compute target path | Same |
| 6 — eval | Build test set from common composition prompts | Same, plus a regression test against the source's original use cases |

## Source kinds

| `source-kind` | What it tells the pipeline | Step 3 behavior |
|---|---|---|
| `floless` | Source is from `src/FloLess/Core/Skills/Resources/`. Apply all seven decoupling categories. | Aggressive — every FloLess-ism rewritten |
| `vendor-doc` | Source is a vendor's published documentation. | Light — preserve original wording for legal/attribution clarity; only AWARE-specific layering |
| `contributor-notes` | Source is a rough contributor draft. | Substantial — skill-creator does most of the heavy lifting in Step 1 |
| `other` | Unknown origin. | Default — conservative; flags potential issues but doesn't auto-rewrite |

## Worked example

The `trimble-connect-files` port (the substrate's first worked example):

```bash
aware skill port \
  --source D:/Repos/floless-app/src/FloLess/Core/Skills/Resources/TrimbleConnect/trimble-connect-files.md \
  --target-agent trimble-connect \
  --source-kind floless
```

Decouplings applied (Step 3):

- ✓ Stripped `group: TrimbleConnect` frontmatter
- ✓ Rewrote *"Use when working with..."* → *"This skill should be used when..."* (third-person)
- ✓ Scoped `.ConfigureAwait(false)` mandate to *".NET implementation note"* (this agent is REST → Tier B → runtime-agnostic)
- ✓ Kept all critical conventions intact (apiBaseUrl, IDs globally unique, DELETE versionId, etc.)
- ✓ Preserved C# code examples but scoped under `**For .NET callers**` blocks

Output: `20-agents/aeco/construction/trimble-connect/skills/files.md`

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `skill.source-not-found` | Source path invalid | Check the path |
| `skill.source-not-markdown` | Source isn't `.md` | Convert or use `--source-kind vendor-doc` with the original format if HTML |
| `skill.decoupling-incomplete` | A FloLess-ism survived Step 3 (grep finds it post-port) | Re-run with `--strict-decoupling` or hand-fix and commit |
| `skill.eval-failed` | Regression test against source's original use cases fails | The port lost a trigger context; re-invoke with the missing context spelled out |

## When NOT to use `port`

- **Source has secrets or customer data**: don't port; that's a fresh-creation case (use `create`).
- **Source is itself an AWARE skill**: don't port; that's `modify` (skills can be refined in place).
- **Source is C++ headers or DLLs**: use [`aware-agent-builder`](../../aware-agent-builder/) instead — that path generates a whole agent including initial skills.

## See also

- [`pipeline.md`](../skills/pipeline.md) — the six-step pipeline overview
- [`decouple-floless-isms.md`](../skills/decouple-floless-isms.md) — Step 3 details
- [`runtime-scoping.md`](../skills/runtime-scoping.md) — Step 4 details
