---
name: self-verify-overlay
description: Use after ANY image/PDF extraction that places things in space (members on a plan, boxes on a schedule, rooms on a layout) — render the extraction back over the source, read the overlay with vision against the source, flag gaps + mistakes, correct, re-render, and loop until clean. The compose-time / review-time correctness check that schema validation and human description-review both miss.
---

# Self-verify an extraction: the overlay-vs-source loop

Schema validation proves an extraction is **well-formed**. It does not prove it is **spatially
right**. A region can be silently **under-extracted** (expected elements missing) or **mis-placed**
(an element bound off its real target) and still pass the schema — and description-level review
("32 members, looks plausible") reliably misses it. The cheap, general fix is a closed visual
loop: **render the extraction back as an overlay on the source, then read that overlay with your
own vision against the source**, until the two agree.

## Where this runs (and where it does NOT)

This is a **compose-time / review-time loop the host AI runs** — when reading a drawing into data
(the bake / re-bake pattern), or when reviewing a `vision.extract` result before its `approve:`
gate. It is **not** a runtime node and **not** a loop inside `vision.extract`.

`vision.extract` is fenced (decalog #9 / RFC #223): it **extracts, never decides**, is
schema-bound, content-hash cached, and **cannot branch control flow** — so a self-correcting loop
cannot live in the run path. This loop adds a **correctness multiplier, not a new freedom**: the
judgment stays where AWARE already allows it (compose time, and the human `approve:` gate), and the
run path stays the reviewed, deterministic plan.

## The loop

1. **Render** the extracted artifact as an overlay on the source raster — the members, boxes, or
   regions you read, drawn at the coordinates you assigned, over the original drawing/photo.
2. **Inspect** the overlay against the source *with vision* (read your own render — do not ask the
   user to eyeball it). Emit a structured finding list:
   ```json
   {
     "gaps":     [{ "where": "skewed bay, grid C–D", "expected": "≈8 beams", "confidence": 0.9 }],
     "mistakes": [{ "where": "label B12",            "issue": "drawn off its member, sits in the gap", "confidence": 0.8 }],
     "clean":    false
   }
   ```
3. **Correct** from the findings — re-read / re-bind only the flagged regions (don't redo the
   parts that already match).
4. **Re-render and re-inspect**, looping until a clean pass (`clean: true`) **or** a max-iteration
   cap (3 is plenty; more usually means the source is genuinely ambiguous, not that another pass
   will help).
5. **Emit residual gaps/mistakes as flags** — anything still unsure after the cap goes into the
   `approve:` summary as RFI-like notes ("bay C–D member count uncertain — confirm"), so the human
   confirms exactly the doubtful spots rather than re-checking the whole extraction.

## Worked example (the bug this was born from)

`steel-from-drawings`, 2026-06-17. A one-pass read placed steel members from a framing plan, but a
whole **skewed bay silently collapsed** — many member labels bound onto a couple of segments
instead of spreading across the bay. Schema valid; counts plausible; description review missed it.

One closed self-verify pass caught and fixed it: render the member overlay → **read the rendered
overlay** against the drawing → observe *"the skew wedge is collapsed, not merely misplaced"* →
root-cause (label coordinates were scale-ambiguous because the image had been downscaled) → fix
(normalize to fractional coordinates) → re-render → re-inspect = correct. The visual check found
what neither schema validation nor description-review did.

## Rules

- **Read your own render — always.** The check is the AI inspecting its overlay in a closed loop,
  never the user eyeballing it. (Same standing discipline behind every visual output: overlay,
  render, diagram, model.)
- **Use it whenever placement matters** — any drawing/image → spatial data extraction. Skip it for
  a non-spatial read (a clean key/value table a parser could have read anyway → prefer a
  deterministic `exec` parser; see [when-to-extract.md](when-to-extract.md)).
- **Confidence, then flag.** A residual you can't resolve is not a failure to hide — surface it at
  the `approve:` gate so the human's review is aimed at the doubtful regions.
- **Cap the loop.** Clean pass or 3 iterations, whichever first. Don't grind on an ambiguous source.

## For reader agents

Any agent that reads a drawing into structured data — the `steel-detailer` family, a steel-takeoff
reader, any drawings-to-scene reader — should run this loop as the verification step of its read,
before handing the data downstream or to a human. It is the generic self-check for "I turned a
picture into data; is the data actually where the picture says?"
