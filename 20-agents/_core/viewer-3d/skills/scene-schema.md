# viewer-3d — scene schema

`viewer-3d.render` consumes a domain-neutral millimetre scene. Producers own domain meaning;
the renderer owns exact visualization and an exhaustive identity receipt. `meta.units` may be
omitted or must be `"mm"`. `meta.up` may be `"z"` (structural coordinates) or `"y"`.

Every element, plate hole, operation, operation instance/effect, reference system, grid axis,
and elevation datum has a non-empty stable `id`. IDs are globally unique. A malformed supported
record rejects the command before HTML is created or an `output-path` is written. An unknown kind
is returned in `unsupported`; it is never silently skipped.

## Scene envelope

```jsonc
{
  "meta": { "name": "Connection", "units": "mm", "up": "z" },
  "groups": [{ "key": "plate", "label": "Plates", "color": "#60a5fa", "opacity": 1 }],
  "elements": [],
  "operations": [],
  "referenceSystems": [],
  "grids": [{ "label": "legacy label", "at": [0,0,0] }],
  "panels": [{ "title": "Takeoff", "note": "…", "columns": ["Part"], "rows": [["PL-1"]] }],
  "camera": { "eye": [5000,5000,5000], "target": [0,0,0] }
}
```

`groups`, `grids`, `panels`, and `camera` retain their original behavior. Element `opacity`
overrides group opacity. Coordinates stay in producer space; `up:"z"` only converts them to the
Three.js screen-up convention.

## Physical elements

Legacy primitives remain supported:

```jsonc
{ "id":"M1", "kind":"line|box|member", "from":[0,0,0], "to":[0,0,3000], "rot":82.7,
  "section":{"w":200,"d":300},
  "xsection":{"shape":"i","d":300,"bf":200,"tw":10,"tf":20},
  "group":"member", "meta":{"profile":"W12X40"} }
{ "id":"P1", "kind":"node", "at":[0,0,0], "size":100 }
{ "id":"X1", "kind":"mesh", "positions":[0,0,0, 100,0,0, 0,100,0], "indices":[0,1,2] }
```

`section.{w,d}` is the required member envelope. An optional canonical
`xsection` gives nominal sharp-corner profile geometry in millimetres:

```text
{shape:"i",d,bf,tw,tf} | {shape:"channel",d,bf,tw,tf} |
{shape:"angle",d,b,t} | {shape:"rhs",d,b,t} |
{shape:"chs",od,t} | {shape:"solid-circle",diameter} | {shape:"rect",w,d} |
{shape:"tee",d,bf,tw,tf} |
{shape:"double-angle",d,b,t,gap,orientation:"llbb"|"slbb"} |
{shape:"outline",outer:[[x,y],...],holes?:[[[x,y],...],...]}
```

Every discriminant and field name is exact and lowercase. `channel` uses
`bf`, never the non-canonical `b` alias. Shape envelope dimensions must agree
with `section.{w,d}`; thickness dimensions must be positive and leave a
non-degenerate web, flange, leg, or void. Consumers that promise nominal
profiles fail a present malformed descriptor rather than guessing thickness.
An absent descriptor remains the legacy rectangular member envelope.

`solid-circle` is a **solid** round bar of `diameter`, centred on the member
axis; its envelope is `{w:diameter,d:diameter}`. It is a structural member section like any other:
`ifc.write` keeps the member's `role` element type (`IfcBeam`, `IfcColumn`,
`IfcMember`) with an `IfcCircleProfileDef` of radius `diameter/2`, and never turns it
into a fastener (only `kind:"rod"`/`"bolt-shank"` records are fasteners). A
hollow round tube stays `chs`. Host bakes: Tekla places the member by its
profile name, as for every single-profile shape; SketchUp draws a round bar.
The Rhino and Revit bakes do not decode `solid-circle` yet: Rhino reports the member
as failed (`invalid-xsection`), and Revit, when no loaded family matches the
designation, uses its labelled nominal placeholder. None of them substitutes
another section silently.

