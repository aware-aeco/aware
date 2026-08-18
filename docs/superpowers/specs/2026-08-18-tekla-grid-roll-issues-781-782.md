# Plan: preserve FloLess grids and member roll in Tekla
_Final candidate after Round 5 — awaiting human sign-off_

## Goal

Resolve floless.app issues #781 and #782 at their actual fault boundary, AWARE's Tekla `bake-scene` writer. A trimmed FloLess grid must arrive as the least-lossy native Tekla grid with an explicit warning instead of being omitted, and a physical member's canonical scene `rot` must produce the same section orientation as the already-world-oriented connection plates and welds.

## Delivery order

The issue runbook requires one issue and one PR at a time. The two changes also overlap the materializer, tests, and identity hash, so delivery is deliberately sequential:

1. reproduce and file the two AWARE issues;
2. branch #781 from current AWARE `main`, land materializer payload v2, review, merge, and update local `main`;
3. branch #782 from that updated `main`, land payload v3, review, and merge;
4. perform the final FloLess browser acceptance against the locally built merged bridge, then mark the still-open FloLess issues `qa-ready`.

No FloLess code change is expected: FloLess already emits the authored data and correctly identifies the old bridge's unsupported result.

## 1. Reproduce and attribute

- Run a source-owned scene carrying independent `startMm`/`endMm` through the unmodified live Tekla bridge and preserve the receipt showing the parent plus every axis/level classified unsupported.
- Run source-owned physical members with non-cardinal `rot` on upward, inverted, horizontal, and sloped axes. Read native `Position.Rotation`, `RotationOffset`, and the actual section coordinate frame to prove the current writer ignores roll and to determine Tekla's enum/sign mapping relative to the canonical scene convention.
- File two focused AWARE issues only after those reproductions, cross-referencing floless.app #781 and #782.

## 2. Grid fallback (#781, materializer payload v2)

### Representation

- Retain finite, increasing validation for every authored axis extent.
- Classify single-family grids as a Tekla-only unsupported family during preflight, before any native insertion. A native Tekla `Grid` requires both X and Y families; inventing an unreceipted perpendicular axis would violate source identity. Emit exhaustive parent/axis/level `unsupported` rows and continue materializing unrelated valid records.
- Treat unique offsets/elevations, whitespace-free label tokens, and finite derived spacings/envelopes/extensions as Tekla representability requirements, not canonical-scene validity. When one fails, classify the whole grid family as exhaustive `unsupported` rows without batch-aborting unrelated records. Reserve `failed`/atomic abort for canonically malformed records such as non-finite authored coordinates or non-increasing authored extents.
- Stop removing a structural grid and its children merely because axes have independent extents.
- Compute the tightest native envelope Tekla's one-`Grid` model can represent without truncating authored lines:
  - for X-family axes, include every authored X-axis `startMm`/`endMm` and the perpendicular Y coordinate span;
  - for Y-family axes, include every authored Y-axis `startMm`/`endMm` and the perpendicular X coordinate span;
  - derive `ExtensionLeftX`, `ExtensionRightX`, `ExtensionLeftY`, and `ExtensionRightY` from those envelopes, allowing extents outside the declared scene bounds;
  - preserve the existing vertical extent and magnetic setting.
- Exact grids stay silent. If independent authored extents require widening any native line, queue one parent warning candidate (`status:"warning"`, code `tekla-grid-axis-extents-expanded`) whose message reports the chosen family envelopes and explains that Tekla uses one shared envelope per family.

### Receipt and retirement safety

- Emit the warning only after a successful commit/read-back. A later failure must return no lossy-materialization warning for an object that was staged and retired.
- Resolve axes and levels exclusively through `realizedReferences`; do not alias child IDs in `nativeById`. The exhaustive receipt must preserve source order, give every child `realizedBy` equal to its parent ID, share the parent's native GUID, preserve the parent ownership UDA, and increment native object count once.
- Before retiring prior owned objects, read back CoordinateX/Y/Z, labels, Origin, all four planar extensions, and `IsMagnetic`; reconstruct both native family envelopes and compare them to the selected envelopes with a fixed 0.1 mm coordinate tolerance. A read-back mismatch aborts and cleans staging.
- Bump `ComputeBakeMaterializationHash` from payload v1 to v2 while leaving the `AWARE_BAKE_V1` ownership marker readable for safe retirement. Test that identical scene/version input produces a different hash across payload revisions.

### Test shape

