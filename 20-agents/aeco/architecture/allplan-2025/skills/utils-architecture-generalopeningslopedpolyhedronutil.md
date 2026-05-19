---
name: allplan-utils-architecture-generalopeningslopedpolyhedronutil
description: This skill encodes the allplan 2025.0 surface of the Utils.Architecture.GeneralOpeningSlopedPolyhedronUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GeneralOpeningSlopedPolyhedronUtil.
---

# Utils.Architecture.GeneralOpeningSlopedPolyhedronUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## GeneralOpeningSlopedPolyhedronUtil (class)

" implementation of the utility for a general opening created by a sloped Polyhedron3D

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/GeneralOpeningSlopedPolyhedronUtil/)

### Methods
#### `create_opening_polygon_and_plane_surfaces( general_ele: BaseElementAdapter, opening_cut_geo: Polyhedron3D ) -> tuple[bool, Polyhedron3D, Polyhedron3D, Polygon2D]`

create the opening polygon and planes for a Polyhedron opening

**Remarks:** create the opening polygon and planes for a Polyhedron opening

**Parameters:**
- `general_ele` (BaseElementAdapter) — general element
- `opening_cut_geo` (Polyhedron3D) — geometry for the opening creation

**Returns:** `tuple[bool, Polyhedron3D, Polyhedron3D, Polygon2D]` — opening created state, bottom plane surface, top plane surface, opening polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/GeneralOpeningSlopedPolyhedronUtil/#Utils.Architecture.GeneralOpeningSlopedPolyhedronUtil.GeneralOpeningSlopedPolyhedronUtil.create_opening_polygon_and_plane_surfaces)

