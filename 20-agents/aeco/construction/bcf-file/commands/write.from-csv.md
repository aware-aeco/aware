# `bcf-file.write.from-csv` — land a tracker/Excel issue list as a BCF

Stateless, **write-mode**. Generates a topic-only BCF from a CSV of issues. Use it when the source-of-truth lives in Excel / Smartsheet / a custom tracker and you need to push those issues into a BCF-consuming tool (ACC Issues, BIMcollab, Revizto). The output carries topics + comments but **no viewpoints** — a CSV can't encode camera geometry — and downstream tools accept topic-only BCFs.

## Lifecycle

`single` — one call, one response

## Mode

`write` — produces a new file. The input CSV is read-only; only `output-path` is created.

## Inputs

| Field | Type | Description |
|---|---|---|
| `input-csv` | string | Path to the source CSV (UTF-8; first row = headers). |
| `output-path` | string | Where to write the generated `.bcfzip`. Overwritten if it exists. |
| `column-mapping` | object | **Required.** Maps BCF fields to CSV column names. Keys are BCF fields; values are header names in `input-csv`. |

`column-mapping` keys (only `title` is required; the rest are optional and omitted from the topic when unmapped):

```yaml
column-mapping:
  title:       "Issue"          # required — the topic title
  description: "Details"
  status:      "Status"         # written verbatim to TopicStatus (free text)
  priority:    "Priority"       # written verbatim to Priority (free text)
  assigned-to: "Owner"
  topic-type:  "Type"
  labels:      "Tags"           # split on ';' into multiple <Label> elements
```

## Outputs

```yaml
path:        string
topic-count: number     # = number of CSV data rows that produced a topic
```

## Under the hood — and the GUID gotcha

For each CSV data row the agent creates a `<topic-guid>/markup.bcf` with a `<Topic>` whose fields come from the mapped columns, then zips the result with a `bcf.version` marker (2.1 by default).

**GUIDs are freshly minted.** A CSV row has no BCF GUID, so the agent generates a new UUID per row. This means:

- **`write.from-csv` is not idempotent across runs.** Run it twice on the same CSV → two BCFs whose topics have *different* GUIDs. If you re-export weekly and the consumer keys on GUID, it will see every topic as new each week. To keep GUIDs stable, add a `guid` column to your CSV and map it (the agent uses a provided `guid` verbatim when the `column-mapping` includes one) — or generate the BCF once and round-trip it thereafter rather than regenerating from CSV.
- Dates: rows have no creation date, so `CreationDate` is set to run time and `CreationAuthor` to `"aware-bcf-file"` unless you map columns for them.
- `status` / `priority` are written **verbatim** — this verb does not normalise. Make the CSV use the vocabulary your *consumer* expects ([bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md)).

## Composition example — Smartsheet export → ACC Issues

```yaml
- id: pull
  agent: file
  command: read
  config: { path: "{{ inputs.smartsheet-export }}" }

- id: bcf
  agent: bcf-file
  command: write.from-csv
  config:
    input-csv:   "{{ inputs.smartsheet-export }}"
    output-path: "./out/tracker.bcfzip"
    column-mapping:
      title:       "Issue"
      description: "Details"
      status:      "Status"
      priority:    "Priority"
      assigned-to: "Owner"
      labels:      "Discipline"
  safety: { snapshot: true }

- id: upload
  agent: trimble-connect
  command: upload
  config: { folder-id: "{{ inputs.issues-folder }}", path: "{{ bcf.path }}" }
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.csv-not-found` | `input-csv` doesn't exist | Check the path |
| `bcf.mapping-missing-title` | `column-mapping.title` not provided | Title is the one required field — add it |
| `bcf.column-not-found` | A mapped column name isn't a header in the CSV | Fix the header name in `column-mapping` (the agent reports which one) |
| `bcf.empty-csv` | CSV has a header row but no data rows | Returns `topic-count: 0` with a valid empty BCF |

## See also

- [bcf-status-and-priority-vocabularies](../skills/bcf-status-and-priority-vocabularies.md) — match your CSV's status/priority to the consumer
- [bcf-round-trip-interop](../skills/bcf-round-trip-interop.md) — why fresh GUIDs break weekly re-exports
- [`read.topics`](./read.topics.md) — verify the generated BCF reads back as expected
