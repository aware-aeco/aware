---
name: dynamo-dynamo-graph-nodes
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Graph.Nodes namespace — 43 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: NodeNameAttribute, NodeSearchTagsAttribute, NodeDescriptionAttribute, DoNotLoadOnPlatformsAttribute, NodeDeprecatedAttribute, AlsoKnownAsAttribute, IsMetaNodeAttribute, IsDesignScriptCompatibleAttribute, and 35 more types.
---

# Dynamo.Graph.Nodes

Auto-generated from vendor docs for dynamo 4.1.1. 43 types in this namespace.

## AlsoKnownAsAttribute (class)

The AlsoKnownAs attribute allows the node implementor to define an array of names that this node might have had in the past.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void AlsoKnownAsAttribute(string[] values)` — Constructs AlsoKnownAsAttribute with defined old names.

### Properties
- `Values` (string[], get/set) — Old names, that node used to have.

## BuiltinNodeCategories (static-class)

Built-in Dynamo Categories. If you want your node to appear in one of the existing Dynamo categories, then use these constants. This ensures that if the names of the categories change down the road, your node will still be placed there.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeCategories.cs)

## CodeBlockNodeModel (class)

Represents codeblock node's functionality.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

### Constructors
- `void CodeBlockNodeModel()` — This is the default constructor only used for search indexing purposes (if doesn't exist then the DynamoModel.AddNodeTypeToSearch() method is crashing due that is expecting a default constructor)
- `void CodeBlockNodeModel(Dynamo.Engine.LibraryServices libraryServices)` — Initilizes a new instance of the Dynamo.Graph.Nodes.CodeBlockNodeModel class
- `void CodeBlockNodeModel(string userCode, System.Guid guid, double xPos, double yPos, Dynamo.Engine.LibraryServices libraryServices, ProtoCore.Namespace.ElementResolver resolver)` — Initializes a new instance of the Dynamo.Graph.Nodes.CodeBlockNodeModel class. Call this constructor only when a first pass compilation of a CBN is needed without factoring in other CBN function definitions.
- `void CodeBlockNodeModel(string code, double x, double y, Dynamo.Engine.LibraryServices libraryServices, ProtoCore.Namespace.ElementResolver resolver)` — Initializes a new instance of the Dynamo.Graph.Nodes.CodeBlockNodeModel class

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

CodeBlockNodeModel.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierNode GetAstIdentifierForOutputIndex(int outputIndex)`

Fetches the ProtoAST Identifier for a given output index.

**Parameters:**
- `outputIndex` (int) — Index of the output port.

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierNode` — Identifier corresponding to the given output port.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierNode GetRawAstIdentifierForOutputIndex(int portIndex)`

Fetches the raw ProtoAST Identifier for a given index.

**Parameters:**
- `portIndex` (int) — Index of the port.

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierNode` — Identifier corresponding to the given port

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `ProtoCore.Type GetTypeHintForOutput(int index)`

Returns possible type of the output at specified output port.

**Parameters:**
- `index` (int) — Index of the port

**Returns:** `ProtoCore.Type` — The type

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `Dynamo.Migration.NodeMigrationData Migrate_2_0_0(Dynamo.Migration.NodeMigrationData data)`

CodeBlockNodeModel.Migrate_2_0_0

**Parameters:**
- `data` (Dynamo.Migration.NodeMigrationData)

**Returns:** `Dynamo.Migration.NodeMigrationData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

CodeBlockNodeModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `void SetCodeContent(string newCode, ProtoCore.Namespace.ElementResolver workspaceElementResolver)`

Sets string content of CodeBlock node.

**Parameters:**
- `newCode` (string) — New content of the code block
- `workspaceElementResolver` (ProtoCore.Namespace.ElementResolver) — Dynamo.Graph.Nodes.CodeBlockNodeModel.ElementResolver object

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `void SetNodeStateBasedOnConnectionAndDefaults()`

If a CBN is in Error state, it will have no code but will have output ports from the last successful compilation if any. In this case it should continue to be in Error state.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

CodeBlockNodeModel.UpdateValueCore

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

### Properties
- `AstIdentifierBase` (string, get) — Code block node displays the value of the left hand side variable of last statement.
- `Code` (string, get) — Returns string content of CodeBlock node.
- `CodeStatements` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.Statement>, get) — Code statement of CBN
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get/set) — Returns Dynamo.Graph.Nodes.CodeBlockNodeModel.ElementResolver for CodeBlock node
- `IsConvertible` (bool, get) — If this node is allowed to be converted to AST node in nodes to code conversion.
- `IsInputNode` (bool, get) — Indicates whether node is input node. Used to bind visibility of UI for user selection.
- `IsOutputNode` (bool, get) — Indicates whether node is an output node. Used to bind visibility of UI for user selection.
- `NodeType` (string, get) — The NodeType property provides a name which maps to the server type for the node. This property should only be used for serialization.
- `ShouldFocus` (bool, get) — Indicates whether code block should not be in focus upon undo/redo actions on node

## CodeBlockUtils (static-class)

Contains helper methods to adjust the contents of codeblock node.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockUtils.cs)

## DoNotLoadOnPlatformsAttribute (class)

The DoNotLoadOnPlatforms attribute allows the node implementer to define an array of contexts.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void DoNotLoadOnPlatformsAttribute(string[] values)` — Creates DoNotLoadOnPlatformsAttribute with restricted contexts.

### Properties
- `Values` (string[], get/set) — Restricted contexts, i.e. contexts in which node won't be loaded. E.g. Revit 2014

## DummyNode (class)

DummyNode is used for tests or in case if node couldn't be loaded.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/DummyNode.cs)

### Constructors
- `void DummyNode()` — This function creates DummyNode. DummyNode is used for tests or in case if node couldn't be loaded.
- `void DummyNode(int inputCount, int outputCount, string legacyName, System.Xml.XmlElement originalElement, string legacyAssembly, Dynamo.Graph.Nodes.DummyNode.Nature nodeNature)` — This function creates DummyNode with specified number of ports.
- `void DummyNode(string id, int inputCount, int outputCount, string legacyAssembly, Newtonsoft.Json.Linq.JObject originalElement)` — This function creates DummyNode with specified number of ports.
- `void DummyNode(string id, int inputCount, int outputCount, string legacyAssembly, string functionName, Newtonsoft.Json.Linq.JObject originalElement)` — This function creates DummyNode with specified number of ports.
- `void DummyNode(string id, int inputCount, int outputCount, string legacyAssembly, string functionName, string typeName, Newtonsoft.Json.Linq.JObject originalElement)` — This function creates DummyNode with specified number of ports.

### Methods
#### `System.Xml.XmlElement CreateElement(System.Xml.XmlDocument xmlDocument, Dynamo.Graph.SaveContext context)`

DummyNode.CreateElement

**Parameters:**
- `xmlDocument` (System.Xml.XmlDocument)
- `context` (Dynamo.Graph.SaveContext)

**Returns:** `System.Xml.XmlElement` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/DummyNode.cs)

#### `void DeserializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

