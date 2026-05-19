---
name: allplan-pythonpart
description: This skill encodes the allplan 2024.0 surface of the PythonPart namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AttrBuilder, PythonPart, View, PythonPartGroup, ViewCode, View2D, View2D3D, View3D.
---

# PythonPart

Auto-generated from vendor docs for allplan 2024.0. 8 types in this namespace.

## AttrBuilder (class)

Define specific PythonPart attributes

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/AttrBuilder/)

### Methods
#### `placement_matrix_attr(matrix)`

Define attribute holding PythonPart matrix for update

**Remarks:** Define attribute holding PythonPart matrix for update

**Parameters:**
- `matrix` (Matrix3D) — placement matrix

**Returns:** `AttributeDoubleVec` — Attribute holding PythonPart matrix for update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/AttrBuilder/#PythonPart.AttrBuilder.placement_matrix_attr)

#### `pyp_file_param_list_attr(python_file, parameter_list)`

Define attribute holding python file name and all parameter - values pairs of pyp file

**Remarks:** Define attribute holding python file name and all parameter - values pairs of pyp file

**Parameters:**
- `python_file` (str) — Filename of PYP file
- `parameter_list` (list[str]) — list of parameters of PYP file

**Returns:** `AttributeStringVec` — Attribute holding values as string vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/AttrBuilder/#PythonPart.AttrBuilder.pyp_file_param_list_attr)

#### `python_part_attr()`

Define attribute holding "PythonPart" identification string

**Remarks:** Define attribute holding "PythonPart" identification string

**Returns:** `AttributeString` — Attribute holding PythonPart identifier

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/AttrBuilder/#PythonPart.AttrBuilder.python_part_attr)

## PythonPart (class)

Definition of class PythonPart

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPart/)

### Constructors
- `PythonPart(name, parameter_list, hash_value, python_file, views=None, matrix=Matrix3D(), common_props=None, reinforcement=None, attribute_list=None, library_elements=None, architecture_elements=None, label_elements=None, fixture_elements=None, placement_matrix=None)` — Initialization of class PythonPart

### Methods
#### `__repr__()`

create the element string

**Remarks:** create the element string

**Returns:** `str` — element string

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.__repr__)

#### `add(view)`

Add one view

**Remarks:** Add one view

**Parameters:**
- `view` (View) — view

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.add)

#### `create()`

create the PythonPart

**Remarks:** create the PythonPart

**Returns:** `list[Any]` — created elements for the PythonPart

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.create)

#### `distortion_state(state)`

Set distortion state

**Remarks:** Set distortion state

**Parameters:**
- `state` (bool) — distortion state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.distortion_state)

#### `leading_macro(macro_leading)`

Set leading macro

**Remarks:** Set leading macro

**Parameters:**
- `macro_leading` (bool) — leading macro state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.leading_macro)

#### `reset(views)`

Reset views

**Remarks:** Reset views

**Parameters:**
- `views` (list[View]) — views

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPart/#PythonPart.PythonPart.reset)

### Properties
- `views` (list[View], get) — Get the views

## PythonPartGroup (class)

Definition of class PythonPartGroup

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPartGroup/)

### Constructors
- `PythonPartGroup(name, parameter_list, hash_value, python_file, pythonparts=None)` — Initialization of class PythonPartGroup

### Methods
#### `__str__()`

create the element string

**Remarks:** create the element string

**Returns:** `str` — element string

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPartGroup/#PythonPart.PythonPartGroup.__str__)

#### `create()`

create the PythonPartGroup

**Remarks:** create the PythonPartGroup

**Returns:** `list[Any]` — list with the created elements for the PythonPartGroup

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/PythonPartGroup/#PythonPart.PythonPartGroup.create)

## View (class)

Definition of a view class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View/)

### Constructors
- `View(viewtype=eGeometry, visibility2d=True, visibility3d=True, start_scale=0.0, end_scale=9999.0, elements=None)` — initialization of view class

### Methods
#### `__repr__()`

create the element string

**Remarks:** create the element string

**Returns:** `str` — element string

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.__repr__)

#### `add(element)`

add one element

**Remarks:** add one element

**Parameters:**
- `element` (Any) — element

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.add)

#### `create()`

Create view

**Remarks:** Create view

**Returns:** `MacroSlideElement` — macro slide element

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.create)

#### `reset(elements)`

Reset elements

**Remarks:** Reset elements

**Parameters:**
- `elements` (list[Any]) — elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View/#PythonPart.View.reset)

### Properties
- `elements` (list[Any], get) — Get all elements
- `end_scale` (float, get) — Get the reference end scale
- `start_scale` (float, get) — Get the reference start scale
- `viewtype` (AllplanBasisEle.MacroSlideType, get) — Get the type
- `visibility2d` (bool, get) — Get the 2D visibility flag
- `visibility3d` (bool, get) — Get the 3D visibility flag
- `visibility_layer_a` (bool, get/set) — Get the Layer A visibility flag
- `visibility_layer_b` (bool, get/set) — Get the Layer B visibility flag
- `visibility_layer_c` (bool, get/set) — Get the Layer C visibility flag

## View2D (class)

Definition of a 2D view class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View2D/)

### Constructors
- `View2D(elements=None, start_scale=0.0, end_scale=9999.0)` — Initialization of 2D view class

## View2D3D (class)

Definition of a 2D/3D view class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View2D3D/)

### Constructors
- `View2D3D(elements=None, start_scale=0.0, end_scale=9999.0)` — Initialization of 3D view class

## View3D (class)

Definition of a 3D view class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/View3D/)

### Constructors
- `View3D(elements=None, start_scale=0.0, end_scale=9999.0)` — Initialization of 3D view class

## ViewCode (class)

Definition of a code view class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/ViewCode/)

### Constructors
- `ViewCode(hash_value)` — Initialization of code view class

### Methods
#### `create()`

Create code view

**Remarks:** Create code view

**Returns:** `MacroSlideElement` — macro slide element

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPart/ViewCode/#PythonPart.ViewCode.create)

