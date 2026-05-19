---
name: dynamo-dynamo-graph-workspaces
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Graph.Workspaces namespace — 28 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CustomNodeWorkspaceModel, HomeWorkspaceModel, ICustomNodeWorkspaceModel, IWorkspaceModel, LayoutExtensions, PackageDependencyState, IPackageInfo, PackageInfo, and 20 more types.
---

# Dynamo.Graph.Workspaces

Auto-generated from vendor docs for dynamo 4.1.0. 28 types in this namespace.

## ConnectorConverter (class)

The ConnectorConverter is used to serialize and deserialize ConnectorModels. The Start and End of a ConnectorModel are references to PortModels, but we want the serialized representation of a Connector to reference these ports by Id. This converter resolves the reference to the correct NodeModel instance by id, and constructs the ConnectorModel.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void ConnectorConverter(Dynamo.Logging.ILogger logger)` — Constructs a ConnectorConverter.

### Methods
#### `bool CanConvert(System.Type objectType)`

ConnectorConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

ConnectorConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

ConnectorConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

## CustomNodeWorkspaceModel (class)

This class contains methods and properties that defines a customnodeworkspace.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

### Constructors
- `void CustomNodeWorkspaceModel(Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel> notes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Annotations.AnnotationModel> annotations, System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel> presets, ProtoCore.Namespace.ElementResolver elementResolver, Dynamo.Graph.Workspaces.WorkspaceInfo info)` — Initializes a new instance of the Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel class by given information about it and specified item collections
- `void CustomNodeWorkspaceModel(Dynamo.Graph.Workspaces.WorkspaceInfo info, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory)` — Initializes a new instance of the Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel class by given information about it and node factory

### Methods
#### `string GetSharedName()`

Gets appropriate name of workspace for sharing.

**Returns:** `string` — The name of workspace for sharing

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

#### `void NodeModified(Dynamo.Graph.Nodes.NodeModel node)`

CustomNodeWorkspaceModel.NodeModified

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

#### `void OnFunctionIdChanged(System.Guid oldId)`

CustomNodeWorkspaceModel.OnFunctionIdChanged

**Parameters:**
- `oldId` (System.Guid)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

#### `void OnInfoChanged()`

CustomNodeWorkspaceModel.OnInfoChanged

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

#### `void Save(string newPath, bool isBackup, Dynamo.Engine.EngineController engine)`

Saves custom node workspace

**Parameters:**
- `newPath` (string) — New location to save the workspace.
- `isBackup` (bool) — Indicates whether saved workspace is backup or not. If it's not backup, we should add it to recent files. Otherwise leave it.
- `engine` (Dynamo.Engine.EngineController) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

#### `void SetInfo(string newName, string newCategory, string newDescription, string newFilename)`

Updates custom node information by given data

**Parameters:**
- `newName` (string) — New name of the workspace. The name will not change if the parameter is omitted.
- `newCategory` (string) — New category of the workspace. The category will not change if the parameter is omitted.
- `newDescription` (string) — New description of the workspace. The description will not change if the parameter is omitted.
- `newFilename` (string) — New file name of the workspace. The file name will not change if the parameter is omitted.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

### Properties
- `Category` (string, get/set) — Search category for this workspace, if it is a Custom Node.
- `CustomNodeDefinition` (Dynamo.CustomNodeDefinition, get) — The definition of this custom node, based on the current state of this workspace.
- `CustomNodeDependencies` (System.Collections.Generic.IEnumerable<Dynamo.CustomNodeDefinition>, get) — All CustomNodeDefinitions which this Custom Node depends on.
- `CustomNodeId` (System.Guid, get) — Returns identifier of the custom node
- `CustomNodeInfo` (Dynamo.CustomNodeInfo, get) — The information about this custom node, based on the current state of this workspace.
- `Description` (string, get/set) — A description of the workspace
- `IsVisibleInDynamoLibrary` (bool, get/set) — Custom node visibility in the Dynamo library

### Events
#### `DefinitionUpdated` (`System.Action`)

**Signature:** `public event System.Action DefinitionUpdated`

Notifies all custom node instances that definition has changed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

#### `FunctionIdChanged` (`System.Action<System.Guid>`)

**Signature:** `public event System.Action<System.Guid> FunctionIdChanged`

Notifies listeners that custom node identifier has changed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

#### `InfoChanged` (`System.Action`)

**Signature:** `public event System.Action InfoChanged`

Notifies listeners that custom node workspace has changed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/CustomNodeWorkspaceModel.cs)

## DescriptionConverter (class)

Type DescriptionConverter

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void DescriptionConverter()` — DescriptionConverter.DescriptionConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

DescriptionConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

DescriptionConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

DescriptionConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

## DummyNodeWriteConverter (class)

DummyNodeWriteConverter is used for serializing DummyNodes to JSON. Note that the DummyNode objects serialize as their original content and not as a DummyNode.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void DummyNodeWriteConverter()` — DummyNodeWriteConverter.DummyNodeWriteConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

DummyNodeWriteConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

DummyNodeWriteConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

DummyNodeWriteConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Properties
- `CanRead` (bool, get) — DummyNodeWriteConverter.CanRead

## ExtraAnnotationViewInfo (class)

Non view-specific container for additional annotation view information required to fully construct a WorkspaceModel from JSON

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void ExtraAnnotationViewInfo()` — ExtraAnnotationViewInfo.ExtraAnnotationViewInfo