`outline` is any **closed polygonal section**, with optional holes: a tapered or
asymmetric built-up plate section, a cold-formed or proprietary profile, a box
with unequal plates.

```jsonc
"xsection": { "shape": "outline",
  "outer": [[-150,-100],[150,-100],[75,100],[-75,100]],
  "holes": [ [[-20,-20],[20,-20],[20,20],[-20,20]] ] }   // optional, default none
```

- **Frame.** Millimetres, in the section plane, in the same frame every other shape
  is drawn in: `x` is the axis `section.w` measures, `y` the axis `section.d`
  measures, and a member's `rot` rolls the points exactly as it rolls an `i` or an
  `rhs`. The origin `(0,0)` is the **member axis** (the `from`→`to` line). The points
  are used as sent and are **not** re-centred on their bounding box, so an eccentric
  section sits eccentric about the axis; a producer that wants a centred section
  sends centred points.
- **Winding.** Either is accepted. A consumer normalises: `outer` counter-clockwise,
  every hole clockwise. A ring whose last point repeats its first is accepted.
- **Validity**, checked by every consumer with the same rules and the same
  wording: each ring has at least 3 distinct points of finite numbers, no repeated
  point, no spike, no self-crossing or self-touching, and non-zero area; each hole
  lies strictly inside `outer` and no two holes touch, overlap or nest; at most 16
  holes and 8192 points in all rings together, each within 1,000,000 mm of the axis.
  "Same point", "touching" and "no area" are judged to 0.00001 mm (the IFC context
  precision `ifc.write` declares). For an outline
  `section.{w,d}` is the **size** of the section (its extent), never its position:
  where the section sits is given by the points alone, and a consumer takes spatial
  bounds from them. When both are given, the outline's bounding-box extents must
  **fit within** `w` × `d` (it need not fill it, nor be centred on the axis); a
  larger outline contradicts the declared envelope and is invalid, as a mismatching
  `tee` or `solid-circle` envelope is. Omitting `section` leaves nothing to check.
- **Invalid is reported, never repaired.** `ifc.write` falls back to the member's
  rectangle exactly as for any other invalid descriptor, **with** a per-member
  warning naming the rule (`invalid outline: holes[0] touches or crosses the outer
  boundary — rectangle fallback`); `viewer-3d` refuses the scene with the same
  reason. A tapered section along the member (a different outline at each end) is
  not expressed: one outline is constant along the member.
- **Sinks.** `ifc.write`: no holes -> `IfcArbitraryClosedProfileDef` over a closed
  `IfcPolyline`; with holes -> `IfcArbitraryProfileDefWithVoids` whose `InnerCurves`
  hold one closed `IfcPolyline` per hole (profile type `.AREA.`). `viewer-3d`
  extrudes the outline with its holes. The Rhino, SketchUp, Tekla and Revit bakes do
  not draw an `outline` yet and fail the member with a message that names `outline`
  (Tekla and Revit place a member by profile name or loaded family, which cannot
  carry an arbitrary section, so they refuse it before any such lookup). None of
  them substitutes another section silently.

`tee` has a top flange (`bf` × `tf`) and a centred web of thickness `tw`; its
envelope is `{w:bf,d:d}`.

`angle` is chiral, and its handedness is part of the descriptor rather than a
rendering choice: the vertical (`d`) leg lies on the section frame's -X side and
the flat (`b`) leg along -Y, so the heel sits at the -X,-Y corner. No `rot`
recovers a mirrored unequal angle, and a mirrored equal one is still a quarter
turn from where it was asked for, so a sink whose native profile seats the
opposite hand must mirror that profile rather than bake the other hand.

