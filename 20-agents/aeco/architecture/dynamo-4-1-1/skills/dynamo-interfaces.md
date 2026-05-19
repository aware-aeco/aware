---
name: dynamo-dynamo-interfaces
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Interfaces namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: IPathResolver, IPathManager, IPreferences, IRenderPrecisionPreference, BackgroundPreviewActiveState, IDynamoModel.
---

# Dynamo.Interfaces

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## BackgroundPreviewActiveState (class)

Represents data about active state of preview background

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPreferences.cs)

### Constructors
- `void BackgroundPreviewActiveState()` — Initializes a new instance of class

### Properties
- `IsActive` (bool, get/set) — Flag which indicates if background preview is active
- `Name` (string, get/set) — Name of background preview

## IDynamoModel (interface)

Interface contains definitions for core model of Dynamo.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Interfaces/IDynamoModel.cs)

### Properties
- `CurrentWorkspace` (Dynamo.Graph.Workspaces.WorkspaceModel, get) — Represents the current workspace.
- `Workspaces` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Workspaces.WorkspaceModel>, get) — Represents the Workspaces in Dynamo.

### Events
#### `EvaluationCompleted` (`System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs>`)

**Signature:** `public event System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs> EvaluationCompleted`

This event is fired when graph evaluation is completed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Interfaces/IDynamoModel.cs)

#### `ShutdownStarted` (`Dynamo.Models.DynamoModelHandler`)

**Signature:** `public event Dynamo.Models.DynamoModelHandler ShutdownStarted`

This event is fired when Dynamo is shutting down.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Interfaces/IDynamoModel.cs)

#### `WorkspaceAdded` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceAdded`

This event is fired when a new workspace is added.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Interfaces/IDynamoModel.cs)

#### `WorkspaceCleared` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceCleared`

This event is fired when workspace is cleared.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Interfaces/IDynamoModel.cs)

#### `WorkspaceOpening` (`System.Action<object>`)

**Signature:** `public event System.Action<object> WorkspaceOpening`

This event is fired when a workspace is opened.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Interfaces/IDynamoModel.cs)

#### `WorkspaceRemoved` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceRemoved`

This event is fired when a workspace is removed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Interfaces/IDynamoModel.cs)

## IPathManager (interface)

This interface provides the most common paths. E.g. core directory, package directory etc.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPathResolver.cs)

### Methods
#### `void AddResolutionPath(string path)`

Call this method to add additional path for consideration when path resolution take place.

**Parameters:**
- `path` (string) — The full path to be considered when PathManager attempts to resolve a file path. If this argument does not represent a valid directory path, this method throws an exception.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPathResolver.cs)

#### `bool ResolveDocumentPath(ref string document)`

Given an initial RTF document file name, this method returns the absolute path of the file, if one exists.

**Parameters:**
- `document` (ref string) — The name of the RTF file. This argument cannot be null or empty.

**Returns:** `bool` — Returns true if the requested document can be located, or false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPathResolver.cs)

#### `bool ResolveLibraryPath(ref string library)`

Given an initial file path with the file name, resolve the full path to the target file.

**Parameters:**
- `library` (ref string) — The initial library file path. This argument can optionally include the full path with a target file name. If a full path is given and it represents an invalid file path, the file name will be searched for in additional resolution paths.

**Returns:** `bool` — Returns true if the requested file can be located, or false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPathResolver.cs)

