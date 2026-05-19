---
name: dynamo-
description: This skill encodes the dynamo 4.1.1 surface of the  namespace — 98 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CommandLineArguments, LuceneStorage, BrowserItemHandler, SearchElementHandler, TaskPriority, TaskMergeInstruction, State, Decision, and 90 more types.
---

# 

Auto-generated from vendor docs for dynamo 4.1.1. 98 types in this namespace.

## ASC_STATUS (enum)

Type ASC_STATUS

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Tools/DynamoInstallDetective/AscSDKWrapper.cs)

### Values
- `BAD_ARGS` = `5`
- `EMPTY_REG_VERSION` = `13`
- `FAILED` = `100`
- `FUNCTION_CALL_FAILED` = `7`
- `INCORRECT_REG_PATH` = `12`
- `INITIALIZE_FAILED` = `8`
- `ODIS_SDK_INITIALIZE_FAILED` = `9`
- `ODIS_SDK_LOCK_FAILED` = `10`
- `ODIS_SDK_UNLOCK_FAILED` = `11`
- `REG_FAILED` = `6`
- `SUCCESS` = `0`
- `UNABLE_TO_SET_INSTALL_PATH` = `4`

## AddGroupToGroupCommand (class)

Type AddGroupToGroupCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void AddGroupToGroupCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid, System.Guid hostModelGuid)` — Creates a command to add a AnnotationModel object to another AnnotationModel.
- `void AddGroupToGroupCommand(System.Guid modelGuid, System.Guid hostModelGuid)` — Creates a command to add a AnnotationModel object to another AnnotationModel.
- `void AddGroupToGroupCommand(string modelGuid, string hostModelGuid)` — Creates a command to add a AnnotationModel object to another AnnotationModel.

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

AddGroupToGroupCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

AddGroupToGroupCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `HostGroupGuid` (System.Guid, get/set) — Id of the the group that should host the other group.

## AddModelToGroupCommand (class)

Type AddModelToGroupCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void AddModelToGroupCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid)` — 
- `void AddModelToGroupCommand(System.Guid modelGuid)` — 
- `void AddModelToGroupCommand(string modelGuid)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

AddModelToGroupCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

AddModelToGroupCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## AddPresetCommand (class)

Type AddPresetCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void AddPresetCommand(string name, string description, System.Collections.Generic.IEnumerable<System.Guid> currentSelectionIds)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

AddPresetCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

AddPresetCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## ApplyPresetCommand (class)

Type ApplyPresetCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void ApplyPresetCommand(System.Guid workspaceID, System.Guid stateID)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

ApplyPresetCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ApplyPresetCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## AskForTrustedLocationResult (enum)

Type AskForTrustedLocationResult

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/PreferenceSettings.cs)

### Values
- `Ask` = `0` — Ask for the Trusted location
- `DontAsk` = `1` — Don't ask about the Trusted location
- `UnableToAsk` = `2` — Unable to ask about the Trusted location

## AssemblyCertificateCheckException (class)

Type AssemblyCertificateCheckException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/CertificateVerification.cs)

### Constructors
- `void AssemblyCertificateCheckException(string assemblyPath)` — AssemblyCertificateCheckException.AssemblyCertificateCheckException

## AssemblyLoadedEventArgs (class)

Type AssemblyLoadedEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

### Constructors
- `void AssemblyLoadedEventArgs(System.Reflection.Assembly assembly)` — Creates AssemblyLoadedEventArgs

### Properties
- `Assembly` (System.Reflection.Assembly, get) — Loaded assembly.

## AssemblyLoadedHandler (delegate)

Type AssemblyLoadedHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

### Constructors
- `void AssemblyLoadedHandler(object object, IntPtr method)` — AssemblyLoadedHandler.AssemblyLoadedHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs args, System.AsyncCallback callback, object object)`

AssemblyLoadedHandler.BeginInvoke

