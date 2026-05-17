# `revit-2026.link.reload-all`

Reload every Revit link + linked CAD/PDF reference and report per-link status.

## When to use

Before a tender export, before a clash run, after a consultant pushes a new IFC into the CDE. Equivalent to the "Manage Links → Reload All" UI but logged and scriptable.

**WRITE-mode** against the host model (link locations + transformation matrices may update). Requires `safety:` block.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `include-cad-links` | boolean | no | `true` | Reload DWG / DGN / DXF links. |
| `include-pdf-links` | boolean | no | `false` | Reload PDF underlays. |

## Output

```yaml
reloaded:
  - link-name: "Acme-MEP.rvt"
    link-type: Revit
    status: Loaded
    last-modified: "2026-05-16T09:14:33Z"
  - link-name: "Acme-Struct.rvt"
    link-type: Revit
    status: NotFound
    last-modified: ""
```

## Worked example

```yaml
- id: refresh-links
  agent: revit-2026
  command: link.reload-all
  inputs:
    include-cad-links: true
  safety:
    transaction-group: link-reload
    snapshot: true
    worksharing:
      check: true
      fail-if-other-user: true
- id: notify-broken
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    channel-id: "{{ secrets.teams.coord }}"
    title: "Broken links detected"
    rows: "{{ refresh-links.reloaded | filter: status != 'Loaded' }}"
    when: "{{ refresh-links.reloaded | any: status != 'Loaded' }}"
```

## Implementation note

Iterates `Document.GetExternalFileReferences()`, calls `RevitLinkType.Reload()` for Revit links and the equivalent CAD/PDF reload paths. Each link is reloaded independently — a single bad link does not abort the rest of the run.

## See also

- `sheet.export-pdfs` — typically follows a successful link reload
- `schedule.find-rows-missing` — verify post-reload integrity
