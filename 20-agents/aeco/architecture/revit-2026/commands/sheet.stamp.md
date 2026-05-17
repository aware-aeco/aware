# `revit-2026.sheet.stamp`

Overlay a date / project / revision / sheet stamp onto a set of PDFs.

## When to use

After `sheet.export-pdfs` produced naked PDFs, to apply the project's transmittal stamp. Does NOT touch the Revit model — pure PDF mutation.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `pdf-paths` | array&lt;string&gt; | yes | — | Files to stamp (typically passed from `sheet.export-pdfs.written[*].path`). |
| `stamp-template` | string | yes | — | Free-text with placeholders: `{date}`, `{project}`, `{revision}`, `{sheet}`. |
| `position` | enum | no | `bottom-right` | `top-right` / `top-left` / `bottom-right` / `bottom-left`. |

## Output

```yaml
stamped:
  - input:  "C:/exports/2026-05-17/AcmeProj-DD-A-100-RevC.pdf"
    output: "C:/exports/2026-05-17/AcmeProj-DD-A-100-RevC-stamped.pdf"
```

## Worked example

```yaml
- id: tender-stamps
  agent: revit-2026
  command: sheet.stamp
  inputs:
    pdf-paths: "{{ tender-pdfs.written[*].path }}"
    stamp-template: "AcmeProj — {date} — Rev {revision} — Issued for Tender"
    position: bottom-right
```

## Implementation note

PDF mutation is in-process (no external `pdfstamp.exe`). Input PDFs are not modified — a `-stamped` suffix is added to the filename. Output preserves PDF/A conformance if the input had it.

## See also

- `sheet.export-pdfs` — produces the input PDFs
- `revision.bump` — increment revision letter (model-side)
