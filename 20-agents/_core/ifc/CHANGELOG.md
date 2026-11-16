# Changelog

## 0.1.6 — 2026-11-16

- A new scene element, `kind:"extrusion"`, for walls, slabs and footings (FloLess add-ons S3, floless.app#3296). It is a `frame` (world mm, unit, orthogonal, right-handed) and 1..8 `items`, each a ring set (`outline` counter-clockwise, optional clockwise `holes`, the same ring rules as the `outline` section with at most 64 holes and 512 points a ring) extruded from `fromMm` along the normal by `depthMm` (at least 0.00001 mm, so no written depth rounds to zero). `class` `wall` / `slab` / `footing` writes `IfcWall` / `IfcSlab` / `IfcFooting` (no class: `IfcBuildingElementProxy`) with its IFC4 `PredefinedType` (validated; `USERDEFINED` requires `objectType`, written as `ObjectType`), placed by the frame (Z = normal, X = uDir), with one `IfcExtrudedAreaSolid` per item (its profile named `item <k>`) over `IfcArbitraryClosedProfileDef` or `IfcArbitraryProfileDefWithVoids`. A wall or footing `axis {u0,u1}` is an `Axis` `Curve2D` polyline; `loadBearing` is `Pset_WallCommon` / `Pset_SlabCommon` / `Pset_FootingCommon` `LoadBearing`; `quantities` are the class's `Qto_*BaseQuantities` (mm, m², m³, kg, with gross and net weight for every class; only the keys sent, any other key refused); `material {layers:[{name, thicknessMm, grade?}]}` (exactly one layer today, unknown keys refused at every level) is an `IfcMaterial` named by its layer; `meta.description` / `meta.properties` go through the member label path. An invalid extrusion is refused before anything is written, naming the exact rule. The result counts them in `extrusions`. A scene without extrusions is written byte for byte as before.

## 0.1.5 — 2026-11-15

- A member's `xsection` may be an arbitrary closed section, `{shape:"outline", outer:[[x,y],...], holes?:[[[x,y],...],...]}` (millimetres, about the member axis, rolled with the member like every other section). Without holes it is an `IfcArbitraryClosedProfileDef` over a closed `IfcPolyline`; with holes, an `IfcArbitraryProfileDefWithVoids` whose `InnerCurves` are one closed `IfcPolyline` per hole (profile type `.AREA.`; the outer ring is written counter-clockwise, the holes clockwise). An invalid outline (fewer than 3 points, non-finite values, self-intersecting, zero area, a hole not strictly inside the outer ring or touching another, more than 16 holes, or wider/deeper than a declared `section.w` × `section.d`) falls back to the rectangle with a per-member warning that names the exact rule. Refs #797.

## 0.1.4 — 2026-11-11

- A member (`member`, `line` or `box`) may carry its own labels in `meta` (#818). `meta.description`, a string of 1..256 UTF-8 bytes with no control characters, is written as the element's IFC `Description`; ObjectType stays the `meta.profile` designation. `meta.properties`, `{ name, properties }` under the same rules as the scene's `meta.project_properties`, is written as one `IfcPropertySet` of `IfcLabel` values in key order, attached to that element by its own `IfcRelDefinesByProperties`, with GlobalIds derived from the element id. A malformed one is refused before anything is written, naming the member. The member's receipt row gains `description: true` and `propertySet: "<name>"` only when they were written; a member without them is written byte for byte as before.

## 0.1.3 — 2026-11-11

- A member's `xsection` may be a solid round bar, `{shape:"solid-circle", diameter}`. It is written as the member's own `IfcBeam`, `IfcColumn` or `IfcMember` (from its `role`) with an `IfcCircleProfileDef` of radius `diameter/2`, never as a fastener. A missing or non-positive `diameter`, or a `section` envelope other than `diameter` by `diameter`, falls back to the rectangle with a warning. `chs` is unchanged (#811).

## 0.1.2 — 2026-08-19

- Mark members whose zero frame came from the vertical seed with `zeroFrame: "vertical-seed"` in the write receipt, so a reader can tell a facing fixed by convention from one derived from the member's geometry.

## 0.1.1 — 2026-08-18

- Validate and normalize finite member `rot` against the shared directed right-handed section frame; reject unsupported Y-up scenes instead of silently exporting them as Z-up.