### Properties
- `BackupDirectory` (string, get) — The directory where the automatically saved files will be stored.
- `CommonDataDirectory` (string, get) — The local directory that contains common data files among users.
- `CommonDefinitions` (string, get) — The local directory that contains custom nodes created by all users.
- `DefaultPackagesDirectory` (string, get) — The default directory for saving packages downloaded through the package manager. This directory is specific to the current user.
- `DefaultUserDefinitions` (string, get) — The default directory that contains custom nodes created by the user.
- `DefinitionDirectories` (System.Collections.Generic.IEnumerable<string>, get) — Directories from where custom nodes are to be loaded. The implementor of this interface method should always guarantee that a non-empty list is returned, and that the first entry represents the default custom node directory. Custom nodes created are stored in the default directory, which is specific to the current user. Changes to custom nodes may or may not be saved to their current location depending on write access.
- `DynamoCoreDirectory` (string, get) — The directory in which DynamoCore.dll is guaranteed to be found.
- `ExtensionsDirectories` (System.Collections.Generic.IEnumerable<string>, get) — The directories, which contains ExtensionDefinition .xml files
- `HostApplicationDirectory` (string, get) — The directory in which the host application such as DynamoRevit or DynamoStudio could be found.
- `LogDirectory` (string, get) — The local directory where log files are generated. This directory is specific to the current user.
- `MajorFileVersion` (int, get) — Major version of assembly file
- `MinorFileVersion` (int, get) — Minor version of assembly file
- `NodeDirectories` (System.Collections.Generic.IEnumerable<string>, get) — Folders in which node assemblies can be located.
- `PackagesDirectories` (System.Collections.Generic.IEnumerable<string>, get) — Directories from where packages are to be loaded. The implementor of this interface method should always guarantee that a non-empty list is returned, and that the first entry represents the default package directory. Packages downloaded through package manager are stored in the default package directory, which is specific to the current user.
- `PathResolver` (Dynamo.Interfaces.IPathResolver, get) — Integration specific PathResolver
- `PreferenceFilePath` (string, get) — Full path to the preference xml file. This setting file is specific to the current user.
- `PreloadedLibraries` (System.Collections.Generic.IEnumerable<string>, get) — A list of node assembly names to be preloaded with Dynamo.
- `PythonTemplateFilePath` (string, get) — Full path to the python template py file. This setting file is specific to the current user.
- `SamplesDirectory` (string, get) — The root directory where all sample files are stored. This directory is common to all users on the machine.
- `TemplatesDirectory` (string, get) — The root directory where all template files are stored
- `UserDataDirectory` (string, get) — The local directory that contains user specific data files.
- `ViewExtensionsDirectories` (System.Collections.Generic.IEnumerable<string>, get) — The directories, which contains ViewExtensionDefinition.xml files

## IPathResolver (interface)

This interface provides paths to external assemblies and node directories.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPathResolver.cs)

### Methods
#### `System.Collections.Generic.IEnumerable<string> GetDynamoUserDataLocations()`

Returns a list of user data folders on this system.

**Returns:** `System.Collections.Generic.IEnumerable<string>` — The implementation of this interface method should return a list of user data folders, one for each of Dynamo product installed on the system. When there is no Dynamo product installed, this method returns an empty list.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPathResolver.cs)

### Properties
- `AdditionalNodeDirectories` (System.Collections.Generic.IEnumerable<string>, get) — Additional directories in which node assemblies can be located. The return value of this property should never be null. Each entry must represent a valid directory, otherwise DirectoryNotFoundException exception is thrown.
- `AdditionalResolutionPaths` (System.Collections.Generic.IEnumerable<string>, get) — Additional directories that should be considered when path resolution is done for a library that does not contain full path information. The return value of this property should never be null. Each entry must represent a valid directory, otherwise DirectoryNotFoundException exception is thrown.
- `CommonDataRootFolder` (string, get) — This property represents the root folder where application common data files (i.e. shared among all users on the same machine) are stored. If this property returns a null or empty string, then PathManager falls back to using "%AppData%\Dynamo\Dynamo Core". If this property returns a string that does not represent an existing folder, PathManager will attempt to create a new directory. If the property does not represent a valid path string, an exception will be thrown by the underlying system IO API invoked. Note that this path should not include the version number as it will be appended by PathManager.
- `PreloadedLibraryPaths` (System.Collections.Generic.IEnumerable<string>, get) — Libraries to be preloaded as part of Dynamo start up sequence. Each entry in this list can either represent full path to a library, or just the assembly name. If absolute path information is not supplied, the library will be looked up through both predefined and additional resolution paths. The return value of this property should never be null.
- `UserDataRootFolder` (string, get) — This property represents the root folder where user specific data files are stored. If this property returns a null or empty string, then PathManager falls back to using "%ProgramData%\Dynamo\Dynamo Core". If this property returns a string that does not represent an existing folder, PathManager will attempt to create a new directory. If the property does not represent a valid path string, an exception will be thrown by the underlying system IO API invoked. Note that this path should not include the version number as it will be appended by PathManager.

