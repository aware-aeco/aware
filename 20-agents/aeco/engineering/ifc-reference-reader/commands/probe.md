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

**The box is usually pinned to the world origin, so its midpoint is not the model's position.** A
file's points include every placement origin — the representation context's `WorldCoordinateSystem`,
the site and building placements, every product's placement — and those sit at `(0,0,0)`. `min` is
therefore `[0,0,0]` on every file measured here, and the midpoint lands about half way from the origin
to the model.

How far off that puts it **depends entirely on how far the model is from the origin**, not on the
algorithm: the centre error is roughly `distance-from-origin / 2`, so in units of the model's own
longest edge it is `distance / (2 × longest-edge)`. Measured 2026-08-06:

| file | centre error, in model longest-edges |
|---|---|
| `example-steel-framing.ifc` (authored at the origin) | **0.01** — the midpoint is essentially exact |
| the four connection fixtures here (~22 m offset, ~1 m connection) | **9.6 – 12.6** |
| `11134_V_Motebello_Heistopp_Rev.ifc` | **12.9** |

**`bbox` alone does NOT tell you which case you are in.** This page used to say it did, via a rule that
is worse than useless: *"when `|max|` is much larger than the box's own span the box is origin-pinned;
when the two are comparable the midpoint is fine."* Whenever `min` is `[0,0,0]` — which the paragraph
above states is every file measured — the span **is** `max`, so those two quantities are the same
number and the ratio is exactly `1`. The rule therefore reports "comparable, the midpoint is fine"
about precisely the files whose midpoint is 9.6–12.6 longest-edges out, and it can never fire the other
way on an origin-pinned box at all.

It is also inverted on the arm that *can* fire. A ratio much larger than 1 requires a box that
**excludes** the origin — a tight box far out, which is the case whose midpoint is trustworthy.
Measured on `baseplate-bp1.ifc`: probe's origin-pinned box scores `1.00` (midpoint 10.2 longest-edges
out), while the real mesh AABB of that same file scores `18.0` (midpoint exact).

What `bbox` does tell you: **`min` at `[0,0,0]` with a distant `max` is the origin-pinned signature**,
and it is the ordinary case. That signature does not separate "the model is authored at the origin", where
the midpoint is essentially exact, from "the model is 22 m out", where it is half that far off — both
put `min` at `[0,0,0]`, and telling them apart needs the model's own size, which only `read-model` has.
So unless you already know the model's scale from elsewhere, treat the midpoint as unusable and read the
far corner instead, with the rotated-frame caveat below.

Every figure in this section is now asserted by `cli-connection-reader/probe.test.mjs` against the four
fixtures that ship in this repo, so it runs on CI rather than resting on a one-off measurement — which
is how the rule above survived being written down.

The corner **farther from the origin** tracks where the model actually is — within one longest-edge
on every file measured (0.01, 0.44, 0.89) — **except under a rotated frame**, where it does not: on `baseplate-rot.ifc`
(yawed 30°) that corner is 10.3 longest-edges out, barely better than the midpoint's 12.6, and on the
wrong side of the model. See the next paragraph.

What this does **not** license is comparing the box's SPAN against a model's to judge position. The
span runs 1× the model's on an origin-authored file and ~18–20× on the offset ones measured here — it is not a size (next paragraph), and a size
would not answer a position question anyway.

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
