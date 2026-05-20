# `bcf-file.diff` — what changed between two BCFs

Stateless, read-only. Compares an older BCF against a newer one and reports topic-level change: topics added, removed, status changes, and which topics gained comments. This is the "what changed since last Friday" verb behind weekly coordination digests.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `old-path` | string | The earlier BCF (`.bcfzip` or folder). |
| `new-path` | string | The later BCF to compare against. |

## Outputs

```yaml
added:
  type: array
  items: string              # topic GUIDs present in new but not old
removed:
  type: array
  items: string              # topic GUIDs present in old but not new
status-changed:
  type: array
  items:
    topic-guid: string
    old-status: string
    new-status: string
comments-added:
  type: array
  items:
    topic-guid:    string
    comment-count: number    # how many NEW comments this topic gained
```

## Under the hood

Diff is keyed on the stable topic GUID:

1. **Set difference on GUIDs** → `added` (in new, not old) and `removed` (in old, not new).
2. **For GUIDs in both**, compare `<Topic TopicStatus>` → `status-changed` when they differ (raw strings, not normalised — you see exactly what the tools wrote).
3. **For GUIDs in both**, set-difference the comment GUIDs → `comments-added` counts the comment GUIDs present in new but not old.

Only these four change classes are reported. Edits to title / description / priority / assignee are **not** diffed — in practice those rarely move and would make the digest noisy. If you need full field-level diff, read both with [`read.topics`](./read.topics.md) and diff in a glue node.

`diff` does not care about BCF version — a 2.1 old vs 3.0 new diffs fine, because it compares GUIDs and status strings, not schema. (`removed` ≠ "resolved": a topic dropped from the file is just gone. Whether that means resolved or lost depends on your pipeline — see [bcf-round-trip-interop](../skills/bcf-round-trip-interop.md).)

## Composition example — Friday digest e-mail

```yaml
- id: changed
  agent: bcf-file
  command: diff
  config:
    old-path: "{{ inputs.last-friday }}"
    new-path: "{{ inputs.this-friday }}"

- id: said
  # for each topic that gained comments, pull the actual text
  agent: bcf-file
  command: read.comments
  config:
    path:       "{{ inputs.this-friday }}"
    topic-guid: "{{ changed.comments-added.*.topic-guid }}"

- id: digest
  agent: html-report
  command: render
  config:
    title: "Coordination digest — week ending {{ run.date }}"
    summary:
      new:      "{{ len(changed.added) }}"
      closed:   "{{ len(changed.removed) }}"
      moved:    "{{ len(changed.status-changed) }}"
      discussed: "{{ len(changed.comments-added) }}"
    detail: "{{ said.comments }}"
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | `old-path` or `new-path` missing | Check both paths |
| `bcf.not-a-bcf` | Either input lacks a `bcf.version` marker | Confirm both are BCFs |

## See also

- [`read.comments`](./read.comments.md) — get the text behind `comments-added`
- [`read.topics`](./read.topics.md) — for a fuller field-level diff in a glue node
- [bcf-round-trip-interop](../skills/bcf-round-trip-interop.md) — why `removed` is ambiguous
