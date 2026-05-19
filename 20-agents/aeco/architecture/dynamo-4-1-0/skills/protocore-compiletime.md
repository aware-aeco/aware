---
name: dynamo-protocore-compiletime
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.CompileTime namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Context.
---

# ProtoCore.CompileTime

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## Context (class)

Type Context

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Context.cs)

### Constructors
- `void Context()` — Context.Context
- `void Context(string source, System.Collections.Generic.Dictionary<string, object> context, System.Collections.Generic.Dictionary<string, bool> flagList)` — Context.Context

### Methods
#### `void SetData(string source, System.Collections.Generic.Dictionary<string, object> context, System.Collections.Generic.Dictionary<string, bool> flagList)`

Context.SetData

**Parameters:**
- `source` (string)
- `context` (System.Collections.Generic.Dictionary<string, object>)
- `flagList` (System.Collections.Generic.Dictionary<string, bool>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Context.cs)

#### `void SetExprInterpreterProperties(int currentBlockID, ProtoCore.Runtime.RuntimeMemory memState, int watchScope, ProtoCore.DebugProperties debugProps)`

Context.SetExprInterpreterProperties

**Parameters:**
- `currentBlockID` (int)
- `memState` (ProtoCore.Runtime.RuntimeMemory)
- `watchScope` (int)
- `debugProps` (ProtoCore.DebugProperties)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Context.cs)

### Properties
- `CurrentBlockId` (int, get) — Context.CurrentBlockId
- `DebugProps` (ProtoCore.DebugProperties, get) — Context.DebugProps
- `DependentVariablesInScope` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get/set) — Context.DependentVariablesInScope
- `GlobalVarList` (System.Collections.Generic.Dictionary<string, object>, get) — Context.GlobalVarList
- `MemoryState` (ProtoCore.Runtime.RuntimeMemory, get) — Context.MemoryState
- `SourceCode` (string, get) — Context.SourceCode
- `WatchClassScope` (int, get/set) — When compiling expression interpreter code, the codegen needs a copy of certain runtime values
- `applySSATransform` (bool, get/set) — This flag controls whether we want a full codeblock to apply SSA Transform. Currently it is used to prevent SSA on inline conditional bodies. This will be resolved when inline replication is fixed
- `execFlagList` (System.Collections.Generic.Dictionary<string, bool>, get) — Context.execFlagList
- `exprExecutionFlags` (System.Collections.Generic.Dictionary<int, bool>, get/set) — Context.exprExecutionFlags
- `guid` (System.Guid, get/set) — Context.guid
- `symbolTable` (ProtoCore.DSASM.SymbolTable, get/set) — Context.symbolTable

