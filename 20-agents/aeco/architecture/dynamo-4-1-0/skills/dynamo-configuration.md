---
name: dynamo-dynamo-configuration
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Configuration namespace — 13 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Configurations, Context, DebugSettings, DynamoPlayerFolder, DynamoPlayerFolderGroup, GraphChecksumItem, GroupStyleItem, PreferenceSettings, and 5 more types.
---

# Dynamo.Configuration

Auto-generated from vendor docs for dynamo 4.1.0. 13 types in this namespace.

## Configurations (class)

This class contains properties that are used in Dynamo.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/Configurations.cs)

### Constructors
- `void Configurations()` — Configurations.Configurations

### Properties
- `SupportedLocaleDic` (System.Collections.Generic.Dictionary<string, string>, get) — Supported languages and locales as a dictionary in the current thread locale

## Context (static-class)

This specifies the Dynamo Context (RunTime). ex : Revit 2014, Revit 2015 etc.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/Context.cs)

## DebugSettings (class)

This class is used for setting debug settings through Dynamo UI. E.g. turn on/off logging; show/hide compiled node values.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/DebugSettings.cs)

### Constructors
- `void DebugSettings()` — DebugSettings.DebugSettings

## DynamoPlayerFolder (class)

This class describes a folder (usually containing Dynamo graphs) added to the Dynamo Player or Generative Design

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/DynamoPlayerFolder.cs)

### Constructors
- `void DynamoPlayerFolder()` — DynamoPlayerFolder.DynamoPlayerFolder

### Properties
- `DisplayName` (string, get/set) — The display name of the folder
- `Id` (string, get/set) — The ID of the folder
- `IsDefault` (bool, get/set) — Is the folder path the default directory of the list
- `IsRemovable` (bool, get/set) — Is this folder removable from the settings (Built-in folders are non-removable)
- `IsValid` (bool, get/set) — Is the folder path a valid path
- `Order` (int, get/set) — The order of the folder
- `Path` (string, get/set) — The full path of the folder

## DynamoPlayerFolderGroup (class)

This class defines a group of folders associated with a Dynamo Player or Generative Design entry point.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/DynamoPlayerFolder.cs)

### Constructors
- `void DynamoPlayerFolderGroup()` — DynamoPlayerFolderGroup.DynamoPlayerFolderGroup

### Properties
- `EntryPoint` (string, get/set) — The name of the Player entry point
- `Folders` (System.Collections.Generic.List<Dynamo.Configuration.DynamoPlayerFolder>, get/set) — The List of Folder Items for this group

## GraphChecksumItem (class)

Represents the stringified version of the nodes connections from a graph

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/GraphChecksumItem.cs)

### Constructors
- `void GraphChecksumItem()` — GraphChecksumItem.GraphChecksumItem

### Properties
- `Checksum` (string, get/set) — GraphChecksumItem.Checksum
- `GraphId` (string, get/set) — GraphChecksumItem.GraphId

## GroupStyleItem (class)

Group specific style item Note: This class does not contain special property yet comparing to base class

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/GroupStyleItem.cs)

### Constructors
- `void GroupStyleItem()` — GroupStyleItem.GroupStyleItem

## PreferenceSettings (class)

PreferenceSettings is a class for GUI to persist certain settings. Upon running of the GUI, those settings that are persistent will be loaded from a XML file from DYNAMO_SETTINGS_FILE. When GUI is closed, the settings are saved back into the XML file.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

### Constructors
- `void PreferenceSettings()` — Initializes a new instance of the Dynamo.Configuration.PreferenceSettings class.

### Methods
#### `Dynamo.Configuration.PreferenceSettings.AskForTrustedLocationResult AskForTrustedLocation(bool isOpenedFile, bool isFileInTrustedLocation, bool isHomeSpace, bool isShowStartPage, bool isDisableTrustWarnings)`

AskForTrustedLocation function

**Parameters:**
- `isOpenedFile` (bool) — 
- `isFileInTrustedLocation` (bool) — 
- `isHomeSpace` (bool) — 
- `isShowStartPage` (bool) — 
- `isDisableTrustWarnings` (bool) — 

