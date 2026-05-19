---
name: allplan-pythonpart
description: This skill encodes the allplan 2025.0 surface of the PythonPart namespace — 9 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AttrBuilder, PythonPartGroup, Functions, View, View2D, PythonPart, View2D3D, ViewCode, and 1 more types.
---

# PythonPart

Auto-generated from vendor docs for allplan 2025.0. 9 types in this namespace.

## AttrBuilder (class)

Define specific PythonPart attributes

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/AttrBuilder/)

### Methods
#### `placement_matrix_attr(matrix: Matrix3D) -> AttributeDoubleVec`

Define attribute holding PythonPart matrix for update

**Remarks:** Define attribute holding PythonPart matrix for update

**Parameters:**
- `matrix` (Matrix3D) — placement matrix

**Returns:** `AttributeDoubleVec` — Attribute holding PythonPart matrix for update

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/AttrBuilder/#PythonPart.AttrBuilder.placement_matrix_attr)

#### `pyp_file_param_list_attr( python_file: str, parameter_list: list[str] ) -> tuple[AttributeStringVec, list[str]]`

Define attribute holding python file name and all parameter - values pairs of pyp file

**Remarks:** Define attribute holding python file name and all parameter - values pairs of pyp file

**Parameters:**
- `python_file` () — Filename of PYP file
- `parameter_list` (list[str]) — list of parameters of PYP file

**Returns:** `tuple[AttributeStringVec, list[str]]` — Attribute holding values as string vector, geometry parameter values

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/AttrBuilder/#PythonPart.AttrBuilder.pyp_file_param_list_attr)

#### `python_part_attr() -> AttributeString`

Define attribute holding "PythonPart" identification string

**Remarks:** Define attribute holding "PythonPart" identification string

**Returns:** `AttributeString` — Attribute holding PythonPart identifier

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/AttrBuilder/#PythonPart.AttrBuilder.python_part_attr)

## Functions (static-class)

Module-level functions of PythonPart

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/)

## PythonPart (class)

Definition of class PythonPart

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPart/)

### Constructors
- `PythonPart( name: str, parameter_list: list[str], hash_value: str, python_file: str, views: list[View2D | PythonPart() | PythonPart() | PythonPart(), common_props: CommonProperties | PythonPart() | PythonPart() | PythonPart() | PythonPart() | PythonPart() | PythonPart() | PythonPart() | PythonPart() | PythonPart() | PythonPart()` — Initialization of class PythonPart

### Methods
#### `__repr__() -> str`

create the element string

**Remarks:** create the element string

**Returns:** `str` — element string

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.__repr__)

#### `add(view: View)`

Add one view

**Remarks:** Add one view

**Parameters:**
- `view` (View) — view

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.add)

#### `create() -> list[AllplanElement]`

create the PythonPart

**Remarks:** create the PythonPart

**Returns:** `list[AllplanElement]` — created elements for the PythonPart

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.create)

#### `distortion_state(state: bool)`

Set distortion state

**Remarks:** Set distortion state

**Parameters:**
- `state` (bool) — distortion state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.distortion_state)

#### `leading_macro(macro_leading: bool)`

Set leading macro

**Remarks:** Set leading macro

**Parameters:**
- `macro_leading` (bool) — leading macro state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.leading_macro)

#### `reset(views: list[View])`

Reset views

**Remarks:** Reset views

**Parameters:**
- `views` (list[View]) — views

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.reset)

### Properties
- `matrix` (Matrix3D, get/set) — Transformation matrix used for local transformation of the PythonPart
- `placement_matrix` (Matrix3D | None, get/set) — Placement matrix
- `views` (list[View], get) — Get the views

## PythonPartGroup (class)

Definition of a PythonPart group

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPartGroup/)

### Constructors
- `PythonPartGroup( name: str, parameter_list: list[str], hash_value: str, python_file: str, pythonparts: list[PythonPart] | PythonPartGroup()` — Initialize TypeError: When list of PythonParts to group contains a non-PythonPart object

### Methods
#### `__str__() -> str`

create the element string

**Remarks:** create the element string

**Returns:** `str` — element string

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPartGroup/#PythonPart.PythonPartGroup.__str__)

#### `append(pythonpart: PythonPart)`