**Parameters:**
- `args` (Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

#### `void EndInvoke(System.IAsyncResult result)`

AssemblyLoadedHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

#### `void Invoke(Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs args)`

AssemblyLoadedHandler.Invoke

**Parameters:**
- `args` (Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

## BreakpointOptions (enum)

Type BreakpointOptions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Values
- `EmitCallrForTempBreakpoint` = `4`
- `EmitIdentifierBreakpoint` = `1`
- `EmitInlineConditionalBreakpoint` = `8`
- `EmitPopForTempBreakpoint` = `2`
- `None` = `0`
- `SuppressNullVarDeclarationBreakpoint` = `16`

## BrowserItemHandler (delegate)

Type BrowserItemHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/BrowserItem.cs)

### Constructors
- `void BrowserItemHandler(object object, IntPtr method)` — BrowserItemHandler.BrowserItemHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Search.BrowserItem ele, System.AsyncCallback callback, object object)`

BrowserItemHandler.BeginInvoke

**Parameters:**
- `ele` (Dynamo.Search.BrowserItem)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/BrowserItem.cs)

#### `void EndInvoke(System.IAsyncResult result)`

BrowserItemHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/BrowserItem.cs)

#### `void Invoke(Dynamo.Search.BrowserItem ele)`

BrowserItemHandler.Invoke

**Parameters:**
- `ele` (Dynamo.Search.BrowserItem)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/BrowserItem.cs)

## BuiltInMethod (class)

Type BuiltInMethod

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/BuiltInMethods.cs)

### Constructors
- `void BuiltInMethod()` — BuiltInMethod.BuiltInMethod

### Properties
- `ID` (ProtoCore.Lang.BuiltInMethods.MethodID, get/set) — BuiltInMethod.ID
- `MethodAttributes` (ProtoCore.AST.AssociativeAST.MethodAttributes, get/set) — BuiltInMethod.MethodAttributes
- `Parameters` (System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, ProtoCore.Type>>, get/set) — BuiltInMethod.Parameters
- `ReturnType` (ProtoCore.Type, get/set) — BuiltInMethod.ReturnType

## CaseInfo (class)

Type CaseInfo

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeSwitch.cs)

### Constructors
- `void CaseInfo()` — CaseInfo.CaseInfo

### Properties
- `Action` (System.Action<object>, get/set) — CaseInfo.Action
- `IsDefault` (bool, get/set) — CaseInfo.IsDefault
- `Target` (System.Type, get/set) — CaseInfo.Target

## Categories (static-class)

Type Categories

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

## CommandLineArguments (struct)

Type CommandLineArguments

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoApplications/StartupUtils.cs)

### Methods
#### `Dynamo.Applications.StartupUtils.CommandLineArguments Parse(string[] args)`

CommandLineArguments.Parse

**Parameters:**
- `args` (string[])

**Returns:** `Dynamo.Applications.StartupUtils.CommandLineArguments` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoApplications/StartupUtils.cs)

### Properties
- `ASMPath` (string, get/set) — CommandLineArguments.ASMPath
- `AnalyticsInfo` (Dynamo.Models.HostAnalyticsInfo, get/set) — CommandLineArguments.AnalyticsInfo
- `CERLocation` (string, get/set) — CommandLineArguments.CERLocation
- `CommandFilePath` (string, get/set) — CommandLineArguments.CommandFilePath
- `CommonDataFolder` (string, get/set) — CommandLineArguments.CommonDataFolder
- `ConvertFile` (bool, get/set) — CommandLineArguments.ConvertFile
- `DisableAnalytics` (bool, get/set) — CommandLineArguments.DisableAnalytics
- `GeometryFilePath` (string, get/set) — CommandLineArguments.GeometryFilePath
- `ImportedPaths` (System.Collections.Generic.IEnumerable<string>, get/set) — CommandLineArguments.ImportedPaths
- `KeepAlive` (bool, get/set) — CommandLineArguments.KeepAlive
- `Locale` (string, get/set) — CommandLineArguments.Locale
- `NoConsole` (bool, get/set) — CommandLineArguments.NoConsole
- `NoNetworkMode` (bool, get/set) — CommandLineArguments.NoNetworkMode
- `OpenFilePath` (string, get/set) — CommandLineArguments.OpenFilePath
- `ServiceMode` (bool, get/set) — Boolean indication of launching Dynamo in service mode, this mode is optimized for minimal launch time
- `UserDataFolder` (string, get/set) — CommandLineArguments.UserDataFolder
- `Verbose` (string, get/set) — CommandLineArguments.Verbose

## CommentType (enum)

Type CommentType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Values
- `Block` = `1`
- `Inline` = `0`

## CompletionType (enum)

Type CompletionType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeCompletion/CodeCompletionServices.cs)

### Values
- `Class` = `3`
- `Constructor` = `2`
- `Keyword` = `5`
- `Method` = `1`
- `Namespace` = `0`
- `Property` = `4`

## ConvertNodesToCodeCommand (class)

Type ConvertNodesToCodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

ConvertNodesToCodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ConvertNodesToCodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## CrashPromptHandler (delegate)

Type CrashPromptHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

### Constructors
- `void CrashPromptHandler(object object, IntPtr method)` — CrashPromptHandler.CrashPromptHandler

### Methods
#### `System.IAsyncResult BeginInvoke(object sender, Dynamo.Core.CrashPromptArgs e, System.AsyncCallback callback, object object)`

CrashPromptHandler.BeginInvoke

**Parameters:**
- `sender` (object)
- `e` (Dynamo.Core.CrashPromptArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void EndInvoke(System.IAsyncResult result)`

CrashPromptHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void Invoke(object sender, Dynamo.Core.CrashPromptArgs e)`

CrashPromptHandler.Invoke

**Parameters:**
- `sender` (object)
- `e` (Dynamo.Core.CrashPromptArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

## CrashReporterStartupOptions (struct)

Type CrashReporterStartupOptions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `CERLocation` (string, get/set) — The Autodesk CrashReport tool location on disk (directory that contains the "senddmp.exe")

## CreateAndConnectNodeCommand (class)

Type CreateAndConnectNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateAndConnectNodeCommand(System.Guid newNodeGuid, System.Guid existingNodeGuid, string newNodeName, int outPortIndex, int inPortIndex, double x, double y, bool createAsDownstreamNode, bool addNewNodeToSelection)` — Creates a new CreateAndConnectNodeCommand with the given inputs

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateAndConnectNodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateAndConnectNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `NewNodeName` (string, get) — CreateAndConnectNodeCommand.NewNodeName

## CreateAnnotationCommand (class)

