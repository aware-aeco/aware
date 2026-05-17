# Document metadata schema

Aconex enforces project-specific document metadata. Uploading without the right fields = rejection.

## Always query the schema first

```yaml
- id: schema
  agent: aconex
  command: documents.required-fields
  inputs:
    project-id: '{{ secrets.aconex-project-id }}'
```

Output tells you exactly which fields the project's document control policy requires.

## Typical UK project schema

Most UK BD-funded projects encode metadata like:

| Field | Example | Notes |
|---|---|---|
| `documentNumber` | `ACME-A-DR-001` | Project-prefix + discipline + type + sequential |
| `revisionDate` | `2026-05-17` | ISO 8601 |
| `revision` | `C` | Letter, usually A-P, then AA, AB |
| `title` | `Site Plan` | Sheet title |
| `discipline` | `A` | A=Architecture, S=Structural, M=Mech, E=Elec, etc. |
| `documentType` | `DR` | DR=Drawing, SP=Spec, RP=Report, CC=Calculation |
| `scale` | `1:100` | For drawings |
| `originator` | `ACME-ARCH` | Organisation code |
| `purposeOfIssue` | `IFC` | IFC=Issued for Construction, IFT=Tender, etc. |
| `phase` | `S4` | RIBA Stage 4 (Technical Design) |
| `client` | `Acme Corp` | Client organisation |

## Bulk upload

For a tender PDF set, batch through `documents.upload` per file with shared metadata except `documentNumber` / `title`. The transport doesn't currently have a `documents.bulk-upload` verb — that's queued for v0.15.x once we have a real Aconex-side test bed.

## Worked example

```yaml
- id: schema
  agent: aconex
  command: documents.required-fields
  inputs:
    project-id: '{{ secrets.aconex-project-id }}'

- id: upload-tender
  agent: aconex
  command: documents.upload
  inputs:
    project-id: '{{ secrets.aconex-project-id }}'
    file-path: '{{ run.tmp-dir }}/A-100-RevC.pdf'
    metadata:
      documentNumber: 'ACME-A-DR-001'
      revision: 'C'
      revisionDate: '{{ run.date }}'
      title: 'Site Plan'
      discipline: 'A'
      documentType: 'DR'
      scale: '1:100'
      purposeOfIssue: 'IFT'
      phase: 'S4'
  safety:
    transaction-group: tender-upload
    snapshot: false  # external CDE — Aconex's own audit trail logs this
```

## Common rejections

- **Duplicate documentNumber** — if a document with that number+revision already exists, the upload fails. Bump revision letter.
- **Discipline mismatch** — if `discipline: A` but the project doesn't have A enabled, upload fails. Check `documents.required-fields` for the enabled discipline list.
- **purposeOfIssue not in allowed values** — projects whitelist IFC/IFT/IFR/etc. Reject any other code.
