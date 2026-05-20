# `bcf-file.filter.by-priority` — keep only topics at or above a priority threshold

Stateless, **write-mode**. Reads a BCF and writes a new BCF containing only topics at or above a priority threshold. The "deal with the criticals first" routing verb. The input file is never modified.

## Lifecycle

`single` — one call, one response

## Mode

`write` — produces a new file. Calling node needs a `safety:` block (non-destructive: reads `input-path`, creates `output-path`).

## Inputs

| Field | Type | Description |
|---|---|---|
| `input-path` | string | Source `.bcfzip` or folder. Read-only. |
| `output-path` | string | Where to write the filtered `.bcfzip`. Overwritten if it exists. |
| `min-priority` | enum `low` \| `normal` \| `high` \| `critical` | Threshold. Keeps topics at this level **and above**. Default `high`. |

## Outputs

```yaml
path:          string
kept-count:    number
dropped-count: number
```

## The priority-mapping problem (read this)

`min-priority` is a clean 4-level enum. **BCF `Priority` is not** — it is a free-text field whose allowed values are defined per-project in `extensions.xml` (BCF 3.0) or left entirely open (2.1). Real files contain `"High"`, `"Major"`, `"1 - Critical"`, `"P1"`, `"Hoch"`, `"normal"`, `""`. The agent therefore **normalises** each topic's raw priority onto the 4-level scale before comparing:

| Normalised level | Raw strings matched (case-insensitive, substring) |
|---|---|
| `critical` | `critical`, `urgent`, `blocker`, `p1`, `1 -`, `highest` |
| `high` | `high`, `major`, `important`, `p2`, `2 -`, `hoch` |
| `normal` | `normal`, `medium`, `moderate`, `p3`, `3 -`, `mittel` |
| `low` | `low`, `minor`, `trivial`, `p4`, `4 -`, `niedrig` |

Order matters: the most-severe matching level wins (so `"high-critical"` resolves to `critical`). **Unrecognised or empty priorities are treated as `normal`** so they are never silently dropped at the `high`/`critical` threshold without being seen — the count of coerced topics is surfaced in `warnings` (see below). If your project uses a vocabulary this table doesn't cover, normalise upstream with a glue node and pass an already-clean BCF, or filter on labels instead. See [bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md).

## Outputs (extended)

```yaml
warnings:
  type: array
  items: string     # e.g. "3 topics had an unrecognised priority, coerced to 'normal'"
```

## Under the hood

Same folder-level copy as [`filter.by-status`](./filter.by-status.md) — matched topics are copied whole (markup + viewpoints + snapshots + comments), `project.bcfp` and `bcf.version` carried over. Only the keep/drop predicate differs.

## Composition example — escalate criticals to a separate package

```yaml
- id: criticals
  agent: bcf-file
  command: filter.by-priority
  config:
    input-path:   "{{ inputs.federated }}"
    output-path:  "./escalate/criticals.bcfzip"
    min-priority: critical
  safety: { snapshot: true }

- id: notify
  agent: microsoft-365
  command: send-mail
  config:
    to:      "{{ inputs.lead-coordinator }}"
    subject: "{{ criticals.kept-count }} critical clashes need sign-off today"
    attach:  "{{ criticals.path }}"
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | `input-path` doesn't exist | Check the path |
| `bcf.empty-result` | Nothing met the threshold | Valid empty BCF written, `kept-count: 0` — lower `min-priority` or check the `warnings` for mass coercion |
| `bcf.output-not-writable` | `output-path` dir missing/read-only | Create dir / fix permissions |

## See also

- [`filter.by-status`](./filter.by-status.md) — the status sibling; same copy semantics
- [bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md) — the full per-tool priority maps and why this verb has to normalise
- [`read.topics`](./read.topics.md) — see your file's real priority strings first