Type CreateAnnotationCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateAnnotationCommand(System.Collections.Generic.IEnumerable<System.Guid> annotationId, string annotationText, double x, double y, bool defaultPosition)` — Recordable command to create a new AnnotationModel
- `void CreateAnnotationCommand(System.Collections.Generic.IEnumerable<System.Guid> annotationId, string annotationText, string annotationDescriptionText, double x, double y, bool defaultPosition)` — Recordable command to create a new AnnotationModel
- `void CreateAnnotationCommand(System.Guid annotationId, string annotationText, double x, double y, bool defaultPosition)` — 
- `void CreateAnnotationCommand(System.Guid annotationId, string annotationText, string annotationDescriptionText, double x, double y, bool defaultPosition)` — Recordable command to create a new AnnotationModel

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateAnnotationCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateAnnotationCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## CreateCustomNodeCommand (class)

Type CreateCustomNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateCustomNodeCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeId, string name, string category, string description, bool makeCurrent)` — 
- `void CreateCustomNodeCommand(System.Guid nodeId, string name, string category, string description, bool makeCurrent)` — 
- `void CreateCustomNodeCommand(string nodeId, string name, string category, string description, bool makeCurrent)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateCustomNodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateCustomNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## CreateNodeCommand (class)

Type CreateNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateNodeCommand(Dynamo.Graph.Nodes.NodeModel node, double x, double y, bool defaultPosition, bool transformCoordinates)` — 
- `void CreateNodeCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeId, string nodeName, double x, double y, bool defaultPosition, bool transformCoordinates)` — 
- `void CreateNodeCommand(string nodeId, string nodeName, double x, double y, bool defaultPosition, bool transformCoordinates)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateNodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `Name` (string, get) — CreateNodeCommand.Name

## CreateNoteCommand (class)

Type CreateNoteCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateNoteCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeIds, string noteText, double x, double y, bool defaultPosition)` — 
- `void CreateNoteCommand(System.Guid nodeId, string noteText, double x, double y, bool defaultPosition)` — 
- `void CreateNoteCommand(string nodeId, string noteText, double x, double y, bool defaultPosition)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateNoteCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateNoteCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## CreateProxyNodeCommand (class)

Type CreateProxyNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateProxyNodeCommand(string nodeId, string customnodeFunctionId, double x, double y, bool defaultPosition, bool transformCoordinates, string name, int inputs, int outputs)` — 

### Methods
#### `void SerializeCore(System.Xml.XmlElement element)`

CreateProxyNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## DebugMode (class)

Type DebugMode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/DebugModes.cs)

### Constructors
- `void DebugMode()` — DebugMode.DebugMode

## Decision (enum)

Type Decision

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Values
- `Abort` = `0` — The migration should not proceed and the file open operation should be aborted. This can be used to indicate that a version of file that is no longer supported and no migration path is provided.
- `Migrate` = `1` — File migration should proceed to migrate the older file version to a newer one.
- `Retain` = `2` — The file version is up-to-date and the file can be used as-is without migration.

## DefaultStartConfiguration (struct)

Type DefaultStartConfiguration

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModel.cs)

### Constructors
- `void DefaultStartConfiguration()` — Initializes a new instance of the DefaultStartConfiguration struct.

### Properties
- `AuthProvider` (Greg.IAuthProvider, get/set) — DefaultStartConfiguration.AuthProvider
- `CLILocale` (string, get/set) — DefaultStartConfiguration.CLILocale
- `CLIMode` (bool, get/set) — CLIMode indicates if we are running in DynamoCLI or DynamoWPFCLI mode.
- `Context` (string, get/set) — DefaultStartConfiguration.Context
- `DefaultPythonEngine` (string, get/set) — Default Python script engine
- `DynamoCorePath` (string, get/set) — DefaultStartConfiguration.DynamoCorePath
- `DynamoHostPath` (string, get/set) — DefaultStartConfiguration.DynamoHostPath
- `EnableUnTrustedLocationsNotifications` (bool, get/set) — Enables or disables notifications about untrusted file locations. When true (default), notifications are shown in the notification center. When false, no notifications are displayed for untrusted locations.
- `Extensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>, get/set) — DefaultStartConfiguration.Extensions
- `GeometryFactoryPath` (string, get/set) — DefaultStartConfiguration.GeometryFactoryPath
- `HostAnalyticsInfo` (Dynamo.Models.HostAnalyticsInfo, get/set) — DefaultStartConfiguration.HostAnalyticsInfo
- `IsHeadless` (bool, get/set) — DefaultStartConfiguration.IsHeadless
- `IsServiceMode` (bool, get/set) — DefaultStartConfiguration.IsServiceMode
- `Logger` (Dynamo.Logging.DynamoLogger, get/set) — Gets or sets the logger instance used for logging messages and events. This property should be set to a valid Dynamo.Logging.DynamoLogger instance to enable logging during the initialization and runtime of Dynamo.
- `NoNetworkMode` (bool, get/set) — DefaultStartConfiguration.NoNetworkMode
- `PathResolver` (Dynamo.Interfaces.IPathResolver, get/set) — DefaultStartConfiguration.PathResolver
- `Preferences` (Dynamo.Interfaces.IPreferences, get/set) — DefaultStartConfiguration.Preferences
- `ProcessMode` (Dynamo.Scheduler.TaskProcessMode, get/set) — DefaultStartConfiguration.ProcessMode
- `PythonTemplatePath` (string, get/set) — DefaultStartConfiguration.PythonTemplatePath
- `SchedulerThread` (Dynamo.Scheduler.ISchedulerThread, get/set) — DefaultStartConfiguration.SchedulerThread
- `StartInTestMode` (bool, get/set) — DefaultStartConfiguration.StartInTestMode

## DeleteModelCommand (class)

Type DeleteModelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void DeleteModelCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuids)` — 
- `void DeleteModelCommand(System.Guid modelGuid)` — 
- `void DeleteModelCommand(string modelGuid)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

DeleteModelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

DeleteModelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## DisplayOptions (enum)

Type DisplayOptions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CrashPromptArgs.cs)

### Values
- `HasDetails` = `2`
- `HasFilePath` = `4`
- `IsDefaultTextOverridden` = `1`

## DisposeDelegate (delegate)

Type DisposeDelegate

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/RuntimeCore.cs)

### Constructors
- `void DisposeDelegate(object object, IntPtr method)` — DisposeDelegate.DisposeDelegate

### Methods
#### `System.IAsyncResult BeginInvoke(ProtoCore.RuntimeCore sender, System.AsyncCallback callback, object object)`

DisposeDelegate.BeginInvoke

**Parameters:**
- `sender` (ProtoCore.RuntimeCore)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void EndInvoke(System.IAsyncResult result)`

