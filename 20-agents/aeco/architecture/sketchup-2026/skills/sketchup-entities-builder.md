---
name: yard-sketchup-entities-builder
description: Sketchup::EntitiesBuilder API reference (YARD)
---

# Sketchup::EntitiesBuilder API reference

Like Geom::PolygonMesh there is minimal validation checks made on the input to the creation of the geometry. Vertices are de-duplicated and edges sharing the same vertices will be de-duplicated. But no intersection of overlapping entities is made. It leaves a higher responsibility on the API user to produce valid geometry.

## Methods

- `add_edge` — Does not split intersecting faces or edges.
- `add_edges` — Does not split intersecting faces or edges.
- `add_face` — Does not split intersecting faces or edges.
- `entities` — The Sketchup::Entities collection the Sketchup::EntitiesBuilder will add the geometry to.
- `valid?` — Indicates whether the builder object is valid and can be used.
- `vertex_at` — Finds an existing Vertex for the given position, otherwise returns nil.
