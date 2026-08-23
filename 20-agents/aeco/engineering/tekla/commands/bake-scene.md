# bake-scene

Materializes a canonical millimetre scene in the open Tekla model. This is a
write command and requires stable `scene.meta.sourceId` and `sceneHash` values.

```powershell
$request | ConvertTo-Json -Depth 100 -Compress |
  aware-tekla bake-scene --json-stdin
```

The scene may contain exact-profile members, plates, rods, washers, nuts and
bolt heads; native bolt arrays, fillet welds and finite cylindrical Boolean
cuts; and structural grids with axes and elevation levels. Every element,
operation, reference, plate hole, bolt instance and hole effect needs a unique,
stable ID. Explicit units other than `mm` are rejected before model mutation.

Tekla stores one shared rectangular envelope per native Grid, while the canonical
scene may give each axis an independent `startMm` and `endMm`. The bridge inserts
the tightest rectangle that contains every authored segment and both families'
coordinate spans, so no grid line is truncated. When this necessarily lengthens
one or more native lines, the committed result includes a
`tekla-grid-axis-extents-expanded` warning. Every axis and level is emitted with
`realizedBy` pointing to the parent grid and shares that single native GUID.
On resolved Tekla 2026, an exact two-word label separated by one ASCII space is
mapped to a deterministic native token and reported in a
`tekla-grid-label-tokenized` warning. Elevations are always populated through
the one parent Grid's `CoordinateZ`/`LabelZ`; automatically generated grid
planes are verified read-only, never created or modified independently.
Single-family grids, duplicate family offsets/elevations, unproven whitespace
grammars, Tekla 2025 multi-word labels, and overflowing derived
spacings/envelopes are exhaustively `unsupported` without aborting unrelated
supported records.

`bake-scene` owns its commit boundary. It stages source-tagged objects, applies
ownership UDAs after each successful `Insert()`, verifies GUID and tag read-back,
replaces only the previous set with the same source ID, and calls
`CommitChanges()` once. A failure before prior-set retirement deletes staging in
reverse order and commits cleanup, leaving the previous set intact. Tekla does
not expose rollback: a deletion/commit failure after retirement starts is
reported as `commit-state-uncertain`; the bridge deliberately issues no further
commit and requires a source-reconciliation retry. The generic `exec`
auto-commit does not wrap this command.

Members require an exact requested Tekla profile. Connection parts use exact
parametric or authored contour geometry and never fall back to `100*100`.
Profile candidates are accepted only after successful insertion and GUID
read-back; this path does not use `CatalogHandler`.

Physical `member`, `line`, and `box` records may carry optional finite numeric
`rot`. Positive degrees follow the canonical right-hand rule about directed
`from→to`; the bridge converts that frame to Tekla's measured FRONT/offset
convention, then verifies both normalized native rotation read-back and the
reselected B-rep vertex orientation before prior-set retirement. Absent/`"z"`
is the only accepted `meta.up`; Y-up fails before mutation.

For guarded automation, pass `expectedModelPath` beside `scene`, or set
`AWARE_TEKLA_EXPECT_MODEL_PATH`. The bridge compares the resolved path with the
live model inside the script immediately before any work-plane change or native
insertion. `AWARE_TEKLA_QA_GUARD=1` makes an expectation mandatory and rejects
conflicting request/environment values.

The result keeps the legacy summary fields and adds exhaustive `emitted`,
`failed`, `unsupported`, and `warnings` arrays. A child or hole effect realized
by a native bolt array carries the bolt array GUID and `realizedBy`, and is not
inserted as a duplicate physical object. Grid axes and levels use the same
parent-realization receipt shape and do not inflate the native object count.

A canonical `xsection.shape:"double-angle"` is the one record kind that becomes
**two** native parts, because Tekla has no native 2L profile. Its row carries
`nativeGuids` (both parts, `a` then `b`) alongside the usual singular
`nativeGuid` (leg `a`), a `legs` array giving each leg's GUID, axis order,
offset and native rotation, and `legProfile` — the derived single-angle profile
actually built. `profile` keeps its usual meaning, the authored designation, and
a `tekla-double-angle-materialized-as-pair` warning names both. Such a member
does contribute 2 to `created`/`native`. The pair's geometry is proved against
the canonical section before commit, so a catalog whose parametric `L` is seated
differently fails the bake rather than committing a wrong pair, and its
`xsection` envelope must agree with the authored `section` exactly as the IFC
and Rhino sinks require. Because one record owns two parts, a bolt, weld or
boolean-cut that names a double-angle member is refused rather than bound to one
leg. `tee` remains explicitly unsupported.

A canonical `xsection.shape:"angle"` is built **mirrored**, because Tekla's
parametric `L h*b*t` seats its vertical leg on +X where the canonical descriptor
puts it on -X. The sink reverses the member's axis — the only way to mirror a
section in Tekla — and carries the matching roll, so the finished part matches
what `viewer-3d`, the IFC sink and Rhino draw. Its row therefore carries
`reversedAxis`, saying that the native part's own from→to is the reverse of the
authored axis and so is what its `nativeRotation` and `nativeRotationOffset` are
measured against; `rot` keeps its usual meaning, the authored canonical roll.
The seating is proved before commit rather than assumed. Every angle's inserted
solid is checked for its heel being on the canonical side and for its bounding
box matching the descriptor's leg lengths — the second is what holds a catalog
profile, whose fillets rule out a vertex comparison but leave its envelope
nominal, so a descriptor cannot resolve to a differently sized catalog angle and
commit. One whose resolved profile is exactly the sharp-corner parametric `L`
the descriptor implies is additionally compared vertex for vertex against the
canonical section. A catalog seated or sized differently therefore fails the bake
instead of committing a wrong member. A wrong leg *thickness* on a catalog
profile stays out of reach, since `t` does not move the envelope.

Its `xsection` envelope must agree with the authored `section` exactly as the IFC
and Rhino sinks require, and `section.w` and `section.d` must both be present and
numeric for that comparison to mean anything — a half-written `section` is
refused rather than quietly skipping the check. A present but malformed `angle`
descriptor (a non-positive dimension, or a `t` that leaves no leg) likewise fails
its record as `invalid-geometry` rather than baking the authored profile
unchecked, exactly as `double-angle` does. Single angles baked before
this change were built unmirrored, so an existing bake must be re-run to pick up
the correct hand.