## IPreferences (interface)

An interface which defines preference settings.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPreferences.cs)

### Methods
#### `bool GetIsBackgroundPreviewActive(string name)`

Returns active state of specified background preview

**Parameters:**
- `name` (string) — Background preview name

**Returns:** `bool` — The active state

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPreferences.cs)

#### `bool Save(string filePath)`

Call this method to serialize PreferenceSettings given the output file path.

**Parameters:**
- `filePath` (string) — The full path of the output file to serialize PreferenceSettings to.

**Returns:** `bool` — Returns true if the serialization is successful, or false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPreferences.cs)

#### `void SetIsBackgroundPreviewActive(string name, bool value)`

Sets active state of specified background preview

**Parameters:**
- `name` (string) — Background preview name
- `value` (bool) — Active state to set

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPreferences.cs)

### Properties
- `BackgroundPreviews` (System.Collections.Generic.List<Dynamo.Interfaces.BackgroundPreviewActiveState>, get/set) — Collection of pairs [BackgroundPreviewName;isActive]
- `BackupFiles` (System.Collections.Generic.List<string>, get/set) — Returns list of backup files
- `CollapseToMinSize` (bool, get/set) — Indicates if the groups should be collapsed by default.
- `ConnectorType` (Dynamo.Graph.Connectors.ConnectorType, get/set) — Indicates which type of connector's should be displayed on canvas. I.e. bezier or polyline
- `ConsoleHeight` (int, get/set) — Returns height of console
- `CustomPackageFolders` (System.Collections.Generic.List<string>, get/set) — Returns list of folders containing zero-touch nodes and custom nodes.
- `GraphChecksumItemsList` (System.Collections.Generic.List<Dynamo.Configuration.GraphChecksumItem>, get/set) — IPreferences.GraphChecksumItemsList
- `GridScaleFactor` (float, get/set) — Sets the scale of the background grid element. Default is 1.0.
- `IsADPAnalyticsReportingApproved` (bool, get/set) — Indicates whether ADP analytics reporting is approved or not.
- `IsBackgroundGridVisible` (bool, get/set) — Indicates whether background grid is visible or not.
- `IsFirstRun` (bool, get/set) — Indicates first run
- `MaxNumRecentFiles` (int, get/set) — Returns maximal count of recent files which can be displayed
- `NumberFormat` (string, get/set) — Returns the decimal precision used to display numbers.
- `OptionalInPortsCollapsed` (bool, get/set) — Indicates if the optional input ports are hidden by default.
- `PackageDirectoriesToUninstall` (System.Collections.Generic.List<string>, get/set) — Returns list of packages used by the Package Manager to determine which packages are scheduled to be deleted or scheduled to be unloaded.
- `PythonTemplateFilePath` (string, get/set) — Return full path to the Python (.py) file to use as a starting template when creating a new PythonScript Node.
- `RecentFiles` (System.Collections.Generic.List<string>, get/set) — Returns list of recent files
- `ShowConnector` (bool, get/set) — Indicates whether connector should be displayed on canvas or not.
- `ShowDefaultGroupDescription` (bool, get/set) — Indicates if groups should display the default description.
- `ShowEdges` (bool, get/set) — Indicates whether surface and solid edges will be rendered.
- `ShowPreviewBubbles` (bool, get/set) — Indicates if preview bubbles should be displayed on nodes.
- `TemplateFilePath` (string, get/set) — Return full path of the template directory with template file(s) to use as a starting template when creating a new graph from a template.
- `UnconnectedOutPortsCollapsed` (bool, get/set) — Indicates if the unconnected output ports are hidden by default.
- `WindowH` (double, get/set) — Returns the last height of the Dynamo window.
- `WindowW` (double, get/set) — Returns the last width of the Dynamo window.
- `WindowX` (double, get/set) — Returns the last X coordinate of the Dynamo window.
- `WindowY` (double, get/set) — Returns the last Y coordinate of the Dynamo window.

## IRenderPrecisionPreference (interface)

Temporary interface to avoid breaking changes. TODO: Merge with IPreferences for 3.0 (DYN-1699)

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/IPreferences.cs)

### Properties
- `RenderPrecision` (int, get/set) — Indicate which render precision will be used