**Returns:** `Dynamo.Configuration.PreferenceSettings.AskForTrustedLocationResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `bool GetIsBackgroundPreviewActive(string name)`

Returns active state of specified background preview

**Parameters:**
- `name` (string) — Background preview name

**Returns:** `bool` — The active state

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `string GetPythonTemplateFilePath()`

Returns the static Python template file path. When the file exists and is not empty, its contents are used to populate new Python Script nodes added to the Dynamo workspace.

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `bool IsTrustedLocation(string location)`

Checkes whether the input argument (path) is among Dynamo's trusted locations Only directories are supported. Subdirectories of a trusted directory are considered trusted.

**Parameters:**
- `location` (string) — An absolute path to a folder or file on disk

**Returns:** `bool` — True if the path is a trusted location, false otherwise

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `Dynamo.Configuration.PreferenceSettings Load(string filePath)`

Loads PreferenceSettings from specified XML file if possible, else initializes PreferenceSettings with default values.

**Parameters:**
- `filePath` (string) — Path of the XML File

**Returns:** `Dynamo.Configuration.PreferenceSettings` — Stored PreferenceSettings from xml file or Default PreferenceSettings if xml file is not found.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `Dynamo.Configuration.PreferenceSettings LoadContent(string content)`

Loads PreferenceSettings from specified XML file if possible, else initializes PreferenceSettings with default values.

**Parameters:**
- `content` (string) — The content of the xml file

**Returns:** `Dynamo.Configuration.PreferenceSettings` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `bool Save(string filePath)`

Saves PreferenceSettings to XML, given a file path.

**Parameters:**
- `filePath` (string) — Path of the XML File to save to.

**Returns:** `bool` — True if file is saved successfully, false if an error occurred.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `bool SaveInternal(string preferenceFilePath)`

Saves PreferenceSettings in a default directory when no path is specified.

**Parameters:**
- `preferenceFilePath` (string) — The file path to save preference settings to. If this parameter is null or empty string, preference settings will be saved to the default path.

**Returns:** `bool` — True if file is saved successfully, false if an error occurred.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `void SetIsBackgroundPreviewActive(string name, bool value)`

Sets active state of specified background preview

**Parameters:**
- `name` (string) — Background preview name
- `value` (bool) — Active state

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

#### `System.Collections.Generic.List<string> StaticFields()`

List of static Fields to be excluded for evaluation due to their access level

**Returns:** `System.Collections.Generic.List<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

