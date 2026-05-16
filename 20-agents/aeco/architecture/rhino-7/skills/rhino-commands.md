---
name: rhino-common-rhino-commands
description: API reference for namespace Rhino.Commands from RhinoCommon.dll
---

# Rhino.Commands

- **Command**
  - Defines a base class for all commands. This class is abstract.
- **CommandEventArgs**
- **CommandStyleAttribute**
  - Decorates commands to provide styles.
- **CustomUndoEventArgs**
  - Argument package that is passed to a custom undo delegate
- **MostRecentCommandDescription**
  - Stores the macro and display string of the most recent command.
- **Result**
  - Defines enumerated constant values for several command result types.
- **RunMode**
  - Provides enumerated constants for a command running mode. This is currently interactive or scripted.
- **SelCommand**
  - For adding nestable whole object and subobject selection commands, derive your command from             SelCommand and override the abstract SelFilter and virtual SelSubObjectFilter functions.
- **Style**
  - Defines bitwise mask flags for different styles of commands, such as             Hidden or DoNotRepeat.
- **TransformCommand**
- **UndoRedoEventArgs**
