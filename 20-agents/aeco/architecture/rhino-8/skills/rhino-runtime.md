---
name: rhino-rhino-runtime
description: This skill encodes the rhino 8.0 surface of the Rhino.Runtime namespace — 43 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AssemblyResolver, CommonObject, CorruptGeometryException, DocumentCollectedException, HostUtils, Interop, LicenseStateChangedEventArgs, NamedParametersEventArgs, and 35 more types.
---

# Rhino.Runtime

Auto-generated from vendor docs for rhino 8.0. 43 types in this namespace.

## AdvancedSetting (enum)

Advanced setting Id

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_AdvancedSetting.htm)

### Values
- `UseNewDraftAngleAnalysisUi` = `0` — UseNewDraftAngleAnalysisUi bool Value, defaults to true
- `UseCompressionWhenSaving` = `1` — Use compressed buffers when saving 3dm files (default is true).
- `TestAdvancedString` = `2` — String value, defaults to "test string"
- `TestAdvancedInt` = `3` — Integer value, defaults 123
- `PdfOptionalContentGroups` = `4` — How optional content should be included when exporting PDF files
- `LeaveFloatingPanelsHiddenOnMac` = `5` — Bool value used on Mac to satisfy a user request to hide floating stuff when a command starts and leave it hidden when the command ends. This is false by default. https://mcneel.myjetbrains.com/youtrack/issue/RH-57945
- `DisableFileWatchers` = `6` — Disable file watchers, using to diagnose Mac Rhino mystery crashes https://mcneel.myjetbrains.com/youtrack/issue/RH-52805
- `EnableCheckForUpdates` = `7` — Auto check for update, enabled by default on Mac, does not show up in Windows https://mcneel.myjetbrains.com/youtrack/issue/RH-60427
- `LeftJustifyNumericText` = `8` — Mainly an issue on Windows; there are different numeric controls scattered across the product and no consistent text alignment. This is an attempt to provide a way to control the preferred alignment style.
- `EnableWindowsGestures` = `9` — https://mcneel.myjetbrains.com/youtrack/issue/RH-62173 Disable gestures for Windows views for a customer who was complaining when upgrading.
- `MakeInteriorNgonVertexesSelectable` = `10` — Set this to false if you only want boundary vertexes selectable when you run PointsOn. https://mcneel.myjetbrains.com/youtrack/issue/RH-62140
- `MacDisplayOldVersionAutosaveWarning` = `11` — Set this to true if you want to be warned about autosave for old file versions on Mac. https://mcneel.myjetbrains.com/youtrack/issue/RH-62359
- `UseNewMissingFontUI` = `12` — Set this to true if you want to be use the new missing font mapping UI. https://mcneel.myjetbrains.com/youtrack/issue/RH-62642
- `ExportDocumentUserText` = `13` — Set to true to write document user text on Export and Copy/Paste. Set to false to not write document user text (default). https://mcneel.myjetbrains.com/youtrack/issue/RH-63589
- `CommandsToIgnoreWhenAutoHidingToolPalettes` = `14` — String list of command names to ignore when closing tool-palette on begin command. https://mcneel.myjetbrains.com/youtrack/issue/RH-65131
- `DisplayNonOriginModelBasepointWarning` = `15` — Set this to true if you want to be warned about ModelBasepoints that are not set to the origin on open/import. https://mcneel.myjetbrains.com/youtrack/issue/RH-67520
- `UseEntireWidthForTopBand` = `16` — Use the entire client width for top and bottom dock sites https://mcneel.myjetbrains.com/youtrack/issue/RH-67783
- `UseMFCMenuBar` = `17` — Experimenting with using a CMFCMenuBar to host main frame menu with a CMFCVisualManager derived class for styling the menu. https://mcneel.myjetbrains.com/youtrack/issue/RH-70314
- `IgnoreRUIPlatform` = `18` — Will ignore the platforms attribute in RUI files when set. Requires restarting Rhino to take affect. https://mcneel.myjetbrains.com/youtrack/issue/RH-70777
- `EnableMcNeelOnlyFeatures` = `19` — Enable McNeel only features for a particular user
- `UseRhinoColorsForModalDialogs` = `20` — Enable to use Eto Appearance page only
- `AcadExportSelectUnexportableObjects` = `21` — Set to true to select objects that could not be exported to DWG, either becuase the object was invalid or the exporter failed for whatever reason. https://mcneel.myjetbrains.com/youtrack/issue/RH-77693
- `AcadExportSortLayers` = `22` — Temporary advanced settings for 8.x so no UI is required. Will be removed in 9.x when proper UI us added. https://mcneel.myjetbrains.com/youtrack/issue/RH-78977 https://mcneel.myjetbrains.com/youtrack/issue/RH-28804 https://mcneel.myjetbrains.com/youtrack/issue/RH-48103
- `AcadExportWriteEmptyLayers` = `23` — ditto
- `DisableModelAndPageUnitsDifferDialog` = `24` — Advanced settings to re-enable the units helper dialogs. see https://mcneel.myjetbrains.com/youtrack/issue/RH-81439
- `DisablePageUnitsNotInchesOrMMDialog` = `25` — ditto
- `UseLegacyCurveJoiner` = `26` — 6-May-2024 Dale Fugier, https://mcneel.myjetbrains.com/youtrack/issue/RH-81815
- `UseLegacyBrepCapper` = `27` — 13-Jun-2024 Dale Fugier, https://mcneel.myjetbrains.com/youtrack/issue/RH-82566
- `HideFloatingWindowsOnDeactivate` = `28` — 19-Sep-2024 Dale Fugier, https://mcneel.myjetbrains.com/youtrack/issue/RH-83845
- `RecordAnimationTargetFolder` = `29` — 14-Oct-2024 Nathan Letwory, https://mcneel.myjetbrains.com/youtrack/issue/RH-79386

## AssemblyResolver (class)

Assembly Resolver for the Rhino App Domain.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_AssemblyResolver.htm)

### Methods
#### `public static void AddSearchFile(string file)`

Register another file with the Assembly Resolver. File must be a .NET assembly, so it should probably be a dll, rhp or exe.

**Parameters:**
- `file` (System.String) — Path of file to include during Assembly Resolver events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_AssemblyResolver_AddSearchFile.htm)

#### `public static void AddSearchFolder(string folder)`

Register a custom folder with the Assembly Resolver. Folders will be searched recursively, so this could potentially be a very expensive operation. If at all possible, you should consider only registering individual files.

**Parameters:**
- `folder` (System.String) — Path of folder to include during Assembly Resolver events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_AssemblyResolver_AddSearchFolder.htm)

### Properties
- `CurrentDomainAssemblyResolve` (ResolveEventHandler, get) — Standard resolver function used by Rhino in execution context. This is added to the Current AssemblyResolve.
- `CurrentDomainReflectionOnlyAssemblyResolve` (ResolveEventHandler, get) — Standard resolver function used by Rhino in reflection-only context. This is added to the Current ReflectionOnlyAssemblyResolve.

## CommonObject (class)

Base class for .NET classes that wrap C++ unmanaged Rhino classes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_CommonObject.htm)

### Constructors
- `protected CommonObject()` — Allows construction from inheriting classes.
- `protected CommonObject(SerializationInfo info, StreamingContext context)` — Protected constructor for internal use.

### Methods
#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_Dispose_1.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_Finalize.htm)

#### `public static CommonObject FromBase64String(int archive3dm, int opennurbs, string base64Data)`

Create a CommonObject instance from a Base64 encoded string. This is typically the values used when passing common objects around as JSON data

**Parameters:**
- `archive3dm` (System.Int32) — [Missing <param name="archive3dm"/> documentation for "M:Rhino.Runtime.CommonObject.FromBase64String(System.Int32,System.Int32,System.String)"]
- `opennurbs` (System.Int32) — [Missing <param name="opennurbs"/> documentation for "M:Rhino.Runtime.CommonObject.FromBase64String(System.Int32,System.Int32,System.String)"]
- `base64Data` (System.String) — [Missing <param name="base64Data"/> documentation for "M:Rhino.Runtime.CommonObject.FromBase64String(System.Int32,System.Int32,System.String)"]

**Returns:** `CommonObject` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.FromBase64String(System.Int32,System.Int32,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_FromBase64String.htm)

#### `public static CommonObject FromJSON(Dictionary<string, string> jsonDictionary)`

Create a CommonObject instance from a JSON dictionary

**Parameters:**
- `jsonDictionary` (System.Collections.Generic.Dictionary<String,String>) — [Missing <param name="jsonDictionary"/> documentation for "M:Rhino.Runtime.CommonObject.FromJSON(System.Collections.Generic.Dictionary{System.String,System.String})"]

**Returns:** `CommonObject` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.FromJSON(System.Collections.Generic.Dictionary{System.String,System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_FromJSON.htm)

#### `public static CommonObject FromJSON(string json)`

Create a CommonObject instance from a JSON string

**Parameters:**
- `json` (System.String) — [Missing <param name="json"/> documentation for "M:Rhino.Runtime.CommonObject.FromJSON(System.String)"]

**Returns:** `CommonObject` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.FromJSON(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_FromJSON_1.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

### Properties
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `PerformCorruptionTesting` (Boolean, get/set) — Used to test ON_Object* pointers to see if they are a brep or mesh that is corrupt enough to crash Rhino.
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use shareable set of information.

## CorruptGeometryException (class)

Thrown when Rhino finds a brep or mesh that will cause a crash if used for calculations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_CorruptGeometryException.htm)

### Properties
- `CommonObject` (CommonObject, get) — Corrupt geometry .NET class
- `Pointer` (IntPtr, get) — pointer to base geometry (ON_Object*)

## DocumentCollectedException (class)

Represents the error that happen when a class user attempts to execute a modifying operation on an object that has been added to a document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_DocumentCollectedException.htm)

### Constructors
- `public DocumentCollectedException()` — Initializes a new instance of the document controlled exception class.
- `public DocumentCollectedException(string message)` — Initializes a new instance of the document collected exception class.

## HostUtils (class)

Contains static methods to deal with the runtime environment.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_HostUtils.htm)

### Methods
#### `public static string AutoInstallPlugInFolder(bool currentUser)`

Gets the auto install plug-in folder for machine or current user.

**Parameters:**
- `currentUser` (System.Boolean) — true if the query relates to the current user.

**Returns:** `String` — The full path to the revelant auto install plug-in directory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_AutoInstallPlugInFolder.htm)

#### `public static int CallFromCoreRhino(string task)`

Don't change this function in ANY way unless you chat with Steve first! This function is called by Rhino on initial startup and the signature must be exact

**Parameters:**
- `task` (System.String) — [Missing <param name="task"/> documentation for "M:Rhino.Runtime.HostUtils.CallFromCoreRhino(System.String)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.CallFromCoreRhino(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_CallFromCoreRhino.htm)

#### `public static bool CheckForRdk(bool throwOnFalse, bool usePreviousResult)`

Determines if the RDK is loaded.

**Parameters:**
- `throwOnFalse` (System.Boolean) — if the RDK is not loaded, then throws a RdkNotLoadedException.
- `usePreviousResult` (System.Boolean) — if true, then the last result can be used instaed of performing a full check.

**Returns:** `Boolean` — true if the RDK is loaded; false if the RDK is not loaded. Note that the RdkNotLoadedException will hinder the retrieval of any return value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_CheckForRdk.htm)

#### `public static void ClearFpuExceptionStatus()`

Clear FPU exception and busy flags (Intel assembly fnclex)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_ClearFpuExceptionStatus.htm)

#### `public static int CreateCommands(IntPtr pPlugIn, Assembly pluginAssembly)`

Parses a plugin and create all the commands defined therein.

**Parameters:**
- `pPlugIn` (System.IntPtr) — Plugin to harvest for commands.
- `pluginAssembly` (System.Reflection.Assembly) — Assembly associated with the plugin.

**Returns:** `Int32` — The number of newly created commands.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_CreateCommands_1.htm)

#### `public static void CreateCommands(PlugIn plugin)`

Parses a plugin and create all the commands defined therein.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — Plugin to harvest for commands.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_CreateCommands.htm)

#### `public static PlugIn CreatePlugIn(Type pluginType, bool printDebugMessages)`

Instantiates a plug-in type and registers the associated commands and classes.

**Parameters:**
- `pluginType` (System.Type) — A plug-in type. This type must derive from PlugIn.
- `printDebugMessages` (System.Boolean) — true if debug messages should be printed.

**Returns:** `PlugIn` — A new plug-in instance.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_CreatePlugIn.htm)

#### `public static string DebugDumpToString(BezierCurve bezierCurve)`

Gets the debug dumps. This is a text description of the geometric contents. DebugDump() is intended for debugging and is not suitable for creating high quality text descriptions of an object.

**Parameters:**
- `bezierCurve` (Rhino.Geometry.BezierCurve) — curve to evaluate

**Returns:** `String` — A debug dump text.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_DebugDumpToString.htm)

#### `public static string DebugDumpToString(GeometryBase geometry)`

Gets the debug dumps. This is a text description of the geometric contents. DebugDump() is intended for debugging and is not suitable for creating high quality text descriptions of an object.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Some geometry.

**Returns:** `String` — A debug dump text.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_DebugDumpToString_1.htm)

#### `public static void DebugString(string msg)`

Prints a debug message to the Rhino Command Line. The message will only appear if the SendDebugToCommandLine property is set to true.

**Parameters:**
- `msg` (System.String) — Message to print.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_DebugString.htm)

#### `public static void DebugString(string format, params Object[] args)`

Prints a debug message to the Rhino Command Line. The message will only appear if the SendDebugToCommandLine property is set to true.

**Parameters:**
- `format` (System.String) — Message to format and print.
- `args` (System.Object[]) — An Object array containing zero or more objects to format.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_DebugString_1.htm)

#### `public static string DescribeGeometry(GeometryBase geometry)`

Returns a description that is similar to the one in the _What command, except for not mentioning units and other attribute data. This description is translated in the current Rhino version.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — [Missing <param name="geometry"/> documentation for "M:Rhino.Runtime.HostUtils.DescribeGeometry(Rhino.Geometry.GeometryBase)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.DescribeGeometry(Rhino.Geometry.GeometryBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_DescribeGeometry.htm)

#### `public static void DisplayOleAlerts(bool display)`

Defines if Ole alerts ("Server busy") alerts should be visualized. This function makes no sense on Mono.

**Parameters:**
- `display` (System.Boolean) — Whether alerts should be visible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_DisplayOleAlerts.htm)

