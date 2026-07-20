# Plan — full clip parity for AWARE's viewer-3d

**Goal.** The shared 3D viewer's clip is today three commands: arm a plane pick, box-around-selection,
clear all. The floless steel editor's clip is an editable object model — draw a box by picking points
with snapping, drag its faces afterwards, enable/rename/delete each clip from a list. This plan brings
the viewer to functional parity.

**Not in scope.** Anything that depends on the editor's document: undo integration
(`api.beginClipEdit`), contract persistence, per-plan/per-sheet scales, the running-snaps preference
UI, and the tool-exclusivity disarms for editing tools the viewer does not have. The viewer's clip is
the only left-click tool, so that whole arbitration layer collapses.

Source of truth for the port: `floless.app/web/steel-3d-view.js` §clip (1466–1801, 1623–1738),
`web/steel-3d-core.js` (`snapPoint`, `PRECEDENCE`), and the clip rows in `web/steel-editor.html`
(5215–5239, 6004–6027).

---

## 0. The invariant that governs the whole port — READ FIRST

**floless's clip code is written for a Z-up world. The viewer's rendered world is always Y-up.**

`conv(P,up)` (viewer_3d.rs:292) maps scene `(x,y,z)` → world `(x,z,y)` when `meta.up==='z'`, and is
the identity when it is `'y'`. Either way what reaches Three.js is Y-up, and `camera.up` is pinned to
`(0,1,0)` (:377).

Therefore, for every ported function:

- The clip subsystem operates **entirely in rendered world space**, where up is **+Y**.
- Any elevation arriving from the scene JSON (`referenceSystems[].levels[].elevationMm`, element
  `from`/`to`) must pass through `conv` **before** it becomes a snap candidate.
- `clipBoxFloorZ` becomes `clipBoxFloorY` — `sceneBox.min.y`.
- The box-height pull axis is world **+Y**.
- `planePatchCorners`'s degenerate-basis guard must test the **up** component, not `z`. floless's
  `Math.abs(n.z) < 0.9 ? (0,0,1) : (1,0,0)` becomes `Math.abs(n.y) < 0.9 ? (0,1,0) : (1,0,0)`.
  Getting this wrong is invisible on a horizontal cut and wrong only on a vertical one.

A literal copy-paste of the floless source is therefore a bug, not a shortcut. Every axis reference is
a decision point.

---

## 1. Unit 1 — clips become editable objects

Today a clip is `{id, kind, planes}` and a box clip **discards its `Box3` at creation** (:1212). There
is nothing for a handle to grab, and `applyClips` unconditionally flattens every clip, so there is
nothing to disable. This unit is a prerequisite for units 3 and 5.

New record, mirroring the work-area record already in the file (:1238):

```js
{ id:'clip'+(++clipSeq), kind:'plane'|'box', enabled:true, label:'Plane 1'|'Box 1',
  n:Vector3, point:Vector3,      // SOURCE, plane only
  box:Box3,                      // SOURCE, box only
  planes:[Plane] }               // DERIVED — never edited directly
```

- `rebuildClipPlanes(c)` — the single derive step: `box ? boxToPlanes(c.box) :
  [new THREE.Plane().setFromNormalAndCoplanarPoint(c.n, c.point)]`.
- `applyClips` gains `.filter(c => c.enabled)`. **It must keep calling `syncClipMirror()`** — the
  shadow pass holds its own array reference and a stale one leaves the model visibly clipped after the
  clip is gone (documented trap at viewer_3d.rs:1190–1193).
- **`boxToPlanes` sign conventions are not to be touched.** The comment at :1177–1180 records that they
  were verified live and warns against "reversing" them.
- Lifecycle, all following the same shape (guard → mutate → `applyClips()` → `renderClipGizmo()`):
  `toggleClip`, `renameClip` (rejects blank and case-insensitive duplicates), `removeClip`,
  `clearClips`, `setSelectedClips`/`selectClip`/`selectedClips`, `deleteSelectedClips`, `getClips`.
- `selectedClipIds:Set` — selection drives gizmo visibility only, never what is cut, so it must **not**
  call `applyClips`.
