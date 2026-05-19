---
name: allplan-pythonpartutil
description: This skill encodes the allplan 2024.0 surface of the PythonPartUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PythonPartUtil.
---

# PythonPartUtil

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## PythonPartUtil (class)

Implementation of the PythonPart utilities

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/)

### Constructors
- `PythonPartUtil(common_props=None)` — Initialize

### Methods
#### `add_architecture_elements(elements)`

add the architecture elements to the PythonPart

**Remarks:** add the architecture elements to the PythonPart

**Parameters:**
- `elements` (Any) — architecture elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_architecture_elements)

#### `add_attribute_list(attribute_list)`

add the attribute list to the PythonPart

**Remarks:** add the attribute list to the PythonPart

**Parameters:**
- `attribute_list` (BuildingElementAttributeList) — attribute list

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_attribute_list)

#### `add_fixture_elements(elements)`

add the fixture elements to the PythonPart

**Remarks:** add the fixture elements to the PythonPart

**Parameters:**
- `elements` (Any) — fixture elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_fixture_elements)

#### `add_label_elements(elements)`

add the label elements to the PythonPart

**Remarks:** add the label elements to the PythonPart

**Parameters:**
- `elements` (Any) — label elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_label_elements)

#### `add_library_elements(elements)`

add the library elements to the PythonPart

**Remarks:** add the library elements to the PythonPart

**Parameters:**
- `elements` (Any) — library elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_library_elements)

#### `add_pythonpart_view_2d(elements, view_data=PythonPartViewData())`

add the elements to a 2D view for the next PythonPart

**Remarks:** add the elements to a 2D view for the next PythonPart

**Parameters:**
- `elements` (ModelElement2D | ModelElement3D | Any) — elements
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_pythonpart_view_2d)

#### `add_pythonpart_view_2d3d(elements, view_data=PythonPartViewData())`

add the elements to a 2D3D view for the next PythonPart

**Remarks:** add the elements to a 2D3D view for the next PythonPart

**Parameters:**
- `elements` (ModelElement2D | ModelElement3D | Any) — elements
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_pythonpart_view_2d3d)

#### `add_pythonpart_view_3d(elements, view_data=PythonPartViewData())`

add the elements to a 3D view for the next PythonPart

**Remarks:** add the elements to a 3D view for the next PythonPart

**Parameters:**
- `elements` (ModelElement2D | ModelElement3D | Any) — elements
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_pythonpart_view_3d)

#### `add_reinforcement_elements(elements)`

add the reinforcement elements to the PythonPart

**Remarks:** add the reinforcement elements to the PythonPart

**Parameters:**
- `elements` (Any) — reinforcement elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_reinforcement_elements)

#### `add_view(elements)`

add a view for the next PythonPart

**Remarks:** add a view for the next PythonPart

**Parameters:**
- `elements` (View | list[View]) — elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_view)

#### `create_pythonpart(build_ele, local_placement_matrix=Matrix3D(), placement_matrix=Matrix3D())`

create a PythonPart with the current views

**Remarks:** create a PythonPart with the current views

**Parameters:**
- `build_ele` (BuildingElement | list[BuildingElement]) — building element with the parameter properties
- `local_placement_matrix` (Matrix3D) — local placement matrix of the PythonPart, used for the local geometry transformation
- `placement_matrix` (Matrix3D) — placement matrix of the PythonPart (model placement)

**Returns:** `list[Any]` — list with the created PythonPart elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.create_pythonpart)

#### `set_view_data(view, view_data)`

set the view data

**Remarks:** set the view data

**Parameters:**
- `view` (View) — view
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartUtil/PythonPartUtil/#PythonPartUtil.PythonPartUtil.set_view_data)