### Methods
#### `bool Equals(object obj)`

ExtraAnnotationViewInfo.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

## ExtraConnectorPinInfo (class)

Container for connector pin view information required to fully construct a WorkspaceViewModel from JSON

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void ExtraConnectorPinInfo()` — ExtraConnectorPinInfo.ExtraConnectorPinInfo

## ExtraNodeViewInfo (class)

Non view-specific container for additional node view information required to fully construct a WorkspaceModel from JSON

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void ExtraNodeViewInfo()` — ExtraNodeViewInfo.ExtraNodeViewInfo

## ExtraNoteViewInfo (class)

Non view-specific container for additional note view information required to fully construct a WorkspaceModel from JSON

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void ExtraNoteViewInfo()` — ExtraNoteViewInfo.ExtraNoteViewInfo

## ExtraWorkspaceViewInfo (class)

Non view-specific container for additional view information required to fully construct a WorkspaceModel from JSON

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void ExtraWorkspaceViewInfo()` — ExtraWorkspaceViewInfo.ExtraWorkspaceViewInfo

## HomeWorkspaceModel (class)

This class contains methods and properties that defines a Dynamo.Graph.Workspaces.WorkspaceModel.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

### Constructors
- `void HomeWorkspaceModel(Dynamo.Engine.EngineController engine, Dynamo.Scheduler.DynamoScheduler scheduler, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite.RawTraceData>>> traceData, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel> notes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Annotations.AnnotationModel> annotations, System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel> presets, ProtoCore.Namespace.ElementResolver resolver, Dynamo.Graph.Workspaces.WorkspaceInfo info, bool verboseLogging, bool isTestMode, Dynamo.Linting.LinterManager linterManager)` — Initializes a new instance of the Dynamo.Graph.Workspaces.HomeWorkspaceModel class by given information about it and specified item collections
- `void HomeWorkspaceModel(Dynamo.Engine.EngineController engine, Dynamo.Scheduler.DynamoScheduler scheduler, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, bool verboseLogging, bool isTestMode, Dynamo.Linting.LinterManager linterManager, string fileName)` — Initializes a new empty instance of the Dynamo.Graph.Workspaces.HomeWorkspaceModel class
- `void HomeWorkspaceModel(System.Guid guid, Dynamo.Engine.EngineController engine, Dynamo.Scheduler.DynamoScheduler scheduler, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite.RawTraceData>>> traceData, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel> notes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Annotations.AnnotationModel> annotations, System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel> presets, ProtoCore.Namespace.ElementResolver resolver, Dynamo.Graph.Workspaces.WorkspaceInfo info, bool verboseLogging, bool isTestMode, Dynamo.Linting.LinterManager linterManager)` — Initializes a new empty instance of the Dynamo.Graph.Workspaces.HomeWorkspaceModel class

### Methods
#### `void Clear()`

Clears this workspace of nodes, notes, and connectors.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void Dispose()`

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void DisposeNode(Dynamo.Graph.Nodes.NodeModel node)`

Called when a node is disposed and removed from the workspace

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel) — The node itself

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void NodeModified(Dynamo.Graph.Nodes.NodeModel node)`

Called when a Node is modified in the workspace

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel) — The node itself

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void OnEvaluationCompleted(Dynamo.Models.EvaluationCompletedEventArgs e)`

Triggers EvaluationCompleted event.

**Parameters:**
- `e` (Dynamo.Models.EvaluationCompletedEventArgs) — The event data containing information if graph evaluation has completed successfully.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void OnEvaluationStarted(System.EventArgs e)`

Triggers EvaluationStarted event.

**Parameters:**
- `e` (System.EventArgs) — The event data.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void OnNodeRemoved(Dynamo.Graph.Nodes.NodeModel node)`

HomeWorkspaceModel.OnNodeRemoved

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void OnRefreshCompleted(Dynamo.Models.EvaluationCompletedEventArgs e)`

Triggers RefreshCompleted event

**Parameters:**
- `e` (Dynamo.Models.EvaluationCompletedEventArgs) — The event data containing information if graph evaluation has completed successfully.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void OnToggleNodeFreeze(Dynamo.Graph.Nodes.NodeModel node)`