DummyNode.DeserializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/DummyNode.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

DummyNode.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/DummyNode.cs)

### Properties
- `FunctionName` (string, get) — Returns the node's function name. This property is only valid for ZeroTouch dummy nodes
- `InputCount` (int, get) — Returns the number of input ports
- `LegacyAssembly` (string, get) — Returns the node assembly
- `LegacyFullName` (string, get) — Returns the original node DSFunction description or UI node type
- `LegacyNodeName` (string, get) — Returns the node name
- `NodeNature` (Dynamo.Graph.Nodes.DummyNode.Nature, get) — Node can be Deprecated or Unresolved
- `OriginalNodeContent` (object, get) — Xml node
- `OutputCount` (int, get) — Returns the number of output ports
- `TypeName` (string, get) — Type name of the node. This is property is only valid for NodeModel dummy nodes

## ElementState (enum)

Represents nodes states.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Values
- `Active` = `1`
- `AstBuildBroken` = `5`
- `Dead` = `0`
- `Error` = `4`
- `Info` = `6`
- `MigratedFormula` = `8`
- `PersistentInfo` = `7`
- `PersistentWarning` = `3`
- `Warning` = `2`

## FunctionCallBase<TController, TDescriptor> (class)

Node base class for all nodes that produce a DS function call.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallBase.cs)

### Constructors
- `void FunctionCallBase(TController controller)` — Default constructor

### Properties
- `Controller` (TController, get) — Controller used to sync node with a function definition.
- `CreationName` (string, get) — The unique name that the node was created by

## FunctionCallNodeController<T> (class)

Controller for nodes that act as function calls.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

### Constructors
- `void FunctionCallNodeController(T definition)` — Default constructor.

### Methods
#### `void AssignIdentifiersForFunctionCall(Dynamo.Graph.Nodes.NodeModel model, ProtoCore.AST.AssociativeAST.AssociativeNode rhs, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> resultAst)`

Produces AST that assigns all necessary Identifiers for the given NodeModel from the produced function call AST.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Model to produce AST for.
- `rhs` (ProtoCore.AST.AssociativeAST.AssociativeNode) — AST for the function call. This will need to be used to assign all output port identifiers.
- `resultAst` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Result accumulator: add all new output AST to this list.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> BuildAst(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes)`

Produces AST for a function call. Takes into account multi-outputs and partial application.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — NodeModel to produce an AST for.
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Arguments to the function call.

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void BuildAstForPartialMultiOutput(Dynamo.Graph.Nodes.NodeModel model, ProtoCore.AST.AssociativeAST.AssociativeNode rhs, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> resultAst)`

Produces AST for a partial function application of a multi-output function.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — NodeModel to produce AST for.
- `rhs` (ProtoCore.AST.AssociativeAST.AssociativeNode) — AST representing the partial application. This will need to be used to assign all output port identifiers.
- `resultAst` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Result accumulator: add all new output AST to this list.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void BuildOutputAst(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> resultAst)`

Produces AST for the given NodeModel that will call the underlying Function and assign all Identifiers for the node.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — NodeModel to produce AST for.
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Arguments to the function call.
- `resultAst` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Result accumulator: add all new output AST to this list.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void DeserializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Deserializes Controller information from XML.

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode GetFunctionApplication(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes)`

Produces AST representing a function application for the given NodeModel, using the given arguments. This should not assign any of the node's identifiers.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Node to produce a function application for.
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Arguments to the function application.

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void InitializeInputs(Dynamo.Graph.Nodes.NodeModel model)`

Initialize all input ports on the given node based on the underlying function.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Node to initialize.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void InitializeOutputs(Dynamo.Graph.Nodes.NodeModel model)`

Initialize all output ports on the given node based on the underlying function.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Node to initialize.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void OnSyncWithDefinitionStart(Dynamo.Graph.Nodes.NodeModel obj)`

Start syncing with its definition.

**Parameters:**
- `obj` (Dynamo.Graph.Nodes.NodeModel) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void OnSyncWithDefintionEnd(Dynamo.Graph.Nodes.NodeModel obj)`

Finish syncing with its definition.

**Parameters:**
- `obj` (Dynamo.Graph.Nodes.NodeModel) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Serializes Controller information from XML.

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `void SyncNodeWithDefinition(Dynamo.Graph.Nodes.NodeModel model)`

Synchronizes a node with this controller, based on the underlying function.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Node to sync.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

### Properties
- `Definition` (T, get/set) — A FunctionDescriptor describing the function that this controller will call.
- `Name` (string, get) — Name for nodes using this controller, based on the underlying FunctionDescriptor.
- `ReturnKeys` (System.Collections.Generic.IEnumerable<string>, get) — ReturnKeys for multi-output functions.

### Events
#### `SyncWithDefinitionEnd` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> SyncWithDefinitionEnd`

Event handler for the event when node finishes syncing with its definition.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

#### `SyncWithDefinitionStart` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> SyncWithDefinitionStart`

Event handler for the event when node starts syncing with its definition.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/FunctionCallNodeController.cs)

## InPortDescriptionsAttribute (class)

Indicates input ports' description

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void InPortDescriptionsAttribute(System.Type resourceType, string[] resourceNames)` — Loads ports descriptions from resource.
- `void InPortDescriptionsAttribute(string[] descriptions)` — Loads ports descriptions, that are specified directly in Attribute.

### Properties
- `PortDescriptions` (System.Collections.Generic.IEnumerable<string>, get) — Port descriptions, that are shown in Dynamo.

## InPortNamesAttribute (class)

Indicates input ports' names.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void InPortNamesAttribute(System.Type resourceType, string[] resourceNames)` — Loads ports names from resource.
- `void InPortNamesAttribute(string[] names)` — Loads ports names, that are specified directly in Attribute.

