---
name: navisworks-autodesk-navisworks-api-plugins
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api.Plugins namespace — 49 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AddInLocation, AddInPlugin, AddInPluginAttribute, AddInPluginRecord, CallCanExecute, ClashResultActionPlugin, ClashResultActionPluginRecord, CommandAttribute, and 41 more types.
---

# Autodesk.Navisworks.Api.Plugins

Auto-generated from vendor docs for navisworks 2026.0. 49 types in this namespace.

## AddInLocation (enum)

Location of an AddInPlugin in the Navisworks menus system

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.AddInLocation)

### Values
- `None` = `0` — Do not display in the menus
- `AddIn` = `1` — Display in the Addin menu
- `Import` = `2` — Display in the Import menu
- `Export` = `3` — Display in the Export menu
- `Help` = `4` — Display in the Help menu
- `CurrentSelectionContextMenu` = `5` — Display in the Context menu for currently selected ModelItem
- `CurrentSelection2DContextMenu` = `6` — Display in the 2D Context menu for currently selected ModelItem

## AddInPlugin (class)

A plugin that can be called either using the GUI, ApplicationAutomation or using the Automation .NET API

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.AddInPlugin)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.CommandState CanExecute()`

This is called to determine the state of plugin. Primarily if Execute can be called.

**Returns:** `Autodesk.Navisworks.Api.Plugins.CommandState` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.AddInPlugin.CanExecute)

#### `int Execute(string[] parameters)`

This method is called when the plugin is executed

**Parameters:**
- `parameters` (string[])

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.AddInPlugin.Execute%28System.String%5B%5D%29)

#### `bool TryShowHelp()`

Called to tell the plugin to display help

**Returns:** `bool` — Return true if you have handled this call

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.AddInPlugin.TryShowHelp)

## AddInPluginAttribute (class)

Optional attribute for use with an AddInPlugin derived class. If this attribute is not used, the default location for an AddInPlugin is AddInLocation.AddIn

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.AddInPluginAttribute)

### Constructors
- `AddInPluginAttribute(Autodesk.Navisworks.Api.Plugins.AddInLocation location)` — Constructor AddInPluginAttribute, takes a single parameter giving the location of the Plugin in the Navisworks menus system

### Properties
- `CallCanExecute` (Autodesk.Navisworks.Api.Plugins.CallCanExecute, get/set) — Determines the conditions in which CanExecuteCommand should be called.
- `CanToggle` (bool, get/set) — Specifies if the command can toggle
- `Icon` (string, get/set) — The 16x16 icon to display for this command
- `LargeIcon` (string, get/set) — The 32x32 icon to display for this command
- `LoadForCanExecute` (bool, get/set) — Specifies if CanExecuteCommand should cause the Plugin to load
- `Location` (Autodesk.Navisworks.Api.Plugins.AddInLocation, get) — Determines where the plugin is displayed in Navisworks menus system
- `Shortcut` (string, get/set) — Specifies the keyboard shortcut for the command.
- `ShortcutWindowTypes` (string, get/set) — The Ids of the windows for which the shortcut is valid. Multiple Ids can be specified by separating with ";". If Ids are specified the shortcut should be valid in all contexts.

## AddInPluginRecord (class)

Represents an AddInPlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.AddInPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.CommandState CanExecute()`

This is called to determine the state of plugin. Primarily if Execute can be called.

**Returns:** `Autodesk.Navisworks.Api.Plugins.CommandState` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.AddInPluginRecord.CanExecute)

#### `int Execute(string[] parameters)`

Executes the plugin, loading it if needed

**Parameters:**
- `parameters` (string[])

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.AddInPluginRecord.Execute%28System.String%5B%5D%29)

#### `Autodesk.Navisworks.Api.Plugins.AddInPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.AddInPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.AddInPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.AddInPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.AddInPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.AddInPluginRecord.TryLoadPlugin)

### Properties
- `CanToggle` (bool, get) — Specifies if the command can toggle
- `Icon` (string, get) — The 16x16 icon to be displayed for this command
- `LargeIcon` (string, get) — The 32x32 icon to be displayed for this command
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.AddInPlugin, get) — Gets the plugin if loaded, otherwise null
- `Location` (Autodesk.Navisworks.Api.Plugins.AddInLocation, get) — Where the plugin is displayed in Navisworks menus system
- `Shortcut` (string, get) — Specifies the keyboard shortcut for the command.
- `ShortcutWindowTypes` (string, get) — The Ids of the windows for which the shortcut is valid. Multiple Ids can be specified by separating with ";". If Ids are specified the shortcut should be valid in all contexts.

## CallCanExecute (enum)

Determines logic used to determine if a command is Enabled

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CallCanExecute)

### Values
- `Always` = `0` — Command is Enabled unless specifically disabled by CanExecute implementation
- `DocumentNotClear` = `1` — Command is disabled if the doument is Clear
- `CurrentSelectionSingle` = `2` — Command is Disabled unless there is a single selection
- `CurrentSelectionMultiple` = `3` — Command is Disabled unless there are multiple selections

## ClashResultActionPlugin (class)

A plugin for implementing an action to apply to clash results.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.ClashResultActionPlugin)

### Methods
#### `void BeginExecute()`

BeginExecute method of ClashResultActionPlugin.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ClashResultActionPlugin.BeginExecute)

#### `bool CanExecute(Autodesk.Navisworks.Api.SavedItem item)`

