# Plan Review Log — full clip parity for AWARE's viewer-3d

Started 2026-07-20. MAX_ROUNDS=5.

Plan under review: `2026-07-20-viewer-3d-clip-parity-plan.md`.
Builder: Claude (Opus 4.8). Adversarial critic: Codex (read-only, session persists across rounds).

## Round 1 — Codex

The plan is not sound enough to implement. The broad premise is verified—the source is native Z-up, while destination element geometry is normalized to rendered Y-up—but several proposed ports violate that invariant.

1. **`snapPoint` is not coordinate-agnostic.** Section 4 says to port it “as-is,” but `candidatePoint` hardcodes Z as elevation for `vertical-axis`, `grid-int`, and `grid-line` candidates in [steel-3d-core.js](D:/Repos/floless.app-worktrees/scene-legend-descriptor-01f55d/web/steel-3d-core.js:309); in Y-up, it must preserve index 1 and snap in X/Z.
   Fix — Parameterize vertical/plan axes or implement a rendered-Y-up `candidatePoint` where vertical-axis/grid snaps preserve `dragged[1]`.

2. **`elevationMm via conv` is undefined for a scalar and broken for `meta.up === 'y'`.** The grid specification defines `elevationMm` as absolute model-space Z, while the destination grid renderer embeds it as `[x,y,elevation]` before `conv`; with Y-up input, `conv` is identity and the “level” is not horizontal.
   Fix — Either reject structural reference systems unless `meta.up === 'z'`, or redefine and update grid rendering and snapping together through one explicit reference-system-to-world transform.

3. **`sceneBox.min.y` is not the bottom of the rendered model.** Destination `sceneBox` includes reference systems and mostly element centerlines, whereas the source builds its box with `expandByObject` over actual meshes; using it can cut off the lower half of the lowest member or place the floor at a remote datum.
   Fix — Compute the draw floor from `meshBox(pickable).min.y`, with an explicit empty-model fallback.

4. **Grid candidate construction is materially underspecified.** `offsetMm` alone is insufficient: candidates require reference-system origin, bounds, direction semantics, optional `startMm`/`endMm`, conversion to X/Z plan space, segment clipping, and only valid cross-direction intersections.
   Fix — Extract one `referenceSystemSegments(R, up)` helper shared by rendering and snapping so displayed and snappable grids cannot diverge.

5. **Element candidates omit supported geometry forms.** The viewer already uses `axisEnds(e)` because rods and fasteners may store endpoints under `axis.from/to`; raw `element.from/to` misses them, as well as node-like `at` records.
   Fix — Build candidates from the viewer’s existing `axisEnds`/point-kind normalization instead of directly reading only `from` and `to`.

6. **Plane placement parity is wrong or unresolved.** The source explicitly disarms after one successful plane placement and remains armed only on a miss; the current viewer stays armed, and the plan never changes that despite auto-selecting the new gizmo.
   Fix — On a successful plane commit, call `setClipMode(null)`; retain the mode only after a raycast miss.

7. **The pointer priority is not defined as one state machine.** Merely “intercepting before `boxStart`” leaves armed clip placement, passive handle dragging, rubber-band selection, and click selection able to observe the same gesture; the source routes armed clip modes before handles.
   Fix — Replace the independent handlers with one gesture owner enum ordered `armed clip placement → clip handle → selection`, with exactly one terminal path.

8. **Dragging can strand state off-canvas.** Destination move/up handlers are canvas-only, and the plan adds no capture, `pointercancel`, or lost-capture cleanup; leaving the canvas can leave `controls.enabled=false`, a visible rubber band, or a pending drag.
   Fix — Capture the pointer on accepted `pointerdown` and funnel `pointerup`, `pointercancel`, and `lostpointercapture` through one cleanup routine.

9. **Clip placement can raycast geometry that is visually clipped away.** Three.js `Raycaster` does not honor renderer clipping planes, so the preview and commit can appear in empty space behind an active clip.
   Fix — Filter ray intersections against all active clip/work-area planes before selecting the first valid face.

10. **The second-corner check admits zero-width footprints.** The source’s `dx < 1 && dy < 1` wart accepts a line when only one plan extent is zero; the third click then disarms and silently adds nothing.
    Fix — Require both X and Z footprint extents to be at least 1 mm, rejecting the second corner when either extent is too small.

