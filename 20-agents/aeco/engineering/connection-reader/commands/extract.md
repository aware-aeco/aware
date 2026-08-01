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
      positions: [number]   # flat x,y,z world coordinates in MILLIMETRES, Y-UP (see below)
      indices:   [number]   # 0-based triangle vertex refs (triples)
  recipe:                   # OPTIONAL — present ONLY when the connection is recognized (see below)
    kind:   string          # the recognized type: "base-plate" or "shear-plate"
    params: object          # the fitted dimensions in mm (plate size + thickness, bolt grid, bolt Ø, edge distance)
    main:   string          # the member GlobalId the plate sits on — a column (base plate) or beam (shear plate); advisory
```

Each part is exactly the shape of a `kind:"mesh"` scene element — hand `positions`/`indices`
straight to `viewer-3d.render` or `ifc.write`.

## `parts` are in web-ifc's Y-UP frame — up is `+Y`, not `+Z`

The mesh comes back exactly as web-ifc tessellates it, and web-ifc bakes a fixed
`(x, y, z) → (x, z, −y)` rotation of IFC's Z-up world into its own **Y-up** renderer frame. So a base
plate's anchors run along `±Y`, and a consumer dropping these vertices into a Z-up scene must rotate
them (`(x, y, z) → (x, −z, y)`) or the connection arrives on its side.

Handing these parts to `viewer-3d.render`, declare it: `meta.up: "y"`. The scene schema keeps
coordinates in producer space and converts via `meta.up`, so the connection renders upright without
anyone rotating vertices.

This is stated rather than changed because it is the frame this command has always returned and
consumers are built on it. It is deliberately **not** the same as
`ifc-reference-reader.read-model`, which rotates back into the file's own Z-up frame so it can be
compared against `probe`'s bbox (aware-aeco/aware#343). Aligning the two is tracked separately; until
then, read each command's frame from its own contract.

The `recipe`, when present, is **frame-independent** — its params are scalars in millimetres that the
consumer re-derives on its own member — so a recipe import is unaffected by any of this. Only the mesh
fallback carries the frame.

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