DisposeDelegate.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void Invoke(ProtoCore.RuntimeCore sender)`

DisposeDelegate.Invoke

**Parameters:**
- `sender` (ProtoCore.RuntimeCore)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/RuntimeCore.cs)

## DragSelectionCommand (class)

Type DragSelectionCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void DragSelectionCommand(Dynamo.Utilities.Point2D mouseCursor, Dynamo.Models.DynamoModel.DragSelectionCommand.Operation operation)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

DragSelectionCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

DragSelectionCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## DynamoModelState (enum)

Type DynamoModelState

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModel.cs)

### Values
- `NotStarted` = `0`
- `StartedUI` = `2`
- `StartedUIless` = `1`

## ErrorEntry (struct)

Type ErrorEntry

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/BuildStatus.cs)

## ErrorType (enum)

Type ErrorType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/BuildStatus.cs)

### Values
- `Error` = `1`
- `OK` = `0`
- `Warning` = `2`

## ForceRunCancelCommand (class)

Type ForceRunCancelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void ForceRunCancelCommand(bool showErrors, bool cancelRun)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

ForceRunCancelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## FutureFileCallback (delegate)

Type FutureFileCallback

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void FutureFileCallback(object object, IntPtr method)` — FutureFileCallback.FutureFileCallback

### Methods
#### `System.IAsyncResult BeginInvoke(string fileName, System.Version fileVersion, System.Version currentVersion, System.AsyncCallback callback, object object)`

FutureFileCallback.BeginInvoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `bool EndInvoke(System.IAsyncResult result)`

FutureFileCallback.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `bool Invoke(string fileName, System.Version fileVersion, System.Version currentVersion)`

FutureFileCallback.Invoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

## GCMark (enum)

Type GCMark

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Heap.cs)

### Values
- `Black` = `2`
- `Gray` = `1`
- `White` = `0`

## IIndexed<T> (interface)

Type IIndexed<T>

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

### Properties
- `Element` (T, get) — IIndexed.Element
- `Index` (int, get) — IIndexed.Index

## IStartConfigCrashReporter (interface)

Type IStartConfigCrashReporter

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `CRStartConfig` (Dynamo.Models.DynamoModel.CrashReporterStartupOptions, get/set) — CERLocation

## IStartConfiguration (interface)

Type IStartConfiguration

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `AuthProvider` (Greg.IAuthProvider, get/set) — IStartConfiguration.AuthProvider
- `Context` (string, get/set) — IStartConfiguration.Context
- `DynamoCorePath` (string, get/set) — IStartConfiguration.DynamoCorePath
- `DynamoHostPath` (string, get/set) — IStartConfiguration.DynamoHostPath
- `EnableUnTrustedLocationsNotifications` (bool, get) — Enables or disables notifications about untrusted file locations. When true (default), notifications are shown in the notification center when trusted locations contain paths pointing to the common application data (ProgramData) directory. When false, no notifications are displayed for untrusted locations. Default value is true to maintain backward compatibility. Note: This does not affect the file trust warning popup when opening files from untrusted locations.
- `Extensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>, get/set) — IStartConfiguration.Extensions
- `GeometryFactoryPath` (string, get/set) — IStartConfiguration.GeometryFactoryPath
- `HostAnalyticsInfo` (Dynamo.Models.HostAnalyticsInfo, get/set) — Configuration object that contains host information like Host name, parent id and session id.
- `IsHeadless` (bool, get/set) — If true, the program does not have a UI. No update checks or analytics collection should be done.
- `NoNetworkMode` (bool, get) — Configuration option to start Dynamo in offline mode.
- `PathResolver` (Dynamo.Interfaces.IPathResolver, get/set) — IStartConfiguration.PathResolver
- `Preferences` (Dynamo.Interfaces.IPreferences, get/set) — IStartConfiguration.Preferences
- `ProcessMode` (Dynamo.Scheduler.TaskProcessMode, get/set) — IStartConfiguration.ProcessMode
- `SchedulerThread` (Dynamo.Scheduler.ISchedulerThread, get/set) — IStartConfiguration.SchedulerThread
- `StartInTestMode` (bool, get/set) — IStartConfiguration.StartInTestMode

## IStartConfigurationLogger (interface)

Type IStartConfigurationLogger

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `Logger` (Dynamo.Logging.DynamoLogger, get/set) — Specify the logger instance.

## InsertFileCommand (class)

Type InsertFileCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void InsertFileCommand(string filePath, bool forceManualExecutionMode)` — Insert Graph or Custom Node from a file path into the current Workspace

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

InsertFileCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

InsertFileCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## LibraryLoadFailedEventArgs (class)

Type LibraryLoadFailedEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

### Constructors
- `void LibraryLoadFailedEventArgs(string libraryPath, string reason, bool throwOnFailure)` — LibraryLoadFailedEventArgs.LibraryLoadFailedEventArgs

### Properties
- `LibraryPath` (string, get) — The path to the library that failed to load
- `Reason` (string, get) — The reason that the library failed to load
- `ThrowOnFailure` (bool, get) — Indicates if the failure should result in an exception being thrown

## LibraryLoadedEventArgs (class)

Type LibraryLoadedEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

### Constructors
- `void LibraryLoadedEventArgs(System.Collections.Generic.IEnumerable<string> libraryPaths)` — LibraryLoadedEventArgs.LibraryLoadedEventArgs

### Properties
- `LibraryPaths` (System.Collections.Generic.IEnumerable<string>, get) — Paths to libraries that are loaded.

## LibraryLoadingEventArgs (class)

Type LibraryLoadingEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

### Constructors
- `void LibraryLoadingEventArgs(string libraryPath)` — LibraryLoadingEventArgs.LibraryLoadingEventArgs