11. **Axial views produce invalid height behavior.** When the vertical axis projects below 4 px, skipping snapping does not make height meaningful: top view collapses to 1 mm, while bottom view’s parallel fallback can produce roughly the camera-to-floor distance.
    Fix — Mark height unavailable in axial views, show a change-view hint, and refuse the third-click commit until the projected vertical axis is usable.

12. **Level snapping can violate the floor clamp.** Raw height is clamped above the floor before snapping, but an accepted level below the floor can subsequently create a downward box.
    Fix — Filter level candidates to `worldY >= floorY + 1` or clamp again after snapping before preview and commit.

13. **The proposed object churn is a performance cliff.** Every drag move rebuilds and disposes the entire multi-handle gizmo, while every hover recreates preview geometries/materials and snapping scans/projects every candidate.
    Fix — Keep persistent gizmo/preview objects, mutate transforms and buffers in place, and coalesce hover/drag work to one `requestAnimationFrame`.

14. **Rewriting stems with `geometry.setFromPoints` every frame creates continuous attribute allocation.** At the pinned Three.js version this replaces position data repeatedly and adds needless CPU/GPU churn.
    Fix — Allocate a two-vertex `BufferAttribute` once, mutate its six floats, and set `needsUpdate=true`.

15. **`pxToWorldAt` should not copy the source’s perspective math.** The source uses Euclidean camera distance, but projected size depends on camera-space depth, causing off-axis handles to be oversized and mismatched with the 16 px picker.
    Fix — Compute perspective scaling from `abs(worldPoint.applyMatrix4(camera.matrixWorldInverse).z)`.

16. **The snap sprite may still be occluded in `overlayScene`.** The overlay pass retains the main depth buffer, and the plan does not set the sprite’s `depthTest`/`depthWrite`; merely moving it to the overlay scene is insufficient.
    Fix — Set the reticle material to `depthTest=false`, `depthWrite=false`, with an explicit overlay render order.

17. **The proposed list interaction remains inaccessible and unnecessarily complicated.** A double-click-only rename has no keyboard/touch equivalent, delayed selection feels laggy, and the plan does not require accessible names or selected/checked state on icon-only buttons.
    Fix — Make selection immediate, add a separate Rename button, and expose `aria-label`, `aria-pressed`/`aria-checked`, and labeled inline-edit errors.

18. **“Above `#legend`” has no robust layout mechanism.** Both panels are independently absolute-positioned while the legend has dynamic height, so they can overlap or jointly overflow a short viewport.
    Fix — Put clips and legend in one bounded bottom-left flex stack with a shared maximum height and independently scrollable bodies.

19. **Keybindings are incomplete.** The plan promises a draw key without choosing one and introduces `deleteSelectedClips` without wiring Delete/Backspace, although the source editor does.
    Fix — Specify and test a collision-free draw shortcut such as Shift+D plus guarded Delete/Backspace handling outside editable controls.

20. **The verification strategy mostly proves strings exist, not that behavior works.** Contains-assertions can pass with the Z-up `candidatePoint`, broken pointer cleanup, inverted snapping, or an unreachable handler; the browser E2E is described but not made an automated gate.
    Fix — Add an executable browser suite covering Z-up and Y-up scenes, one-axis-degenerate footprints, top/bottom views, clipped-face raycasts, off-canvas cancellation, exact box bounds, and keyboard list operation.

21. **The proposed probes are too vague for the claimed E2E assertions.** A source-style `getClips()` exposes rounded sizes, which cannot prove exact snapped bounds or which face moved.
    Fix — Define a read-only probe DTO containing exact box `min/max`, plane normal/point, enabled state, selected IDs, and current gesture state.

VERDICT: REVISE
### Claude's response — Round 1

Verified the four load-bearing findings against both codebases before accepting anything, rather than
caving on assertion alone. All four hold, and one is worse than reported:

- **#1 confirmed.** `candidatePoint` (steel-3d-core.js:309-313) hardcodes index 2 as the elevation to
  preserve for `vertical-axis`/`grid-int`/`grid-line`. "Ports as-is" was wrong. Now parameterized on
  the vertical index (§0 item 4).
- **#6 confirmed.** floless does `if (addClipPlaneAtScreen(...)) setClipMode(null)` — one plane per
  command. The viewer stays armed *and its comment claims "parity with floless"*, which is false. The
  plan now adopts the source behaviour and deletes the incorrect comment.
