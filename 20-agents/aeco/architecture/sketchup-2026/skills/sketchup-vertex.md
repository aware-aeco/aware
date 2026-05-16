---
name: yard-sketchup-vertex
description: Sketchup::Vertex API reference (YARD)
---

# Sketchup::Vertex API reference

A Vertex. A Vertex represents the end of an Edge or a point inside a Face.

## Methods

- `common_edge` — The common_edge method is used to find a common edge that is defined by this vertex and another vertex
- `curve_interior?` — This method doesn't actually return a boolean as the question mark post-fix would normally indicate. But the result still evaluates to truthy or falsy.
- `edges` — The edges method is used to retrieve an Array of edges that use the Vertex.
- `faces` — The faces method is used to retrieve an Array of faces that use the vertex.
- `loops` — The loops method is used to retrieve an Array of loops that use the vertex.
- `position` — The position method is used to retrieve the Point3d position of a vertex.
- `used_by?` — The used_by? method is used to determine if the Vertex is used by a given Edge or Face.
