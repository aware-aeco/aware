---
name: yard-layout-path
description: Layout::Path API reference (YARD)
---

# Layout::Path API reference

A path entity represents a continuous, multi-segment polyline or bezier curve.

## Methods

- `initialize` — The #initialize method creates a new Layout::Path between a start point and an end point, or from a provided Rectangle or Ellipse.
- `new_arc` — The new_arc method creates a new arc-shaped Layout::Path.
- `append_point` — The #append_point method appends a Geom::Point2d to the end of the Layout::Path.
- `arc` — The #arc method returns the parameters of an arc from the Layout::Path, or nil if path is not an arc.
- `circle` — The #circle method returns the parameters of a circle from the Layout::Path, or nil if path is not a circle.
- `close` — The #close method closes the Layout::Path.
- `closed?` — The #closed? method returns whether the Layout::Path is closed.
- `end_arrow` — The #end_arrow method creates a new Layout::Path from an end arrow.
- `end_point` — The #end_point method returns the end point of the Layout::Path.
- `parametric_length` — The #parametric_length method returns the parametric length for the Layout::Path. The parametric length is the length with respect to the curve of the Layout::Path.
- `point_at` — The #point_at method returns the Geom::Point2d at a given parametric value.
- `point_types` — The #point_types method returns an array of point types corresponding to the Geom::Point2ds in the Layout::Path.
- `points` — The #points method returns an array of Geom::Point2ds in the Layout::Path.
- `start_arrow` — The #start_arrow method creates a new Layout::Path from a start arrow.
- `start_point` — The #start_point method returns the start point of the Layout::Path.
- `tangent_at` — The #tangent_at method returns the tangent Geom::Vector2d at the given parametric value.
- `winding` — The #winding method returns the winding type of the Layout::Path.
