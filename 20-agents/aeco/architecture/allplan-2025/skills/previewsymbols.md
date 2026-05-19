---
name: allplan-previewsymbols
description: This skill encodes the allplan 2025.0 surface of the PreviewSymbols namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PreviewSymbols.
---

# PreviewSymbols

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## PreviewSymbols (enum)

implementation of the preview symbols

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/)

### Constructors
- `PreviewSymbols()` — initialize

### Methods
#### `add_arrow( reference_point: Point3D, width: float, color: Union[ARGB, int], rotation_angle: Angle, )`

add an arrow symbol

**Remarks:** add an arrow symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `rotation_angle` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_arrow)

#### `add_circle( reference_point: Point3D, radius_pixel: int, color: Union[ARGB, int] )`

add a circle symbol

**Remarks:** add a circle symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `radius_pixel` (int) — radius in pixel
- `color` (Union[ARGB, int]) — color as ARGB value or color ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_circle)

#### `add_coord_cross(plane: Union[Plane3D, AxisPlacement3D], arm_length: float)`

add a coordinate cross symbol

**Remarks:** add a coordinate cross symbol

**Parameters:**
- `plane` (Union[Plane3D, AxisPlacement3D]) — plane of the coordinate cross
- `arm_length` (float) — length of the arms

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_coord_cross)

#### `add_cross(reference_point: Point3D, width: float, color: Union[ARGB, int])`

add a cross symbol

**Remarks:** add a cross symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_cross)

#### `add_filled_rectangle( reference_point: Point3D, width: float, color: Union[ARGB, int], rotation_angle: Angle, )`

add a filled rectangle symbol

**Remarks:** add a filled rectangle symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `rotation_angle` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_filled_rectangle)

#### `add_mark(reference_point: Point3D, width: float, color: Union[ARGB, int])`

add mark symbol

**Remarks:** add mark symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_mark)

#### `add_polyline( reference_point: Point3D, polyline: Polyline2D, color: Union[ARGB, int], line_pattern: int = 4095, )`

add a polyline symbol

**Remarks:** add a polyline symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `polyline` (Polyline2D) — polyline
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `line_pattern` (int) — line pattern as 0/1 patten

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_polyline)

#### `add_text( text: str, reference_point: Point3D, ref_pnt_pos: TextReferencePointPosition, height: float, color: Union[ARGB, int], rotation_angle: Angle, )`

add a text symbol

**Remarks:** add a text symbol

**Parameters:**
- `text` (str) — text
- `reference_point` (Point3D) — reference point
- `ref_pnt_pos` (TextReferencePointPosition) — point position of the reference point
- `height` (float) — height
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `rotation_angle` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_text)

#### `draw( insert_matrix: Matrix3D, view_projection: ViewWorldProjection, use_system_angle: bool = True, )`

draw the symbols

**Remarks:** draw the symbols

**Parameters:**
- `insert_matrix` (Matrix3D) — insert matrix
- `view_projection` (ViewWorldProjection) — view world projection
- `use_system_angle` (bool) — use the system angle state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PreviewSymbols/#PreviewSymbols.PreviewSymbols.draw)

