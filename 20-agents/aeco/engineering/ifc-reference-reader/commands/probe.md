# `ifc-reference-reader.probe` — cheap facts about an IFC, without tessellating

Stateless, read-only. Opens the file and answers schema / units / element count / rough extent
**without walking any geometry**, so it stays fast on a large model.

## Lifecycle
`single` — one call, one response.

## Inputs
| Field | Type | Description |
|---|---|---|
| `ifc-path` | string | Path to the `.ifc` / `.ifczip` file. |

## Outputs
```yaml
schema:   string   # "IFC2X3" | "IFC4" | "IFC4X3_ADD2" | …
units:
  declared: string # "MILLI.METRE" | "METRE" | … | null if the file did not say
elements: number   # elements placed DIRECTLY in the spatial structure — see the caveat below
frame:    string   # "z-up" — the frame `bbox` is in. Check this, not a version (see read-model.md)
bbox:              # APPROXIMATE, in millimetres, Z-up — or null when it cannot be established
  min: [x, y, z]
  max: [x, y, z]
storeys:           # descending by count — what a read-model `storeys` filter can select
  - { name: "L2", elements: 3696 }    # name is null for an unnamed storey
types:             # the same population by IFC entity, for the `ifc-types` filter
  - { name: "IFCMEMBER", elements: 9210 }
```

## The breakdown is how you choose what to read

`read-model` can read one storey, or a few IFC types, instead of a whole building. This is where you
find out what to ask it for — and it stays cheap, because it walks the same relationship tables a read
already walks before it streams, and tessellates nothing.

```
storeys: L2 ×3696, LG ×3351, L1 ×3256, L3 ×1405, L6 ×1300, L5 ×1124, L7 ×1122, BASEMENT ×882, …
types:   IFCMEMBER ×9210, IFCPLATE ×2665, IFCWALLSTANDARDCASE ×2341, IFCBUILDINGELEMENTPROXY ×1129, …
```

**These count the population a READ returns, not `elements`.** On the model above `elements` is 5,878
while a full read yields 17,460, because a read also returns everything transitively aggregated beneath
what was placed. A breakdown that summed to `elements` would under-predict every read by 3×, which is
the only thing anyone would use it for.

**They are an upper bound.** Whether an element carries a drawable triangle is only knowable by
tessellating it — precisely what this command exists not to do. A read reports the shortfall as
`skipped`.

**Scope: elements in the spatial structure.** An element outside it is not listed — on that same model
exactly one object, a site's own surface. It cannot be reached by `storeys` (it has no storey) but
`ifc-types` can still reach it, so the type rows are a lower bound for that one filter.

## `elements` is a spatial-containment count, not "how many objects you will get"

It counts what `IfcRelContainedInSpatialStructure` places in the building. That relationship is
schema-stable across IFC2X3 / IFC4 / IFC4X3 — unlike an element-type allowlist, which silently
undercounts whichever schema it was not written against — but it is **not** the same population
`read-model` returns:

- IFC forbids an assembly's parts from *also* being spatially contained, so the parts of an
  `IfcElementAssembly` are not counted here even though `read-model` returns each one.
- A product in no spatial structure at all is not counted.
- IFC2X3 permits any `IfcProduct` in `RelatedElements`, and IFC4X3 usage includes annotations and
  grids — so the count can include things that are not building elements.

Treat it as a **cheap order-of-magnitude signal** for "is this file big?", never as a number that
must equal `read-model`'s object count. The two are produced by different mechanisms on purpose:
this one must not tessellate, and that one must return everything drawable.

## Why it exists
So a consumer can decide whether to load a file **at all** before paying to tessellate it.
Tessellating a 300 MB model in order to discover it is too big is precisely the freeze a size cap is
meant to prevent — the check has to come first, and it has to be cheap.

## `units.declared` is provenance, not a factor
web-ifc reads `IfcUnitAssignment` itself and normalises geometry to metres **before any caller sees a
vertex**. Nothing downstream may scale mesh coordinates by this value.

