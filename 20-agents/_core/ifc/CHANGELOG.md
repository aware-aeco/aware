# Changelog

## 0.1.4 — 2026-11-11

- A member (`member`, `line` or `box`) may carry its own labels in `meta` (#818). `meta.description`, a string of 1..256 UTF-8 bytes with no control characters, is written as the element's IFC `Description`; ObjectType stays the `meta.profile` designation. `meta.properties`, `{ name, properties }` under the same rules as the scene's `meta.project_properties`, is written as one `IfcPropertySet` of `IfcLabel` values in key order, attached to that element by its own `IfcRelDefinesByProperties`, with GlobalIds derived from the element id. A malformed one is refused before anything is written, naming the member. The member's receipt row gains `description: true` and `propertySet: "<name>"` only when they were written; a member without them is written byte for byte as before.

## 0.1.3 — 2026-11-11

- A member's `xsection` may be a solid round bar, `{shape:"solid-circle", diameter}`. It is written as the member's own `IfcBeam`, `IfcColumn` or `IfcMember` (from its `role`) with an `IfcCircleProfileDef` of radius `diameter/2`, never as a fastener. A missing or non-positive `diameter`, or a `section` envelope other than `diameter` by `diameter`, falls back to the rectangle with a warning. `chs` is unchanged (#811).

## 0.1.2 — 2026-08-19

- Mark members whose zero frame came from the vertical seed with `zeroFrame: "vertical-seed"` in the write receipt, so a reader can tell a facing fixed by convention from one derived from the member's geometry.

## 0.1.1 — 2026-08-18

- Validate and normalize finite member `rot` against the shared directed right-handed section frame; reject unsupported Y-up scenes instead of silently exporting them as Z-up.
