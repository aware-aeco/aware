# `ifc-inspector.psets.export` — property sets to CSV

Stateless, **write-mode** (writes a CSV; reads the IFC). Exports the property sets of every entity of a given class as a flat CSV — one row per entity, one column per `Pset.Property`. This is the door-schedule / window-schedule / fire-rating extraction verb, and the backbone of a COBie deliverable.

## Lifecycle

`single` — one call, one response

## Mode

`write` — the input IFC is read-only; only `output-csv` is created.

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.ifc` / `.ifczip` path. |
| `class` | string | Exact IFC class to export, e.g. `"IfcDoor"`. |
| `pset-names` | array of string (optional) | Restrict to these property sets. Empty / omitted = **all** Psets found on the class. |
| `output-csv` | string | Where to write the CSV. Overwritten if it exists. |

## Outputs

```yaml
path:      string
row-count: number      # = entities of `class` that had at least one matching Pset
columns:   array       # the resolved column headers, e.g. ["GlobalId","Name","Pset_DoorCommon.FireRating", …]
```

## Under the hood — the ragged-columns problem

Different instances of the same class can carry **different** Psets (a fire door has `Pset_DoorCommon.FireRating`; a regular door may not). CSV needs a fixed column set, so the agent does **two passes**:

1. **Discover** — walk every entity of `class`, collect the union of all `Pset.Property` keys (filtered by `pset-names` if given). This becomes the header row, namespaced as `<PsetName>.<PropertyName>` to avoid collisions between identically-named properties in different Psets.
2. **Emit** — one row per entity; a cell is blank where that entity lacks that property.

Leading columns are always `GlobalId` and `Name` so the CSV joins back to [`entities.list-guids`](./entities.list-guids.md) and to BCF/clash data. A blank cell means "property absent on this entity" — to turn that into a compliance pass/fail, use [`psets.find-missing`](./psets.find-missing.md) instead, which is purpose-built for it.

## Composition example — COBie door schedule

```yaml
- id: doors
  agent: ifc-inspector
  command: psets.export
  config:
    path:       "{{ inputs.ifc }}"
    class:      "IfcDoor"
    pset-names: ["Pset_DoorCommon", "COBie_Component"]
    output-csv: "./cobie/doors.csv"
  safety: { snapshot: false }     # write is a fresh CSV, nothing to snapshot

- id: check
  inline:
    kind: assert
    description: COBie requires every door row to have a fire rating
    code: d => d["row-count"] > 0 && d.columns.includes("Pset_DoorCommon.FireRating")
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.unknown-class` | `class` invalid for the schema | Check spelling/schema |
| `ifc.empty-result` | No entities of `class`, or none carry the named Psets | Writes a header-only CSV, `row-count: 0`; verify with [`entities.count-by-class`](./entities.count-by-class.md) |
| `ifc.output-not-writable` | `output-csv` dir missing/read-only | Create dir / fix permissions |

## See also

- [ifc-psets-and-cobie](../skills/ifc-psets-and-cobie.md) — COBie Pset naming, instance vs type properties
- [`psets.find-missing`](./psets.find-missing.md) — pass/fail on required properties (vs raw export)
- [`entities.get-by-guid`](./entities.get-by-guid.md) — the single-entity version of this data
