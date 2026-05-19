---
name: rhino-rhino-commands
description: This skill encodes the rhino 8.0 surface of the Rhino.Commands namespace — 12 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Command, CommandEventArgs, CustomUndoEventArgs, CommandStyleAttribute, MostRecentCommandDescription, SelCommand, UndoRedoEventArgs, TransformCommand, and 4 more types.
---

# Rhino.Commands

Auto-generated from vendor docs for rhino 8.0. 12 types in this namespace.

## Command (class)

Defines a base class for all commands. This class is abstract.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_Command.htm)

### Constructors
- `protected Command()` — Default protected constructor. It only allows instantiation through sub-classing.

### Methods
#### `public static void DisplayHelp(Guid commandId)`

Displays help for a command.

**Parameters:**
- `commandId` (System.Guid) — A command ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_DisplayHelp.htm)

#### `public static string GetCommandContextHelpUrl(Guid commandId)`

Returns a command's context help url.

**Parameters:**
- `commandId` (System.Guid) — A command ID.

**Returns:** `String` — The command's context url if provided.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_GetCommandContextHelpUrl.htm)

#### `public static string[] GetCommandNames(bool english, bool loaded)`

Gets list of command names in Rhino. This list does not include Test, Alpha, or System commands.

**Parameters:**
- `english` (System.Boolean) — if true, retrieve the English name for every command. if false, retrieve the local name for every command.
- `loaded` (System.Boolean) — if true, only get names of currently loaded commands. if false, get names of all registered (may not be currently loaded) commands.

**Returns:** `String[]` — An array instance with command names. This array could be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_GetCommandNames.htm)

#### `public static Guid[] GetCommandStack()`

Determines if Rhino is currently running a command. Because Rhino allow for transparent commands (commands that can be run from inside of other commands), this method returns the total ids of active commands.

**Returns:** `Guid[]` — Ids of running commands or null if no commands are currently running. The "active" command is at the end of this list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_GetCommandStack.htm)

#### `public static MostRecentCommandDescription[] GetMostRecentCommands()`

Gets an array of most recent command descriptions.

**Returns:** `MostRecentCommandDescription[]` — An array of command descriptions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_GetMostRecentCommands.htm)

#### `public static bool InCommand()`

Determines if Rhino is currently running a command.

**Returns:** `Boolean` — true if a command is currently running, false if no commands are currently running.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_InCommand.htm)

#### `public static bool InScriptRunnerCommand()`

This is a low level tool to determine if Rhino is currently running a script running command like "ReadCommandFile" or the RhinoScript plug-in's "RunScript".

**Returns:** `Boolean` — true if a script running command is active.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_InScriptRunnerCommand.htm)

#### `public static bool IsCommand(string name)`

Determines is a string is a command.

**Parameters:**
- `name` (System.String) — A string.

**Returns:** `Boolean` — true if the string is a command.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_IsCommand.htm)

#### `public static bool IsValidCommandName(string name)`

Determines if a string is a valid command name.

**Parameters:**
- `name` (System.String) — A string.

**Returns:** `Boolean` — true if the string is a valid command name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_IsValidCommandName.htm)

#### `public static Guid LookupCommandId(string name, bool searchForEnglishName)`

Returns the ID of a command.

**Parameters:**
- `name` (System.String) — The name of the command.
- `searchForEnglishName` (System.Boolean) — true if the name is to searched in English. This ensures that a '_' is prepended to the name.

**Returns:** `Guid` — An of the command, or Empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_LookupCommandId.htm)

#### `public static string LookupCommandName(Guid commandId, bool englishName)`

Returns the command name given a command ID.

**Parameters:**
- `commandId` (System.Guid) — A command ID.
- `englishName` (System.Boolean) — true if the requested command is in English.

**Returns:** `String` — The command name, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_LookupCommandName.htm)

#### `protected virtual void OnHelp()`

Is called when the user needs assistance with this command.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_OnHelp.htm)

#### `protected virtual bool ReplayHistory(ReplayHistoryData replayData)`