### Properties
- `LibraryPath` (string, get) — LibraryLoadingEventArgs.LibraryPath

## LogWarningHandler (delegate)

Type LogWarningHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/MigrationRewriter.cs)

### Constructors
- `void LogWarningHandler(object object, IntPtr method)` — LogWarningHandler.LogWarningHandler

### Methods
#### `System.IAsyncResult BeginInvoke(string oldNodeName, string newNodeName, System.AsyncCallback callback, object object)`

LogWarningHandler.BeginInvoke

**Parameters:**
- `oldNodeName` (string)
- `newNodeName` (string)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/MigrationRewriter.cs)

#### `void EndInvoke(System.IAsyncResult result)`

LogWarningHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/MigrationRewriter.cs)

#### `void Invoke(string oldNodeName, string newNodeName)`

LogWarningHandler.Invoke

**Parameters:**
- `oldNodeName` (string)
- `newNodeName` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/MigrationRewriter.cs)

## LuceneStorage (enum)

Type LuceneStorage

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Utilities/LuceneSearchUtility.cs)

### Values
- `FILE_SYSTEM` = `1`
- `RAM` = `0`

## MakeConnectionCommand (class)

Type MakeConnectionCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void MakeConnectionCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeId, int portIndex, Dynamo.Graph.Nodes.PortType portType, Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode mode)` — Recordable command ConnectionCommand constructor
- `void MakeConnectionCommand(System.Guid nodeId, int portIndex, Dynamo.Graph.Nodes.PortType portType, Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode mode)` — Recordable command ConnectionCommand constructor
- `void MakeConnectionCommand(string nodeId, int portIndex, Dynamo.Graph.Nodes.PortType portType, Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode mode)` — Recordable command ConnectionCommand constructor

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

MakeConnectionCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

MakeConnectionCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `ConnectionMode` (Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode, get) — MakeConnectionCommand.ConnectionMode
- `PortIndex` (int, get) — MakeConnectionCommand.PortIndex

## MessageType (enum)

Type MessageType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/BuildStatus.cs)

### Values
- `Error` = `2`
- `Info` = `0`
- `Warning` = `1`

## MethodID (enum)

Type MethodID

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/BuiltInMethods.cs)

### Values
- `AllFalse` = `0`
- `AllTrue` = `1`
- `Average` = `2`
- `Break` = `47`
- `Concat` = `3`
- `ConditionalIf` = `55`
- `Contains` = `4`
- `ContainsKey` = `51`
- `Count` = `5`
- `CountFalse` = `7`
- `CountTrue` = `6`
- `Difference` = `8`
- `Dot` = `9`
- `Equals` = `10`
- `Evaluate` = `52`
- `Flatten` = `13`
- `GC` = `54`
- `GetElapsedTime` = `11`
- `GetKeys` = `48`
- `GetType` = `12`
- `GetValues` = `49`
- `ImportData` = `14`
- `IndexOf` = `15`
- `InlineConditional` = `46`
- `Insert` = `16`
- `Intersection` = `17`
- `InvalidMethodID` = `-1`
- `IsHomogeneous` = `20`
- `IsRectangular` = `19`
- `IsUniformDepth` = `18`
- `LoadCSV` = `21`
- `Map` = `22`
- `MapTo` = `23`
- `NodeAstFailed` = `53`
- `NormalizeDepth` = `24`
- `Print` = `25`
- `RangeExpression` = `40`
- `Rank` = `26`
- `Remove` = `27`
- `RemoveDuplicates` = `28`
- `RemoveIfNot` = `30`
- `RemoveKey` = `50`
- `RemoveNulls` = `29`
- `Reorder` = `39`
- `Reverse` = `31`
- `Sleep` = `32`
- `SomeFalse` = `33`
- `SomeNulls` = `34`
- `SomeTrue` = `35`
- `Sort` = `36`
- `SortIndexByValue` = `37`
- `SortPointer` = `38`
- `Sum` = `41`
- `ToStringFromArray` = `43`
- `ToStringFromArrayAndFormat` = `57`
- `ToStringFromObject` = `42`
- `ToStringFromObjectAndFormat` = `56`
- `Transpose` = `44`
- `Union` = `45`

## Mode (enum)

Type Mode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Values
- `Begin` = `0` — Begin connection.
- `BeginCreateConnections` = `7` — End current connection and create new connections.
- `BeginDuplicateConnection` = `6` — Begin duplicate connection.
- `BeginShiftReconnections` = `2` — Begin shift reconnections.
- `Cancel` = `5` — Cancel connection.
- `End` = `1` — End connection.
- `EndAndStartCtrlConnection` = `4` — End and start control connections.
- `EndShiftReconnections` = `3` — End shift reconnections.

## ModelBasedRecordableCommand (class)

Type ModelBasedRecordableCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void ModelBasedRecordableCommand(System.Collections.Generic.IEnumerable<System.Guid> guids)` — A collection of System.Guid identifying the objects on which to operate.

### Methods
#### `System.Collections.Generic.IEnumerable<System.Guid> DeserializeGuid(System.Xml.XmlElement element, Dynamo.Utilities.XmlElementHelper helper)`

ModelBasedRecordableCommand.DeserializeGuid

**Parameters:**
- `element` (System.Xml.XmlElement)
- `helper` (Dynamo.Utilities.XmlElementHelper)