Test whether the item is in a state that is appropriate to use this plugin.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ClashResultActionPlugin.CanExecute%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void EndExecute()`

EndExecute method of ClashResultActionPlugin.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ClashResultActionPlugin.EndExecute)

#### `bool ExecuteAction(Autodesk.Navisworks.Api.SavedItem item)`

ExecuteAction method of ClashResultActionPlugin.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ClashResultActionPlugin.ExecuteAction%28Autodesk.Navisworks.Api.SavedItem%29)

#### `bool SupportsMultipleItems()`

If false is returned then the plugin will only be available to be called when just a single item is selected in the clash UI Results list.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ClashResultActionPlugin.SupportsMultipleItems)

## ClashResultActionPluginRecord (class)

Represents a ClashResultActionPluginRecord record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.ClashResultActionPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.ClashResultActionPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.ClashResultActionPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ClashResultActionPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.ClashResultActionPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.ClashResultActionPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ClashResultActionPluginRecord.TryLoadPlugin)

### Properties
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.ClashResultActionPlugin, get) — Gets the plugin if loaded, otherwise null

## CommandAttribute (class)

Defines a new command in the GUI system

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CommandAttribute)

### Constructors
- `CommandAttribute(string name)` — Constructor

### Properties
- `CallCanExecute` (Autodesk.Navisworks.Api.Plugins.CallCanExecute, get/set) — Determines the conditions in which CanExecuteCommand should be called.
- `CanToggle` (bool, get/set) — Specifies if the command can toggle
- `DisplayName` (string, get/set) — The text to display for this command
- `ExtendedToolTip` (string, get/set) — The Extended Tooltip to display for this command
- `Icon` (string, get/set) — The 16x16 icon to display for this command
- `LargeIcon` (string, get/set) — The 32x32 icon to display for this command
- `LoadForCanExecute` (bool, get/set) — Specifies if CanExecuteCommand should cause the Plugin to load
- `Name` (string, get) — The name of the command
- `Shortcut` (string, get/set) — Specifies the keyboard shortcut for the command.
- `ShortcutWindowTypes` (string, get/set) — The Ids of the windows for which the shortcut is valid. Multiple Ids can be specified by separating with ";". If Ids are specified the shortcut should be valid in all contexts.
- `ToolTip` (string, get/set) — The Tooltip to display for this command

## CommandHandlerPlugin (class)

A plugin that can be used to add commands to the GUI

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CommandHandlerPlugin)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.CommandState CanExecuteCommand(string name)`

Called to determine if a command can be executed

**Parameters:**
- `name` (string) — The name of the command

**Returns:** `Autodesk.Navisworks.Api.Plugins.CommandState` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPlugin.CanExecuteCommand%28System.String%29)

#### `bool CanExecuteRibbonTab(string name)`

Called to determine if a ribbon tab can be executed

**Parameters:**
- `name` (string) — The name of the ribbon tab

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPlugin.CanExecuteRibbonTab%28System.String%29)

#### `int ExecuteCommand(string name, string[] parameters)`

Called when a command is executed

**Parameters:**
- `name` (string) — The name of the command
- `parameters` (string[]) — Paramaters to be passed to the command

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPlugin.ExecuteCommand%28System.String%2CSystem.String%5B%5D%29)

#### `bool TryShowCommandHelp(string name)`

Called to tell the plugin to display help

**Parameters:**
- `name` (string) — The name of the command

**Returns:** `bool` — Return true if you have handled this call

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPlugin.TryShowCommandHelp%28System.String%29)

## CommandHandlerPluginRecord (class)

Represents an CommandHandlerPlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.CommandState CanExecuteCommand(string commandId)`

Calls CanExecuteCommand on the plugin

**Parameters:**
- `commandId` (string) — The Id of the command

**Returns:** `Autodesk.Navisworks.Api.Plugins.CommandState` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.CanExecuteCommand%28System.String%29)

#### `bool CanExecuteRibbonTab(string ribbonId)`

Calls CanExecuteRibbonTab on the plugin

**Parameters:**
- `ribbonId` (string) — The Id of the ribbon tab

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.CanExecuteRibbonTab%28System.String%29)

#### `int ExecuteCommand(string commandId, string[] parameters)`

Calls ExecuteCommand on the plugin

**Parameters:**
- `commandId` (string) — The Id of the command
- `parameters` (string[]) — Paramaters to be passed to the command

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.ExecuteCommand%28System.String%2CSystem.String%5B%5D%29)

#### `string ExtractNameFromId(string id)`

Helper to extract a name from a Id.

**Parameters:**
- `id` (string)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.ExtractNameFromId%28System.String%29)

#### `Autodesk.Navisworks.Api.Plugins.CommandHandlerPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.CommandHandlerPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.LoadPlugin)

#### `string MakeIdFromName(string name)`

Helper to make an Id from a name

**Parameters:**
- `name` (string)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.MakeIdFromName%28System.String%29)

#### `bool ShowCommandHelp(string commandId)`

Calls TryShowCommandHelp on the plugin

**Parameters:**
- `commandId` (string) — The Id of the command

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.ShowCommandHelp%28System.String%29)

#### `Autodesk.Navisworks.Api.Plugins.CommandHandlerPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.CommandHandlerPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.TryLoadPlugin)

#### `bool TryShowCommandHelp(string commandId)`

Calls TryShowCommandHelp on the plugin

**Parameters:**
- `commandId` (string) — The Id of the command

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CommandHandlerPluginRecord.TryShowCommandHelp%28System.String%29)

### Properties
- `CommandRecords` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.CommandRecord>, get) — Commands defined by this plugin
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.CommandHandlerPlugin, get) — Gets the plugin if loaded, otherwise null
- `RibbonLayoutRecords` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.RibbonLayoutRecord>, get) — Xaml files that define the layout of the RibbonTabs and Commands
- `RibbonTabRecords` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.RibbonTabRecord>, get) — Ribbon Tabs defined by this plugin
- `ToolTipsRecords` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.ToolTipsRecord>, get) — ToolTips files that define tooltips for RibbonTabs and Commands

## CommandRecord (class)

Represents the definition of a command in the GUI system

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CommandRecord)

### Properties
- `CanToggle` (bool, get) — Specifies if the command can toggle
- `DisplayName` (string, get) — The text to be displayed for this command
- `ExtendedToolTip` (string, get) — The extended tooltip to be displayed for this command
- `Icon` (string, get) — The 16x16 icon to be displayed for this command
- `Id` (string, get) — The full Id of the command. This is a combination of and Plugin.Id
- `LargeIcon` (string, get) — The 32x32 icon to be displayed for this command
- `Shortcut` (string, get) — Specifies the keyboard shortcut for the command.
- `ShortcutWindowTypes` (string, get) — The Ids of the windows for which the shortcut is valid. Multiple Ids can be specified by separating with ";". If Ids are specified the shortcut should be valid in all contexts.
- `ToolTip` (string, get) — The tooltip to be displayed for this command

## CommandState (class)

