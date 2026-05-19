---
name: allplan-testhelper-unittestrunnerutil
description: This skill encodes the allplan 2025.0 surface of the TestHelper.UnitTestRunnerUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: UnitTestRunnerUtil.
---

# TestHelper.UnitTestRunnerUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## UnitTestRunnerUtil (class)

implementation of the unit test runner utilities

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestRunnerUtil/)

### Methods
#### `init_sys_path(test_project_path: str) -> object > None`

initialize the sys path

**Remarks:** initialize the sys path

**Parameters:**
- `test_project_path` (str) — path of the test project

**Returns:** `object > None` — code coverage object

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestRunnerUtil/#TestHelper.UnitTestRunnerUtil.UnitTestRunnerUtil.init_sys_path)

#### `init_unit_test(load_resources: bool) -> object`

initialize the unit test

**Remarks:** initialize the unit test

**Parameters:**
- `load_resources` (bool) — load the resources

**Returns:** `object` — Allplan document

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestRunnerUtil/#TestHelper.UnitTestRunnerUtil.UnitTestRunnerUtil.init_unit_test)

#### `run_unit_tests( tests: List[TestSuite], code_coverage: object, doc: object, single_test: object = None, ) -> TestResult`

run the unit tests

**Remarks:** run the unit tests

**Parameters:**
- `tests` (List[TestSuite]) — description
- `code_coverage` (object) — description
- `doc` (object) — document of the Allplan drawing files
- `single_test` (object) — description

**Returns:** `TestResult` — test result

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestRunnerUtil/#TestHelper.UnitTestRunnerUtil.UnitTestRunnerUtil.run_unit_tests)

#### `unload_modules(test_project_path: str)`

remove the test project path

**Remarks:** remove the test project path

**Parameters:**
- `test_project_path` (str) — path of the test project

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestRunnerUtil/#TestHelper.UnitTestRunnerUtil.UnitTestRunnerUtil.unload_modules)