**Returns:** `System.Collections.Generic.IEnumerable<System.Guid>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ModelBasedRecordableCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `ModelGuid` (System.Guid, get) — The first System.Guid in the Dynamo.Models.DynamoModel.ModelBasedRecordableCommand.ModelGuid collection, or an empty System.Guid.
- `ModelGuids` (System.Collections.Generic.IEnumerable<System.Guid>, get) — A collection of System.Guid.

## ModelEventCommand (class)

Type ModelEventCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void ModelEventCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid, string eventName)` — ModelEventCommand.ModelEventCommand
- `void ModelEventCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid, string eventName, int value)` — ModelEventCommand.ModelEventCommand
- `void ModelEventCommand(System.Guid modelGuid, string eventName)` — ModelEventCommand.ModelEventCommand
- `void ModelEventCommand(System.Guid modelGuid, string eventName, int value)` — ModelEventCommand.ModelEventCommand
- `void ModelEventCommand(string modelGuid, string eventName)` — ModelEventCommand.ModelEventCommand
- `void ModelEventCommand(string modelGuid, string eventName, int value)` — ModelEventCommand.ModelEventCommand

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

ModelEventCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ModelEventCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `EventName` (string, get) — ModelEventCommand.EventName
- `Value` (int, get) — ModelEventCommand.Value

## MutateTestCommand (class)

Type MutateTestCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void MutateTestCommand()` — MutateTestCommand.MutateTestCommand

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

MutateTestCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

MutateTestCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## Nature (enum)

Type Nature

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/DummyNode.cs)

### Values
- `Deprecated` = `0`
- `Unresolved` = `1`

## NodeFieldsEnum (enum)

Type NodeFieldsEnum

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/LuceneConfig.cs)

### Values
- `Author` = `10` — Package author name
- `CategorySplitted` = `3` — CategorySplitted - For this case we will be using just the last Category (the last word after the dot separator in FullCategoryName)
- `Description` = `4` — Description - The description of the node
- `DocName` = `6` — DocName - Name of the Document
- `Documentation` = `7` — Documentation - Documentation of the node
- `FullCategoryName` = `2` — FullCategoryName - The category of the node
- `Hosts` = `8` — Hosts - Package hosts
- `Name` = `0` — Name - The name of the node
- `NameSplitted` = `1` — NameSplitted - The name of the node splitted using just the last part (e.g. List.Chop we will be using just Chop)
- `Parameters` = `9` — Node Input Parameters as string (there are nodes with same name and category but different parameters)
- `SearchKeywords` = `5` — SearchKeywords - Several keywords that will be used when searching any word (this values are coming from xml files like BuiltIn.xml, DesignScriptBuiltin.xml or ProtoGeometry.xml)

## ObsoleteFileCallback (delegate)

Type ObsoleteFileCallback

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void ObsoleteFileCallback(object object, IntPtr method)` — ObsoleteFileCallback.ObsoleteFileCallback

### Methods
#### `System.IAsyncResult BeginInvoke(string fileName, System.Version fileVersion, System.Version currentVersion, System.AsyncCallback callback, object object)`

ObsoleteFileCallback.BeginInvoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void EndInvoke(System.IAsyncResult result)`

ObsoleteFileCallback.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void Invoke(string fileName, System.Version fileVersion, System.Version currentVersion)`

ObsoleteFileCallback.Invoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

## OpenFileCommand (class)

Type OpenFileCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void OpenFileCommand(string filePath, bool forceManualExecutionMode)` — Constructor
- `void OpenFileCommand(string filePath, bool forceManualExecutionMode, bool isTemplate)` — Constructor

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

OpenFileCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

OpenFileCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## OpenFileFromJsonCommand (class)

Type OpenFileFromJsonCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void OpenFileFromJsonCommand(string fileContents, bool forceManualExecutionMode)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

OpenFileFromJsonCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

OpenFileFromJsonCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## Operation (enum)

Type Operation

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEventArgs.cs)

### Values
- `FitView` = `0`
- `ZoomIn` = `1`
- `ZoomOut` = `2`

## Operation (enum)

Type Operation

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEventArgs.cs)

### Values
- `BeginDrag` = `0` — Begin dragging.
- `EndDrag` = `1` — End dragging.

## Operation (enum)

Type Operation

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEventArgs.cs)

### Values
- `Redo` = `1` — Redo.
- `Undo` = `0` — Undo.

## PausePlaybackCommand (class)

Type PausePlaybackCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void PausePlaybackCommand(int pauseDurationInMs)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

PausePlaybackCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

PausePlaybackCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## PointEventHandler (delegate)

Type PointEventHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void PointEventHandler(object object, IntPtr method)` — PointEventHandler.PointEventHandler

### Methods
#### `System.IAsyncResult BeginInvoke(object sender, System.EventArgs e, System.AsyncCallback callback, object object)`

PointEventHandler.BeginInvoke

**Parameters:**
- `sender` (object)
- `e` (System.EventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void EndInvoke(System.IAsyncResult result)`

PointEventHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Invoke(object sender, System.EventArgs e)`

PointEventHandler.Invoke

**Parameters:**
- `sender` (object)
- `e` (System.EventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

## PreferenceItem (enum)

Type PreferenceItem

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Configuration/PathManager.cs)

### Values
- `Backup` = `0`
- `Samples` = `2`
- `Templates` = `1`

## RawTraceData (class)

Type RawTraceData

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/CallSite.cs)

### Constructors
- `void RawTraceData(string callSiteID, string data)` — RawTraceData.RawTraceData

### Properties
- `Data` (string, get) — RawTraceData.Data
- `ID` (string, get) — RawTraceData.ID

## RecordableCommand (class)

