---
name: dynamo-dynamo-models-dynamomodel
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Models.DynamoModel namespace — 38 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DynamoModelState, IStartConfiguration, CrashReporterStartupOptions, IStartConfigCrashReporter, IStartConfigurationLogger, DefaultStartConfiguration, RunCompletedHandler, CrashPromptHandler, and 30 more types.
---

# Dynamo.Models.DynamoModel

Auto-generated from vendor docs for dynamo 4.1.0. 38 types in this namespace.

## AddGroupToGroupCommand (class)

Type AddGroupToGroupCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void AddGroupToGroupCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid, System.Guid hostModelGuid)` — Creates a command to add a AnnotationModel object to another AnnotationModel.
- `void AddGroupToGroupCommand(System.Guid modelGuid, System.Guid hostModelGuid)` — Creates a command to add a AnnotationModel object to another AnnotationModel.
- `void AddGroupToGroupCommand(string modelGuid, string hostModelGuid)` — Creates a command to add a AnnotationModel object to another AnnotationModel.

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

AddGroupToGroupCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

AddGroupToGroupCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `HostGroupGuid` (System.Guid, get/set) — Id of the the group that should host the other group.

## AddModelToGroupCommand (class)

Type AddModelToGroupCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void AddModelToGroupCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid)` — 
- `void AddModelToGroupCommand(System.Guid modelGuid)` — 
- `void AddModelToGroupCommand(string modelGuid)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

AddModelToGroupCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

AddModelToGroupCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## AddPresetCommand (class)

Type AddPresetCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void AddPresetCommand(string name, string description, System.Collections.Generic.IEnumerable<System.Guid> currentSelectionIds)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

AddPresetCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

AddPresetCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## ApplyPresetCommand (class)

Type ApplyPresetCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void ApplyPresetCommand(System.Guid workspaceID, System.Guid stateID)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

ApplyPresetCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ApplyPresetCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## ConvertNodesToCodeCommand (class)

Type ConvertNodesToCodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

ConvertNodesToCodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ConvertNodesToCodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## CrashPromptHandler (delegate)

Type CrashPromptHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

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

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void EndInvoke(System.IAsyncResult result)`

CrashPromptHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void Invoke(object sender, Dynamo.Core.CrashPromptArgs e)`

CrashPromptHandler.Invoke

**Parameters:**
- `sender` (object)
- `e` (Dynamo.Core.CrashPromptArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

## CrashReporterStartupOptions (struct)

Type CrashReporterStartupOptions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `CERLocation` (string, get/set) — The Autodesk CrashReport tool location on disk (directory that contains the "senddmp.exe")

## CreateAndConnectNodeCommand (class)

Type CreateAndConnectNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateAndConnectNodeCommand(System.Guid newNodeGuid, System.Guid existingNodeGuid, string newNodeName, int outPortIndex, int inPortIndex, double x, double y, bool createAsDownstreamNode, bool addNewNodeToSelection)` — Creates a new CreateAndConnectNodeCommand with the given inputs

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateAndConnectNodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateAndConnectNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `NewNodeName` (string, get) — CreateAndConnectNodeCommand.NewNodeName

## CreateAnnotationCommand (class)

Type CreateAnnotationCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

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

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateAnnotationCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## CreateCustomNodeCommand (class)

Type CreateCustomNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateCustomNodeCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeId, string name, string category, string description, bool makeCurrent)` — 
- `void CreateCustomNodeCommand(System.Guid nodeId, string name, string category, string description, bool makeCurrent)` — 
- `void CreateCustomNodeCommand(string nodeId, string name, string category, string description, bool makeCurrent)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateCustomNodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateCustomNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## CreateNodeCommand (class)

Type CreateNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateNodeCommand(Dynamo.Graph.Nodes.NodeModel node, double x, double y, bool defaultPosition, bool transformCoordinates)` — 
- `void CreateNodeCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeId, string nodeName, double x, double y, bool defaultPosition, bool transformCoordinates)` — 
- `void CreateNodeCommand(string nodeId, string nodeName, double x, double y, bool defaultPosition, bool transformCoordinates)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateNodeCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `Name` (string, get) — CreateNodeCommand.Name

## CreateNoteCommand (class)