### Properties
- `PortNames` (System.Collections.Generic.IEnumerable<string>, get) — Port titles, that are shown in Dynamo.

## InPortTypesAttribute (class)

Indicates input ports' types

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void InPortTypesAttribute(System.Type resourceType, string[] resourceNames)` — Loads ports types from resource.
- `void InPortTypesAttribute(string[] types)` — Loads ports types, that are specified directly in Attribute.

### Properties
- `PortTypes` (System.Collections.Generic.IEnumerable<string>, get) — Port types, that are shown in Dynamo tooltips.

## Info (class)

Type Info

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Methods
#### `bool Equals(object other)`

Info.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `int GetHashCode()`

Info.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `string ToString()`

Info.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

## IsDesignScriptCompatibleAttribute (class)

The IsDesignScriptCompatibleAttribute indicates if the node is able to work with DesignScript evaluation engine.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void IsDesignScriptCompatibleAttribute()` — IsDesignScriptCompatibleAttribute.IsDesignScriptCompatibleAttribute

## IsMetaNodeAttribute (class)

The MetaNode attribute means this node shouldn't be added to the category, only its instances are allowed

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void IsMetaNodeAttribute()` — IsMetaNodeAttribute.IsMetaNodeAttribute

## LacingStrategy (enum)

Defines Lacing strategy for nodes. Learn more about lacing here: http://dynamoprimer.com/06_Designing-with-Lists/6-1_whats-a-list.html

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Values
- `Auto` = `5`
- `CrossProduct` = `4`
- `Disabled` = `0`
- `First` = `1`
- `Longest` = `3`
- `Shortest` = `2`

## NodeDeprecatedAttribute (class)

Flag to hide deprecated nodes in search, but allow in workflows

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void NodeDeprecatedAttribute()` — NodeDeprecatedAttribute.NodeDeprecatedAttribute

## NodeDescriptionAttribute (class)

The NodeDescriptionAttribute attribute allows the node implementer to define node description.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void NodeDescriptionAttribute(string description)` — Creates NodeDescriptionAttribute with string.
- `void NodeDescriptionAttribute(string descriptionResourceID, System.Type resourceType)` — Creates NodeDescriptionAttribute with description from resx file.

### Properties
- `ElementDescription` (string, get/set) — Description of the node. E.g. "Get a color given a color range." for ColorRange node.

## NodeInputData (class)

Represents a node which acts as a UI input for the graph - may also hold a value for that input

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeInputData.cs)

### Constructors
- `void NodeInputData()` — NodeInputData.NodeInputData

### Methods
#### `bool Equals(object obj)`

NodeInputData.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeInputData.cs)

### Properties
- `Choices` (System.Collections.Generic.List<string>, get/set) — If this input is a dropdownSelection type a list of choices a user can select.
- `Description` (string, get/set) — Description displayed to user of this input node.
- `Id` (System.Guid, get/set) — The id of the node.
- `MaximumValue` (System.Nullable<double>, get/set) — if this input is a Number, the max value of that number.
- `MinimumValue` (System.Nullable<double>, get/set) — if this input is a Number, the min value of that number.
- `Name` (string, get/set) — Display name of the input node.
- `NumberType` (string, get/set) — if this input is a Number, the number type Double or Integer that the number returns.
- `SelectedIndex` (System.Nullable<int>, get/set) — The index of the selected item.
- `StepValue` (System.Nullable<double>, get/set) — if this input is a Number, the step value of that number when it acts as a slider.
- `Type` (Dynamo.Graph.Nodes.NodeInputTypes, get/set) — The type of input this node is.
- `Type2` (Dynamo.Graph.Nodes.NodeInputTypes, get/set) — The type of input this node is.
- `Value` (string, get/set) — The value of the input when the graph was saved. This should always be a string for all types.

## NodeInputTypes (enum)

Possible graph input types.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeInputData.cs)

### Values
- `booleanInput` = `1`
- `colorInput` = `3`
- `dateInput` = `4`
- `dropdownSelection` = `7` — Should not be used when setting NodeInputData.Type, use selection instead, can be used with Type2.
- `hostSelection` = `6` — Should not be used when setting NodeInputData.Type, use selection instead, can be used with Type2.
- `numberInput` = `0`
- `selectionInput` = `5`
- `stringInput` = `2`

## NodeModel (class)

Type NodeModel

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Constructors
- `void NodeModel()` — Protected constructor used during deserialization.
- `void NodeModel(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> inPorts, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> outPorts)` — Protected constructor used during deserialization.

### Methods
#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> BuildOutputAst(System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes)`

Override this to declare the outputs for each of this Node's output ports.

**Parameters:**
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Ast for inputs indexed by input port index.

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>` — Sequence of AssociativeNodes representing this Node's code output.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void ClearDirtyFlag()`

NodeModel.ClearDirtyFlag

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void ClearErrorsAndWarnings()`

Clears the errors/warnings that are generated when running the graph.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void ClearInfoMessages()`

Clears the info messages that are generated when running the graph. If the current State is Info or PersistentInfo, it will be set to Active. If the current State is Warning, Error, or another non-info state, it is preserved.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void ConfigureSnapEdges(System.Collections.Generic.IList<Dynamo.Graph.Nodes.PortModel> ports)`

Configures the snap edges. This class was made protected during refactoring for serialization. When RegisterInputPorts and RegisterOutputPorts are finally removed, this method should be called in a collection changed event handler on InPorts and OutPorts.

**Parameters:**
- `ports` (System.Collections.Generic.IList<Dynamo.Graph.Nodes.PortModel>) — The ports.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void Deselect()`

NodeModel.Deselect

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

NodeModel.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void DispatchOnUIThread(System.Action a)`

Called by nodes for behavior that they want to dispatch on the UI thread Triggers event to be received by the UI. If no UI exists, behavior will not be executed.

**Parameters:**
- `a` (System.Action) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void Dispose()`

NodeModel.Dispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void Error(string p)`

NodeModel.Error

**Parameters:**
- `p` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierNode GetAstIdentifierForOutputIndex(int outputIndex)`

Fetches the ProtoAST Identifier for a given output port.