#### `public static void ExceptionReport(Exception ex)`

Informs RhinoCommon of an exception that has been handled but that the developer wants to screen.

**Parameters:**
- `ex` (System.Exception) — An exception.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_ExceptionReport.htm)

#### `public static void ExceptionReport(string source, Exception ex)`

Informs RhinoCommon of an exception that has been handled but that the developer wants to screen.

**Parameters:**
- `source` (System.String) — An exception source text.
- `ex` (System.Exception) — An exception.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_ExceptionReport_1.htm)

#### `public static bool ExecuteNamedCallback(string name, NamedParametersEventArgs args)`

Execute a named callback

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.HostUtils.ExecuteNamedCallback(System.String,Rhino.Runtime.NamedParametersEventArgs)"]
- `args` (Rhino.Runtime.NamedParametersEventArgs) — [Missing <param name="args"/> documentation for "M:Rhino.Runtime.HostUtils.ExecuteNamedCallback(System.String,Rhino.Runtime.NamedParametersEventArgs)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.ExecuteNamedCallback(System.String,Rhino.Runtime.NamedParametersEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_ExecuteNamedCallback.htm)

#### `public static bool FileNameEndsWithRhinoBackupExtension(string fileName)`

Strip file extension from file name and check to see if it is a valid Rhino backup file extension.

**Parameters:**
- `fileName` (System.String) — File name to check.

**Returns:** `Boolean` — Returns true if the file name has an extension like 3dmbak.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_FileNameEndsWithRhinoBackupExtension.htm)

#### `public static bool FileNameEndsWithRhinoExtension(string fileName)`

Strip file extension from file name and check to see if it is a valid Rhino file extension.

**Parameters:**
- `fileName` (System.String) — File name to check.

**Returns:** `Boolean` — Returns true if the file name has an extension like 3dm.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_FileNameEndsWithRhinoExtension.htm)

#### `public static bool GetAbsolutePath(string relativePath, bool bRelativePathisFileName, string relativeTo, bool bRelativeToIsFileName, out string pathOut)`

Call this method to convert a relative path to an absolute path relative to the specified path.

**Parameters:**
- `relativePath` (System.String) — Relative path to convert to an absolute path
- `bRelativePathisFileName` (System.Boolean) — If true then lpsFrom is treated as a file name otherwise it is treated as a directory name
- `relativeTo` (System.String) — File or folder the path is relative to
- `bRelativeToIsFileName` (System.Boolean) — If true then lpsFrom is treated as a file name otherwise it is treated as a directory name
- `pathOut` (System.String) — Reference to string which will receive the computed absolute path

**Returns:** `Boolean` — Returns true if parameters are valid and lpsRelativePath is indeed relative to lpsRelativeTo otherwise returns false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetAbsolutePath.htm)

#### `public static IEnumerable<DirectoryInfo> GetActivePlugInVersionFolders()`

Recurses through the auto install plug-in folders and returns the directories containing "active" versions of plug-ins.

**Remarks:** If the same package is installed in both the user and machine locations, the newest directory wins.

**Returns:** `IEnumerable<DirectoryInfo>` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.GetActivePlugInVersionFolders"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetActivePlugInVersionFolders.htm)

#### `public static IEnumerable<DirectoryInfo> GetActivePlugInVersionFolders(bool currentUser)`

Recurses through the auto install plug-in folders and returns the directories containing "active" versions of plug-ins.

**Parameters:**
- `currentUser` (System.Boolean) — Current user (true) or machine (false).

**Returns:** `IEnumerable<DirectoryInfo>` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.GetActivePlugInVersionFolders(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetActivePlugInVersionFolders_1.htm)

#### `public static string[] GetAssemblySearchPaths()`

Returns list of directory names where additional assemblies (plug-ins, DLLs, Grasshopper components) may be located

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.GetAssemblySearchPaths"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetAssemblySearchPaths.htm)

#### `public static void GetCurrentProcessInfo(out string processName, out Version processVersion)`

Returns information about the current process. If Rhino is the top level process, processName is "Rhino". Otherwise, processName is the name, without extension, of the main module that is executing. For example, "compute.backend" or "Revit". processVersion is the System.Version of the running process. It is the FileVersion of the executable.

**Parameters:**
- `processName` (System.String) — [Missing <param name="processName"/> documentation for "M:Rhino.Runtime.HostUtils.GetCurrentProcessInfo(System.String@,System.Version@)"]
- `processVersion` (System.Version) — [Missing <param name="processVersion"/> documentation for "M:Rhino.Runtime.HostUtils.GetCurrentProcessInfo(System.String@,System.Version@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetCurrentProcessInfo.htm)

#### `public static Tuple<string, Type>[] GetCustomComputeEndpoints()`

Used by compute to define custom endpoints

**Returns:** `Tuple<String,Type>[]` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.GetCustomComputeEndpoints"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetCustomComputeEndpoints.htm)

#### `public static T GetPlatformService<T>(string assemblyPath = null, string typeFullName = null) where T : class`

For internal use only. Loads an assembly for dependency injection via IPlatformServiceLocator.

**Parameters:**
- `assemblyPath` (System.String) — The relative path of the assembly, relative to the position of RhinoCommon.dll
- `typeFullName` (System.String) — The full name of the type that is IPlatformServiceLocator. This is optional.

**Returns:** `T` — An instance, or null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetPlatformService__1.htm)

#### `public static double GetPrinterDPI(string printerName, bool horizontal)`

Get the output resolution for a given printer.

**Parameters:**
- `printerName` (System.String) — [Missing <param name="printerName"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterDPI(System.String,System.Boolean)"]
- `horizontal` (System.Boolean) — get the horizontal or vertical resolution

**Returns:** `Double` — Dot per inch resolution for a given printer on success. 0 if an error occurred

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetPrinterDPI.htm)

#### `public static bool GetPrinterFormMargins(string printerName, string formName, bool portrait, out double leftMillimeters, out double topMillimeters, out double rightMillimeters, out double bottomMillimeters)`

Get limit margins for a given form (page size) and a given printer. This is the physical limit area that a printer can print on a given page

**Parameters:**
- `printerName` (System.String) — [Missing <param name="printerName"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormMargins(System.String,System.String,System.Boolean,System.Double@,System.Double@,System.Double@,System.Double@)"]
- `formName` (System.String) — [Missing <param name="formName"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormMargins(System.String,System.String,System.Boolean,System.Double@,System.Double@,System.Double@,System.Double@)"]
- `portrait` (System.Boolean) — [Missing <param name="portrait"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormMargins(System.String,System.String,System.Boolean,System.Double@,System.Double@,System.Double@,System.Double@)"]
- `leftMillimeters` (System.Double) — [Missing <param name="leftMillimeters"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormMargins(System.String,System.String,System.Boolean,System.Double@,System.Double@,System.Double@,System.Double@)"]
- `topMillimeters` (System.Double) — [Missing <param name="topMillimeters"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormMargins(System.String,System.String,System.Boolean,System.Double@,System.Double@,System.Double@,System.Double@)"]
- `rightMillimeters` (System.Double) — [Missing <param name="rightMillimeters"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormMargins(System.String,System.String,System.Boolean,System.Double@,System.Double@,System.Double@,System.Double@)"]
- `bottomMillimeters` (System.Double) — [Missing <param name="bottomMillimeters"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormMargins(System.String,System.String,System.Boolean,System.Double@,System.Double@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetPrinterFormMargins.htm)

#### `public static string[] GetPrinterFormNames(string printerName)`

Get list of form names available for a given printer

**Parameters:**
- `printerName` (System.String) — name or printer to query

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormNames(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetPrinterFormNames.htm)

#### `public static bool GetPrinterFormSize(string printerName, string formName, out double widthMillimeters, out double heightMillimeters)`

Get the size of a sheet for a given form name / printer combination

**Parameters:**
- `printerName` (System.String) — [Missing <param name="printerName"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormSize(System.String,System.String,System.Double@,System.Double@)"]
- `formName` (System.String) — [Missing <param name="formName"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormSize(System.String,System.String,System.Double@,System.Double@)"]
- `widthMillimeters` (System.Double) — [Missing <param name="widthMillimeters"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormSize(System.String,System.String,System.Double@,System.Double@)"]
- `heightMillimeters` (System.Double) — [Missing <param name="heightMillimeters"/> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterFormSize(System.String,System.String,System.Double@,System.Double@)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetPrinterFormSize.htm)

#### `public static string[] GetPrinterNames()`

Get list of printers available on this system

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.GetPrinterNames"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetPrinterNames.htm)

#### `public static Assembly GetRhinoDotNetAssembly()`

Only works on Windows. Returns null on Mac.

**Returns:** `Assembly` — An assembly.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetRhinoDotNetAssembly.htm)

#### `public static int GetSystemProcessorCount()`

Get the processor count on this hardware. It supports querying on CPUs with more than 64 processors (Windows).

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.GetSystemProcessorCount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetSystemProcessorCount.htm)

#### `public static IEnumerable<string> GetSystemReferenceAssemblies()`

Gets the system reference assemblies to use when compiling code dynamically with Roslyn. Includes RhinoCommon, Rhino.UI, and Eto.

**Remarks:** Note that this list of assemblies is not guaranteed to be loadable and should only be used when compiling code dynamically.

**Returns:** `IEnumerable<String>` — An enumeration of paths to each of the rhino system assemblies to be used for compilation

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_GetSystemReferenceAssemblies.htm)

#### `public static void InPlaceConstCast(GeometryBase geometry, bool makeNonConst)`

DO NOT USE UNLESS YOU ARE CERTAIN ABOUT THE IMPLICATIONS. This is an expert user function which should not be needed in most cases. This function is similar to a const_cast in C++ to allow an object to be made temporarily modifiable without causing RhinoCommon to convert the class from const to non-const by creating a duplicate.You must call this function with a true parameter, make your modifications, and then restore the const flag by calling this function again with a false parameter. If you have any questions, please contact McNeel developer support before using!

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Some geometry.
- `makeNonConst` (System.Boolean) — A boolean value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_InPlaceConstCast.htm)

#### `public static void InitializeRhinoCommon()`

Makes sure all static RhinoCommon components is set up correctly. This happens automatically when a plug-in is loaded, so you probably won't have to call this method.

**Remarks:** Subsequent calls to this method will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_InitializeRhinoCommon.htm)

#### `public static void InitializeRhinoCommon_RDK()`

Makes sure all static RhinoCommon RDK components are set up correctly. This happens automatically when the RDK is loaded, so you probably won't have to call this method.

**Remarks:** Subsequent calls to this method will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_InitializeRhinoCommon_RDK.htm)

#### `public static void InitializeZooClient()`

Initializes the ZooClient and Rhino license manager, this should get called automatically when RhinoCommon is loaded so you probably won't have to call this method.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_InitializeZooClient.htm)

#### `public static bool IsManagedDll(string path)`

Inspects a dll to see if it is compiled as native code or as a .NET assembly

**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.Runtime.HostUtils.IsManagedDll(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.IsManagedDll(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_IsManagedDll.htm)

#### `public static bool IsRhinoBackupFileExtension(string fileExtension)`

Check to see if the file extension is a valid Rhino file extension.

**Parameters:**
- `fileExtension` (System.String) — [Missing <param name="fileExtension"/> documentation for "M:Rhino.Runtime.HostUtils.IsRhinoBackupFileExtension(System.String)"]

**Returns:** `Boolean` — Return true if fileExtension is ".3dmbak", "3dmbak", ".3dm.bak", "3dm.bak", ".3dx.bak" or "3dx.bak", ignoring case.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_IsRhinoBackupFileExtension.htm)

#### `public static bool IsRhinoFileExtension(string fileExtension)`

Check to see if the file extension is a valid Rhino file extension.

**Parameters:**
- `fileExtension` (System.String) — [Missing <param name="fileExtension"/> documentation for "M:Rhino.Runtime.HostUtils.IsRhinoFileExtension(System.String)"]

**Returns:** `Boolean` — Returns true if fileExtension is ".3dm", "3dm", ".3dx" or "3dx", ignoring case.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_IsRhinoFileExtension.htm)

#### `public static Assembly LoadAssemblyFrom(string path)`

Calls Assembly.LoadFrom in .NET 4.8. May call a different routine under .NET Core

**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.Runtime.HostUtils.LoadAssemblyFrom(System.String)"]

**Returns:** `Assembly` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.LoadAssemblyFrom(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_LoadAssemblyFrom.htm)

#### `public static Assembly LoadAssemblyFromStream(Stream stream)`

Calls Assembly.Load(byte[]) in .NET 4.8. May call a different routine under .NET Core

**Parameters:**
- `stream` (System.IO.Stream) — [Missing <param name="stream"/> documentation for "M:Rhino.Runtime.HostUtils.LoadAssemblyFromStream(System.IO.Stream)"]

**Returns:** `Assembly` — [Missing <returns> documentation for "M:Rhino.Runtime.HostUtils.LoadAssemblyFromStream(System.IO.Stream)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_LoadAssemblyFromStream.htm)

#### `public static void LogDebugEvent(string message)`

Logs a debug event. The function will log the filename and line number from where this function is called, in addition to the input message.

**Parameters:**
- `message` (System.String) — The event message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_LogDebugEvent.htm)

#### `public static void RecordInitInstanceTime(string description)`

Used to help record times at startup with the -stopwatch flag to help determine bottlenecks in start up speed

**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.Runtime.HostUtils.RecordInitInstanceTime(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_RecordInitInstanceTime.htm)

#### `public static void RegisterComputeEndpoint(string endpointPath, Type t)`

Register a class that can participate as a compute endpoint

**Parameters:**
- `endpointPath` (System.String) — [Missing <param name="endpointPath"/> documentation for "M:Rhino.Runtime.HostUtils.RegisterComputeEndpoint(System.String,System.Type)"]
- `t` (System.Type) — [Missing <param name="t"/> documentation for "M:Rhino.Runtime.HostUtils.RegisterComputeEndpoint(System.String,System.Type)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_RegisterComputeEndpoint.htm)

#### `public static bool RegisterDynamicCommand(PlugIn plugin, Command cmd)`

Adds a new dynamic command to Rhino.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — Plugin that owns the command.
- `cmd` (Rhino.Commands.Command) — Command to add.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_RegisterDynamicCommand.htm)

#### `public static void RegisterNamedCallback(string name, EventHandler<NamedParametersEventArgs> callback)`

Register a named callback

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.HostUtils.RegisterNamedCallback(System.String,System.EventHandler{Rhino.Runtime.NamedParametersEventArgs})"]
- `callback` (System.EventHandler<NamedParametersEventArgs>) — [Missing <param name="callback"/> documentation for "M:Rhino.Runtime.HostUtils.RegisterNamedCallback(System.String,System.EventHandler{Rhino.Runtime.NamedParametersEventArgs})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_RegisterNamedCallback.htm)

#### `public static void RemoveNamedCallback(string name)`

Remove a named callback from the dictionary of callbacks

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.HostUtils.RemoveNamedCallback(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_RemoveNamedCallback.htm)

#### `public static void RhinoCommonExceptionHandler(string title, Object sender, Exception ex)`

For internal use only!!! Unhanded exception handler, writes stack trace to RhinoDotNet.txt file

**Parameters:**
- `title` (System.String) — Exception title to write to text file
- `sender` (System.Object) — [Missing <param name="sender"/> documentation for "M:Rhino.Runtime.HostUtils.RhinoCommonExceptionHandler(System.String,System.Object,System.Exception)"]
- `ex` (System.Exception) — [Missing <param name="ex"/> documentation for "M:Rhino.Runtime.HostUtils.RhinoCommonExceptionHandler(System.String,System.Object,System.Exception)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_RhinoCommonExceptionHandler.htm)

#### `public static void SendLogMessageToCloudCallbackProc(HostUtils.LogMessageType msg_type, IntPtr pwStringClass, IntPtr pwStringDesc, IntPtr pwStringMessage)`

Informs RhinoCommon of an message that has been handled but that the developer wants to screen.

**Parameters:**
- `msg_type` (Rhino.Runtime.HostUtils.LogMessageType) — The messag type
- `pwStringClass` (System.IntPtr) — The top level message type.
- `pwStringDesc` (System.IntPtr) — Finer grained description of the message.
- `pwStringMessage` (System.IntPtr) — The message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_SendLogMessageToCloudCallbackProc.htm)

#### `public static void SetInShutDown()`

Informs the runtime that the application is shutting down.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_SetInShutDown.htm)

#### `public static void ShutDownRhinoCommon_RDK()`

Makes sure all static RhinoCommon RDK components are de-initialized so they aren't calling into space when the RDK is unloaded.

**Remarks:** Subsequent calls to this method will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_ShutDownRhinoCommon_RDK.htm)

#### `public static void UnhandledThreadException(Object sender, ThreadExceptionEventArgs e)`

Exception handler for exceptions occurring on the UI thread

**Parameters:**
- `sender` (System.Object) — [Missing <param name="sender"/> documentation for "M:Rhino.Runtime.HostUtils.UnhandledThreadException(System.Object,System.Threading.ThreadExceptionEventArgs)"]
- `e` (System.Threading.ThreadExceptionEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.Runtime.HostUtils.UnhandledThreadException(System.Object,System.Threading.ThreadExceptionEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_HostUtils_UnhandledThreadException.htm)

### Properties
- `ComputerSerialNumber` (String, get) — Gets the serial number of the computer running Rhino.
- `CurrentOSLanguage` (UInt32, get) — Get the current operating system language.
- `DeviceId` (Guid, get/set) — The DeviceId is a unique, stable ID that anonymously identifies the device that Rhino is running on. It is computed based on hardware information that should not change when the OS is upgraded, or if commonly modified hardware are added or removed from the computer. The machine-specific information is hashed using a cryptographic hash to make it anonymous.
- `DeviceName` (String, get) — Name of the computer running Rhino. If the computer is part of a Windows Domain, the computer name has "@[DOMAIN]" appended.
- `IsPreRelease` (Boolean, get) — Returns true when Rhino build is Beta or WIP, false otherwise
- `OperatingSystemBuildNumber` (String, get) — Returns Operating System Build Number "11763" | "7601" | ... | "Unknown"
- `OperatingSystemEdition` (String, get) — Returns Operating System Edition: "Professional" | "ServerDatacenter" | ... | "Unknown"
- `OperatingSystemInstallationType` (String, get) — Returns Operating System Installation Type: "Client" | "Server" | "Unknown"
- `OperatingSystemProductName` (String, get) — Returns Operating System Edition: "Professional" | "ServerDatacenter" | ... | "Unknown"
- `OperatingSystemVersion` (String, get) — Returns Operating System Version "6.1" | "6.3" | ... | "Unknown"
- `RhinoAssemblyDirectory` (String, get) — Gets the Rhino system managed assembly directory.
- `RunningAsRhinoInside` (Boolean, get) — Indicates whether Rhino is running inside another application. returns false if Rhino.exe is the top-level application. returns true if some other application is the top-level application.
- `RunningInDarkMode` (Boolean, get) — Returns true if the host operating system is in dark mode and Rhino supports dark mode.
- `RunningInMono` (Boolean, get) — Tests if this process is currently executing under the Mono runtime.
- `RunningInNetCore` (Boolean, get) — Tests if this process is currently executing under the .NET Core runtime.
- `RunningInNetFramework` (Boolean, get) — Tests if this process is currently executing under the .NET Framework runtime.
- `RunningInRhino` (Boolean, get) — Tests if RhinoCommon is currently executing inside of the Rhino.exe process. There are other cases where RhinoCommon could be running; specifically inside of Visual Studio when something like a windows form is being worked on in the resource editor or running stand-alone when compiled to be used as a version of OpenNURBS.
- `RunningInWindowsContainer` (Boolean, get) — Tests if this process is currently executing inside a Windows Container.
- `RunningOniOS` (Boolean, get) — Tests if this process is currently executing on the iOS platform.
- `RunningOnOSX` (Boolean, get) — Tests if this process is currently executing on the Mac OSX platform.
- `RunningOnServer` (Boolean, get) — Tests if this process is currently executing in a server environment.
- `RunningOnWindows` (Boolean, get) — Tests if this process is currently executing on the Windows platform.
- `SendDebugToCommandLine` (Boolean, get/set) — Gets or sets whether debug messages are printed to the command line.

### Events
#### `OnExceptionReport` (`Rhino.Runtime.HostUtils.ExceptionReportDelegate`)

**Signature:** `public static event HostUtils.ExceptionReportDelegate OnExceptionReport`

Is raised when an exception is reported with one of the ExceptionReport(Exception) method.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_Runtime_HostUtils_OnExceptionReport.htm)

#### `OnSendLogMessageToCloud` (`Rhino.Runtime.HostUtils.SendLogMessageToCloudDelegate`)

**Signature:** `public static event HostUtils.SendLogMessageToCloudDelegate OnSendLogMessageToCloud`

Is raised when an exception is reported with one of the method.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_Runtime_HostUtils_OnSendLogMessageToCloud.htm)

## HostUtils.ExceptionReportDelegate (delegate)

Represents a reference to a method that will be called when an exception occurs.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_HostUtils_ExceptionReportDelegate.htm)

