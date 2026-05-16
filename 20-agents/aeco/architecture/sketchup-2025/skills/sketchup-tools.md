---
name: yard-sketchup-tools
description: Sketchup::Tools API reference (YARD)
---

# Sketchup::Tools API reference

The Tools class contains methods to manipulate a collection of SketchUp tools. You access this collection by calling the Model.tools method.

## Methods

- `active_tool` — The #active_tool method is used to obtain the active Ruby tool.
- `active_tool_id` — The active_tool_id method is used to retrieve the active tool's id.
- `active_tool_name` — The active_tool_name method is used to retrieve the active tool's name.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `model` — The model method is used to get the model associated with this tools object.
- `pop_tool` — The pop_tool method is used to pop the last pushed tool on the tool stack.
- `push_tool` — The push_tool method is used to push (aka activate) a user-defined tool. See the Tool interface for details on creating your own SketchUp tool.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
