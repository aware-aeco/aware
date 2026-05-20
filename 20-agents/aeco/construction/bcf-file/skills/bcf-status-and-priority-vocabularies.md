---
name: bcf-status-and-priority-vocabularies
description: This skill should be used when filtering or routing BCF topics by status or priority, or when generating BCF from a tracker — because BCF status/type/priority are open free-text fields that every tool populates differently. Encodes the per-tool vocabularies, the normalisation map used by filter.by-priority, and the rule to match against real values, not assumed enums.
---

# BCF status & priority vocabularies

## The rule

**`TopicStatus`, `TopicType`, and `Priority` are open string fields. There is no closed enum in the BCF spec.** In 3.0 a project *may* declare its allowed values in `extensions.xml`; in 2.1 they're entirely free. Consequence: never hard-code an assumed status string. Always run [`read.topics`](../commands/read.topics.md) first to see the literal values *this* file uses, then build your filter from reality.

A filter on `["Open"]` will silently keep zero topics if the file uses `"Active"` — and the pipeline forwards an empty BCF without erroring. That is the classic BCF integration bug.

## What the tools actually emit

Status strings vary by tool and by language locale:

| Tool | Typical status values |
|---|---|
| Navisworks (clash) | `New`, `Active`, `Reviewed`, `Approved`, `Resolved` |
| Solibri | `Open`, `Closed` (+ a separate reviewer "accepted" state that does **not** export) |
| BIMcollab | `Open`, `In Progress`, `Solved`, `Closed`, `ReOpened` |
| Revizto | `Open`, `In Progress`, `Resolved`, `Closed` |
| ACC / BIM 360 Issues | `Open`, `Answered`, `Closed`, `In Dispute`, `Draft` |
| Trimble Connect | `Open`, `In Progress`, `Resolved`, `Closed` |

So "is this issue still live?" is not `status == "Open"`. It is closer to `status ∈ {Open, Active, New, In Progress, ReOpened, Answered}` — and you confirm the set against the actual file.

`TopicType` is just as varied: `Clash`, `Issue`, `Inquiry`, `Request`, `Remark`, `Fault`, `Comment`, plus tool- and project-specific types.

## The priority normalisation map (used by `filter.by-priority`)

Priority is the worst offender — `High`, `Major`, `1 - Critical`, `P1`, `Hoch`, `normal`, `""` all coexist. [`filter.by-priority`](../commands/filter.by-priority.md) takes a clean 4-level enum (`low`/`normal`/`high`/`critical`) and normalises each topic's raw priority before comparing:

| Normalised level | Raw substrings matched (case-insensitive) |
|---|---|
| `critical` | `critical`, `urgent`, `blocker`, `p1`, `1 -`, `highest` |
| `high` | `high`, `major`, `important`, `p2`, `2 -`, `hoch` |
| `normal` | `normal`, `medium`, `moderate`, `p3`, `3 -`, `mittel` |
| `low` | `low`, `minor`, `trivial`, `p4`, `4 -`, `niedrig` |

Most-severe match wins. **Unrecognised or empty priorities coerce to `normal`** and are counted in the verb's `warnings` — so they are never silently dropped below a `high`/`critical` threshold without the operator seeing the coercion count.

## Judgment

- **Match reality, not assumption.** [`read.topics`](../commands/read.topics.md) → inspect the distinct status/priority strings → build the filter.
- **Normalise upstream when the vocabulary is exotic.** If a file uses a status set this skill doesn't cover, rewrite the strings in a glue node and pass an already-clean BCF, rather than fighting the filter.
- **Prefer labels for project-specific axes.** If you need to route on "discipline" or "stage," that's a `Label`, not a status — filter on labels in a glue node.
- **When generating BCF** ([`write.from-csv`](../commands/write.from-csv.md)), make the CSV use the vocabulary the *consumer* expects. The agent writes status/priority verbatim — it does not translate on write.

## See also

- [`read.topics`](../commands/read.topics.md) — discover the real values
- [`filter.by-status`](../commands/filter.by-status.md) · [`filter.by-priority`](../commands/filter.by-priority.md) — the verbs this skill governs
- [bcf-21-vs-30](./bcf-21-vs-30.md) — `extensions.xml` declares allowed values in 3.0
