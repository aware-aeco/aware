---
name: tekla-coordinate-systems
description: This skill should be used when reading drawing-space coordinates from Tekla and writing them back into the model, when picking points on a drawing view, when extracting connection or part positions from PDFs that originated as Tekla drawings, or whenever a downstream node needs world-space coordinates derived from a 2D source. Encodes the three Tekla coordinate spaces (drawing, view, model world) and the `TransformationPlane` conversions between them. Apply when writing `tekla.insert` callers, when converting Think Node PDF extractions into model positions, or when debugging "the connection is in the wrong place" bugs.
---

# Tekla coordinate systems

**Three coordinate spaces in Tekla. Conversions go through `TransformationPlane`. Missing this is the #1 source of "the connection is in the wrong place" bugs.**

| Space | What it represents | When it appears |
|---|---|---|
| **Drawing space** | 2D coordinates inside a drawing sheet | PDF callouts, drawing annotations, paper space |
| **View space** | 2D coordinates inside a specific view on a drawing | Per-view picks, click handlers on a drawing view |
| **Model world space** | The actual 3D position in the model | Where the steel actually goes |

## The conversion chain

```
PDF page coords  →  Drawing space   (manual: account for page origin + scale)
Drawing space    →  View space      (View's TransformationPlane)
View space       →  Model world     (active Drawing's TransformationPlane)
```

The Tekla API exposes `TransformationPlane` on both `View` and `Drawing` objects. Apply it in the correct direction.

```csharp
// Inverse of the plane = drawing-to-world
var worldPt = drawing.TransformationPlane.Inverse().Transform(drawingPt);

// Plane itself = world-to-drawing
var drawingPt = drawing.TransformationPlane.Transform(worldPt);
```

## The trap

When the active drawing changes between events, the `TransformationPlane` changes with it. Caching geometry from one drawing and applying it later when a different drawing is active produces garbage coordinates. Always:

1. Capture `TransformationPlane` **at the moment** drawing coordinates are read.
2. Convert to world space immediately if the values will be used later.
3. Do not cache drawing-space coordinates without their corresponding plane.

## Special case: drawing views

A `View` inside a drawing has its **own** `TransformationPlane` (separate from the drawing's). The view's plane maps view-local 2D → drawing-2D, and the drawing's plane maps drawing-2D → world. Do not conflate them.

## Quick check

A downstream node receiving `position: { x, y, z = 0 }` from a Tekla node usually indicates a 2D position masquerading as 3D — the z=0 is the tell-tale sign that someone forgot the world-space conversion. Reject it.

## Source

The math is documented under `Tekla.Structures.Drawing.TransformationPlane` in the Tekla Open API. Vendor errata for specific Tekla versions occasionally change sign conventions — re-test the round-trip on each version bump.
