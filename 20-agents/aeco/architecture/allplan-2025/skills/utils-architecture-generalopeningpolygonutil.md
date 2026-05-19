---
name: allplan-utils-architecture-generalopeningpolygonutil
description: This skill encodes the allplan 2025.0 surface of the Utils.Architecture.GeneralOpeningPolygonUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GeneralOpeningPolygonUtil.
---

# Utils.Architecture.GeneralOpeningPolygonUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## GeneralOpeningPolygonUtil (class)

" implementation of the utility for the general opening polygon creation from an opening body

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/GeneralOpeningPolygonUtil/)

### Methods
#### `calculate_opening_polygon( opening: Polyhedron3D | BRep3D, axis_line: Line3D, outer_element_geo: Polygon2D, general_ele: BaseElementAdapter, ) -> Polygon2D`

calculate the opening polygon

**Remarks:** calculate the opening polygon

**Parameters:**
- `opening` (Polyhedron3D | BRep3D) — opening
- `axis_line` (Line3D) — axis line
- `outer_element_geo` (Polygon2D) — outer element geometry
- `general_ele` (BaseElementAdapter) — general element

**Returns:** `Polygon2D` — opening polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/GeneralOpeningPolygonUtil/#Utils.Architecture.GeneralOpeningPolygonUtil.GeneralOpeningPolygonUtil.calculate_opening_polygon)

#### `get_general_element_geometry( general_ele: BaseElementAdapter, ) -> tuple[Polyhedron3D | None, Polygon2D | None]`

get the geometry of the general element

**Remarks:** get the geometry of the general element

**Parameters:**
- `general_ele` (BaseElementAdapter) — general element

**Returns:** `tuple[Polyhedron3D | None, Polygon2D | None]` — 3D geometry, ground view polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/GeneralOpeningPolygonUtil/#Utils.Architecture.GeneralOpeningPolygonUtil.GeneralOpeningPolygonUtil.get_general_element_geometry)

