---
name: tekla-tekla-structures-drawing-automation
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Drawing.Automation namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AutoDrawingRule, AutoDrawingsStatusEnum, DrawingCreator.
---

# Tekla.Structures.Drawing.Automation

Auto-generated from vendor docs for tekla 2026.0. 3 types in this namespace.

## AutoDrawingRule (class)

The AutoDrawingRule class contains the definition of a rule for the Tekla Structures AutoDrawing tool.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/14fb0762-786d-a810-3a17-c47fa3e05982)

### Constructors
- `AutoDrawingRule(...)` — Creates a new AutoDrawing rule by using an existing rule file.

### Properties
- `Filename` (object, get/set) — The filename of the AutoDrawing script.

## AutoDrawingsStatusEnum (enum)

The status of the AutoDrawing script execution.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/23152b18-adf3-1cd5-e9c7-0d6d3278d1d8)

### Values
- `OPERATION_OK` = `0` — The AutoDrawing script was executed without errors.
- `OPERATION_FAILED` = `1` — An error occured during the AutoDrawing script execution. See the AutoDrawing log file for more details.
- `ERROR_NUMBERING_NOT_UPTODATE` = `2` — The AutoDrawing script could not be executed because the numbering was not up-to-date.
- `ERROR_DRAWING_EDITOR_MUST_BE_CLOSED` = `3` — The command could not be executed because a drawing was open. Close the drawing and try again.

## DrawingCreator (class)

The DrawingCreator class is for handling the creation of drawings using the built-in logic of Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f976051e-3068-321a-e408-f370c1ac0bf5)

### Methods
#### `CreateDrawings(AutoDrawingRule, Identifier)(...)`

Creates drawings using the AutoDrawing tool of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/76d514c8-aa1c-c10a-014a-62dd24b119ed)

#### `CreateDrawings(AutoDrawingRule, Identifier, AutoDrawingsStatusEnum.)(...)`

Creates drawings using the AutoDrawing tool of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/7eac6939-0fe3-e07b-3dc7-7bc1804527c8)

#### `CreateDrawings(AutoDrawingRule, List.Identifier., AutoDrawingsStatusEnum.)(...)`

Creates drawings using the AutoDrawing tool of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/be9b7c58-c015-fd84-1bb8-ec0094be3a92)