When a node is frozen, then the node and its dependencies should be deleted from the current AST

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel) — The node.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void RegisterNode(Dynamo.Graph.Nodes.NodeModel node)`

Called when a node is added to the workspace and event handlers are to be added

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel) — The node itself

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `void Run()`

This method is typically called from the main application thread (as a result of user actions such as button click or node UI changes) to schedule an update of the graph. This call may or may not represent an actual update. In the event that the user action does not result in actual graph update (e.g. moving of node on UI), the update task will not be scheduled for execution.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

### Properties
- `EngineController` (Dynamo.Engine.EngineController, get) — Returns Dynamo.Graph.Workspaces.HomeWorkspaceModel.EngineController object assosiated with thisPreloadedTraceData home workspace to coordinate the interactions between some DesignScript sub components like library managment, live runner and so on.
- `EvaluationCount` (long, get) — Evaluation count is incremented whenever the graph is evaluated. It is set to zero when the graph is Cleared.
- `GraphDocumentationURL` (System.Uri, get/set) — Link to documentation page for this workspace
- `HasRunWithoutCrash` (bool, get/set) — Indicates whether a run has completed successfully. This flag is critical to ensuring that crashing run-auto files are not left in run-auto upon reopening.
- `IsTestMode` (bool, get/set) — Flag specifying if this workspace is operating in "test mode".
- `RunSettings` (Dynamo.Models.RunSettings, get/set) — Returns Dynamo.Graph.Workspaces.HomeWorkspaceModel.EngineController object containing properties to control how execution is carried out.
- `Thumbnail` (string, get/set) — Workspace thumbnail as Base64 string. Returns null if provide value is not Base64 encoded.

### Events
#### `EvaluationCompleted` (`System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs>`)

**Signature:** `public event System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs> EvaluationCompleted`

Notifies listeners that graph evaluation is completed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `EvaluationStarted` (`System.EventHandler<System.EventArgs>`)

**Signature:** `public event System.EventHandler<System.EventArgs> EvaluationStarted`

Notifies listeners that graph evaluation is started. At this stage, the graph has not been analyzed yet so Dynamo could find that the Graph does not need to be executed at all, or only part of it could be executed (delta compute)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

#### `RefreshCompleted` (`System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs>`)

**Signature:** `public event System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs> RefreshCompleted`

Notifies listeners when all tasks in scheduler related to current evalution are completed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/HomeWorkspaceModel.cs)

## ICustomNodeWorkspaceModel (interface)

Type ICustomNodeWorkspaceModel

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/ICustomNodeWorkspaceModel.cs)

### Properties
- `CustomNodeDefinition` (Dynamo.CustomNodeDefinition, get) — ICustomNodeWorkspaceModel.CustomNodeDefinition
- `CustomNodeId` (System.Guid, get) — ICustomNodeWorkspaceModel.CustomNodeId
- `CustomNodeInfo` (Dynamo.CustomNodeInfo, get) — ICustomNodeWorkspaceModel.CustomNodeInfo

### Events
#### `DefinitionUpdated` (`System.Action`)

**Signature:** `public event System.Action DefinitionUpdated`

ICustomNodeWorkspaceModel.DefinitionUpdated

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/ICustomNodeWorkspaceModel.cs)

#### `FunctionIdChanged` (`System.Action<System.Guid>`)

**Signature:** `public event System.Action<System.Guid> FunctionIdChanged`

ICustomNodeWorkspaceModel.FunctionIdChanged

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/ICustomNodeWorkspaceModel.cs)

#### `InfoChanged` (`System.Action`)

**Signature:** `public event System.Action InfoChanged`

ICustomNodeWorkspaceModel.InfoChanged

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/ICustomNodeWorkspaceModel.cs)

## IPackageInfo (interface)

Interface for types containing info about a package

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PackageDependencyInfo.cs)

### Properties
- `Name` (string, get) — Name of the package
- `Version` (System.Version, get) — Version of the package

## IWorkspaceModel (interface)

Exposes workspace model.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/IWorkspaceModel.cs)

### Methods
#### `void RecordModelsForModification(System.Collections.Generic.IEnumerable<Dynamo.Graph.ModelBase> models)`

Implement to record node modification for undo/redo

**Parameters:**
- `models` (System.Collections.Generic.IEnumerable<Dynamo.Graph.ModelBase>) — Collection of Dynamo.Graph.ModelBase objects to record.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/IWorkspaceModel.cs)

### Properties
- `CenterX` (double, get) — Returns X coordinate of center point of visible workspace part
- `CenterY` (double, get) — Returns Y coordinate of center point of visible workspace part
- `Connectors` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Connectors.ConnectorModel>, get) — Returns list of connectors in the workspace
- `CurrentSelection` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>, get) — Returns list of selected nodes.
- `Description` (string, get) — Returns description of the workspace
- `FileName` (string, get) — Returns FileName of the workspace
- `Name` (string, get) — Returns name of the workspace
- `Nodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>, get) — Returns list of nodes owned by this workspace.

### Events
#### `ConnectorAdded` (`System.Action<Dynamo.Graph.Connectors.ConnectorModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Connectors.ConnectorModel> ConnectorAdded`

Triggers when a connector is added to the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/IWorkspaceModel.cs)

#### `ConnectorDeleted` (`System.Action<Dynamo.Graph.Connectors.ConnectorModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Connectors.ConnectorModel> ConnectorDeleted`

Triggers when a connector is removed from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/IWorkspaceModel.cs)

#### `NodeAdded` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeAdded`

Triggers when a node is added to the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/IWorkspaceModel.cs)

#### `NodeRemoved` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeRemoved`

Triggers when a node is removed from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/IWorkspaceModel.cs)

#### `NodesCleared` (`System.Action`)

**Signature:** `public event System.Action NodesCleared`