`double-angle` describes two identical sharp-corner angles.
`orientation:"llbb"` places their long (`d`) legs back-to-back, so its
envelope is `{w:2*b+gap,d:d}`. `orientation:"slbb"` places their short (`b`)
legs back-to-back, so its envelope is `{w:2*d+gap,d:b}`. In both orientations
the two angles are a side-by-side mirrored pair; the suffix changes which leg
forms the adjacent backs, never the axis on which the pair is separated. `gap`
is the clear, non-negative separation between the facing back surfaces; zero
means those surfaces touch without overlap. All fields and orientation values
are exact lowercase tokens. A sink that cannot materialize the disconnected
double-angle geometry must report it as explicitly unsupported; it must never
silently substitute a different section.

`rot` is optional and is applicable only to physical `member`, `line`, and `box` records. When
present it must be a finite JSON number of degrees: positive is right-handed about the directed
`from→to` axis. Consumers normalize with `((degrees % 360) + 360) % 360`, subtracting 360 at 180,
so the canonical range is `[-180,180)` and negative zero becomes positive zero.

The deterministic zero-section frame uses declared scene up (`+Z` when absent/`"z"`, `+Y` for
`"y"`). Let `d = to−from` be the **raw, unnormalized** axis, `n=normalize(d)`, `u` be scene up, and
`q = d − (d·u)u` be the part of the raw axis perpendicular to up. When `|q|² <= 1e-6·|d|²`, zero X
is scene `+X` projected perpendicular to `n` and normalized, and zero Y is `n×X`. Otherwise zero Y
is `u` projected perpendicular to `n`, and zero X is `Y×n`. `rot` rotates this frame once about
`n`.

**Compute that branch test exactly as written: the cross-multiplied comparison
`|q|² <= 1e-6·|d|²`, on the raw `d`.** Every obvious algebraic rearrangement of it is equivalent in
exact arithmetic and *not* in floating point, and by the paragraph below a member placed on the
wrong side comes out with its facing up to 180° away — so two implementations that round the test
differently disagree about real geometry while both believing they conform. Each rearrangement
below has a witness (all with `u = +Z` and `from = [0,0,0]`), and each is forbidden:

- **Do not test `1−(n·u)²`.** That subtraction cancels away about six significant digits at the
  threshold and more than twelve deeper into the band. Witness:
  `to = [-0.0009800740994886435, -0.00019863222178516707, 0.999999499999875]`, whose true
  perpendicular component is just outside the threshold but which the cancelling form seeds.
- **Do not test the normalized perpendicular `|n − (n·u)u|² <= 1e-6`.** The branch would inherit
  however the implementation normalizes, and dividing each component by the length versus
  multiplying by the reciprocal differ by an ulp. Witness:
  `to = [26.55075466049515, -17.294594180470543, 31686.662128779197]` — the dividing form seeds,
  the reciprocal form projects, frames **57°** apart.
- **Do not divide: `|q|²/|d|² <= 1e-6` is not the same test.** The quotient rounds to a different
  side of the threshold than the cross-multiplied comparison does. Witness:
  `to = [1.1976826775898164, -0.5615310404423273, 1322.784621855746]`, where
  `|q|² = 1.7497609055789547` exceeds `1e-6·|d|² = 1.7497609055789545` and the member projects,
  while `|q|²/|d|²` rounds to exactly `1e-6` and seeds — frames **64.9°** apart. "Ratio against
  `|d|²`" means the cross-multiplied comparison, never a division.

Taken exactly as written, the test involves no normalization and no division, and is therefore
reproducible across implementations.

**The two rules do not meet, and the projected seed does not make them.** They are separate
conventions and the frame jumps where they change over. Writing `n` as
`[sinθ·cosφ, sinθ·sinφ, cosθ]` under Z-up, the projected-up rule leaves the threshold at exactly
`zero X = (−sinφ, cosφ, 0)` while the seeded rule holds `zero X ≈ +X` for every `φ`, so crossing
the threshold rotates the frame about `n` by `φ + 90°` — nothing at `φ = 270°`, a full 180°
inversion at `φ = 90°`. No seed removes this, because the direction the projected-up rule
approaches depends on the member's azimuth, so agreeing at one azimuth forces disagreeing at
another; and no placement of the threshold removes it either, because a frame varying
continuously over every axis direction does not exist. The threshold is therefore a deterministic
convention with an accepted discontinuity, not an approximation of some continuous rule. A
near-vertical member's facing is convention rather than geometry, and producers and consumers
match AWARE on one only by applying this exact test with this exact constant first. AWARE's own
implementation reports which rule ran per member (`ZeroFrameSource`) so a consumer can tell the
two apart.
The viewer supports both declared up axes and accounts for its reflective Z-up screen conversion;
export sinks may explicitly reject Y-up until they implement a reviewed coordinate transform.