Represents the state of a command in the GUI

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CommandState)

### Constructors
- `CommandState()` — Constructor
- `CommandState(bool enabled)` — Constructor

### Properties
- `IsChecked` (bool, get/set) — Specifies if the command should be checked
- `IsEnabled` (bool, get/set) — Specifies if the command should be enabled
- `IsVisible` (bool, get/set) — Specifies if the command should be visible
- `OverrideDisplayName` (string, get/set) — Specifies if the display name for the command should be overridden

## CustomPlugin (class)

A custom plugin is the most basic plugin that can be defined. It can be used when none of the specialized behaviour of other plugin tyes is required.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CustomPlugin)

## CustomPluginRecord (class)

Record representing a CustomPlugin

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.CustomPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.CustomPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.CustomPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CustomPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.CustomPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.CustomPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.CustomPluginRecord.TryLoadPlugin)

### Properties
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.CustomPlugin, get) — Gets the plugin if loaded, otherwise null

## DockPanePlugin (class)

A plugin that can be used to add a docking pane to the GUI.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.DockPanePlugin)

### Methods
#### `void ActivatePane()`

Activates the pane (i.e. makes it the window with keyboard focus), also making it visible first, if necessary.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.ActivatePane)

#### `System.Windows.Forms.Control CreateControlPane()`

Called to tell the plugin to create it's pane. The pane will be resized as specified by the DockPanePluginAttribute. Should be overriden in conjunction with DestroyControlPane.

**Returns:** `System.Windows.Forms.Control` — A Control that contains the pane content

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.CreateControlPane)

#### `System.Windows.Forms.IWin32Window CreateHWndPane(System.Windows.Forms.IWin32Window parent)`

Called to tell the plugin to create it's pane. The pane will be resized as specified by the DockPanePluginAttribute. Should be overriden in conjunction with DestroyHWndPane.

**Parameters:**
- `parent` (System.Windows.Forms.IWin32Window) — The pane should be created using this window as it's parent

**Returns:** `System.Windows.Forms.IWin32Window` — The handle of window that contains the pane content

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.CreateHWndPane%28System.Windows.Forms.IWin32Window%29)

#### `void DestroyControlPane(System.Windows.Forms.Control pane)`

Called to tell the plugin to destroy it's pane Should be overriden in conjunction with CreateControlPane.

**Parameters:**
- `pane` (System.Windows.Forms.Control) — The pane to destroy

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.DestroyControlPane%28System.Windows.Forms.Control%29)

#### `void DestroyHWndPane(System.Windows.Forms.IWin32Window pane)`

Called to tell the plugin to destroy it's pane. Should be overriden in conjunction with CreateHWndPane.

**Parameters:**
- `pane` (System.Windows.Forms.IWin32Window) — The pane to destroy

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.DestroyHWndPane%28System.Windows.Forms.IWin32Window%29)

#### `void OnActivePaneChanged(bool isActive)`

Called when the active pane changes

**Parameters:**
- `isActive` (bool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.OnActivePaneChanged%28System.Boolean%29)

#### `void OnVisibleChanged()`

Called when the visibility of the pane changes

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.OnVisibleChanged)

#### `bool TryShowHelp()`

Called to tell the plugin to display help

**Returns:** `bool` — Return true if you have handled this call

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.TryShowHelp)

#### `bool TryShowHelpAtScreenPoint(int x, int y)`

Called to tell the plugin to display help at a screen point

**Parameters:**
- `x` (int)
- `y` (int)

**Returns:** `bool` — Return true if you have handled this call

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.TryShowHelpAtScreenPoint%28System.Int32%2CSystem.Int32%29)

#### `bool TryShowHelpForHighlight()`

Called to tell the plugin to display help for highlight

**Returns:** `bool` — Return true if you have handled this call

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePlugin.TryShowHelpForHighlight)

### Properties
- `Visible` (bool, get/set) — The visibility of the pane

## DockPanePluginAttribute (class)

Optional attribute for use with an DockPanePlugin derived class. If this attribute is not used, the default values are the same as using a DockPanePluginAttribute(100,100)

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.DockPanePluginAttribute)

### Constructors
- `DockPanePluginAttribute(int preferredWidth, int preferredHeight)` — Constructor DockPanePluginAttribute, takes a pair of values that specify the prefered size for the pane. Default values are: MinimumWidth=0, MinimumHeight=0, AutoScroll=true, FixedSize=true.

### Properties
- `AutoScroll` (bool, get/set) — Determines if scroll bars should appear when the pane gets bellow it's minimum size
- `FixedSize` (bool, get/set) — Determines if the pane should be resizable
- `MinimumHeight` (int, get/set) — Minimum height of pane, 0 means no preference
- `MinimumWidth` (int, get/set) — Minimum width of pane, 0 means no preference
- `PreferredHeight` (int, get) — Preferred height of pane, 0 means no preference
- `PreferredWidth` (int, get) — Preferred width of pane, 0 means no preference

## DockPanePluginRecord (class)

Represents an DockPanePlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.DockPanePluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.DockPanePlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.DockPanePlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.DockPanePlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.DockPanePlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.DockPanePluginRecord.TryLoadPlugin)

### Properties
- `AutoScroll` (bool, get) — Determines if scroll bars should appear when the pane gets bellow it's minimum size
- `FixedSize` (bool, get) — Determines if the pane should be resizable
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.DockPanePlugin, get) — Gets the plugin if loaded, otherwise null
- `MinimumHeight` (int, get) — Minimum height of pane, 0 means no preference
- `MinimumWidth` (int, get) — Minimum width of pane, 0 means no preference
- `PreferredHeight` (int, get) — Preferred height of pane, 0 means no preference
- `PreferredWidth` (int, get) — Preferred width of pane, 0 means no preference

## EventWatcherPlugin (class)

Differs fom other plugin types in that the plugin is not delay loaded. A typical implementation will subscribe to some API events in OnLoaded and unsubscribe in OnUnloading.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.EventWatcherPlugin)

### Methods
#### `void OnLoaded()`

Called once immediately after the plugin has been loaded (created). Default implementation does nothing. This is a good place to subscribe to API events.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.EventWatcherPlugin.OnLoaded)