Repeats an operation of a command. In order to make this function work, you will likely need to grab the Result property that gives the list of input objects. Then, you will be able to replace these inputs by using one of the UpdateToX() methods of the ReplayHistoryResult.You should NOT use any document AddX() or ReplaceX() functions, as they will break history.

**Parameters:**
- `replayData` (Rhino.DocObjects.ReplayHistoryData) — The replay history information.

**Returns:** `Boolean` — true if the operation succeeded. The default implementation always returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_ReplayHistory.htm)

#### `protected abstract Result RunCommand(RhinoDoc doc, RunMode mode)`

Executes the command.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The current document.
- `mode` (Rhino.Commands.RunMode) — The command running mode.

**Returns:** `Result` — The command result code.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_RunCommand.htm)

#### `public static void RunProxyCommand(Command.RunCommandDelegate commandCallback, RhinoDoc doc, Object data)`

Execute some code as if it were running in a command

**Parameters:**
- `commandCallback` (Rhino.Commands.Command.RunCommandDelegate) — [Missing <param name="commandCallback"/> documentation for "M:Rhino.Commands.Command.RunProxyCommand(Rhino.Commands.Command.RunCommandDelegate,Rhino.RhinoDoc,System.Object)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Commands.Command.RunProxyCommand(Rhino.Commands.Command.RunCommandDelegate,Rhino.RhinoDoc,System.Object)"]
- `data` (System.Object) — optional extra data to pass to callback

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_RunProxyCommand.htm)

### Properties
- `CommandContextHelpUrl` (String, get) — Gets the URL of the command contextual help. This is usually a location of a local CHM file. The default implementation return an empty string.
- `EnglishName` (String, get) — Gets the name of the command. This method is abstract.
- `Id` (Guid, get) — Gets the unique ID of this command. It is best to use a Guid attribute for each custom derived command class since this will keep the id consistent between sessions of Rhino GuidAttribute
- `LastCommandId` (Guid, get) — Gets the ID of the last commands.
- `LastCommandResult` (Result, get) — Gets the result code of the last command.
- `LocalName` (String, get) — Gets the local name of the command.
- `PlugIn` (PlugIn, get) — Gets the plug-in where this commands is placed.
- `Settings` (PersistentSettings, get) — Gets the settings of the command.

### Events
#### `BeginCommand` (`System.EventHandler<CommandEventArgs>`)

**Signature:** `public static event EventHandler<CommandEventArgs> BeginCommand`

Called just before command.RunCommand().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_Commands_Command_BeginCommand.htm)

#### `EndCommand` (`System.EventHandler<CommandEventArgs>`)

**Signature:** `public static event EventHandler<CommandEventArgs> EndCommand`

Called immediately after command.RunCommand().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_Commands_Command_EndCommand.htm)

#### `UndoRedo` (`System.EventHandler<UndoRedoEventArgs>`)

**Signature:** `public static event EventHandler<UndoRedoEventArgs> UndoRedo`

Used to monitor Rhino's built in undo/redo support.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_Commands_Command_UndoRedo.htm)

## Command.RunCommandDelegate (delegate)

[Missing <summary> documentation for "T:Rhino.Commands.Command.RunCommandDelegate"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_Command_RunCommandDelegate.htm)

## CommandEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Commands.CommandEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_CommandEventArgs.htm)

### Properties
- `CommandEnglishName` (String, get) — Gets the English name of the command that raised this event.
- `CommandHelpURL` (String, get) — Gets the help url of the command that raised this event.
- `CommandId` (Guid, get) — Gets the ID of the command that raised this event.
- `CommandLocalName` (String, get) — Gets the name of the command that raised this event in the local language.
- `CommandPluginName` (String, get) — Gets the name of the plug-in that this command belongs to. If the command is internal to Rhino, then this property is an empty string.
- `CommandResult` (Result, get) — Gets the result of the command that raised this event. This value is only meaningful during EndCommand events.
- `Document` (RhinoDoc, get) — 
- `DocumentRuntimeSerialNumber` (UInt32, get) — 

