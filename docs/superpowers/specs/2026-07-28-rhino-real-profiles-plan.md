# Plan: Materialize nominal steel cross-sections in Rhino
_Round 1 — revised after adversarial review_

## Goal

Make `rhino-8/bake-scene` consume the canonical FloLess `element.xsection`
descriptor so Rhino receives nominal sharp-corner I, channel, angle, RHS, CHS,
and rectangular member solids instead of bounding boxes, without weakening
bake ownership, rollback, or live-document safety.

The parity target is the FloLess/IFC discriminated `xsection` contract, not the
older approximate AWARE viewer or SketchUp implementation.

## Approach

1. Add a pure, host-neutral cross-section decoder in `cli-rhino`.
   - Absent `xsection` is the only legacy rectangular fallback and uses
     `section.{w,d}`.
   - A present descriptor must be an object with the exact lowercase shape
     `i|channel|angle|rhs|chs|rect` and finite JSON-number dimensions.
   - Null, scalar, missing/unknown/wrong-case shape, string-number, Boolean,
     non-finite, zero, negative, or partial descriptors fail preflight.
     Unknown extra fields are ignored for forward-compatible metadata.
   - Reconcile the envelope with
     `max(0.000001 mm, 1e-9 * max(1 mm, |actual|, |expected|))` tolerance:
     `i.{bf,d}`, `channel.{bf,d}`, `angle.{b,d}`, `rhs.{b,d}`, and
     `rect.{w,d}` map to `section.{w,d}`; `chs.od` must independently match
     both `section.w` and `section.d`.
     A channel `b` alias is rejected; only canonical `bf` is accepted.
   - Validate analytic topology constraints (`2tf<d`, `tw<w`, `2t<min(b,d)`
     for hollow sections, and the corresponding angle/channel clearances).
2. Put the normalized profile plan on each host-neutral `supported` row.
   Dispatch happens only after that validation succeeds. The embedded Python
   consumes that normalized plan and repeats live document-tolerance checks;
   it does not parse designations or invent thicknesses.
3. Build every profile near WorldXY with Rhino's direct `Extrusion` API:
   `SetPathAndUp`, `SetOuterProfile`, and `AddInnerProfile`.
   - Polygon curves model I/channel/angle/RHS/rect.
   - True circles model CHS.
   - `AddInnerProfile` explicitly assigns RHS/CHS voids, avoiding planar-region
     loop inference and Boolean subtraction.
   - Extrude along local +Z, validate the local result, then apply one rigid
     WorldXY→member-frame transform.
4. Prove the materialized topology before mutation and again after transform:
   - expected profile and cap counts;
   - both caps present;
   - valid closed outward solid;
   - transform success, member endpoints, local-frame envelope, and member
     length within a scale-aware linear tolerance;
   - analytic cross-sectional area/volume within dimensionally correct
     scale-aware margins;
   - every derived residual edge or void exceeds document tolerance.
   Let `e = max(document absolute tolerance in mm,
   1e-9 * max(1 mm, expected span))`. For a profile with total outer-plus-inner
   perimeter `P`, expected area `A`, and member length `L`, use
   `areaTolerance = max(P*e + pi*e^2, A*1e-9)` and
   `volumeTolerance = max(A*e + L*areaTolerance + e*areaTolerance,
   A*L*1e-9)`. Compare endpoint and envelope coordinates in the member's
   orthonormal frame, not with a world-axis bounding box.
5. Harden the transaction boundary exposed during review:
   `BeginUndoRecord(...) == 0` is a structured pre-mutation failure. No layer
   or object-table write may occur without an owned undo record.
6. Revise materialization identity:
   - include a `rhino-profile-v2` geometry revision in the hash;
   - stamp new objects with a V2 ownership marker;
   - accept and retire valid V1 and V2 owned objects so upgrading replaces the
     rectangular batch rather than duplicating it.
