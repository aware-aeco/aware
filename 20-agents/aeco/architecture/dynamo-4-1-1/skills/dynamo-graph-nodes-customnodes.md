---
name: dynamo-dynamo-graph-nodes-customnodes
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Graph.Nodes.CustomNodes namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CustomNodeController<T>, Function, Symbol, Output, ICustomNodeManager, ICustomNodeSource.
---

# Dynamo.Graph.Nodes.CustomNodes

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## CustomNodeController<T> (class)

Controller that synchronizes a custom node with a node definition.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

### Constructors
- `void CustomNodeController(T def)` — CustomNodeController.CustomNodeController

### Methods
#### `void AssignIdentifiersForFunctionCall(Dynamo.Graph.Nodes.NodeModel model, ProtoCore.AST.AssociativeAST.AssociativeNode rhs, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> resultAst)`

CustomNodeController.AssignIdentifiersForFunctionCall

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)
- `rhs` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `resultAst` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

#### `void BuildOutputAst(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> resultAst)`

CustomNodeController.BuildOutputAst

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `resultAst` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode GetFunctionApplication(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes)`

CustomNodeController.GetFunctionApplication

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

#### `void InitializeInputs(Dynamo.Graph.Nodes.NodeModel model)`

CustomNodeController.InitializeInputs

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

#### `void InitializeOutputs(Dynamo.Graph.Nodes.NodeModel model)`

CustomNodeController.InitializeOutputs

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

#### `bool IsInSyncWithNode(Dynamo.Graph.Nodes.NodeModel model)`

Return if the custom node instance is in sync with its definition. It may be out of sync if .dyf file is opened and updated and then .dyn file is opened.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

#### `void SerializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext saveContext)`

Serializes CustomNode.

**Parameters:**
- `nodeElement` (System.Xml.XmlElement) — Xml node
- `saveContext` (Dynamo.Graph.SaveContext) — SaveContext is used in base class.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

#### `void SyncNodeWithDefinition(Dynamo.Graph.Nodes.NodeModel model)`

Synchronizes custom node with its definition.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Custom node model

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/CustomNodeController.cs)

## Function (class)

DesignScript Custom Node instance.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

### Constructors
- `void Function(Dynamo.CustomNodeDefinition def, string name, string description, string category)` — Initializes a new instance of the Dynamo.Graph.Nodes.CustomNodes.Function class.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

Function.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `void ResyncWithDefinition(Dynamo.CustomNodeDefinition def)`

Validates passed Custom Node definition and synchronizes node with it.

**Parameters:**
- `def` (Dynamo.CustomNodeDefinition) — Custom Node definition.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Function.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `void UpdatePortsForUnresolved(Dynamo.Graph.Nodes.PortModel[] inputs, Dynamo.Graph.Nodes.PortModel[] outputs)`

Initializes ports with default information when the function is unresolved.

**Parameters:**
- `inputs` (Dynamo.Graph.Nodes.PortModel[]) — The input nodes for tis function node.
- `outputs` (Dynamo.Graph.Nodes.PortModel[]) — The output nodes for tis function node.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

### Properties
- `Definition` (Dynamo.CustomNodeDefinition, get) — Returns customNode definition.
- `FunctionSignature` (System.Guid, get) — The unique id of the underlying function.
- `FunctionType` (string, get) — It indicates which of the three types of function calls this node represents, a call to an external graph, a call to a function with a vararg argument, or a standard function.
- `NodeType` (string, get) — The type of node.

## ICustomNodeManager (interface)

Type ICustomNodeManager

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/ICustomNodeManager.cs)

### Methods
#### `System.Collections.Generic.IEnumerable<Dynamo.CustomNodeInfo> AddUninitializedCustomNodesInPath(string path, bool isTestMode, bool isPackageMember)`

ICustomNodeManager.AddUninitializedCustomNodesInPath

**Parameters:**
- `path` (string)
- `isTestMode` (bool)
- `isPackageMember` (bool)

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.CustomNodeInfo>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/ICustomNodeManager.cs)

#### `System.Guid GuidFromPath(string path)`

ICustomNodeManager.GuidFromPath

**Parameters:**
- `path` (string)

**Returns:** `System.Guid` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/ICustomNodeManager.cs)

#### `bool TryGetFunctionWorkspace(System.Guid id, bool isTestMode, ref Dynamo.Graph.Workspaces.ICustomNodeWorkspaceModel ws)`

ICustomNodeManager.TryGetFunctionWorkspace

**Parameters:**
- `id` (System.Guid)
- `isTestMode` (bool)
- `ws` (ref Dynamo.Graph.Workspaces.ICustomNodeWorkspaceModel)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/ICustomNodeManager.cs)

## ICustomNodeSource (interface)

Type ICustomNodeSource

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/ICustomNodeSource.cs)

### Methods
#### `Dynamo.Graph.Nodes.CustomNodes.Function CreateCustomNodeInstance(System.Guid id, string name, bool isTestMode)`

Creates a new Custom Node Instance.

**Parameters:**
- `id` (System.Guid) — Identifier referring to a custom node definition.
- `name` (string) — 
- `isTestMode` (bool) — 

**Returns:** `Dynamo.Graph.Nodes.CustomNodes.Function` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/ICustomNodeSource.cs)

## Output (class)

Represents function output. It contains functionality to manage expressions and store function's node value to pass.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

### Constructors
- `void Output()` — Create output node.
- `void Output(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> inPorts, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> outPorts)` — Create output node.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

Output.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `void SerializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

Output.SerializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

Output.UpdateValueCore

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

### Properties
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get/set) — Element resolver
- `IsInputNode` (bool, get) — Indicates whether node is input node. Used to bind visibility of UI for user selection.
- `IsOutputNode` (bool, get) — Indicates whether node is output node. Used to bind visibility of UI for user selection.
- `NodeType` (string, get) — The NodeType property provides a name which maps to the server type for the node. This property should only be used for serialization.
- `Return` (System.Tuple<string, string>, get) — Output name and its description tuple.
- `Symbol` (string, get/set) — Text in output node.

## Symbol (class)

Represents function entry point. It contains functionality to manage expressions and applies input values to function logic.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

### Constructors
- `void Symbol()` — Initializes a new instance of the Dynamo.Graph.Nodes.CustomNodes.Symbol class.
- `void Symbol(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> inPorts, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.PortModel> outPorts, Dynamo.Library.TypedParameter parameter, ProtoCore.Namespace.ElementResolver elementResolver)` — Initializes a new instance of the Dynamo.Graph.Nodes.CustomNodes.Symbol class.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

Symbol.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierNode GetAstIdentifierForOutputIndex(int outputIndex)`

Returns ProtoCore.AST.AssociativeAST.IdentifierNode by passed output index.

**Parameters:**
- `outputIndex` (int) — Output index.

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `void SerializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

Symbol.SerializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

Symbol.UpdateValueCore

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/CustomNodes/Function.cs)

### Properties
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get/set) — Responsible for resolving a partial class name to its fully resolved name
- `InputSymbol` (string, get/set) — Represents string input.
- `IsInputNode` (bool, get) — Indicates whether node is input node. Used to bind visibility of UI for user selection.
- `IsOutputNode` (bool, get) — Indicates whether node is output node. Used to bind visibility of UI for user selection.
- `NodeType` (string, get) — The NodeType property provides a name which maps to the server type for the node. This property should only be used for serialization.
- `Parameter` (Dynamo.Library.TypedParameter, get) — Returns tuple of Input parameter and its type.