- `addClipPlaneAtScreen` gains the new fields and auto-selects the new clip.

**Deliberate omission:** `addClipBox(pad)` (box-around-selection) is *dead code in floless* — the menu
item arms the 3-click draw instead, and floless's tooltip is stale as a result. The viewer's
`addClipBox` is live and bound to Shift+B. Decision: **keep it**, as a second command distinct from the
draw. Shift+B stays "box around selection"; the draw gets its own menu item and key. Rationale: it is
already shipped behaviour with a passing test, and removing a working command is not parity work.

## 2. Unit 2 — the hover ghost on an armed plane pick

Today an armed plane pick gives a crosshair cursor and nothing else: you click blind and find out where
the cut landed afterwards.

- Factor the raycast out of `addClipPlaneAtScreen` into `clipPlaneAtScreen(cx,cy) → {n,point}|null`, so
  the ghost and the commit **cannot disagree** (this is why floless shares it).
- `planePatchCorners(hp,n,r)` with the up-axis guard from §0.
- `setClipPlanePreview(hp,n)` — a translucent fill (`PlaneGeometry(2r,2r)`, opacity .3,
  `side:DoubleSide`, `depthTest:false`, `renderOrder:995`) plus a `LineLoop` outline
  (`renderOrder:996`), both added to **`overlayScene`** so no clip can section its own preview.
  Disposes the previous group's geometry and materials, array-safe.
- `r = CLIP_PLANE_PATCH_R` = 304.8 mm → a 2'×2' marker. The ghost and the placed gizmo's outline share
  the size and position so the preview lands exactly where the click will.
- Hook: a `pointermove` listener calling `clipPlanePreviewAt` while `clipMode==='plane'`, plus a
  `pointerleave` that clears it. **The viewer has no hover handling today** — this is new plumbing, and
  it must not fight the existing rubber-band `pointermove` (:1159), which early-outs on `!boxStart`.

Colours are floless's, and all three already exist in the viewer's palette family:
`CLIP_PLANE_COLOR 0x3b82f6`, `CLIP_BOX_COLOR 0x93c5fd`, `CLIP_PREVIEW_COLOR 0xbfdbfe`.

## 3. Unit 3 — the gizmo and handle dragging

The manipulator is the same for a plane and for each box face: a translucent disc lying in the
plane/face plus a normal arrow (stem + cone). Grab either and slide along the normal.

- `renderClipGizmo()` builds a `THREE.Group` **in `overlayScene`** — handles added to `scene` would be
  clipped by the very planes they edit.
  - Box: a `Box3Helper` (`renderOrder:996`) + 6 handles over
    `FACE_AXES=[{axis:'x',sign:±1},{axis:'y',sign:±1},{axis:'z',sign:±1}]`, each anchored at a face
    centre with the outward face normal as its drag axis.
  - Plane: a `LineLoop` patch at `c.point` + one handle along `c.n`.
  - Per handle: `disc` (`CircleGeometry`, opacity .32, `renderOrder:998`), `stem` (`Line`, rewritten
    per frame, **no `clipHandle` flag** so it is not pickable), `cone` (`ConeGeometry`,
    `renderOrder:999`). All `depthTest:false`.
- `sizeClipHandles()` runs **per frame** from the render loop: screen-constant sizing via `pxToWorldAt`
  — disc 14 px, arrow 11 px at a 34 px offset along the normal from `baseHp`, stem rewritten to match.
  `pxToWorldAt` and `lineClosestT` are new helpers (~10 lines each).
  - Port note: floless's branch order makes its generic 10 px "box face pads" branch unreachable, and
    its comment does not match any object `addHandle` builds. Do not port the dead branch or its
    comment; port the 14/11/34 numbers, which are the live ones.
- `pickClipHandle(cx,cy)` — screen-space nearest within **16 px**, no raycast, so a handle buried in
  geometry is still grabbable (consistent with `depthTest:false`).