Triggers when nodes are cleared from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/IWorkspaceModel.cs)

## IdReferenceResolver (class)

The IdReferenceResolver class allows us to use the Guid of an object as the reference id during serialization.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void IdReferenceResolver()` — IdReferenceResolver.IdReferenceResolver

### Methods
#### `void AddReference(object context, string reference, object value)`

IdReferenceResolver.AddReference

**Parameters:**
- `context` (object)
- `reference` (string)
- `value` (object)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void AddToReferenceMap(System.Guid oldId, object newObject)`

Add a reference to a newly created object, referencing an old id.

**Parameters:**
- `oldId` (System.Guid) — The old id of the object.
- `newObject` (object) — The new object which maps to the old id.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `string GetReference(object context, object value)`

IdReferenceResolver.GetReference

**Parameters:**
- `context` (object)
- `value` (object)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `bool IsReferenced(object context, object value)`

IdReferenceResolver.IsReferenced

**Parameters:**
- `context` (object)
- `value` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ResolveReference(object context, string reference)`

IdReferenceResolver.ResolveReference

**Parameters:**
- `context` (object)
- `reference` (string)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ResolveReferenceFromMap(object context, string reference)`

Resolve a reference to a newly created object, given the original id for the object.

**Parameters:**
- `context` (object) — 
- `reference` (string) — 

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

## IdToGuidConverter (class)

This converter is used to attempt to convert an id string to a guid - if the id is not a guid string, it will create a UUID based on the string.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void IdToGuidConverter()` — IdToGuidConverter.IdToGuidConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

IdToGuidConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

IdToGuidConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

IdToGuidConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

## LayoutExtensions (static-class)

Layout class contains methods for organizing graphs.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/LayoutExtensions.cs)

## LinterManagerConverter (class)

Type LinterManagerConverter

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void LinterManagerConverter(Dynamo.Logging.ILogger logger)` — LinterManagerConverter.LinterManagerConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

LinterManagerConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

LinterManagerConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

LinterManagerConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

## NodeLibraryDependencyConverter (class)

Is used to serialize and deserialize graph workspace node library references

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void NodeLibraryDependencyConverter(Dynamo.Logging.ILogger logger)` — Constructs a WorkspaceNodeReferenceConverter.

### Methods
#### `bool CanConvert(System.Type objectType)`

NodeLibraryDependencyConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

NodeLibraryDependencyConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

NodeLibraryDependencyConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Properties
- `CanRead` (bool, get) — NodeLibraryDependencyConverter.CanRead

## NodeReadConverter (class)

The NodeModelConverter is used to serialize and deserialize NodeModels. These nodes require a CustomNodeDefinition which can only be supplied by looking it up in the CustomNodeManager.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void NodeReadConverter(Dynamo.Core.CustomNodeManager manager, Dynamo.Engine.LibraryServices libraryServices, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory nodeFactory, bool isTestMode)` — NodeReadConverter.NodeReadConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

NodeReadConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

NodeReadConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

NodeReadConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Properties
- `CanRead` (bool, get) — NodeReadConverter.CanRead
- `CanWrite` (bool, get) — NodeReadConverter.CanWrite
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get/set) — NodeReadConverter.ElementResolver

## PackageDependencyState (enum)

Enum containing the different types of package dependency states.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PackageDependencyInfo.cs)

### Values
- `IncorrectVersion` = `1`
- `Loaded` = `0`
- `Missing` = `2`
- `RequiresRestart` = `4`
- `Warning` = `3`

## PackageInfo (class)

Class containing info about a package

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PackageDependencyInfo.cs)

### Methods
#### `bool Equals(object obj)`

Checks whether two PackageInfos are equal They are equal if their Name and Versions are equal

**Parameters:**
- `obj` (object) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PackageDependencyInfo.cs)

#### `int GetHashCode()`

Gets the hashcode for this PackageInfo

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PackageDependencyInfo.cs)

#### `string ToString()`

Get the string representing this PackageInfo

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PackageDependencyInfo.cs)

### Properties
- `Name` (string, get) — Name of the package
- `Version` (System.Version, get) — Version of the package

## PresetExtensions (static-class)

Extension methods for adding and removing Presets to Workspaces.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PresetExtensions.cs)

### Methods
#### `void ImportPresets(Dynamo.Graph.Workspaces.WorkspaceModel workspace, System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel> presetCollection)`

Adds a specified collection Dynamo.Graph.Presets.PresetModel objects to the preset collection of the workspace.

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel) — 
- `presetCollection` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel>) — Dynamo.Graph.Presets.PresetModel objects to add.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/PresetExtensions.cs)

## SerializationExtensions (static-class)

Contains methods for serializing a WorkspaceModel to json.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationExtensions.cs)

## TypedParameterConverter (class)

