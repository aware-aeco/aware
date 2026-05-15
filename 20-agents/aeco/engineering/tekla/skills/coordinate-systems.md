# Tekla Skill · Coordinate systems

**Three coordinate spaces in Tekla. Conversions go through `TransformationPlane`. Missing this is the #1 source of "the connection is in the wrong place" bugs.**

| Space | What it represents | When you see it |
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

The Tekla API exposes `TransformationPlane` on both `View` and `Drawing` objects. **You must apply it in the correct direction.**

```csharp
// Inverse of the plane = drawing-to-world
var worldPt = drawing.TransformationPlane.Inverse().Transform(drawingPt);

// Plane itself = world-to-drawing
var drawingPt = drawing.TransformationPlane.Transform(worldPt);
```

## The trap

When the active drawing changes between events, the `TransformationPlane` changes with it. If you cache geometry from one drawing and apply it later when a different drawing is active, you get garbage coordinates. Always:

1. Capture `TransformationPlane` **at the moment** you read drawing coordinates
2. Convert to world space immediately if you're going to use it later
3. Don't cache drawing-space coordinates without their corresponding plane

## Special case: drawing views

A `View` inside a drawing has its **own** `TransformationPlane` (separate from the drawing's). The view's plane maps view-local 2D → drawing-2D, and the drawing's plane maps drawing-2D → world. Don't conflate them.

## Quick check

If your downstream node receives `position: { x, y, z = 0 }` from a Tekla node, that's a 2D position pretending to be 3D — the z=0 is a tell-tale sign someone forgot the world-space conversion. Reject it.

## Source

The math is described in the Tekla Open API docs under `Tekla.Structures.Drawing.TransformationPlane`. Vendor-side errata for specific Tekla versions occasionally change the sign convention — re-test the round-trip on each version bump.