**Parameters:**
- `outputIndex` (int) — Index of the output port.

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierNode` — Identifier corresponding to the given output port.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `string GetDescriptionStringFromAttributes()`

Returns the description from type information

**Returns:** `string` — The value or "No description provided"

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `Dynamo.Graph.Nodes.NodeModel.ExecutionHints GetExecutionHintsCore()`

NodeModel.GetExecutionHintsCore

**Returns:** `Dynamo.Graph.Nodes.NodeModel.ExecutionHints` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `ProtoCore.Type GetTypeHintForOutput(int index)`

The possible type of output at specified port. This type information is not necessary to be accurate.

**Parameters:**
- `index` (int)

**Returns:** `ProtoCore.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `ProtoCore.Mirror.MirrorData GetValue(int outPortIndex, Dynamo.Engine.EngineController engine)`

Returns the most recent value of this node stored in an EngineController that has evaluated it.

**Parameters:**
- `outPortIndex` (int) — 
- `engine` (Dynamo.Engine.EngineController) — 

**Returns:** `ProtoCore.Mirror.MirrorData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void Info(string p, bool isPersistent)`

Set an info on a node.

**Parameters:**
- `p` (string) — The info text.
- `isPersistent` (bool) — Is the info persistent? If true, the info will not be cleared when the node is next evaluated. If false, the info will be cleared on the next evaluation.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void MarkNodeAsModified(bool forceExecute)`

NodeModel.MarkNodeAsModified

**Parameters:**
- `forceExecute` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `Dynamo.Migration.NodeMigrationData MigrateShortestLacingToAutoLacing(Dynamo.Migration.NodeMigrationData data)`

NodeModel.MigrateShortestLacingToAutoLacing

**Parameters:**
- `data` (Dynamo.Migration.NodeMigrationData)

**Returns:** `Dynamo.Migration.NodeMigrationData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `Dynamo.Migration.NodeMigrationData MigrateToDsFunction(Dynamo.Migration.NodeMigrationData data, string name, string funcName)`

NodeModel.MigrateToDsFunction

**Parameters:**
- `data` (Dynamo.Migration.NodeMigrationData)
- `name` (string)
- `funcName` (string)

**Returns:** `Dynamo.Migration.NodeMigrationData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `Dynamo.Migration.NodeMigrationData MigrateToDsFunction(Dynamo.Migration.NodeMigrationData data, string assembly, string name, string funcName)`

NodeModel.MigrateToDsFunction

**Parameters:**
- `data` (Dynamo.Migration.NodeMigrationData)
- `assembly` (string)
- `name` (string)
- `funcName` (string)

**Returns:** `Dynamo.Migration.NodeMigrationData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `Dynamo.Migration.NodeMigrationData MigrateToDsVarArgFunction(Dynamo.Migration.NodeMigrationData data, string assembly, string name, string funcName)`

NodeModel.MigrateToDsVarArgFunction

**Parameters:**
- `data` (Dynamo.Migration.NodeMigrationData)
- `assembly` (string)
- `name` (string)
- `funcName` (string)

**Returns:** `Dynamo.Migration.NodeMigrationData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void NotifyAstBuildBroken(string p)`

Change the state of node to ElementState.AstBuildBroken and display "p" in tooltip window.

**Parameters:**
- `p` (string) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void OnBuilt()`

Callback for when this NodeModel has been compiled.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void OnConnectorAdded(Dynamo.Graph.Connectors.ConnectorModel obj)`

NodeModel.OnConnectorAdded

