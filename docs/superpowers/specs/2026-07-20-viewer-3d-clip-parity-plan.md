# Plan v3 — full clip parity for AWARE's viewer-3d

_Rounds 1–2 incorporated. The log records what I verified, where I went beyond the proposed fix, and
the two places I scoped down deliberately._

**Goal.** The viewer's clip is today three commands: arm a plane pick, box-around-selection, clear all.
The floless steel editor's clip is an editable object model — draw a box by picking points with
snapping, drag its faces afterwards, enable/rename/delete each clip from a list. This brings the viewer
to functional parity.

**Not in scope.** Anything depending on the editor's document: undo integration (`api.beginClipEdit`),
contract persistence, per-plan/per-sheet scales, the running-snaps preference UI, and tool-exclusivity
disarms for editing tools the viewer does not have.

Source: `floless.app/web/steel-3d-view.js` §clip (1466–1801, 1623–1738), `web/steel-3d-core.js`
(`snapPoint`/`candidatePoint`/`PRECEDENCE`), clip rows in `web/steel-editor.html` (5215–5239,
6004–6027). Destination: `cli/src/render/viewer_3d.rs`.

---

## 0. Coordinate invariants

### 0.1 Elements: the rendered world is always Y-up

`conv(P,up)` (:292) maps scene `(x,y,z)` → world `(x,z,y)` when `meta.up==='z'`, identity when `'y'`;
`camera.up` is pinned `(0,1,0)` (:377). The clip subsystem works **entirely in rendered world space,
up = +Y**. Each of these is a decision point where copy-paste is a bug:

1. Box-height pull axis is world **+Y**.
2. `planePatchCorners`'s degenerate-basis guard tests the **up** component:
   `Math.abs(n.y) < 0.9 ? (0,1,0) : (1,0,0)`. Wrong only on a *vertical* cut — invisible in the common
   case, which is why §7 requires a behavioural test, not a string guard.