Connection solids use direct geometry rather than member cross-section hints:

```jsonc
{ "id":"PL-1", "kind":"plate",
  "frame":{"origin":[0,0,1000],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
  "outline":[[-200,-150],[200,-150],[200,150],[-200,150]],
  "thicknessMm":20,
  "holes":[{"id":"H-1","center":[0,0],"diameterMm":24}] }

{ "id":"R-1", "kind":"rod|bolt-shank",
  "axis":{"from":[0,0,0],"to":[0,0,500]}, "diameterMm":20 }

{ "id":"W-1", "kind":"washer", "center":[0,0,510],
  "axis":[0,0,1],
  "outerDiameterMm":44, "innerDiameterMm":22, "thicknessMm":4 }

{ "id":"N-1", "kind":"nut|bolt-head", "center":[0,0,530],
  "axis":[0,0,1],
  "acrossFlatsMm":32, "thicknessMm":18, "phaseRad":0 }
```

- A plate `origin` is its mid-plane frame origin. `outline` is a polygon in `(u,v)` and the
  solid spans `±thicknessMm/2` along `normal`. `uDir`, `vDir`, and `normal` must form a finite,
  nonzero, orthogonal right-handed frame.
- Every circular plate hole must lie wholly inside the outline and must not overlap or touch
  another hole. It becomes a real profile void, not a painted marker.
- Rods and bolt shanks are exact cylinders along the full axis.
- A washer is an exact annular prism centered at `center`, oriented by the nonzero direction
  vector `axis`, and extruded to
  `thicknessMm`.
- A nut or bolt head is a regular hexagonal prism. For across-flats dimension `a`, its
  circumradius is `a/sqrt(3)`; vertex zero is at `phaseRad` in the deterministic local axis basis.

## Extruded elements: walls, slabs and footings (`kind:"extrusion"`)

A planar shape with optional openings extruded along a direction, with an optional building class.
It is **not** a member: it has no `from`/`to` axis, no `section`, no `rot` and no `role`, and no sink
treats it as a beam, column or brace (a slab faked as a member would export as `IfcBeam` and bake as
one; see FloLess S3 §9).

```jsonc
{ "id": "W-1", "kind": "extrusion",
  "frame": { "origin": [1000,2000,0], "uDir": [1,0,0], "vDir": [0,1,0], "normal": [0,0,1] },
  "items": [ { "outline": [[0,-100],[6000,-100],[6000,100],[0,100]],
               "holes": [],                    // optional rings, default none
               "fromMm": 0, "depthMm": 3000 } ],
  "class": "wall",                             // optional: "wall" | "slab" | "footing"; absent = generic shape
  "predefinedType": "SOLIDWALL",               // optional, the class's IFC4 enumeration
  "objectType": "Acme CMU 200",                // required when predefinedType is USERDEFINED
  "axis": { "u0": 0, "u1": 6000 },             // walls and footings only
  "loadBearing": true,                         // optional boolean
  "material": { "layers": [ { "name": "Concrete block", "thicknessMm": 200, "grade": "CMU-200" } ] }, // optional, one layer today
  "quantities": { "lengthMm": 6000, "widthMm": 200, "heightMm": 3000, "grossVolumeM3": 3.6 },
  "group": "wall",
  "meta": { "description": "…", "properties": { "name": "FloLess", "properties": { "…": "…" } } } }
```

