---
name: allplan-typecollections-modelelelist
description: This skill encodes the allplan 2025.0 surface of the TypeCollections.ModelEleList namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelEleList.
---

# TypeCollections.ModelEleList

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## ModelEleList (class)

implementation of the model element list

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/)

### Constructors
- `ModelEleList(com_prop: CommonProperties = GetCurrentCommonProperties())` — initialize

### Methods
#### `__getitem__(index: int) -> AllplanElement`

get the model element by an index

**Remarks:** get the model element by an index

**Parameters:**
- `index` (int) — index

**Returns:** `AllplanElement` — model element as AllplanElement

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.__getitem__)

#### `append_geometry_2d( geo_ele: ListModelEle2D | ModelEle2D | ModelEle2DList, com_prop: CommonProperties | None = None, )`

append a 2D geometry element to the list

**Remarks:** append a 2D geometry element to the list

**Parameters:**
- `geo_ele` (ListModelEle2D | ModelEle2D | ModelEle2DList) — 2D geometry element
- `com_prop` (CommonProperties | None) — common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.append_geometry_2d)

#### `append_geometry_3d( geo_ele: ListModelEle3D | ModelEle3D | ModelEle3DList | Curve3DList, com_prop: CommonProperties | None = None, )`

append a 3D geometry element to the list

**Remarks:** append a 3D geometry element to the list

**Parameters:**
- `geo_ele` (ListModelEle3D | ModelEle3D | ModelEle3DList | Curve3DList) — 3D geometry element
- `com_prop` (CommonProperties | None) — common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.append_geometry_3d)

#### `append_geometry_3d_with_texture( geo_ele: ListModelEle3D | ModelEle3D | ModelEle3DList | Curve3DList, texture_def: TextureDefinition, com_prop: CommonProperties | None = None, )`

append a 3D geometry element to the list

**Remarks:** append a 3D geometry element to the list

**Parameters:**
- `geo_ele` (ListModelEle3D | ModelEle3D | ModelEle3DList | Curve3DList) — 3D geometry element
- `texture_def` (TextureDefinition) — texture definition
- `com_prop` (CommonProperties | None) — common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.append_geometry_3d_with_texture)

#### `set_color(color: int)`

set the color

**Remarks:** set the color

**Parameters:**
- `color` (int) — color

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_color)

#### `set_common_properties(com_prop: CommonProperties)`

set the common properties

**Remarks:** set the common properties

**Parameters:**
- `com_prop` (CommonProperties) — common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_common_properties)

#### `set_element_attributes(index: int, attributes: list[Attribute])`

set the attributes to the element with the defined index

**Remarks:** set the attributes to the element with the defined index

**Parameters:**
- `index` (int) — index
- `attributes` (list[Attribute]) — attributes

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_element_attributes)

#### `set_pen(pen: int)`

set the pen

**Remarks:** set the pen

**Parameters:**
- `pen` (int) — pen

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_pen)

#### `set_stroke(stroke: int)`

set the stroke

**Remarks:** set the stroke

**Parameters:**
- `stroke` (int) — stroke

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModelEleList/#TypeCollections.ModelEleList.ModelEleList.set_stroke)

