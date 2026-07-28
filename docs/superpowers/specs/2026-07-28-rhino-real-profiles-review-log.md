# Plan Review Log: Rhino real steel cross-sections

Started 2026-07-28 Europe/Warsaw. MAX_ROUNDS=5.

## Round 1 — Codex unavailable

`codex-cli 0.130.0` started thread
`019fa85b-2647-7d31-a3b3-df289be60262` but the configured
`gpt-5.6-sol` model rejected that client as too old. No verdict file was
produced. Per the review runbook this is not an approval and must not be
retried blindly; use independent code-review agents plus the live Rhino
end-to-end gate as the fallback.

## Round 2 — Codex

After upgrading Codex CLI from 0.130.0 to 0.145.0, review thread
`019fa85e-c3b8-7033-9d46-42798c229814` completed with `VERDICT: REVISE`.
The material findings were:

1. Define parity against the canonical FloLess/IFC contract, not the older
   approximate viewer and SketchUp implementations.
2. Resolve the contradiction between `xsection` and `section` dimension
   authority.
3. Specify strict malformed/legacy decoder behavior.
4. Validate derived clearances against document tolerance.
5. Construct near the origin and transform once, avoiding large-coordinate
   numerical fragility.
6. Prove hollow-loop retention, dimensions, area, volume, and orientation.
7. Prefer Rhino's direct `Extrusion` outer/inner profile API if it works.
8. Validate/normalize profiles before contacting Rhino.
9. Reject `BeginUndoRecord == 0` before mutation.
10. Revise the materialization hash/marker while retiring V1 objects.
11. Replace source-string-only tests with executable decoder tests.
12. Strengthen the live replacement/rollback/Undo gate.
13. Add geometry diagnostics and update stale public documentation.
14. Call the result a nominal sharp-corner profile, not an exact manufactured
    rolled section.

### Builder response

Accepted all correctness and safety findings. A live Rhino 8.31 prototype of
`Extrusion.SetPathAndUp` + `SetOuterProfile` + `AddInnerProfile` returned two
profiles, two caps, a valid outward solid, and the analytic RHS volume, so the
revised plan uses that simpler API. A second live probe reproduced the undo
defect (`outer_serial=22`, nested `inner_serial=0`); it is filed as #324 and
included in scope. The plan now defines exact schema behavior, dimension
authority, tolerance checks, local construction, V2 identity migration,
executable tests, stronger live invariants, diagnostics, and documentation.

Rejected only the suggestion to rework the generic viewer and SketchUp in the
same PR: the user asked to repair Rhino, and those consumers are explicitly
approximate. Their behavior no longer defines the parity claim.

## Round 3 — Codex

The revised plan remained `VERDICT: REVISE` with seven specific proof gaps:

1. Exercise all six profile families in live Rhino, not only RHS and W/I.
2. Force a failure after staging and prove staged rollback plus prior-batch
   preservation.
3. Read geometry back from the Rhino document and revalidate it before prior
   retirement; derive receipt diagnostics from that read-back.
4. Define each family's envelope mapping and a numeric millimetre tolerance.
5. Define dimensionally correct scale-aware length, area, and volume margins.
6. Check transform success and repeat all geometry invariants after transform,
   including at far world coordinates.
7. Prove the installed sidecar is the exact built binary by version/commit or
   hash.

### Builder response

Accepted all seven. The plan now pins the family-to-envelope mapping and its
host-neutral tolerance; gives dimensionally correct linear/area/volume
tolerance formulas; validates local, transformed, and document-read-back
geometry; exercises all six families both normally and far from the origin;
forces a post-stage retirement failure with a locked prior object; and gates
the live run on matching SHA-256 hashes plus assembly version/commit evidence.

## Round 4 — Codex

The next pass remained `VERDICT: REVISE` with six remaining gaps:

1. Correct canonical channels from `b` to `bf` and reject the wrong alias.
2. Undo and Redo the failed post-stage bake record, proving neither operation
   changes the restored prior GUID/ownership/geometry set.
3. Repeat live six-family coverage in a non-millimetre document.
4. Prove a document-tolerance-specific rejection happens before undo/mutation.
5. Define observable Brep cap/profile counts after document read-back.
6. Isolate and completely dispose the locked-object fault fixture.

### Builder response

Accepted all six. FloLess and IFC both define channel width as `bf`; the plan
now tests canonical acceptance and `b` rejection. It defines read-back caps as
the two axial planar Brep faces and profiles as their identical loop counts.
Disposable millimetre and metre documents prove unit conversion, a separate
coarse-tolerance document proves pre-undo rejection, and a disposable locked
source proves rollback plus harmless Undo/Redo without contaminating the
user's production model.

## Round 5 — Codex

`VERDICT: APPROVED`. The reviewer found no remaining material correctness,
geometry, ownership, rollback, testing, or operational gaps after the schema,
unit-system, document-read-back, and disposable fault-fixture revisions.

## Post-implementation adversarial review

Two independent code reviewers initially found three gaps:

1. repeated profiles reused the first document object's measured area instead
   of measuring every authoritative Rhino read-back;
2. FloLess could still retry a Rhino mutation after `lost response` or
   `connection reset`, even though dispatch made completion uncertain; and
3. the I-section tolerance residual used the two flange outstands combined
   instead of testing each individual outstand.

All three were corrected. The cache and its dead branch were removed, every
document Brep now owns an independent `VolumeMassProperties` check before prior
geometry is retired, every Rhino transport-loss class fails closed without an
automatic retry, and the I residual is `(bf - tw) / 2` with a focused test.
Both reviewers then returned no blocking findings. The final suites passed
83/83 Rhino tests and 16/16 FloLess adapter/route tests.