It is reported so a user can see what they were handed, and so a file that *lies* about its units can
be spotted and overridden. `null` means the file did not declare a length unit — an honest unknown,
never a guess.

Measured across four real files (2026-07-25): three millimetre files came back divided by 1000, and a
file declaring `METRE` was left alone.

## `bbox` is in the same frame `read-model` returns
IFC's own world frame: **X and Y in plan, Z up**, millimetres — reported as `frame: "z-up"` in the
output, so a consumer checks it instead of assuming. `read-model`'s vertices use that frame too (it
rotates web-ifc's Y-up tessellation back — see `read-model.md`), so the two share axes and units and
their spans are comparable. They did not used to be, which is the bug this pairing exists to make
impossible: aware-aeco/aware#343.

Shared axes are not a shared origin, though — see below. Compare *scale*, not containment.

## `bbox` is approximate, and `null` when it cannot be trusted
It comes from the file's own `IfcCartesianPoint`s scaled by the declared unit, so it includes local
profile coordinates and ignores placement nesting. It is not the authoritative extent — that comes
from real geometry, via `read-model`.

**Read the CORNERS, never the centre.** The box is the extent of the points the file writes, and a
file's points include every placement origin — so the box is routinely pinned to the world origin at
one end and to the model at the other, which puts its centre roughly half way to the model rather than
on it. Measured on the four connection fixtures in this repo (2026-08-06), the box centre sits **9.6 to
12.6 model-spans** away from the centre of what `read-model` returns. A consumer asking *"is this
reference sitting miles from my model?"* must therefore compare the box's **extent** against the
model's, not its midpoint; a midpoint comparison reports "miles away" for a file that is placed
correctly.

**It is not a size readout, and the gap can be large in either direction.** Because every point is read
as if it were a world coordinate, a file with nested placements is overstated: for
`11134_V_Motebello_Heistopp_Rev.ifc` this reports `42.64 × 43.93 × 74.07 m` for a model whose geometry
renders `3.62 × 3.66 × 0.74 m` — a ~10× overstatement that is within what "approximate" permits and
still useless as a size. For the same reason it can also *miss* geometry: on `baseplate-rot.ifc`, whose
frame is yawed 30° about the vertical, the tessellated connection lands outside the reported box in
plan. Ask it *"is this far from the origin, or wildly mis-scaled?"*; ask `read-model` *"how big is
it?"*. Tightening it to the placed geometry without tessellating is tracked in aware-aeco/aware#348 —
where three cheap approaches are measured and closed out, so read it before attempting a fourth. The
short version: a point-based extent **cannot** bound a swept solid, because the size lives in numbers
(`IFCRECTANGLEPROFILEDEF(…,0.4,0.4)`, `IFCEXTRUDEDAREASOLID(…,1.)`) and not in any point; composing
placements is a no-op on files that place at the origin and put their coordinates in the
representation; and excluding transform points empties the box entirely, because on a swept file
*every* point is a placement origin.

**It is `null` rather than a guess** when the length unit cannot be resolved to millimetres, or when
the file has no usable 3D points. Both cases used to produce a plausible-looking box — a factor-1
fallback for unrecognised units, and a zero box for no points. Neither is harmless: a wrong extent is
precisely what a consumer's "this looks 1000× off" check reads, so a guess here causes the exact
misjudgement the size check exists to make correctly. A zero box is a claim ("a point at the
origin"); `null` is the truth ("could not tell").

Units resolve through the SI prefix for an `IfcSIUnit`, and through `ConversionFactor` for an
`IfcConversionBasedUnit` — so an imperial file scales by its real 25.4, not by 1.

## Under the hood
Element counting walks `IfcRelContainedInSpatialStructure` rather than a hardcoded list of IFC element
types, which would silently undercount whichever schema version it was not written against.

## Failure modes
| Error | Cause | Recovery |
|---|---|---|
| `ifc-path is required` | no path given | pass `ifc-path` |
| `ENOENT …` | file not found | check the path |