This converter is used to attempt to convert TypedParameter into a json object

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void TypedParameterConverter()` — TypedParameterConverter.TypedParameterConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

TypedParameterConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

TypedParameterConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

TypedParameterConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

## WorkspaceInfo (class)

Contains sufficient data to create a Dynamo.Graph.Workspaces.WorkspaceModel object

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceInfo.cs)

### Constructors
- `void WorkspaceInfo()` — Initializes a new instance of the Dynamo.Graph.Workspaces.WorkspaceInfo class with default workspace data.
- `void WorkspaceInfo(string id, string name, string description, Dynamo.Models.RunType runType)` — Initializes a new instance of the Dynamo.Graph.Workspaces.WorkspaceInfo class with default workspace data.

### Properties
- `Category` (string, get) — Returns full category name of custom node
- `Description` (string, get) — Returns description of the workspace
- `FileName` (string, get) — Returns file name of the workspace
- `HasRunWithoutCrash` (bool, get) — Indicates if the workspace was executed successfully at last time
- `ID` (string, get) — Returns System.Guid identifier string value of custom node workspace
- `IsCustomNodeWorkspace` (bool, get) — Indicates whether the workspace is custom node or home workspace
- `IsVisibleInDynamoLibrary` (bool, get) — Indicates if the custom node is visible node library
- `Name` (string, get) — Returns name of the workspace
- `RunPeriod` (int, get) — Returns run period value of the home workspace if RunType is Periodic
- `RunType` (Dynamo.Models.RunType, get) — Returns run type of the home workspace
- `ScaleFactor` (double, get) — Returns the scale factor for ProtoGeometry geometries
- `Version` (string, get) — Returns version of Dynamo where the workspace was created
- `X` (double, get) — Returns X coordinate of top left corner of visible workspace part
- `Y` (double, get) — Returns Y coordinate of top left corner of visible workspace part
- `Zoom` (double, get) — Returns zoom value of the workspace

## WorkspaceModel (class)

Represents base class for all kind of workspaces which contains general data such as Name, collections of nodes, notes, annotations, etc.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void WorkspaceModel(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel> notes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Annotations.AnnotationModel> annotations, Dynamo.Graph.Workspaces.WorkspaceInfo info, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel> presets, ProtoCore.Namespace.ElementResolver resolver)` — WorkspaceModel.WorkspaceModel
- `void WorkspaceModel(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel> notes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Annotations.AnnotationModel> annotations, Dynamo.Graph.Workspaces.WorkspaceInfo info, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel> presets, ProtoCore.Namespace.ElementResolver resolver, Dynamo.Linting.LinterManager linterManager)` — WorkspaceModel.WorkspaceModel

### Methods
#### `void Clear()`

Clears this workspace of nodes, notes, and connectors.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void CreateModel(System.Xml.XmlElement modelData)`

Creates Dynamo.Graph.ModelBase object by given xml data and adds it to corresponding collection of the workspace.

**Parameters:**
- `modelData` (System.Xml.XmlElement) — Xml data to create model

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void DeleteModel(System.Xml.XmlElement modelData)`

Deletes Dynamo.Graph.ModelBase object given by System.Xml.XmlElement from a corresponding collection of the workspace.

**Parameters:**
- `modelData` (System.Xml.XmlElement) — Dynamo.Graph.ModelBase object given by System.Xml.XmlElement

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Dispose()`

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void DisposeNode(Dynamo.Graph.Nodes.NodeModel node)`

WorkspaceModel.DisposeNode

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `Dynamo.Graph.Workspaces.WorkspaceModel FromJson(string json, Dynamo.Engine.LibraryServices libraryServices, Dynamo.Engine.EngineController engineController, Dynamo.Scheduler.DynamoScheduler scheduler, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, bool isTestMode, bool verboseLogging, Dynamo.Core.CustomNodeManager manager)`

Load a WorkspaceModel from json. If the WorkspaceModel is a HomeWorkspaceModel it will be set as the current workspace.

**Parameters:**
- `json` (string) — 
- `libraryServices` (Dynamo.Engine.LibraryServices) — 
- `engineController` (Dynamo.Engine.EngineController) — 
- `scheduler` (Dynamo.Scheduler.DynamoScheduler) — 
- `factory` (Dynamo.Graph.Nodes.NodeLoaders.NodeFactory) — 
- `isTestMode` (bool) — 
- `verboseLogging` (bool) — 
- `manager` (Dynamo.Core.CustomNodeManager) — 

**Returns:** `Dynamo.Graph.Workspaces.WorkspaceModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `Dynamo.Graph.Workspaces.WorkspaceModel FromJson(string json, Dynamo.Engine.LibraryServices libraryServices, Dynamo.Engine.EngineController engineController, Dynamo.Scheduler.DynamoScheduler scheduler, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, bool isTestMode, bool verboseLogging, Dynamo.Core.CustomNodeManager manager, Dynamo.Linting.LinterManager linterManager)`

Load a WorkspaceModel from json. If the WorkspaceModel is a HomeWorkspaceModel it will be set as the current workspace.

**Parameters:**
- `json` (string) — 
- `libraryServices` (Dynamo.Engine.LibraryServices) — 
- `engineController` (Dynamo.Engine.EngineController) — 
- `scheduler` (Dynamo.Scheduler.DynamoScheduler) — 
- `factory` (Dynamo.Graph.Nodes.NodeLoaders.NodeFactory) — 
- `isTestMode` (bool) — 
- `verboseLogging` (bool) — 
- `manager` (Dynamo.Core.CustomNodeManager) — 
- `linterManager` (Dynamo.Linting.LinterManager)

**Returns:** `Dynamo.Graph.Workspaces.WorkspaceModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `Dynamo.Graph.ModelBase GetModelForElement(System.Xml.XmlElement modelData)`