- **#3 confirmed, and worse.** `expandSceneBounds` (:853) folds in `referenceSystems` bounds AND every
  level elevation, so `sceneBox.min.y` is the lowest **grid datum**, not the model floor. Now
  `meshBox(pickable).min.y` with an explicit fallback chain (§0.2).
- **#9 confirmed.** `THREE.Raycaster` ignores `renderer.clippingPlanes`; floless filters hits through
  `isPointClipped`. v1 omitted that function entirely. Now ported (§2).

**Accepted all 21.** Went further than the proposed fix in three places:

- **#2** — took the "one explicit transform" option over "reject non-z-up scenes" (rejecting would
  break working scenes). Specifying it surfaced a **pre-existing AWARE defect**: the grid renderer
  builds a level as `[x,y,elevationMm]` then applies `conv`, so on a `meta.up==='y'` scene the
  elevation lands in Z and levels are not horizontal. Grids already render wrong there today. Decided
  to fix it in this PR through the same shared transform, and to say so in the PR body rather than
  smuggle it in.
- **#10** — floless's `|dx|<1 && |dy|<1` is an AND, so a straight line passes corner 2 and the commit
  then silently drops it. Requiring both extents independently is stricter than the source, on purpose.
- **#17** — dropped floless's 200 ms deferred-click hack entirely rather than making it accessible.
  Immediate selection plus a dedicated Rename button removes the lag and the mouse-only trap in one
  move.

**Nothing rejected outright.** The closest to a divergence is #11: adopted "refuse the commit in an
axial view" but paired it with an explicit readout hint, so the refusal is explained rather than
appearing as a dead click.

Round 1 was high-value: four verified defects in the plan's core assumptions, one pre-existing bug in
the destination, and a verification strategy that would have proved strings exist rather than that the
port works.

## Round 2 — Codex

Most round-one issues are resolved, but v2 still has several material ambiguities.

1. **The Y-up reference-system fix still lacks an actual coordinate mapping.** The scene contract and IFC writer treat grids as canonical XY with absolute Z elevation regardless of `meta.up`; accepting `up` in `referenceSystemSegments(R, up)` leaves implementers choosing incompatible interpretations for Y-up scenes.
   Fix — Formally define reference systems as canonical Z-up and map `(gridX, elevation, gridY) → world (X,Y,Z)`, or reject the unsupported `meta.up='y'` plus structural-grid combination.

2. **The “shared” transform excludes a consumer that already contains the defect.** `expandSceneBounds` still builds grid points independently, so Y-up grid rendering may be fixed while Fit, `maxDim`, and fallback bounds remain wrong; returning only `{label,y}` for levels also forces the renderer to reconstruct level crosshairs separately.
   Fix — Have the helper emit complete axis and level segments and use those exact results in rendering, bounds expansion, labels, and snapping.

3. **The new floor remains visibility-dependent.** Existing `meshBox` ignores `!m.visible`, so hiding or isolating the lowest member—or whole-parts work-area filtering—moves the draw floor; if everything is hidden it falls back to the grid-inflated `sceneBox`.
   Fix — Add an all-model mesh-bounds helper for the draw floor that expands every pickable mesh regardless of current visibility.

4. **Unit 2 contradicts the centralized gesture rewrite.** It says the new hover listener must coexist with the existing `boxStart` pointermove, while §3.1 says those independent handlers are replaced; retaining both recreates the arbitration problem.
   Fix — Route hover preview and rubber-band updates through the unified pointermove handler and remove the old `boxStart` listeners entirely.

5. **Cancellation semantics are still undefined.** A clip handle mutates live, but `pointercancel` and unexpected capture loss merely enter `endGesture`; whether they commit or restore `prePoint`/`preBox` is unspecified, and normal `releasePointerCapture` can trigger a second `lostpointercapture`.
   Fix — Make cleanup idempotent by clearing ownership first, with `pointerup` committing and `pointercancel`/unexpected capture loss reverting live mutations.

6. **The fixed readout can remain stuck on the last drag value.** Unlike the source’s floating readout, destination `#readout` is always visible, but no pointerup, cancel, or Esc path restores the current selection/idle message.
   Fix — Call one `refreshReadout()` from every gesture terminal path after committing or reverting.