Add a new PythonPart to the group

**Remarks:** Add a new PythonPart to the group

**Parameters:**
- `pythonpart` (PythonPart) — PythonPart to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPartGroup/#PythonPart.PythonPartGroup.append)

#### `create() -> list[AllplanElement]`

create the PythonPartGroup

**Remarks:** create the PythonPartGroup

**Returns:** `list[AllplanElement]` — list with the created elements for the PythonPartGroup

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPartGroup/#PythonPart.PythonPartGroup.create)

#### `extend(pythonparts: list[PythonPart])`

Extend the group with new PythonParts

**Remarks:** Extend the group with new PythonParts

**Parameters:**
- `pythonparts` (list[PythonPart]) — list of PythonParts to add to the group

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPartGroup/#PythonPart.PythonPartGroup.extend)

#### `from_build_ele( build_ele: BuildingElement, name: str = "", pythonparts: list[PythonPart] | None = None, type_uuid: str = "", type_display_name: str = "", ) -> PythonPartGroup`

Initialize the PythonPart group from a BuildingElement

**Remarks:** Initialize the PythonPart group from a BuildingElement

**Parameters:**
- `build_ele` (BuildingElement) — Building element containing parameter values of the PythonPart group
- `name` (str) — Name of the group. If not provided, the name of the .pyp file will be used
- `pythonparts` (list[PythonPart] | None) — List of PythonParts to put into group. You can initialize an empty group and append PythonParts to it using append() or extend()
- `type_uuid` (str) — define the selectable type defines the selectable type
- `type_display_name` (str) — display name for the tooltip and object palette

**Returns:** `PythonPartGroup` — Constructed PythonPart group

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/PythonPartGroup/#PythonPart.PythonPartGroup.from_build_ele)

### Properties
- `pythonparts` (list[PythonPart], get) — List of the PythonParts in the group

## View (class)

Definition of a view class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View/)

### Constructors
- `View( viewtype: MacroSlideType = eGeometry, visibility2d: bool = True, visibility3d: bool = True, start_scale: float = 0.0, end_scale: float = 9999.0, elements: list[Any] | View()` — initialization of view class

### Methods
#### `__repr__() -> str`

create the element string

**Remarks:** create the element string

**Returns:** `str` — element string

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.__repr__)

#### `add(element: Any)`

add one element

**Remarks:** add one element

**Parameters:**
- `element` (Any) — element

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.add)

#### `create() -> MacroSlideElement`

Create view

**Remarks:** Create view

**Returns:** `MacroSlideElement` — macro slide element

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.create)

#### `reset(elements: list[Any])`

Reset elements

**Remarks:** Reset elements

**Parameters:**
- `elements` (list[Any]) — elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.reset)

### Properties
- `elements` (list[Any], get) — Get all elements
- `end_scale` (float, get) — Get the reference end scale
- `start_scale` (float, get) — Get the reference start scale
- `viewtype` (MacroSlideType, get) — Get the type
- `visibility2d` (bool, get) — Get the 2D visibility flag
- `visibility3d` (bool, get) — Get the 3D visibility flag
- `visibility_layer_a` (bool, get/set) — Get the Layer A visibility flag
- `visibility_layer_b` (bool, get/set) — Get the Layer B visibility flag
- `visibility_layer_c` (bool, get/set) — Get the Layer C visibility flag

## View2D (class)

Definition of a 2D view class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View2D/)

### Constructors
- `View2D( elements: list[Any] | View2D()` — Initialization of 2D view class

## View2D3D (class)

Definition of a 2D/3D view class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View2D3D/)

### Constructors
- `View2D3D( elements: list[Any] | View2D3D()` — Initialization of 3D view class

## View3D (class)

Definition of a 3D view class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/View3D/)

### Constructors
- `View3D( elements: list[Any] | View3D()` — Initialization of 3D view class

## ViewCode (class)

Definition of a code view class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/ViewCode/)

### Constructors
- `ViewCode(hash_value: str)` — Initialization of code view class

### Methods
#### `create() -> MacroSlideElement`

Create code view

**Remarks:** Create code view

**Returns:** `MacroSlideElement` — macro slide element

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPart/ViewCode/#PythonPart.ViewCode.create)