Gets model by GUID which is contained in given Xml data.

**Parameters:**
- `modelData` (System.Xml.XmlElement) — Xml data to find model.

**Returns:** `Dynamo.Graph.ModelBase` — Found Dynamo.Graph.ModelBase object.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `Dynamo.Graph.ModelBase GetModelInternal(System.Guid modelGuid)`

Returns model by GUID

**Parameters:**
- `modelGuid` (System.Guid) — Identifier of the requested model.

**Returns:** `Dynamo.Graph.ModelBase` — Found Dynamo.Graph.ModelBase object.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `string GetSharedName()`

Returns appropriate name of workspace for sharing.

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Log(Dynamo.Logging.ILogMessage obj)`

WorkspaceModel.Log

**Parameters:**
- `obj` (Dynamo.Logging.ILogMessage)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Log(string msg)`

WorkspaceModel.Log

**Parameters:**
- `msg` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Log(string msg, Dynamo.Logging.WarningLevel severity)`

WorkspaceModel.Log

**Parameters:**
- `msg` (string)
- `severity` (Dynamo.Logging.WarningLevel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void NodeModified(Dynamo.Graph.Nodes.NodeModel node)`

Indicates that the AST for a node in this workspace requires recompilation

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnAnnotationAdded(Dynamo.Graph.Annotations.AnnotationModel annotation)`

WorkspaceModel.OnAnnotationAdded

**Parameters:**
- `annotation` (Dynamo.Graph.Annotations.AnnotationModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnAnnotationRemoved(Dynamo.Graph.Annotations.AnnotationModel annotation)`

WorkspaceModel.OnAnnotationRemoved