Type RecordableCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void RecordableCommand()` — Constructs an instance of RecordableCommand derived class. This constructor is made protected to indicate that the class instance can only be instantiated through a derived class.
- `void RecordableCommand(string tag)` — Constructs an instance of RecordableCommand derived class, assigning a new tag to it.

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

Derived classes must implement this method to perform the actual command execution. A typical implementation of this method involves calling a corresponding method on DynamoModel by passing itself as the only argument.

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel) — The DynamoModel object on which this command should be executed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

Derived classes must implement this method to serialize all relevant information into the XmlElement supplied to it. Typically the method is a direct mirror of DeserializeCore.

**Parameters:**
- `element` (System.Xml.XmlElement) — All arguments that are required for this command are written into this XmlElement. The information written here must be exactly what DeserializeCore method expects.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## RunCancelCommand (class)

Type RunCancelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void RunCancelCommand(bool showErrors, bool cancelRun)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

RunCancelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

RunCancelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## RunCompletedHandler (delegate)

Type RunCompletedHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

### Constructors
- `void RunCompletedHandler(object object, IntPtr method)` — RunCompletedHandler.RunCompletedHandler

### Methods
#### `System.IAsyncResult BeginInvoke(object sender, bool success, System.AsyncCallback callback, object object)`

RunCompletedHandler.BeginInvoke

**Parameters:**
- `sender` (object)
- `success` (bool)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void EndInvoke(System.IAsyncResult result)`

RunCompletedHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void Invoke(object sender, bool success)`

RunCompletedHandler.Invoke

**Parameters:**
- `sender` (object)
- `success` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/DynamoModelEvents.cs)

## ScheduledTypes (enum)

Type ScheduledTypes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Values
- `None` = `0` — Invalid scheduled state. The initial state for every package, before it is interpreted by Dynamo
- `ScheduledForDeletion` = `2` — The package is scheduled to be deleted. After the next Dynamo restart, the package will deleted from the package locations
- `ScheduledForUnload` = `1` — The package is scheduled to be unloaded. After the next Dynamo restart, the package will be in an Unloaded state

## SearchElementHandler (delegate)

Type SearchElementHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/SearchElements/SearchElementBase.cs)

### Constructors
- `void SearchElementHandler(object object, IntPtr method)` — SearchElementHandler.SearchElementHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Search.SearchElements.SearchElementBase ele, System.AsyncCallback callback, object object)`

SearchElementHandler.BeginInvoke

**Parameters:**
- `ele` (Dynamo.Search.SearchElements.SearchElementBase)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/SearchElements/SearchElementBase.cs)

#### `void EndInvoke(System.IAsyncResult result)`

SearchElementHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/SearchElements/SearchElementBase.cs)

#### `void Invoke(Dynamo.Search.SearchElements.SearchElementBase ele)`

SearchElementHandler.Invoke

**Parameters:**
- `ele` (Dynamo.Search.SearchElements.SearchElementBase)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Search/SearchElements/SearchElementBase.cs)

## SelectInRegionCommand (class)

Type SelectInRegionCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void SelectInRegionCommand(Dynamo.Utilities.Rect2D region, bool isCrossSelection)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

SelectInRegionCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

SelectInRegionCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## SelectModelCommand (class)

Type SelectModelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void SelectModelCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuids, Dynamo.Utilities.ModifierKeys modifiers)` — 
- `void SelectModelCommand(System.Guid modelGuid, Dynamo.Utilities.ModifierKeys modifiers)` — 
- `void SelectModelCommand(string modelGuid, Dynamo.Utilities.ModifierKeys modifiers)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

SelectModelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

SelectModelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## SingleRunTraceData (class)

Type SingleRunTraceData

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/CallSite.cs)

### Methods
#### `bool Contains(string data)`

SingleRunTraceData.Contains

**Parameters:**
- `data` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `string GetLeftMostData()`

This gets the zero-most, left most index null if no data

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `System.Collections.Generic.List<string> RecursiveGetNestedData()`

SingleRunTraceData.RecursiveGetNestedData

