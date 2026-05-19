---
name: dynamo-dynamo-engine-codegeneration
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Engine.CodeGeneration namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AstBuilder, CompilationContext, CompiledEventArgs, CompilingEventArgs, IAstNodeContainer.
---

# Dynamo.Engine.CodeGeneration

Auto-generated from vendor docs for dynamo 4.1.1. 5 types in this namespace.

## AstBuilder (class)

AstBuilder is a factory class to create different kinds of AST nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/AstBuilder.cs)

### Methods
#### `System.Collections.Generic.IEnumerable<System.Tuple<Dynamo.Graph.Nodes.NodeModel, System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>>> CompileToAstNodes(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, Dynamo.Engine.CodeGeneration.CompilationContext context, bool verboseLogging)`

Compile a collection of NodeModel to AST nodes in different contexts. If the context is ForNodeToCode, nodes should already be sorted in topological order.

**Parameters:**
- `nodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>) — 
- `context` (Dynamo.Engine.CodeGeneration.CompilationContext) — 
- `verboseLogging` (bool) — 

**Returns:** `System.Collections.Generic.IEnumerable<System.Tuple<Dynamo.Graph.Nodes.NodeModel, System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/AstBuilder.cs)

## CompilationContext (enum)

The context of AST compilation

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/CompilationContext.cs)

### Values
- `DeltaExecution` = `1` — Compiled AST nodes finally will be executed.
- `NodeToCode` = `2` — Compiled AST nodes used in node to code.
- `None` = `0` — No specific context.
- `PreviewGraph` = `3` — Compiled AST nodes used in previwing graph.

## CompiledEventArgs (class)

This event is triggered when a NodeModel has been compiled to a list of AST nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/CompiledEventArgs.cs)

### Properties
- `AstNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>, get) — Built AST nodes.
- `NodeId` (System.Guid, get) — Guid of node that has been built to AST nodes.

## CompilingEventArgs (class)

This event is triggerred when compiling a NodeModel to AST nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/CompilingEventArgs.cs)

### Constructors
- `void CompilingEventArgs(System.Guid node)` — CompilingEventArgs.CompilingEventArgs

### Properties
- `NodeId` (System.Guid, get) — Guid of NodeModel that is being compiled to AST.

## IAstNodeContainer (interface)

Returns notification for AST compilation events.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/IAstNodeContainer.cs)

### Methods
#### `void OnCompiled(System.Guid nodeGuid, System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> astNodes)`

Indicates a NodeModel has been compiled to AST nodes.

**Parameters:**
- `nodeGuid` (System.Guid) — 
- `astNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/IAstNodeContainer.cs)

#### `void OnCompiling(System.Guid nodeGuid)`

Indicates to start compiling a NodeModel to AST nodes.

**Parameters:**
- `nodeGuid` (System.Guid) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/CodeGeneration/IAstNodeContainer.cs)

