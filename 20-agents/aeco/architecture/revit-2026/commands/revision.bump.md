# `revit-2026.revision.bump`

Add the next revision letter to a set of sheets, lock the prior revision.

## When to use

At each formal issue (Issued for Tender / Construction / Record). Pairs with `sheet.export-pdfs` + `sheet.stamp` to produce the transmittal pack and lock-in the Revit-side record.

**WRITE-mode** to the Revit model. The calling app MUST declare a `safety:` block (per app-spec § Safety contract). Without it the CLI refuses to run.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `sheet-numbers` | array&lt;string&gt; | yes | — | Sheets to receive the new revision. |
| `description` | string | yes | — | Visible in the revision schedule. |
| `revision-date` | string | no | today | ISO date. |
| `issued-to` | string | no | — | Stored on the `Revision`. |
| `issued-by` | string | no | — | Stored on the `Revision`. |

## Output

```yaml
added-revision:
  revision-letter: "D"
  revision-date:   "2026-05-17"
affected-sheets:
  - "A-100"
  - "A-101"
```

## Worked example

```yaml
- id: bump-tender
  agent: revit-2026
  command: revision.bump
  inputs:
    sheet-numbers: ["A-100","A-101","A-200","A-201"]
    description: "Issued for Tender"
    issued-by: "MP"
    issued-to: "Acme GC"
  safety:
    transaction-group: revision-bump
    snapshot: true
    worksharing:
      check: true
      fail-if-other-user: true
    audit-stamp:
      uda-prefix: AWARE_
```

## Implementation note

Wraps `Document.AddRevision(date, description, issuedTo, issuedBy, numberType, sequence)` followed by `Revision.Issue()` on the prior letter to lock it. All steps inside a single `TransactionGroup` so a partial failure rolls everything back.

## See also

- `sheet.export-pdfs` — produce the PDF transmittal set
- `sheet.stamp` — stamp the PDFs with the new revision letter
- `sheet.list` — verify result by listing post-bump