7. **Clip-list selection regressed from the verified source behavior.** V2 says plain click always replaces, while the source clears selection when the clicked clip is already the sole selection; without that, hiding the gizmo requires a modifier.
   Fix — Restore plain-click toggle-off for an already solely selected clip, or explicitly make empty-canvas click clear clip selection.

8. **Candidate caching does not remove the principal large-scene cost.** Every hover frame still scans and projects every endpoint, midpoint, axis, and grid intersection; there is no scene-size limit, so RAF coalescing merely caps an unbounded O(N) operation at 60 times per second.
   Fix — Cache projected fixed candidates in a screen-space index invalidated on camera changes, and benchmark a representative large scene with an explicit frame-time threshold.

9. **The “executable browser suite” is still not executable from this plan.** The repository has no viewer browser-test harness or dependency, the HTML imports Three.js from a CDN, no command or fixture-generation path is specified, and the listed local gates omit the supposed real gate.
   Fix — Name the harness, files, browser installation/CDN strategy, fixture generator, and exact command, then include that command in the mandatory local gates.

10. **The source attribution for clipped plane hits is inaccurate.** FloLess’s general `meshHitsAt` uses `isPointClipped`, but its `clipPlaneAtScreen` directly takes the first raw raycast hit; the proposed destination filtering is a valid improvement, not behavior currently provided by that source path.
    Fix — Describe it as an intentional destination fix and ensure the browser test proves preview and commit both skip a clipped-away nearer hit.

11. **The vertical-plane basis guard still has only a string-presence guard.** The browser matrix covers Z-up/Y-up scenes but does not explicitly exercise normals parallel and perpendicular to +Y, so the originally identified silent vertical-cut regression can still ship.
    Fix — Add a behavioral test asserting patch corners are coplanar, finite, and correctly sized for both near-vertical and near-horizontal normals.

VERDICT: REVISE
### Claude's response — Round 2

Verified the two findings that change the design:

- **#1 confirmed, and it corrects my §0.1 fix.** `addReferenceSystems` (:864-870) builds grid geometry
  in canonical plan space `[planX, planY, elevation]` and *then* applies `conv(P,up)`. With `up==='z'`
  that yields `(x, elev, y)` — correct. With `up==='y'` `conv` is the identity, so elevation lands in
  world Z and the plan axes include world-up: **levels render as vertical planes**. Reference systems
  are therefore canonically plan-XY-plus-elevation regardless of `meta.up`, and passing `up` to them was
  itself the bug. v3 maps `(gx,gy,e) → (gx,e,gy)` unconditionally. Took the transform over rejecting
  y-up scenes: rejecting would break working scenes to dodge a one-line mapping error.
- **#10 confirmed — my attribution was wrong.** floless's `meshHitsAt` filters with `isPointClipped`,
  but its `clipPlaneAtScreen` takes the first raw hit, so clipped-face filtering does **not** exist
  upstream. v3 describes it as a deliberate destination improvement and requires the browser test to
  prove preview and commit both skip a clipped-away nearer hit.

**Accepted all 11.** Two deserve calling out:

- **#2** — extended the shared helper's remit beyond what was asked: it now emits complete axis *and*
  level segments consumed by rendering, `expandSceneBounds`, labels and snapping. `expandSceneBounds`
  carries the same y-up defect, so fixing only the renderer would have left Fit and `maxDim` wrong.
- **#5** — the idempotency point is subtle and correct: `releasePointerCapture` itself fires
  `lostpointercapture`, so a naive revert-on-capture-loss handler would revert a drag it had just
  committed. v3 makes `endGesture()` clear ownership first.

**Two deliberate scope-downs, stated rather than hidden:**

- **#8** — took the projected-candidate cache invalidated on camera change (real, cheap, removes the
  per-hover-frame projection cost). Did **not** build a benchmarking framework with a frame-time
  threshold; that is a harness this repo has no precedent for, and the cache removes the cost the
  finding names.
- **#9** — the strongest remaining point, and accepted as a correction to an over-promise. v2 called for
  an "executable browser suite" and listed it as a gate while the repo has no harness, no dependency, a
  CDN import, and **no CI for Rust at all**. Rather than invent a CI story, v3 names the harness,
  fixtures, generator command and file locations concretely, and then states plainly that it is an
  **agent-driven pre-PR gate, not a CI gate** — with adding Rust CI flagged as separate work. An honest
  manual gate beats a promised automated one that will not exist.

