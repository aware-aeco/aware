# COBie and handover

Solibri's COBie export is the easiest path to a client handover
spreadsheet from an IFC model. The audit's BIM-manager named COBie
extraction as scenario #7.

## What COBie is

COBie (Construction-Operations Building information exchange) is a
spreadsheet schema with these tabs:

| Tab | Holds |
|---|---|
| Contact | People + organisations involved |
| Facility | Top-level project info |
| Floor | Each storey |
| Space | Rooms (= IfcSpace) |
| Zone | Groupings of spaces |
| Type | Element type catalogue (= IfcType) |
| Component | Individual instances |
| System | MEP systems |
| Assembly | Composite parts |
| Connection | Asset-to-asset links |
| Spare | Replacement parts inventory |
| Resource | Materials, labour |
| Job | Maintenance tasks |
| Document | Submittal references |
| Attribute | Custom properties |
| Coordinate | Geometry placement |
| Issue | Outstanding items |

A handover-grade IFC needs each tab populated. Solibri's COBie
ruleset catches gaps before export.

## The end-of-project flow

```yaml
nodes:
  - id: model
    agent: solibri
    command: model.open
    inputs:
      path: '\\office\projects\acme\handover\acme-final.ifc'

  - id: cobie-check
    agent: solibri
    command: ruleset.load
    inputs:
      name: 'COBie Compliance'

  - id: run
    agent: solibri
    command: check.run
    inputs:
      model-id: '{{ model.model-id }}'
      ruleset-id: '{{ cobie-check.ruleset-id }}'
    safety:
      transaction-group: cobie-handover
      snapshot: false

  - id: gate
    agent: aware-runtime
    command: assert
    inputs:
      expr: '{{ run.summary.critical == 0 }}'
      on-fail: 'COBie compliance failed — {{ run.summary.critical }} critical issues. Resolve before export.'

  - id: export
    agent: solibri
    command: cobie.export
    inputs:
      model-id: '{{ model.model-id }}'
      output-path: '{{ run.tmp-dir }}/acme-cobie-{{ run.date }}.xlsx'

  - id: upload
    agent: microsoft-365
    command: upload-file
    inputs:
      drive-id: '{{ secrets.handover-drive-id }}'
      folder-id: '{{ secrets.handover-folder-id }}'
      filename: 'acme-cobie-{{ run.date }}.xlsx'
      path: '{{ export.path }}'
    safety:
      transaction-group: cobie-handover
      snapshot: false
```

(The `assert` primitive ships in v0.19 — until then, route through a
`approve:` gate or a downstream node that fails on non-zero critical.)

## Solibri's COBie mapping

Solibri reads from IFC Psets:
- `Pset_BuildingCommon`, `Pset_SpaceCommon`, etc.
- Custom Psets named `COBie_*` (preferred for handover)
- Classification references (IfcClassificationReference)

If the model's authoring tool (Revit, ArchiCAD) doesn't write the
right Psets, the COBie export will be sparse. The `COBie Compliance`
ruleset names which Psets are missing — fix in the authoring tool,
re-export IFC, re-run.

## Alternatives

If you don't have Solibri:
- **CobieExtension for Revit** (Whitlock & Weinberger) — Revit-side
  authoring + export
- **CobieExtension for ArchiCAD** — same on the Graphisoft side
- **buildingSMART validation service** — schema-only, no Pset richness

This agent is the Solibri path; the others might be future agents
(scoped on demand).