## CommandStyleAttribute (class)

Decorates commands to provide styles.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_CommandStyleAttribute.htm)

### Constructors
- `public CommandStyleAttribute(Style styles)` — Initializes a new command style attribute class.

### Properties
- `Styles` (Style, get) — Gets the associated style.

## CustomUndoEventArgs (class)

Argument package that is passed to a custom undo delegate

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_CustomUndoEventArgs.htm)

### Properties
- `ActionDescription` (String, get) — 
- `CommandId` (Guid, get) — 
- `CreatedByRedo` (Boolean, get) — 
- `Document` (RhinoDoc, get) — 
- `Tag` (Object, get) — 
- `UndoSerialNumber` (UInt32, get) — 

## MostRecentCommandDescription (class)

Stores the macro and display string of the most recent command.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_MostRecentCommandDescription.htm)

### Constructors
- `public MostRecentCommandDescription()` — Initializes a new instance of the MostRecentCommandDescription class

### Properties
- `DisplayString` (String, get/set) — 
- `Macro` (String, get/set) — 

## Result (enum)

Defines enumerated constant values for several command result types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_Result.htm)

### Values
- `Success` = `0` — Command worked.
- `Cancel` = `1` — User canceled command.
- `Nothing` = `2` — Command did nothing but cancel was not pressed.
- `Failure` = `3` — Command failed (bad input, computational problem, etc.)
- `UnknownCommand` = `4` — Command not found (user probably had a typo in command name).
- `CancelModelessDialog` = `5` — Commands canceled and modeless dialog.
- `ExitRhino` = `268435455` — exit RhinoCommon.

## RunMode (enum)

Provides enumerated constants for a command running mode. This is currently interactive or scripted.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_RunMode.htm)

### Values
- `Interactive` = `0` — Can use dialogs for input. Must use message boxes to report serious error conditions.
- `Scripted` = `1` — All input must come from command line, GetPoint, GetObject, GetString, etc. Must use message boxes to report serious error conditions. Script mode gets used when a command is run with a hyphen (-) prefix.

## SelCommand (class)

For adding nestable whole object and subobject selection commands, derive your command from SelCommand and override the abstract SelFilter and virtual SelSubObjectFilter functions.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_SelCommand.htm)

### Constructors
- `protected SelCommand()` — Initializes a new instance of the SelCommand class

### Methods
#### `protected virtual void OnHelp()`

Is called when the user needs assistance with this command.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_OnHelp.htm)

#### `protected virtual bool ReplayHistory(ReplayHistoryData replayData)`

Repeats an operation of a command. In order to make this function work, you will likely need to grab the Result property that gives the list of input objects. Then, you will be able to replace these inputs by using one of the UpdateToX() methods of the ReplayHistoryResult.You should NOT use any document AddX() or ReplaceX() functions, as they will break history.

**Parameters:**
- `replayData` (Rhino.DocObjects.ReplayHistoryData) — The replay history information.

**Returns:** `Boolean` — true if the operation succeeded. The default implementation always returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_ReplayHistory.htm)

#### `protected override Result RunCommand(RhinoDoc doc, RunMode mode)`

(Overrides Command.RunCommand(RhinoDoc, RunMode).)

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Commands.SelCommand.RunCommand(Rhino.RhinoDoc,Rhino.Commands.RunMode)"]
- `mode` (Rhino.Commands.RunMode) — [Missing <param name="mode"/> documentation for "M:Rhino.Commands.SelCommand.RunCommand(Rhino.RhinoDoc,Rhino.Commands.RunMode)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Commands.SelCommand.RunCommand(Rhino.RhinoDoc,Rhino.Commands.RunMode)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_SelCommand_RunCommand.htm)

#### `protected abstract bool SelFilter(RhinoObject rhObj)`

Override this abstract function and return true if object should be selected.

**Parameters:**
- `rhObj` (Rhino.DocObjects.RhinoObject) — The object to check regarding selection status.

**Returns:** `Boolean` — true if the object should be selected; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_SelCommand_SelFilter.htm)

