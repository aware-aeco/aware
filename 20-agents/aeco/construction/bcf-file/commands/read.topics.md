# `bcf-file.read.topics` — list every topic with its metadata

Stateless, read-only. Parses a BCF archive (or unpacked folder) and returns one record per topic with all the markup-level metadata a coordinator reasons about: status, type, priority, assignee, dates, labels. This is the entry point for almost every BCF workflow — you read the topics, then filter / route / report on them.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | Path to a `.bcfzip` file **or** an unpacked BCF folder. Both layouts are accepted; the agent detects which by checking for a `bcf.version` marker at the root. |

## Outputs

```yaml
bcf-version:  string        # "2.1" or "3.0", read from the bcf.version marker
project-name: string        # from project.bcfp, or "" if absent (it's optional in the spec)
topics:
  type: array
  items:
    guid:             string   # the topic folder name; stable across tools
    title:            string
    topic-status:     string   # free text — see gotcha below
    topic-type:       string   # free text — "Clash", "Issue", "Inquiry", "Request", …
    priority:         string   # free text — see filter.by-priority for the mapping problem
    assigned-to:      string
    creation-date:    string   # ISO-8601
    creation-author:  string
    stage:            string   # topic lifecycle stage; present in both 2.1 and 3.0, "" if unset
    labels:           array
    description:      string
```

## Under the hood

Each topic is a folder named by its GUID; the metadata lives in `markup.bcf`:

```xml
<Markup>
  <Topic Guid="3a9c…" TopicType="Clash" TopicStatus="Open">
    <Title>Pipe clashes with structural beam</Title>
    <Priority>High</Priority>
    <CreationDate>2026-05-17T09:00:00Z</CreationDate>
    <CreationAuthor>P.Lisowski</CreationAuthor>
    <AssignedTo>mep-consultant@acme.com</AssignedTo>
    <Labels><Label>MEP</Label><Label>Level-3</Label></Labels>
    <Description>Detected by Navisworks clash test 'MEP-Struct-Hard'.</Description>
  </Topic>
  <!-- comments + viewpoint refs follow; read.comments / read.viewpoints surface those -->
</Markup>
```

The agent walks every `*/markup.bcf`, reads the `<Topic>` element only, and ignores comments and viewpoints (use [`read.comments`](./read.comments.md) and [`read.viewpoints`](./read.viewpoints.md) for those). `TopicStatus`, `TopicType`, and `Priority` are **open string fields** in the BCF spec — there is no closed enum. Whatever the emitting tool wrote, you get back verbatim. See [bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md) for the per-tool vocabularies you'll encounter in the wild.

## Composition examples

### Weekly digest — count open topics by assignee

```yaml
- id: read
  agent: bcf-file
  command: read.topics
  config: { path: "{{ inputs.bcf }}" }

- id: open-only
  inline:
    kind: filter
    description: Keep only topics still open
    code: t => ["Open", "ReOpened", "Active"].includes(t["topic-status"])

- id: by-assignee
  inline:
    kind: group-count
    description: Group the open topics by assignee for the digest table
    code: t => t["assigned-to"] || "(unassigned)"
```

### Feed an HTML report

```yaml
- id: read
  agent: bcf-file
  command: read.topics
  config: { path: "{{ inputs.bcf }}" }

- id: report
  agent: html-report
  command: render
  config:
    title: "Coordination issues — {{ read.project-name }}"
    rows: "{{ read.topics }}"
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | `path` doesn't exist | Check the path |
| `bcf.not-a-bcf` | Zip/folder has no `bcf.version` marker and no `*/markup.bcf` | Confirm it's a BCF, not a raw IFC or a clash-report PDF |
| `bcf.unsupported-version` | `bcf.version` is neither `2.1` nor `3.0` (e.g. legacy `2.0`) | Convert upstream, or open an issue — 2.0 support is post-v0 |
| `bcf.corrupt-markup` | A `markup.bcf` is malformed XML | The agent reports which topic GUID failed and skips it; the rest still return. Inspect that topic folder by hand. |

## See also

- [bcf-format-anatomy](../skills/bcf-format-anatomy.md) — the archive layout this verb reads
- [bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md) — why `topic-status` is free text
- [`read.comments`](./read.comments.md) · [`read.viewpoints`](./read.viewpoints.md) — the other two read verbs
- [`filter.by-status`](./filter.by-status.md) · [`diff`](./diff.md) — common next steps
