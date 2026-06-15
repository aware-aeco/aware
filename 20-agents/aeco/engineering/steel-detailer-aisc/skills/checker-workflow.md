---
name: steel-detailer-aisc-checker-workflow
description: Use when the user wants to check a structural model or set of drawings against AISC detailing rules — explains the checker pattern, how to compose the lookup command with Tekla / Steel-from-Drawings workflows, and what a compliance report looks like.
---

# Checker workflow — checking a model against AISC rules

The checker is a **composed workflow** (`.flo`) that reads a structural model and checks it against the verified AISC rules database deterministically. It has no LLM in the run path.

## Workflow pattern

```
[model-source]
     │  StructuralModel JSON (grids, members, connections)
     ▼
[extract-connections]          (Tekla or Steel-from-Drawings read)
     │  list of connections with bolt diameters, spacings, edge distances, weld sizes
     ▼
[lookup-rule per check]        (steel-detailer-aisc lookup command, once per rule)
     │  {rule, value, units, citation, found}
     ▼
[compare]                      (model value vs rule value — model-free arithmetic)
     │  pass / fail / not-checked (found: false)
     ▼
[report]                       (HTML or JSON compliance summary)
```

## What the checker can verify deterministically

| Check | Rule ID | What it tests |
|---|---|---|
| Bolt spacing | `bolt.spacing.min` | center-to-center ≥ 2.67d |
| Clear distance | `bolt.spacing.clear` | clear gap ≥ d |
| Edge distance (1" bolt) | `bolt.edge.min.1.0in` | edge ≥ 1¼ in. |
| Max edge distance | `bolt.edge.max` | ≤ 12t, ≤ 6 in. |
| Hole size (1" bolt) | `bolt.hole.standard.1.0in` | hole = 1⅛ in. |
| Fillet min size | `weld.fillet.min.to.025in` … `weld.fillet.min.over075in` | min leg by plate thickness |
| Fillet max size | `weld.fillet.max.standard` | max leg ≤ t − 1/16 in. |
| Min weld length | `weld.fillet.min.length` | length ≥ 4 × leg |
| Steel grade (W-shapes) | `material.grade.w-shapes` | A992, Fy=50/Fu=65 |

## What the checker cannot verify (honest boundaries)

- **Capacity / strength** — bearing (§J3.11), slip, block shear require design loads. Lookup provides the equation coefficients; the check requires the applied force, which is not in a detailing model.
- **Approval-trail rules** — e.g. "EOR has approved oversized holes" cannot be determined from geometry alone.
- **Paywalled graphics** — weld symbols require a licensed copy of BS EN ISO 2553 / AWS D1.1 for interpretation.

When `found: false` for any rule, the checker reports "**rule not in verified database — check manually**" and does NOT interpolate a value.

## How to author the checker .flo

```yaml
# Example snippet for a bolt-spacing check
node lookup-bolt-spacing {
  agent: steel-detailer-aisc
  command: lookup
  input:
    rule: "bolt.spacing.min"
}

node check-spacings {
  agent: tekla          # or: script node using viewer-3d scene output
  command: exec
  input:
    scene: "{{ read-model.result }}"
    min_spacing_rule: "{{ lookup-bolt-spacing.result }}"
}
```

A `script` (model-free arithmetic) node compares `connection.spacing` against `rule.value`; output is `{pass, fail, not_checked}` per connection.

## Composing with Steel-from-Drawings

When the model comes from `floless-app-steel-from-drawings` (PDF → baked scene), the scene JSON already contains member sizes and grid data. A checker `.flo` can directly consume the baked scene:

```
[steel-from-drawings/read]  →  [steel-detailer-aisc/lookup bolt.spacing.min]
                            →  [compare spacing vs 2.67d]
                            →  [report violations]
```

This makes the end-to-end flow: PDF drawing → structural model → compliance report, with every rule traced to a cited source.

## Source

- Checker pattern: decalog #9 (no LLM in the run path) + AWARE app-spec §4 (node composition).
- Rule values: `20-agents/aeco/engineering/steel-detailer-aisc/rules/aisc-360-22.json` (66 verified rules, 2026-06-14).