## HostUtils.LogMessageType (enum)

Represents the type of message that is being sent to the OnSendLogMessageToCloud event

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_HostUtils_LogMessageType.htm)

### Values
- `unknown` = `0` — Unknown message type
- `information` = `1` — Message is informational only
- `warning` = `2` — Message is a warning
- `error` = `3` — Message is an error
- `assert` = `4` — Message is a debug ASSERT

## HostUtils.SendLogMessageToCloudDelegate (delegate)

Represents a reference to a method that will be called when an exception occurs.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_HostUtils_SendLogMessageToCloudDelegate.htm)

## IPlatformServiceLocator (interface)

Get platform specific services that are used internally for general cross platform funtions in RhinoCommon. This includes services like localization and GUI components that have concrete implementations in the RhinoWindows or RhinoMac assemblies

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_IPlatformServiceLocator.htm)

### Methods
#### `T GetService<T>() where T : class`

Used to get service of a specific type

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Runtime.IPlatformServiceLocator.GetService``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IPlatformServiceLocator_GetService__1.htm)

## IShrinkWrapService (interface)

Internal interface used by ShrinkWrap functions

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_IShrinkWrapService.htm)

### Methods
#### `Mesh ShrinkWrap(IEnumerable<GeometryBase> geometryBases, ShrinkWrapParameters parameters, MeshingParameters meshingParameters)`

Create a mesh from a collection of geometry base objects Null on error or incompatible settings

**Parameters:**
- `geometryBases` (System.Collections.Generic.IEnumerable<GeometryBase>) — [Missing <param name="geometryBases"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters)"]
- `meshingParameters` (Rhino.Geometry.MeshingParameters) — [Missing <param name="meshingParameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap_4.htm)

#### `Mesh ShrinkWrap(IEnumerable<GeometryBase> geometryBases, ShrinkWrapParameters parameters, MeshingParameters meshingParameters, CancellationToken cancellationToken)`

Create a mesh from a collection of geometry base objects Null on error or incompatible settings

**Parameters:**
- `geometryBases` (System.Collections.Generic.IEnumerable<GeometryBase>) — [Missing <param name="geometryBases"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters,System.Threading.CancellationToken)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters,System.Threading.CancellationToken)"]
- `meshingParameters` (Rhino.Geometry.MeshingParameters) — [Missing <param name="meshingParameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters,System.Threading.CancellationToken)"]
- `cancellationToken` (System.Threading.CancellationToken) — [Missing <param name="cancellationToken"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters,System.Threading.CancellationToken)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase},Rhino.Geometry.ShrinkWrapParameters,Rhino.Geometry.MeshingParameters,System.Threading.CancellationToken)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap_5.htm)

#### `Mesh ShrinkWrap(IEnumerable<Mesh> meshes, ShrinkWrapParameters parameters)`

Create mesh from input meshes. Null on error or incompatible settings

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — [Missing <param name="meshes"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Geometry.ShrinkWrapParameters)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Geometry.ShrinkWrapParameters)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Geometry.ShrinkWrapParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap_6.htm)

#### `Mesh ShrinkWrap(IEnumerable<Mesh> meshes, ShrinkWrapParameters parameters, CancellationToken cancellationToken)`

Create mesh from input meshes. Null on error or incompatible settings

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — [Missing <param name="meshes"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]
- `cancellationToken` (System.Threading.CancellationToken) — [Missing <param name="cancellationToken"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap_7.htm)

#### `Mesh ShrinkWrap(Mesh mesh, ShrinkWrapParameters parameters)`

Create a shrinkwrap from a single mesh Null on error or incompatible settings

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.Mesh,Rhino.Geometry.ShrinkWrapParameters)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.Mesh,Rhino.Geometry.ShrinkWrapParameters)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.Mesh,Rhino.Geometry.ShrinkWrapParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap.htm)

#### `Mesh ShrinkWrap(Mesh mesh, ShrinkWrapParameters parameters, CancellationToken cancellationToken)`

Create a shrinkwrap from a single mesh Null on error or incompatible settings

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.Mesh,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.Mesh,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]
- `cancellationToken` (System.Threading.CancellationToken) — [Missing <param name="cancellationToken"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.Mesh,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.Mesh,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap_1.htm)

#### `Mesh ShrinkWrap(PointCloud pointCloud, ShrinkWrapParameters parameters)`

Create mesh from point cloud input. Null on error or incompatible settings

**Parameters:**
- `pointCloud` (Rhino.Geometry.PointCloud) — [Missing <param name="pointCloud"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.PointCloud,Rhino.Geometry.ShrinkWrapParameters)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.PointCloud,Rhino.Geometry.ShrinkWrapParameters)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.PointCloud,Rhino.Geometry.ShrinkWrapParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap_2.htm)

#### `Mesh ShrinkWrap(PointCloud pointCloud, ShrinkWrapParameters parameters, CancellationToken cancellationToken)`

Create mesh from point cloud input. Null on error or incompatible settings

**Parameters:**
- `pointCloud` (Rhino.Geometry.PointCloud) — [Missing <param name="pointCloud"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.PointCloud,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]
- `parameters` (Rhino.Geometry.ShrinkWrapParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.PointCloud,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]
- `cancellationToken` (System.Threading.CancellationToken) — [Missing <param name="cancellationToken"/> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.PointCloud,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Runtime.IShrinkWrapService.ShrinkWrap(Rhino.Geometry.PointCloud,Rhino.Geometry.ShrinkWrapParameters,System.Threading.CancellationToken)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IShrinkWrapService_ShrinkWrap_3.htm)

## IZooClientUtilities (interface)

Interface implemented in ZooClient and added to Rhino via dependency injection

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_IZooClientUtilities.htm)

