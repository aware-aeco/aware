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
bbox:              # APPROXIMATE, in millimetres, Z-up — or null when it cannot be established
  min: [x, y, z]
  max: [x, y, z]
```

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
IFC's own world frame: **X and Y in plan, Z up**, millimetres. `read-model`'s vertices use that frame
too (it rotates web-ifc's Y-up tessellation back — see `read-model.md`), so the two are directly
comparable. They did not used to be, which is the bug this pairing exists to make impossible:
aware-aeco/aware#343.

## `bbox` is approximate, and `null` when it cannot be trusted
It comes from the file's own `IfcCartesianPoint`s scaled by the declared unit, so it includes local
profile coordinates and ignores placement nesting. That is good enough for the two questions it is
asked — *"is this roughly 1000× off?"* and *"is this sitting 74 m from the origin?"* — and it is not
the authoritative extent. That comes from real geometry, via `read-model`.

**It is not a size readout, and the gap can be large in either direction.** Because every point is read
as if it were a world coordinate, a file with nested placements is overstated: for
`11134_V_Motebello_Heistopp_Rev.ifc` this reports `42.64 × 43.93 × 74.07 m` for a model whose geometry
renders `3.62 × 3.66 × 0.74 m` — a ~10× overstatement that is within what "approximate" permits and
still useless as a size. For the same reason it can also *miss* geometry: on `baseplate-rot.ifc`, whose
frame is yawed 30° about the vertical, the tessellated connection lands outside the reported box in
plan. Ask it *"is this far from the origin, or wildly mis-scaled?"*; ask `read-model` *"how big is
it?"*. Tightening it to the placed geometry without tessellating is tracked separately.

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