**Parameters:**
- `obj` (Dynamo.Graph.Connectors.ConnectorModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void OnNodeModified(bool forceExecute)`

NodeModel.OnNodeModified

**Parameters:**
- `forceExecute` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void OnSave()`

Called when the node's Workspace has been saved.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void OnUpdateASTCollection()`

NodeModel.OnUpdateASTCollection

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void RegisterAllPorts()`

Updates UI so that all ports reflect current state of node ports.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void RegisterInputPorts(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortData> portDatas)`

Reads inputs list and adds ports for each input. TODO: DYN-6445 - evaluate if this API can be removed.

**Parameters:**
- `portDatas` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortData>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void RegisterOutputPorts(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortData> portDatas)`

Reads outputs list and adds ports for each output TODO: DYN-6445 - evaluate if this API can be removed.

**Parameters:**
- `portDatas` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortData>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `bool RequestVisualUpdateAsync(Dynamo.Scheduler.IScheduler scheduler, Dynamo.Engine.EngineController engine, Dynamo.Visualization.IRenderPackageFactory factory, bool forceUpdate)`

Call this method to asynchronously regenerate render package for this node. This method accesses core properties of a NodeModel and therefore is typically called on the main/UI thread.

**Parameters:**
- `scheduler` (Dynamo.Scheduler.IScheduler) — An IScheduler on which the task will be scheduled.
- `engine` (Dynamo.Engine.EngineController) — The EngineController which will be used to get MirrorData for the node.
- `factory` (Dynamo.Visualization.IRenderPackageFactory) — An IRenderPackageFactory which will be used to generate IRenderPackage objects.
- `forceUpdate` (bool) — Normally, render packages are only generated when the node's IsUpdated parameter is true. By setting forceUpdate to true, the render packages will be updated.

**Returns:** `bool` — Flag which indicates if geometry update has been scheduled

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `bool RequestVisualUpdateAsync(Dynamo.Scheduler.IScheduler scheduler, Dynamo.Engine.EngineController engine, Dynamo.Visualization.IRenderPackageFactory factory, bool forceUpdate, bool ignoreIsVisible)`

Call this method to asynchronously regenerate render package for this node. This method accesses core properties of a NodeModel and therefore is typically called on the main/UI thread.

**Parameters:**
- `scheduler` (Dynamo.Scheduler.IScheduler) — An IScheduler on which the task will be scheduled.
- `engine` (Dynamo.Engine.EngineController) — The EngineController which will be used to get MirrorData for the node.
- `factory` (Dynamo.Visualization.IRenderPackageFactory) — An IRenderPackageFactory which will be used to generate IRenderPackage objects.
- `forceUpdate` (bool) — Normally, render packages are only generated when the node's IsUpdated parameter is true. By setting forceUpdate to true, the render packages will be updated.
- `ignoreIsVisible` (bool) — Normally, render packages are only generated when the node's IsVisible parameter is true. By setting ignore to true, the render package will be updated.

**Returns:** `bool` — Flag which indicates if geometry update has been scheduled

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void SelectDownstreamNeighbours()`

Recursively selects all nodes downstream to this node

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void SelectNeighbors()`

Selects all neighboring nodes ans connector pins to this node

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void SelectUpstreamAndDownstreamNeighbours()`

Recursively selects all nodes upstream and downstream to this node

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void SelectUpstreamNeighbours()`

Recursively selects all nodes upstream to this node

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

NodeModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void SetNameFromNodeNameAttribute()`

Sets the name of this node from the attributes on the class definining it.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void SetNodeStateBasedOnConnectionAndDefaults()`

Sets the Dynamo.Graph.Nodes.ElementState for the node based on the port's default value status and connectivity.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `bool ShouldSerializeX()`

The default behavior for ModelBase objects is to not serialize the X and Y properties. This overload allows the serialization of the X property for NodeModel.

**Returns:** `bool` — True.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `bool ShouldSerializeY()`

The default behavior for ModelBase objects is to not serialize the X and Y properties. This overload allows the serialization of the Y property for NodeModel.

**Returns:** `bool` — True

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `bool TryGetInput(int data, ref System.Tuple<int, Dynamo.Graph.Nodes.NodeModel> input)`

Attempts to get the input for a certain port.

**Parameters:**
- `data` (int) — PortData to look for an input for.
- `input` (ref System.Tuple<int, Dynamo.Graph.Nodes.NodeModel>) — If an input is found, it will be assigned.

**Returns:** `bool` — True if there is an input, false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `bool TryGetOutput(int output, ref System.Collections.Generic.HashSet<System.Tuple<int, Dynamo.Graph.Nodes.NodeModel>> newOutputs)`

Attempts to get the output for a certain port.

**Parameters:**
- `output` (int) — Index to look for an output for.
- `newOutputs` (ref System.Collections.Generic.HashSet<System.Tuple<int, Dynamo.Graph.Nodes.NodeModel>>) — If an output is found, it will be assigned.

**Returns:** `bool` — True if there is an output, false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

NodeModel.UpdateValueCore

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void UseLevelAndReplicationGuide(System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputs)`

Apppend replication guide to the input parameter based on lacing strategy.

**Parameters:**
- `inputs` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `void Warning(string p, bool isPersistent)`

Set a warning on a node.

**Parameters:**
- `p` (string) — The warning text.
- `isPersistent` (bool) — Is the warning persistent? If true, the warning will not be cleared when the node is next evaluated and any additional warning messages will be concatenated to the persistent error message. If false, the warning will be cleared on the next evaluation.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Properties
- `AllConnectors` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Connectors.ConnectorModel>, get) — All of the connectors entering and exiting the NodeModel.
- `AreAllOutputsConnected` (bool, get) — Are all the outputs of this node connected?
- `ArgumentLacing` (Dynamo.Graph.Nodes.LacingStrategy, get) — Control how arguments lists of various sizes are laced.
- `AstIdentifierBase` (string, get) — Return a variable whose value will be displayed in preview window. Derived nodes may overwrite this function to display default value of this node. E.g., code block node may want to display the value of the left hand side variable of last statement.
- `AstIdentifierForPreview` (ProtoCore.AST.AssociativeAST.IdentifierNode, get) — ProtoAST Identifier for result of the node before any output unpacking has taken place. If there is only one output for the node, this is equivalent to GetAstIdentifierForOutputIndex(0).
- `AstIdentifierGuid` (string, get) — A unique ID that will be appended to all identifiers of this node.
- `CachedValue` (ProtoCore.Mirror.MirrorData, get) — The value of this node after the most recent computation As this property could be modified by the virtual machine, it's dangerous to access this value without using the active Scheduler. Use the Scheduler to remove the possibility of race conditions.
- `CanUpdatePeriodically` (bool, get/set) — NodeModel.CanUpdatePeriodically
- `Category` (string, get/set) — Category property
- `CreationName` (string, get) — The unique name that was created the node by
- `Description` (string, get/set) — Description of this Node.
- `DictionaryLink` (string, get/set) — Dictionary Link property
- `DismissedAlerts` (System.Collections.ObjectModel.ObservableCollection<string>, get/set) — NodeModel.DismissedAlerts
- `DismissedAlertsCount` (int, get) — Returns the number of dismissed error/warning/info messages.
- `DisplayLabels` (bool, get/set) — Enable or disable label display. Default is false.
- `GUID` (System.Guid, get/set) — Id for this node, must be unique within the graph.
- `InPorts` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Graph.Nodes.PortModel>, get) — Collection of PortModels representing all Input ports.
- `InputData` (Dynamo.Graph.Nodes.NodeInputData, get) — NodeModel.InputData
- `InputNodes` (System.Collections.Generic.IDictionary<int, System.Tuple<int, Dynamo.Graph.Nodes.NodeModel>>, get) — NodeModel.InputNodes
- `IsConvertible` (bool, get) — If this node is allowed to be converted to AST node in nodes to code conversion.
- `IsCustomFunction` (bool, get) — Returns whether this node represents a built-in or custom function.
- `IsFrozen` (bool, get/set) — A flag indicating whether the node is frozen. When a node is frozen, the node, and all nodes downstream will not participate in execution. This will return true if any upstream node is frozen or if the node was explicitly frozen.
- `IsInErrorState` (bool, get) — If the state of node is Error or AstBuildBroken
- `IsInputNode` (bool, get) — Input nodes are used in Customizer and Presets. Input nodes can be numbers, number sliders, strings, bool, code blocks, custom nodes and color palette node. This property is true for nodes that are potential inputs for Customizers and Presets.
- `IsModified` (bool, get) — NodeModel.IsModified
- `IsOutputNode` (bool, get) — Output nodes are used by applications that consume graphs outside of Dynamo such as Optioneering, Optimization, and Dynamo Player. Output nodes can be any node that returns a single output or a dictionary. Code block nodes and Custom nodes are specifically excluded at this time even though they can return a single output for sake of clarity.
- `IsPartiallyApplied` (bool, get) — Is this node being applied partially, resulting in a partial function?
- `IsResizable` (bool, get) — Indicates whether this node supports user-resizable UI elements
- `IsSetAsInput` (bool, get/set) — This property is user-controllable via a checkbox and is set to true when a user wishes to include this node in a Customizer as an interactive control.
- `IsSetAsOutput` (bool, get/set) — This property is user-controllable via a checkbox and is set to true when a user wishes to include this node in the OutputData block of the Dyn file.
- `IsTopMostNode` (bool, get) — If node is connected to some other node(other than Output) then it is not a 'top' node
- `IsVisible` (bool, get) — Returns whether the node is to be included in visualizations.
- `Name` (string, get/set) — The name that is displayed in the UI for this NodeModel.
- `NeedsForceExecution` (bool, get) — NodeModel.NeedsForceExecution
- `NodeInfos` (System.Collections.Generic.List<Dynamo.Graph.Nodes.Info>, get) — A publicly accessible collector of all Info/Warning/Error data
- `NodeType` (string, get) — The NodeType property provides a name which maps to the server type for the node. This property should only be used for serialization.
- `OutPorts` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Graph.Nodes.PortModel>, get) — Collection of PortModels representing all Output ports.
- `OutputData` (Dynamo.Graph.Nodes.NodeOutputData, get) — NodeModel.OutputData
- `OutputNodes` (System.Collections.Generic.IDictionary<int, System.Collections.Generic.HashSet<System.Tuple<int, Dynamo.Graph.Nodes.NodeModel>>>, get) — NodeModel.OutputNodes
- `PreviewPinned` (bool, get) — Indicates if node preview is pinned
- `RaisesModificationEvents` (bool, get/set) — Indicate if the node should respond to NodeModified event. It always should be true, unless is temporarily set to false to avoid flood of Modified event.
- `ShouldDisplayPreview` (bool, get) — NodeModel.ShouldDisplayPreview
- `State` (Dynamo.Graph.Nodes.ElementState, get/set) — The Node's state, which determines the coloring of the Node in the canvas.
- `Tags` (System.Collections.Generic.List<string>, get) — Search tags for this Node.