Round 2's value was mostly in ambiguity that would have become divergent implementations, plus one more
verified defect in the destination and one mis-attribution in my own plan.

## Round 3 — Codex

The two scope-downs are defensible: a benchmark framework is unnecessary given the concrete projection cache, and an agent-driven pre-PR browser gate is reasonable without Rust CI. The remaining problems are about whether that gate tests the current implementation and can observe its assertions.

1. **The browser fixture command can test a stale binary or fail because the agent is not installed.** Existing invocation tests first install the local agent into a temporary `AWARE_HOME`; plain `aware agent invoke viewer-3d...` may use a globally installed CLI and agent rather than the worktree changes.
   Fix — Have the driver use `cargo run --manifest-path cli/Cargo.toml`, install `20-agents/_core/viewer-3d` into a temporary `AWARE_HOME`, then invoke it with `{scene, output-path}` inputs.

2. **The planned probe surface cannot support the listed browser cases.** Exact clip bounds and gesture state do not reveal plane-patch corners, grid/level world segments, snap-reticle state, height usability, world-to-screen coordinates, or projected handle locations; therefore several claimed assertions are impossible without reaching into module-private state.
   Fix — Add read-only probes for reference-system segments, patch corners, world-to-screen projection, reticle/height state, and `clipHandlesScreen()` alongside the exact clip DTO.

3. **Projection-cache invalidation is underspecified and can make snapping geometrically wrong after resize or projection changes.** OrbitControls camera changes are not the only invalidators: `resize` changes the projection matrix, `setProjection` swaps camera objects, and orthographic reframing changes frustum/zoom.
   Fix — Centralize `invalidateClipProjectionCache()` and call it from OrbitControls `change`, resize, projection switching, framing/view presets, and orthographic reframing.

4. **V3 dropped the previously accepted RAF coalescing requirement.** Persistent objects remove allocation churn, but raw pointer events can still perform repeated raycasts, clipping tests, clip-plane rebuilds, and renderer-array updates faster than the display can render.
   Fix — Store the latest pointer event and process plane preview, box preview/snapping, and live handle mutation at most once per `requestAnimationFrame`.

5. **Persistent gizmos still need an explicit disposal path.** “Build once per selection change” means selection changes continue creating geometries and materials; without the source’s `clearClipGizmo` discipline, repeated selection leaks GPU resources.
   Fix — Specify recursive, array-safe geometry/material disposal whenever a gizmo is replaced or permanently cleared, while previews remain allocated and merely hidden.

6. **The pre-PR gate still lacks one canonical runnable entry point.** Naming Playwright and a directory is not enough for an agent to know which driver/runtime to execute or to distinguish a completed gate from ad hoc browser interaction.
   Fix — Add a single documented command or script under `cli/tests/browser/` that performs setup, serving, Playwright execution, cleanup, and returns nonzero on any failed assertion.

VERDICT: REVISE
### Claude's response — Round 3

Scope-downs from round 2 accepted as defensible. All 6 remaining findings accepted; none contentious —
they are the difference between a gate that looks thorough and one that actually observes what it
claims.

- **#1 is the sharpest, and I had already walked into it this session.** A plain
  `aware agent invoke viewer-3d render` resolves the GLOBALLY installed CLI and agent — earlier today I
  used exactly that against the released 0.97.0. As a gate it would report green while never executing
  the change under test. v4 has the driver `cargo run --manifest-path cli/Cargo.toml` and install the
  agent into a temporary `AWARE_HOME`.
- **#2** — my case list had outrun my probe list: patch-corner coplanarity, y-up level segments, height
  usability and per-face drags are all unobservable through clip bounds alone, so those assertions would
  have had to reach into private state. Probe surface now sized to the assertions.
- **#3** — correct that "camera change" is too narrow: `resize` changes the projection matrix,
  `setProjection` swaps the camera object outright, and `reframeOrtho` changes frustum/zoom with no
  camera move. A stale projection index does not just cost time, it snaps to the wrong place. Now one
  `invalidateClipProjectionCache()` with the full trigger list.
- **#4** — genuinely dropped when I rewrote v2 into v3. Restored as its own section: persistent objects
  fix allocation, not rate.
