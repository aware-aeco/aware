# `revit-2026.sheet.export-pdfs`

Batch-export `ViewSheet`s to PDF with a naming template, PDF/A mode, and embed-fonts control.

## When to use

End-of-stage deliverable assembly: tender set, IFC set, IFP set. Use after `sheet.list` (to enumerate) and before `sheet.stamp` (to apply project stamp).

**WRITE-mode** to the filesystem only — no Revit-model writes. Calling apps must declare a `safety:` block (per app-spec) even though the model is untouched, so output paths get logged.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `sheet-numbers` | array&lt;string&gt; | yes | — | The sheets to export (e.g. `["A-100", "A-101"]`). |
| `output-dir` | string | yes | — | Must exist and be writable. |
| `naming-template` | string | no | `"{sheet}.pdf"` | Placeholders: `{project}`, `{phase}`, `{sheet}`, `{rev}`, `{date}`. |
| `pdf-a` | boolean | no | `false` | Embed PDF/A-1b conformance markers. |
| `embed-fonts` | boolean | no | `true` | Embed all used fonts into the PDF. |
| `color` | enum | no | `color` | `color` / `black-and-white` / `grays`. |

## Output

```yaml
written:
  - sheet-number: "A-100"
    path: "C:/exports/2026-05-17/AcmeProj-DD-A-100-RevC.pdf"
    size-bytes: 1842334
```

## Worked example

```yaml
- id: tender-pdfs
  agent: revit-2026
  command: sheet.export-pdfs
  inputs:
    sheet-numbers: "{{ sheet-list.sheets[*].sheet-number }}"
    output-dir: "//office/projects/AcmeProj/tender/2026-05-17"
    naming-template: "AcmeProj-DD-{sheet}-Rev{rev}.pdf"
    pdf-a: true
    color: color
  safety:
    snapshot: false   # read-only against the model
    audit-stamp: false
```

## Implementation note

Wraps the modern Revit 2022+ `PDFExportOptions` API (no PrintManager required). Output is one PDF per sheet, never a combined set — bundle later via the `pdf` toolchain if you need a merged deliverable. Names that would collide get a numeric suffix.

## See also

- `sheet.stamp` — overlay date / revision / project stamp post-export
- `sheet.list` — enumerate sheets to export
- `link.reload-all` — make sure links are current before exporting