- **Identity.** `id` is required and unique across the scene, like every record. It is the stable
  reference target for anything that later points at this element (a base plate bearing on a
  footing, a member connected to a wall). No reference field is defined yet; a producer keeps an
  element's `id` stable across exports so such references can be added without renaming. The
  element's IFC `Name` is `meta.label`, else `name`, else `id`, and must be at most 255 UTF-8 bytes
  (an `IfcLabel`); a longer one is refused naming the field.
- **Frame.** World millimetres. `uDir`, `vDir` and `normal` are unit vectors (to 0.000001),
  mutually orthogonal (to 0.000001) and right-handed: `normal = uDir × vDir`. A non-unit, skewed or
  left-handed frame is refused, never normalised. `rot` does not apply.
- **Items.** 1 to 8 solids. Each is its rings in the frame's `(u, v)` plane, from the plane `fromMm`
  (any finite number, along `normal`) to `fromMm + depthMm`; `depthMm` is at least 0.00001 mm (the ring tolerance, so no written depth rounds to zero). It is **not** centred
  on the plane (unlike `plate`). The items of one element are separate solids that together are the
  element (a wall partitioned into height bands is one wall with one item per band); a producer
  keeps them disjoint.
- **Rings** follow the `outline` section's rules and wording (at least 3 distinct finite points,
  no repeated point, no spike, simple, non-zero area, holes strictly inside and neither touching,
  overlapping nor nesting, judged to 0.00001 mm), with these differences: the `outline` **must** be
  counter-clockwise and every hole clockwise (an opposite winding is refused, never reversed); at
  most 64 holes per item and 512 points in any one ring; each coordinate within 1,000,000 mm of the
  frame origin. A repeated closing point is accepted.
- **`predefinedType`** must be in its class's IFC4 enumeration, exact upper case: wall
  `MOVABLE PARAPET PARTITIONING PLUMBINGWALL SHEAR SOLIDWALL STANDARD POLYGONAL ELEMENTEDWALL
  USERDEFINED NOTDEFINED`; slab `FLOOR ROOF LANDING BASESLAB USERDEFINED NOTDEFINED`; footing
  `CAISSON_FOUNDATION FOOTING_BEAM PAD_FOOTING PILE_CAP STRIP_FOOTING USERDEFINED NOTDEFINED`; a
  generic shape `COMPLEX ELEMENT PARTIAL PROVISIONFORVOID PROVISIONFORSPACE USERDEFINED NOTDEFINED`.
  `USERDEFINED` requires `objectType` (IFC4 `CorrectPredefinedType`), a label of 1 to 255 bytes.
- **`axis`** is the centreline along `u` at `v = 0`, from `u0` to `u1 > u0`, for a wall or a footing
  (a strip footing); refused on a slab or a generic shape.
- **`loadBearing`** and **`quantities`** need a class. `quantities` values are finite and
  non-negative, in mm, m², m³ and kg, and only the keys of the class's IFC4 base quantity set are
  accepted (any other key is refused, not dropped):

  | Class | Keys -> `Qto_*BaseQuantities` |
  |---|---|
  | wall | `lengthMm` Length, `widthMm` Width, `heightMm` Height, `grossSideAreaM2` GrossSideArea, `netSideAreaM2` NetSideArea, `grossVolumeM3` GrossVolume, `netVolumeM3` NetVolume, `grossWeightKg` GrossWeight, `netWeightKg` NetWeight |
  | slab | `widthMm` Width (the thickness), `perimeterMm` Perimeter, `grossAreaM2` GrossArea, `netAreaM2` NetArea, `grossVolumeM3` GrossVolume, `netVolumeM3` NetVolume, `grossWeightKg` GrossWeight, `netWeightKg` NetWeight |
  | footing | `lengthMm` Length, `widthMm` Width, `heightMm` Height, `crossSectionAreaM2` CrossSectionArea, `outerSurfaceAreaM2` OuterSurfaceArea, `grossSurfaceAreaM2` GrossSurfaceArea, `grossVolumeM3` GrossVolume, `netVolumeM3` NetVolume, `grossWeightKg` GrossWeight, `netWeightKg` NetWeight |

