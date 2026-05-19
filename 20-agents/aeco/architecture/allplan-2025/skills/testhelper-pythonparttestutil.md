---
name: allplan-testhelper-pythonparttestutil
description: This skill encodes the allplan 2025.0 surface of the TestHelper.PythonPartTestUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PythonPartTestUtil.
---

# TestHelper.PythonPartTestUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## PythonPartTestUtil (class)

Implementation of the test utilities for the nodes

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PythonPartTestUtil/)

### Methods
#### `create_interactor( doc: DocumentAdapter, example_name: str, input_data: Optional[List[Any]] = None, print_script_name: bool = True, ) -> Tuple[ Optional[object], PythonWpfPalette, List[BuildingElement], CoordinateInputMock, ]`

Read the example

**Remarks:** Read the example

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files
- `example_name` (str) — name of the example
- `input_data` (Optional[List[Any]]) — input data
- `print_script_name` (bool) — print script name state

**Returns:** `Tuple[Optional[object], PythonWpfPalette, List[BuildingElement], CoordinateInputMock]` — tuple(created interactor, palette, list with the building elements, coordinate input)

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PythonPartTestUtil/#TestHelper.PythonPartTestUtil.PythonPartTestUtil.create_interactor)

#### `start_script( doc: DocumentAdapter, example_name: str, print_script_name: bool = True, parameter_list: Optional[List[Any]] = None, modify_uuid_list: Optional[List[str]] = None, ) -> Tuple[ Optional[Any], List[BuildingElement], CoordinateInputMock, PythonWpfPalette ]`

Start the example script

**Remarks:** Start the example script

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files
- `example_name` (str) — name of the example
- `print_script_name` (bool) — print the script name in the trace output state
- `parameter_list` (Optional[List[Any]]) — list with the interactor parameters
- `modify_uuid_list` (Optional[List[str]]) — list with the UUIDs of the modified elements

**Returns:** `Tuple[Optional[Any], List[BuildingElement], CoordinateInputMock, PythonWpfPalette]` — tuple(created interactor, list with the building elements, coordinate input, palette)

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PythonPartTestUtil/#TestHelper.PythonPartTestUtil.PythonPartTestUtil.start_script)