### Methods
#### `bool AskUserForLicense(Object verify, ZooClientParameters parameters)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.AskUserForLicense(System.Object,Rhino.Runtime.ZooClientParameters)"]
- `parameters` (Rhino.Runtime.ZooClientParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.AskUserForLicense(System.Object,Rhino.Runtime.ZooClientParameters)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.AskUserForLicense(System.Object,Rhino.Runtime.ZooClientParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_AskUserForLicense.htm)

#### `bool CheckInLicense(Object verify, Guid productId)`

Checks in a checked out license to the owning Zoo.

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.CheckInLicense(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.CheckInLicense(System.Object,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.CheckInLicense(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_CheckInLicense.htm)

#### `bool CheckOutLicense(Object verify, Guid productId)`

Checks out a loaned out license from the owning Zoo.

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.CheckOutLicense(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.CheckOutLicense(System.Object,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.CheckOutLicense(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_CheckOutLicense.htm)

#### `bool ConvertLicense(Object verify, Guid productId)`

Converts a standalone license to a network license.

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ConvertLicense(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ConvertLicense(System.Object,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.ConvertLicense(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_ConvertLicense.htm)

#### `bool DeleteLicense(Object verify, Guid productId)`

31-Mar-2015 Dale Fugier, http://mcneel.myjetbrains.com/youtrack/issue/MR-1725 Deletes a license along with its license file.

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.DeleteLicense(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.DeleteLicense(System.Object,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.DeleteLicense(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_DeleteLicense.htm)

#### `string Echo(Object verify, string message)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.Echo(System.Object,System.String)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.Echo(System.Object,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.Echo(System.Object,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_Echo.htm)

#### `DateTime GetCurrentTime()`

Gets the current time, trying to access an NTP server before using local time.

**Returns:** `DateTime` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetCurrentTime"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_GetCurrentTime.htm)

#### `bool GetLicense(Object verify, ZooClientParameters parameters)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicense(System.Object,Rhino.Runtime.ZooClientParameters)"]
- `parameters` (Rhino.Runtime.ZooClientParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicense(System.Object,Rhino.Runtime.ZooClientParameters)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicense(System.Object,Rhino.Runtime.ZooClientParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_GetLicense.htm)

#### `LicenseStatus[] GetLicenseStatus(Object verify)`

Returns the current status of every license for ui purposes

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicenseStatus(System.Object)"]

**Returns:** `LicenseStatus[]` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicenseStatus(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_GetLicenseStatus.htm)

#### `int GetLicenseType(Object verify, Guid productId)`

Returns the type of a specified product license

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicenseType(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicenseType(System.Object,System.Guid)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetLicenseType(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_GetLicenseType.htm)

#### `LicenseStatus GetOneLicenseStatus(Object verify, Guid productId)`

Returns the current status of a license for ui purposes

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetOneLicenseStatus(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetOneLicenseStatus(System.Object,System.Guid)"]

**Returns:** `LicenseStatus` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetOneLicenseStatus(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_GetOneLicenseStatus.htm)

#### `bool GetRegisteredOwnerInfo(Object verify, Guid productId, ref string registeredOwner, ref string registeredOrganization)`

Returns the registered owner and organization of a license 4-Sept-2014 Dale Fugier, http://mcneel.myjetbrains.com/youtrack/issue/RH-28623

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetRegisteredOwnerInfo(System.Object,System.Guid,System.String@,System.String@)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetRegisteredOwnerInfo(System.Object,System.Guid,System.String@,System.String@)"]
- `registeredOwner` (System.String) — [Missing <param name="registeredOwner"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetRegisteredOwnerInfo(System.Object,System.Guid,System.String@,System.String@)"]
- `registeredOrganization` (System.String) — [Missing <param name="registeredOrganization"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetRegisteredOwnerInfo(System.Object,System.Guid,System.String@,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.GetRegisteredOwnerInfo(System.Object,System.Guid,System.String@,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_GetRegisteredOwnerInfo.htm)

#### `bool Initialize(Object verify)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.Initialize(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.Initialize(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_Initialize.htm)

#### `bool IsCheckOutEnabled(Object verify)`

Returns whether or not license checkout is enabled

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.IsCheckOutEnabled(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.IsCheckOutEnabled(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_IsCheckOutEnabled.htm)

#### `bool LicenseOptionsHandler(Object verify, ZooClientParameters parameters)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.LicenseOptionsHandler(System.Object,Rhino.Runtime.ZooClientParameters)"]
- `parameters` (Rhino.Runtime.ZooClientParameters) — [Missing <param name="parameters"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.LicenseOptionsHandler(System.Object,Rhino.Runtime.ZooClientParameters)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.LicenseOptionsHandler(System.Object,Rhino.Runtime.ZooClientParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_LicenseOptionsHandler.htm)

#### `bool LoginToCloudZoo()`

Logs the user in to the cloud zoo. This logs out the current user and voids any existing leases.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.LoginToCloudZoo"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_LoginToCloudZoo.htm)

#### `bool LogoutOfCloudZoo()`

Logs the user out of the cloud zoo. This logs out the current user and voids any existing leases.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.LogoutOfCloudZoo"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_LogoutOfCloudZoo.htm)

#### `bool ReturnLicense(Object verify, Guid productId)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ReturnLicense(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ReturnLicense(System.Object,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.ReturnLicense(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_ReturnLicense.htm)

#### `bool ReturnLicense(Object verify, string productPath, Guid productId)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ReturnLicense(System.Object,System.String,System.Guid)"]
- `productPath` (System.String) — [Missing <param name="productPath"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ReturnLicense(System.Object,System.String,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ReturnLicense(System.Object,System.String,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.ReturnLicense(System.Object,System.String,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_ReturnLicense_1.htm)

#### `void ShowBuyLicenseUi(Object verify, Guid productId)`



**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowBuyLicenseUi(System.Object,System.Guid)"]
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowBuyLicenseUi(System.Object,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_ShowBuyLicenseUi.htm)

#### `bool ShowLicenseValidationUi(Object verify, string cdkey)`

Shows user interface to validate and register a license. Returns true if the license is successfully validated; false otherwise

**Parameters:**
- `verify` (System.Object) — [Missing <param name="verify"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowLicenseValidationUi(System.Object,System.String)"]
- `cdkey` (System.String) — [Missing <param name="cdkey"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowLicenseValidationUi(System.Object,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowLicenseValidationUi(System.Object,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_ShowLicenseValidationUi.htm)

#### `bool ShowRhinoExpiredMessage(Mode mode, ref int result)`



**Parameters:**
- `mode` (Rhino.Runtime.Mode) — [Missing <param name="mode"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowRhinoExpiredMessage(Rhino.Runtime.Mode,System.Int32@)"]
- `result` (System.Int32) — [Missing <param name="result"/> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowRhinoExpiredMessage(Rhino.Runtime.Mode,System.Int32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.IZooClientUtilities.ShowRhinoExpiredMessage(Rhino.Runtime.Mode,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_IZooClientUtilities_ShowRhinoExpiredMessage.htm)

### Properties
- `LoggedInUserAvatar` (Image, get) — Returns the logged in user's avatar picture. Returns a default avatar if the user does not have an avatar or if the avatar could not be fetched.
- `LoggedInUserName` (String, get) — Returns the name of the logged in user, or null if the user is not logged in.
- `UserIsLoggedIn` (Boolean, get) — Returns true if the user is logged in; else returns false. A logged in user does not guarantee that the auth tokens managed by the CloudZooManager instance are valid.

## ImportOptionsSections (enum)

Standard CRhImportOptions sections to import

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ImportOptionsSections.htm)

### Values
- `AdvancedDisplay` = `0`
- `Alias` = `1`
- `Appearance` = `2`
- `ChooseOneObject` = `3`
- `ControlPointContextMenu` = `4`
- `CursorToolTip` = `5`
- `Display` = `6`
- `File` = `7`
- `General` = `8`
- `Grid` = `9`
- `ModelAid` = `10`
- `Mouse` = `11`
- `NeverRepeatCommands` = `12`
- `ObjectContextMenu` = `13`
- `SearchPath` = `14`
- `ShortcutKey` = `15`
- `Smarttrack` = `16`
- `View` = `17`
- `ViewportContextMenu` = `18`
- `ToolPaletteSettings` = `19` — Tool palette settings for Mac, ignored for Windows
- `Count` = `20` — Must always be the last item

## Interop (class)

Contains static methods to marshal objects between RhinoCommon and legacy Rhino_DotNet or C++.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Interop.htm)

### Methods
#### `public static CommandLineOption CommandLineOptionFromNativePointer(IntPtr ptrCommandLineOption)`

Create a command line option for a native pointer. Do not hold onto this class as it does not control the lifetime of the underlying pointer

**Parameters:**
- `ptrCommandLineOption` (System.IntPtr) — [Missing <param name="ptrCommandLineOption"/> documentation for "M:Rhino.Runtime.Interop.CommandLineOptionFromNativePointer(System.IntPtr)"]

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.CommandLineOptionFromNativePointer(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_CommandLineOptionFromNativePointer.htm)

#### `public static GeometryBase CreateFromNativePointer(IntPtr pGeometry)`

Constructs a RhinoCommon Geometry class from a given ON_Geomety*. The ON_Geometry* must be declared on the heap and its lifetime becomes controlled by RhinoCommon.

**Parameters:**
- `pGeometry` (System.IntPtr) — ON_Geometry*

**Returns:** `GeometryBase` — The appropriate geometry class in RhinoCommon on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_CreateFromNativePointer.htm)

#### `public static IntPtr FileReadOptionsConstPointer(FileReadOptions options)`

Returns the underlying const CRhinoFileReadOptions* for a Rhino.FileIO.FileReadOptions object. You should only be interested in using this function if you are writing C++ code.

**Parameters:**
- `options` (Rhino.FileIO.FileReadOptions) — A FileReadOptions object.

**Returns:** `IntPtr` — A pointer to the Rhino const object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_FileReadOptionsConstPointer.htm)

#### `public static IntPtr FileWriteOptionsConstPointer(FileWriteOptions options)`

Returns the underlying const CRhinoFileWriteOptions* for a Rhino.FileIO.FileWriteOptions object. You should only be interested in using this function if you are writing C++ code.

**Parameters:**
- `options` (Rhino.FileIO.FileWriteOptions) — A FileWriteOptions object.

**Returns:** `IntPtr` — A pointer to the Rhino const object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_FileWriteOptionsConstPointer.htm)

#### `public static Font FontFromPointer(IntPtr ptrManagedFont)`

Create managed Font from native ON_Font*

**Parameters:**
- `ptrManagedFont` (System.IntPtr) — [Missing <param name="ptrManagedFont"/> documentation for "M:Rhino.Runtime.Interop.FontFromPointer(System.IntPtr)"]

**Returns:** `Font` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.FontFromPointer(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_FontFromPointer.htm)

#### `public static Brep FromOnBrep(Object source)`

Copies a Rhino_DotNet brep to a RhinoCommon brep class.

**Parameters:**
- `source` (System.Object) — RMA.OpenNURBS.IOnBrep or RMA.OpenNURBS.OnBrep.

**Returns:** `Brep` — RhinoCommon object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_FromOnBrep.htm)

#### `public static Curve FromOnCurve(Object source)`

Copies a Rhino_DotNet curve to a RhinoCommon curve class.

**Parameters:**
- `source` (System.Object) — RMA.OpenNURBS.IOnCurve or RMA.OpenNURBS.OnCurve.

**Returns:** `Curve` — RhinoCommon object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_FromOnCurve.htm)

#### `public static Mesh FromOnMesh(Object source)`

Copies a Rhino_DotNet mesh to a RhinoCommon mesh class.

**Parameters:**
- `source` (System.Object) — RMA.OpenNURBS.IOnMesh or RMA.OpenNURBS.OnMesh.

**Returns:** `Mesh` — RhinoCommon object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_FromOnMesh.htm)

#### `public static Surface FromOnSurface(Object source)`

Copies a Rhino_DotNet surface to a RhinoCommon Surface class.

**Parameters:**
- `source` (System.Object) — Any of the following in the RMA.OpenNURBS namespace are acceptable. IOnSurface, OnSurface, IOnPlaneSurface, OnPlaneSurface, IOnClippingPlaneSurface, OnClippingPlaneSurface, IOnNurbsSurface, OnNurbsSurfac, IOnRevSurface, OnRevSurface, IOnSumSurface, OnSumSurface.

**Returns:** `Surface` — RhinoCommon object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_FromOnSurface.htm)

#### `public static IntPtr NSFontFromFont(Font font)`

Get native NSFont* from a Rhino Font. Only works on Mac

**Parameters:**
- `font` (Rhino.DocObjects.Font) — [Missing <param name="font"/> documentation for "M:Rhino.Runtime.Interop.NSFontFromFont(Rhino.DocObjects.Font)"]

**Returns:** `IntPtr` — NSFont* on success. IntPtr.Zero on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NSFontFromFont.htm)

#### `public static IntPtr NSFontFromFont(Font font, double pointSize)`

Get native NSFont* from a Rhino Font. Only works on Mac

**Parameters:**
- `font` (Rhino.DocObjects.Font) — [Missing <param name="font"/> documentation for "M:Rhino.Runtime.Interop.NSFontFromFont(Rhino.DocObjects.Font,System.Double)"]
- `pointSize` (System.Double) — Point size

**Returns:** `IntPtr` — NSFont* on success. IntPtr.Zero on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NSFontFromFont_1.htm)

#### `public static IntPtr NativeGeometryConstPointer(GeometryBase geometry)`

Returns the underlying const ON_Geometry* for a RhinoCommon class. You should only be interested in using this function if you are writing C++ code.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — A geometry object. This can be null and in such a case Zero is returned.

**Returns:** `IntPtr` — A pointer to the const geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeGeometryConstPointer.htm)

#### `public static IntPtr NativeGeometryNonConstPointer(GeometryBase geometry)`

Returns the underlying non-const ON_Geometry* for a RhinoCommon class. You should only be interested in using this function if you are writing C++ code.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — A geometry object. This can be null and in such a case Zero is returned.

**Returns:** `IntPtr` — A pointer to the non-const geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeGeometryNonConstPointer.htm)

#### `public static IntPtr NativeNonConstPointer(DisplayPipeline pipeline)`

Get CRhinoDisplayPipeline* for a DisplayPipeline instance

**Parameters:**
- `pipeline` (Rhino.Display.DisplayPipeline) — [Missing <param name="pipeline"/> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Display.DisplayPipeline)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Display.DisplayPipeline)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeNonConstPointer.htm)

#### `public static IntPtr NativeNonConstPointer(GetPoint getPoint)`

Get CRhinoGetPoint* from a GetPoint instance

**Parameters:**
- `getPoint` (Rhino.Input.Custom.GetPoint) — [Missing <param name="getPoint"/> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Input.Custom.GetPoint)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Input.Custom.GetPoint)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeNonConstPointer_4.htm)

#### `public static IntPtr NativeNonConstPointer(RhinoViewport viewport)`

Get CRhinoViewport* from a RhinoViewport instance

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Display.RhinoViewport)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Display.RhinoViewport)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeNonConstPointer_1.htm)

#### `public static IntPtr NativeNonConstPointer(ViewCaptureSettings settings)`

Get a CRhinoPrintInfo* for a given ViewCaptureSettings class

**Parameters:**
- `settings` (Rhino.Display.ViewCaptureSettings) — [Missing <param name="settings"/> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Display.ViewCaptureSettings)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.Display.ViewCaptureSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeNonConstPointer_2.htm)

#### `public static IntPtr NativeNonConstPointer(ViewportInfo viewport)`

Get ON_Viewport* from a ViewportInfo instance

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="viewport"/> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.DocObjects.ViewportInfo)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.NativeNonConstPointer(Rhino.DocObjects.ViewportInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeNonConstPointer_3.htm)

#### `public static IntPtr NativeRhinoDocPointer(RhinoDoc doc)`

Gets the C++ CRhinoDoc* for a given RhinoCommon RhinoDoc class.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A document.

**Returns:** `IntPtr` — A pointer value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_NativeRhinoDocPointer.htm)

#### `public static IntPtr PlugInPointer(PlugIn plugin)`

Gets a C++ plug-in pointer for a given RhinoCommon plug-in. This is a Rhino SDK function.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — A plug-in.

**Returns:** `IntPtr` — A pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_PlugInPointer.htm)

#### `public static IntPtr RhinoObjectConstPointer(RhinoObject rhinoObject)`

Returns the underlying const CRhinoObject* for a RhinoCommon class. You should only be interested in using this function if you are writing C++ code.

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — A Rhino object.

**Returns:** `IntPtr` — A pointer to the Rhino const object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_RhinoObjectConstPointer.htm)

#### `public static RhinoObject RhinoObjectFromPointer(IntPtr pRhinoObject)`

Constructs a RhinoCommon Rhino object from an unmanaged C++ RhinoObject pointer.

**Parameters:**
- `pRhinoObject` (System.IntPtr) — The original pointer.

**Returns:** `RhinoObject` — A new Rhino object, or null if the pointer was invalid or Zero.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_RhinoObjectFromPointer.htm)

