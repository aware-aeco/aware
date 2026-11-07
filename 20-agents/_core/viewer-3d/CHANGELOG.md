# Changelog

## 0.2.4 — 2026-11-06

- A structural grid is drawn at every level: each level shows the grid's axes, labelled, at its own elevation (with the level's name at a free corner) instead of the axes once plus a crosshair per level. A grid with no axes still draws its levels as crosshairs. The receipt reports a level as `renderedKind: "grid-level"`.
- The Objects panel has a **Grid lines** block for every scene with a structural grid, with or without a `legend` descriptor: an **All levels** switch for a grid with more than one level and one switch per level, highest level first. They only show or hide; selection, isolation and **Show all** leave the grid alone, and a search lists the grid rows it matches without counting them. A level with `visible: false` starts switched off, so a producer can open a model with only the bottom-of-steel grid shown (#771).
- Clip-box drawing snaps only to the grid levels that are switched on.
- Grid and level labels are as wide as their text, so a long level name (`EL +99.350 BOB`) is no longer cut to its middle.

## 0.2.3 — 2026-11-04

- The Objects panel finds an object by name: the search matches a row, or the category or section it sits in (typing a steel profile lists every part mark under it), and **Enter** selects the one match and zooms to it. When the text typed is exactly one row's label it counts as the single match, so `S202-b3` is found among `S202-b30…`. A line under the search says "Enter to select and zoom" or how many rows match.
- A producer can name what its rows are with `legend.searchPlaceholder` (for example "Search part mark or profile…"), and start a long category folded with `collapsed: true` on it; a search still opens the categories it matches. Both are optional and an older viewer ignores them.
- Selecting an object in 3D reveals its row: its category opens and the row scrolls into view, also after switching modes.
- The pick strip no longer repeats the group: a meta value equal to the group pill (a steel member's `profile`) is not printed again.

## 0.2.2 — 2026-11-03

- Add **Ghost unselected**, matching FloLess's 3D model: with it on and something selected, everything else fades so the selection stands out; with nothing selected the model looks normal. It lives in a new Visibility (eye) menu, and the eye shows a slash and a dot while it is on. It works over every display mode (Solid, Wire, X-ray, Realistic, Shadowed, each with its own fade level), follows a selection made on the canvas or in the Objects panel, and turns itself off while the selection is hidden, fully transparent or clipped away entirely, so the model never fades to nothing. Clicks under Ghost land on the drawn part of the selection rather than a faded part in front of it. Exposed on `window.__viewer3d` as `setGhost`, `ghost`, `ghostActive` and `opacityOf`.
- The toolbar uses the same icons as FloLess's 3D model toolbar (projection, fit, zoom to selected, display mode, visibility, clip, work area). Each icon button keeps its themed tooltip and has an accessible name; the projection and display-mode buttons show the current choice's icon, and a menu button reports whether its menu is open. A **Zoom to selected** button (the same as Alt+Z) sits beside Fit, and the display modes are listed in FloLess's order.
- **Work area → Set to all objects** now bounds the model closely: the box is padded by 0.5% of the model's largest dimension instead of 4%. On a 6 m frame 152 mm deep the old pad made the box 640 mm deep; it is now 213 mm. Clip boxes and "Define from selection" keep their roomier pad.

## 0.2.1 — 2026-10-23

- The partial-mode "Not shown" note words each left-out record in plain English by its failure code and kind (`washer j-bp-1:wsh0: its hole is as large as the washer`, `rod R-2: it has no length (both ends are the same point)`, `hole H-3 in PL-1: it runs over the edge of the plate`, `bolt group BA-1: it needs a part that was left out`, …), with a plain fallback (`it could not be drawn`) for anything unmapped, instead of the strict validator's schema wording (`innerDiameterMm must be smaller than outerDiameterMm`). The note stays escaped and capped at 8 lines. The receipt's `failed[].message` keeps the exact validator sentence unchanged for callers (#723).

## 0.2.0 — 2026-10-23

- Add opt-in **partial render**: `render` accepts `partial: true` (a node's `config:` or `inputs:`). One invalid record no longer blanks the whole view: only the record that cannot be drawn is left out and reported in the receipt's `failed` list, as `{ id|null, status:"failed", kind, code, message, path, parentId? }` with code `invalid-record`, `invalid-hole`, `duplicate-id`, `depends-on-failed`, `rejected` or `invalid-reference-system`. The default (`partial` absent or `false`) is the unchanged atomic render, byte for byte; a pinned `viewer-3d@0.1.x` caller sees no difference.
- Partial mode keeps the strict validator as the only judge. Each element is checked on its own with the same rules (an invalid plate hole is left off its plate and the plate is kept; the first authored record owns a duplicated id), then the unchanged strict check runs on what is left and removes, one at a time and at most 64, the whole top-level record it names (an operation or reference system, and every cascade: a bolt array whose washer was left out goes with it). The page and the receipt are built from the scene that survives, so a left-out record never reaches the HTML.
- The scene's own shape stays atomic in partial mode: `scene` an object, `elements`/`operations`/`referenceSystems`/`groups`/`grids`/`panels` arrays when present, each panel's `columns`/`rows` arrays, and valid `meta.up`/`meta.units`.
- The legend drops rows (and the categories, sections and modes they empty) that named something left out, without a `legendError` warning; an authored-invalid legend still falls back as before.
- The page shows a small "Not shown" note (static, escaped text; no console message) built from `scene.meta.leftOut` (strings the producer supplies; other entries ignored) and the `failed` rows, at most 8 lines then "and N more". It appears only in partial mode, only when there is something to say.

## 0.1.2 — 2026-08-19

- Select the renderer's zero-section branch from the raw axis as `|q|² <= 1e-6·|d|²`, rather than a cancelling `1 - (n·u)²` on the normalized axis, so a near-vertical member is no longer drawn with its section turned away from how the IFC and Tekla sinks export it. The raw ratio also removes the branch's dependence on how each implementation normalizes, which alone could move a member across the threshold.
- State the canonical zero-section frame's vertical seam as accepted policy: the two rules differ by `azimuth + 90°` where they change over (up to a full 180° inversion), and no seed or threshold placement can close it.

## 0.1.1 — 2026-08-18

- Render every canonical cross-section shape and finite member `rot` from the shared directed right-handed section frame, including Z-up's reflective screen conversion and strict scene-up validation.
