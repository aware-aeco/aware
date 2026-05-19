---
name: allplan-typecollections-modelelelist
description: This skill encodes the allplan 2024.0 surface of the TypeCollections.ModelEleList namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelEleList.
---

# TypeCollections.ModelEleList

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## ModelEleList (class)

implementation of the model element list

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/TypeCollections/ModelEleList/ModelEleList/)

### Constructors
- `ModelEleList(com_prop=GetCurrentCommonProperties())` — initialize

### Methods
#### `append_geometry_2d(geo_ele, com_prop=None)`

append a 2D geometry element to the list

**Remarks:** append a 2D geometry element to the list

**Parameters:**
- `geo_ele` (Union[ListModelEle2D, ModelEle2D, ModelEle2DList]) — 2D geometry element
- `com_prop` (Optional[CommonProperties]) — common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/TypeCollections/ModelEleList/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.append_geometry_2d)

#### `append_geometry_3d(geo_ele, com_prop=None)`

append a 3D geometry element to the list

**Remarks:** append a 3D geometry element to the list

**Parameters:**
- `geo_ele` (Union[ListModelEle3D, ModelEle3D, ModelEle3DList]) — 3D geometry element
- `com_prop` (Optional[CommonProperties]) — common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/TypeCollections/ModelEleList/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.append_geometry_3d)

#### `set_color(color)`

set the color

**Remarks:** set the color

**Parameters:**
- `color` (int) — color

[Docs](https://pythonparts.allplan.com/2024/api_reference/TypeCollections/ModelEleList/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_color)

#### `set_pen(pen)`

set the pen

**Remarks:** set the pen

**Parameters:**
- `pen` (int) — pen

[Docs](https://pythonparts.allplan.com/2024/api_reference/TypeCollections/ModelEleList/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_pen)

#### `set_stroke(stroke)`

set the stroke

**Remarks:** set the stroke

**Parameters:**
- `stroke` (int) — stroke

[Docs](https://pythonparts.allplan.com/2024/api_reference/TypeCollections/ModelEleList/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_stroke)