- `startClipDrag` / `onMoveClip`:
  - Drag updates **source geometry only** (`c.point` or `c.box`), then
    `rebuildClipPlanes → applyClips → renderClipGizmo`.
  - Box faces clamp to a **1 mm minimum extent** (`min+1` / `max-1`) so a box can never invert.
  - `dragging` stays null until the pointer passes `DRAG_TOL_PX` (4), so a click that does not move
    changes nothing.
  - Readout: plane shows the delta moved (`⟂`), box shows the resulting extent along the drag axis
    (axis letter). **Divergence from floless:** it positions a floating readout at cursor+14; the
    viewer has a fixed `#readout` panel written via `replaceChildren`. Use the existing panel — do not
    introduce a second, cursor-following readout element.
  - Esc mid-drag reverts from `prePoint`/`preBox`. (floless leaves an undo entry behind here; with no
    undo stack in the viewer that wart does not port.)
- Pointer arbitration — the delicate part. Today the chain is, at `pointerup` (:1163–1168):
  *drag past 5 px → box-select; else armed clip plane; else pick.* A handle drag must intercept at
  **`pointerdown`**, before `boxStart` is set (:1158), and must set `controls.enabled=false` for the
  duration. Left-button orbit is already disabled (`mouseButtons.LEFT:-1`), so the only contender is
  box-select.

## 4. Unit 4 — the 3-click box draw, and snapping

CAD "rectangle then extrude": click two floor corners, then move up/down to pull the height and click
to commit. `clipBoxDraft: null → {a,b:null} → {a,b} → committed`.

State machine (each step's Esc behaviour is part of the contract):

| State | Click | Preview | Esc |
|---|---|---|---|
| armed, no corners | set `{a}`; a miss (ray parallel to the floor) does nothing | reticle only | disarm |
| `{a, b:null}` | set `b`; **reject a same-point click** (<1 mm on each axis independently) without clearing the draft | reticle + flat floor rectangle | back to armed |
| `{a, b}` | commit at the pulled height; disarm unconditionally; add the box **only if all three extents are strictly positive** | reticle at a snapped level + the pulled 3D box | back to `{a, b:null}` |

- Floor = `sceneBox.min.y` (§0) — the bottom of the whole model, not a ground plane and not the camera
  target.
- Height = closest point on the vertical line through the footprint centre to the cursor ray, clamped
  to **≥1 mm above the floor**, then snapped to level elevations. **Port the edge-on guard**: if a
  1-ft vertical segment at the footprint centre projects to under 4 px (a top view), skip snapping —
  every level collapses to one screen point and snapping would silently pick the lowest.
- `setClipPreview(box)` — `Box3Helper` in `overlayScene`, `renderOrder:997`.

### Snapping — 100% new construction in the viewer

There is no snapping engine here; the only `snap` in the file is the ViewCube's face classifier and is
unrelated. What ports as-is is the *pure* core: `snapPoint(dragged, candidates, toScreen, tolPx)` and
its `PRECEDENCE` tie-break table, `SNAP_TOL_PX = 10`.

What must be **rebuilt**, because floless derives it from the editor's 2D plan sheets:

| Candidate | floless source | Viewer source |
|---|---|---|
| `vertex` | member work-points per plan | element `from`/`to`, via `conv` |
| `midpoint` | beam midpoints | element midpoints |
| `centerline` | beam centrelines | element axes |
| `vertical-axis` | columns | elements whose axis is world-Y-dominant |
| `grid-line` / `grid-int` | `api.getGrid()` | `SCENE.referenceSystems[].axes[].offsetMm` + pairwise intersections |
| `level` | `elevationLevelsByPlan` | `SCENE.referenceSystems[].levels[].elevationMm`, via `conv` |

**The grid must be read from the scene JSON, not from the rendered geometry.** Grid axes render as
inert `THREE.Line`s outside `pickable`/`targetOf`, and a Rust test
(`a_target_the_renderer_skips_is_not_a_valid_target`) asserts a grid id is *not* addressable. The JSON
is the clean source and is already Rust-validated.

- Cache candidates **once per draw** (`clipCandidates`, `clipLevels`), dropped on arm/disarm. In the
  viewer the scene is immutable after `renderScene`, so floless's four model-change invalidation sites
  reduce to one — but the cache must still be dropped on arm/disarm so a stale reticle cannot survive.
- `candAllowed3d` reduces to `() => true` (no snap-preferences UI).
- Reticle: a camera-facing sprite whose glyph names the snap type, screen-constant at 44 px, cyan
  `#22d3ee` with a dark halo so it reads against any background. Port the glyph set
  (`vertex→square, intersection/grid-int→x, midpoint→tri, centerline/grid-line→hourglass, level→bar`,
  else dot) — textures are lazily built canvases, which is runtime-only and cannot affect render
  determinism.
- **Divergence:** floless's marker lives in `scene` (so a clip can section its own snap reticle — a
  wart). Put it in `overlayScene`.

