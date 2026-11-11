# Changelog

## 0.1.3 — 2026-11-11

- A member's `xsection` may be a solid round bar, `{shape:"circle", d}`. It is written as the member's own `IfcBeam`, `IfcColumn` or `IfcMember` (from its `role`) with an `IfcCircleProfileDef` of radius `d/2`, never as a fastener. A missing or non-positive `d`, or a `section` envelope other than `d` by `d`, falls back to the rectangle with a warning. `chs` is unchanged (#811).

## 0.1.2 — 2026-08-19

- Mark members whose zero frame came from the vertical seed with `zeroFrame: "vertical-seed"` in the write receipt, so a reader can tell a facing fixed by convention from one derived from the member's geometry.

## 0.1.1 — 2026-08-18

- Validate and normalize finite member `rot` against the shared directed right-handed section frame; reject unsupported Y-up scenes instead of silently exporting them as Z-up.
