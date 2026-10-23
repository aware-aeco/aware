# Changelog

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