### Properties
- `BackgroundPreviews` (System.Collections.Generic.List<Dynamo.Interfaces.BackgroundPreviewActiveState>, get/set) — Collection of pairs [BackgroundPreviewName;isActive]
- `BackupFiles` (System.Collections.Generic.List<string>, get/set) — A list of backup file paths.
- `BackupFilesCount` (int, get/set) — This defines how many files will be backed up.
- `BackupInterval` (int, get/set) — This defines how long (in milliseconds) will the graph be automatically saved.
- `BackupLocation` (string, get/set) — Backup files path
- `CollapseToMinSize` (bool, get/set) — Indicates if the groups should be collapsed to minimal size by default.
- `ConnectorType` (Dynamo.Graph.Connectors.ConnectorType, get/set) — The types of connector: Bezier or Polyline.
- `ConsoleHeight` (int, get/set) — The height of the console display.
- `CustomPackageFolders` (System.Collections.Generic.List<string>, get/set) — A list of folders packages, custom nodes or direct paths to .dll and .ds files.
- `DefaultEnableLegacyPolyCurveBehavior` (bool, get/set) — PolyCurve normal and direction behavior has been made predictable in Dynamo 3.0 and has therefore changed. This defines whether legacy (pre-3.0) PolyCurve behavior is selected by default. This flag can be overridden by individual workspaces that have the EnableLegacyPolyCurveBehavior flag defined. Note: For internal use only and will be removed in a future version of Dynamo.
- `DefaultNodeAutocompleteSuggestion` (Dynamo.Models.NodeAutocompleteSuggestion, get/set) — Defines the default method of the Node Autocomplete
- `DefaultPythonEngine` (string, get/set) — Engine used by default for new Python script and string nodes. If not empty, this takes precedence over any system settings.
- `DefaultRunType` (Dynamo.Models.RunType, get/set) — Defines the default run type when opening a workspace
- `DefaultScaleFactor` (double, get/set) — Default geometry scale factor for a new workspace
- `DisableBuiltinPackages` (bool, get/set) — If enabled Dynamo Built-In Packages will not be loaded.
- `DisableCustomPackageLocations` (bool, get/set) — If enabled user's custom package locations will not be loaded.
- `DisableTrustWarnings` (bool, get/set) — If true, trust warnings for opening .dyn files from untrusted locations will not be shown. Do not use this property setter, it does nothing. Exists only to support serialization.
- `DynamoPlayerFolderGroups` (System.Collections.Generic.List<Dynamo.Configuration.DynamoPlayerFolderGroup>, get/set) — Collections of folders used by individual Dynamo Player or Generative Design as entry points.
- `EnableDynamoPlayerRenamedWatchAsOutput` (bool, get/set) — Enable legacy behavior for Dynamo Player to allow renamed Watch nodes to be seen as graph output. This flag is for use in the 2024 product release year and can removed for 2025
- `EnableNewNodeAutoCompleteUI` (bool, get/set) — This allows the user to enable or disable the new node auto complete menu.
- `EnableNodeAutoComplete` (bool, get/set) — This defines if user wants to see the enabled node Auto Complete feature for port interaction.
- `EnableNotificationCenter` (bool, get/set) — This defines if user wants to see the enabled Dynamo Notification Center.
- `EnablePersistExtensions` (bool, get/set) — This defines if user wants the Extensions settings to persist across sessions.
- `EnableStaticSplashScreen` (bool, get/set) — This defines if the user wants to see the static splash screen again
- `GraphChecksumItemsList` (System.Collections.Generic.List<Dynamo.Configuration.GraphChecksumItem>, get/set) — Return a list of GraphChecksumItems
- `GraphicScaleUnit` (string, get/set) — Contains the currently selected unit used for scaling the graphic helpers (grids, axes)
- `GridScaleFactor` (float, get/set) — Sets the background grid element scale
- `GroupStyleItemsList` (System.Collections.Generic.List<Dynamo.Configuration.GroupStyleItem>, get/set) — Stores the group styles added in the preference settings
- `HideAutocompleteMethodOptions` (bool, get/set) — If true, autocomplete method options are hidden from UI
- `HideNodesBelowSpecificConfidenceLevel` (bool, get/set) — This defines if user wants to hide the nodes below a specific confidenc level.
- `HomePageSettings` (System.Collections.Generic.List<string>, get/set) — Persistence for Dynamo HomePage
- `IronPythonResolveTargetVersion` (string, get/set) — The Version of the IronPython package that Dynamo will download when it is found as missing in graphs.
- `IsADPAnalyticsReportingApproved` (bool, get/set) — Indicates whether ADP analytics reporting is approved or not. Note that the getter will communicate to a analytics server which might be slow. This API should only be used in UI scenarios (not in performance sensitive areas)
- `IsAutoSyncDocumentBrowser` (bool, get/set) — This defines if user wants the Document Browser content to be automatically synced to the selected Node. The default value is true.
- `IsBackgroundGridVisible` (bool, get/set) — Should the background grid be shown?
- `IsCreatedFromValidFile` (bool, get) — Indicates when an instance has been created from a preferences file correctly, a support property
- `IsFirstRun` (bool, get/set) — Indicates first run
- `IsIronPythonDialogDisabled` (bool, get/set) — This defines if user wants to see the Iron Python Extension Dialog box on every new session.
- `IsMLAutocompleteTOUApproved` (bool, get/set) — This defines if the user is agree to the ML Automcomplete Terms of Use
- `IsTimeStampIncludedInExportFilePath` (bool, get/set) — This defines if the user export file path would include timestamp
- `LibraryWidth` (int, get/set) — The width of the library pane.
- `LibraryZoomScale` (int, get/set) — Indicates the zoom scale of the library
- `Locale` (string, get/set) — The locale of Dynamo UI, serialize locale instead of language name as ease of conversion back and forth
- `MLRecommendationConfidenceLevel` (int, get/set) — This defines the level of confidence related to the ML recommendation.
- `MLRecommendationNumberOfResults` (int, get/set) — This defines the number of results of the ML recommendation
- `MaxNumRecentFiles` (int, get/set) — The maximum number of recent file paths to be saved.
- `NamespacesToExcludeFromLibrary` (System.Collections.Generic.List<string>, get/set) — Indicates (if any) which namespaces should not be displayed in the Dynamo node library. String format: "[library name]:[fully qualified namespace]"
- `NamespacesToExcludeFromLibrarySpecified` (bool, get/set) — True if the NamespacesToExcludeFromLibrary element is found in DynamoSettings.xml.
- `NodeSearchTagSizeLimit` (int, get/set) — Limits the size of the tags used by the EntryDictionary
- `NumberFormat` (string, get/set) — The decimal precision used to display numbers.
- `OpenFileInManualExecutionMode` (bool, get/set) — Indicates the default state of the "Open in Manual Mode" checkbox in OpenFileDialog
- `OptionalInPortsCollapsed` (bool, get/set) — Indicates if the optional input ports are collapsed by default.
- `PackageDirectoriesToUninstall` (System.Collections.Generic.List<string>, get/set) — A list of packages used by the Package Manager to determine which packages are marked for deletion.
- `PackageDownloadTouAccepted` (bool, get/set) — Indicates if the user has accepted the terms of use for downloading packages from package manager.
- `PredefinedGroupStyleFontSizes` (System.Collections.ObjectModel.ObservableCollection<int>, get) — Return the predefined Font size values
- `PythonScriptZoomScale` (int, get/set) — Indicates the zoom scale of the Python editor
- `PythonTemplateFilePath` (string, get/set) — Path to the Python (.py) file to use as a starting template when creating a new PythonScript Node.
- `ReadNotificationIds` (System.Collections.Generic.List<string>, get/set) — Stores the notification ids that was read by the user
- `RecentFiles` (System.Collections.Generic.List<string>, get/set) — A list of recently opened file paths.
- `RenderPrecision` (int, get/set) — Indicate which render precision will be used
- `SelectedPackagePathForInstall` (string, get/set) — Currently selected package path where all packages downloaded from the Package Manager will be installed. The default package path for install is the user data directory currently used by the Dynamo environment.
- `ShowCodeBlockLineNumber` (bool, get/set) — Indicates if code block node line numbers should be displayed.
- `ShowConnector` (bool, get/set) — Should connectors be visible?
- `ShowConnectorToolTip` (bool, get/set) — Should connector tooltip be visible?
- `ShowDefaultGroupDescription` (bool, get/set) — Indicates if groups should display the default description.
- `ShowDetailedLayout` (bool, get/set) — Indicates whether show detailed or compact layout during search.
- `ShowEdges` (bool, get/set) — Indicates whether surface and solid edges will be rendered.
- `ShowPreviewBubbles` (bool, get/set) — Indicates if preview bubbles should be displayed on nodes.
- `ShowPythonAutoMigrationNotifications` (bool, get/set) — Controls whether Dynamo shows upgrade notifications for legacy CPython nodes when opening a graph. These notices appear when a graph contains CPython-engine Python nodes that are automatically upgraded to PythonNet3: • save/close confirmation dialog • banner inside the Python Script Editor NOTE: This setting is not related to the historical IronPython2 → CPython3 migration.
- `ShowRunPreview` (bool, get/set) — Show Run Preview flag.
- `ShowTabsAndSpacesInScriptEditor` (bool, get/set) — This defines if user wants to see the whitespaces and tabs in python script editor.
- `TemplateFilePath` (string, get/set) — Template path
- `TrustedLocations` (System.Collections.Generic.List<string>, get) — Represents a copy of the list of trusted locations that the user added. Do not use this list to check if a new path is trusted or not. To check if a new path is trusted or not please use the IsTrustedLocation API (IsTrustedLocation supports locations)
- `UnconnectedOutPortsCollapsed` (bool, get/set) — Indicates if the unconnected output ports are hidden by default.
- `UseHardwareAcceleration` (bool, get/set) — Should Dynamo use hardware acceleration if it is supported?
- `UseHostScaleUnits` (bool, get/set) — Indicates if Host units should be used for graphic helpers for Dynamo Revit
- `UseRenderInstancing` (bool, get/set) — Indicates whether background preview use instancing when rendering geometry. be rendered.
- `ViewExtensionSettings` (System.Collections.Generic.List<Dynamo.Configuration.ViewExtensionSettings>, get/set) — Settings that apply to view extensions.
- `WindowH` (double, get/set) — The last height of the Dynamo window.
- `WindowW` (double, get/set) — The last width of the Dynamo window.
- `WindowX` (double, get/set) — The last X coordinate of the Dynamo window.
- `WindowY` (double, get/set) — The last Y coordinate of the Dynamo window.