### Events
#### `ConnectorAdded` (`System.Action<Dynamo.Graph.Connectors.ConnectorModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Connectors.ConnectorModel> ConnectorAdded`

Event fired when a new ConnectorModel has been attached to one of this node's inputs.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `Modified` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> Modified`

Event fired when the node's DesignScript AST should be recompiled

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `NodeExecutionBegin` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeExecutionBegin`

Event triggered before a node is executed. Note: This event will only be triggered when profiling is active.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `NodeExecutionEnd` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeExecutionEnd`

Event triggered after a node is executed. Note: This event will only be triggered when profiling is active.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `NodeInfoMessagesClearing` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeInfoMessagesClearing`

Event triggered whenever the node re-executes to clear its info messages

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `NodeMessagesClearing` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeMessagesClearing`

Event triggered whenever the node re-executes to clear its warnings and errors.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `NodeWarningMessagesClearing` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> NodeWarningMessagesClearing`

Event triggered when the node clears only warning messages

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `PortConnected` (`System.Action<Dynamo.Graph.Nodes.PortModel, Dynamo.Graph.Connectors.ConnectorModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.PortModel, Dynamo.Graph.Connectors.ConnectorModel> PortConnected`

Event triggered when a port is connected.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `PortDisconnected` (`System.Action<Dynamo.Graph.Nodes.PortModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.PortModel> PortDisconnected`

Event triggered when a port is disconnected.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `RenderPackagesUpdated` (`System.Action<Dynamo.Graph.Nodes.NodeModel, Dynamo.Visualization.RenderPackageCache>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel, Dynamo.Visualization.RenderPackageCache> RenderPackagesUpdated`

NodeModel.RenderPackagesUpdated

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `RequestRenderPackages` (`System.Func<Dynamo.Visualization.RenderPackageCache>`)

**Signature:** `public event System.Func<Dynamo.Visualization.RenderPackageCache> RequestRenderPackages`



[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

#### `UpdateASTCollection` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> UpdateASTCollection`

Event fired when the node's DesignScript AST should be updated. This event deletes the frozen nodes from AST collection.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

## NodeModelExtensions (static-class)

Type NodeModelExtensions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModelExtensions.cs)

## NodeNameAttribute (class)

Type NodeNameAttribute

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void NodeNameAttribute(string elementName)` — NodeNameAttribute.NodeNameAttribute

### Properties
- `Name` (string, get/set) — NodeNameAttribute.Name

## NodeObsoleteAttribute (class)

The NodeObsoleteAttribute indicates this node is obsolete

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void NodeObsoleteAttribute(string message)` — NodeObsoleteAttribute.NodeObsoleteAttribute
- `void NodeObsoleteAttribute(string descriptionResourceID, System.Type resourceType)` — NodeObsoleteAttribute.NodeObsoleteAttribute

## NodeOutputData (class)

Represents a node which acts as a UI output for the graph - may also hold a value for that output

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeOutputData.cs)

### Constructors
- `void NodeOutputData()` — NodeOutputData.NodeOutputData

### Methods
#### `bool Equals(object obj)`

NodeOutputData.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeOutputData.cs)

#### `Dynamo.Graph.Nodes.NodeOutputTypes getNodeOutputTypeFromType(System.Type type)`

NodeOutputData.getNodeOutputTypeFromType

**Parameters:**
- `type` (System.Type)

**Returns:** `Dynamo.Graph.Nodes.NodeOutputTypes` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeOutputData.cs)

### Properties
- `Description` (string, get/set) — Description displayed to user of this output node.
- `Id` (System.Guid, get/set) — The id of the node.
- `InitialValue` (string, get/set) — The value of the output when the graph was saved. This should always be a string for all types.
- `Name` (string, get/set) — Display name of the output node.
- `Type` (Dynamo.Graph.Nodes.NodeOutputTypes, get/set) — The type of output this node is.

## NodeOutputTypes (enum)

Type NodeOutputTypes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeOutputData.cs)

### Values
- `booleanOutput` = `2`
- `floatOutput` = `1`
- `integerOutput` = `0`
- `stringOutput` = `3`
- `unknownOutput` = `4`

## NodeSearchTagsAttribute (class)

The NodeSearchTagsAttribute attribute allows the node implementer to define search tags.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void NodeSearchTagsAttribute(string tagsID, System.Type resourceType)` — Creates NodeSearchTagsAttribute with tags from resx file.
- `void NodeSearchTagsAttribute(string[] tags)` — Creates NodeSearchTagsAttribute with string tags.

### Properties
- `Tags` (System.Collections.Generic.List<string>, get/set) — Search tags used in library search.

## OutPortDescriptionsAttribute (class)

Indicates output ports' description

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void OutPortDescriptionsAttribute(System.Type resourceType, string[] resourceNames)` — Loads ports descriptions from resource.
- `void OutPortDescriptionsAttribute(string[] descriptions)` — Loads ports descriptions, that are specified directly in Attribute.

### Properties
- `PortDescriptions` (System.Collections.Generic.IEnumerable<string>, get) — Port descriptions, that are shown in Dynamo.