- Put grid-envelope selection in a host-agnostic helper exposed to the canonical script through public Roslyn globals, so unit tests execute the exact production calculation rather than a copied fixture. Keep generic exec scripts source-compatible by only adding a globals member.
- Cover exact grids, heterogeneous shorter axes, extents outside bounds, exhaustive unsupported classification of all-X/all-Y families and other Tekla-only representability limits, axis order, canonically malformed extents, overflow, duplicate offsets/elevations, whitespace labels, continuation of unrelated records, warning suppression on later failure, read-back mismatch cleanup, exhaustive child receipt identity, and one native Grid insertion.
- Semantically compile the canonical script against locally installed Tekla 2025 and 2026 assemblies in addition to syntax/source guards. Add a mandatory local pre-merge command that fails when either supported install or compilation result is missing; record both results in the PR evidence.
- Update Tekla `bake-scene` documentation and bump the `tekla` parent manifest from 0.1.0 to 0.1.1 in this first PR.

## 3. Canonical member roll (#782, materializer payload v3)

### Contract first

- Update the canonical viewer scene schema before implementation. `rot` is an optional JSON number of finite degrees, positive by the right-hand rule about the directed `from→to` axis, measured from one deterministic zero-section frame shared by viewer, IFC, and Tekla, and applicable only to physical `member`, `line`, and `box` records.
- Tekla accepts only absent/`"z"` `meta.up` during preflight; reject `"y"` and unknown values before mutation instead of silently interpreting them in a Z-up host. Add cross-sink golden vectors against the IFC zero-frame algorithm.
- Validate `meta.up` as exactly `"z"` or `"y"` in the canonical viewer. Viewer supports both. IFC and Tekla explicitly reject Y-up before output/mutation until a reviewed Y→Z export transform exists; neither may silently treat unknown values as Y-up or Z-up.
- Follow the AWARE skill-authoring workflow. Implement and validate the same `rot` contract in `viewer-3d` (including the reflective Z-up screen conversion) and IFC, whose current fallback silently coerces malformed values. Update the canonical schema plus IFC and Tekla command descriptions; bump `viewer-3d` 0.1.0→0.1.1, `ifc` 0.1.0→0.1.1, and `tekla` 0.1.1→0.1.2. Add no skill-level version metadata.

### Conversion and validation

- Require the raw value to be a JSON numeric type; reject null, strings, NaN/infinity, and other types.
- Normalize in all three consumers with the exact algorithm `r = ((degrees % 360) + 360) % 360; if (r >= 180) r -= 360; if (r == 0) r = +0`, yielding `[-180,180)` and canonicalizing negative zero. Let declared scene-up `u` be `+Z` for absent/Z-up and `+Y` for Y-up. For normalized member axis `n`, if `1-(n·u)² <= 1e-6`, zero-frame X is normalized scene `+X` projected perpendicular to `n` and Y=`n×X`; otherwise zero-frame Y is normalized `u` projected perpendicular to `n`, and X=`Y×n`. Rotate that frame by right-hand-rule `rot` about directed `from→to`. Test `-540,-360,-180,-0,+0,+180,+360,+540`, axis vectors immediately below/on/above the inclusive threshold, and a Y-up scene against its equivalently transformed Z-up scene.
- Let the live probe establish a universal Tekla rotation enum/sign conversion from exact native profile geometry. Probe W/I, channel, unequal-leg angle, RHS/HSS, rectangular/legacy-rectangle, and CHS representatives on upward, inverted, horizontal, and sloped axes. Verify reselected B-rep/DSTV face or edge geometry; use `GetCoordinateSystem()` only if the probe first proves it changes with profile roll. Compare geometry modulo its symmetry group: I/W and non-square rectangle/RHS modulo 180 degrees, square RHS modulo 90 degrees, CHS continuously invariant. Use channel and unequal-leg angle geometry to prove full-angle sign/handedness; use enum/offset read-back to prove the canonical scalar for every family. If one universal mapping does not hold, stop and revise the contract/plan rather than infer families from arbitrary profile strings or call unsafe `CatalogHandler` APIs.
- Centralize the proven conversion in a host-agnostic helper exposed through Roslyn globals; the canonical script calls the same helper unit tests execute.
- Apply roll exactly once. Preserve the directed scene axis so reversing `from`/`to` obeys the canonical right-hand rule.

### Read-back and tests