**Parameters:**
- `annotation` (Dynamo.Graph.Annotations.AnnotationModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnAnnotationsCleared()`

WorkspaceModel.OnAnnotationsCleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnConnectorAdded(Dynamo.Graph.Connectors.ConnectorModel obj)`

WorkspaceModel.OnConnectorAdded

**Parameters:**
- `obj` (Dynamo.Graph.Connectors.ConnectorModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnConnectorDeleted(Dynamo.Graph.Connectors.ConnectorModel obj)`

WorkspaceModel.OnConnectorDeleted

**Parameters:**
- `obj` (Dynamo.Graph.Connectors.ConnectorModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnDummyNodesReloaded()`

This method invokes the DummyNodesReloaded event on the workspace model.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnNodeAdded(Dynamo.Graph.Nodes.NodeModel node)`

WorkspaceModel.OnNodeAdded

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnNodeRemoved(Dynamo.Graph.Nodes.NodeModel node)`

WorkspaceModel.OnNodeRemoved

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnNodesCleared()`

WorkspaceModel.OnNodesCleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnNodesClearingConnectors()`

WorkspaceModel.OnNodesClearingConnectors

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnNoteAdded(Dynamo.Graph.Notes.NoteModel note)`

WorkspaceModel.OnNoteAdded

**Parameters:**
- `note` (Dynamo.Graph.Notes.NoteModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnNoteRemoved(Dynamo.Graph.Notes.NoteModel note)`

WorkspaceModel.OnNoteRemoved

**Parameters:**
- `note` (Dynamo.Graph.Notes.NoteModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnNotesCleared()`

WorkspaceModel.OnNotesCleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnPopulateJSONWorkspace(Newtonsoft.Json.Linq.JObject modelData)`

WorkspaceModel.OnPopulateJSONWorkspace

**Parameters:**
- `modelData` (Newtonsoft.Json.Linq.JObject)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnSaving(System.Xml.XmlDocument obj)`

WorkspaceModel.OnSaving

**Parameters:**
- `obj` (System.Xml.XmlDocument)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void OnToggleNodeFreeze(Dynamo.Graph.Nodes.NodeModel obj)`

WorkspaceModel.OnToggleNodeFreeze

**Parameters:**
- `obj` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void RecordModelsForModification(System.Collections.Generic.IEnumerable<Dynamo.Graph.ModelBase> models)`

Implement recording node modification for undo/redo.

**Parameters:**
- `models` (System.Collections.Generic.IEnumerable<Dynamo.Graph.ModelBase>) — Collection of Dynamo.Graph.ModelBase objects to record.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void RegisterNode(Dynamo.Graph.Nodes.NodeModel node)`

WorkspaceModel.RegisterNode

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void ReloadModel(System.Xml.XmlElement modelData)`

Updates Dynamo.Graph.ModelBase object with given xml data

**Parameters:**
- `modelData` (System.Xml.XmlElement) — Xml data to update model

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void RemoveGroup(Dynamo.Graph.ModelBase model)`

Deletes Dynamo.Graph.Annotations.AnnotationModel object from annotation collection of the workspace.

**Parameters:**
- `model` (Dynamo.Graph.ModelBase) — Dynamo.Graph.Annotations.AnnotationModel object to remove.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void ReportPosition()`

Called when workspace position is changed. This method notifyies all the listeners when a workspace is changed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void ResetWorkspaceCore()`

Derived workspace classes can choose to override this method to perform clean-up specific to them.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Save(string filePath, bool isBackup, Dynamo.Engine.EngineController engine)`

Workspace's Save method serializes the Workspace to JSON and writes it to the specified file path.

**Parameters:**
- `filePath` (string) — The path of the file.
- `isBackup` (bool) — A flag indicating whether this save operation represents a backup. If it's not backup, we should add it to recent files. Otherwise leave it.
- `engine` (Dynamo.Engine.EngineController) — An EngineController instance to be used to serialize node bindings.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void UpdateUndoRedoStack()`

Notifies the UI that the undo/redo state has changed so that undo/redo buttons can be enabled/disabled.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void UpdateWithExtraWorkspaceViewInfo(Dynamo.Graph.Workspaces.ExtraWorkspaceViewInfo workspaceViewInfo)`

Updates a workspace model with extra view information. When loading a workspace from JSON, the data is split into two parts, model and view. This method sets the view information.

**Parameters:**
- `workspaceViewInfo` (Dynamo.Graph.Workspaces.ExtraWorkspaceViewInfo) — The extra view information from the workspace to update the model with.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void UpdateWithExtraWorkspaceViewInfo(Dynamo.Graph.Workspaces.ExtraWorkspaceViewInfo workspaceViewInfo, double offsetX, double offsetY)`

Updates a workspace model with extra view information. When loading a workspace from JSON, the data is split into two parts, model and view. This method sets the view information. This overload allows to 'move' the incoming when placing them

**Parameters:**
- `workspaceViewInfo` (Dynamo.Graph.Workspaces.ExtraWorkspaceViewInfo) — 
- `offsetX` (double) — Offset in X direction (positive - left, negative - right)
- `offsetY` (double) — Offset in Y direction (positive - down, negative - up)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Properties
- `Annotations` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Annotations.AnnotationModel>, get) — Returns all of the annotations currently present in the workspace.
- `Author` (string, get/set) — An author of the workspace
- `CanRedo` (bool, get) — Determine if redo operation is currently possible.
- `CanUndo` (bool, get) — Determine if undo operation is currently possible.
- `CenterX` (double, get/set) — Implements Dynamo.Graph.ILocatable.CenterX property.
- `CenterY` (double, get/set) — Implements Dynamo.Graph.ILocatable.CenterY property.
- `Connectors` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Connectors.ConnectorModel>, get) — All of the connectors currently in the workspace.
- `CurrentSelection` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>, get) — WorkspaceModel.CurrentSelection
- `Dependencies` (System.Collections.Generic.HashSet<System.Guid>, get) — gathers the direct customNode workspace dependencies of this workspace.
- `Description` (string, get/set) — A description of the workspace
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get) — Returns Dynamo.Graph.Workspaces.WorkspaceModel.ElementResolver. This property resolves partial class name to fully resolved name.
- `FileName` (string, get/set) — Path to the file this workspace is associated with. If null or empty, this workspace has never been saved.
- `Guid` (System.Guid, get) — A unique identifier for the workspace.
- `HasUnsavedChanges` (bool, get/set) — Are there unsaved changes in the workspace?
- `Height` (double, get/set) — Returns the height of the workspace's bounds.
- `IsReadOnly` (bool, get/set) — Returns if current workspace is readonly.
- `LastSaved` (System.DateTime, get/set) — The date of the last save.
- `Name` (string, get/set) — The name of this workspace.
- `Nodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>, get) — All of the nodes currently in the workspace.
- `Notes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel>, get) — Returns the notes Dynamo.Graph.Notes.NoteModel collection.
- `Presets` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel>, get) — A set of input parameter states, this can be used to set the graph to a serialized state.
- `Rect` (Dynamo.Utilities.Rect2D, get) — Returns the bounds of the workspace.
- `ScaleFactor` (double, get) — The geometry scale factor specific to the workspace obtained from user input when selecting the scale of the model with which he/she is working. This is used by ProtoGeometry to scale geometric values appropriately before passing them to ASM. This property is set either when reading the setting from a DYN file or when the setting is updated from the UI.
- `ShowPythonAutoMigrationNotifications` (bool, get) — Flag indicating whether the “Python Engine Change” notice should be shown on save/close. Runtime-only (not serialized) and reset on workspace switch.
- `Width` (double, get/set) — Returns the width of the workspace's bounds.
- `X` (double, get/set) — Returns or set the X position of the workspace.
- `Y` (double, get/set) — Returns or set the Y position of the workspace
- `Zoom` (double, get/set) — Get or set the zoom value of the workspace.

### Events
#### `AnnotationAdded` (`System.Action<Dynamo.Graph.Annotations.AnnotationModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Annotations.AnnotationModel> AnnotationAdded`