- **`material`** is an assignment of ordered layers, outermost first:
  `{ "layers": [ { "name", "thicknessMm", "grade"? } ] }`. Today exactly **one** layer is accepted
  (two or more, or none, is refused with its rule); `name` is an IFC label, `thicknessMm` is finite
  and greater than zero, and `grade` is an optional label (a host catalogue grade). An unknown key is
  refused at every level, so several layers (an IFC material layer set) can be added later without an
  older consumer silently dropping them. AWARE does not compare the layer thickness with the
  element's geometry or quantities; the producer owns that.
- **Labels.** `meta.description` and `meta.properties` follow the member label rules exactly
  (`ifc.write` writes them through the same code), and `meta.label` / `name` name the element.
- **Invalid is reported, never repaired.** Both `ifc.write` and `viewer-3d` refuse an invalid
  extrusion with the same rule, beginning with the field (`items[0].holes[1] is not inside the outer
  boundary`); `viewer-3d`'s `partial:true` leaves that one element out instead.

`ifc.write` writes:

| Scene | IFC4 |
|---|---|
| `class: wall` / `slab` / `footing` / none | `IfcWall` / `IfcSlab` / `IfcFooting` / `IfcBuildingElementProxy`, with `PredefinedType` (unset when absent) and `ObjectType` = `objectType` |
| `frame` | `IfcLocalPlacement` of an `IfcAxis2Placement3D` at `origin`, Z = `normal`, X = `uDir` |
| each item | one `IfcExtrudedAreaSolid` in the `Body` (`SweptSolid`) representation: `IfcArbitraryClosedProfileDef` over a closed `IfcPolyline`, or `IfcArbitraryProfileDefWithVoids` with one `IfcPolyline` per hole; positioned at `(0,0,fromMm)`, direction `(0,0,1)`, depth `depthMm` |
| `axis` | an `Axis` `Curve2D` representation (an `Axis` sub-context): a 2D `IfcPolyline` `(u0,0)`→`(u1,0)` |
| `loadBearing` | `Pset_WallCommon` / `Pset_SlabCommon` / `Pset_FootingCommon` `LoadBearing` (`IfcBoolean`) |
| `quantities` | the class's `Qto_*BaseQuantities` (table above), only the keys present; lengths in the file's mm, areas m², volumes m³, weights kg (an explicit `IfcSIUnit` on each `IfcQuantityWeight`) |
| `material.layers[0]` | `IfcMaterial` named by the layer's `name`, via `IfcRelAssociatesMaterial` (deduplicated case-insensitively with member grades); `thicknessMm` and `grade` are not written yet (several layers will become an `IfcMaterialLayerSet` with its usage) |
| `meta.description` / `meta.properties` | `Description` and one property set, as for members |

The receipt row is `{id, kind:"extrusion", status:"emitted", ifcGuid, entityType, items}`, plus
`description` / `propertySet` when labels were written; the result counts them in `extrusions`.
An extrusion is not yet a `bolt-array` or `boolean-cut` target.

Per sink:

| Sink | `extrusion` |
|---|---|
| `viewer-3d` | drawn: one mesh per element (every item, holes as real openings), at its true depth, built in the frame's own coordinates; measured from the outlines' lowest corner and the first item's start plane; anything still below float32 resolution at its offset (a very thin later item, a tiny edge far from that corner) is drawn and gets an `extrusion-below-display-resolution` warning |
| `ifc.write` | written as above |
| Tekla bake | unsupported: receipted `unsupported` (`unsupported-kind`) naming `extrusion`; nothing is placed instead |
| Revit bake | unsupported, as Tekla |
| Rhino bake | unsupported, as Tekla |
| SketchUp bake | unsupported, as Tekla |

