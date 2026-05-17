# `acc-docs` vs `aps-data-management`

Both touch the same underlying storage; the difference is altitude.

| Concern | `acc-docs` (this agent) | `aps-data-management` |
|---|---|---|
| API surface | High-level ACC Docs REST (sheets, transmittals, permissions in ACC vocabulary) | Lower-level Autodesk Platform Services (BIM 360 / ACC Data Management API — folders, items, versions in Forge vocabulary) |
| Sheets concept | Yes — ACC's first-class sheet abstraction with revisions + issued-for | No — just files and versions |
| Transmittals | Yes — `transmittals.create` | No — would have to compose from `items` + `versions` |
| Permissions | Yes — `permissions.list` / `grant` | Limited — folder ACLs only |
| Folder ops | Yes | Yes |
| Versions | Yes — high-level: current version + history | Yes — low-level: every individual version object |
| Audience | BIM managers + architects | Forge developers + ops |

## Use the high-level agent when you can

For 95% of architect / BIM-mgr workflows, `acc-docs` is the right
agent — its verbs match the mental units you already work with.

Reach for `aps-data-management` only when:
- You need raw version-level introspection (e.g. "who created v3.2 of
  this file at 11:42pm")
- You're building infrastructure (a custom ACC sync tool, a data
  warehouse loader)
- Your IT department gave you a narrow ACC token that ACC Docs API
  doesn't accept but the underlying APS API does

## Auth scope

Both agents share the `acc` credential. `aware connect acc` provisions
a token with the `data:read data:write account:read viewables:read`
scopes, which covers both. For tenant-restricted hubs, use the
device-code flow (v0.13):

```bash
aware connect acc --device-code --tenant <acc-hub-id>
```

## Common cross-agent flow

```yaml
nodes:
  # High-level: list sheets needing re-issue
  - id: stale-sheets
    agent: acc-docs
    command: sheets.list
    inputs:
      project-id: '{{ secrets.acc-project-id }}'
      since: '{{ last-friday.iso }}'

  # Low-level: get full version history for one sheet
  - id: history
    agent: aps-data-management
    command: get-item-versions
    inputs:
      project-id: '{{ secrets.acc-project-id }}'
      item-id: '{{ stale-sheets.sheets[0].id }}'

  # Back to high-level: create transmittal
  - id: transmit
    agent: acc-docs
    command: transmittals.create
    inputs:
      project-id: '{{ secrets.acc-project-id }}'
      title: 'Sheets re-issued — {{ run.date }}'
      file-ids: '{{ stale-sheets.sheets[*].id }}'
      recipient-emails: ['client@acme.com', 'gc@acme.com']
    safety:
      transaction-group: re-issue
      snapshot: false
```