#### `protected virtual bool SelSubObjectFilter(RhinoObject rhObj, List<ComponentIndex> indicesToSelect)`

To select subobjects, override this virtual function, add component indices of the subobjects that should get selected to indicesToSelect list and return true. This is called only if the SelFilter returns false and the whole object does not get selected.

**Parameters:**
- `rhObj` (Rhino.DocObjects.RhinoObject) — The object to check regarding selection status.
- `indicesToSelect` (System.Collections.Generic.List<ComponentIndex>) — The component indices of the subobjects to select.

**Returns:** `Boolean` — true if components added to indicesToSelect should get selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_SelCommand_SelSubObjectFilter.htm)

### Properties
- `BeQuiet` (Boolean, get/set) — 
- `CommandContextHelpUrl` (String, get) — Gets the URL of the command contextual help. This is usually a location of a local CHM file. The default implementation return an empty string.
- `EnglishName` (String, get) — Gets the name of the command. This method is abstract.
- `Id` (Guid, get) — Gets the unique ID of this command. It is best to use a Guid attribute for each custom derived command class since this will keep the id consistent between sessions of Rhino GuidAttribute
- `LocalName` (String, get) — Gets the local name of the command.
- `PlugIn` (PlugIn, get) — Gets the plug-in where this commands is placed.
- `Settings` (PersistentSettings, get) — Gets the settings of the command.
- `TestGrips` (Boolean, get/set) — 
- `TestLights` (Boolean, get/set) — 

## Style (enum)

Defines bitwise mask flags for different styles of commands, such as Hidden or DoNotRepeat.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_Style.htm)

### Values
- `None` = `0` — No flag is defined.
- `Hidden` = `1` — Also known as a "test" command. The command name does not auto-complete when typed on the command line an is therefore not discoverable. Useful for writing commands that users don't normally have access to.
- `ScriptRunner` = `2` — For commands that want to run scripts as if they were typed at the command line (like RhinoScript's RunScript command)
- `Transparent` = `4` — Transparent commands can be run inside of other commands. The command does not modify the contents of the model's geometry in any way. Examples of transparent commands include commands that change views and toggle snap states. Any command that adds or deletes, a view cannot be transparent.
- `DoNotRepeat` = `8` — The command should not be repeated by pressing "ENTER" immediately after the command finishes.
- `NotUndoable` = `16` — By default, all commands are undo-able.
- `System` = `32` — reserved

## TransformCommand (class)

[Missing <summary> documentation for "T:Rhino.Commands.TransformCommand"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_TransformCommand.htm)

### Constructors
- `protected TransformCommand()` — Initializes a new instance of the TransformCommand class

### Methods
#### `protected void DuplicateObjects(TransformObjectList list)`



**Parameters:**
- `list` (Rhino.Collections.TransformObjectList) — [Missing <param name="list"/> documentation for "M:Rhino.Commands.TransformCommand.DuplicateObjects(Rhino.Collections.TransformObjectList)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_TransformCommand_DuplicateObjects.htm)

#### `protected virtual void OnHelp()`

Is called when the user needs assistance with this command.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_OnHelp.htm)

#### `protected virtual bool ReplayHistory(ReplayHistoryData replayData)`

Repeats an operation of a command. In order to make this function work, you will likely need to grab the Result property that gives the list of input objects. Then, you will be able to replace these inputs by using one of the UpdateToX() methods of the ReplayHistoryResult.You should NOT use any document AddX() or ReplaceX() functions, as they will break history.

**Parameters:**
- `replayData` (Rhino.DocObjects.ReplayHistoryData) — The replay history information.

**Returns:** `Boolean` — true if the operation succeeded. The default implementation always returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_ReplayHistory.htm)

#### `protected void ResetGrips(TransformObjectList list)`

Sets dynamic grip locations back to starting grip locations. This makes things like the Copy command work when grips are "copied".

