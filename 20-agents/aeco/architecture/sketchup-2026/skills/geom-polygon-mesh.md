---
name: yard-geom-polygon-mesh
description: Geom::PolygonMesh API reference (YARD)
---

# Geom::PolygonMesh API reference

As of SketchUp 2022.0 the new Sketchup::EntitiesBuilder interface can be used to generate bulk geometry. It has similar performance as PolygonMesh, but with similar degree of per-entity control as Sketchup::Entities.

## Methods

- `initialize` — When creating a mesh with normals and/or UVQ data it's critical that the number of points estimated is equal to or higher than the final number of points added. If fewer points are estimated the normals and UVQ data might end up out of sync.
- `add_point` — In SketchUp 2021.1 this method was improved to be faster. See #initialize for details.
- `add_polygon` — In SketchUp 2021.1 this method was improved to be faster. See #initialize for details.
- `count_points` — The #count_points method is used to count the number of points in a mesh.
- `count_polygons` — The #count_polygons count the number of polygons in the mesh.
- `normal_at` — Index starts at 1.
- `point_at` — Index starts at 1.
- `point_index` — Returns 0 if point is not found.
- `points` — The #points method is used to retrieve an array of points (vertices) in the mesh
- `polygon_at` — Index starts at 1.
- `polygon_points_at` — Index starts at 1.
- `polygons` — The #polygons method is used to retrieve an array of all polygons in the mesh.
- `set_point` — Index starts at 1.
- `set_uv` — If you don't specify how many points you will be adding to the mesh when you initiate it you may risk the UV data becoming out of sync.
- `transform!` — The #transform! method is used to apply a transformation to a mesh.
- `uv_at` — Index starts at 1.
- `uvs` — The #uvs method is used to retrieve an array of uv coordinates in the mesh.
