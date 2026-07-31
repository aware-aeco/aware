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
elements: number   # elements placed in the spatial structure
bbox:              # APPROXIMATE, in millimetres
  min: [x, y, z]
  max: [x, y, z]
```

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

## `bbox` is approximate
It comes from the file's own `IfcCartesianPoint`s scaled by the declared unit, so it includes local
profile coordinates and ignores placement nesting. That is good enough for the two questions it is
asked — *"is this roughly 1000× off?"* and *"is this sitting 74 m from the origin?"* — and it is not
the authoritative extent. That comes from real geometry, via `read-model`.

## Under the hood
Element counting walks `IfcRelContainedInSpatialStructure` rather than a hardcoded list of IFC element
types, which would silently undercount whichever schema version it was not written against.

## Failure modes
| Error | Cause | Recovery |
|---|---|---|
| `ifc-path is required` | no path given | pass `ifc-path` |
| `ENOENT …` | file not found | check the path |