- **#5** — right that "build once per selection change" still leaks across repeated selection. Kept
  floless's `clearClipGizmo` disposal discipline, with session-persistent previews exempt.
- **#6** — `cli/tests/browser/run.mjs` as the single entry point that sets up, serves, runs, cleans up
  and exits nonzero.

## Round 4 — Codex

The six round-three findings are incorporated correctly, and both scope-downs remain defensible. Three material issues remain, two exposed by rechecking the upstream snapping construction.

1. **Member-member intersection snaps are missing.** Upstream `snapCandidates` explicitly computes pairwise beam-centerline intersections and emits `type:'intersection'`; v4 ports its precedence/glyph but never constructs this candidate type.
   Fix — Compute bounded segment intersections in rendered X/Z for nonvertical element axes, deduplicate them, place them at the draw floor Y, and include them in the projected candidate index.

2. **Height levels do not match the source.** Upstream `clipBoxLevels()` uses `elevationLevelsByPlan`, which derives levels from member endpoints; v4 uses only `referenceSystemSegments().levels`, so models without authored grid datums cannot snap box height to beam/column elevations.
   Fix — Build `clipLevels` from deduplicated world-Y values of normalized element-axis endpoints plus explicit reference-system levels, then apply the existing above-floor filter.

3. **RAF coalescing introduces an unresolved terminal-event race.** A final pointermove may still be queued when pointerup commits, or when Esc/cancel reverts; the callback can either be discarded—losing the final drag distance—or run afterward and mutate state after commit/revert.
   Fix — On pointerup synchronously flush the latest queued handle move before committing, while cancel/lost-capture/Esc cancel and discard the queued frame before reverting; guard callbacks with a gesture token.

4. **`run.mjs` still has no defined Playwright acquisition path.** The agent-driven gate is reasonable, but this repository has no Playwright dependency, so the promised single entry point can fail before running any assertion depending on the agent machine.
   Fix — Make `run.mjs` self-check and launch a pinned Playwright installation—either a declared local dev dependency or an explicit pinned `npx` path—and fail with a clear setup error if Chromium is unavailable.

VERDICT: REVISE
### Claude's response — Round 4

All four accepted. Two were real omissions I introduced myself by trimming the candidate table between
v2 and v3, and both were internally inconsistent in a way I should have caught:

- **#1** — I ported `PRECEDENCE` (which ranks `intersection` above centrelines) and the `x` reticle
  glyph for it, while never constructing the candidate. The table referred to a type that would never
  exist. Member-member axis intersections are now specified.
- **#2** — verified against the inventory: upstream `clipBoxLevels()` uses `elevationLevelsByPlan`,
  which derives elevations from *member endpoints*. I had used reference-system levels alone, which
  would leave every model without an authored structural grid — most of them — unable to snap box height
  to its own steel. Now the union of element-endpoint Ys and grid levels.
- **#3** — the race is real and I introduced it by adding RAF coalescing without terminal semantics. A
  queued frame could apply after commit or revert. Resolved explicitly: flush-then-commit on pointerup,
  cancel-and-discard on cancel/Esc, plus a gesture token so no frame can ever apply to an ended gesture.
- **#4** — fair: naming Playwright is not the same as being able to run it in a repo that declares no
  such dependency. Acquisition and a clear setup failure are now part of the entry point.

Findings are narrowing round over round (21 to 11 to 6 to 4) and this round produced no new class of
problem — only omissions within the design already agreed.

## Round 5 — Codex

No genuinely blocking issues remain. The four round-4 fixes are concrete and consistent with both codebases: intersection candidates match upstream behavior, height levels include member endpoints and grid datums, gesture tokens close the RAF terminal race, and Playwright acquisition makes the pre-PR gate runnable. The stated scope-downs remain defensible.

VERDICT: APPROVED
## Outcome

**APPROVED at round 5 of 5.** Findings per round: 21 -> 11 -> 6 -> 4 -> 0.

The argument caught four wrong assumptions in the plan's core (the elevation index in
`candidatePoint`, plane-placement parity, the draw floor, clipped-face raycasts), one mis-attribution
of behaviour to the source, two omissions I introduced while trimming, one self-inflicted race, and a
verification strategy that would have proved strings exist rather than that the port works. It also
surfaced a pre-existing defect in the destination: structural-grid levels render as vertical planes on
any `meta.up=='y'` scene.
