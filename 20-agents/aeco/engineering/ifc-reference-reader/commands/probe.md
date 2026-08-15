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
the site and building placements, every product's placement — and those sit at `(0,0,0)`. On a file
authored wholly in the positive octant `min` is therefore `[0,0,0]` exactly, and the midpoint lands
about half way from the origin to the model. (It is not a law: `baseplate-origin.ifc`, whose anchors
straddle the origin, reports a negative `min`.)

How far off that puts it is **dominated by how far the model is from the origin**: the centre error is
roughly `distance-from-origin / 2`, so in units of the model's own longest edge it is
`distance / (2 × longest-edge)`. That term is a property of the file, not of the algorithm, and on the
offset fixtures it accounts for essentially all of the error.

It is not the whole of it, though. `baseplate-origin.ifc` is authored *at* the origin, where the term
predicts ~0, and still measures `0.44` — because the box cannot see the swept column that model is
mostly made of (below).

That residual is **this fixture's, not a floor under every file**. How much survives at zero distance
depends on how the geometry is represented: a model given as explicit 3D points (a BRep, or tessellated
items) hands `probe` the real vertices and can measure essentially zero, and even an unseen sweep that
happens to be symmetric about the point-box midpoint leaves the centre where it was. So read the formula
as the dominant term, with a residual whose size is a question about the file's representation rather
than a constant. Measured 2026-08-07:

| file | centre error, in model longest-edges |
|---|---|
| `baseplate-origin.ifc` (authored at the origin) | **0.44** |
| `example-steel-framing.ifc` (authored from the origin) | **0.01** — the midpoint is essentially exact |
| the four connection fixtures here (~23 m out, ~1 m connection) | **9.6 – 12.6** |
| `11134_V_Motebello_Heistopp_Rev.ifc` | **12.9** |

**`bbox` alone does NOT tell you which case you are in.** This page used to say it did:

> when `|max|` is much larger than the box's own span, the box is origin-pinned and the midpoint is
> meaningless. When the two are comparable, the file is authored near the origin and the midpoint is
> fine.

**The rule cannot fire on any file measured here, and a ratio it *could* fire on would not mean what
it says.** Two separate reasons, and it is worth keeping them apart:

*First, the arm it offers is unreachable on every file measured.* Whenever a box **contains the
origin**, `|max|` is at most the box's own span on each axis, so the ratio is `≤ 1` and "much larger"
cannot happen. It is exactly `1` when `min` is `[0,0,0]` (all four offset fixtures) and below it
otherwise (`0.50` on `baseplate-origin.ifc`).

Note what that argument does *not* say. Containing the origin is a property of the **files**, not of
this command: `probe` bounds the file's own 3D points and never inserts the origin, so a file **can**
produce a box that excludes the origin, and such a box **can** score above `1`. Neither follows
automatically from nonzero anchors — points at `(1,1,1)` and `(-1,-1,-1)` are both nonzero and still
bracket the origin, and even an origin-excluding box scores at most `1` if some other axis is wide
enough. Every file measured here anchors those points at `(0,0,0)`, which is ordinary but not
guaranteed. Read the `≤ 1` as measured, not as a law about the algorithm.

*Second — and this is the part that holds regardless* — a high ratio would not rescue the midpoint
anyway. It tells you only that the box **excludes** the origin, which is not the same as a trustworthy
midpoint: a point-based box can exclude the origin, score 84, and still have
its midpoint 0.44 longest-edges out, because it cannot see the swept column (below). **If you need the
model's position, call `read-model`** — the same answer this page already gives for size. That is not a
counsel of despair: `read-model` is what a consumer runs next anyway, and `probe`'s job is to say
whether that call is affordable, which `elements`, `storeys` and `types` answer without the box.

What is asserted on CI, in `cli-connection-reader/probe.test.mjs` against the five fixtures that ship
here: the origin-pinned `min`, the `≤ 1` lemma and its `= 1` equality case, the non-containment and its
exact Z shortfall, the 9–13 midpoint band, and the origin-authored contrast. The figures above drawn
from `~/Downloads` files (`0.01`, `12.9`) and the far-corner numbers below are still one-off
measurements — treat them as dated evidence, not as guarded contract.

The corner **farther from the origin** tracks where the model actually is — within one longest-edge
on every file measured (0.01, 0.44, 0.89, corner-to-corner) — **except under a rotated frame**, where it
does not: on `baseplate-rot.ifc` (yawed 30°) that corner is 10.3 longest-edges out, barely better than
the midpoint's 12.6, and on the wrong side of the model. Nothing in the response marks that case, so
this is an observation about the files measured, **not a method to follow**.

What this does **not** license is comparing the box's SPAN against a model's to judge position. Measured
against each model's longest edge, the span runs **0.2×** on `baseplate-origin.ifc`, ~1× on
`example-steel-framing.ifc`, and **17–18×** on the four offset fixtures — so it is neither bounded nor
monotonic in the distance you would be trying to read off it. It is not a size (next paragraph), and a
size would not answer a position question anyway.

**It is not a size readout, and the gap can be large in either direction.** Because every point is read
as if it were a world coordinate, a file with nested placements is overstated: for
`11134_V_Motebello_Heistopp_Rev.ifc` this reports `42.64 × 43.93 × 74.07 m` for a model whose geometry
renders `3.62 × 3.66 × 0.74 m` — a ~10× overstatement that is within what "approximate" permits and
still useless as a size. For the same reason it can also *miss* geometry: on `baseplate-rot.ifc`, whose
frame is yawed 30° about the vertical, the tessellated connection lands outside the reported box in
plan. Ask it *"is this file affordable to read?"* — `elements`, `storeys`, `types`; ask `read-model`
*"how big is it, and where?"*. Tightening it to the placed geometry without tessellating is tracked in aware-aeco/aware#348 —
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
