---
name: allplan-pythonpartutil
description: This skill encodes the allplan 2025.0 surface of the PythonPartUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PythonPartUtil.
---

# PythonPartUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## PythonPartUtil (class)

Implementation of the PythonPart utilities

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/)

### Constructors
- `PythonPartUtil(common_props: CommonProperties | PythonPartUtil()` — Initialize

### Methods
#### `add_architecture_elements(elements: Any)`

Add the architecture elements to the PythonPart. The elements will be appended as CHILD-elements.

**Remarks:** Add the architecture elements to the PythonPart. The elements will be appended as CHILD-elements.

**Parameters:**
- `elements` (Any) — architecture elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_architecture_elements)

#### `add_assemblygroup_elements(elements: Any)`

Add the library elements to the PythonPart. The elements will be appended as CHILD-elements.

**Remarks:** Add the library elements to the PythonPart. The elements will be appended as CHILD-elements.

**Parameters:**
- `elements` (Any) — library elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_assemblygroup_elements)

#### `add_attribute_list(attribute_list: BuildingElementAttributeList)`

Add the attribute list to the PythonPart

**Remarks:** Add the attribute list to the PythonPart

**Parameters:**
- `attribute_list` (BuildingElementAttributeList) — attribute list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_attribute_list)

#### `add_attribute_list_to_sub_element_in_structured_container( attribute_list: BuildingElementAttributeList, object_id: UUID = UUID(int=0) )`

Add attribute list to sub element of StructuredContainer to the PythonPart

**Remarks:** Add attribute list to sub element of StructuredContainer to the PythonPart

**Parameters:**
- `attribute_list` (BuildingElementAttributeList) — attribute list
- `object_id` (UUID) — logical object id

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_attribute_list_to_sub_element_in_structured_container)

#### `add_fixture_elements(elements: Any)`

Add the fixture elements to the PythonPart. The elements will be appended as CHILD-elements.

**Remarks:** Add the fixture elements to the PythonPart. The elements will be appended as CHILD-elements.

**Parameters:**
- `elements` (Any) — fixture elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_fixture_elements)

#### `add_label_elements(elements: Any)`

Add the label elements to the PythonPart.

**Remarks:** Add the label elements to the PythonPart.

**Parameters:**
- `elements` (Any) — label elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_label_elements)

#### `add_library_elements(elements: Any)`

Add the library elements to the PythonPart. The elements will be appended as CHILD-elements.

**Remarks:** Add the library elements to the PythonPart. The elements will be appended as CHILD-elements.

**Parameters:**
- `elements` (Any) — library elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_library_elements)

#### `add_mwsgroup_elements(elements: Any)`

Add the library elements to the PythonPart. The elements will be appended as CHILD-elements.

**Remarks:** Add the library elements to the PythonPart. The elements will be appended as CHILD-elements.

**Parameters:**
- `elements` (Any) — library elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_mwsgroup_elements)

#### `add_pythonpart_view_2d( elements: ModelElement2D | ModelElement3D | Any, view_data: PythonPartViewData = PythonPartViewData(), )`

Add the elements to a 2D view for the next PythonPart. Elements added to this view will be visible only in a ground view.

**Remarks:** Add the elements to a 2D view for the next PythonPart. Elements added to this view will be visible only in a ground view.

**Parameters:**
- `elements` (ModelElement2D | ModelElement3D | Any) — elements
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_pythonpart_view_2d)

#### `add_pythonpart_view_2d3d( elements: ModelElement2D | ModelElement3D | Any, view_data: PythonPartViewData = PythonPartViewData(), )`

Add the elements to a 2D3D view for the next PythonPart. Elements added to this view will be visible in both ground and isometric view.

**Remarks:** Add the elements to a 2D3D view for the next PythonPart. Elements added to this view will be visible in both ground and isometric view.

**Parameters:**
- `elements` (ModelElement2D | ModelElement3D | Any) — elements
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_pythonpart_view_2d3d)

#### `add_pythonpart_view_3d( elements: ModelElement2D | ModelElement3D | Any, view_data: PythonPartViewData = PythonPartViewData(), )`

Add the elements to a 3D view for the next PythonPart. Elements added to this view will be visible only in an isometric view.

**Remarks:** Add the elements to a 3D view for the next PythonPart. Elements added to this view will be visible only in an isometric view.

**Parameters:**
- `elements` (ModelElement2D | ModelElement3D | Any) — elements
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_pythonpart_view_3d)

#### `add_reinforcement_elements(elements: Any)`

Add the reinforcement elements to the PythonPart. The elements will be appended as CHILD-elements.

**Remarks:** Add the reinforcement elements to the PythonPart. The elements will be appended as CHILD-elements.

**Parameters:**
- `elements` (Any) — reinforcement elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_reinforcement_elements)

#### `add_view(elements: View | list[View])`

Add a view for the next PythonPart.

**Remarks:** Add a view for the next PythonPart.

**Parameters:**
- `elements` (View | list[View]) — elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.add_view)

#### `create_pythonpart( build_ele: BuildingElement | list[BuildingElement], local_placement_matrix: Matrix3D = Matrix3D(), placement_matrix: Matrix3D = Matrix3D(), type_uuid: str = "", type_display_name: str = "", ) -> list[Any]`

create a PythonPart with the current views

**Remarks:** create a PythonPart with the current views

**Parameters:**
- `build_ele` (BuildingElement | list[BuildingElement]) — building element with the parameter properties
- `local_placement_matrix` (Matrix3D) — local placement matrix of the PythonPart, used for the local geometry transformation
- `placement_matrix` (Matrix3D) — placement matrix of the PythonPart (model placement)
- `type_uuid` (str) — define the selectable type
- `type_display_name` (str) — display name for the tooltip and object palette

**Returns:** `list[Any]` — list with the created PythonPart elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.create_pythonpart)

#### `get_pythonpart( build_ele: BuildingElement | list[BuildingElement], local_placement_matrix: Matrix3D = Matrix3D(), placement_matrix: Matrix3D = Matrix3D(), type_uuid: str = "", type_display_name: str = "", ) -> PythonPart`

get the PythonPart

**Remarks:** get the PythonPart

**Parameters:**
- `build_ele` (BuildingElement | list[BuildingElement]) — building element with the parameter properties
- `local_placement_matrix` (Matrix3D) — local placement matrix of the PythonPart, used for the local geometry transformation
- `placement_matrix` (Matrix3D) — placement matrix of the PythonPart (model placement)
- `type_uuid` (str) — define the selectable type
- `type_display_name` (str) — display name for the tooltip and object palette

**Returns:** `PythonPart` — created PythonPart

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.get_pythonpart)

#### `set_view_data(view: View, view_data: PythonPartViewData)`

set the view data

**Remarks:** set the view data

**Parameters:**
- `view` (View) — view
- `view_data` (PythonPartViewData) — view data

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartUtil/#PythonPartUtil.PythonPartUtil.set_view_data)