Event that is fired when an annotation is added to the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `AnnotationRemoved` (`System.Action<Dynamo.Graph.Annotations.AnnotationModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Annotations.AnnotationModel> AnnotationRemoved`

Event that is fired when an annotation is removed from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `AnnotationsCleared` (`System.Action`)

**Signature:** `public event System.Action AnnotationsCleared`

Event that is fired when annotations are cleared from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `ConnectorAdded` (`System.Action<Dynamo.Graph.Connectors.ConnectorModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Connectors.ConnectorModel> ConnectorAdded`

Event that is fired when a connector is added to the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `ConnectorDeleted` (`System.Action<Dynamo.Graph.Connectors.ConnectorModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Connectors.ConnectorModel> ConnectorDeleted`

Event that is fired when a connector is deleted from a workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `CurrentOffsetChanged` (`Dynamo.Graph.Workspaces.WorkspaceModel.PointEventHandler`)

**Signature:** `public event Dynamo.Graph.Workspaces.WorkspaceModel.PointEventHandler CurrentOffsetChanged`

Event that is fired every time the position offset of a workspace changes.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `Disposed` (`System.Action`)

**Signature:** `public event System.Action Disposed`

Event that is fired when this workspace is disposed of.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `DummyNodesReloaded` (`System.Action`)

**Signature:** `public event System.Action DummyNodesReloaded`

This event is raised after the workspace tries to resolve existing dummyNodes - for example after a new package or library is loaded.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

Triggers when something needs to be logged

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `NodeAdded` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeAdded`

Event that is fired when a node is added to the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `NodeRemoved` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeRemoved`

Event that is fired when a node is removed from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `NodesCleared` (`System.Action`)

**Signature:** `public event System.Action NodesCleared`

Event that is fired when nodes are cleared from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `NoteAdded` (`System.Action<Dynamo.Graph.Notes.NoteModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Notes.NoteModel> NoteAdded`

Event that is fired when a note is added to the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `NoteRemoved` (`System.Action<Dynamo.Graph.Notes.NoteModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Notes.NoteModel> NoteRemoved`

Event that is fired when a note is removed from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `NotesCleared` (`System.Action`)

**Signature:** `public event System.Action NotesCleared`

Event that is fired when notes are cleared from the workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `RequestNodeCentered` (`Dynamo.Models.NodeEventHandler`)

**Signature:** `public event Dynamo.Models.NodeEventHandler RequestNodeCentered`

Event that is fired when a workspace requests that a Node or Note model is centered.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `Saved` (`System.Action`)

**Signature:** `public event System.Action Saved`

Event that is fired when the workspace is saved.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `Saving` (`System.Action<System.Xml.XmlDocument>`)

**Signature:** `public event System.Action<System.Xml.XmlDocument> Saving`

Event that is fired during the saving of the workspace. Add additional XmlNode objects to the XmlDocument provided, in order to save data to the file.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `WorkspaceSaving` (`System.Action<Dynamo.Graph.SaveContext>`)

**Signature:** `public event System.Action<Dynamo.Graph.SaveContext> WorkspaceSaving`

Event that is fired when the workspace is starting the save process.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

## WorkspaceReadConverter (class)

The WorkspaceConverter is used to serialize and deserialize WorkspaceModels. Construction of a WorkspaceModel requires things like an EngineController, a NodeFactory, and a Scheduler. These must be supplied at the time of construction and should not be serialized.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void WorkspaceReadConverter(Dynamo.Engine.EngineController engine, Dynamo.Scheduler.DynamoScheduler scheduler, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, bool isTestMode, bool verboseLogging)` — WorkspaceReadConverter.WorkspaceReadConverter
- `void WorkspaceReadConverter(Dynamo.Engine.EngineController engine, Dynamo.Scheduler.DynamoScheduler scheduler, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory, bool isTestMode, bool verboseLogging, Dynamo.Linting.LinterManager linterManager)` — WorkspaceReadConverter.WorkspaceReadConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

WorkspaceReadConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

WorkspaceReadConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

WorkspaceReadConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Properties
- `CanWrite` (bool, get) — WorkspaceReadConverter.CanWrite

## WorkspaceWriteConverter (class)

WorkspaceWriteConverter is used for serializing Workspaces to JSON.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Constructors
- `void WorkspaceWriteConverter(Dynamo.Engine.EngineController engine)` — WorkspaceWriteConverter.WorkspaceWriteConverter

### Methods
#### `bool CanConvert(System.Type objectType)`

WorkspaceWriteConverter.CanConvert

**Parameters:**
- `objectType` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, object existingValue, Newtonsoft.Json.JsonSerializer serializer)`

WorkspaceWriteConverter.ReadJson

**Parameters:**
- `reader` (Newtonsoft.Json.JsonReader)
- `objectType` (System.Type)
- `existingValue` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

#### `void WriteJson(Newtonsoft.Json.JsonWriter writer, object value, Newtonsoft.Json.JsonSerializer serializer)`

WorkspaceWriteConverter.WriteJson

**Parameters:**
- `writer` (Newtonsoft.Json.JsonWriter)
- `value` (object)
- `serializer` (Newtonsoft.Json.JsonSerializer)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Workspaces/SerializationConverters.cs)

### Properties
- `CanRead` (bool, get) — WorkspaceWriteConverter.CanRead