#### `void OnUnloading()`

Called once just before the plugin is unloaded. If the plugin implements IDisposable, then Dispose will be called immediately after the plugin is unloaded. Plugins are typically unloaded at application shutdown or when the end user has disabled a plugin. Default implementation does nothing. This is a good place to unsubscribe from API events.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.EventWatcherPlugin.OnUnloading)

## EventWatcherPluginRecord (class)

Record representing a EventWatcherPlugin

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.EventWatcherPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.EventWatcherPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.EventWatcherPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.EventWatcherPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.EventWatcherPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.EventWatcherPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.EventWatcherPluginRecord.TryLoadPlugin)

### Properties
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.EventWatcherPlugin, get) — Gets the plugin if loaded, otherwise null

## ExecuteDisabledException (class)

The exception that is thrown when you try to call Execute on a plugin who's CanExecute method states that it should be disabled.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.ExecuteDisabledException)

### Constructors
- `ExecuteDisabledException()` — Creates a PluginLoadException
- `ExecuteDisabledException(string message)` — Creates a PluginLoadException
- `ExecuteDisabledException(string message, System.Exception innerException)` — Creates a PluginLoadException

## FileMetadata (class)

Metadata about a file or URI.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.FileMetadata)

### Constructors
- `FileMetadata()` — Default constructor.

### Methods
#### `static Autodesk.Navisworks.Api.Plugins.FileMetadata InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of FileMetadata.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Plugins.FileMetadata` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileMetadata.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of FileMetadata.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileMetadata.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `HasModificationTime` (bool, get) — Is the ModificationTime property valid?
- `HasRevisionId` (bool, get) — Is the RevisionId property valid?
- `HasSize` (bool, get) — Is the Size property valid?
- `ModificationTime` (System.DateTime, get/set) — The modification time of the URI.
- `RevisionId` (string, get/set) — The revision id of the URI.
- `Size` (ulong, get/set) — The size in bytes of the URI.

## FileProtocolHandle (class)

A file handle opened via the FileProtocolPlugin.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.FileProtocolHandle)

### Methods
#### `bool Close()`

Closes an open file handle.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolHandle.Close)

#### `bool Read(in byte[] data, ulong offset, ulong count)`

Reads data from a file.

**Parameters:**
- `data` (byte[])
- `offset` (ulong)
- `count` (ulong)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolHandle.Read%28System.Byte%5B%5D%2CSystem.UInt64%2CSystem.UInt64%29)

#### `bool Write(byte[] data, ulong offset, ulong count)`

Writes data to a file.

**Parameters:**
- `data` (byte[])
- `offset` (ulong)
- `count` (ulong)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolHandle.Write%28System.Byte%5B%5D%2CSystem.UInt64%2CSystem.UInt64%29)

## FileProtocolOpenMode (enum)

Open mode for FileProtocolPlugin interface.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.FileProtocolOpenMode)

### Values
- `Read` = `0` — Open for read
- `Write` = `1` — Open for write

## FileProtocolPlugin (class)

A plugin for implementing a custom file protocol.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.FileProtocolPlugin)

### Methods
#### `bool GetFile(System.Uri uri, string localPath, Autodesk.Navisworks.Api.Progress progress)`

Downloads a remote URI to a local path.

**Parameters:**
- `uri` (System.Uri)
- `localPath` (string)
- `progress` (Autodesk.Navisworks.Api.Progress)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.GetFile%28System.Uri%2CSystem.String%2CAutodesk.Navisworks.Api.Progress%29)

#### `bool GetFileInfo(System.Uri uri, Autodesk.Navisworks.Api.Plugins.FileMetadata info)`

Returns information about a URI.

**Parameters:**
- `uri` (System.Uri)
- `info` (Autodesk.Navisworks.Api.Plugins.FileMetadata)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.GetFileInfo%28System.Uri%2CAutodesk.Navisworks.Api.Plugins.FileMetadata%29)

#### `bool GetFileToCache(System.Uri uri, Autodesk.Navisworks.Api.Progress progress, out string cachePath)`

Downloads a remote URI to the protocols internal cache.

**Parameters:**
- `uri` (System.Uri)
- `progress` (Autodesk.Navisworks.Api.Progress)
- `cachePath` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.GetFileToCache%28System.Uri%2CAutodesk.Navisworks.Api.Progress%2CSystem.String%40%29)

#### `Autodesk.Navisworks.Api.Plugins.FileProtocolHandle OpenFile(System.Uri uri, Autodesk.Navisworks.Api.Plugins.FileProtocolOpenMode mode)`

Directly opens a URI.

**Parameters:**
- `uri` (System.Uri)
- `mode` (Autodesk.Navisworks.Api.Plugins.FileProtocolOpenMode)

