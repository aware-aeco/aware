# `aware-skill-builder.modify` — refine an existing AWARE skill

Stateless. Improves a skill already in the repo. Used when a contributor reports the skill didn't trigger, gave bad output, or missed a known gotcha.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `agent` | string | Agent that owns the skill. |
| `skill-name` | string | Short filename (no extension). |
| `issue` | string | What's wrong? *"Doesn't trigger on 'upload IFC to TC'"*, *"description too vague"*, *"missing the parentId-in-query-not-body gotcha"*, etc. |
| `failing-prompts` | array (optional) | Prompts that should have triggered the skill but didn't. Feeds Step 6 eval directly. |

## Outputs

```yaml
path:        string
changes:     array         # diff summary: what was modified and why
eval-passed: bool
```

## CLI form

```bash
aware skill modify \
  --agent trimble-connect \
  --skill-name files \
  --issue "Doesn't trigger when user says 'push BIM model to TC'" \
  --failing-prompt "push BIM model to TC" \
  --failing-prompt "send IFC to Trimble Connect fab folder"
```

## What runs internally

Targeted re-invocation of the pipeline. Most steps short-circuit:

1. **Step 1** — `skill-creator` in `modify` mode, fed the existing skill + the issue
2. **Step 2** — frontmatter sanity check (re-apply conventions if drift detected)
3. **Step 3** — skip (already AWARE-native)
4. **Step 4** — skip (runtime scope is fixed by the parent agent)
5. **Step 5** — already in place; manifest already lists it
6. **Step 6** — **always run.** Re-eval with the new description + failing prompts

The most common modify case is description optimization: the body is fine but the description doesn't enumerate enough trigger phrases. In that case, only Step 1 (`optimize-description` mode) + Step 6 run.

## Common issues + their fix

| Issue | What modify does |
|---|---|
| "Doesn't trigger on X" | Adds X-flavor phrases to the description; re-runs eval with X in test set |
| "Triggers when it shouldn't" | Narrows description; adds negative tests to eval set |
| "Missing gotcha Y" | Inserts Y as a new section in the body; tags as a critical convention if applicable |
| "Code example doesn't match current SDK version" | Updates the example; bumps the agent manifest's version (patch) |

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `skill.not-found` | filename doesn't exist | Check spelling or use `aware agent list-skills <agent>` |
| `skill.eval-still-failing` | Modification didn't fix the issue | Inspect skill-creator's rationale output; consider creating a sibling skill instead |
| `skill.regression` | Modify broke a previously-passing eval prompt | Revert; use `port` from the prior git history or hand-fix narrowly |

## See also

- [`create.md`](./create.md) — when no existing skill to start from
- [`eval.md`](./eval.md) — to run only the eval pass without modifying
