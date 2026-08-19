# Changelog

## 0.1.2 — 2026-08-19

- Select the renderer's zero-section branch from the raw axis as `|q|² <= 1e-6·|d|²`, rather than a cancelling `1 - (n·u)²` on the normalized axis, so a near-vertical member is no longer drawn with its section turned away from how the IFC and Tekla sinks export it. The raw ratio also removes the branch's dependence on how each implementation normalizes, which alone could move a member across the threshold.
- State the canonical zero-section frame's vertical seam as accepted policy: the two rules differ by `azimuth + 90°` where they change over (up to a full 180° inversion), and no seed or threshold placement can close it.

## 0.1.1 — 2026-08-18

- Render every canonical cross-section shape and finite member `rot` from the shared directed right-handed section frame, including Z-up's reflective screen conversion and strict scene-up validation.
