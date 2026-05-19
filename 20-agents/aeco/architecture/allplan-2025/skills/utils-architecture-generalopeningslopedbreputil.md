---
name: allplan-utils-architecture-generalopeningslopedbreputil
description: This skill encodes the allplan 2025.0 surface of the Utils.Architecture.GeneralOpeningSlopedBRepUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GeneralOpeningSlopedBRepUtil.
---

# Utils.Architecture.GeneralOpeningSlopedBRepUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## GeneralOpeningSlopedBRepUtil (class)

" implementation of the utility for a general opening created by a sloped BRep3D

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/GeneralOpeningSlopedBRepUtil/)

### Methods
#### `create_opening_polygons_and_plane_surfaces( general_ele: BaseElementAdapter, opening_cut_geo: BRep3D ) -> tuple[bool, BRep3D, BRep3D, Polygon2D]`

create the opening polygon and planes for a Polyhedron opening

**Remarks:** create the opening polygon and planes for a Polyhedron opening

**Parameters:**
- `general_ele` (BaseElementAdapter) — general element
- `opening_cut_geo` (BRep3D) — geometry for the opening creation

**Returns:** `tuple[bool, BRep3D, BRep3D, Polygon2D]` — opening created state, bottom plane surface, top plane surface, opening polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/GeneralOpeningSlopedBRepUtil/#Utils.Architecture.GeneralOpeningSlopedBRepUtil.GeneralOpeningSlopedBRepUtil.create_opening_polygons_and_plane_surfaces)