No host bake substitutes a beam, a plate or a mesh for an extrusion.

## Operations

`operations` may contain `bolt-array`, `weld`, and `boolean-cut` records. The viewer consumes
the physical child solids from `elements`; it never invents a second bolt, weld, or cut solid.
Every `bolt-array` must name a non-empty catalog `standard`; this keeps viewer, IFC, and native
Tekla materialization on one deterministic contract.
`bolt-array` and `weld` are acknowledged as relationship rows with
`geometryDuplicated:false`. Exact Boolean CSG is not implemented, so `boolean-cut` is explicitly
returned in `unsupported` with code `exact-csg-not-available` rather than approximated.
Bolt-array `instances[].id` and `instances[].holeEffects[].id` are also receipt identities.

A bolt's `holeEffects` is the **ply stack it passes through — one effect per ply, not a fixed pair.**
`partToBoltTo` and `partToBeBolted` name the primary pair and must both appear among the effects;
beyond them an instance may hole any further element it genuinely passes through. That is what makes
double shear expressible: a double-sided gusset clamps plate + angle leg + plate, so one bolt drills
three plies. Each extra ply is held to the same bar as the pair — a real physical element of a
holeable kind, never one of the array's own bolt components (a bolt does not drill itself) — and no
ply may be holed twice by one instance. Producers that emit only the pair are unaffected.

## Structural grids and elevation datums

```jsonc
{
  "id":"GRID-1", "kind":"structural-grid", "origin":[1000,2000,0],
  "bounds":{"minX":-1000,"maxX":5000,"minY":-2000,"maxY":3000},
  "axes":[
    {"id":"GX-1","direction":"x","offsetMm":0,"label":"1","startMm":-2000,"endMm":3000},
    {"id":"GY-A","direction":"y","offsetMm":0,"label":"A"}
  ],
  "levels":[{"id":"L-0","elevationMm":0,"label":"Ground"},
            {"id":"L-1","elevationMm":3000,"label":"Level 1","visible":false}]
}
```

`bounds` and axis offsets are local to `origin` in plan. A direction `"x"` axis is located at
local X=`offsetMm` and runs along Y; direction `"y"` is located at local Y and runs along X.
Optional `startMm`/`endMm` replace the matching bound. `elevationMm` is an absolute world Z datum.
Each level draws the grid's axes at its own elevation, with the axis labels at their ends and the
level's label at the plan's (minX, minY) corner; a grid with no axes draws each level as a crosshair
instead. `origin[2]` places nothing. A blank level label is shown as `Level at <elevationMm> mm`.

