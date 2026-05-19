---
name: allplan-testhelper-profileutilaustin
description: This skill encodes the allplan 2025.0 surface of the TestHelper.ProfileUtilAustin namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: MySimpleAustin.
---

# TestHelper.ProfileUtilAustin

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## MySimpleAustin (class)

implementation of the simple Austin class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtilAustin/)

### Constructors
- `MySimpleAustin( austin_running_event: Event, test_running_event: Event, terminate_event: Event, process_id: str, result_file_name: str, )` — initialize

### Methods
#### `on_ready( _process: Any, _child_process: Any, _command_line: str, _data: Any = None ) -> Any`

function description

**Remarks:** function description

**Parameters:**
- `_process` (Any) — process
- `_child_process` (Any) — child process
- `_command_line` (str) — command line
- `_data` (Any) — data

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtilAustin/#TestHelper.ProfileUtilAustin.MySimpleAustin.on_ready)

#### `on_sample_received(text: str)`

receive the sample result

**Remarks:** receive the sample result

**Parameters:**
- `text` (str) — sample result

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtilAustin/#TestHelper.ProfileUtilAustin.MySimpleAustin.on_sample_received)

#### `on_terminate(_stats: Dict[str, str])`

terminate the profiling

**Remarks:** terminate the profiling

**Parameters:**
- `_stats` (Dict[str, str]) — stats

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtilAustin/#TestHelper.ProfileUtilAustin.MySimpleAustin.on_terminate)