**Parameters:**
- `list` (Rhino.Collections.TransformObjectList) — A list of object to transform. This is a special list type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_TransformCommand_ResetGrips.htm)

#### `protected abstract Result RunCommand(RhinoDoc doc, RunMode mode)`

Executes the command.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The current document.
- `mode` (Rhino.Commands.RunMode) — The command running mode.

**Returns:** `Result` — The command result code.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_Command_RunCommand.htm)

#### `protected Result SelectObjects(string prompt, ObjectType filter, TransformObjectList list)`

Selects objects within the command.

**Parameters:**
- `prompt` (System.String) — The selection prompt.
- `filter` (Rhino.DocObjects.ObjectType) — Geometry filter to limit selection. Use function above if you do not need specific types.
- `list` (Rhino.Collections.TransformObjectList) — A list of objects to transform. This is a special list type.

**Returns:** `Result` — The operation result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_TransformCommand_SelectObjects_1.htm)

#### `protected Result SelectObjects(string prompt, TransformObjectList list)`

Selects objects within the command.

**Parameters:**
- `prompt` (System.String) — The selection prompt.
- `list` (Rhino.Collections.TransformObjectList) — A list of objects to transform. This is a special list type.

**Returns:** `Result` — The operation result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_TransformCommand_SelectObjects.htm)

#### `protected void TransformObjects(TransformObjectList list, Transform xform, bool copy, bool autoHistory)`



**Parameters:**
- `list` (Rhino.Collections.TransformObjectList) — [Missing <param name="list"/> documentation for "M:Rhino.Commands.TransformCommand.TransformObjects(Rhino.Collections.TransformObjectList,Rhino.Geometry.Transform,System.Boolean,System.Boolean)"]
- `xform` (Rhino.Geometry.Transform) — [Missing <param name="xform"/> documentation for "M:Rhino.Commands.TransformCommand.TransformObjects(Rhino.Collections.TransformObjectList,Rhino.Geometry.Transform,System.Boolean,System.Boolean)"]
- `copy` (System.Boolean) — [Missing <param name="copy"/> documentation for "M:Rhino.Commands.TransformCommand.TransformObjects(Rhino.Collections.TransformObjectList,Rhino.Geometry.Transform,System.Boolean,System.Boolean)"]
- `autoHistory` (System.Boolean) — [Missing <param name="autoHistory"/> documentation for "M:Rhino.Commands.TransformCommand.TransformObjects(Rhino.Collections.TransformObjectList,Rhino.Geometry.Transform,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Commands_TransformCommand_TransformObjects.htm)

### Properties
- `CommandContextHelpUrl` (String, get) — Gets the URL of the command contextual help. This is usually a location of a local CHM file. The default implementation return an empty string.
- `EnglishName` (String, get) — Gets the name of the command. This method is abstract.
- `Id` (Guid, get) — Gets the unique ID of this command. It is best to use a Guid attribute for each custom derived command class since this will keep the id consistent between sessions of Rhino GuidAttribute
- `LocalName` (String, get) — Gets the local name of the command.
- `PlugIn` (PlugIn, get) — Gets the plug-in where this commands is placed.
- `Settings` (PersistentSettings, get) — Gets the settings of the command.

## UndoRedoEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Commands.UndoRedoEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Commands_UndoRedoEventArgs.htm)

### Properties
- `CommandId` (Guid, get) — 
- `IsBeforeBeginRecording` (Boolean, get) — 
- `IsBeforeEndRecording` (Boolean, get) — 
- `IsBeginRecording` (Boolean, get) — 
- `IsBeginRedo` (Boolean, get) — 
- `IsBeginUndo` (Boolean, get) — 
- `IsEndRecording` (Boolean, get) — 
- `IsEndRedo` (Boolean, get) — 
- `IsEndUndo` (Boolean, get) — 
- `IsPurgeRecord` (Boolean, get) — 
- `UndoSerialNumber` (UInt32, get) — 