The Objects panel lists every grid under **Grid lines**, highest level first: an **All levels** switch
(tri-state over the grid's levels) when the grid has several, then one switch per level. A level with
`visible:false` (optional boolean, default `true`) starts switched off. The switches only show or hide:
a grid row does not select or isolate, isolation does not hide a grid, **Show all** leaves the grid as it
is, and a search lists grid rows that match their label or "grid lines" without counting them as
objects. Clip-box drawing snaps only to the levels switched on. The block appears with or without a
`legend` descriptor. The legacy `grids[]` label
array remains supported for old producers. A structural grid must contain at least one labelled
level so every sink represents the same authored elevation set.

## Output and receipts

Successful output is:

```jsonc
{
  "ok": true,
  "html": "<!doctype html>…",
  "bytes": 12345,
  "emitted": [{"id":"PL-1","status":"emitted","kind":"plate","renderedKind":"plate"}],
  "failed": [],
  "unsupported": [{"id":"CUT-1","status":"unsupported","kind":"boolean-cut",
    "code":"exact-csg-not-available","message":"viewer-3d does not approximate Boolean CSG"}],
  "warnings": [],
  "output-path": "optional/path.html",
  "path": "optional/path.html"
}
```

`emitted`, `failed`, `unsupported`, and `warnings` are always present on success. Every input
identity appears exactly once in `emitted` or `unsupported`. Invalid supported input returns a
validation error and no partial HTML, unless the caller opted in with `partial: true` (below). `output-path`/`path` are present only when requested; dry
runs report the would-be path without writing it.

The HTML uses a pinned Three.js CDN and therefore needs a script-enabled client surface. Camera
fit and zoom-to-selected, projection/display controls, a Visibility menu with Ghost unselected (fade
everything but the selection), selection, clipping, the work area, the ViewCube, group legend, and
themed tooltips are built into the renderer; the toolbar uses the same icons as FloLess's 3D model.

## Objects panel (`legend`)

A scene may carry `legend`, a descriptor for the Objects panel: `{ v: 1, interaction: "select",
modes: [{ key, label, sections: [{ key, label, categories: [{ key, label, rows: [{ key, label,
targets: [<element id>…] }] }] }] }] }` (a row may name `groups` instead of `targets`). Row and
category keys are unique per mode, and an element sits in at most one row per mode; a descriptor
that breaks a rule is dropped whole (`legendError`) and the plain group list is shown instead.

The panel's search matches a row, its category or its section, and Enter selects the one match
and zooms to it. Two optional keys help a long list:

- `legend.searchPlaceholder` — the search box's hint, naming what the rows are (e.g. "Search part
  mark or profile…"); without it the hint is "Search objects…".
- `collapsed: true` on a category — it starts folded; a search opens the categories it matches,
  and selecting an object in 3D opens the category holding its row.

## Partial render (`partial: true`)

By default one invalid supported record fails the whole render. A caller that would rather see
the rest of the model passes `partial: true` (a node's `config:` or `inputs:`). The strict
validator is still the only judge; partial mode changes who pays for a failure: the one record
that failed is left out and listed in `failed`, and the page and receipt are built from what
survives (a left-out record is not in the page's scene JSON).

```jsonc
"failed": [
  {"id":"W-9","status":"failed","kind":"washer","code":"invalid-record",
   "message":"viewer-3d render: `elements[7]` innerDiameterMm must be smaller than outerDiameterMm",
   "path":"elements[7]"},
  {"id":"H-2","status":"failed","kind":"hole","code":"invalid-hole",
   "message":"…","path":"elements[1].holes[1]","parentId":"PL-1"}
]
```

`path` is where the author wrote the record. `id` is `null` when the record has no usable id.
`code` is one of `invalid-record` (an element failed its own rules), `invalid-hole` (a plate hole
was left off its plate; the plate is kept), `duplicate-id` (the first authored record keeps the
id), `depends-on-failed` (an operation named something left out, so it goes whole: a bolt array is
never half-drawn), `rejected` (the strict check refused an operation or element for another
reason, e.g. two bolt arrays claiming one part: the earlier one wins) and
`invalid-reference-system` (a grid with any invalid axis or level is left out whole; the row
carries `childCount`). The message is the strict validator's own sentence.

Still atomic, because there is nothing to act on: `scene` not an object, `elements`/`operations`/
`referenceSystems`/`groups`/`grids`/`panels` present but not arrays, a panel whose `columns`/
`rows` are not arrays, bad `meta.up`/`meta.units`, and a scene where more than 64 whole
operations/reference systems must be removed. A scene whose every element failed still renders
(`ok: true`, `emitted: []`) with the full `failed` list.

The legend drops rows that named something left out (and the categories, sections and modes they
empty) without a `legendError` warning; an authored-invalid legend falls back as before.

**"Not shown" note.** In partial mode the page shows a small note listing what was left out: each
string in `scene.meta.leftOut` (an array the producer supplies, e.g. parts it never sent;
non-strings are ignored) followed by one plain-English line per `failed` row (e.g. `washer W-3: its hole is as large as
the washer`; the exact validator sentence stays in the receipt's `failed[].message`), at most 8 lines then "and N more".
It is static escaped text, there is no console message, and it appears only when there is
something to say. Without `partial`, `meta.leftOut` is carried in the scene but not displayed.
