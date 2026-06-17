# File read / write — moving bytes in and out of a composition

`file.read`, `file.write`, and `file.write-csv` are the stateless IO verbs: pull a
file's contents into a node, or land a node's output on disk. They are the generic
sink/source other agents compose with — e.g. an inspector `write`s a quarantine copy
of a malformed upload, or a converter `read`s an exported sheet before parsing it.

## Pick the verb

| Verb | Use when | Mode |
|---|---|---|
| `read` | You need a file's contents in the composition (text, or base64 for binary). | read |
| `write` | You have content (string or JSON) to persist to one path. | write |
| `write-csv` | You have tabular records and want a well-formed, column-ordered CSV. | write |

`read`/`write` move opaque content; `write-csv` understands rows and columns, so prefer
it over hand-assembling CSV text through `write` (it handles RFC-4180 quoting for you).

## Read

`read` returns `content` plus the byte `bytes` count. Use `encoding: text` (default) for
UTF-8 text and `encoding: base64` for binary files (images, PDFs) so the bytes survive
being carried through the JSON envelope intact.

## Write

`write` overwrites the destination and, by default, creates missing parent directories
(`create-dirs: true`) — so a node can write to `./out/sub/report.json` without a prior
mkdir step. A string is written as UTF-8; a structured value is serialized to JSON. The
input is named `bytes` (not `content`) for symmetry with `read`.

## write-csv

Give it `columns` (the exact header order) and `rows`. Rows may be **objects** keyed by
the column names, or **arrays** positional to `columns`. Keys absent from a row render as
empty cells, so heterogeneous rows are safe. Example — export an audit table:

```yaml
- id: csv
  agent: file
  command: write-csv
  config:
    path:    "audit-{{ inputs.topic }}.csv"
    columns: [date, author, comment]
    rows:    "{{ comments.rows }}"
```

## Safety

`write` / `write-csv` are `mode: write` — a composing app declares its own `safety:` block
on the node when the write matters, per the app-spec safety contract. `read` is `mode: read`
and needs none. Paths are taken per call; the agent declares `filesystem: read/write: any`,
and the runtime prompts for the concrete path scope at install per the agent permission model.