## OutPortNamesAttribute (class)

Indicates output ports' names.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void OutPortNamesAttribute(System.Type resourceType, string[] resourceNames)` — Loads ports names from resource.
- `void OutPortNamesAttribute(string[] names)` — Loads ports names, that are specified directly in Attribute.

### Properties
- `PortNames` (System.Collections.Generic.IEnumerable<string>, get) — Port titles, that are shown in Dynamo.

## OutPortTypesAttribute (class)

Indicates output ports' types

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/Attributes.cs)

### Constructors
- `void OutPortTypesAttribute(System.Type resourceType, string[] resourceNames)` — Loads ports types from resource.
- `void OutPortTypesAttribute(string[] types)` — Loads ports types, that are specified directly in Attribute.

### Properties
- `PortTypes` (System.Collections.Generic.IEnumerable<string>, get) — Port types, that are shown in Dynamo tooltips.

## PortData (class)

PortData stores information for port. It's used for constructing PortModel.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

### Constructors
- `void PortData(string name, string toolTipString)` — Creates PortData.
- `void PortData(string name, string toolTipString, ProtoCore.AST.AssociativeAST.AssociativeNode defaultValue)` — Creates PortData.

### Properties
- `DefaultValue` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — Default value of the port.
- `Height` (double, get/set) — Height of the port.
- `LineIndex` (int, get/set) — This property is used in code block nodes.
- `Name` (string, get/set) — Name of the port.
- `ToolTipString` (string, get/set) — Tooltip of the port.

## PortEventType (enum)

Defines Enum for Mouse events. Used in port snapping.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Values
- `MouseEnter` = `0`
- `MouseLeave` = `1`
- `MouseLeftButtonDown` = `2`

## PortModel (class)

PortModel represents Dynamo ports.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

### Constructors
- `void PortModel(Dynamo.Graph.Nodes.PortType portType, Dynamo.Graph.Nodes.NodeModel owner, Dynamo.Graph.Nodes.PortData data)` — Creates PortModel.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

PortModel.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

#### `void Dispose()`

PortModel.Dispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

#### `bool Equals(Dynamo.Graph.Nodes.PortModel other)`

PortModel.Equals

**Parameters:**
- `other` (Dynamo.Graph.Nodes.PortModel)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

#### `int GetHashCode()`

PortModel.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

PortModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

### Properties
- `Center` (Dynamo.Utilities.Point2D, get) — Center is used by connected connectors to update their shape The "center" of a port is derived from the type of port and offsets from the node origin based on the port's index in the ports collection.
- `Connectors` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Graph.Connectors.ConnectorModel>, get/set) — Returns the connectors between the specified ports.
- `DefaultValue` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — Default value for port.
- `GUID` (System.Guid, get/set) — ID of the PortModel, which is unique within the graph.
- `Index` (int, get) — Index of the port.
- `IsConnected` (bool, get) — Returns true if the port has connectors or if the default value is enabled and not null. Otherwise, returns false.
- `IsEnabled` (bool, get/set) — Indicates whether the port is enabled or not.
- `KeepListStructure` (bool, get/set) — A flag which determines whether data from this node will be re-aligned into the original structure of the nested list.
- `Level` (int, get/set) — The Level at which objects will be extracted from a nested list. The deepest level of a nested list is -1.
- `LineIndex` (int, get) — Returns the LineIndex of that port. The vertical position of PortModel is dependent on LineIndex.
- `MarginThickness` (Dynamo.Utilities.Thickness, get) — Controls the space between successive output ports
- `Name` (string, get) — Name of the port.
- `Owner` (Dynamo.Graph.Nodes.NodeModel, get) — Returns the Node.
- `PortType` (Dynamo.Graph.Nodes.PortType, get) — Type of the port. It can be incoming or outcoming.
- `ToolTip` (string, get/set) — Tooltip of the port.
- `UseLevels` (bool, get/set) — A flag which determines whether this Port will extract data from a specific level in a nested list.
- `UsingDefaultValue` (bool, get/set) — Controls whether this port is set to use it's default value (true) or yield a closure (false).
- `extensionEdges` (Dynamo.Graph.Nodes.SnapExtensionEdges, get/set) — Based on extensionEdges port is aligned in UI.

## PortType (enum)

Interaction logic for dynPort.xaml

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/PortModel.cs)

### Values
- `Input` = `0`
- `Output` = `1`

## ScopedNodeModel (class)

ScopedNodeModel will put its children in its scope so that they won't get compiled in global scope.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ScopedNodeModel.cs)

### Constructors
- `void ScopedNodeModel()` — ScopedNodeModel.ScopedNodeModel
- `void ScopedNodeModel(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> inPorts, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> outPorts)` — ScopedNodeModel.ScopedNodeModel

### Methods
#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> BuildOutputAstInScope(System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes, bool verboseLogging, Dynamo.Engine.CodeGeneration.AstBuilder builder)`

Similar to NodeModel.BuildOutputAst(). When compiled to AST, for ScopedNodeModel this method will be called when all requirements are satisfied. The derived class needs to implement this method to compile its children into some scopes.

**Parameters:**
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — 
- `verboseLogging` (bool) — 
- `builder` (Dynamo.Engine.CodeGeneration.AstBuilder) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ScopedNodeModel.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> GetInScopeNodesForInport(int portIndex, bool checkEscape, bool isInclusive, bool forceToGetNodeForInport)`

Returns all nodes that in its input ports' scope. A node is in its scope if that node is one of its upstream nodes.

**Parameters:**
- `portIndex` (int) — Inport index
- `checkEscape` (bool) — If need to exclude nodes that one of their downstream nodes are not in the scope
- `isInclusive` (bool) — If a upstream node is ScopedNodeModel, need to include all upstream nodes of that node.
- `forceToGetNodeForInport` (bool) — 

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ScopedNodeModel.cs)

#### `bool IsScopedInport(int portIndex)`

Specify if the corresponding inport has scope or not.

**Parameters:**
- `portIndex` (int) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ScopedNodeModel.cs)

## SnapExtensionEdges (enum)

Returns one of the possible values(none, top, bottom) where a port can be snapped.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Values
- `Bottom` = `2`
- `None` = `0`
- `Top` = `1`

## Statement (class)

Statements are used in CBN in order to create output ports.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

### Methods
#### `Dynamo.Graph.Nodes.Statement CreateInstance(ProtoCore.AST.Node parsedNode)`

Creates Statement from node

**Parameters:**
- `parsedNode` (ProtoCore.AST.Node) — ProtoCore.AST.Node

**Returns:** `Dynamo.Graph.Nodes.Statement` — Statement

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `System.Collections.Generic.List<string> GetDefinedVariableNames(Dynamo.Graph.Nodes.Statement s)`

Returns the names of the variables that have been declared in the statement

**Parameters:**
- `s` (Dynamo.Graph.Nodes.Statement) — Statement whose variable names to be queried.

**Returns:** `System.Collections.Generic.List<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

