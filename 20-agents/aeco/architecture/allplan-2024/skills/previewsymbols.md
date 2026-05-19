---
name: allplan-previewsymbols
description: This skill encodes the allplan 2024.0 surface of the PreviewSymbols namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PreviewSymbols.
---

# PreviewSymbols

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## PreviewSymbols (enum)

implementation of the preview symbols

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/)

### Constructors
- `PreviewSymbols()` — initialize

### Methods
#### `__get_color(color)`

get the color as ARGB value

**Remarks:** get the color as ARGB value

**Parameters:**
- `color` (Union[ARGB, int]) — color as ARGB or color ID

**Returns:** `ARGB` — _color as ARGB value

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.__get_color)

#### `add_arrow(reference_point, width, color, rotation_angle)`

add an arrow symbol

**Remarks:** add an arrow symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `rotation_angle` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_arrow)

#### `add_circle(reference_point, radius_pixel, color)`

add a circle symbol

**Remarks:** add a circle symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `radius_pixel` (int) — radius in pixel
- `color` (Union[ARGB, int]) — color as ARGB value or color ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_circle)

#### `add_coord_cross(plane, arm_length)`

add a coordinate cross symbol

**Remarks:** add a coordinate cross symbol

**Parameters:**
- `plane` (Union[Plane3D, AxisPlacement3D]) — plane of the coordinate cross
- `arm_length` (float) — length of the arms

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_coord_cross)

#### `add_cross(reference_point, width, color)`

add a cross symbol

**Remarks:** add a cross symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_cross)

#### `add_filled_rectangle(reference_point, width, color, rotation_angle)`

add a filled rectangle symbol

**Remarks:** add a filled rectangle symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `rotation_angle` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_filled_rectangle)

#### `add_mark(reference_point, width, color)`

add mark symbol

**Remarks:** add mark symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `width` (float) — width
- `color` (Union[ARGB, int]) — color as ARGB value or color ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_mark)

#### `add_polyline(reference_point, polyline, color, line_pattern=4095)`

add a polyline symbol

**Remarks:** add a polyline symbol

**Parameters:**
- `reference_point` (Point3D) — reference point
- `polyline` (Polyline2D) — polyline
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `line_pattern` (object) — line pattern as 0/1 patten

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_polyline)

#### `add_text(text, reference_point, ref_pnt_pos, height, color, rotation_angle)`

add a text symbol

**Remarks:** add a text symbol

**Parameters:**
- `text` (str) — text
- `reference_point` (Point3D) — reference point
- `ref_pnt_pos` (TextReferencePointPosition) — point position of the reference point
- `height` (float) — height
- `color` (Union[ARGB, int]) — color as ARGB value or color ID
- `rotation_angle` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.add_text)

#### `draw(insert_matrix, view_projection)`

draw the symbols

**Remarks:** draw the symbols

**Parameters:**
- `insert_matrix` (Matrix3D) — insert matrix
- `view_projection` (ViewWorldProjection) — view world projection

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PreviewSymbols/PreviewSymbols/#PreviewSymbols.PreviewSymbols.draw)

