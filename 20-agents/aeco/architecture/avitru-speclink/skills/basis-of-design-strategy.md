# Basis-of-design strategy

A US spec lists 1 "basis of design" manufacturer + N "approved equals" per section. The architect's call — Avitru's "approved equals" pool comes with each MasterFormat section out of the box; the architect picks + adds.

## When to set basis-of-design via AWARE vs the UI

| Use AWARE | Use the UI |
|---|---|
| Bulk re-baseline (e.g. "set Kohler as BoD across every plumbing section") | One-off per-section decisions |
| Round-trip from a manufacturer-selection spreadsheet | Browsing Avitru's catalog interactively |
| Compliance audit (verify every section has BoD set before issue) | Discussion with client over phone |

## Bulk setter

```yaml
- id: plumbing-sections
  agent: avitru-speclink
  command: sections.list
  inputs:
    spec-id: '{{ secrets.acme-cd-spec-id }}'

- id: plumbing-only
  inline:
    kind: map
    description: Filter to Division 22 (Plumbing) sections
    code: |
      sections => sections.filter(s =>
        s['masterformat-code'].startsWith('22'))

- id: set-kohler
  for-each: '{{ plumbing-only }}'           # v0.19 primitive
  do:
    - agent: avitru-speclink
      command: manufacturers.set-basis-of-design
      inputs:
        spec-id: '{{ secrets.acme-cd-spec-id }}'
        section-id: '{{ item.id }}'
        manufacturer-id: '{{ secrets.kohler-manufacturer-id }}'
      safety:
        transaction-group: bod-bulk-set
        snapshot: false  # spec history captures the change
```

## Compliance audit (pre-issue)

The "is every section ready for tender" check:

```yaml
- id: sections
  agent: avitru-speclink
  command: sections.list
  inputs:
    spec-id: '{{ secrets.acme-cd-spec-id }}'

- id: missing-bod
  for-each: '{{ sections.sections }}'
  do:
    - agent: avitru-speclink
      command: manufacturers.list-for-section
      inputs:
        section-id: '{{ item.id }}'
  filter: '{{ output.manufacturers.every(m => !m["included-by-default"]) }}'

- id: alert
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    team-id: 'Project Acme'
    channel-id: 'specs'
    title: 'Sections with no basis-of-design — {{ missing-bod.length }} need attention'
    rows: '{{ missing-bod }}'
```

## Risk: vendor lock

Avitru's catalog leans toward manufacturers who pay for placement. For non-Avitru products (e.g. small UK-side manufacturers in US projects, niche specialties), the BoD picker won't find them — write a "User Manufacturer" entry in the Avitru UI, then this agent can reference its ID. AWARE doesn't have a `manufacturers.create-user` verb yet; queued for v0.18.x.