#### `void GetReferencedVariables(ProtoCore.AST.Node astNode, System.Collections.Generic.List<Dynamo.Graph.Nodes.Variable> refVariableList)`

Returns variables from AST nodes. E.g. a+5. Here "a" is variable.

**Parameters:**
- `astNode` (ProtoCore.AST.Node) — ProtoCore.AST.Node
- `refVariableList` (System.Collections.Generic.List<Dynamo.Graph.Nodes.Variable>) — list of variables

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

### Properties
- `AstNode` (ProtoCore.AST.Node, get) — ProtoCore.AST.Node
- `CurrentState` (Dynamo.Graph.Nodes.Statement.State, get) — Returns the State of the Statement. E.g. normal, warning or error.
- `CurrentType` (Dynamo.Graph.Nodes.Statement.StatementType, get) — Returns the type of the statement. E.g. expression, literal etc.
- `EndLine` (int, get) — Returns the index of the EndLine. E.g. a+5 +6+3; Endline will be 2.
- `FirstDefinedVariable` (Dynamo.Graph.Nodes.Variable, get) — Statement.FirstDefinedVariable
- `StartLine` (int, get) — Returns the index of the Startline. E.g. a+5 StartLine will be 1.

## TypeLoadData (class)

This class represents data loaded from assembly for each type. Based on this info node model is created.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/TypeLoadData.cs)

### Constructors
- `void TypeLoadData(System.Type typeIn)` — Creates TypeLoadData.

### Properties
- `Assembly` (System.Reflection.Assembly, get) — Assembly containing the type.
- `Category` (string, get) — The category of this type, used in search.
- `IsObsolete` (bool, get) — Specifies whether or not this Type is obsolete.

## UIDispatcherEventArgs (class)

This class represents the UIDIspatcher thread event arguments.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeModel.cs)

### Constructors
- `void UIDispatcherEventArgs(System.Action a)` — Creates UIDispatcherEventArgs.

### Properties
- `ActionToDispatch` (System.Action, get/set) — Action to call on UI thread.

## Utilities (static-class)

Service class for formatting node text data

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeCategories.cs)

## Variable (class)

Represents variable in CBN.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

### Constructors
- `void Variable(ProtoCore.AST.AssociativeAST.IdentifierNode identNode)` — Creates Variable

### Methods
#### `void SetCorrectColumn(System.Collections.Generic.List<Dynamo.Graph.Nodes.Variable> refVar, Dynamo.Graph.Nodes.Statement.StatementType type, int line)`

Moves column index back only if variable is not an expression.

**Parameters:**
- `refVar` (System.Collections.Generic.List<Dynamo.Graph.Nodes.Variable>) — list of variables
- `type` (Dynamo.Graph.Nodes.Statement.StatementType) — statement type
- `line` (int) — line index

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CodeBlockNode.cs)

### Properties
- `EndColumn` (int, get) — Returns the index of end column.
- `Name` (string, get) — This returns the name of the variable. E.g. a = 5; Name will be "a".
- `NameWithIndex` (string, get) — This returns the name of the list including its index. E.g. for "a[0] = 5;", NameWithIndex will be "a[0]". It simply returns the name of the variable otherwise. E.g. for "a = 5;" NameWithIndex will be "a".
- `Row` (int, get) — Returns the index of row.
- `StartColumn` (int, get) — Returns the index of start column.

## VariableInputNode (class)

Base class for nodes that have dynamic incoming ports. E.g. list.create.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

### Constructors
- `void VariableInputNode()` — Protected constructor for deserialization
- `void VariableInputNode(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> inPorts, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> outPorts)` — Protected constructor for deserialization

### Methods
#### `void AddInput()`

VariableInputNode.AddInput

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

VariableInputNode.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `int GetInputIndex()`

VariableInputNode.GetInputIndex

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `string GetInputName(int index)`

VariableInputNode.GetInputName

**Parameters:**
- `index` (int)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `string GetInputTooltip(int index)`

VariableInputNode.GetInputTooltip

**Parameters:**
- `index` (int)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void OnBuilt()`

VariableInputNode.OnBuilt

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void RemoveInput()`

VariableInputNode.RemoveInput

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

VariableInputNode.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

### Properties
- `VariableInputPorts` (bool, get) — A flag used during serialization to indicated that the node has a variable number of input ports.

## VariableInputNodeController (class)

This is a helper class that processess inputs of VariableInputNode.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

### Constructors
- `void VariableInputNodeController(Dynamo.Graph.Nodes.NodeModel model)` — VariableInputNodeController.VariableInputNodeController

### Methods
#### `void AddInputToModel()`

Adds an input to this node. Called when the '+' button is clicked.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void DeserializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Deserializes object

**Parameters:**
- `element` (System.Xml.XmlElement) — xml node
- `context` (Dynamo.Graph.SaveContext) — save context

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `int GetInputIndexFromModel()`

Fetches the index number to use for the next port.

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `string GetInputName(int index)`

VariableInputNodeController.GetInputName

**Parameters:**
- `index` (int)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `string GetInputTooltip(int index)`

VariableInputNodeController.GetInputTooltip

**Parameters:**
- `index` (int)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void OnBuilt()`

This is called when a node is built.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void RemoveInputFromModel()`

Removes an input from this node. Called when the '-' button is clicked.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Serializes object

**Parameters:**
- `element` (System.Xml.XmlElement) — xml node
- `context` (Dynamo.Graph.SaveContext) — save context

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void SerializeInputCount(System.Xml.XmlElement nodeElement, int amount)`

Serializes the input count of a VariableInputNode to Xml.

**Parameters:**
- `nodeElement` (System.Xml.XmlElement) — 
- `amount` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

#### `void SetNumInputs(int numInputs)`

Set the number of inputs.

**Parameters:**
- `numInputs` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/VariableInputNode.cs)

