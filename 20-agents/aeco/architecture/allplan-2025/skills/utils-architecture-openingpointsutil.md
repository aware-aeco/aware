---
name: allplan-utils-architecture-openingpointsutil
description: This skill encodes the allplan 2025.0 surface of the Utils.Architecture.OpeningPointsUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: OpeningPointsUtil.
---

# Utils.Architecture.OpeningPointsUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## OpeningPointsUtil (class)

implementation of the opening points utilities

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/)

### Methods
#### `calc_point_at_placement_arc( local_dist: float, placement_arc: Arc2D, placement_polyline: Polyline2D ) -> tuple[bool, Point2D]`

calculate the point at the placement arc

**Remarks:** calculate the point at the placement arc

**Parameters:**
- `local_dist` (float) — local point distance at the placement arc
- `placement_arc` (Arc2D) — placement arc
- `placement_polyline` (Polyline2D) — placement polyline

**Returns:** `tuple[bool, Point2D]` — found state, point at the polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.calc_point_at_placement_arc)

#### `create_opening_end_point_for_axis_element( opening_start_pnt: Point2D, opening_width: float, element_axis: Line2D | Arc2D, element_polygon: Polygon2D | Polyline2D, placement_line: Line2D, ) -> Point2D`

create the opening points for an axis element

**Remarks:** create the opening points for an axis element

**Parameters:**
- `opening_start_pnt` (Point2D) — opening start point
- `opening_width` (float) — opening width
- `element_axis` (Line2D | Arc2D) — element axis
- `element_polygon` (Polygon2D | Polyline2D) — element polygon / polyline
- `placement_line` (Line2D) — placement line

**Returns:** `Point2D` — end point of the opening

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.create_opening_end_point_for_axis_element)

#### `create_opening_end_point_for_shaped_element( opening_start_pnt: Point2D, opening_width: float, placement_line: Line2D ) -> Point2D`

create the opening points for a shaped element

**Remarks:** create the opening points for a shaped element

**Parameters:**
- `opening_start_pnt` (Point2D) — opening start point
- `opening_width` (float) — opening width
- `placement_line` (Line2D) — placement line

**Returns:** `Point2D` — end point of the opening

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.create_opening_end_point_for_shaped_element)

#### `create_opening_points_for_axis_element( opening_start_pnt: Point2D, opening_end_pnt: Point2D, opening_width: float, element_thickness: float, ) -> list[Point2D]`

create the opening points for an axis element

**Remarks:** create the opening points for an axis element

**Parameters:**
- `opening_start_pnt` (Point2D) — opening start point
- `opening_end_pnt` (Point2D) — opening end point
- `opening_width` (float) — opening width
- `element_thickness` (float) — element thickness

**Returns:** `list[Point2D]` — opening points

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.create_opening_points_for_axis_element)

#### `get_distance_from_offset_end_point( input_pnt: Point3D, element_axis: Line2D | Arc2D | None, offset_end_pnt: Point2D, placement_line: Line2D, placement_arc: Arc2D | None, element_polygon: Polygon2D | Polyline2D, ) -> float`

get the local distance between input and offset end point

**Remarks:** get the local distance between input and offset end point

**Parameters:**
- `input_pnt` (Point3D) — input point
- `element_axis` (Line2D | Arc2D | None) — element axis
- `offset_end_pnt` (Point2D) — offset end point
- `placement_line` (Line2D) — placement line
- `placement_arc` (Arc2D | None) — placement arc
- `element_polygon` (Polygon2D | Polyline2D) — element polygon / polyline

**Returns:** `float` — distance between input and start point

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.get_distance_from_offset_end_point)

#### `get_distance_from_offset_start_point( input_pnt: Point3D, element_axis: Line2D | Arc2D | None, offset_start_pnt: Point2D, placement_line: Line2D, placement_arc: Arc2D | None, element_polygon: Polygon2D | Polyline2D, ) -> float`

get the local distance between input and offset start point

**Remarks:** get the local distance between input and offset start point

**Parameters:**
- `input_pnt` (Point3D) — input point
- `element_axis` (Line2D | Arc2D | None) — element axis
- `offset_start_pnt` (Point2D) — offset start point
- `placement_line` (Line2D) — placement line
- `placement_arc` (Arc2D | None) — placement arc
- `element_polygon` (Polygon2D | Polyline2D) — element polygon / polyline

**Returns:** `float` — distance between input and start point

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.get_distance_from_offset_start_point)

#### `get_opening_offset_points( element: BaseElementAdapter, opening_start_pnt: Point2D, placement_line: Line2D, ) -> tuple[bool, Point2D, Point2D]`

get the left and right point for the opening offset

**Remarks:** get the left and right point for the opening offset

**Parameters:**
- `element` (BaseElementAdapter) — element for the opening
- `opening_start_pnt` (Point2D) — opening start point
- `placement_line` (Line2D) — placement line from the opening start point

**Returns:** `tuple[bool, Point2D, Point2D]` — points created, start offset point, end offset point

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.get_opening_offset_points)

#### `get_start_point_from_end_offset( element_axis: Line2D | Arc2D | None, offset_end_pnt: Point2D, placement_line: Line2D, placement_arc: Arc2D | None, opening_width: float, element_polygon: Polygon2D | Polyline2D, offset: float, ) -> tuple[bool, Point2D]`

get the start point by an offset from an offset start point

**Remarks:** get the start point by an offset from an offset start point

**Parameters:**
- `element_axis` (Line2D | Arc2D | None) — element axis
- `offset_end_pnt` (Point2D) — offset end point
- `placement_line` (Line2D) — placement line
- `placement_arc` (Arc2D | None) — placement arc
- `opening_width` (float) — opening width
- `element_polygon` (Polygon2D | Polyline2D) — element polygon / polyline
- `offset` (float) — offset from the offset start point

**Returns:** `tuple[bool, Point2D]` — start point of the opening

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.get_start_point_from_end_offset)

#### `get_start_point_from_start_offset( element_axis: Line2D | Arc2D | None, offset_start_pnt: Point2D, placement_line: Line2D, placement_arc: Arc2D | None, element_polygon: Polygon2D | Polyline2D, offset: float, ) -> tuple[bool, Point2D]`

get the start point by an offset from an offset start point

**Remarks:** get the start point by an offset from an offset start point

**Parameters:**
- `element_axis` (Line2D | Arc2D | None) — element axis
- `offset_start_pnt` (Point2D) — offset start point
- `placement_line` (Line2D) — placement line
- `placement_arc` (Arc2D | None) — placement arc
- `element_polygon` (Polygon2D | Polyline2D) — element polygon / polyline
- `offset` (float) — offset from the offset start point

**Returns:** `tuple[bool, Point2D]` — start point of the opening

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/OpeningPointsUtil/#Utils.Architecture.OpeningPointsUtil.OpeningPointsUtil.get_start_point_from_start_offset)

