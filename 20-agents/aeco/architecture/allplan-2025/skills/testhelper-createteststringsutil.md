---
name: allplan-testhelper-createteststringsutil
description: This skill encodes the allplan 2025.0 surface of the TestHelper.CreateTestStringsUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CreateTestStringsUtil.
---

# TestHelper.CreateTestStringsUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## CreateTestStringsUtil (class)

Implementation of the model element string utils

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/CreateTestStringsUtil/)

### Methods
#### `copy_element_and_parameter_strings_to_clipboard(model_ele_list: list[Any])`

create a string from the model elements and parameters and copy it the to clipboard

**Remarks:** create a string from the model elements and parameters and copy it the to clipboard

**Parameters:**
- `model_ele_list` (list[Any]) — model elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/CreateTestStringsUtil/#TestHelper.CreateTestStringsUtil.CreateTestStringsUtil.copy_element_and_parameter_strings_to_clipboard)

#### `copy_element_strings_to_clipboard(model_ele_list: list[Any])`

create a string from the model elements and copy it the to clipboard

**Remarks:** create a string from the model elements and copy it the to clipboard

**Parameters:**
- `model_ele_list` (list[Any]) — model elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/CreateTestStringsUtil/#TestHelper.CreateTestStringsUtil.CreateTestStringsUtil.copy_element_strings_to_clipboard)

#### `copy_parameter_strings_to_clipboard(model_ele_list: list[Any])`

create a string from the parameters and copy it the to clipboard

**Remarks:** create a string from the parameters and copy it the to clipboard

**Parameters:**
- `model_ele_list` (list[Any]) — model elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/CreateTestStringsUtil/#TestHelper.CreateTestStringsUtil.CreateTestStringsUtil.copy_parameter_strings_to_clipboard)

#### `get_geometry_elements_text(model_elements: list[Any]) -> str`

get the element text from the geometry elements

**Remarks:** get the element text from the geometry elements

**Parameters:**
- `model_elements` (list[Any]) — model elements

**Returns:** `str` — string of the geometry from the model elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/CreateTestStringsUtil/#TestHelper.CreateTestStringsUtil.CreateTestStringsUtil.get_geometry_elements_text)

#### `get_model_elements_data( model_ele_list: list[Any], ) -> tuple[list[Any], list[Any], list[Any]]`

get the exploded model elements and the parameter list Explodes the PythonPart/PythonPartGroup and gets the model objects from the slides

**Remarks:** get the exploded model elements and the parameter list Explodes the PythonPart/PythonPartGroup and gets the model objects from the slides

**Parameters:**
- `model_ele_list` (list[Any]) — list with the created model elements

**Returns:** `tuple[list[Any], list[Any], list[Any]]` — list with the model elements contained in the PythonPart, parameters of the PythonPart

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/CreateTestStringsUtil/#TestHelper.CreateTestStringsUtil.CreateTestStringsUtil.get_model_elements_data)

