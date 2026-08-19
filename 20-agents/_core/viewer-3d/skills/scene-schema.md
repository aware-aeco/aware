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
{shape:"chs",od,t} | {shape:"rect",w,d} |
{shape:"tee",d,bf,tw,tf} |
{shape:"double-angle",d,b,t,gap,orientation:"llbb"|"slbb"}
```

Every discriminant and field name is exact and lowercase. `channel` uses
`bf`, never the non-canonical `b` alias. Shape envelope dimensions must agree
with `section.{w,d}`; thickness dimensions must be positive and leave a
non-degenerate web, flange, leg, or void. Consumers that promise nominal
profiles fail a present malformed descriptor rather than guessing thickness.
An absent descriptor remains the legacy rectangular member envelope.

`tee` has a top flange (`bf` × `tf`) and a centred web of thickness `tw`; its
envelope is `{w:bf,d:d}`. `double-angle` describes two identical sharp-corner
angles. `orientation:"llbb"` places their long (`d`) legs back-to-back, so its
envelope is `{w:2*b+gap,d:d}`. `orientation:"slbb"` places their short (`b`)
legs back-to-back, so its envelope is `{w:b,d:2*d+gap}`. `gap` is the clear,
positive separation between the facing legs. All fields and orientation values
are exact lowercase tokens. A sink that cannot materialize the disconnected
double-angle geometry must report it as explicitly unsupported; it must never
silently substitute a different section.

`rot` is optional and is applicable only to physical `member`, `line`, and `box` records. When
present it must be a finite JSON number of degrees: positive is right-handed about the directed
`from→to` axis. Consumers normalize with `((degrees % 360) + 360) % 360`, subtracting 360 at 180,
so the canonical range is `[-180,180)` and negative zero becomes positive zero.

The deterministic zero-section frame uses declared scene up (`+Z` when absent/`"z"`, `+Y` for
`"y"`). Let `n=normalize(to−from)`, `u` be scene up, and `p = n − (n·u)u` be the part of the axis
perpendicular to up. When `|p|² <= 1e-6`, zero X is scene `+X` projected perpendicular to `n` and
normalized, and zero Y is `n×X`. Otherwise zero Y is `u` projected perpendicular to `n`, and zero
X is `Y×n`. `rot` rotates this frame once about `n`. Compute the branch test as `|p|²` and not as
`1−(n·u)²`: the two agree in exact arithmetic, but the second cancels away about six significant
digits at the threshold and more than twelve deeper into the band, and real members fall on
opposite sides of the two readings — which, by the paragraph below, means two conforming
implementations disagreeing about one member's facing by as much as 180°.

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
  "levels":[{"id":"L-1","elevationMm":3000,"label":"Level 1"}]
}
```

`bounds` and axis offsets are local to `origin` in plan. A direction `"x"` axis is located at
local X=`offsetMm` and runs along Y; direction `"y"` is located at local Y and runs along X.
Optional `startMm`/`endMm` replace the matching bound. `elevationMm` is an absolute world Z datum.
Each level renders as a labelled crosshair spanning the grid bounds. The legacy `grids[]` label
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
validation error and no partial HTML. `output-path`/`path` are present only when requested; dry
runs report the would-be path without writing it.

The HTML uses a pinned Three.js CDN and therefore needs a script-enabled client surface. Camera
fit, projection/display controls, selection, clipping, the ViewCube, group legend, and themed
tooltips are built into the renderer.