**Returns:** `Autodesk.Navisworks.Api.Plugins.FileProtocolHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.OpenFile%28System.Uri%2CAutodesk.Navisworks.Api.Plugins.FileProtocolOpenMode%29)

#### `bool PutFile(string localPath, System.Uri uri, Autodesk.Navisworks.Api.Progress progress)`

Uploads a file to a remote URI.

**Parameters:**
- `localPath` (string)
- `uri` (System.Uri)
- `progress` (Autodesk.Navisworks.Api.Progress)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.PutFile%28System.String%2CSystem.Uri%2CAutodesk.Navisworks.Api.Progress%29)

#### `bool TryGetDisplayName(System.Uri uri, out string displayName)`

Returns the normal display name for a given URI.

**Parameters:**
- `uri` (System.Uri)
- `displayName` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.TryGetDisplayName%28System.Uri%2CSystem.String%40%29)

#### `bool TryGetMRUPath(System.Uri logicalUri, out string mruPath)`

Returns the path to store in the MRU list for a given URI.

**Parameters:**
- `logicalUri` (System.Uri)
- `mruPath` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.TryGetMRUPath%28System.Uri%2CSystem.String%40%29)

#### `bool TryGetPhysicalPath(System.Uri logicalUri, out string physicalPath)`

Returns the verbose display name for a given URI.

**Parameters:**
- `logicalUri` (System.Uri)
- `physicalPath` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.TryGetPhysicalPath%28System.Uri%2CSystem.String%40%29)

#### `bool TryGetVerboseDisplayName(System.Uri uri, out string displayName)`

Returns the verbose display name for a given URI.

**Parameters:**
- `uri` (System.Uri)
- `displayName` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.TryGetVerboseDisplayName%28System.Uri%2CSystem.String%40%29)

#### `bool TryResolveAbsolute(string originalParent, string originalChild, string resolvedParent, out string resolvedChild)`

Called to resolve an absolute URI.

**Parameters:**
- `originalParent` (string)
- `originalChild` (string)
- `resolvedParent` (string)
- `resolvedChild` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.TryResolveAbsolute%28System.String%2CSystem.String%2CSystem.String%2CSystem.String%40%29)

#### `bool TryResolveRelative(string resolvedParent, string relativeChild, out string resolvedChild)`

Called to resolve an relative URI.

**Parameters:**
- `resolvedParent` (string)
- `relativeChild` (string)
- `resolvedChild` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPlugin.TryResolveRelative%28System.String%2CSystem.String%2CSystem.String%40%29)

### Properties
- `IsRemote` (bool, get) — Does this protocol access remote data?
- `SupportedSchemes` (System.Collections.Generic.IEnumerable<string>, get) — List of schemes that this protocol supports.
- `SupportsDirectOpen` (bool, get) — Does this protocol support direct opening of a file?
- `SupportsGetPut` (bool, get) — Does this protocol support the GetFile and PutFile methods?
- `SupportsOwnCache` (bool, get) — Does this protocol support it's own cache?

## FileProtocolPluginRecord (class)

Represents a FileProtocolPlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.FileProtocolPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.FileProtocolPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.FileProtocolPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.FileProtocolPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.FileProtocolPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.FileProtocolPluginRecord.TryLoadPlugin)

### Properties
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.FileProtocolPlugin, get) — Gets the plugin if loaded, otherwise null

## HomeScreenError (class)

HomeScreenError class in Autodesk.Navisworks.Api.Plugins.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.HomeScreenError)

### Constructors
- `HomeScreenError()` — Constructs a new HomeScreenError.

### Properties
- `ErrorStatus` (Autodesk.Navisworks.Api.Plugins.HomeScreenErrorStatus, get/set) — ErrorStatus property of HomeScreenError.
- `HttpStatusCode` (int, get/set) — HttpStatusCode property of HomeScreenError.

## HomeScreenErrorStatus (enum)

HomeScreenErrorStatus enum in Autodesk.Navisworks.Api.Plugins.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.HomeScreenErrorStatus)

### Values
- `Unknown` = `0`
- `CertificateCommonNameIsIncorrect` = `1`
- `CertificateExpired` = `2`
- `ClientCertificateContainsErrors` = `3`
- `CertificateRevoked` = `4`
- `CertificateIsInvalid` = `5`
- `ServerUnreachable` = `6`
- `Timeout` = `7`
- `ErrorHttpInvalidServerResponse` = `8`
- `ConnectionAborted` = `9`
- `ConnectionReset` = `10`
- `Disconnected` = `11`
- `CannotConnect` = `12`
- `HostNameNotResolved` = `13`
- `OperationCanceled` = `14`
- `RedirectFailed` = `15`
- `UnexpectedError` = `16`
- `ValidAuthenticationCredentialsRequired` = `17`
- `ValidProxyAuthenticationRequired` = `18`

## HomeScreenExtensionPlugin (class)

A plugin for implementing a home screen extension

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.HomeScreenExtensionPlugin)

### Methods
#### `string GetErrorHtml(Autodesk.Navisworks.Api.Plugins.HomeScreenError error)`

The HTML to load in the iframe if the FrameUrl cannot be loaded.

**Parameters:**
- `error` (Autodesk.Navisworks.Api.Plugins.HomeScreenError)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.HomeScreenExtensionPlugin.GetErrorHtml%28Autodesk.Navisworks.Api.Plugins.HomeScreenError%29)

#### `string[] GetHostObjectOrigins()`

Returns the host object origins.

**Returns:** `string[]` — The host object origins

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.HomeScreenExtensionPlugin.GetHostObjectOrigins)

#### `void SetHostObjectOrigins(string[] hostObjectOrigins)`

Sets the host object origins. The host object is only accessible if the iframe's origin matches one of the provided origins (for security reasons).

**Parameters:**
- `hostObjectOrigins` (string[])

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.HomeScreenExtensionPlugin.SetHostObjectOrigins%28System.String%5B%5D%29)

### Properties
- `FrameUrl` (System.Uri, get) — The URL for the iframe to display.
- `HostObject` (object, get) — The host object for the iframe.
- `HostObjectName` (string, get) — The name to access HostObject.

## HomeScreenExtensionPluginRecord (class)

Represents a HomeScreenExtensionPlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.HomeScreenExtensionPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.HomeScreenExtensionPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.HomeScreenExtensionPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.HomeScreenExtensionPluginRecord.LoadPlugin)

## InputPlugin (class)

A plugin that can respond to mouse and keyboard inputs.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.InputPlugin)

### Methods
#### `bool ContextMenu(Autodesk.Navisworks.Api.View view, int x, int y)`

ContextMenu handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `x` (int)
- `y` (int)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.ContextMenu%28Autodesk.Navisworks.Api.View%2CSystem.Int32%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.Cursor GetCursor(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier)`

GetCursor handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)

**Returns:** `Autodesk.Navisworks.Api.Cursor` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.GetCursor%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%29)

#### `Autodesk.Navisworks.Api.HelpIdResult GetHelpIdAtPoint(Autodesk.Navisworks.Api.View view, int x, int y)`

GetHelpIdAtPoint handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `x` (int)
- `y` (int)

**Returns:** `Autodesk.Navisworks.Api.HelpIdResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.GetHelpIdAtPoint%28Autodesk.Navisworks.Api.View%2CSystem.Int32%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.HelpIdResult GetHelpIdForHighlight(Autodesk.Navisworks.Api.View view)`

GetHelpIdForHighlight handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)

