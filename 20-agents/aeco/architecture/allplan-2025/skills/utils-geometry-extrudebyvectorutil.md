---
name: allplan-utils-geometry-extrudebyvectorutil
description: This skill encodes the allplan 2025.0 surface of the Utils.Geometry.ExtrudeByVectorUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ExtrudeByVectorUtil.
---

# Utils.Geometry.ExtrudeByVectorUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## ExtrudeByVectorUtil (class)

implementation of the extrude by vector utility

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Geometry/ExtrudeByVectorUtil/)

### Methods
#### `extrude( geo_elements: list[CURVES], extrusion_vec: Vector3D, from_center: bool, closed_volume: bool, ) -> Polyhedron3D | BRep3D | None`

extrude the geometry elements by an extrusion vector

**Remarks:** extrude the geometry elements by an extrusion vector

**Parameters:**
- `geo_elements` (list[CURVES]) — geometry elements
- `extrusion_vec` (Vector3D) — extrusion vector
- `from_center` (bool) — extrude from center
- `closed_volume` (bool) — closed volume state

**Returns:** `Polyhedron3D | BRep3D | None` — extrusion element

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Geometry/ExtrudeByVectorUtil/#Utils.Geometry.ExtrudeByVectorUtil.ExtrudeByVectorUtil.extrude)