### Events
#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

Log message event

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/PreferenceSettings.cs)

## StyleItem (class)

This class stores the group styles added by the user

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/StyleItem.cs)

### Constructors
- `void StyleItem()` — StyleItem.StyleItem

### Properties
- `FontSize` (int, get/set) — This property will support the font size of the GroupStyle
- `GroupStyleId` (System.Guid, get/set) — This property will support the id of the Groupstyle
- `HexColorString` (string, get/set) — StyleItem.HexColorString
- `IsDefault` (bool, get/set) — This property will describe if the current GroupStyle added is a default one or a custom one.
- `Name` (string, get/set) — StyleItem.Name

## ViewExtensionDisplayMode (enum)

Possible display modes for an extension UI control.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/ViewExtensionSettings.cs)

### Values
- `DockRight` = `1` — Extension control should be displayed docked to the right side.
- `FloatingWindow` = `2` — Extension control should be displayed in a floating window.
- `Unspecified` = `0` — Not really a display mode but rather the absence of one.

## ViewExtensionSettings (class)

Settings that apply to a view extension specifically.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/ViewExtensionSettings.cs)

### Constructors
- `void ViewExtensionSettings()` — ViewExtensionSettings.ViewExtensionSettings

### Properties
- `DisplayMode` (Dynamo.Configuration.ViewExtensionDisplayMode, get/set) — Specifies how an extension UI control should be displayed.
- `IsOpen` (bool, get/set) — Specifies if the extension was Open in the last session before closing Dynamo, if the property to remember view extension status was enabled.Default: False
- `Name` (string, get/set) — Name of the view extension.
- `UniqueId` (string, get/set) — UniqueId of the view extension.
- `WindowSettings` (Dynamo.Configuration.WindowSettings, get/set) — Window settings for the extension control when displayed in FloatingWindow mode.

## WindowSettings (class)

Settings that define how to display an extension control in floating window mode.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/ViewExtensionSettings.cs)

### Constructors
- `void WindowSettings()` — WindowSettings.WindowSettings

### Properties
- `Height` (int, get/set) — Height of the window.
- `Left` (int, get/set) — Coordinates of the leftmost side of the window.
- `Status` (Dynamo.Configuration.WindowStatus, get/set) — Status of the window, i.e. whether it is maximized.
- `Top` (int, get/set) — Coordinates of the topmost side of the window.
- `Width` (int, get/set) — Width of the window.

## WindowStatus (enum)

Possible status of a floating window.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Configuration/ViewExtensionSettings.cs)

### Values
- `Maximized` = `1` — The window is maximized.
- `Normal` = `0` — The window can be moved and resized.