## 5. Unit 5 — the clip list

No list exists today: clips have ids but nothing renders them, and clear-all is the only removal path.

Row contract, from floless: **swatch = enable/disable** (filled = cutting, hollow = off),
**label = select (click) / rename (double-click)**, **× = delete**. The single click is deferred ~200 ms
so a double-click can cancel it, with the modifier flags snapshotted into a plain object because the
event is dead by the time the timer fires. Multi-select: Shift = contiguous range from the anchor,
Ctrl/Cmd = toggle, plain = replace (or clear when it is already the sole selection).

**Placement decision.** floless puts clips in its legend. The viewer's `#legend` is descriptor-driven
(`scene.legend`) and its `#side` is hidden when the scene supplies no panels, so neither is a safe
host. Use a dedicated `.panel #clips`, bottom-left above `#legend`, rendered only when
`clips.length > 0`. It must be a bounded, **themed** scroll container — copy the objects panel's
`scrollbar-width`/`::-webkit-scrollbar*` block verbatim; a native light scrollbar on this dark panel is
exactly the leak the house rule names.

Accessibility — the viewer's bar is higher than floless's here, and the port must **improve on the
source**: floless's swatch carries `role="checkbox"` + `aria-checked` but no `tabindex` and no keydown
handler, so it is mouse-only, and the label and × are bare `<span>`s. In the viewer every row control
must be a real `<button type="button">` with a ≥24 px hit area and `:focus-visible`, matching the
objects panel's existing convention.

Tooltips are `data-tip`, never native `title=` — there is a test asserting the absence.

## 6. Verification

- **Rust guards** (the file's convention is contains-assertions pinning the exact load-bearing source
  line, plus negative guards that stop a fixed bug returning):
  - a `ships_*` test naming every new function, DOM id, `data-*` hook and keybinding;
  - the sign conventions in `boxToPlanes` and the `applyClips` enabled-filter;
  - the up-axis guard from §0, pinned explicitly — it is the port's likeliest silent bug;
  - the 1 mm box-extent clamp and the same-point rejection;
  - `renders_identical_bytes_for_identical_scene` must keep passing (no clock, no RNG, no
    iteration-order leak into the HTML);
  - ARIA assertions for the new row controls.
- **`__viewer3d` probe surface** — extend it, since it is how any headless E2E asserts state:
  `clipHandlesScreen()` (projected handle positions so a test can drag a specific one),
  `clipPlanePreviewShown()`, `clipBoxFloorAt(x,y)`, `getClips()`, `selectedClips()`. Per the file's own
  stated philosophy, expose the probe that *proves the thing works*, not merely that it was requested.
- **Local gates, all three, run by hand**: `cargo test`, `cargo fmt --all`,
  `cargo clippy --all-targets -- -D warnings`. **CI runs none of them** — there is no Rust job in any
  workflow, so nothing will catch a regression for me.
- **Real browser E2E** on the rendered document: draw a box with snapping and assert the reticle and
  the committed extents; drag a face handle and assert the section moves; toggle/rename/delete from the
  list; Esc step-back at each stage; and a regression pass that click-to-inspect, box-select and Alt+Z
  still behave with a clip present.

## 7. Sequencing

1 → 2 → 3 → 5 → 4. Snapping (4) is the largest and least certain unit, so it lands last, on top of an
already-working editable clip model. Units 1–3 and 5 are independently shippable and already close the
"nothing is manipulable" complaint that started this.