Type CreateNoteCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateNoteCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeIds, string noteText, double x, double y, bool defaultPosition)` — 
- `void CreateNoteCommand(System.Guid nodeId, string noteText, double x, double y, bool defaultPosition)` — 
- `void CreateNoteCommand(string nodeId, string noteText, double x, double y, bool defaultPosition)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

CreateNoteCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

CreateNoteCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## CreateProxyNodeCommand (class)

Type CreateProxyNodeCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void CreateProxyNodeCommand(string nodeId, string customnodeFunctionId, double x, double y, bool defaultPosition, bool transformCoordinates, string name, int inputs, int outputs)` — 

### Methods
#### `void SerializeCore(System.Xml.XmlElement element)`

CreateProxyNodeCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## DefaultStartConfiguration (struct)

Type DefaultStartConfiguration

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

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

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void DeleteModelCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuids)` — 
- `void DeleteModelCommand(System.Guid modelGuid)` — 
- `void DeleteModelCommand(string modelGuid)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

DeleteModelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

DeleteModelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## DragSelectionCommand (class)

Type DragSelectionCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void DragSelectionCommand(Dynamo.Utilities.Point2D mouseCursor, Dynamo.Models.DynamoModel.DragSelectionCommand.Operation operation)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

DragSelectionCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

DragSelectionCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## DynamoModelState (enum)

Type DynamoModelState

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Values
- `NotStarted` = `0`
- `StartedUI` = `2`
- `StartedUIless` = `1`

## ForceRunCancelCommand (class)

Type ForceRunCancelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void ForceRunCancelCommand(bool showErrors, bool cancelRun)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

ForceRunCancelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## IStartConfigCrashReporter (interface)

Type IStartConfigCrashReporter

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `CRStartConfig` (Dynamo.Models.DynamoModel.CrashReporterStartupOptions, get/set) — CERLocation

## IStartConfiguration (interface)

Type IStartConfiguration

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

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

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `Logger` (Dynamo.Logging.DynamoLogger, get/set) — Specify the logger instance.

## InsertFileCommand (class)

Type InsertFileCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void InsertFileCommand(string filePath, bool forceManualExecutionMode)` — Insert Graph or Custom Node from a file path into the current Workspace

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

InsertFileCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

InsertFileCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## MakeConnectionCommand (class)

Type MakeConnectionCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void MakeConnectionCommand(System.Collections.Generic.IEnumerable<System.Guid> nodeId, int portIndex, Dynamo.Graph.Nodes.PortType portType, Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode mode)` — Recordable command ConnectionCommand constructor
- `void MakeConnectionCommand(System.Guid nodeId, int portIndex, Dynamo.Graph.Nodes.PortType portType, Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode mode)` — Recordable command ConnectionCommand constructor
- `void MakeConnectionCommand(string nodeId, int portIndex, Dynamo.Graph.Nodes.PortType portType, Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode mode)` — Recordable command ConnectionCommand constructor

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

MakeConnectionCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

MakeConnectionCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `ConnectionMode` (Dynamo.Models.DynamoModel.MakeConnectionCommand.Mode, get) — MakeConnectionCommand.ConnectionMode
- `PortIndex` (int, get) — MakeConnectionCommand.PortIndex

## ModelBasedRecordableCommand (class)

Type ModelBasedRecordableCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void ModelBasedRecordableCommand(System.Collections.Generic.IEnumerable<System.Guid> guids)` — A collection of System.Guid identifying the objects on which to operate.

### Methods
#### `System.Collections.Generic.IEnumerable<System.Guid> DeserializeGuid(System.Xml.XmlElement element, Dynamo.Utilities.XmlElementHelper helper)`

ModelBasedRecordableCommand.DeserializeGuid

**Parameters:**
- `element` (System.Xml.XmlElement)
- `helper` (Dynamo.Utilities.XmlElementHelper)

**Returns:** `System.Collections.Generic.IEnumerable<System.Guid>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ModelBasedRecordableCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `ModelGuid` (System.Guid, get) — The first System.Guid in the Dynamo.Models.DynamoModel.ModelBasedRecordableCommand.ModelGuid collection, or an empty System.Guid.
- `ModelGuids` (System.Collections.Generic.IEnumerable<System.Guid>, get) — A collection of System.Guid.

## ModelEventCommand (class)

Type ModelEventCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

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

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

ModelEventCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `EventName` (string, get) — ModelEventCommand.EventName
- `Value` (int, get) — ModelEventCommand.Value

## MutateTestCommand (class)

Type MutateTestCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void MutateTestCommand()` — MutateTestCommand.MutateTestCommand

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

MutateTestCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

MutateTestCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## OpenFileCommand (class)

Type OpenFileCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void OpenFileCommand(string filePath, bool forceManualExecutionMode)` — Constructor
- `void OpenFileCommand(string filePath, bool forceManualExecutionMode, bool isTemplate)` — Constructor

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

OpenFileCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

OpenFileCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## OpenFileFromJsonCommand (class)

Type OpenFileFromJsonCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void OpenFileFromJsonCommand(string fileContents, bool forceManualExecutionMode)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

OpenFileFromJsonCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

OpenFileFromJsonCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## PausePlaybackCommand (class)

Type PausePlaybackCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void PausePlaybackCommand(int pauseDurationInMs)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

PausePlaybackCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

PausePlaybackCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## RecordableCommand (class)

Type RecordableCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void RecordableCommand()` — Constructs an instance of RecordableCommand derived class. This constructor is made protected to indicate that the class instance can only be instantiated through a derived class.
- `void RecordableCommand(string tag)` — Constructs an instance of RecordableCommand derived class, assigning a new tag to it.

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

Derived classes must implement this method to perform the actual command execution. A typical implementation of this method involves calling a corresponding method on DynamoModel by passing itself as the only argument.

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel) — The DynamoModel object on which this command should be executed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

Derived classes must implement this method to serialize all relevant information into the XmlElement supplied to it. Typically the method is a direct mirror of DeserializeCore.

**Parameters:**
- `element` (System.Xml.XmlElement) — All arguments that are required for this command are written into this XmlElement. The information written here must be exactly what DeserializeCore method expects.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## RunCancelCommand (class)

Type RunCancelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void RunCancelCommand(bool showErrors, bool cancelRun)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

RunCancelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

RunCancelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## RunCompletedHandler (delegate)

Type RunCompletedHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

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

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void EndInvoke(System.IAsyncResult result)`

RunCompletedHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

#### `void Invoke(object sender, bool success)`

RunCompletedHandler.Invoke

**Parameters:**
- `sender` (object)
- `success` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEvents.cs)

## SelectInRegionCommand (class)

Type SelectInRegionCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void SelectInRegionCommand(Dynamo.Utilities.Rect2D region, bool isCrossSelection)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

SelectInRegionCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

SelectInRegionCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## SelectModelCommand (class)

Type SelectModelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void SelectModelCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuids, Dynamo.Utilities.ModifierKeys modifiers)` — 
- `void SelectModelCommand(System.Guid modelGuid, Dynamo.Utilities.ModifierKeys modifiers)` — 
- `void SelectModelCommand(string modelGuid, Dynamo.Utilities.ModifierKeys modifiers)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

SelectModelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

SelectModelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## SwitchTabCommand (class)

Type SwitchTabCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void SwitchTabCommand(int modelIndex)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

SwitchTabCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

SwitchTabCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## UndoRedoCommand (class)

Type UndoRedoCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void UndoRedoCommand(Dynamo.Models.DynamoModel.UndoRedoCommand.Operation operation)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

UndoRedoCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

UndoRedoCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## UngroupModelCommand (class)

Type UngroupModelCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Constructors
- `void UngroupModelCommand(System.Collections.Generic.IEnumerable<System.Guid> modelGuid)` — 
- `void UngroupModelCommand(System.Guid modelGuid)` — 
- `void UngroupModelCommand(string modelGuid)` — 

### Methods
#### `void ExecuteCore(Dynamo.Models.DynamoModel dynamoModel)`

UngroupModelCommand.ExecuteCore

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

UngroupModelCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

## UpdateModelValueCommand (class)

Type UpdateModelValueCommand

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

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

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `void SerializeCore(System.Xml.XmlElement element)`

UpdateModelValueCommand.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

#### `string ToString()`

UpdateModelValueCommand.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs)

### Properties
- `ModelGuids` (System.Collections.Generic.IEnumerable<System.Guid>, get) — A collection of System.Guid which identify the model objects to update.
- `Name` (string, get) — The name of the property to update.
- `Value` (string, get) — The new value to apply to the property.

