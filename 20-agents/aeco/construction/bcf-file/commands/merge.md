# `bcf-file.merge` — combine many BCFs into one delivery package

Stateless, **write-mode**. Merges multiple BCF files into a single output BCF, de-duplicating topics by GUID. The federated-coordination verb: the architect's BCF + the MEP BCF + the structural BCF → one package the next tool consumes in a single import.

## Lifecycle

`single` — one call, one response

## Mode

`write` — produces a new file. Inputs are read-only; only `output-path` is created.

## Inputs

| Field | Type | Description |
|---|---|---|
| `input-paths` | array of string | Two or more source `.bcfzip` files / folders, **in precedence order** (later wins on conflict — see below). |
| `output-path` | string | Where to write the merged `.bcfzip`. Overwritten if it exists. |

## Outputs

```yaml
path:          string
topic-count:   number    # distinct topics in the output
deduped-count: number    # how many topics appeared in more than one source
```

## De-duplication rules

Topic GUIDs are stable across tools (a topic created in Solibri keeps its GUID through Navisworks, ACC, back to Solibri). So the same issue can arrive from two sources and must be reconciled, not duplicated:

- **Topic-level fields** (status, priority, assignee, title, description): **later source in `input-paths` wins.** Order your `input-paths` so the most-current source is last.
- **Comments**: **unioned by comment GUID** — every comment from every source is kept; duplicates (same GUID) collapse to one. This is almost always what you want: comments are append-only across the round-trip.
- **Viewpoints**: unioned by viewpoint GUID; snapshots travel with their viewpoint.
- **`project.bcfp`**: taken from the **last** source that has one.

`deduped-count` tells you how many topics were seen in more than one input — a high number is normal for a re-merge of an evolving round-trip; zero means the sources were disjoint.

## Version gotcha (read this)

**Do not merge mixed BCF versions blindly.** If `input-paths` mixes 2.1 and 3.0 files, the merge would have to drop 3.0-only fields (Stage, threaded replies, extension schemas) or fabricate them — both lossy. The agent **refuses** a mixed-version merge with `bcf.version-mismatch` rather than silently degrading. Normalise first with [`convert`](./convert.md), then merge same-version files. The output `bcf.version` equals the (now uniform) input version.

## Composition example — federated weekly package

```yaml
- id: align
  # convert any stragglers to 2.1 so the merge is uniform
  agent: bcf-file
  command: convert
  config: { input-path: "{{ inputs.mep-30 }}", output-path: "./tmp/mep-21.bcfzip", target-version: "2.1" }
  safety: { snapshot: true }

- id: merge
  agent: bcf-file
  command: merge
  config:
    input-paths:                       # precedence order: structural is most current → last
      - "{{ inputs.arch }}"
      - "./tmp/mep-21.bcfzip"
      - "{{ inputs.struct }}"
    output-path: "./deliver/week-21.bcfzip"
  safety: { snapshot: true }
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | An `input-paths` entry doesn't exist | Check each path |
| `bcf.version-mismatch` | Sources mix 2.1 and 3.0 | [`convert`](./convert.md) the odd ones out first, then merge |
| `bcf.single-input` | Only one path given | Merge needs ≥ 2; for a 1→1 pass use [`filter.by-status`](./filter.by-status.md) or [`convert`](./convert.md) |
| `bcf.output-not-writable` | `output-path` dir missing/read-only | Create dir / fix permissions |

## See also

- [`convert`](./convert.md) — align versions before merging
- [`diff`](./diff.md) — the inverse question: what changed between two BCFs
- [bcf-round-trip-interop](../skills/bcf-round-trip-interop.md) — why GUIDs are stable and what breaks the round-trip
