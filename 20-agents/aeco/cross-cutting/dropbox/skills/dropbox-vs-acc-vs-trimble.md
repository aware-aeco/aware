# Dropbox vs ACC Docs vs Trimble Connect

Three CDEs sit in the cross-cutting tier. AECO firms run one of them.

| Concern | Dropbox | ACC Docs | Trimble Connect |
|---|---|---|---|
| Sweet spot | Mid-sized + smaller firms, design studios, consultants | Autodesk-aligned shops, GCs using ACC Build | Trimble-aligned shops, structural fab, Tekla projects |
| File ops | First-class | First-class | First-class |
| Sheets concept (cross-PDF revisions) | No | Yes (`acc-docs.sheets.*`) | Limited |
| Shared links with expiry | Yes (Business tier) | Limited (per-folder ACL) | Limited |
| Paper / notebooks | Yes (`paper.*`) | No | No |
| Issues / coordination | No (separate tool needed) | Yes (`acc-issues.*`) | Yes (TC Tasks) |
| Transmittals | Manual (via shared links) | Yes (`acc-docs.transmittals.create`) | Manual |

## When to reach for which

| Workflow | Use |
|---|---|
| Designer's hero renders going to client | Dropbox or Drive — clients have Dropbox/Drive accounts more often than ACC seats |
| Tender drawing set with revision control | ACC Docs — sheets abstraction is the right shape |
| Steel fab pack to shop floor + erection partner | Trimble Connect — the steel-fab CDE |
| Mid-sized firm without an enterprise CDE | Dropbox Business — lowest friction |

## When you need all three

Larger projects routinely mix CDEs across consultants:
- Architect on Dropbox
- GC on ACC
- Steel fab on Trimble Connect

AWARE composes:

```yaml
- id: arch-pdf
  agent: revit-2026
  command: sheet.export-pdfs
  # ...

# Route to all three CDEs in parallel
- id: to-dropbox
  agent: dropbox
  command: files.upload
  inputs:
    local-path: '{{ arch-pdf.written[0].path }}'
    remote-path: '/Projects/Acme/Tender/{{ run.date }}.pdf'

- id: to-acc
  agent: acc-docs
  command: sheets.upload-set
  inputs:
    project-id: '{{ secrets.acc-project-id }}'
    pdf-paths:  '{{ arch-pdf.written[*].path }}'
    revision-letter: 'E'
    issued-for: 'Issued for Tender'

- id: to-tc
  agent: trimble-connect
  command: upload
  inputs:
    project-id: '{{ secrets.tc-project-id }}'
    folder-id:  '{{ secrets.tc-tender-folder-id }}'
    filename:   '{{ run.date }}.pdf'
    bytes:      '{{ arch-pdf.written[0].bytes }}'
```

The substrate stays out of the way; each agent does its CDE's idioms correctly.

## Auth

```bash
aware connect dropbox --oauth          # PKCE flow, default
aware connect dropbox --device-code    # for headless / IT-managed workstations
```

Dropbox uses the standard OAuth 2.0 flow. Business-tier scopes (team admin, password-protected links, expiry on shared links) require an enterprise app + admin consent — talk to your Dropbox admin if `aware app run` reports a 403 on those endpoints.