**Returns:** `Autodesk.Navisworks.Api.HelpIdResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.GetHelpIdForHighlight%28Autodesk.Navisworks.Api.View%29)

#### `Autodesk.Navisworks.Api.TooltipResult GetTooltip(Autodesk.Navisworks.Api.View view, int x, int y)`

GetTooltip handler

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `x` (int)
- `y` (int)

**Returns:** `Autodesk.Navisworks.Api.TooltipResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.GetTooltip%28Autodesk.Navisworks.Api.View%2CSystem.Int32%2CSystem.Int32%29)

#### `bool KeyDown(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, ushort key, double timeOffset)`

MouseLeave handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `key` (ushort)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.KeyDown%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Double%29)

#### `bool KeyDrag(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, double timeOffset)`

KeyDrag handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.KeyDrag%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Double%29)

#### `bool KeyUp(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, ushort key, double timeOffset)`

KeyUp handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `key` (ushort)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.KeyUp%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Double%29)

#### `bool ModifierKeyDown(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, double timeOffset)`

ModifierKeyDown handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.ModifierKeyDown%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Double%29)

#### `bool ModifierKeyUp(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, double timeOffset)`

ModifierKeyUp handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.ModifierKeyUp%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Double%29)

#### `bool MouseDown(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, ushort button, int x, int y, double timeOffset)`

Mouse down handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `button` (ushort)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.MouseDown%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `bool MouseDrag(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, int x, int y, double timeOffset)`

Mouse drag handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.MouseDrag%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `bool MouseLeave(Autodesk.Navisworks.Api.View view, double timeOffset)`

Mouse leave handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.MouseLeave%28Autodesk.Navisworks.Api.View%2CSystem.Double%29)

#### `bool MouseMove(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, int x, int y, double timeOffset)`

Mouse move handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.MouseMove%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `bool MouseUp(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, ushort button, int x, int y, double timeOffset)`

Mouse up handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `button` (ushort)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.MouseUp%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `bool WheelDrag(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, int x, int y, ushort wheel, double len, double timeOffset)`

WheelDrag handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `x` (int)
- `y` (int)
- `wheel` (ushort)
- `len` (double)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPlugin.WheelDrag%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Int32%2CSystem.Int32%2CSystem.UInt16%2CSystem.Double%2CSystem.Double%29)

## InputPluginRecord (class)

Represents an InputPlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.InputPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.InputPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.InputPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.InputPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.InputPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.InputPluginRecord.TryLoadPlugin)

### Properties
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.InputPlugin, get) — Gets the plugin if loaded, otherwise null

## InterfaceAttribute (class)

Attribute that declares a plugin implements a particular interface or behaviours.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.InterfaceAttribute)

### Constructors
- `InterfaceAttribute(string name, string developerId)` — Attribute that declares a plugin implements particular interface or behaviour. The combination of Name and DeveloperId is used to form an Id that identifies which interface is being supported.

### Properties
- `DeveloperId` (string, get) — The 4 character ADN developer code or a GUID. The combination of this and Name is used to form and Id that identifies which interface is being supported.
- `DisplayName` (string, get/set) — Optional additional information about the interface implementation
- `Name` (string, get) — The name of the Inteface. The combination of this and DeveloperId is used to form an Id that identifies which interface is being supported.
- `UserData` (string, get/set) — Optional additional information about the interface implementation

## InterfaceRecord (class)

Record that declares a plugin implements a particular interface or behaviour.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.InterfaceRecord)

### Properties
- `DeveloperId` (string, get) — The 4 character ADN developer code or a GUID. The combination of this and Name is used to form Id that identifies which interface is being supported.
- `DisplayName` (string, get) — Optional additional information about the interface implementation
- `Id` (string, get) — The full identifier of the interface being supported. This is formed as Name.DeveloperId
- `Name` (string, get) — The name of the Inteface. The combination of this and DeveloperId is used to form Id that identifies which interface is being supported.
- `UserData` (string, get) — Optional additional information about the interface implementation

## Plugin (class)

Base class for all plugins

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.Plugin)

### Methods
#### `string GetString(string name)`

Looks up a localizable string that is specific to this plugin

**Parameters:**
- `name` (string) — The identifier of the string to lookup

**Returns:** `string` — Returns localized string

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.Plugin.GetString%28System.String%29)

#### `string GetStringSafe(string name)`

Looks up a localizable string that is specific to this plugin

**Parameters:**
- `name` (string) — The identifier of the string to lookup

**Returns:** `string` — Returns localized string or value of 'name' if it cannot be found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.Plugin.GetStringSafe%28System.String%29)

#### `string TryGetString(string name)`

Looks up a localizable string that is specific to this plugin

**Parameters:**
- `name` (string) — The identifier of the string to lookup

**Returns:** `string` — Returns localized string or Null if it cannot be found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.Plugin.TryGetString%28System.String%29)

### Properties
- `DeveloperId` (string, get) — The 4 character ADN developer code or a GUID. The combination of this and Name should make the plugin unique
- `Id` (string, get) — The full identifier for the plugin. This is formed as Name.DeveloperId
- `Name` (string, get) — The name of the Plugin. The combination of this and DeveloperId should make the plugin unique
- `PluginRecord` (Autodesk.Navisworks.Api.Plugins.PluginRecord, get) — The PluginRecord that provides access to the attribute defined properties for this plugin.

## PluginAttribute (class)

The standard attribute which must be applied to all plug-ins.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.PluginAttribute)

### Constructors
- `PluginAttribute(string name, string developerId)` — The standard attribute constructor which must be applied to all plug-ins. The combination of Name and DeveloperId should make the plugin unique

### Properties
- `DeveloperId` (string, get) — The 4 character ADN developer code or a GUID. The combination of this and Name should make the plugin unique
- `DisplayName` (string, get/set) — Text to display in the Navisworks GUI
- `ExtendedToolTip` (string, get/set) — The extended tooltip to display when the Plugin is highlighted in the Navisworks GUI
- `Name` (string, get) — The name of the Plugin. The combination of this and DeveloperId should make the plugin unique
- `Options` (Autodesk.Navisworks.Api.Plugins.PluginOptions, get/set) — Options that modify how a plugin behaves
- `SupportsIsSelfEnabled` (bool, get/set) — Determines if Plugin.IsSelfEnabled will be called.
- `ToolTip` (string, get/set) — The tooltip to display when the Plugin is highlighted in the Navisworks GUI

