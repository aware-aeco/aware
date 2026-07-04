# `connection-reader.extract` — tessellate one connection into mesh scene parts

Stateless, read-only. Tessellates a single connection (chosen from a `list` result) into AWARE
`mesh` scene primitives, ready to bake, render (`viewer-3d`), or write back to IFC (`ifc`).

## Lifecycle
`single` — one call, one response.

## Inputs
| Field | Type | Description |
|---|---|---|
| `ifc-path` | string | Path to the `.ifc` / `.ifczip` file. |
| `id` | string | The connection's `IfcElementAssembly` GlobalId, from a `list` result. |

## Outputs
```yaml
connection:
  id:      string           # the GlobalId requested
  name:    string           # human label ("BEAM B156")
  type:    string           # assembly ObjectType (may be null)
  members: [string]         # GlobalIds of the members (columns/beams) the connection sits on
  parts:
    type: array
    items:
      id:        string     # part GlobalId
      role:      string     # "plate" | "bolt" | "weld"
      positions: [number]   # flat x,y,z world coordinates in MILLIMETRES
      indices:   [number]   # 0-based triangle vertex refs (triples)
  recipe:                   # OPTIONAL — present ONLY when the connection is recognized (see below)
    kind:   string          # the recognized type: "base-plate" or "shear-plate"
    params: object          # the fitted dimensions in mm (plate size + thickness, bolt grid, bolt Ø, edge distance)
    main:   string          # the member GlobalId the plate sits on — a column (base plate) or beam (shear plate); advisory
```

Each part is exactly the shape of a `kind:"mesh"` scene element — hand `positions`/`indices`
straight to `viewer-3d.render` or `ifc.write`.

## Recognition (optional `recipe`)
When the tessellated parts match a **supported pattern**, `extract` also **fits a parametric
`recipe`** so a consumer can import the connection as an **editable recipe instance** instead of
opaque geometry, and re-derive it on its own member. Two patterns are recognized today:

- **base plate** — a horizontal plate with a grid of vertical anchor bolts passing through it; the
  recipe carries the plate width/depth/thickness, the anchor grid cols×rows, the bolt diameter and
  edge distance (all in millimetres), and `main` is the **column** it sits on.
- **shear / fin plate** — a vertical plate lapping a beam web with a single vertical line of
  horizontal bolts; the recipe carries the plate thickness/height/width, the bolt count and pitch,
  the bolt diameter and edge distance (all in millimetres), and `main` is the **beam** it hangs off.

The bolt orientation is what tells them apart — vertical anchors for a base plate, horizontal bolts
for a fin plate — so the two never collide. The mesh `parts` are **always** returned as the fallback;
`recipe` appears **only when recognition is confident**, so an unrecognized connection (a welded
gusset, a multi-column fin plate, anything unfamiliar) simply comes back as mesh with no `recipe`.
Recognition grows one supported type at a time.

## Under the hood
Drives the bundled **web-ifc** WASM engine: `OpenModel` → tessellate the assembly's plate/bolt/weld
elements (`StreamAllMeshesWithTypes`) → apply each placed geometry's world transform → scale metres
to millimetres. web-ifc resolves the real geometry (extrusions, faceted breps, boolean-clip copes,
mapped-item bolts); this agent never re-implements a geometry kernel. Deterministic per (file, id).

## Composition example
```yaml
- id: read
  agent: connection-reader
  command: extract
  config:
    ifc-path: "{{ inputs.ifc }}"
    id: "{{ inputs.connection_id }}"
```

## Failure modes
| Error | Cause | Recovery |
|---|---|---|
| `id … is required for extract` | no `id` | run `list` first, pass a candidate's `id` |
| `no IfcElementAssembly with GlobalId …` | wrong/typo id | use an `id` from `list` on the same file |
| (a part missing) | web-ifc could not tessellate a degenerate element | expected for zero-volume welds; other parts are unaffected |
