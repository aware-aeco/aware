# `bcf-file.read.comments` — pull the full comment trail

Stateless, read-only. Returns every comment across every topic (or scoped to one topic with `topic-guid`). Comments are the audit trail of a coordination issue — who said what, when, and against which status. Use this verb when the deliverable is "show me the conversation," not "show me the issue."

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.bcfzip` file or unpacked BCF folder. |
| `topic-guid` | string (optional) | Scope to a single topic. Omit for **all** comments across **all** topics. |

## Outputs

```yaml
comments:
  type: array
  items:
    topic-guid:   string
    comment-guid: string
    author:       string
    date:         string      # ISO-8601, the comment's <Date>
    comment:      string      # the comment body
```

Comments are returned in document order within each topic (which BCF authors by creation time). When `topic-guid` is omitted, topics come out in archive-walk order — sort by `date` downstream if you need a global timeline.

## Under the hood

Comments are siblings of `<Topic>` inside `markup.bcf`:

```xml
<Comment Guid="def-456">
  <Date>2026-05-17T10:15:00Z</Date>
  <Author>mep-consultant@acme.com</Author>
  <Comment>Acknowledged. Pipe re-routed in v07.</Comment>
  <Viewpoint Guid="vp-789" />   <!-- optional: ties the comment to a viewpoint -->
</Comment>
```

BCF 2.1 comments are **flat** (no threading). BCF 3.0 adds an optional `<ReplyToComment>` reference and makes `ModifiedDate` required; this verb returns the flat list either way — if you need the 3.0 reply tree, read the raw `markup.bcf` (the agent does not synthesise threading on 2.1 files). See [bcf-21-vs-30](../skills/bcf-21-vs-30.md).

## Composition examples

### Full audit-trail CSV for one disputed topic

```yaml
- id: comments
  agent: bcf-file
  command: read.comments
  config:
    path: "{{ inputs.bcf }}"
    topic-guid: "{{ inputs.disputed-topic }}"

- id: csv
  agent: file
  command: write-csv
  config:
    path: "audit-{{ inputs.disputed-topic }}.csv"
    columns: [date, author, comment]
    rows: "{{ comments.comments }}"
```

### "What was said since Friday" — pair with `diff`

```yaml
- id: changed
  agent: bcf-file
  command: diff
  config: { old-path: "{{ inputs.last-week }}", new-path: "{{ inputs.this-week }}" }

# diff tells you WHICH topics gained comments; read.comments tells you WHAT was said
- id: new-comments
  agent: bcf-file
  command: read.comments
  config:
    path: "{{ inputs.this-week }}"
    topic-guid: "{{ changed.comments-added.*.topic-guid }}"   # fan-out per changed topic
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | `path` doesn't exist | Check the path |
| `bcf.topic-not-found` | `topic-guid` given but no such topic folder | Run [`read.topics`](./read.topics.md) first to get valid GUIDs |
| `bcf.corrupt-markup` | A `markup.bcf` is malformed | Failing topic is reported and skipped; others still return |

## See also

- [`read.topics`](./read.topics.md) — get the topic GUIDs to scope by
- [`diff`](./diff.md) — find which topics gained comments between two BCFs
- [bcf-21-vs-30](../skills/bcf-21-vs-30.md) — flat vs threaded comments
