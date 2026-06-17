# `file.write-csv` — write rows to a CSV file

Stateless, `mode: write`. Emits a well-formed, column-ordered, RFC-4180-quoted CSV
from a list of records. The tabular counterpart of `write` — prefer it over assembling
CSV text by hand, since it handles header order, quoting, and missing cells for you.

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `path` | string (required) | — | Destination CSV path (absolute or relative to the run's working dir). |
| `columns` | array&lt;string&gt; | — | Column keys to write, in this exact order. Becomes the header row. |
| `rows` | array | — | Rows: an array of **objects** (keyed by column names) or **arrays** (positional to `columns`). Missing keys render as empty cells. |

## Outputs (single)

```yaml
path:      string    # the resolved absolute path written
row-count: integer    # data rows written (excluding the header)
```

## Composition example — export an audit table

```yaml
nodes:
  - id: comments
    agent: bcf-file
    command: read.comments
    config: { path: "{{ inputs.bcf }}" }

  - id: csv
    agent: file
    command: write-csv
    config:
      path:    "audit-{{ inputs.topic }}.csv"
      columns: [date, author, comment]
      rows:    "{{ comments.comments }}"

connections:
  - { from: comments, to: csv, label: Rows }
```

## Failure modes

- **A row references a key not in `columns`** → that value is dropped (only `columns`
  are written). **Parent missing** → created by default (shares `write`'s `create-dirs`).
  **Permission denied** → `error.permission-denied`.

## See also

- [read-write.md](../skills/read-write.md) · [write.md](./write.md)
