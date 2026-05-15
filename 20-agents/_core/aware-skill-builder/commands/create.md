# `aware-skill-builder.create` — author a new skill in a target agent

Stateless. Creates a new skill file for an existing AWARE agent, starting from a topic + initial notes. Walks all six pipeline steps.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `agent` | string | Target agent id (e.g. `tekla`, `trimble-connect`, `slack`). |
| `skill-name` | string | Short filename without extension (e.g. `coordinate-systems`). The agent prefix is added automatically in the frontmatter `name`. |
| `topic` | string | One-paragraph subject. *"How to convert drawing-space coordinates to model world space in Tekla."* |
| `notes` | string | Initial knowledge. Rough paragraph, list of gotchas, vendor doc excerpt, code snippets — whatever's available. |
| `tier` | int (1 or 2) | Optional. 1 = judgment / gotcha; 2 = API reference. Default 1. |

## Outputs

```yaml
path:              string         # absolute path of the new skill file
manifest-updated:  bool           # whether the agent's manifest's skills: list was appended
eval-passed:       bool           # Step 6 result
eval-accuracy:     number         # 0.0-1.0
```

## CLI form

```bash
aware skill create \
  --agent tekla \
  --skill-name coordinate-systems \
  --topic "Convert drawing-space coordinates to model world space" \
  --notes-file ./notes-draft.md \
  --tier 1
```

## What runs internally

See [`pipeline.md`](../skills/pipeline.md). The command executes all six steps:

1. Invoke Anthropic `skill-creator` in **create** mode → see [`skill-creator-invocation.md`](../skills/skill-creator-invocation.md)
2. Apply AWARE frontmatter conventions → [`frontmatter-conventions.md`](../skills/frontmatter-conventions.md)
3. Decouple FloLess-isms → [`decouple-floless-isms.md`](../skills/decouple-floless-isms.md) (no-op for new skills with no FloLess inheritance)
4. Apply runtime scoping → [`runtime-scoping.md`](../skills/runtime-scoping.md)
5. Place in folder + update manifest → [`folder-placement-and-manifest.md`](../skills/folder-placement-and-manifest.md)
6. Run skill-creator eval (Step 6 is mandatory)

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `skill.agent-not-found` | Target agent doesn't exist | `aware agent list` to verify; create the agent first |
| `skill.name-collision` | A skill with this filename already exists in the agent's skills/ | Pick a different short-name or use `modify` |
| `skill.eval-failed` | Step 6 accuracy < 0.9 | Re-invoke with sharper notes; or use `modify` with the failing prompts as input |
| `skill.manifest-corrupted` | Agent's manifest.yaml has syntax errors | Fix manifest first |

## See also

- [`port.md`](./port.md) — same pipeline but starting from an existing source file
- [`modify.md`](./modify.md) — refine an already-committed skill
- [`eval.md`](./eval.md) — re-run Step 6 only on an existing skill
