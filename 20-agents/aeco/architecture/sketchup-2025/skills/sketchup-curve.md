---
name: yard-sketchup-curve
description: Sketchup::Curve API reference (YARD)
---

# Sketchup::Curve API reference

The Curve class is used by SketchUp to unite a series of Edge objects into one conceptual entity. Since SketchUp is a surface modeler, all circles, arcs, and arbitrary curves are really just edges that are bound together in sequence.

## Methods

- `count_edges` — The count_edges method is used to retrieve the number of Edge objects that make up the Curve.
- `each_edge` — The each_edge method is used to iterate through all of the Edge objects in the curve.
- `edges` — The edges method is used to retrieve an array of Edge objects that make up the Curve.
- `first_edge` — The first_edge method is used to retrieve the first edge of the curve.
- `is_polygon?` — Returns True if this edge was originally created by the polygon tool, otherwise false.
- `last_edge` — The last_edge method is used to retrieve the last edge of the curve.
- `length` — The length method retrieves the length of the curve.
- `move_vertices` — The #move_vertices method moves the vertices in the curve to points.
- `vertices` — The vertices method retrieves a collection of all vertices in a curve.
