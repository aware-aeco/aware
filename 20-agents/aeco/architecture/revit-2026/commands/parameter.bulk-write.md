# `revit-2026.parameter.bulk-write`

Bulk-write a parameter across many elements, keyed by an identity column.

## When to use

When the source-of-truth for a parameter lives outside Revit (Excel sheet, schedule from another tool, spec document). Common case: update Fire Rating from this Excel sheet keyed by Mark.

**WRITE-mode** to the Revit model. The calling app MUST declare a `safety:` block per app-spec.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `key-parameter` | enum | no | `mark` | `mark` / `element-id` / `type-name`. |
| `target-parameter` | string | yes | — | Name of the parameter to write. |
| `rows` | array&lt;{key,value}&gt; | yes | — | One row per element. |

## Output

```yaml
written: 132
skipped-missing: ["W18","W22"]
skipped-read-only: ["W37"]
```

## Worked example

```yaml
- id: excel-rows
  agent: microsoft-365
  command: excel.range.read
  inputs:
    file-id: "{{ secrets.excel.fire-ratings }}"
    range: "FireRatings!A2:B500"
- id: fire-ratings-write
  agent: revit-2026
  command: parameter.bulk-write
  inputs:
    key-parameter: mark
    target-parameter: "Fire Rating"
    rows: "{{ excel-rows.rows }}"
  safety:
    transaction-group: bulk-fire-rating
    snapshot: true
    worksharing:
      check: true
      fail-if-other-user: true
    audit-stamp:
      uda-prefix: AWARE_
```

## Implementation note

One `Transaction` per 100-row batch (so a failure mid-write loses at most 100 elements of progress). Elements not found by the key are added to `skipped-missing`. Parameters that are read-only on the element type are added to `skipped-read-only` — they're not errors.

## See also

- `element.by-parameter-value` — find candidates first
- `revision.bump` — re-revise the affected sheets after a bulk change
