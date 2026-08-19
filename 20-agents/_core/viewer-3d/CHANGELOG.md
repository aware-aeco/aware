# Changelog

## 0.1.2 — 2026-08-19

- Select the renderer's zero-section branch on the exact perpendicular component rather than a cancelling `1 - (n·u)²`, so a near-vertical member is no longer drawn with its section turned up to 180° away from how the IFC and Tekla sinks export it.
- State the canonical zero-section frame's vertical seam as accepted policy: the two rules differ by `azimuth + 90°` where they change over (up to a full 180° inversion), and no seed or threshold placement can close it.

## 0.1.1 — 2026-08-18

- Render every canonical cross-section shape and finite member `rot` from the shared directed right-handed section frame, including Z-up's reflective screen conversion and strict scene-up validation.
