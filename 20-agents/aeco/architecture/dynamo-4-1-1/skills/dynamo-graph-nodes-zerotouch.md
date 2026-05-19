---
name: dynamo-dynamo-graph-nodes-zerotouch
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Graph.Nodes.ZeroTouch namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DSFunction, DSFunctionBase, ZeroTouchNodeController<T>, DSVarArgFunction, ZeroTouchVarArgNodeController<T>, UnresolvedFunctionException.
---

# Dynamo.Graph.Nodes.ZeroTouch

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## DSFunction (class)

DesignScript function node. All functions from DesignScript share the same function node but internally have different procedure.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunction.cs)

### Constructors
- `void DSFunction(Dynamo.Engine.FunctionDescriptor functionDescription)` — Initializes a new instance of the Dynamo.Graph.Nodes.ZeroTouch.DSFunction class.

### Properties
- `FunctionSignature` (string, get) — DSFunction.FunctionSignature
- `IsInputNode` (bool, get) — Indicates whether Node is input or not. Used to bind visibility of UI for user selection.
- `NodeType` (string, get) — The NodeType property provides a name which maps to the server type for the node. This property should only be used for serialization.

## DSFunctionBase (class)

Base class for NodeModels representing zero-touch-imported-function calls.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

### Constructors
- `void DSFunctionBase(Dynamo.Graph.Nodes.ZeroTouch.ZeroTouchNodeController<Dynamo.Engine.FunctionDescriptor> controller)` — Initializes a new instance of the Dynamo.Graph.Nodes.ZeroTouch.DSFunctionBase class.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

DSFunctionBase.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierNode GetAstIdentifierForOutputIndex(int outputIndex)`

Fetches the ProtoAST Identifier for a given output index.

**Parameters:**
- `outputIndex` (int) — Index of the output port.

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierNode` — Identifier corresponding to the given output port.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `ProtoCore.Type GetTypeHintForOutput(int index)`

Returns the possible type of output at specified port.

**Parameters:**
- `index` (int) — Index of the port

**Returns:** `ProtoCore.Type` — The type

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Copy command will call it to serialize this node to xml data.

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

### Properties
- `IsConvertible` (bool, get) — Indicates if this node is allowed to be converted to AST node in nodes to code conversion.

## DSVarArgFunction (class)

DesignScript var-arg function node. All functions from DesignScript share the same function node but internally have different procedure.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSVarArgFunction.cs)

### Constructors
- `void DSVarArgFunction(Dynamo.Engine.FunctionDescriptor descriptor)` — Initializes a new instance of the Dynamo.Graph.Nodes.ZeroTouch.DSVarArgFunction class.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

DSVarArgFunction.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSVarArgFunction.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

DSVarArgFunction.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSVarArgFunction.cs)

### Properties
- `FunctionSignature` (string, get) — The function name with required parameters.
- `FunctionType` (string, get) — It indicates which of the three types of function calls this node represents, a call to an external graph, a call to a function with a vararg argument, or a standard function.
- `NodeType` (string, get) — The NodeType property provides a name which maps to the server type for the node. This property should only be used for serialization.
- `VarInputController` (Dynamo.Graph.Nodes.VariableInputNodeController, get) — Custom VariableInput controller for DSVarArgFunctions.

## UnresolvedFunctionException (class)

The exception that is thrown when definition for a custom node cannot be found

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/UnresolvedFunctionException.cs)

### Constructors
- `void UnresolvedFunctionException(string functionName)` — Initializes a new instance of the Dynamo.Graph.Nodes.ZeroTouch.UnresolvedFunctionException class with the name of the custom node that causes this exception.

### Properties
- `FunctionName` (string, get) — Returns user friendly name of not found custom node.

## ZeroTouchNodeController<T> (class)

Controller that synchronizes a node with a zero-touch function definition.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

### Constructors
- `void ZeroTouchNodeController(T definition)` — Initializes a new instance of the Dynamo.Graph.Nodes.ZeroTouch.ZeroTouchNodeController`1 class with FunctionDescriptor.

### Methods
#### `ProtoCore.AST.AssociativeAST.AssociativeNode CreateFunctionObject(Dynamo.Graph.Nodes.NodeModel model, ProtoCore.AST.AssociativeAST.AssociativeNode functionNode, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputs)`

Creates a FunctionObject representing a partial application of a function.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Node to produce FunctionObject for.
- `functionNode` (ProtoCore.AST.AssociativeAST.AssociativeNode) — AST representing the function to make a FunctionObject out of.
- `inputs` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Arguments to be applied partially.

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode GetFunctionApplication(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes)`

ZeroTouchNodeController.GetFunctionApplication

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `void InitializeFunctionParameters(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter> parameters)`

Initializes a node's InPortData based on a list of parameters.

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — Node to initialize.
- `parameters` (System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter>) — Parameters used for initialization.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `void InitializeInputs(Dynamo.Graph.Nodes.NodeModel model)`

ZeroTouchNodeController.InitializeInputs

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `void InitializeOutputs(Dynamo.Graph.Nodes.NodeModel model)`

ZeroTouchNodeController.InitializeOutputs

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `bool IsConstructor()`

Is this function a constructor of a class?

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `bool IsInstanceMember()`

Is this function an instance member of a class?

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `bool IsStaticMember()`

Is this function a static member of a class?

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Serializes data into given System.Xml.XmlElement object.

**Parameters:**
- `element` (System.Xml.XmlElement) — System.Xml.XmlElement object to store data.
- `context` (Dynamo.Graph.SaveContext) — Saving context.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

#### `void SyncNodeWithDefinition(Dynamo.Graph.Nodes.NodeModel model)`

Syncronizes custom node instance with its definition

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel) — The custom node instance

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSFunctionBase.cs)

### Properties
- `Category` (string, get) — Category of function, taken from Definition.
- `Description` (string, get) — Description of function, taken from Definition.
- `MangledName` (string, get) — MangledName of function, taken from Definition.

## ZeroTouchVarArgNodeController<T> (class)

Controller that extends Zero Touch synchronization with VarArg function compilation.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSVarArgFunction.cs)

### Constructors
- `void ZeroTouchVarArgNodeController(T zeroTouchDef)` — Initializes a new instance of the Dynamo.Graph.Nodes.ZeroTouch.ZeroTouchVarArgNodeController`1 class with FunctionDescriptor.

### Methods
#### `void BuildOutputAst(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputAstNodes, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> resultAst)`

ZeroTouchVarArgNodeController.BuildOutputAst

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)
- `inputAstNodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `resultAst` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSVarArgFunction.cs)

#### `void InitializeFunctionParameters(Dynamo.Graph.Nodes.NodeModel model, System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter> parameters)`

ZeroTouchVarArgNodeController.InitializeFunctionParameters

**Parameters:**
- `model` (Dynamo.Graph.Nodes.NodeModel)
- `parameters` (System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/ZeroTouch/DSVarArgFunction.cs)