3. **`candidatePoint` is not coordinate-agnostic** (steel-3d-core.js:309–313): it hardcodes index 2 as
   the elevation to preserve for `vertical-axis`/`grid-int`/`grid-line` (*"grid steers PLAN only —
   never yanks the elevation"*). In a Y-up world the elevation is index **1**, plan axes are X/Z. Port
   with the vertical index parameterized. `snapPoint` and `PRECEDENCE` are genuinely axis-agnostic and
   port unchanged.

### 0.2 Reference systems are canonically plan-XY + elevation — `up` is NOT a switch for them

Verified at `addReferenceSystems` (:864–870): grid geometry is built in canonical grid space as
`[planX, planY, elevation]` and *then* passed through `conv(P,up)`.

- `up==='z'` → `(x, elev, y)`. Elevation becomes world-up. Correct.
- `up==='y'` → identity → `(x, y, elev)`. **Elevation lands in world Z and the plan axes include
  world-up: levels render as vertical planes.**

So the contract is "plan XY plus absolute elevation", independent of `meta.up`, and the correct mapping
is *always* the z-up one: `(gx, gy, e) → world (gx, e, gy)`. Passing `up` to reference-system geometry
was the bug.

**Pre-existing defect, fixed here.** Structural grids already render wrong on any `meta.up==='y'`
scene, today, independent of this work. `expandSceneBounds` (:853) carries the identical bug, so Fit,
`maxDim` and the fallback bounds are wrong there too. The one shared helper below fixes render, bounds,
labels and snapping together. **This will be called out explicitly in the PR body, not smuggled in.**

```
referenceSystemSegments(R) → { axes:[{label, direction, a:Vector3, b:Vector3}],
                               levels:[{label, y, segments:[[Vector3,Vector3],…]}] }
```

It consumes `origin`, `bounds`, `direction`, and the optional per-axis `startMm`/`endMm`, and emits
**complete world-space segments** — including the level crosshairs — so the renderer never reconstructs
geometry the snapper derived differently. Consumers: `addReferenceSystems`, `expandSceneBounds`, the
axis/level labels, and the snap-candidate builder.

### 0.3 The draw floor ignores visibility

`expandSceneBounds` folds in grid bounds and every level elevation, so `sceneBox.min.y` is the lowest
**grid datum**, not the model floor. But `meshBox` is also wrong for this purpose: it filters on
`m.visible`, so hiding or isolating the lowest member — or work-area whole-parts filtering — would
*move the floor under the user mid-session*.

Add an all-model bounds helper that expands **every** pickable mesh regardless of current visibility.
Fallback chain: all-mesh bounds → `sceneBox` → 0.

---

## 1. Unit 1 — clips become editable objects

A clip is today `{id, kind, planes}` and a box clip **discards its `Box3` at creation** (:1212): nothing
for a handle to grab. `applyClips` unconditionally flattens: nothing to disable.

```js
{ id, kind:'plane'|'box', enabled:true, label,
  n:Vector3, point:Vector3,   // SOURCE, plane only
  box:Box3,                   // SOURCE, box only
  planes:[Plane] }            // DERIVED — never edited directly
```

- `rebuildClipPlanes(c)` — the single derive step.
- `applyClips` gains `.filter(c => c.enabled)` and **must keep calling `syncClipMirror()`** — the shadow
  pass holds its own array reference; a stale one leaves the model visibly clipped after the clip is
  gone (trap at :1190–1193).
- **`boxToPlanes` signs are not to be touched** (:1177–1180 records they were verified live).
- Lifecycle: `toggleClip`, `renameClip` (rejects blank + case-insensitive duplicates), `removeClip`,
  `clearClips`, `setSelectedClips`/`selectClip`/`selectedClips`, `deleteSelectedClips`, `getClips`.
- `selectedClipIds:Set` drives gizmo visibility only — must **not** call `applyClips`.
- **Plane placement disarms on success.** floless: `if (addClipPlaneAtScreen(...)) setClipMode(null)` —
  one plane per command, armed only on a miss. The viewer stays armed *and its comment claims "parity
  with floless"*, which is false. Adopt the source behaviour; **delete the incorrect comment**.
- **Keep `addClipBox(pad)`** as a command distinct from the draw — live, shipped and tested here.

## 2. Unit 2 — hover ghost on an armed plane pick

- Factor the raycast into `clipPlaneAtScreen(cx,cy) → {n,point}|null` so ghost and commit **cannot
  disagree**.
- **Filter clipped hits — as a deliberate destination improvement, not a port.** `THREE.Raycaster`
  ignores `renderer.clippingPlanes`, so a pick can land on a face that is visually cut away. floless's
  general `meshHitsAt` filters via `isPointClipped`, but *its own `clipPlaneAtScreen` takes the first
  raw hit* — so this behaviour does not exist upstream and must not be described as ported. Port
  `isPointClipped` and apply it to the intersection list before taking the first face.
- `planePatchCorners` with the up-axis guard (§0.1).
- `setClipPlanePreview(hp,n)` — translucent fill (`renderOrder:995`) + `LineLoop` outline (`:996`), both
  `depthTest:false`, in **`overlayScene`**, allocated **once** and re-oriented per hover (not rebuilt).
- `r = CLIP_PLANE_PATCH_R` = 304.8 mm; ghost and placed outline share size and position.

## 3. Unit 3 — gizmo and handle dragging

Same manipulator for a plane and each box face: a translucent disc in the plane/face plus a normal
arrow (stem + cone); grab either and slide along the normal. Built in **`overlayScene`** — handles in
`scene` would be clipped by the planes they edit.

- Box: `Box3Helper` + 6 handles over `FACE_AXES`, at face centres, outward normals. Plane: `LineLoop`
  patch at `c.point` + one handle along `c.n`. Per handle: `disc` (`:998`), `stem` (**not** flagged
  `clipHandle`, so unpickable), `cone` (`:999`), all `depthTest:false`.
- **Persistent, mutated in place.** floless tears down and rebuilds the whole gizmo group on *every drag
  move frame* and rewrites stems with `geometry.setFromPoints` per frame. Build once per selection
  change and update transforms; give the stem a two-vertex `BufferAttribute` allocated once, mutate its
  six floats, set `needsUpdate`.
  - **Persistent still needs disposal.** "Build once per *selection change*" means a user cycling
    selections keeps allocating geometries and materials. Keep floless's `clearClipGizmo` discipline —
    recursive, array-safe geometry + material disposal whenever a gizmo is replaced or cleared.
    Previews (ghost patch, box helper, reticle) are allocated once for the session and merely hidden,
    so they are exempt.
- `sizeClipHandles()` per frame: disc 14 px, arrow 11 px at a 34 px offset along the normal.
  - **`pxToWorldAt` uses camera-space depth**, `Math.abs(p.applyMatrix4(camera.matrixWorldInverse).z)`,
    not floless's Euclidean `camera.position.distanceTo(p)`: projected size depends on view-space z, so
    the Euclidean form oversizes off-axis handles and desynchronises the drawn size from the 16 px
    picker.
  - Do not port floless's dead "box face pads" branch or its comment.
- `pickClipHandle` — screen-space nearest within **16 px**, no raycast, so a handle buried in geometry
  is still grabbable (consistent with `depthTest:false`).
- Drag updates **source geometry only**, then `rebuildClipPlanes → applyClips → update gizmo`. Box faces
  clamp to a **1 mm minimum extent**. `dragging` stays null until past `DRAG_TOL_PX` (4).
- Readout: plane shows delta moved (`⟂`), box the resulting extent along the drag axis, in the existing
  fixed `#readout` panel — no second cursor-following element.

### 3.0 One frame's work per frame

Persistent objects remove allocation churn but not *rate*: raw pointer events fire faster than the
display refreshes, and each one would otherwise raycast, run clipping tests, rebuild clip planes and
re-point the renderer's array. **Store the latest pointer event and do the work at most once per
`requestAnimationFrame`** — plane-ghost preview, box preview + snapping, and live handle mutation all
coalesce through it.

**Coalescing creates a terminal-event race that must be resolved explicitly**, or the queued frame
either loses the last few millimetres of a drag or, worse, mutates state *after* the commit or revert:

| Terminal | Queued frame |
|---|---|
| `pointerup` | **flush synchronously, then commit** — the final drag distance is part of the result |
| `pointercancel`, capture loss, `Esc` | **cancel and discard, then revert** |

Every queued callback carries the gesture token it was scheduled under and is a no-op if the token has
changed, so a frame can never apply to a gesture that has already ended.

### 3.1 One gesture owner — the old handlers are removed, not supplemented

```
gesture = null | 'clip-place' | 'clip-draw' | 'clip-handle' | 'box-select'
```
decided once on `pointerdown`, priority **armed clip mode → clip handle → box-select**, one terminal
path each. **The existing `boxStart` `pointerdown`/`pointermove`/`pointerup` listeners (:1158–1168) are
deleted**, and rubber-band updates plus hover preview both route through the unified `pointermove`.
Keeping the old listeners alongside a new owner would recreate exactly the arbitration problem the owner
exists to remove.

### 3.2 Capture and cancellation

`setPointerCapture` on an accepted `pointerdown`; `pointerup`, `pointercancel` **and**
`lostpointercapture` all funnel into one `endGesture()`. Without it, releasing off-canvas strands
`controls.enabled=false` (orbit dead until reload), a visible rubber band, or a half-finished drag.

Semantics must be explicit, because a handle drag mutates live:

| Terminal | Outcome |
|---|---|
| `pointerup` | **commit** — keep the live mutation |
| `pointercancel`, unexpected capture loss | **revert** to `prePoint`/`preBox` |
| `Esc` mid-drag | revert (highest-priority key branch) |

`endGesture()` **clears ownership first and is idempotent** — a normal `releasePointerCapture` itself
fires `lostpointercapture`, so a non-idempotent handler would revert a just-committed drag.

Every terminal path calls one **`refreshReadout()`**. floless's readout floats and hides itself; the
viewer's `#readout` is always visible, so without this it stays stuck on the last drag value forever.

## 4. Unit 4 — the 3-click box draw, and snapping

| State | Click | Preview | Esc |
|---|---|---|---|
| armed, no corners | set `{a}`; a miss does nothing | reticle | disarm |
| `{a, b:null}` | set `b` — **reject unless BOTH plan extents ≥ 1 mm** | reticle + floor rectangle | back to armed |
| `{a, b}` | commit at the pulled height; **refused while height is unusable** | reticle at snapped level + pulled box | back to `{a, b:null}` |

- **Both extents independently.** floless rejects a second corner only when `|dx|<1` **and** `|dy|<1`,
  so a straight line passes and the third click silently adds nothing.
- Floor per §0.3. Height = closest point on the vertical line through the footprint centre to the cursor
  ray, clamped ≥ 1 mm above the floor, then snapped to levels.
- **Axial views: refuse, don't fudge.** When a 1-ft vertical segment at the footprint centre projects
  under 4 px (top/bottom view), height is meaningless — floless keeps the pulled value, which collapses
  to the 1 mm clamp in a top view and can yield roughly camera-to-floor distance in a bottom view. Mark
  height unavailable, say so in the readout (*"Orbit off a top view to set the height"*), and **refuse
  the third-click commit** until the axis is usable.
- **Re-clamp after snapping** and filter level candidates to `y ≥ floorY + 1`; a level below the floor
  would otherwise produce a downward box.
- `setClipPreview` — persistent `Box3Helper` (`:997`) in `overlayScene`, box mutated in place.

### Snapping

Ports unchanged: `snapPoint`, `PRECEDENCE`, `SNAP_TOL_PX = 10`. Ported parameterized: `candidatePoint`.

| Candidate | Viewer source |
|---|---|
| `vertex` | endpoints via the existing **`axisEnds(e)`** normalization + `at`/`center` records — **not** raw `from`/`to`, which misses rods, fasteners and node-like elements |
| `midpoint` / `centerline` | element axes from the same normalization |
| `intersection` | **pairwise bounded intersections of non-vertical element axes in rendered X/Z**, deduplicated, placed at the draw-floor Y. Upstream `snapCandidates` emits these and `PRECEDENCE` ranks them above centrelines; omitting them while porting the precedence table and the `x` reticle glyph would have left both referring to a candidate type that never existed |
| `vertical-axis` | elements whose axis is world-Y-dominant |
| `grid-line` / `grid-int` | `referenceSystemSegments` (§0.2); intersections only between **cross-direction** axes |
| `level` | **element-axis endpoint world-Y values, deduplicated, unioned with** `referenceSystemSegments().levels` |

**Levels come from the model, not only from authored datums.** Upstream `clipBoxLevels()` uses
`elevationLevelsByPlan`, which derives elevations from *member endpoints* — grid levels are an addition,
not the source. Using reference-system levels alone would leave every model without an authored
structural grid unable to snap box height to its own beam and column elevations, which is most of them.
The above-floor filter (`y ≥ floorY + 1`) applies to the union.

Grid candidates come from the **shared transform**, never from rendered geometry: grid axes are inert
`THREE.Line`s outside `pickable`/`targetOf`, and a Rust test asserts a grid id is not addressable.

**Cost control.** Candidates are cached once per draw (the scene is immutable after `renderScene`), but
that alone leaves every hover frame projecting every candidate. Cache the **projected screen positions**
of fixed candidates in a screen-space index, so projection is paid per camera move rather than per hover
frame. `candAllowed3d` → `() => true`.

**Invalidation is a single `invalidateClipProjectionCache()`, and "camera change" is not the only
trigger** — a stale index does not merely cost time, it snaps to the wrong place. Call it from:
OrbitControls `change`; `resize` (new projection matrix); `setProjection` (the camera *object* is
swapped, :374–382); `applyView`/`frameBox` (view presets and framing); and `reframeOrtho` (frustum and
zoom change without a camera move).

Reticle: camera-facing sprite, screen-constant 44 px, cyan `#22d3ee` with a dark halo, glyph per snap
type, material **`depthTest:false, depthWrite:false`** with an explicit render order — the overlay pass
reuses the main depth buffer, so `overlayScene` membership alone does not guarantee visibility. (floless
puts its marker in `scene`, where a clip can section its own snap reticle.)

## 5. Unit 5 — the clip list

**Swatch = enable/disable** (filled cutting, hollow off), **label = select**, **Rename button**,
**× delete**.

- **Selection is immediate; rename is its own button.** floless defers the single click ~200 ms so a
  double-click can rename, snapshotting modifier flags because the event is dead by the time the timer
  fires. That makes selection feel laggy and leaves rename mouse-only. The hack does not port.
- Multi-select: Shift = contiguous range from the anchor, Ctrl/Cmd = toggle, plain = replace — **except
  a plain click on an already-solely-selected clip clears the selection**, which is the source
  behaviour and the only modifier-free way to dismiss the gizmo.
- **Every row control is a real `<button type="button">`** with a ≥24 px hit area, `:focus-visible`,
  `aria-label`, and `aria-pressed`/`aria-checked` — matching the objects panel and improving on floless,
  whose swatch has `role="checkbox"` but no `tabindex` and no keydown handler, with bare `<span>` label
  and ×.
- Inline rename errors (blank, duplicate) are announced, not just toasted.
- Tooltips `data-tip`, never native `title=` (a test asserts the absence).

**Layout.** Two independently absolutely-positioned panels with dynamic heights can overlap or jointly
overflow a short viewport. Put clips and `#legend` in **one bounded bottom-left flex column**, shared
max-height, independently scrolling bodies, both carrying the objects panel's themed scrollbar block
verbatim.

**This unit goes through the `frontend-design` skill and the `ui-ux-designer` agent before
implementation**, inside the locked dark theme (`:root` at :43, no new colours).

## 6. Keybindings

| Key | Action |
|---|---|
| `Shift+X` | arm clip plane (exists) |
| `Shift+B` | box around selection (exists, unchanged) |
| `Shift+D` | arm the 3-click draw (new) |
| `Esc` | step back / disarm / revert mid-drag |
| `Delete` / `Backspace` | delete selected clips (new; only when clips are selected) |

All behind the existing `typingInto` guard (:1283) — without it, typing a word containing a shortcut
letter into any field swings the camera and swallows the character.

## 7. Verification

**Contains-assertions are necessary but not sufficient.** A string-presence test passes just as happily
with a Z-up `candidatePoint`, stranded pointer state, or an unreachable handler.

- **Rust guards** for wiring, `boxToPlanes` signs, the `applyClips` enabled-filter, the 1 mm clamps,
  ARIA on new controls, and `renders_identical_bytes_for_identical_scene` staying green.
- **Browser suite — one runnable entry point.** `cli/tests/browser/run.mjs` is the whole gate: it
  generates fixtures, serves them, runs Playwright, cleans up, and **exits nonzero on any failed
  assertion**, so "the gate passed" is distinguishable from ad hoc clicking around.
  - **It must exercise the worktree build, not an installed release.** A plain
    `aware agent invoke viewer-3d render` resolves the *globally installed* CLI and agent — it would
    happily test a shipped binary and report green while the change under test is untouched. The driver
    therefore uses `cargo run --manifest-path cli/Cargo.toml`, installs `20-agents/_core/viewer-3d`
    into a temporary `AWARE_HOME`, and invokes that, mirroring the existing invocation tests.
  - **Playwright acquisition is part of the entry point.** This repo declares no Playwright dependency,
    so the gate can otherwise die before asserting anything, on the one machine it matters. `run.mjs`
    self-checks for a **pinned** Playwright + Chromium (declared dev dependency, or an explicit pinned
    `npx` invocation) and fails with a clear setup message naming the install command — never a stack
    trace that reads like a product failure.
  - Cases: **a z-up and a y-up scene** (§0.1/§0.2, both directions);
  **patch-corner coplanarity, finiteness and size for near-vertical and near-horizontal normals** (the
  basis guard's silent failure mode); a one-axis-degenerate footprint; top and bottom views refusing the
  commit; a pick against a face hidden behind an active clip; releasing the pointer off-canvas; exact
  committed box bounds after a snap; face-handle drag moving the right face; full keyboard operation of
  the list.
- **Honest scoping:** this is an **agent-driven gate, not a CI gate**, and the plan should not pretend
  otherwise. The repo has *no* CI job for Rust at all, has no browser-test harness or dependency today,
  and the template imports Three.js from a CDN so the suite needs network. It runs on demand before a
  PR, not on every push. Adding Rust CI is worthwhile and is its own piece of work.
- **Probe surface — sized to the assertions above.** floless's `getClips()` returns *rounded* sizes,
  which cannot prove exact snapped bounds or which face moved; and clip bounds alone cannot observe most
  of the cases listed, which would force a test to reach into module-private state. Expose, read-only:
  - exact `box.min`/`box.max`, plane `n`/`point`, `enabled`, `label`, `selectedIds`;
  - current gesture state and draft stage;
  - `clipHandlesScreen()` — projected handle positions, so a test can drag a **specific** face;
  - `planePatchCorners()` for the placed/previewed plane (the basis-guard assertion);
  - `referenceSystemSegments()` world segments and level Ys (the y-up assertion);
  - reticle state (visible, snap type, world point) and height usability;
  - `worldToScreen(p)`, so a test can convert its own expectations rather than re-deriving projection.
- **Local gates run by hand**: `cargo test`, `cargo fmt --all`,
  `cargo clippy --all-targets -- -D warnings`.

## 8. Sequencing

1 → 2 → 3 → 5 → 4. Snapping is the largest and least certain unit and lands last, on an already-working
editable clip model. Units 1–3 and 5 are independently shippable and close the "nothing is manipulable"
complaint on their own.
