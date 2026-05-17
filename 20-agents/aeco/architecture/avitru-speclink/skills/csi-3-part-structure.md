# CSI MasterFormat 3-part section structure

US specs follow CSI MasterFormat's 3-part body structure per section. AWARE apps that touch specs should respect this.

## Part 1 — General

- References (citing standards: ASTM, AISC, ACI, etc.)
- Submittals required (product data, samples, shop drawings)
- Quality assurance (qualifications, mock-ups)
- Warranty
- Coordination notes

Generally written first; sets the boundary conditions for Parts 2 + 3.

## Part 2 — Products

- Manufacturers (basis-of-design + approved equals)
- Materials (with specifications: ASTM grades, dimensions, finishes)
- Accessories (fasteners, sealants, primers)
- Source quality control (factory testing)

The "what" of the spec.

## Part 3 — Execution

- Examination (site verification before install)
- Preparation (surface prep, substrate conditioning)
- Installation (procedures, tolerances)
- Field quality control (on-site testing, inspections)
- Cleaning + protection
- Closeout (record drawings, O&M docs)

The "how" of the spec.

## Spec compliance audit

The auditing flow checks that all three parts are populated for every section in scope:

```yaml
- id: sections
  agent: avitru-speclink
  command: sections.list
  inputs:
    spec-id: '{{ secrets.acme-cd-spec-id }}'

- id: incomplete
  inline:
    kind: map
    description: Find sections with any part empty/partial
    code: |
      sections => sections.filter(s =>
        s['part-1-status'] !== 'complete' ||
        s['part-2-status'] !== 'complete' ||
        s['part-3-status'] !== 'complete')

- id: alert
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    team-id: 'Project Acme'
    channel-id: 'specs'
    title: 'Incomplete sections — {{ incomplete.length }} need work'
    rows: '{{ incomplete }}'
```

## MasterFormat 50-division structure

CSI MasterFormat groups sections into divisions (00-49 used; 50 reserved):

| Division | Title |
|---|---|
| 00 | Procurement and Contracting Requirements |
| 01 | General Requirements |
| 02 | Existing Conditions |
| 03 | Concrete |
| 04 | Masonry |
| 05 | Metals |
| 06 | Wood, Plastics, and Composites |
| 07 | Thermal and Moisture Protection |
| 08 | Openings |
| 09 | Finishes |
| 10 | Specialties |
| 11 | Equipment |
| 12 | Furnishings |
| 13 | Special Construction |
| 14 | Conveying Equipment |
| 21–23 | Fire suppression / Plumbing / HVAC |
| 25 | Integrated Automation |
| 26–28 | Electrical / Communications / Electronic Safety |
| 31–35 | Earthwork / Exterior / Utilities / Transportation / Waterway |
| 40–48 | Process / Material processing / industrial-specific |

Section codes are `DD EE FF` (Division, Even-numbered group, sequential).

Use `csi-masterformat.translate` (sibling agent in v0.18) to cross-walk MasterFormat ↔ Uniclass / OmniClass / CAWS / OCCS.