7. Preserve existing roll/frame calculation, source ownership, staged
   replacement, rollback, and active-view warning behavior. After staging each
   object, read its Brep back from `RhinoDoc.Objects` and repeat the
   validity/orientation/endpoints/envelope/area/volume checks on that document
   object before retiring any prior batch. Receipt diagnostics are derived
   from this read-back geometry and report geometry revision, normalized
   shape/dimensions, profile/cap counts, and volume. On the read-back Brep,
   `capCount` is the number of planar faces whose normals are parallel to the
   member axis; it must be exactly two. `profileCount` is the loop count on
   each axial cap (one for solid sections, two for RHS/CHS), and both caps must
   have identical outer/inner loop topology.
8. Add executable, table-driven tests for the pure decoder, every family,
   schema conflicts, malformed values, tolerance/clearance boundaries, V1→V2
   ownership compatibility, hash revision, and zero-undo ordering. Source
   assertions remain only for live-Rhino-only safety ordering.
9. Update Rhino's manifest, command page, example, receipt schema, and shared
   scene-schema documentation to describe the discriminated union and its
   nominal sharp-corner accuracy.
10. Build and test `cli-rhino`, install the local sidecar, and prove the exact
    executable used: record the publish artifact SHA-256 plus assembly
    informational version/commit, copy it to the installed bridge path, and
    require the installed file SHA-256 to match before testing. Drive a
    six-member bridge fixture through live Rhino containing one
    I/channel/angle/RHS/CHS/rect member in disposable millimetre and metre
    Rhino documents, then drive the real FloLess UI to replace the 187-member
    rectangular batch in live Rhino.
    Verify:
    - each of the six fixture families has its expected outer/inner topology,
      dimensions, outward solidity, analytic area, and volume in both unit
      regimes, with physically equivalent results after conversion to mm;
    - all 187 objects are valid outward Breps;
    - W profiles have the expected 12-edge I outline and analytic volume;
    - the receipt reports V2/I geometry rather than `rect`;
    - a malformed host-neutral follow-up performs no write and preserves the
      valid V2 batch;
    - in a disposable source/document, locking one prior source-owned object
      forces retirement to fail after the replacement batch has staged, and
      rollback removes every staged object while preserving the complete prior
      GUID/ownership/geometry set; immediately Undo and Redo that failed bake
      record and require the same exact prior set after each operation;
    - a disposable coarse-tolerance document rejects a profile whose residual
      web/flange/void is smaller than document tolerance before
      `BeginUndoRecord`, preserving the prior set and undo state;
    - Undo/Redo restores the old/new source-owned set;
    - a far-from-origin six-family fixture repeats the post-transform
      endpoints/envelope/validity/orientation/volume assertions;
    - a Rhino-native viewport screenshot visibly shows flanges and webs.
    Dispose the temporary unit/tolerance/fault-injection documents and their
    locks, layers, objects, and undo state before the production-model gate;
    never run those destructive probes against the user's 187-member source.

## Key decisions & tradeoffs

- `xsection` owns every shape dimension when present; `section` is a required
  compatibility envelope and must agree, rather than silently overriding it.
- An absent descriptor stays rectangular for legacy scenes. A present but bad
  descriptor fails closed—wrong geometry is worse than a clear refusal.
- Direct `Extrusion` was live-prototyped in Rhino 8.31 with an RHS outer and
  inner profile: two profiles, two caps, valid outward solid, correct volume.
- These are nominal sharp-corner profiles. Rolled fillets and HSS corner radii
  are intentionally not claimed as manufactured-profile exactness.
- This remains an AWARE bridge change. FloLess already authors the canonical
  descriptor; duplicating section knowledge in its UI/server is unnecessary.

## Risks / open questions

- Rhino document tolerance varies with model units, so live checks happen
  after millimetre conversion even though schema validation happens earlier.
- `BeginUndoRecord == 0` is a verified current AWARE defect (#324) and is fixed
  in scope because this geometry change exercises the same mutation boundary.
- The installed AWARE release predates this change, so live verification needs
  a locally built sidecar until a later release.

## Out of scope

- Rolled fillets, corner radii, tapers, copes, holes, connection geometry, and
  material textures.
- Reworking SketchUp or the generic viewer in this PR.
- Cutting an AWARE or FloLess release.
