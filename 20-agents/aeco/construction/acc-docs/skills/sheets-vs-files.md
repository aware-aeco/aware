# Sheets vs Files — two ways the same PDF can live in ACC

ACC Docs surfaces PDF drawings two ways. Knowing which is which prevents the "I uploaded the file, why don't sheets see it" Friday-evening problem.

## Files

- **Storage primitive.** Anything you upload to a folder. PDF, DWG, IFC, Excel, anything.
- **Versioned.** Each new upload to the same `file-id` increments the version.
- **No domain concept.** A 200-page tender PDF is one file with one version.

Read with: `files.list`. Write with: `files.upload`, `files.upload-version`.

## Sheets

- **Sheet abstraction.** Each PDF *page* surfaces as a sheet with cross-PDF navigation, hyperlinks, revisions.
- **Sheet number aware.** Honors the project's sheet-number conventions (A-100, S-101, etc.).
- **Revision-letter scoped.** Each sheet has a current revision letter, revision history, and `issued-for` status.
- **Cross-PDF.** Sheets from multiple uploaded PDFs unify in the Sheets tool — a consultant browsing sheets sees A-100 from arch + S-100 from struct as siblings, not separate PDFs.

Read with: `sheets.list`. Write with: `sheets.upload-set`.

## Use sheets when:

- You're issuing a drawing pack for review / construction
- Consultants need cross-discipline browsing
- The project tracks revisions per sheet

## Use files when:

- The thing isn't a drawing (specs, reports, models, schedules)
- You don't need ACC's revision system (e.g. raw point-cloud data)
- You want one PDF to stay one PDF (e.g. a 500-page operations manual)

## How a sheet upload differs from a file upload

```yaml
# Wrong (the BIM-mgr Friday-evening problem):
- id: tender
  agent: acc-docs
  command: files.upload
  inputs:
    folder-id: 'tender-folder'
    file-path: '{{ run.tmp-dir }}/A-100-RevC.pdf'
# Result: file in folder, NOT in Sheets tab — consultants can't browse it cross-discipline

# Right:
- id: tender
  agent: acc-docs
  command: sheets.upload-set
  inputs:
    project-id: '{{ secrets.acc-project-id }}'
    pdf-paths:
      - '{{ run.tmp-dir }}/A-100-RevC.pdf'
      - '{{ run.tmp-dir }}/A-101-RevC.pdf'
    revision-letter: 'C'
    issued-for: 'Issued for Tender'
# Result: sheets surface in Sheets tab, available for cross-PDF nav + revision tracking
```

## Common pitfall: mixed sheet/file storage

Don't upload tender PDFs to a "Tender" *file* folder *and* also upload them as sheets. Sheets is the authoritative store; the file copy becomes stale on the next revision. Pick one.

If your firm's IT mandates "all PDFs in a specific folder", upload as **sheets** to that folder via the `sheets.upload-set` `folder-id` parameter (forthcoming — currently `sheets.upload-set` lands at the project's default sheet folder; explicit `folder-id` queued for v0.15.x). Until then, accept that sheets live in ACC's sheet store.