## PluginLoadException (class)

The exception that is thrown when a plugin fails to load

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.PluginLoadException)

### Constructors
- `PluginLoadException()` — Creates a PluginLoadException
- `PluginLoadException(string message)` — Creates a PluginLoadException
- `PluginLoadException(string message, System.Exception innerException)` — Creates a PluginLoadException

## PluginOptions (enum)

Options that modify how a plugin behaves

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.PluginOptions)

### Values
- `None` = `0` — No additional options
- `SupportsControls` = `1` — Supports being used in a control (see Autodesk.Navisworks.Api.Controls)

## PluginRecord (class)

Metadata for a plugin that can be loaded into Navisworks. The information in PluginRecord is available even if the corresponding Plugin hasn’t been loaded.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.PluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.InterfaceRecord FindInterfaceRecord(string interfaceId)`

Finds an InterfaceRecord

**Parameters:**
- `interfaceId` (string) — The Id of the InterfaceRecord

**Returns:** `Autodesk.Navisworks.Api.Plugins.InterfaceRecord` — null if the InterfaceRecord cannot be found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.PluginRecord.FindInterfaceRecord%28System.String%29)

#### `Autodesk.Navisworks.Api.Plugins.Plugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.Plugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.PluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.Plugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.Plugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.PluginRecord.TryLoadPlugin)

### Properties
- `DeveloperId` (string, get) — Gets the 4 character ADN developer code or a GUID. The combination of this and Name should make the plugin unique
- `DisplayName` (string, get) — Text to display in the Navisworks GUI.
- `ExtendedToolTip` (string, get) — Extended Tooltip to display when the Plugin is highlighted in the Navisworks GUI
- `HasAttemptedCreate` (bool, get) — Determines if an attempt has been made to create the Plugin.
- `HasFailedCreate` (bool, get) — Determines if an attempt has been made to create the plugin, and it has failed.
- `HasFailedIsSelfEnabled` (bool, get) — Where SupportsIsSelfEnabled is true then the plugin will be created early and IsSelfEnabled will be called to determine the result. Will return false if the plugin could not be created to make the call.
- `Id` (string, get) — The full identifier for the plugin. This is formed as Name.DeveloperId
- `InterfaceRecords` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.InterfaceRecord>, get) — Returns a collection of InterfaceRecords that declare which interfaces we support
- `IsEnabled` (bool, get) — Gets whether the Plugin is currently enabled for use or not.
- `IsLoaded` (bool, get) — Gets whether the Plugin is currently loaded
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.Plugin, get) — Gets the plugin if loaded, otherwise null
- `Name` (string, get) — Gets the name of the Plugin. The combination of this and DeveloperId should make the plugin unique
- `PluginOptions` (Autodesk.Navisworks.Api.Plugins.PluginOptions, get) — Gets flags for how plugins run
- `SupportsIsSelfEnabled` (bool, get) — Determines if IsSelfEnabled will be called.
- `ToolTip` (string, get) — Tooltip to display when the Plugin is highlighted in the Navisworks GUI

## RenderPlugin (class)

A plugin that can perform custom rendering.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.RenderPlugin)

### Methods
#### `Autodesk.Navisworks.Api.BoundingBox3D MakeRenderBoundingBox(Autodesk.Navisworks.Api.View viewer)`

Should return bounding box of any additional model space geometry that will be rendered so that near/far clipping planes can be adjusted accordingly.

**Parameters:**
- `viewer` (Autodesk.Navisworks.Api.View)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — Bounding box

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.RenderPlugin.MakeRenderBoundingBox%28Autodesk.Navisworks.Api.View%29)

#### `void OverlayRender(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.Graphics graphics)`

Override to allow custom drawing overlayed on main render.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `graphics` (Autodesk.Navisworks.Api.Graphics)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.RenderPlugin.OverlayRender%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.Graphics%29)

#### `void Render(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.Graphics graphics)`

Override to allow custom drawing in main render.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `graphics` (Autodesk.Navisworks.Api.Graphics)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.RenderPlugin.Render%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.Graphics%29)

## RenderPluginRecord (class)

Represents an RenderPlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.RenderPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.RenderPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.RenderPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.RenderPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.RenderPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.RenderPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.RenderPluginRecord.TryLoadPlugin)

### Properties
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.RenderPlugin, get) — Gets the plugin if loaded, otherwise null

## RibbonLayoutAttribute (class)

Defines a new ribbon layout in the GUI system

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.RibbonLayoutAttribute)

### Constructors
- `RibbonLayoutAttribute(string xaml)` — Constructor

### Properties
- `Xaml` (string, get) — The Xaml file that specifies the Ribbon tab and commands

## RibbonLayoutRecord (class)

Represents the definition of a ribbon layout in the GUI system

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.RibbonLayoutRecord)

### Properties
- `Xaml` (string, get) — The path to the Xaml file that specifies the layout of ribbon and commands

## RibbonTabAttribute (class)

Defines a ribbon tab in the GUI system.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.RibbonTabAttribute)

### Constructors
- `RibbonTabAttribute(string name)` — Constructor

### Properties
- `CallCanExecute` (Autodesk.Navisworks.Api.Plugins.CallCanExecute, get/set) — Determines the conditions in which CanExecuteRibbonTab should be called.
- `DisplayName` (string, get/set) — The text to display for this Ribbon tab
- `LoadForCanExecute` (bool, get/set) — Specifies if CanExecuteRibbonTab should cause the Plugin to load
- `Name` (string, get) — The name of the Ribbon tab

## RibbonTabRecord (class)

Represents the definition of a ribbon tab in the GUI system.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.RibbonTabRecord)

### Properties
- `DisplayName` (string, get) — The text to be displayed for this Ribbon tab
- `Id` (string, get) — The full Id of the ribbon tab. This is a combination of and Plugin.Id

## StringsAttribute (class)

Attribute which causes a string localization file to be loaded for a particular plug-in.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.StringsAttribute)

### Constructors
- `StringsAttribute(string fileName)` — Attribute causes a string localization file to be loaded for a particular plug-in.

### Properties
- `FileName` (string, get) — The filename containing the strings.

