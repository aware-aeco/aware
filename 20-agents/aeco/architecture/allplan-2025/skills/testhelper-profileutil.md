---
name: allplan-testhelper-profileutil
description: This skill encodes the allplan 2025.0 surface of the TestHelper.ProfileUtil namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, ProfileUtil, ProfilerType.
---

# TestHelper.ProfileUtil

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of TestHelper.ProfileUtil

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtil/)

### Methods
#### `use_line_profiler(mod: Any) -> Any`

implementation of the line usage function

**Remarks:** implementation of the line usage function

**Parameters:**
- `mod` (Any) — module to add

**Returns:** `Any` — result of the executed function

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtil/#TestHelper.ProfileUtil.use_line_profiler)

## ProfileUtil (class)

Profiler utility class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtil/ProfileUtil/)

### Methods
#### `profile( function: Callable, *param: Any, calls_to_print: int = 20, description: str = "", show_graphical_results: bool = False, profiler_type: ProfilerType = C_PROFILE, show_separator_line: bool = False, result_file_name: str = "" ) -> Any`

profile by cprofile

**Remarks:** profile by cprofile

**Parameters:**
- `function` (Callable) — function to execute
- `param` (Any) — function parameter
- `calls_to_print` (int) — number of calls shown in the profile report (only for C_Profile)
- `description` (str) — description of the text
- `show_graphical_results` (bool) — show graphical results (only for C_Profile)
- `profiler_type` (ProfilerType) — type of the profiler
- `show_separator_line` (bool) — show a separator line before the results
- `result_file_name` (str) — name of the result file

**Returns:** `Any` — return value of the call function

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtil/ProfileUtil/#TestHelper.ProfileUtil.ProfileUtil.profile)

## ProfilerType (enum)

Definition of profiler types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/ProfileUtil/ProfilerType/)

### Values
- `AUSTIN` = `4`
- `C_PROFILE` = `1`
- `LINE_PROFILER` = `3`
- `PY_INSTRUMENT` = `2`

