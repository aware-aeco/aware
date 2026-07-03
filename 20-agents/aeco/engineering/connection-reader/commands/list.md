# `connection-reader.list` — list the connection candidates in an IFC

Stateless, read-only. Enumerates the connections in an IFC file **without** tessellating any
geometry, so it returns quickly even on a large model. Backs the "which connection?" picker a UI
shows before an import.

## Lifecycle
`single` — one call, one response.

## Inputs
| Field | Type | Description |
|---|---|---|
| `ifc-path` | string | Path to the `.ifc` / `.ifczip` file. |

## Outputs
```yaml
connections:
  type: array
  items:
    id:      string   # IfcElementAssembly GlobalId — pass this to `extract`
    name:    string   # human label, e.g. "COLUMN C102" / "BEAM B156" (Name + Tag)
    type:    string   # the assembly ObjectType, when present (may be null)
    plates:  number   # IfcPlate count in the assembly
    bolts:   number   # IfcMechanicalFastener count
    welds:   number   # IfcFastener count
    members: number   # count of members (columns/beams) the connection sits on
```

## What counts as a connection
One candidate per **IfcElementAssembly that carries connection hardware** — i.e. it aggregates at
least one plate, bolt, or weld. Assemblies with no hardware (a bare member) are skipped. The
grouping is per-assembly for now; splitting one assembly into multiple joints is a later refinement.

## Under the hood
Reads `IfcRelAggregates` to map each assembly to its parts, classifies parts by IFC type, and counts
hardware. No geometry is tessellated (that is `extract`'s job), which is why it is fast.

## Failure modes
| Error | Cause | Recovery |
|---|---|---|
| `ifc-path is required` | no path given | pass `ifc-path` |
| `ENOENT …` | file not found | check the path |
| (empty `connections`) | no assemblies carry hardware | the model may not be a fabricated/detailed IFC |