#### `public static Object ToIRhinoViewport(RhinoViewport source)`

Convert a Rhino.Display.Viewport to an RMA.Rhino.IRhinoViewport.

**Parameters:**
- `source` (Rhino.Display.RhinoViewport) — A RhinoCommon viewport.

**Returns:** `Object` — Rhino_DotNet IRhinoViewport object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_ToIRhinoViewport.htm)

#### `public static Object ToOnBrep(Brep source)`

Constructs a Rhino_DotNet OnBrep that is a copy of a given brep.

**Parameters:**
- `source` (Rhino.Geometry.Brep) — A source brep.

**Returns:** `Object` — Rhino_DotNet object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_ToOnBrep.htm)

#### `public static Object ToOnCurve(Curve source)`

Constructs a Rhino_DotNet OnCurve that is a copy of a given curve.

**Parameters:**
- `source` (Rhino.Geometry.Curve) — A RhinoCommon source curve.

**Returns:** `Object` — Rhino_DotNet object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_ToOnCurve.htm)

#### `public static Object ToOnMesh(Mesh source)`

Constructs a Rhino_DotNet OnMesh that is a copy of a given mesh.

**Parameters:**
- `source` (Rhino.Geometry.Mesh) — A source brep.

**Returns:** `Object` — Rhino_DotNet object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_ToOnMesh.htm)

#### `public static Object ToOnSurface(Surface source)`

Constructs a Rhino_DotNet OnSurface that is a copy of a given curve.

**Parameters:**
- `source` (Rhino.Geometry.Surface) — A source brep.

**Returns:** `Object` — Rhino_DotNet object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_ToOnSurface.htm)

#### `public static Object ToOnXform(Transform source)`

Constructs a Rhino_DotNet OnXform from a given RhinoCommon Transform.

**Parameters:**
- `source` (Rhino.Geometry.Transform) — A RhinoCommon source transform.

**Returns:** `Object` — Rhino_DotNet object on success. This will be an independent copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_ToOnXform.htm)

#### `public static bool TryCopyFromOnArc(Object source, out Arc destination)`

Attempts to copy the contents of a RMA.OpenNURBS.OnArc to a Rhino.Geometry.Arc.

**Parameters:**
- `source` (System.Object) — A source OnArc.
- `destination` (Rhino.Geometry.Arc) — A destination arc.

**Returns:** `Boolean` — true if the operation succeeded; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_TryCopyFromOnArc.htm)

#### `public static bool TryCopyToOnArc(Arc source, Object destination)`

Attempts to copy the contents of a Rhino.Geometry.Arc to a RMA.OpenNURBS.OnArc.

**Parameters:**
- `source` (Rhino.Geometry.Arc) — A source arc.
- `destination` (System.Object) — A destination OnArc.

**Returns:** `Boolean` — true if the operation succeeded; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_TryCopyToOnArc.htm)

#### `public static ViewCaptureSettings ViewCaptureFromPointer(IntPtr ptrViewCapture)`

Create a ViewCaptureSettings class from a native const CRhinoPrintInfo* The pointer values are copied

**Parameters:**
- `ptrViewCapture` (System.IntPtr) — [Missing <param name="ptrViewCapture"/> documentation for "M:Rhino.Runtime.Interop.ViewCaptureFromPointer(System.IntPtr)"]

**Returns:** `ViewCaptureSettings` — [Missing <returns> documentation for "M:Rhino.Runtime.Interop.ViewCaptureFromPointer(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Interop_ViewCaptureFromPointer.htm)

## LicenseStateChangedEventArgs (class)

Passed to LicenseStateChanged event on RhinoApp

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_LicenseStateChangedEventArgs.htm)

### Constructors
- `public LicenseStateChangedEventArgs(bool callingRhinoCommonAllowed)` — LicenseStateChangedEventArgs constructor

### Properties
- `CallingRhinoCommonAllowed` (Boolean, get) — true if RhinoCommon calls will never raise Rhino.Runtime.NotLicensedException. false otherwise

## LicenseTypes (enum)

Different licensing modes that Rhino can run in

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_LicenseTypes.htm)

### Values
- `Undefined` = `0` — Licensing mode not define
- `Standalone` = `1` — Standalone license installed on this computer.
- `ZooAutoDetect` = `2` — Classic Zoo license with the Zoo server automatically detected at runtime.
- `ZooManualDetect` = `3` — Classic Zoo license with the Zoo server specified by hostname or IP
- `CloudZoo` = `4` — Cloud Zoo licenese

## Mode (enum)

Contains enumerated constant values to represent Rhino's Runtime Mode.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Mode.htm)

### Values
- `NormalMode` = `0` — Running with a commercial, educational, or evaluation license ke
- `ViewerMode` = `1` — Running as a viewer
- `BetaMode` = `2` — Running as a Beta product
- `InvalidMode` = `100` — Invalid mode; this is an error condition

## NamedParametersEventArgs (class)

Dictionary style class used for named callbacks from C++ -> .NET

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_NamedParametersEventArgs.htm)

### Constructors
- `public NamedParametersEventArgs()` — Construct a new named parameter even args. You should dispose this class when you are done with it

### Methods
#### `public void Dispose()`

Dispose native resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Dispose.htm)

#### `protected override void Finalize()`

Finalizer in case Dispose wasn't called

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Finalize.htm)

#### `public void Set(string name, Arc value)`

Set an arc for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Arc)"]
- `value` (Rhino.Geometry.Arc) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Arc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set.htm)

#### `public void Set(string name, bool value)`

Set a bool value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Boolean)"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_8.htm)

#### `public void Set(string name, Color value)`

Set a Color value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Drawing.Color)"]
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_15.htm)

#### `public void Set(string name, double value)`

Set a double value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Double)"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_14.htm)

#### `public void Set(string name, GeometryBase value)`

Set geometry for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.GeometryBase)"]
- `value` (Rhino.Geometry.GeometryBase) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.GeometryBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_1.htm)

#### `public void Set(string name, Guid value)`

Set a Color value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Guid)"]
- `value` (System.Guid) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_17.htm)

#### `public void Set(string name, IEnumerable<GeometryBase> values)`

Set a list of geometry for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase})"]
- `values` (System.Collections.Generic.IEnumerable<GeometryBase>) — [Missing <param name="values"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_10.htm)

#### `public void Set(string name, IEnumerable<Guid> guidList)`

Set a list of UUIDs as a value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{System.Guid})"]
- `guidList` (System.Collections.Generic.IEnumerable<Guid>) — [Missing <param name="guidList"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{System.Guid})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_11.htm)

#### `public void Set(string name, IEnumerable<ObjRef> values)`

Set a list of ObjRef instances as a value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]
- `values` (System.Collections.Generic.IEnumerable<ObjRef>) — [Missing <param name="values"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_9.htm)

#### `public void Set(string name, IEnumerable<string> strings)`

Set a list of strings as a value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{System.String})"]
- `strings` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="strings"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_12.htm)

#### `public void Set(string name, IEnumerable<uint> values)`

Set a list of uint as a value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{System.UInt32})"]
- `values` (System.Collections.Generic.IEnumerable<UInt32>) — [Missing <param name="values"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Collections.Generic.IEnumerable{System.UInt32})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_13.htm)

#### `public void Set(string name, int value)`

Set an int value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Int32)"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_18.htm)

#### `public void Set(string name, Line value)`

Set a line for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Line)"]
- `value` (Rhino.Geometry.Line) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Line)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_2.htm)

#### `public void Set(string name, MeshingParameters value)`

Set an MeshingParameters for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.MeshingParameters)"]
- `value` (Rhino.Geometry.MeshingParameters) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.MeshingParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_3.htm)

#### `public void Set(string name, Plane plane)`

Set a plane for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Plane)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Plane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_4.htm)

#### `public void Set(string name, Point value)`

Set a Point value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Drawing.Point)"]
- `value` (System.Drawing.Point) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_16.htm)

#### `public void Set(string name, Point3d value)`

Set a Point3d value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Point3d)"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_5.htm)

#### `public void Set(string name, Point3d[] pts)`

Set a point array for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Point3d[])"]
- `pts` (Rhino.Geometry.Point3d[]) — [Missing <param name="pts"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Point3d[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_6.htm)

#### `public void Set(string name, string value)`

Set a string value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.String)"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_19.htm)

#### `public void Set(string name, uint value)`

Set an unsigned int for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.UInt32)"]
- `value` (System.UInt32) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_20.htm)

#### `public void Set(string name, Vector3d value)`

Set a Vector3d value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Vector3d)"]
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.Set(System.String,Rhino.Geometry.Vector3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_Set_7.htm)

#### `public void SetWindowHandle(string name, IntPtr value)`

Set a HWND on Windows or NSView* on Mac

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.SetWindowHandle(System.String,System.IntPtr)"]
- `value` (System.IntPtr) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.SetWindowHandle(System.String,System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_SetWindowHandle.htm)

#### `public void SetWindowImageHandle(string name, IntPtr value)`

Set a HWND on Windows or NSView* on Mac

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.SetWindowImageHandle(System.String,System.IntPtr)"]
- `value` (System.IntPtr) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.SetWindowImageHandle(System.String,System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_SetWindowImageHandle.htm)

#### `public bool TryGetArc(string name, out Arc value)`