**Returns:** `System.Collections.Generic.List<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/CallSite.cs)

### Properties
- `HasAnyNestedData` (bool, get) — Is there any data anywhere in this run data, or is it all empty structure
- `HasData` (bool, get) — SingleRunTraceData.HasData
- `HasNestedData` (bool, get) — SingleRunTraceData.HasNestedData
- `IsEmpty` (bool, get) — Does this struct contain any trace data

## StackFrameFlagOptions (enum)

Type StackFrameFlagOptions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Values
- `FepRun` = `1`
- `IsExternalFunction` = `3`
- `IsFunctionStepOver` = `4`
- `IsReplicating` = `2`

## State (enum)

Type State

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/SyncDataManager.cs)

### Values
- `CompletionHandled` = `5` — Post-execute action of task is completed and registered event handlers of task completion are notified.
- `Discarded` = `1` — Task is dropped from the scheduler due to compacting process
- `ExecutionCompleted` = `4` — Task execution is completed successfully
- `ExecutionFailed` = `3` — Task execution is completed with errors
- `ExecutionStarting` = `2` — Task is about to be executed
- `Scheduled` = `0` — Task is added to the scheduler

## State (enum)

Type State

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/SyncDataManager.cs)

### Values
- `Error` = `2`
- `Normal` = `0`
- `Warning` = `1`

## State (enum)

Type State

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDownloadHandle.cs)

### Values
- `Downloaded` = `2`
- `Downloading` = `1`
- `Error` = `5`
- `Installed` = `4`
- `Installing` = `3`
- `Uninitialized` = `0`

## State (enum)

Type State

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDownloadHandle.cs)

### Values
- `Compressing` = `2`
- `Copying` = `1`
- `Error` = `5`
- `Ready` = `0`
- `Uploaded` = `4`
- `Uploading` = `3`

## State (enum)

Type State

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DebuggerProperties.cs)

### Values
- `ExecutionBegin` = `0`
- `ExecutionBreak` = `2`
- `ExecutionEnd` = `1`
- `ExecutionResume` = `3`
- `Invalid` = `-1`

## StateTypes (enum)

Type StateTypes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Values
- `Error` = `3` — The package was not loaded in Dynamo because of an error. See the Error property for more information
- `Loaded` = `1` — The package is fully loaded and is ready to be used
- `None` = `0` — Invalid state. The initial state for every package, before it is interpreted by Dynamo
- `Unloaded` = `2` — The package is not loaded in Dynamo and not deleted from package locations

## StatementType (enum)

Type StatementType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

### Values
- `AssignmentVar` = `4`
- `Collection` = `3`
- `Expression` = `1`
- `FuncDeclaration` = `5`
- `Literal` = `2`
- `None` = `0`

## SwitchTabCommand (class)

Type SwitchTabCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void SwitchTabCommand(int modelIndex)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

SwitchTabCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

SwitchTabCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## SymbolConflictWarningHandler (delegate)

Type SymbolConflictWarningHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

### Constructors
- `void SymbolConflictWarningHandler(object object, IntPtr method)` — SymbolConflictWarningHandler.SymbolConflictWarningHandler

### Methods
#### `System.IAsyncResult BeginInvoke(string symbolName, string[] collidingSymbolNames, System.AsyncCallback callback, object object)`

SymbolConflictWarningHandler.BeginInvoke

**Parameters:**
- `symbolName` (string)
- `collidingSymbolNames` (string[])
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

#### `void EndInvoke(System.IAsyncResult result)`

SymbolConflictWarningHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

#### `void Invoke(string symbolName, string[] collidingSymbolNames)`

SymbolConflictWarningHandler.Invoke

**Parameters:**
- `symbolName` (string)
- `collidingSymbolNames` (string[])

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

## TaskMergeInstruction (enum)

Type TaskMergeInstruction

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

### Values
- `KeepBoth` = `0` — Both the AsyncTask objects in comparison should be kept.
- `KeepOther` = `2` — The current instance of AsyncTask should be discarded while keeping the other AsyncTask in comparison.
- `KeepThis` = `1` — The current instance of AsyncTask should be kept while discarding the other AsyncTask in comparison.

## TaskPriority (enum)

Type TaskPriority

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

### Values
- `AboveNormal` = `2`
- `BelowNormal` = `4`
- `Critical` = `0`
- `Highest` = `1`
- `Idle` = `6`
- `Lowest` = `5`
- `Normal` = `3`

## UnTrustedAssemblyException (class)

Type UnTrustedAssemblyException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/CertificateVerification.cs)

### Constructors
- `void UnTrustedAssemblyException(string assemblyPath)` — UnTrustedAssemblyException.UnTrustedAssemblyException

## UndoAction (enum)

Type UndoAction

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Notes/NoteModel.cs)

### Values
- `Pin` = `0`
- `Unpin` = `1`

## UndoRedoCommand (class)

Type UndoRedoCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void UndoRedoCommand(Dynamo.Models.DynamoModel.UndoRedoCommand.Operation operation)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

UndoRedoCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

UndoRedoCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## UngroupModelCommand (class)

Type UngroupModelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void UngroupModelCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid)` — 
- `void UngroupModelCommand(System.Guid modelGuid)` — 
- `void UngroupModelCommand(string modelGuid)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

UngroupModelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

UngroupModelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

## UpdateModelValueCommand (class)

Type UpdateModelValueCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void UpdateModelValueCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid, string name, string value)` — 
- `void UpdateModelValueCommand(System.Guid workspaceGuid, System.Collections.Generic.IEnumerable<System.Guid> modelGuid, string name, string value)` — 
- `void UpdateModelValueCommand(System.Guid workspaceGuid, System.Guid modelGuid, string name, string value)` — 
- `void UpdateModelValueCommand(System.Guid modelGuid, string name, string value)` — 
- `void UpdateModelValueCommand(string modelGuid, string name, string value)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

UpdateModelValueCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

UpdateModelValueCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

#### `string ToString()`

UpdateModelValueCommand.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `ModelGuids` (System.Collections.Generic.IEnumerable<System.Guid>, get) — A collection of System.Guid which identify the model objects to update.
- `Name` (string, get) — The name of the property to update.
- `Value` (string, get) — The new value to apply to the property.

## UploadType (enum)

Type UploadType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadHandle.cs)

### Values
- `Local` = `0`
- `Submit` = `1`

## Usage (class)

Type Usage

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/PythonEngineUpgradeService.cs)

### Properties
- `CustomNodeDefIdsWithPython` (System.Collections.Generic.IEnumerable<System.Guid>, get) — Usage.CustomNodeDefIdsWithPython
- `DirectPythonNodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>, get) — Usage.DirectPythonNodes
- `Workspace` (Dynamo.Graph.Workspaces.WorkspaceModel, get) — Usage.Workspace

## UserAction (enum)

Type UserAction

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/UndoRedoRecorder.cs)

### Values
- `Creation` = `0`
- `Deletion` = `2`
- `Modification` = `1`

## WinTrust (class)

Type WinTrust

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/CertificateVerification.cs)

### Methods
#### `bool VerifyEmbeddedSignature(string fileName)`

WinTrust.VerifyEmbeddedSignature

**Parameters:**
- `fileName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/CertificateVerification.cs)

## WorkspaceSavedEvent (delegate)

Type WorkspaceSavedEvent

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void WorkspaceSavedEvent(object object, IntPtr method)` — WorkspaceSavedEvent.WorkspaceSavedEvent

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Graph.Workspaces.WorkspaceModel model, System.AsyncCallback callback, object object)`

WorkspaceSavedEvent.BeginInvoke

**Parameters:**
- `model` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void EndInvoke(System.IAsyncResult result)`

WorkspaceSavedEvent.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Invoke(Dynamo.Graph.Workspaces.WorkspaceModel model)`

WorkspaceSavedEvent.Invoke

**Parameters:**
- `model` (Dynamo.Graph.Workspaces.WorkspaceModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

