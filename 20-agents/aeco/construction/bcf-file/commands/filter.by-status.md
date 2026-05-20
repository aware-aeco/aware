# `bcf-file.filter.by-status` — emit a new BCF of only the topics you want

Stateless, **write-mode**. Reads a BCF and writes a *new* BCF containing only the topics whose status is in your list. The input file is never modified. This is the "split open from closed before routing" verb — most coordination pipelines forward only `Open` / `ReOpened` issues to the next tool and archive the rest.

## Lifecycle

`single` — one call, one response

## Mode

`write` — produces a new file. In an app, the calling node needs a `safety:` block per the [app-spec safety contract](../../../../../10-core/app-spec.md). Filtering is non-destructive (it only ever *reads* `input-path` and *creates* `output-path`), but it is still a write because it lands a new artifact.

## Inputs

| Field | Type | Description |
|---|---|---|
| `input-path` | string | Source `.bcfzip` or folder. Read-only. |
| `output-path` | string | Where to write the filtered `.bcfzip`. Overwritten if it exists. |
| `statuses` | array of string | Statuses to **keep**, matched case-insensitively. e.g. `["Open","ReOpened"]`. |

## Outputs

```yaml
path:          string    # the output-path that was written
kept-count:    number    # topics copied through
dropped-count: number    # topics excluded
```

## Under the hood

A topic is a self-contained folder, so filtering is a folder-level copy: the agent reads each `markup.bcf`, compares `<Topic TopicStatus>` against `statuses` (case-insensitive), and copies the *entire* topic folder — `markup.bcf`, all `.bcfv`, all `.png` — into the new archive when it matches. `project.bcfp` and `bcf.version` are carried over unchanged. Nothing inside a kept topic is rewritten, so viewpoints, snapshots, comments, and any vendor extension fields survive verbatim.

**Status is free text.** BCF does not define a closed status enum — `Open`, `Closed`, `ReOpened`, `Resolved`, `Active`, `Verified`, `In Progress`, `Approved` all appear in the wild depending on the emitting tool. Matching is case-insensitive but otherwise literal: `"Open"` will **not** match `"Active"`. Run [`read.topics`](./read.topics.md) first to see exactly what status strings your file uses, then build the `statuses` list from reality, not assumption. See [bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md).

## Composition example — route open issues onward, archive the rest

```yaml
- id: open
  agent: bcf-file
  command: filter.by-status
  config:
    input-path:  "{{ inputs.federated }}"
    output-path: "./route/open.bcfzip"
    statuses:    ["Open", "ReOpened", "Active"]
  safety: { snapshot: true }     # write-mode node

- id: forward
  agent: trimble-connect
  command: upload
  config:
    folder-id: "{{ inputs.coordination-folder }}"
    path:      "{{ open.path }}"
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | `input-path` doesn't exist | Check the path |
| `bcf.empty-result` | No topic matched — `output-path` would be a topic-less BCF | The agent still writes a valid empty BCF and returns `kept-count: 0`; check your `statuses` strings against [`read.topics`](./read.topics.md) output |
| `bcf.output-not-writable` | `output-path` dir missing or read-only | Create the dir / fix permissions |

## See also

- [`filter.by-priority`](./filter.by-priority.md) — the priority-threshold sibling
- [`read.topics`](./read.topics.md) — discover the real status strings first
- [bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md) — per-tool status vocabularies