Get a arc for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetArc(System.String,Rhino.Geometry.Arc@)"]
- `value` (Rhino.Geometry.Arc) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetArc(System.String,Rhino.Geometry.Arc@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetArc(System.String,Rhino.Geometry.Arc@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetArc.htm)

#### `public bool TryGetBool(string name, out bool value)`

Try to get a bool value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetBool(System.String,System.Boolean@)"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetBool(System.String,System.Boolean@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetBool(System.String,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetBool.htm)

#### `public bool TryGetColor(string name, out Color value)`

Try to get a Color value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetColor(System.String,System.Drawing.Color@)"]
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetColor(System.String,System.Drawing.Color@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetColor(System.String,System.Drawing.Color@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetColor.htm)

#### `public bool TryGetDouble(string name, out double value)`

Try to get a double value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetDouble(System.String,System.Double@)"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetDouble(System.String,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetDouble(System.String,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetDouble.htm)

#### `public bool TryGetGeometry(string name, out GeometryBase[] values)`



**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGeometry(System.String,Rhino.Geometry.GeometryBase[]@)"]
- `values` (Rhino.Geometry.GeometryBase[]) — [Missing <param name="values"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGeometry(System.String,Rhino.Geometry.GeometryBase[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGeometry(System.String,Rhino.Geometry.GeometryBase[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetGeometry.htm)

#### `public bool TryGetGuid(string name, out Guid value)`

Try to get a Color value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGuid(System.String,System.Guid@)"]
- `value` (System.Guid) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGuid(System.String,System.Guid@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGuid(System.String,System.Guid@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetGuid.htm)

#### `public bool TryGetGuids(string name, out Guid[] value)`

Try to get a UUID array value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGuids(System.String,System.Guid[]@)"]
- `value` (System.Guid[]) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGuids(System.String,System.Guid[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetGuids(System.String,System.Guid[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetGuids.htm)

#### `public bool TryGetInt(string name, out int value)`

Try to get an int value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetInt(System.String,System.Int32@)"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetInt(System.String,System.Int32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetInt(System.String,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetInt.htm)

#### `public bool TryGetLine(string name, out Line value)`

Get a line for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetLine(System.String,Rhino.Geometry.Line@)"]
- `value` (Rhino.Geometry.Line) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetLine(System.String,Rhino.Geometry.Line@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetLine(System.String,Rhino.Geometry.Line@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetLine.htm)

#### `public bool TryGetMeshParameters(string name, out MeshingParameters value)`

Get an MeshingParameters for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetMeshParameters(System.String,Rhino.Geometry.MeshingParameters@)"]
- `value` (Rhino.Geometry.MeshingParameters) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetMeshParameters(System.String,Rhino.Geometry.MeshingParameters@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetMeshParameters(System.String,Rhino.Geometry.MeshingParameters@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetMeshParameters.htm)

#### `public bool TryGetObjRefs(string name, out ObjRef[] value)`

Try to get an array of ObjRef instances for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetObjRefs(System.String,Rhino.DocObjects.ObjRef[]@)"]
- `value` (Rhino.DocObjects.ObjRef[]) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetObjRefs(System.String,Rhino.DocObjects.ObjRef[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetObjRefs(System.String,Rhino.DocObjects.ObjRef[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetObjRefs.htm)

#### `public bool TryGetPlane(string name, out Plane plane)`

Get a plane for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPlane(System.String,Rhino.Geometry.Plane@)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPlane(System.String,Rhino.Geometry.Plane@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPlane(System.String,Rhino.Geometry.Plane@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetPlane.htm)

#### `public bool TryGetPoint(string name, out Point3d value)`

Try to get a Point3d value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoint(System.String,Rhino.Geometry.Point3d@)"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoint(System.String,Rhino.Geometry.Point3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoint(System.String,Rhino.Geometry.Point3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetPoint.htm)

#### `public bool TryGetPoint2i(string name, out Point value)`

Try to get a Point value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoint2i(System.String,System.Drawing.Point@)"]
- `value` (System.Drawing.Point) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoint2i(System.String,System.Drawing.Point@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoint2i(System.String,System.Drawing.Point@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetPoint2i.htm)

#### `public bool TryGetPoints(string name, out Point3d[] pts)`

Gets a point array for the specified key

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoints(System.String,Rhino.Geometry.Point3d[]@)"]
- `pts` (Rhino.Geometry.Point3d[]) — [Missing <param name="pts"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoints(System.String,Rhino.Geometry.Point3d[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetPoints(System.String,Rhino.Geometry.Point3d[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetPoints.htm)

#### `public bool TryGetRhinoObjects(string key, out RhinoObject[] values)`

Get array of RhinoObject for the specified key

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetRhinoObjects(System.String,Rhino.DocObjects.RhinoObject[]@)"]
- `values` (Rhino.DocObjects.RhinoObject[]) — [Missing <param name="values"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetRhinoObjects(System.String,Rhino.DocObjects.RhinoObject[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetRhinoObjects(System.String,Rhino.DocObjects.RhinoObject[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetRhinoObjects.htm)

#### `public bool TryGetString(string name, out string value)`

Try to get a string value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetString(System.String,System.String@)"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetString(System.String,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetString(System.String,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetString.htm)

#### `public bool TryGetStrings(string name, out string[] value)`

Try to get a string array for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetStrings(System.String,System.String[]@)"]
- `value` (System.String[]) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetStrings(System.String,System.String[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetStrings(System.String,System.String[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetStrings.htm)

#### `public bool TryGetUints(string name, out uint[] value)`

Try to get a uint array value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUints(System.String,System.UInt32[]@)"]
- `value` (System.UInt32[]) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUints(System.String,System.UInt32[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUints(System.String,System.UInt32[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetUints.htm)

#### `public bool TryGetUnmangedPointer(string name, out IntPtr value)`

Gets a HWND on Windows or NSVIew* on Mac

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUnmangedPointer(System.String,System.IntPtr@)"]
- `value` (System.IntPtr) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUnmangedPointer(System.String,System.IntPtr@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUnmangedPointer(System.String,System.IntPtr@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetUnmangedPointer.htm)

#### `public bool TryGetUnsignedInt(string name, out uint value)`

Try to get an unsigned int for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUnsignedInt(System.String,System.UInt32@)"]
- `value` (System.UInt32) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUnsignedInt(System.String,System.UInt32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetUnsignedInt(System.String,System.UInt32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetUnsignedInt.htm)

#### `public bool TryGetVector(string name, out Vector3d value)`

Try to get a Vector3d value for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetVector(System.String,Rhino.Geometry.Vector3d@)"]
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetVector(System.String,Rhino.Geometry.Vector3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetVector(System.String,Rhino.Geometry.Vector3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetVector.htm)

#### `public bool TryGetViewport(string name, out ViewportInfo viewport)`

Try to get a viewport for a given key name

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetViewport(System.String,Rhino.DocObjects.ViewportInfo@)"]
- `viewport` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="viewport"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetViewport(System.String,Rhino.DocObjects.ViewportInfo@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetViewport(System.String,Rhino.DocObjects.ViewportInfo@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetViewport.htm)

#### `public bool TryGetWindowHandle(string name, out IntPtr value)`

Gets a HWND on Windows or NSVIew* on Mac

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetWindowHandle(System.String,System.IntPtr@)"]
- `value` (System.IntPtr) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetWindowHandle(System.String,System.IntPtr@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetWindowHandle(System.String,System.IntPtr@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetWindowHandle.htm)

#### `public bool TryGetWindowImageHandle(string name, out IntPtr value)`

Gets a HWND on Windows or NSVIew* on Mac

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetWindowImageHandle(System.String,System.IntPtr@)"]
- `value` (System.IntPtr) — [Missing <param name="value"/> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetWindowImageHandle(System.String,System.IntPtr@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.NamedParametersEventArgs.TryGetWindowImageHandle(System.String,System.IntPtr@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_NamedParametersEventArgs_TryGetWindowImageHandle.htm)

## NotLicensedException (class)

Exception thrown when calling functions in RhinoCommon and the application is executing without a license

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_NotLicensedException.htm)

### Constructors
- `public NotLicensedException()` — Default constructor
- `public NotLicensedException(string message)` — Create a new instance with a custom message
- `public NotLicensedException(string message, Exception inner)` — Create a new instance with a custom message and an inner exception

## PythonCompiledCode (class)

Represents scripting compiled code.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_PythonCompiledCode.htm)

### Constructors
- `protected PythonCompiledCode()` — Initializes a new instance of the PythonCompiledCode class

### Methods
#### `public abstract void Execute(PythonScript scope)`

Executes the script in a specific scope.

**Parameters:**
- `scope` (Rhino.Runtime.PythonScript) — The scope where the script should be executed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonCompiledCode_Execute.htm)

## PythonScript (class)

Represents a Python script.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_PythonScript.htm)

### Constructors
- `protected PythonScript()` — Initializes a new instance of the PythonScript class.

### Methods
#### `public static void AddRuntimeAssembly(Assembly assembly)`

Add assembly to list of assemblies used by python

**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.Runtime.PythonScript.AddRuntimeAssembly(System.Reflection.Assembly)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_AddRuntimeAssembly.htm)

#### `public abstract PythonCompiledCode Compile(string script)`

Compiles a class in a quick-to-execute proxy.

**Parameters:**
- `script` (System.String) — A string text.

**Returns:** `PythonCompiledCode` — A Python compiled code instance.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_Compile.htm)

#### `public abstract bool ContainsVariable(string name)`

Determines if the main scripting context has a variable with a name.

**Parameters:**
- `name` (System.String) — The variable name.

**Returns:** `Boolean` — true if the variable is present.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_ContainsVariable.htm)

#### `public static PythonScript Create()`

Constructs a new Python script context.

**Returns:** `PythonScript` — A new Python script, or null if none could be created. Rhino 4 always returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_Create.htm)

#### `public abstract Object CreateTextEditorControl(string script, Action<string> helpcallback)`

Creates a control where the user is able to type Python code.

**Parameters:**
- `script` (System.String) — A starting script.
- `helpcallback` (System.Action<String>) — A method that is called when help is shown for a function, a class or a method.

**Returns:** `Object` — A Windows Forms control.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_CreateTextEditorControl.htm)

#### `public abstract Object EvaluateExpression(string statements, string expression)`

Evaluates statements and an expression in the main scripting context.

**Parameters:**
- `statements` (System.String) — One or several statements.
- `expression` (System.String) — An expression.

**Returns:** `Object` — The expression result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_EvaluateExpression.htm)

#### `public abstract bool ExecuteFile(string path)`

Executes a Python file. The file is executed in a new, __main__ scope.

**Parameters:**
- `path` (System.String) — The path to the file.

**Returns:** `Boolean` — true if the file executed. This method can throw scripting-runtime based exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_ExecuteFile.htm)

#### `public abstract bool ExecuteFileInScope(string path)`

Executes a Python file in the calling script scope. All old variables are kept.

**Parameters:**
- `path` (System.String) — The path to the file.

**Returns:** `Boolean` — true if the file executed. This method can throw scripting-runtime based exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_ExecuteFileInScope.htm)

#### `public abstract bool ExecuteScript(string script)`

Executes a Python string.

**Parameters:**
- `script` (System.String) — A Python text.

**Returns:** `Boolean` — true if the file executed. This method can throw scripting-runtime based exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_ExecuteScript.htm)

#### `protected abstract string[] GetSearchPaths()`

Protected helper function for static SearchPaths

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Runtime.PythonScript.GetSearchPaths"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_GetSearchPaths.htm)

#### `public abstract string GetStackTraceFromException(Exception ex)`

Retrieves a meaningful representation of the call stack.

**Parameters:**
- `ex` (System.Exception) — An exception that was thrown by some of the methods in this class.

**Returns:** `String` — A string that represents the Python exception.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_GetStackTraceFromException.htm)

#### `public abstract Object GetVariable(string name)`

Gets the object associated with a variable name in the main scripting context.

**Parameters:**
- `name` (System.String) — A variable name.

**Returns:** `Object` — The variable object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_GetVariable.htm)

#### `public abstract IEnumerable<string> GetVariableNames()`

Retrieves all variable names in the script.

**Returns:** `IEnumerable<String>` — An enumerable set with all names of the variables.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_GetVariableNames.htm)

#### `public abstract void RemoveVariable(string name)`

Removes a defined variable from the main scripting context.

**Parameters:**
- `name` (System.String) — The variable name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_RemoveVariable.htm)

#### `public static Assembly[] RuntimeAssemblies()`

Get list of assemblies used by python for library browser and inclusion into the runtime

**Returns:** `Assembly[]` — [Missing <returns> documentation for "M:Rhino.Runtime.PythonScript.RuntimeAssemblies"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_RuntimeAssemblies.htm)

#### `public virtual void SetIntellisenseVariable(string name, Object value)`

Sets a variable for runtime introspection.

**Parameters:**
- `name` (System.String) — A variable name.
- `value` (System.Object) — A variable value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_SetIntellisenseVariable.htm)

#### `protected abstract void SetSearchPaths(string[] paths)`

Protected helper function for static SearchPaths

**Parameters:**
- `paths` (System.String[]) — [Missing <param name="paths"/> documentation for "M:Rhino.Runtime.PythonScript.SetSearchPaths(System.String[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_SetSearchPaths.htm)

#### `public abstract void SetVariable(string name, Object value)`

Sets a variable with a name and an object. Object can be null (Nothing in Visual Basic).

**Parameters:**
- `name` (System.String) — A valid variable name in Python.
- `value` (System.Object) — A valid value for that variable name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_SetVariable.htm)

#### `public virtual void SetupScriptContext(Object doc)`

Setups the script context. Use a RhinoDoc instance unless unsure.

**Parameters:**
- `doc` (System.Object) — Document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_PythonScript_SetupScriptContext.htm)

### Properties
- `ContextId` (Int32, get/set) — Gets or sets a context unique identified.
- `Output` (Action<String>, get/set) — Gets or sets the Python script "print()" target. By default string output goes to the Rhino.RhinoApp.Write function. Set Output if you want to redirect the output from python to a different function while this script executes.
- `ScriptContextCommand` (Command, get/set) — Command associated with this script. Used for localiation
- `ScriptContextDoc` (Object, get/set) — object set to variable held in scriptcontext.doc.
- `SearchPaths` (String[], get/set) — Get/Set additional search paths used by the python interpreter

## RdkNotLoadedException (class)

Is thrown when the RDK is not loaded.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_RdkNotLoadedException.htm)

### Constructors
- `public RdkNotLoadedException()` — Initializes a new instance of the RDK not loaded exception with a standard message.

## RiskyAction (class)

Defines risky actions that need to be reported in crash exceptions

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_RiskyAction.htm)

### Constructors
- `public RiskyAction(string description, string file = "", string member = "", int line = 0)` — Always create this in a using block

### Methods
#### `public void Dispose()`

IDisposable implementation

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_RiskyAction_Dispose.htm)

## Skin (class)

Represents a customized environment that changes the appearance of Rhino. Skin DLLs must contain a single class that derives from the Skin class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Skin.htm)

### Constructors
- `protected Skin()` — Initializes a new instance of the Skin class.

### Methods
#### `protected virtual void HideSplash()`

Is called when the splash screen should be hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_HideSplash.htm)

#### `protected virtual void OnBeginLoadAtStartPlugIns(int expectedCount)`

Is called when the first plug-in that loads at start-up is going to be loaded.

**Parameters:**
- `expectedCount` (System.Int32) — The complete amount of plug-ins.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_OnBeginLoadAtStartPlugIns.htm)

#### `protected virtual void OnBeginLoadPlugIn(string description)`

Is called when a specific plug-in is going to be loaded.

**Parameters:**
- `description` (System.String) — The plug-in description.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_OnBeginLoadPlugIn.htm)

#### `protected virtual void OnBuiltInCommandsRegistered()`

Is called when built-in commands are registered.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_OnBuiltInCommandsRegistered.htm)

#### `protected virtual void OnEndLoadAtStartPlugIns()`

Is called after all of the load at start plug-ins have been loaded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_OnEndLoadAtStartPlugIns.htm)

#### `protected virtual void OnEndLoadPlugIn()`

Is called after each plug-in has been loaded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_OnEndLoadPlugIn.htm)

#### `protected virtual void OnLicenseCheckCompleted()`

Is called when the license check is completed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_OnLicenseCheckCompleted.htm)

#### `protected virtual void OnMainFrameWindowCreated()`

Is called when the main frame window is created.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_OnMainFrameWindowCreated.htm)

#### `protected virtual void ShowHelp()`

Called when the "help" splash screen should be shown. Default implementation just calls ShowSplash()

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_ShowHelp.htm)

#### `protected virtual void ShowSplash()`

Is called when the splash screen should be shown.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Skin_ShowSplash.htm)

### Properties
- `ActiveSkin` (Skin, get) — Any time Rhino is running there is at most one skin being used (and possibly no skin). If a RhinoCommon based Skin class is being used, use ActiveSkin to get at the instance of this Skin class. May return null if no Skin is being used or if the skin is not a RhinoCommon based skin.
- `ApplicationName` (String, get) — If you want to provide a custom name for your skin.
- `MainRhinoIcon` (Bitmap, get) — If you want to provide a custom icon for your skin.
- `Settings` (PersistentSettings, get) — Gets access to the skin persistent settings.

## TextFields (class)

This Class Processes Text Field Functions

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_TextFields.htm)

### Methods
#### `public static double Area(string id)`

Returns the area value for a selected object id

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.Area(System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Area(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Area.htm)

#### `public static double Area(string id, string unitSystem)`

Returns the area value for a selected object id in a specified unit system

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.Area(System.String,System.String)"]
- `unitSystem` (System.String) — [Missing <param name="unitSystem"/> documentation for "M:Rhino.Runtime.TextFields.Area(System.String,System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Area(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Area_1.htm)

#### `public static string BlockAttributeText(string key, string prompt, string defaultValue)`

User text associated with a block

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.TextFields.BlockAttributeText(System.String,System.String,System.String)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Runtime.TextFields.BlockAttributeText(System.String,System.String,System.String)"]
- `defaultValue` (System.String) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Runtime.TextFields.BlockAttributeText(System.String,System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.BlockAttributeText(System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_BlockAttributeText.htm)

#### `public static string BlockDescription(string definitionNameOrId)`

Returns a block definition description

**Parameters:**
- `definitionNameOrId` (System.String) — [Missing <param name="definitionNameOrId"/> documentation for "M:Rhino.Runtime.TextFields.BlockDescription(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.BlockDescription(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_BlockDescription.htm)

#### `public static int BlockInstanceCount(string instanceDefinitionNameOrId)`

Returns the number of block instances found in the document with the specified block definition name or the instance reference id.

**Parameters:**
- `instanceDefinitionNameOrId` (System.String) — [Missing <param name="instanceDefinitionNameOrId"/> documentation for "M:Rhino.Runtime.TextFields.BlockInstanceCount(System.String)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.BlockInstanceCount(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_BlockInstanceCount.htm)

#### `public static string BlockInstanceName(string blockId)`

Returns the block definition name of a block instance Use BlockName Instead

**Parameters:**
- `blockId` (System.String) — [Missing <param name="blockId"/> documentation for "M:Rhino.Runtime.TextFields.BlockInstanceName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.BlockInstanceName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_BlockInstanceName.htm)

#### `public static string BlockName(string blockId)`

Returns the block definition name of a block instance

**Parameters:**
- `blockId` (System.String) — [Missing <param name="blockId"/> documentation for "M:Rhino.Runtime.TextFields.BlockName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.BlockName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_BlockName.htm)

#### `public static double CurveLength(string id)`

Get length of a curve given a string id

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.CurveLength(System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.CurveLength(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_CurveLength.htm)

#### `public static double CurveLength(string id, string unitSystem)`

Get length of a curve given a string id in a specified unit system. UnitSystem enumeration as string

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.CurveLength(System.String,System.String)"]
- `unitSystem` (System.String) — [Missing <param name="unitSystem"/> documentation for "M:Rhino.Runtime.TextFields.CurveLength(System.String,System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.CurveLength(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_CurveLength_1.htm)

#### `public static string Date()`

Current date

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Date"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Date.htm)

#### `public static string Date(string dateFormat)`

Current date in a specified format

**Parameters:**
- `dateFormat` (System.String) — [Missing <param name="dateFormat"/> documentation for "M:Rhino.Runtime.TextFields.Date(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Date(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Date_1.htm)

#### `public static string Date(string dateFormat, string languageId)`

Current Date in a specific format and language

**Parameters:**
- `dateFormat` (System.String) — [Missing <param name="dateFormat"/> documentation for "M:Rhino.Runtime.TextFields.Date(System.String,System.String)"]
- `languageId` (System.String) — [Missing <param name="languageId"/> documentation for "M:Rhino.Runtime.TextFields.Date(System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Date(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Date_2.htm)

#### `public static string DateModified()`

Date the document was last edited

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.DateModified"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_DateModified.htm)

#### `public static string DateModified(string dateFormat)`

Date the document was last edited in a specified format

**Parameters:**
- `dateFormat` (System.String) — [Missing <param name="dateFormat"/> documentation for "M:Rhino.Runtime.TextFields.DateModified(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.DateModified(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_DateModified_1.htm)

#### `public static string DateModified(string dateFormat, string languageId)`

Date the document was last edited in a specific format and language in local time

**Parameters:**
- `dateFormat` (System.String) — [Missing <param name="dateFormat"/> documentation for "M:Rhino.Runtime.TextFields.DateModified(System.String,System.String)"]
- `languageId` (System.String) — [Missing <param name="languageId"/> documentation for "M:Rhino.Runtime.TextFields.DateModified(System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.DateModified(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_DateModified_2.htm)

#### `public static string DetailScale(string detailId, string scaleFormat)`

Returns a detail views scale

**Parameters:**
- `detailId` (System.String) — [Missing <param name="detailId"/> documentation for "M:Rhino.Runtime.TextFields.DetailScale(System.String,System.String)"]
- `scaleFormat` (System.String) — [Missing <param name="scaleFormat"/> documentation for "M:Rhino.Runtime.TextFields.DetailScale(System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.DetailScale(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_DetailScale.htm)

#### `public static string DocumentText(string key)`

Return document user string for a given key

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.TextFields.DocumentText(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.DocumentText(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_DocumentText.htm)

#### `public static string FileName()`

Return full path to the document

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.FileName"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_FileName.htm)

#### `public static string FileName(string options)`

Return path to the document

**Parameters:**
- `options` (System.String) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.TextFields.FileName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.FileName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_FileName_1.htm)

#### `public static TextFields.InstanceAttributeField[] GetInstanceAttributeFields(InstanceDefinition idef)`



**Parameters:**
- `idef` (Rhino.DocObjects.InstanceDefinition) — [Missing <param name="idef"/> documentation for "M:Rhino.Runtime.TextFields.GetInstanceAttributeFields(Rhino.DocObjects.InstanceDefinition)"]

**Returns:** `TextFields.InstanceAttributeField[]` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.GetInstanceAttributeFields(Rhino.DocObjects.InstanceDefinition)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_GetInstanceAttributeFields.htm)

#### `public static TextFields.InstanceAttributeField[] GetInstanceAttributeFields(string str)`

Gets an array of block attribute definitions associated with a TextObject.

**Parameters:**
- `str` (System.String) — TextObject to check for block attribute definitions

**Returns:** `TextFields.InstanceAttributeField[]` — Will return a empty array if text is null or there is no attributes otherwise; returns a list of one or more attribute definitions embedded in the text string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_GetInstanceAttributeFields_2.htm)

#### `public static TextFields.InstanceAttributeField[] GetInstanceAttributeFields(TextObject text)`

Gets an array of block attribute definitions associated with a TextObject.

**Parameters:**
- `text` (Rhino.DocObjects.TextObject) — TextObject to check for block attribute definitions

**Returns:** `TextFields.InstanceAttributeField[]` — Will return a empty array if text is null or there is no attributes otherwise; returns a list of one or more attribute definitions embedded in the text string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_GetInstanceAttributeFields_1.htm)

#### `public static string LayerName(string layerId)`

Returns the name of a layer based on the layers guid

**Parameters:**
- `layerId` (System.String) — [Missing <param name="layerId"/> documentation for "M:Rhino.Runtime.TextFields.LayerName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.LayerName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_LayerName.htm)

#### `public static string LayoutUserText(string key)`

Returns a value from the active layouts user text strings for the specified key.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.TextFields.LayoutUserText(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.LayoutUserText(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_LayoutUserText.htm)

#### `public static string LayoutUserText(string layoutId, string key)`

Returns user text value from a layout id Key

**Parameters:**
- `layoutId` (System.String) — [Missing <param name="layoutId"/> documentation for "M:Rhino.Runtime.TextFields.LayoutUserText(System.String,System.String)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.TextFields.LayoutUserText(System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.LayoutUserText(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_LayoutUserText_1.htm)

#### `public static string ModelUnits()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.ModelUnits"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_ModelUnits.htm)

#### `public static string Notes()`

Notes for a RhinoDoc

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Notes"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Notes.htm)

#### `public static int NumPages()`

Number of layout pages in a document

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.NumPages"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_NumPages.htm)

#### `public static string ObjectLayer(string id)`

Return an object's layer name

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.ObjectLayer(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.ObjectLayer(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_ObjectLayer.htm)

#### `public static string ObjectName()`

Returns an attached Text objects attribute name.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.ObjectName"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_ObjectName.htm)

#### `public static string ObjectName(string id)`

Return an object's name

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.ObjectName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.ObjectName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_ObjectName_1.htm)

#### `public static string ObjectPageName(string id)`

Returns the layout page name the object resides on

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.ObjectPageName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.ObjectPageName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_ObjectPageName.htm)

#### `public static int ObjectPageNumber(string id)`

Returns the layout page number the object resides on

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.ObjectPageNumber(System.String)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.ObjectPageNumber(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_ObjectPageNumber.htm)

#### `public static double PageHeight()`

Return the current layout page height in the layout units

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.PageHeight"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_PageHeight.htm)

#### `public static string PageName()`

Return the current layout page name

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.PageName"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_PageName.htm)

#### `public static string PageName(string id)`

Return the page name for the specified View ID

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.PageName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.PageName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_PageName_1.htm)

#### `public static int PageNumber()`

Returns the current layouts page number

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.PageNumber"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_PageNumber.htm)

#### `public static double PageWidth()`

Return the current layout page width in the layout units

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.PageWidth"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_PageWidth.htm)

#### `public static string PaperName()`

Returns the layouts selected paper name example Letter / A4 / A6

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.PaperName"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_PaperName.htm)

#### `public static string PointCoordinate(string pointId, string axis)`

returns the location of a 3d point

**Parameters:**
- `pointId` (System.String) — [Missing <param name="pointId"/> documentation for "M:Rhino.Runtime.TextFields.PointCoordinate(System.String,System.String)"]
- `axis` (System.String) — [Missing <param name="axis"/> documentation for "M:Rhino.Runtime.TextFields.PointCoordinate(System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.PointCoordinate(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_PointCoordinate.htm)

#### `public static bool TryFormat(string text, RhinoDoc doc, out string result)`

Formats the contents of a text string with field expressions and returns the result

**Parameters:**
- `text` (System.String) — The text formula to format
- `doc` (Rhino.RhinoDoc) — The Rhino document to evaluate
- `result` (System.String) — The result of the formatted expression. Otherwise, if the text is null or the evaluation process fails, the result will be empty.

**Returns:** `Boolean` — Returns True if the expression is formatted properly; Otherwise False.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_TryFormat.htm)

#### `public static bool TryParse(string text, RhinoDoc doc, out List<string> result)`

Evaluates the contents of a text string with field expressions and returns the result

**Parameters:**
- `text` (System.String) — The text formula to parse and evaluate
- `doc` (Rhino.RhinoDoc) — The Rhino document to evaluate
- `result` (System.Collections.Generic.List<String>) — The result of the evaluated expression. Otherwise, if the text is null or the evaluation process fails, the result will be empty.

**Returns:** `Boolean` — Returns True if the expression is evaluated properly; Otherwise False.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_TryParse.htm)

#### `public static string UserText(string id, string key)`

User text associated with an object, block or layout

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_UserText.htm)

#### `public static string UserText(string id, string key, string prompt)`

User text associated with an object, block or layout

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_UserText_1.htm)

#### `public static string UserText(string id, string key, string prompt, string defaultValue)`

User text associated with an object, block or layout

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String,System.String)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String,System.String)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String,System.String)"]
- `defaultValue` (System.String) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.UserText(System.String,System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_UserText_2.htm)

#### `public static double Volume(string id)`

Returns the volume for the selected object id open objects return 0 volume

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Volume.htm)

#### `public static double Volume(string id, string opt)`

Returns a volume; if the opt param is a bool "true/false" or "1/0" then open objects are inspected or ignored. if the opt param is a unit system as a string "millimeter" then the volume will be returned if the object is closed and in the specified unit system. Open objects return 0

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String,System.String)"]
- `opt` (System.String) — [Missing <param name="opt"/> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String,System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Volume_1.htm)

#### `public static double Volume(string id, string unitSystem, string allowOpenObjects)`

Returns a volume from an object Id. If allowOpenObjects = false objects which are open will return 0.0 as a volume.

**Parameters:**
- `id` (System.String) — [Missing <param name="id"/> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String,System.String,System.String)"]
- `unitSystem` (System.String) — [Missing <param name="unitSystem"/> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String,System.String,System.String)"]
- `allowOpenObjects` (System.String) — [Missing <param name="allowOpenObjects"/> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String,System.String,System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Runtime.TextFields.Volume(System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_TextFields_Volume_2.htm)

## TextFields.InstanceAttributeField (class)

[Missing <summary> documentation for "T:Rhino.Runtime.TextFields.InstanceAttributeField"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_TextFields_InstanceAttributeField.htm)

### Constructors
- `public InstanceAttributeField(string key, string prompt, string defaultValue)` — Block attribute definition.

### Properties
- `DefaultValue` (String, get) — Default value used when inserting a block
- `Key` (String, get) — Attribute key
- `Prompt` (String, get) — Prompt displayed by the UI when inserting a block

## ViewCaptureWriter (class)

Callback system used by SVG and PDF exporter to generate documents. Not intended for general SDK usage

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter.htm)

### Constructors
- `public ViewCaptureWriter(double dpi, Size pageSize)` — Initializes a new instance of the ViewCaptureWriter class

### Methods
#### `public void Draw(IntPtr constPtrPrintInfo, RhinoDoc doc)`



**Parameters:**
- `constPtrPrintInfo` (System.IntPtr) — [Missing <param name="constPtrPrintInfo"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.Draw(System.IntPtr,Rhino.RhinoDoc)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.Draw(System.IntPtr,Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_Draw.htm)

#### `protected abstract void DrawBitmap(Bitmap bitmap, double m11, double m12, double m21, double m22, double dx, double dy)`



**Parameters:**
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawBitmap(System.Drawing.Bitmap,System.Double,System.Double,System.Double,System.Double,System.Double,System.Double)"]
- `m11` (System.Double) — [Missing <param name="m11"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawBitmap(System.Drawing.Bitmap,System.Double,System.Double,System.Double,System.Double,System.Double,System.Double)"]
- `m12` (System.Double) — [Missing <param name="m12"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawBitmap(System.Drawing.Bitmap,System.Double,System.Double,System.Double,System.Double,System.Double,System.Double)"]
- `m21` (System.Double) — [Missing <param name="m21"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawBitmap(System.Drawing.Bitmap,System.Double,System.Double,System.Double,System.Double,System.Double,System.Double)"]
- `m22` (System.Double) — [Missing <param name="m22"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawBitmap(System.Drawing.Bitmap,System.Double,System.Double,System.Double,System.Double,System.Double,System.Double)"]
- `dx` (System.Double) — [Missing <param name="dx"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawBitmap(System.Drawing.Bitmap,System.Double,System.Double,System.Double,System.Double,System.Double,System.Double)"]
- `dy` (System.Double) — [Missing <param name="dy"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawBitmap(System.Drawing.Bitmap,System.Double,System.Double,System.Double,System.Double,System.Double,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_DrawBitmap.htm)

#### `protected abstract void DrawCircle(PointF center, float diameter, Color fillColor, ViewCaptureWriter.Pen stroke)`



**Parameters:**
- `center` (System.Drawing.PointF) — [Missing <param name="center"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawCircle(System.Drawing.PointF,System.Single,System.Drawing.Color,Rhino.Runtime.ViewCaptureWriter.Pen)"]
- `diameter` (System.Single) — [Missing <param name="diameter"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawCircle(System.Drawing.PointF,System.Single,System.Drawing.Color,Rhino.Runtime.ViewCaptureWriter.Pen)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawCircle(System.Drawing.PointF,System.Single,System.Drawing.Color,Rhino.Runtime.ViewCaptureWriter.Pen)"]
- `stroke` (Rhino.Runtime.ViewCaptureWriter.Pen) — [Missing <param name="stroke"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawCircle(System.Drawing.PointF,System.Single,System.Drawing.Color,Rhino.Runtime.ViewCaptureWriter.Pen)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_DrawCircle.htm)

#### `protected abstract void DrawGradientHatch(DisplayPipeline pipeline, Hatch hatch, HatchPattern pattern, Color[] gradientColors, float[] gradientStops, Point3d gradientPoint1, Point3d gradientPoint2, bool linearGradient, Color boundaryColor, double pointScale, double effectiveHatchScale)`



**Parameters:**
- `pipeline` (Rhino.Display.DisplayPipeline) — [Missing <param name="pipeline"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `pattern` (Rhino.DocObjects.HatchPattern) — [Missing <param name="pattern"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `gradientColors` (System.Drawing.Color[]) — [Missing <param name="gradientColors"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `gradientStops` (System.Single[]) — [Missing <param name="gradientStops"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `gradientPoint1` (Rhino.Geometry.Point3d) — [Missing <param name="gradientPoint1"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `gradientPoint2` (Rhino.Geometry.Point3d) — [Missing <param name="gradientPoint2"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `linearGradient` (System.Boolean) — [Missing <param name="linearGradient"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `boundaryColor` (System.Drawing.Color) — [Missing <param name="boundaryColor"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `pointScale` (System.Double) — [Missing <param name="pointScale"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]
- `effectiveHatchScale` (System.Double) — [Missing <param name="effectiveHatchScale"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawGradientHatch(Rhino.Display.DisplayPipeline,Rhino.Geometry.Hatch,Rhino.DocObjects.HatchPattern,System.Drawing.Color[],System.Single[],Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Drawing.Color,System.Double,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_DrawGradientHatch.htm)

#### `protected abstract void DrawPath(ViewCaptureWriter.PathPoint[] points, ViewCaptureWriter.Pen pen, bool linearGradient, ColorStop[] stops, Point2d[] gradientPoints, double pointScale)`



**Parameters:**
- `points` (Rhino.Runtime.ViewCaptureWriter.PathPoint[]) — [Missing <param name="points"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[],Rhino.Runtime.ViewCaptureWriter.Pen,System.Boolean,Rhino.Display.ColorStop[],Rhino.Geometry.Point2d[],System.Double)"]
- `pen` (Rhino.Runtime.ViewCaptureWriter.Pen) — [Missing <param name="pen"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[],Rhino.Runtime.ViewCaptureWriter.Pen,System.Boolean,Rhino.Display.ColorStop[],Rhino.Geometry.Point2d[],System.Double)"]
- `linearGradient` (System.Boolean) — [Missing <param name="linearGradient"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[],Rhino.Runtime.ViewCaptureWriter.Pen,System.Boolean,Rhino.Display.ColorStop[],Rhino.Geometry.Point2d[],System.Double)"]
- `stops` (Rhino.Display.ColorStop[]) — [Missing <param name="stops"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[],Rhino.Runtime.ViewCaptureWriter.Pen,System.Boolean,Rhino.Display.ColorStop[],Rhino.Geometry.Point2d[],System.Double)"]
- `gradientPoints` (Rhino.Geometry.Point2d[]) — [Missing <param name="gradientPoints"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[],Rhino.Runtime.ViewCaptureWriter.Pen,System.Boolean,Rhino.Display.ColorStop[],Rhino.Geometry.Point2d[],System.Double)"]
- `pointScale` (System.Double) — [Missing <param name="pointScale"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[],Rhino.Runtime.ViewCaptureWriter.Pen,System.Boolean,Rhino.Display.ColorStop[],Rhino.Geometry.Point2d[],System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_DrawPath.htm)

#### `protected abstract void DrawRectangle(RectangleF rect, Color fillColor, float strokeWidth, Color strokeColor, float cornerRadius)`



**Parameters:**
- `rect` (System.Drawing.RectangleF) — [Missing <param name="rect"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawRectangle(System.Drawing.RectangleF,System.Drawing.Color,System.Single,System.Drawing.Color,System.Single)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawRectangle(System.Drawing.RectangleF,System.Drawing.Color,System.Single,System.Drawing.Color,System.Single)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawRectangle(System.Drawing.RectangleF,System.Drawing.Color,System.Single,System.Drawing.Color,System.Single)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawRectangle(System.Drawing.RectangleF,System.Drawing.Color,System.Single,System.Drawing.Color,System.Single)"]
- `cornerRadius` (System.Single) — [Missing <param name="cornerRadius"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawRectangle(System.Drawing.RectangleF,System.Drawing.Color,System.Single,System.Drawing.Color,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_DrawRectangle.htm)

#### `protected abstract void DrawScreenText(string text, Color textColor, double x, double y, float angle, int horizontalAlignment, float heightPoints, Font font)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]
- `textColor` (System.Drawing.Color) — [Missing <param name="textColor"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]
- `x` (System.Double) — [Missing <param name="x"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]
- `y` (System.Double) — [Missing <param name="y"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]
- `angle` (System.Single) — [Missing <param name="angle"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]
- `horizontalAlignment` (System.Int32) — [Missing <param name="horizontalAlignment"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]
- `heightPoints` (System.Single) — [Missing <param name="heightPoints"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]
- `font` (Rhino.DocObjects.Font) — [Missing <param name="font"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.DrawScreenText(System.String,System.Drawing.Color,System.Double,System.Double,System.Single,System.Int32,System.Single,Rhino.DocObjects.Font)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_DrawScreenText.htm)

#### `protected abstract void FillPolygon(PointF[] points, Color fillColor)`



**Parameters:**
- `points` (System.Drawing.PointF[]) — [Missing <param name="points"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.FillPolygon(System.Drawing.PointF[],System.Drawing.Color)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.FillPolygon(System.Drawing.PointF[],System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_FillPolygon.htm)

#### `protected void Flush()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_Flush.htm)

#### `protected void PopClipPath()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_PopClipPath.htm)

#### `protected void PushClipPath(RectangleF rect)`



**Parameters:**
- `rect` (System.Drawing.RectangleF) — [Missing <param name="rect"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.PushClipPath(System.Drawing.RectangleF)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_PushClipPath_1.htm)

#### `protected void PushClipPath(ViewCaptureWriter.PathPoint[] points)`



**Parameters:**
- `points` (Rhino.Runtime.ViewCaptureWriter.PathPoint[]) — [Missing <param name="points"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.PushClipPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_PushClipPath.htm)

#### `protected abstract void SetClipPath(ViewCaptureWriter.PathPoint[] points)`



**Parameters:**
- `points` (Rhino.Runtime.ViewCaptureWriter.PathPoint[]) — [Missing <param name="points"/> documentation for "M:Rhino.Runtime.ViewCaptureWriter.SetClipPath(Rhino.Runtime.ViewCaptureWriter.PathPoint[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_SetClipPath.htm)

#### `protected virtual bool SupportsArc()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.ViewCaptureWriter.SupportsArc"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ViewCaptureWriter_SupportsArc.htm)

### Properties
- `Dpi` (Double, get) — 
- `PageSize` (Size, get) — 

## ViewCaptureWriter.FillProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.FillProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_FillProc.htm)

## ViewCaptureWriter.PathPoint (struct)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.PathPoint"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_PathPoint.htm)

### Properties
- `Location` (PointF, get/set) — 
- `PointType` (ViewCaptureWriter.PointType, get/set) — 

## ViewCaptureWriter.Pen (class)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.Pen"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_Pen.htm)

### Constructors
- `public Pen()` — Initializes a new instance of the ViewCaptureWriter.Pen class

### Properties
- `Cap` (LineCapStyle, get) — 
- `Color` (Color, get) — 
- `Join` (LineJoinStyle, get) — 
- `Pattern` (Single[], get) — 
- `Width` (Single, get) — 

## ViewCaptureWriter.PointType (enum)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.PointType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_PointType.htm)

### Values
- `Move` = `0`
- `Line` = `1`
- `CubicBezier` = `2`
- `Close` = `3`
- `Arc` = `4`

## ViewCaptureWriter.SetClipRectProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.SetClipRectProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_SetClipRectProc.htm)

## ViewCaptureWriter.VectorArcProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorArcProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorArcProc.htm)

## ViewCaptureWriter.VectorBitmapProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorBitmapProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorBitmapProc.htm)

## ViewCaptureWriter.VectorClipPathProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorClipPathProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorClipPathProc.htm)

## ViewCaptureWriter.VectorFillPolygonProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorFillPolygonProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorFillPolygonProc.htm)

## ViewCaptureWriter.VectorGradientProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorGradientProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorGradientProc.htm)

## ViewCaptureWriter.VectorPathProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorPathProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorPathProc.htm)

## ViewCaptureWriter.VectorPointProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorPointProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorPointProc.htm)

## ViewCaptureWriter.VectorPolylineProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorPolylineProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorPolylineProc.htm)

## ViewCaptureWriter.VectorRoundedRectProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorRoundedRectProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorRoundedRectProc.htm)

## ViewCaptureWriter.VectorStringProc (delegate)

[Missing <summary> documentation for "T:Rhino.Runtime.ViewCaptureWriter.VectorStringProc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ViewCaptureWriter_VectorStringProc.htm)

## ZooClientParameters (class)

ZooClientParameters is a read-only set of parameters that control the flow of licensing inside ZooClient. Because this class flows through a number of other classes, functions, and UI, it is read-only so that inadvertent changes are not made to the data as it propagates from the caller.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_ZooClientParameters.htm)

### Constructors
- `public ZooClientParameters(Guid productGuid, Guid licenseGuid, string productTitle, int productBuildType, LicenseCapabilities capabilities, string licenseEntryTextMask, string productPath, Object parentWindow, LicenseTypes selectedLicenseType, ValidateProductKeyDelegate validateProductKey, OnLeaseChangedDelegate onLeaseChangedDelegate, VerifyLicenseKeyDelegate verifyLicenseKeyDelegate, VerifyPreviousVersionLicenseDelegate verifyPreviousVersionLicenseKeyDelegate)` — ZooClientParameters Constructor

### Methods
#### `public ValidateResult VerifyLicenseKey(string licenseKey, string validationCode, DateTime validationCodeInstallDate, bool gracePeriodExpired, out LicenseData licenseData)`

Called by GetLicense to ensure that the license key entered by the user is legitimate and can be used.

**Parameters:**
- `licenseKey` (System.String) — License key string entered by user
- `validationCode` (System.String) — Validation code entered by user (only if a previous call to VerifyLicenseKey set LicenseData.RequiresOnlineValidation to true).
- `validationCodeInstallDate` (System.DateTime) — Date that validation code was entered by user (only if a previous call to VerifyLicenseKey set LicenseData.RequiresOnlineValidation to true).
- `gracePeriodExpired` (System.Boolean) — Date by which license validation must complete successfully.
- `licenseData` (Rhino.PlugIns.LicenseData) — Output parameter where return data about the license is set.

**Returns:** `ValidateResult` — [Missing <returns> documentation for "M:Rhino.Runtime.ZooClientParameters.VerifyLicenseKey(System.String,System.String,System.DateTime,System.Boolean,Rhino.PlugIns.LicenseData@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ZooClientParameters_VerifyLicenseKey.htm)

#### `public bool VerifyPreviousVersionLicense(string license, string previousVersionLicense, out string errorMessage)`

When a caller calls GetLicense, ZooClient may call VerifyPreviousVersionLicense to ensure previousVersionLicense is legitimate and can be used to upgrade license.

**Parameters:**
- `license` (System.String) — License key for current product. This was returned by a previous call to VerifyLicenseKey or ValidateProductKey.
- `previousVersionLicense` (System.String) — License key entered by user to show upgrade eligibility for license.
- `errorMessage` (System.String) — Error message to be displayed to user if something isn't correct.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.ZooClientParameters.VerifyPreviousVersionLicense(System.String,System.String,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_ZooClientParameters_VerifyPreviousVersionLicense.htm)

### Properties
- `Capabilities` (LicenseCapabilities, get/set) — LicenseCapabilities flags that set options for how licenses can be obtained for this product
- `LicenseEntryTextMask` (String, get) — Text mask in the form @"RH4A-AAAA-AAAA-AAAA-AAAA-AAAA" that informs the user what numbers they are looking for
- `LicenseGuid` (Guid, get) — Guid used by ZooClient to identify the license saved by ZooClient. This differs from ProductGuid because different versions of a plug-in with the same Plug-in ID may need different licenses.
- `OnLeaseChanged` (OnLeaseChangedDelegate, get) — Delegate called by ZooClient when a cloud zoo lease is changed
- `ParentWindow` (Object, get) — Parent window assigned to any licensing dialogs that appear. If null, the Rhino main window is used.
- `ProductBuildType` (Int32, get) — Product build type. Must be one of LicenseBuildType values.
- `ProductGuid` (Guid, get) — Guid used by ZooClient to identify the plug-in requesting a license from ZooClient. This Guid may be used by different versions of the plug-in. If different licenses are used by different versions of the plug-in, the plug-in must also specify a LicenseGuid.
- `ProductPath` (String, get) — Path to the application calling ZooClient
- `ProductTitle` (String, get) — Title of the product, "Rhinoceros 6" for example.
- `SelectedLicenseType` (LicenseTypes, get/set) — License type selected by default when user is prompted to enter a license key

