# Plan: `xsection {shape:"outline"}` — an arbitrary closed member section, with holes (#797, floless.app#3382 row A3)

Status: DRAFT for Codex plan review. Base: `origin/main` @ 27a470e3d (v0.179.0).

## Problem

A scene member cannot carry an arbitrary closed cross-section. `xsection.shape` is the closed list
`i | channel | angle | rhs | chs | solid-circle | rect | tee | double-angle`. `ifc.write` (`emit_profile`) falls
back to a rectangle with `unknown xsection shape '<x>'`; `viewer-3d` refuses the scene; `outline` is read only
for `kind:"plate"`. FloLess's custom/catalog section engine (epic floless.app#3382) already evaluates every
section to `{ outer, holes[] }` in mm about a reference point, and needs to send exactly that.

## The contract (one canonical descriptor)

```json
"xsection": { "shape": "outline", "outer": [[x,y], ...], "holes": [ [[x,y], ...], ... ] }
```

* **Units and frame.** mm, in the section plane. `x` runs along the member's section width axis (the axis
  `section.w` measures), `y` along the section depth axis (`section.d`) — the same 2D frame every parametric
  shape is drawn in, rotated by the member's `rot` roll exactly as they are. The origin `(0,0)` is the member
  axis (`from`→`to` line). Unlike the parametric shapes it is NOT re-centred on the bounding box: the outline is
  used as sent, so an eccentric section (FloLess' "reference point is never replaced by the bounding-box
  centre") sits eccentric about the axis.
* **Winding.** Either is accepted. The validator normalises: `outer` counter-clockwise, each hole clockwise
  (viewed along the extrusion direction = the IFC convention). A ring whose last point repeats its first is
  accepted and the repeat is dropped.
* **Limits.** each ring has 3..=4096 distinct vertices (bounds the O(n^2) simplicity test); `holes` is optional
  (absent / `null` = `[]`), at most 16.
* **Envelope.** When `section.w` and `section.d` are both given (positive finite numbers), the outline's
  bounding-box extents must fit within them: `(maxx-minx) <= w` and `(maxy-miny) <= d` (tolerance
  `max(1e-6, 1e-9*scale)` as the siblings use). Not equality — the siblings' exact-envelope rule is for shapes
  whose envelope is a function of their parameters and centred on the axis; an eccentric outline is not.
  Reasoning for reject-vs-warn: siblings (`tee`, `double-angle`, `solid-circle`) treat an envelope mismatch as an
  INVALID descriptor, not a soft warning (IFC: rectangle fallback + warning; viewer: refuse the scene). Outline
  follows: a declared envelope that the outline exceeds is invalid, never silently accepted.

### Validation (errors-as-data, each with an exact reason)

One shared module `cli/src/render/section_outline.rs`: `parse_outline(xsection, declared_wd) ->
Result<OutlineSection{outer,holes}, String>` where the `Err` string names the ring and the rule:
not an array / fewer than 3 points / point not a `[x,y]` pair of finite numbers / > 4096 points / zero-length
(repeated consecutive) edge / spike (an edge folding back on its neighbour) / self-intersecting / zero area /
`holes` not an array / more than 16 holes / hole not strictly inside outer (any vertex on or outside, or any edge
touching an outer edge) / two holes overlapping, touching or nested / outline wider or deeper than `section.w`
or `section.d`. Reuses `geom.rs` (`polygon_is_simple_nonzero`, `point_in_polygon`, `segments_intersect`).

### Sinks

| Sink | Behaviour |
|---|---|
| `ifc.write` (`ifc.rs`) | no holes -> `IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,name,#IfcPolyline)`; with holes -> `IFCARBITRARYPROFILEDEFWITHVOIDS(.AREA.,name,#outer,(#inner,...))`. Polylines closed (first point repeated). Points rotated by the member roll (these profile entities have no `Position`, as the existing double-angle composite). Invalid -> the existing per-member labelled `rectangle fallback` warning, naming the rule. Never a silent rectangle. |
| `viewer-3d` | scene validation refuses an invalid outline with the same reason (as every sibling shape); a valid one is extruded as a `THREE.Shape` with `holes`. |
| Rhino bake | already per-member `failed/invalid-xsection`; message now names `outline` as not supported by the Rhino bake yet. |
| SketchUp bake | today an unknown shape silently falls to the designation classifier. Now a per-member failure naming `outline`. |
| Tekla bake | places by profile name and ignores xsection; now a per-member `invalid-geometry` failure naming `outline` (a profile name cannot carry an arbitrary section; the Item projection is a later row). |
| Revit bake | unknown shape already lands as a labelled `profile-family-not-loaded` placeholder warning ("scene carried no drawable section"); documented, not changed. |

### Specs / versions

`scene-schema.md` (xsection list, new `outline` section, the host table), `20-agents/_core/ifc/manifest.yaml`
description + schema text + 0.1.5 + CHANGELOG, `viewer-3d` 0.2.6 + CHANGELOG. Host-bake agent manifests are not
bumped in this PR (v0.177.0/#813 touched `BakeSceneScript.rb` the same way and bumped them in the registry
follow-up #821); `registry-catalog.json` regenerated by `aware agent reindex`.

## Tests (TDD)

Rust unit tests per validation rule in `section_outline.rs`; IFC text tests for the no-holes and with-holes cases
(entity names, coordinates, `.AREA.`, `InnerCurves`, no `IFCRECTANGLEPROFILEDEF`, roll applied) and the labelled
fallback; viewer tests for accept (HTML carries the outline kind) / reject (error names the rule); C# tests for
the Rhino message. Then the real E2E in a temp `AWARE_HOME`.

## Questions for the reviewer

1. Is containment (not equality) the right envelope rule given an eccentric reference point?
2. Is the IFC entity syntax right (IFC4 `IfcArbitraryProfileDefWithVoids(ProfileType, ProfileName, OuterCurve, InnerCurves)`)?
3. Is "rotate the points by roll, since there is no Position" correct and consistent with how `rot` applies to the
   parametric shapes (`prof_pos` carries a RefDirection) and the double-angle composite?
4. Anything where the invalid path could still produce a silent wrong section?

## Revision 1 (response to Codex round 1: REVISE)

1. **`section.{w,d}` semantics.** For an outline it is the *size* of the section (extent), never its position;
   position comes from the points alone, and consumers take spatial bounds from them. Both are optional (a
   missing `section` is the legacy default for every shape, so nothing is checked). The viewer's scene bounds now
   include a sphere about each member end that holds the outline whatever the roll (`expandSceneBounds`).
2. **Total vertex budget**: already 8192 across all rings (the per-ring figure in the first draft is gone).
3. **Scale-aware predicates.** `section_outline` no longer uses `geom`'s absolute `1e-9` segment tests. Every
   coordinate is bounded to |c| <= 1e6 mm, and "same point / touching / no area" use a 1e-6 mm length tolerance (a
   hair under IFC's 1e-5 mm context precision) via distance tests, not raw cross products; "no area" is
   `area <= 1e-6 * max(1, extent)`. A test exercises 900 km off-axis coordinates.
4. **Rendered geometry.** The real-browser E2E asserts the extruded `BufferGeometry` of the viewer for both
   windings, a repeated closing point, a hole, an eccentric outline and a roll (vertex extents and the hole's
   absence of faces), on top of the HTML-contains check. The viewer drops a repeated closing point itself.
5. **Per-host tests.** Rhino (`BakeSceneRules.Validate` row `invalid-xsection` naming `outline`), SketchUp and Tekla
   (the script text pins the guard before any classification / roll plan; Tekla's Roslyn syntax test compiles it),
   Revit (`TrySectionOutlineMm("outline")` is false, so the existing labelled placeholder warning applies). No live
   hosts are available in this environment; that is stated in the PR.
6. **Byte-for-byte.** The only change on the existing path is an extra argument to `emit_profile`; the E2E writes
   a scene of the existing shapes (including a rolled double-angle and a member with no `xsection`) with the new
   build and with a pre-change build and compares the IFC bytes.

## Revision 2 (response to Codex round 2: REVISE)

1. **Area predicate** now measures from the ring's own first vertex and against the ring's own extent (`area <=
   1e-6 mm * extent`), so a 0.5 x 0.5 mm ring 900 km off-axis is accepted (test added).
2. **Vertex budget** counts after each ring's optional closing repeat is dropped (a ring of exactly 8192 distinct
   vertices plus its repeat is within the limit); a raw `8192 + 1 + 16` guard still refuses an absurd array
   before it is read (test added).
3. **Revit loaded-family path.** A member whose `xsection.shape` is `outline` is refused in `ReadMember`, before
   the loaded-family lookup, with a per-member `invalid-xsection` row; the first draft's claim that Revit placed a
   labelled placeholder was wrong for a loaded family. `scene-schema.md` corrected; a parse-level test added.
4. **Browser geometry and byte identity** are done as evidence, and the browser gate (`cli/tests/browser/run.mjs`)
   gains an outline case (both windings, repeated closing point, hole, eccentric, rolled) asserting every authored
   point is a rendered vertex through the rendered roll basis and that no other vertex exists. The pre-change
   comparison ran: a scene of existing shapes (i, rolled double-angle, no `xsection`, solid-circle) written by the
   `origin/main` build and by this build is byte-identical.

## Revision 3 (Codex round 3: one defect; then code review round 1: one finding)

* Round 3 (final plan round): winding normalisation still used absolute-coordinate area. `signed_area2` now takes
  the area from the ring's own first vertex, so the sign and the normalisation agree with the "no area" check
  (test: a clockwise 0.0001 mm square at 900 km). That round was not re-run: the plan-review budget is three.
* Code review round 1 (P2): the viewer stores vertices in single precision, so a section far from the axis lost
  small features. The viewer now draws an outline about its own centre and moves the mesh back along the rolled
  section axes (`profileOffset`); a 0.002 mm square 90 m off-axis keeps all 8 exact vertices in the browser.
