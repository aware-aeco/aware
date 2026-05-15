---
name: aware-skill-builder-skill-creator-invocation
description: This skill should be used when invoking Anthropic's built-in `skill-creator` from inside the aware-skill-builder pipeline. Covers the four modes (create, modify, optimize-description, eval), what arguments each one accepts, what to pass for AWARE-specific context, and how to interpret skill-creator's output. Apply whenever the pipeline reaches Step 1 — that is, every skill operation.
---

# Invoking Anthropic's skill-creator

**skill-creator is the engine. aware-skill-builder is the orchestration on top. Get the invocation right or every downstream step is fighting bad output.**

## Why skill-creator is the engine, not a wrapper

skill-creator already encodes:

- The right frontmatter shape (name + description, plus optional metadata)
- Third-person description phrasing
- Imperative voice for the body
- Progressive disclosure (SKILL.md lean, references for depth)
- Description triggers that work with Claude's skill-selection model

Re-implementing any of that in AWARE would diverge and decay. Delegate.

## The four modes

| Mode | When | Inputs | Output |
|---|---|---|---|
| **create** | New skill from scratch OR porting a source file (porting = creation) | topic, target context, optional source markdown | full skill markdown |
| **modify** | Refining an existing skill | existing skill text, issue description | updated skill markdown |
| **optimize-description** | Skill triggers wrong (false positives/negatives) but body is fine | existing description, examples of correct/incorrect triggers | new description only |
| **eval** | Verifying triggering accuracy | skill description, test prompts (positive/negative) | accuracy + failing prompts |

## AWARE-specific context to pass

skill-creator does not know about AWARE conventions. Pass these explicitly in the invocation prompt:

```
Target use context: AWARE agent skills loaded into Claude Code / Codex /
OpenCode via the aware-aeco plugin. When the AI composes an AECO automation
app that involves <AGENT>, this skill should trigger so the AI knows
to <SPECIFIC GOTCHA>.

Constraints:
- Skill content is generic AECO knowledge, NOT FloLess-runtime-specific.
  No references to "FloLess injects token", "Smart Node", "inputs[]" —
  strip if found.
- The skill is part of an AWARE agent at agents/<vertical>/<name>/.
  The agent's manifest references it via the `skills:` list.
- This is a Tier <1|2> skill — <judgment/gotcha | API reference>.
- Skill audience is the AI composing AECO apps — the description
  should trigger on phrases like <SPECIFIC TRIGGER PHRASES>.

Current skill body length: <N> lines. Keep tight/surgical, not balloon.
```

## Example invocation (from the worked port)

When porting `trimble-connect-files` (FloLess source) into AWARE:

```
Skill location: D:\Repos\aware-aeco\20-agents\aeco\construction\
                trimble-connect\skills\files.md
Source: D:\Repos\floless-app\src\FloLess\Core\Skills\Resources\
        TrimbleConnect\trimble-connect-files.md

Target use context: AWARE compositions that upload/download/browse
Trimble Connect files. The agent is REST-only — no .NET runtime mandate.

Constraints:
- Drop the FloLess `group: TrimbleConnect` frontmatter line.
- Re-tune the description to trigger on phrases like "upload to TC",
  "download from Trimble", "list TC folders", "BCF topic file link".
- Keep the C# code examples but scope `.ConfigureAwait(false)` to a
  ".NET implementation note" — non-.NET runtimes follow their own conventions.
- Preserve all gotchas: apiBaseUrl already has /2.0, rootId not rootFolderId,
  DELETE uses versionId, 3-step upload, parentId in query not body.

Mode: create (porting = creation).
Length target: ~470 lines, mostly preserved structure.
```

## Interpreting skill-creator's output

skill-creator returns:

1. **Cleaned skill markdown** — apply this to the target file
2. **Frontmatter** — should have proper `name` + `description`
3. **A short rationale** — what was changed and why
4. **Optional eval results** — if eval mode was requested

If skill-creator's output:

- Adds `name:` and `description:` that match AWARE conventions → use as-is
- Suggests breaking the skill into multiple files → consider; usually means scope creep, but sometimes correct
- Inserts examples that reference Claude Code-specific tooling (Slash commands, etc.) → strip — those are out of scope for AWARE agent skills
- Adds `group:` or other FloLess-runtime fields → strip; skill-creator wouldn't do this, but verify

## Failure modes

| skill-creator output | Cause | Recovery |
|---|---|---|
| Refuses with "skill name conflicts" | Name collides with existing AWARE skill | Pick a more specific name (agent prefix + topic) |
| Returns a description that's too narrow | Insufficient context in the invocation | Re-invoke with more trigger phrases listed |
| Returns a description that's too broad (false positives) | Too many synonyms | Re-invoke in "optimize-description" mode with examples |
| Returns boilerplate / template | Topic too vague | Provide concrete code samples or vendor docs in source |

## Eval mode specifics

Eval mode requires a test set. Construct it like this:

```yaml
test-prompts:
  # Positives — should trigger the skill
  - prompt: "Upload my Tekla drawing to Trimble Connect"
    should-trigger: true
  - prompt: "Push the IFC export to TC fab folder"
    should-trigger: true
  - prompt: "List files in the TC project root folder"
    should-trigger: true

  # Negatives — should NOT trigger
  - prompt: "Send a Slack message about the new assembly"
    should-trigger: false
  - prompt: "Update Excel cell B12 with the part count"
    should-trigger: false
```

Accuracy below 0.90 → re-run create/modify with a sharper description.