## ToolPlugin (class)

A plugin that provides the behaviour of a custom tool.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.ToolPlugin)

### Methods
#### `bool ContextMenu(Autodesk.Navisworks.Api.View view, int x, int y)`

ContextMenu handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `x` (int)
- `y` (int)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.ContextMenu%28Autodesk.Navisworks.Api.View%2CSystem.Int32%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.Cursor GetCursor(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier)`

GetCursor handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)

**Returns:** `Autodesk.Navisworks.Api.Cursor` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.GetCursor%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%29)

#### `Autodesk.Navisworks.Api.HelpIdResult GetHelpIdAtPoint(Autodesk.Navisworks.Api.View view, int x, int y)`

GetHelpIdAtPoint handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `x` (int)
- `y` (int)

**Returns:** `Autodesk.Navisworks.Api.HelpIdResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.GetHelpIdAtPoint%28Autodesk.Navisworks.Api.View%2CSystem.Int32%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.HelpIdResult GetHelpIdForHighlight(Autodesk.Navisworks.Api.View view)`

GetHelpIdForHighlight handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)

**Returns:** `Autodesk.Navisworks.Api.HelpIdResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.GetHelpIdForHighlight%28Autodesk.Navisworks.Api.View%29)

#### `Autodesk.Navisworks.Api.TooltipResult GetTooltip(Autodesk.Navisworks.Api.View view, int x, int y)`

GetTooltip handler

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `x` (int)
- `y` (int)

**Returns:** `Autodesk.Navisworks.Api.TooltipResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.GetTooltip%28Autodesk.Navisworks.Api.View%2CSystem.Int32%2CSystem.Int32%29)

#### `bool KeyDown(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, ushort key, double timeOffset)`

MouseLeave handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `key` (ushort)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.KeyDown%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Double%29)

#### `bool KeyDrag(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, double timeOffset)`

KeyDrag handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.KeyDrag%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Double%29)

#### `bool KeyUp(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, ushort key, double timeOffset)`

KeyUp handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `key` (ushort)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.KeyUp%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Double%29)

#### `Autodesk.Navisworks.Api.BoundingBox3D MakeRenderBoundingBox(Autodesk.Navisworks.Api.View viewer)`

Should return bounding box of any additional model space geometry that will be rendered so that near/far clipping planes can be adjusted accordingly.

**Parameters:**
- `viewer` (Autodesk.Navisworks.Api.View)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — Bounding box

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.MakeRenderBoundingBox%28Autodesk.Navisworks.Api.View%29)

#### `bool ModifierKeyDown(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, double timeOffset)`

ModifierKeyDown handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.ModifierKeyDown%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Double%29)

#### `bool ModifierKeyUp(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, double timeOffset)`

ModifierKeyUp handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.ModifierKeyUp%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Double%29)

#### `bool MouseDown(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, ushort button, int x, int y, double timeOffset)`

Mouse down handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `button` (ushort)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.MouseDown%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `bool MouseDrag(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, int x, int y, double timeOffset)`

Mouse drag handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.MouseDrag%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `bool MouseLeave(Autodesk.Navisworks.Api.View view, double timeOffset)`

Mouse leave handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.MouseLeave%28Autodesk.Navisworks.Api.View%2CSystem.Double%29)

#### `bool MouseMove(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, int x, int y, double timeOffset)`

Mouse move handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.MouseMove%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `bool MouseUp(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifiers, ushort button, int x, int y, double timeOffset)`

Mouse up handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifiers` (Autodesk.Navisworks.Api.KeyModifiers)
- `button` (ushort)
- `x` (int)
- `y` (int)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.MouseUp%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.UInt16%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%29)

#### `void OverlayRender(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.Graphics graphics)`

Override to allow custom drawing overlayed on main render.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `graphics` (Autodesk.Navisworks.Api.Graphics)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.OverlayRender%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.Graphics%29)

#### `void Render(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.Graphics graphics)`

Override to allow custom drawing in main render.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `graphics` (Autodesk.Navisworks.Api.Graphics)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.Render%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.Graphics%29)

#### `bool WheelDrag(Autodesk.Navisworks.Api.View view, Autodesk.Navisworks.Api.KeyModifiers modifier, int x, int y, ushort wheel, double len, double timeOffset)`

WheelDrag handler.

**Parameters:**
- `view` (Autodesk.Navisworks.Api.View)
- `modifier` (Autodesk.Navisworks.Api.KeyModifiers)
- `x` (int)
- `y` (int)
- `wheel` (ushort)
- `len` (double)
- `timeOffset` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPlugin.WheelDrag%28Autodesk.Navisworks.Api.View%2CAutodesk.Navisworks.Api.KeyModifiers%2CSystem.Int32%2CSystem.Int32%2CSystem.UInt16%2CSystem.Double%2CSystem.Double%29)

## ToolPluginRecord (class)

Represents an ToolPlugin record in Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.ToolPluginRecord)

### Methods
#### `Autodesk.Navisworks.Api.Plugins.ToolPlugin LoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.ToolPlugin` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPluginRecord.LoadPlugin)

#### `Autodesk.Navisworks.Api.Plugins.ToolPlugin TryLoadPlugin()`

Returns the plugin, loads it if required

**Returns:** `Autodesk.Navisworks.Api.Plugins.ToolPlugin` — null if plugin failed to load

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Plugins.ToolPluginRecord.TryLoadPlugin)

### Properties
- `LoadedPlugin` (Autodesk.Navisworks.Api.Plugins.ToolPlugin, get) — Gets the plugin if loaded, otherwise null

## ToolTipsAttribute (class)

Defines tooltips for ribbon tabs and commands in the GUI system.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.ToolTipsAttribute)

### Constructors
- `ToolTipsAttribute(string xaml)` — Constructor

### Properties
- `Xaml` (string, get) — The file that specifies the tooltips for Ribbon tab and commands

## ToolTipsRecord (class)

Represents the definition of tooltips for ribbon tabs and commands in the GUI system.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Plugins.ToolTipsRecord)

### Properties
- `Xaml` (string, get) — The path to the file that specifies the tool tips for the ribbon and commands