- Before retirement, read back the native rotation enum and normalized offset and verify section orientation from the same proven B-rep/DSTV observable, not only the scalar property. Reuse fixed acceptance constants everywhere: 0.1 mm for reconstructed coordinates/B-rep points, 0.01 degrees for modulo-angle comparison, and normalized-vector dot product at least `cos(0.01 degrees)` with positive handedness.
- Apply identical type/finite/normalization/rejection rules in viewer and IFC. Cover absent, zero, negative, +360/-360, non-cardinal, huge finite, null, string, and non-finite values; Z-up reflection; upward, inverted, horizontal, and sloped axes; axis reversal; non-member exclusions; every canonical `xsection.shape`, absent `xsection`, and read-back/failure cleanup with cross-sink golden vectors.
- Extend `cli/tests/browser/run.mjs` with Z/Y-up, axis reversal, near-vertical threshold, boundary-angle, and non-cardinal fixtures. Inspect actual rendered world-space bases/vertices through `window.__viewer3d`, capture a screenshot, require a clean browser console, and record the mandatory browser-gate result in PR evidence.
- Semantically compile against Tekla 2025 and 2026 with the mandatory non-skippable local gate. Add a non-cardinal live fixture (82.7 degrees), representative asymmetric profile families, and the complete axis-orientation matrix used by the probe.
- Bump materializer payload v2 to v3, again preserving `AWARE_BAKE_V1` retirement compatibility and testing the hash transition.

## 4. Verification for each PR

- `dotnet test cli-tekla/Tests/AwareTekla.Tests.csproj`.
- `dotnet format`/the repository's C# formatting gate if present.
- From `cli/`: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`.
- Every live/browser mutation targets a disposable test model only. Add optional production-compatible `expectedModelPath` to the same `bake-scene` invocation plus `AWARE_TEKLA_EXPECT_MODEL_PATH` and `AWARE_TEKLA_QA_GUARD=1` environment controls. Outside QA mode, omission preserves existing callers; any supplied expectation is enforced. In QA mode, a non-empty expectation is mandatory, conflicting request/environment paths fail, and omission fails. Forward the single resolved path into Roslyn globals and compare its canonical full path case-insensitively to `Model.GetInfo().ModelPath` inside the canonical script immediately before work-plane change or insertion. A mismatch fails before mutation, closing the separate-probe TOCTOU window. Preserve a recoverable pre-test snapshot, route to an explicit PID/version with no same-major sibling, and retire all test-owned objects afterward. Use a source ID unique per scenario but stable across that scenario's replacement sequence.
- Live Tekla 2025 and 2026 where installed and available. Both local installs must pass the non-skippable semantic-compilation gate; if only one running host can be exercised, report the unavailable live branch explicitly.
- #781 live assertions: zero grid unsupported rows, exactly one post-commit warning for the lossy fixture, exhaustive emitted rows with parent/child identity, one native Grid, and numerically reconstructed shared envelopes containing every authored axis.
- #782 live assertions: stored rotation plus actual section frame matches the canonical right-handed roll for the full axis/profile matrix. For the base-plate fixture, use the detailing tolerance declared by the fixture (default 0.1 mm): bearing-plane signed distance must be within tolerance, weld paths must lie on their intended joint within tolerance, and only unexpected penetrations are failures. Explicitly allow authored bolt-through-ply, weld-to-part, bearing-contact, and Boolean-cut intersections.
- Exercise retirement compatibility live with retained bridge binaries from the exact baseline and PR1 merge commits: seed the same scenario/source using v1 and replace it using v2, then seed using v2 and replace using v3. Assert old GUID retirement, no duplicates, new ownership/hash, and survival of the prior set when an injected failure occurs before retirement.

## 5. FloLess acceptance and delivery

- Add a Windows CI job that builds/tests `cli-tekla` on every PR and `main` push and uploads a commit-SHA-addressed bridge zip. Proprietary 2025/2026 semantic compilation remains the mandatory local pre-merge gate; the CI artifact makes the exact merged bridge reproducible for QA without a release.
- Build/use that exact merged-commit bridge and drive FloLess Exports -> Send to a CAD app through real browser clicks, never simulation, in the disposable verified model. Assert visible receipt/output, DOM state, screenshot, clean console, and post-test cleanup. Link the artifact plus install/model/host instructions in each FloLess issue before applying `qa-ready`.
- Run final Codex review on each AWARE branch, address findings, push, open a PR, wait for green CI, and merge only after final-commit review.
- Confirm both AWARE PRs are `MERGED`; comment on floless.app #781/#782 with upstream issues/PRs and verification evidence; apply `qa-ready`; leave them open for human QA. Do not release AWARE or FloLess.

## Key decisions

- The defect belongs in AWARE. FloLess is preserving authored scene data; only the target writer can own native representational loss and orientation conversion.
- A single native Grid with the tightest non-truncating per-family envelope preserves registration and source identity. Individual GridPlanes would change the parent/child native identity and retirement model and are intentionally out of scope.
- Materializer payload versions are semantic cache/identity versions; the stable ownership prefix is not changed because old objects must remain discoverable and safely retireable.
- Existing FloLess tolerance of the old unsupported-grid receipt remains for compatibility with installed older AWARE versions.

## Out of scope

- Exact independent per-axis extents in a single native Tekla Grid.
- Replacing one structural grid with one GridPlane object per axis.
- Composing or correcting scene geometry in FloLess.
- Any tag, package publication, or release.
