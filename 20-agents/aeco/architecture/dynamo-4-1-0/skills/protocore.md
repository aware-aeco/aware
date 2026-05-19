---
name: dynamo-protocore
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore namespace — 38 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildHaltException, OutputMessage, IOutputStream, TextOutputStream, ConsoleOutputStream, BuildStatus, Language, LanguageCodeBlock, and 30 more types.
---

# ProtoCore

Auto-generated from vendor docs for dynamo 4.1.0. 38 types in this namespace.

## AtLevel (class)

Type AtLevel

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

### Constructors
- `void AtLevel(int level, bool isDominant)` — AtLevel.AtLevel

### Properties
- `IsDominant` (bool, get) — AtLevel.IsDominant
- `Level` (int, get) — AtLevel.Level

## BackpatchNode (class)

Type BackpatchNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

### Constructors
- `void BackpatchNode()` — BackpatchNode.BackpatchNode

## BackpatchTable (class)

Type BackpatchTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

### Constructors
- `void BackpatchTable()` — BackpatchTable.BackpatchTable

### Methods
#### `void Append(int bp, int pc)`

BackpatchTable.Append

**Parameters:**
- `bp` (int)
- `pc` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

### Properties
- `backpatchList` (System.Collections.Generic.List<ProtoCore.BackpatchNode>, get) — BackpatchTable.backpatchList

## BuildHaltException (class)

Type BuildHaltException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Constructors
- `void BuildHaltException(string message)` — BuildHaltException.BuildHaltException

### Properties
- `ErrorMessage` (string, get) — BuildHaltException.ErrorMessage

## BuildStatus (class)

Type BuildStatus

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Constructors
- `void BuildStatus(ProtoCore.Core core, System.IO.TextWriter writer, bool errorAsWarning)` — BuildStatus.BuildStatus

### Methods
#### `void ClearErrors()`

BuildStatus.ClearErrors

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void ClearWarnings()`

BuildStatus.ClearWarnings

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void ClearWarningsForAst(int astID)`

BuildStatus.ClearWarningsForAst

**Parameters:**
- `astID` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void ClearWarningsForGraph(System.Guid guid)`

BuildStatus.ClearWarningsForGraph

**Parameters:**
- `guid` (System.Guid)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void LogDeprecatedMethodWarning(string oldMethodName, string newMethodName)`

BuildStatus.LogDeprecatedMethodWarning

**Parameters:**
- `oldMethodName` (string)
- `newMethodName` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void LogSemanticError(string msg, string fileName, int line, int col, ProtoCore.AssociativeGraph.GraphNode graphNode)`

BuildStatus.LogSemanticError

**Parameters:**
- `msg` (string)
- `fileName` (string)
- `line` (int)
- `col` (int)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void LogSymbolConflictWarning(string symbolName, string[] collidingSymbolNames)`

Logs the warning where the usage of a symbol (symbolName) cannot be resolved because it collides with multiple symbols(collidingSymbolNames)

**Parameters:**
- `symbolName` (string) — 
- `collidingSymbolNames` (string[]) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void LogSyntaxError(string msg, string fileName, int line, int col)`

BuildStatus.LogSyntaxError

**Parameters:**
- `msg` (string)
- `fileName` (string)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void LogUnboundVariableWarning(ProtoCore.DSASM.SymbolNode unboundSymbol, string message, string fileName, int line, int col, ProtoCore.AssociativeGraph.GraphNode graphNode)`

Logs the unbound variable warning and sets the unbound symbol

**Parameters:**
- `unboundSymbol` (ProtoCore.DSASM.SymbolNode) — 
- `message` (string) — 
- `fileName` (string) — 
- `line` (int) — 
- `col` (int) — 
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void LogWarning(ProtoCore.BuildData.WarningID warningID, string message, string fileName, int line, int col, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.DSASM.SymbolNode associatedSymbol)`

BuildStatus.LogWarning

**Parameters:**
- `warningID` (ProtoCore.BuildData.WarningID)
- `message` (string)
- `fileName` (string)
- `line` (int)
- `col` (int)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `associatedSymbol` (ProtoCore.DSASM.SymbolNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void RemoveUnboundVariableWarnings(System.Collections.Generic.HashSet<ProtoCore.DSASM.SymbolNode> symbolList)`

Remove unbound variable warnings that match all symbols in the symbolList

**Parameters:**
- `symbolList` (System.Collections.Generic.HashSet<ProtoCore.DSASM.SymbolNode>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

#### `void ReportBuildResult()`

BuildStatus.ReportBuildResult

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Properties
- `BuildSucceeded` (bool, get) — BuildStatus.BuildSucceeded
- `ErrorCount` (int, get) — BuildStatus.ErrorCount
- `Errors` (System.Collections.Generic.IEnumerable<ProtoCore.BuildData.ErrorEntry>, get) — BuildStatus.Errors
- `MessageHandler` (ProtoCore.IOutputStream, get/set) — BuildStatus.MessageHandler
- `WarningCount` (int, get) — BuildStatus.WarningCount
- `Warnings` (System.Collections.Generic.IEnumerable<ProtoCore.BuildData.WarningEntry>, get) — BuildStatus.Warnings

## CallSite (class)

Type CallSite

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

### Constructors
- `void CallSite(int classScope, string methodName, ProtoCore.Lang.FunctionTable globalFunctionTable, ProtoCore.ExecutionMode execMode, string serializedTraceData)` — Constructs an instance of the CallSite object given its scope and method information. This constructor optionally takes in a preloaded trace data information.

### Methods
#### `System.Collections.Generic.IList<string> GetAllSerializablesFromSingleRunTraceData(ProtoCore.CallSite.RawTraceData callSiteData)`

Returns a flat collection of strings from a serialized representation of a SingleRunTraceData object.

**Parameters:**
- `callSiteData` (ProtoCore.CallSite.RawTraceData) — The serialized representation of a SingleRunTraceData object.

**Returns:** `System.Collections.Generic.IList<string>` — A flat collection of strings.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `System.Collections.Generic.IList<string> GetOrphanedSerializables()`

Returns all serializables that were created historically, but were not re-created in the most recent graph update.

**Returns:** `System.Collections.Generic.IList<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `string GetTraceDataToSave()`

Call this method to obtain the Base64 encoded string that represents this callsite instance's trace data

**Returns:** `string` — Returns the Base64 encoded string that represents the trace data of this callsite

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `ProtoCore.DSASM.StackValue JILDispatch(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments, System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>> replicationGuides, ProtoCore.Lang.Replication.DominantListStructure domintListStructure, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore, ProtoCore.Runtime.Context context)`

CallSite.JILDispatch

**Parameters:**
- `arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationGuides` (System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>>)
- `domintListStructure` (ProtoCore.Lang.Replication.DominantListStructure)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `runtimeCore` (ProtoCore.RuntimeCore)
- `context` (ProtoCore.Runtime.Context)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `ProtoCore.DSASM.StackValue JILDispatchViaNewInterpreter(ProtoCore.Runtime.Context context, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments, System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>> replicationGuides, ProtoCore.Lang.Replication.DominantListStructure domintListStructure, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore)`

CallSite.JILDispatchViaNewInterpreter

**Parameters:**
- `context` (ProtoCore.Runtime.Context)
- `arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationGuides` (System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>>)
- `domintListStructure` (ProtoCore.Lang.Replication.DominantListStructure)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `void LoadSerializedDataIntoTraceCache(string serializedTraceData)`

Load the serialized data provided into this callsite's trace cache

**Parameters:**
- `serializedTraceData` (string) — The data to load

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `ProtoCore.DSASM.StackValue PerformReturnTypeCoerce(ProtoCore.DSASM.ProcedureNode procNode, ProtoCore.RuntimeCore runtimeCore, ProtoCore.DSASM.StackValue ret)`

CallSite.PerformReturnTypeCoerce

**Parameters:**
- `procNode` (ProtoCore.DSASM.ProcedureNode)
- `runtimeCore` (ProtoCore.RuntimeCore)
- `ret` (ProtoCore.DSASM.StackValue)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `ProtoCore.DSASM.StackValue PerformReturnTypeCoerce(ProtoCore.FunctionEndPoint functionEndPoint, ProtoCore.RuntimeCore runtimeCore, ProtoCore.DSASM.StackValue ret)`

CallSite.PerformReturnTypeCoerce

**Parameters:**
- `functionEndPoint` (ProtoCore.FunctionEndPoint)
- `runtimeCore` (ProtoCore.RuntimeCore)
- `ret` (ProtoCore.DSASM.StackValue)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `void UpdateCallSite(int classScope, string methodName)`

CallSite.UpdateCallSite

**Parameters:**
- `classScope` (int)
- `methodName` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `bool WillCallReplicate(ProtoCore.Runtime.Context context, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments, System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>> partialReplicationGuides, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore, ref System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>> replicationTrials)`

Conservative guess as to whether this call will replicate or not This may give inaccurate answers if the node cluster doesn't actually exist

**Parameters:**
- `context` (ProtoCore.Runtime.Context) — 
- `arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `partialReplicationGuides` (System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>>) — 
- `stackFrame` (ProtoCore.DSASM.StackFrame) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 
- `replicationTrials` (ref System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>>) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

### Properties
- `CallSiteID` (System.Guid, get) — CallSite.CallSiteID
- `MethodName` (string, get) — The method group name that is associated with this function
- `TraceData` (System.Collections.Generic.List<ProtoCore.CallSite.SingleRunTraceData>, get) — CallSite.TraceData

## CodeGen (class)

Type CodeGen

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

### Constructors
- `void CodeGen(ProtoCore.Core coreObj, ProtoCore.DSASM.CodeBlock parentBlock)` — CodeGen.CodeGen

### Methods
#### `void AllocateVar(ProtoCore.DSASM.SymbolNode symbol)`

CodeGen.AllocateVar

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void Backpatch(System.Collections.Generic.List<ProtoCore.BackpatchNode> table, int pc)`

CodeGen.Backpatch

**Parameters:**
- `table` (System.Collections.Generic.List<ProtoCore.BackpatchNode>)
- `pc` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void Backpatch(int bp, int pc)`

CodeGen.Backpatch

**Parameters:**
- `bp` (int)
- `pc` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `ProtoCore.DSASM.StackValue BuildOperand(ProtoCore.DSASM.SymbolNode symbol)`

CodeGen.BuildOperand

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void BuildRealDependencyForIdentList(ProtoCore.AssociativeGraph.GraphNode graphNode)`

CodeGen.BuildRealDependencyForIdentList

**Parameters:**
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void DFSGetSymbolList(ProtoCore.AST.Node pNode, ref ProtoCore.Type lefttype, ProtoCore.AssociativeGraph.UpdateNodeRef nodeRef)`

CodeGen.DFSGetSymbolList

**Parameters:**
- `pNode` (ProtoCore.AST.Node)
- `lefttype` (ref ProtoCore.Type)
- `nodeRef` (ProtoCore.AssociativeGraph.UpdateNodeRef)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void DFSGetSymbolList_Simple(ProtoCore.AST.Node pNode, ref ProtoCore.Type lefttype, ref int functionindex, ProtoCore.AssociativeGraph.UpdateNodeRef nodeRef)`

CodeGen.DFSGetSymbolList_Simple

**Parameters:**
- `pNode` (ProtoCore.AST.Node)
- `lefttype` (ref ProtoCore.Type)
- `functionindex` (ref int)
- `nodeRef` (ProtoCore.AssociativeGraph.UpdateNodeRef)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `int DfsEmitArrayIndexHeap(ProtoCore.AST.Node node, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.AST.Node parentNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.DfsEmitArrayIndexHeap

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `parentNode` (ProtoCore.AST.Node)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `bool DfsEmitIdentList(ProtoCore.AST.Node pNode, ProtoCore.AST.Node parentNode, int contextClassScope, ref ProtoCore.Type lefttype, ref int depth, ref ProtoCore.Type finalType, bool isLeftidentList, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass, ProtoCore.AST.Node binaryExpNode)`

CodeGen.DfsEmitIdentList

**Parameters:**
- `pNode` (ProtoCore.AST.Node)
- `parentNode` (ProtoCore.AST.Node)
- `contextClassScope` (int)
- `lefttype` (ref ProtoCore.Type)
- `depth` (ref int)
- `finalType` (ref ProtoCore.Type)
- `isLeftidentList` (bool)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)
- `binaryExpNode` (ProtoCore.AST.Node)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void DfsTraverse(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, bool isBooleanOp, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass, ProtoCore.AST.Node parentNode)`

CodeGen.DfsTraverse

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `isBooleanOp` (bool)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)
- `parentNode` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `int Emit(ProtoCore.AST.Node codeblocknode, ProtoCore.AssociativeGraph.GraphNode graphNode)`

CodeGen.Emit

**Parameters:**
- `codeblocknode` (ProtoCore.AST.Node)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitAtLevel(ProtoCore.AST.AssociativeAST.AtLevelNode atLevel)`

CodeGen.EmitAtLevel

**Parameters:**
- `atLevel` (ProtoCore.AST.AssociativeAST.AtLevelNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitBinary(ProtoCore.DSASM.OpCode opcode, int line, int col, int eline, int ecol)`

CodeGen.EmitBinary

**Parameters:**
- `opcode` (ProtoCore.DSASM.OpCode)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitBinaryOperation(ProtoCore.Type leftType, ProtoCore.Type rightType, ProtoCore.DSASM.Operator optr)`

CodeGen.EmitBinaryOperation

**Parameters:**
- `leftType` (ProtoCore.Type)
- `rightType` (ProtoCore.Type)
- `optr` (ProtoCore.DSASM.Operator)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitBooleanNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitBooleanNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitBounceIntrinsic(int blockId, int entry)`

CodeGen.EmitBounceIntrinsic

**Parameters:**
- `blockId` (int)
- `entry` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCJmp(int label, int line, int col, int eline, int ecol)`

CodeGen.EmitCJmp

**Parameters:**
- `label` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCall(int fi, int blockId, int type, int line, int col, int endLine, int endCol, int entrypoint)`

CodeGen.EmitCall

**Parameters:**
- `fi` (int)
- `blockId` (int)
- `type` (int)
- `line` (int)
- `col` (int)
- `endLine` (int)
- `endCol` (int)
- `entrypoint` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCallBaseCtor(int fi, int ci, int offset)`

CodeGen.EmitCallBaseCtor

**Parameters:**
- `fi` (int)
- `ci` (int)
- `offset` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCast(int UID, int rank)`

CodeGen.EmitCast

**Parameters:**
- `UID` (int)
- `rank` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCharNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, bool isBooleanOp, ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitCharNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `isBooleanOp` (bool)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCompileLog(string message)`

CodeGen.EmitCompileLog

**Parameters:**
- `message` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCompileLogFunctionEnd()`

CodeGen.EmitCompileLogFunctionEnd

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitCompileLogFunctionStart(string function)`

CodeGen.EmitCompileLogFunctionStart

**Parameters:**
- `function` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitDefaultArgNode(ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitDefaultArgNode

**Parameters:**
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitDoubleNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, bool isBooleanOp, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitDoubleNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `isBooleanOp` (bool)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitDynamicCall(int functionIndex, int type, int line, int col, int endline, int endcol, int entrypoint)`

CodeGen.EmitDynamicCall

**Parameters:**
- `functionIndex` (int)
- `type` (int)
- `line` (int)
- `col` (int)
- `endline` (int)
- `endcol` (int)
- `entrypoint` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitDynamicNode(ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitDynamicNode

**Parameters:**
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitExprListNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass, ProtoCore.AST.Node parentNode)`

CodeGen.EmitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)
- `parentNode` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitFunctionPointer(ProtoCore.DSASM.ProcedureNode procNode)`

CodeGen.EmitFunctionPointer

**Parameters:**
- `procNode` (ProtoCore.DSASM.ProcedureNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitGetterSetterForIdentList(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass, ref bool isCollapsed, ProtoCore.AST.Node setterArgument)`

CodeGen.EmitGetterSetterForIdentList

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)
- `isCollapsed` (ref bool)
- `setterArgument` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitIdentifierListNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, bool isBooleanOp, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass, ProtoCore.AST.Node bnode)`

CodeGen.EmitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `isBooleanOp` (bool)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)
- `bnode` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitInstrConsole(string instr)`

CodeGen.EmitInstrConsole

**Parameters:**
- `instr` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitInstrConsole(string instr, string op1)`

CodeGen.EmitInstrConsole

**Parameters:**
- `instr` (string)
- `op1` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitInstrConsole(string instr, string op1, string op2)`

CodeGen.EmitInstrConsole

**Parameters:**
- `instr` (string)
- `op1` (string)
- `op2` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitInstrConsole(string instr, string op1, string op2, string op3)`

CodeGen.EmitInstrConsole

**Parameters:**
- `instr` (string)
- `op1` (string)
- `op2` (string)
- `op3` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitIntNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, bool isBooleanOp, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitIntNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `isBooleanOp` (bool)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitJmp(int L1, int line, int col, int eline, int ecol)`

CodeGen.EmitJmp

**Parameters:**
- `L1` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitLoadElement(ProtoCore.DSASM.SymbolNode symbol, int blockId)`

CodeGen.EmitLoadElement

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)
- `blockId` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitNullNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, bool isBooleanOp, ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitNullNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `isBooleanOp` (bool)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitOpWithEmptyAtLevelAndGuides(bool emit, ProtoCore.DSASM.StackValue op, string value, ProtoCore.AST.Node node)`

CodeGen.EmitOpWithEmptyAtLevelAndGuides

**Parameters:**
- `emit` (bool)
- `op` (ProtoCore.DSASM.StackValue)
- `value` (string)
- `node` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPop(ProtoCore.DSASM.StackValue op, int classIndex, int blockId, int line, int col, int eline, int ecol)`

CodeGen.EmitPop

**Parameters:**
- `op` (ProtoCore.DSASM.StackValue)
- `classIndex` (int)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPopArray(int size)`

CodeGen.EmitPopArray

**Parameters:**
- `size` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPopForSymbol(ProtoCore.DSASM.SymbolNode symbol, int blockId, int line, int col, int eline, int ecol)`

CodeGen.EmitPopForSymbol

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPopForSymbolW(ProtoCore.DSASM.SymbolNode symbol, int blockId, int line, int col, int eline, int ecol)`

CodeGen.EmitPopForSymbolW

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPopReplicationGuides(int replicationGuide)`

CodeGen.EmitPopReplicationGuides

**Parameters:**
- `replicationGuide` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPopUpdateInstruction(ProtoCore.DSASM.StackValue op, int astID, int blockId, int line, int col, int eline, int ecol)`

CodeGen.EmitPopUpdateInstruction

**Parameters:**
- `op` (ProtoCore.DSASM.StackValue)
- `astID` (int)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPopm(ProtoCore.DSASM.StackValue op, int blockId, int line, int col, int endline, int endcol)`

CodeGen.EmitPopm

**Parameters:**
- `op` (ProtoCore.DSASM.StackValue)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `endline` (int)
- `endcol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPush(ProtoCore.DSASM.StackValue op, int blockId, int line, int col)`

CodeGen.EmitPush

**Parameters:**
- `op` (ProtoCore.DSASM.StackValue)
- `blockId` (int)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushBlockID(int blockID)`

CodeGen.EmitPushBlockID

**Parameters:**
- `blockID` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushDimensions(int dimensions)`

CodeGen.EmitPushDimensions

**Parameters:**
- `dimensions` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushForSymbol(ProtoCore.DSASM.SymbolNode symbol, int blockId, ProtoCore.AST.Node identNode)`

CodeGen.EmitPushForSymbol

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)
- `blockId` (int)
- `identNode` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushForSymbolW(ProtoCore.DSASM.SymbolNode symbol, int blockId, int line, int col)`

CodeGen.EmitPushForSymbolW

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)
- `blockId` (int)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushLevel(int level, bool isDominant)`

CodeGen.EmitPushLevel

**Parameters:**
- `level` (int)
- `isDominant` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushNull()`

CodeGen.EmitPushNull

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushReplicationGuide(int repGuide, bool isLongest)`

CodeGen.EmitPushReplicationGuide

**Parameters:**
- `repGuide` (int)
- `isLongest` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushUpdateInstruction(ProtoCore.DSASM.StackValue op, int astID, int blockID, int line, int col)`

CodeGen.EmitPushUpdateInstruction

**Parameters:**
- `op` (ProtoCore.DSASM.StackValue)
- `astID` (int)
- `blockID` (int)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitPushm(ProtoCore.DSASM.StackValue op, int classIndex, int blockId, int line, int col, int eline, int ecol)`

CodeGen.EmitPushm

**Parameters:**
- `op` (ProtoCore.DSASM.StackValue)
- `classIndex` (int)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitReplicationGuides(System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> replicationGuidesList)`

CodeGen.EmitReplicationGuides

**Parameters:**
- `replicationGuidesList` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitRetb(int line, int col, int endline, int endcol)`

CodeGen.EmitRetb

**Parameters:**
- `line` (int)
- `col` (int)
- `endline` (int)
- `endcol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitRetcn(int blockId, int line, int col, int endline, int endcol)`

CodeGen.EmitRetcn

**Parameters:**
- `blockId` (int)
- `line` (int)
- `col` (int)
- `endline` (int)
- `endcol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitReturn(int line, int col, int endline, int endcol)`

CodeGen.EmitReturn

**Parameters:**
- `line` (int)
- `col` (int)
- `endline` (int)
- `endcol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitReturnNull()`

CodeGen.EmitReturnNull

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitReturnNull(System.Nullable<System.Guid> guid)`

CodeGen.EmitReturnNull

**Parameters:**
- `guid` (System.Nullable<System.Guid>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitReturnStatement(ProtoCore.AST.Node node, ProtoCore.Type inferedType)`

CodeGen.EmitReturnStatement

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ProtoCore.Type)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitReturnToRegister(int line, int col, int endline, int endcol)`

CodeGen.EmitReturnToRegister

**Parameters:**
- `line` (int)
- `col` (int)
- `endline` (int)
- `endcol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitSetElement(ProtoCore.DSASM.SymbolNode symbol, int blockId, int line, int col, int eline, int ecol)`

CodeGen.EmitSetElement

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitSetMemElement(ProtoCore.DSASM.StackValue op, int blockId, int line, int col, int endline, int endcol)`

CodeGen.EmitSetMemElement

**Parameters:**
- `op` (ProtoCore.DSASM.StackValue)
- `blockId` (int)
- `line` (int)
- `col` (int)
- `endline` (int)
- `endcol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitStringNode(ProtoCore.AST.Node node, ref ProtoCore.Type inferedType, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitStringNode

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `inferedType` (ref ProtoCore.Type)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitThisPointerNode(ProtoCore.CompilerDefinitions.SubCompilePass subPass)`

CodeGen.EmitThisPointerNode

**Parameters:**
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void EmitUnary(ProtoCore.DSASM.OpCode opcode, int line, int col, int eline, int ecol)`

CodeGen.EmitUnary

**Parameters:**
- `opcode` (ProtoCore.DSASM.OpCode)
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void GenerateCallsiteIdentifierForGraphNode(ProtoCore.AssociativeGraph.GraphNode graphNode, string procName)`

Generates unique identifier for the callsite associated with the graphnode

**Parameters:**
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode) — 
- `procName` (string) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `string GetConstructBlockName(string construct)`

CodeGen.GetConstructBlockName

**Parameters:**
- `construct` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `ProtoCore.DSASM.DebugInfo GetDebugObject(int line, int col, int eline, int ecol, int nextStep_a, int nextStep_b)`

CodeGen.GetDebugObject

**Parameters:**
- `line` (int)
- `col` (int)
- `eline` (int)
- `ecol` (int)
- `nextStep_a` (int)
- `nextStep_b` (int)

**Returns:** `ProtoCore.DSASM.DebugInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `bool InsideFunction()`

CodeGen.InsideFunction

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `bool IsConstantExpression(ProtoCore.AST.Node node)`

CodeGen.IsConstantExpression

**Parameters:**
- `node` (ProtoCore.AST.Node)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `bool IsInLanguageBlockDefinedInFunction()`

CodeGen.IsInLanguageBlockDefinedInFunction

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `bool IsProperty(string name)`

CodeGen.IsProperty

**Parameters:**
- `name` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `ProtoCore.DSASM.AttributeEntry PopulateAttribute(object anode)`

CodeGen.PopulateAttribute

**Parameters:**
- `anode` (object)

**Returns:** `ProtoCore.DSASM.AttributeEntry` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `System.Collections.Generic.List<ProtoCore.DSASM.AttributeEntry> PopulateAttributes(object attributenodes)`

CodeGen.PopulateAttributes

**Parameters:**
- `attributenodes` (object)

**Returns:** `System.Collections.Generic.List<ProtoCore.DSASM.AttributeEntry>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void PushGraphNode(ProtoCore.AssociativeGraph.GraphNode graphnode)`

Append the graphnode to the instruction stream and procedure nodes

**Parameters:**
- `graphnode` (ProtoCore.AssociativeGraph.GraphNode) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void SetEntry()`

CodeGen.SetEntry

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void SetStackIndex(ProtoCore.DSASM.SymbolNode symbol)`

CodeGen.SetStackIndex

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `ProtoCore.DSASM.ProcedureNode TraverseFunctionCall(ProtoCore.AST.Node node, ProtoCore.AST.Node parentNode, int lefttype, int depth, ref ProtoCore.Type inferedType, ProtoCore.AssociativeGraph.GraphNode graphNode, ProtoCore.CompilerDefinitions.SubCompilePass subPass, ProtoCore.AST.Node bnode)`

CodeGen.TraverseFunctionCall

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `parentNode` (ProtoCore.AST.Node)
- `lefttype` (int)
- `depth` (int)
- `inferedType` (ref ProtoCore.Type)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `subPass` (ProtoCore.CompilerDefinitions.SubCompilePass)
- `bnode` (ProtoCore.AST.Node)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `bool VerifyAllocation(string name, int classScope, int functionScope, ref ProtoCore.DSASM.SymbolNode symbol, ref bool isAccessible)`

Verify the allocation of a variable in the current scope and parent scopes

**Parameters:**
- `name` (string) — 
- `classScope` (int) — 
- `functionScope` (int) — 
- `symbol` (ref ProtoCore.DSASM.SymbolNode) — 
- `isAccessible` (ref bool) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `bool VerifyAllocationInScope(string name, int classScope, int functionScope, ref ProtoCore.DSASM.SymbolNode symbol, ref bool isAccessible)`

Verifies the allocation of a variable in the given symbol table

**Parameters:**
- `name` (string) — 
- `classScope` (int) — 
- `functionScope` (int) — 
- `symbol` (ref ProtoCore.DSASM.SymbolNode) — 
- `isAccessible` (ref bool) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void setBlkId(int b)`

CodeGen.setBlkId

**Parameters:**
- `b` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

#### `void updatePcDictionary(int line, int col)`

CodeGen.updatePcDictionary

**Parameters:**
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGen.cs)

### Properties
- `codeBlock` (ProtoCore.DSASM.CodeBlock, get/set) — CodeGen.codeBlock
- `context` (ProtoCore.CompileTime.Context, get/set) — CodeGen.context

## CodeGenDS (class)

The code generator takes Abstract Syntax Tree and generates the DesignScript code

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGenDS.cs)

### Constructors
- `void CodeGenDS(System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> astList)` — This is used during ProtoAST generation to connect BinaryExpressionNode's generated from Block nodes to its child AST tree - pratapa

### Methods
#### `void EmitCode(string code)`

This function prints the DS code into the destination stream

**Parameters:**
- `code` (string) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGenDS.cs)

#### `string GenerateCode()`

CodeGenDS.GenerateCode

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeGenDS.cs)

### Properties
- `astNodeList` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>, get) — CodeGenDS.astNodeList

## Compiler (class)

Type Compiler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Compiler.cs)

### Constructors
- `void Compiler(ProtoCore.Core core)` — Compiler.Compiler

### Methods
#### `bool Compile(ref int blockId, ProtoCore.DSASM.CodeBlock parentBlock, ProtoCore.LanguageCodeBlock codeblock, ProtoCore.CompileTime.Context callContext, ProtoCore.DebugServices.EventSink sink, ProtoCore.AST.Node codeBlockNode, ProtoCore.AssociativeGraph.GraphNode graphNode)`

Compiler.Compile

**Parameters:**
- `blockId` (ref int)
- `parentBlock` (ProtoCore.DSASM.CodeBlock)
- `codeblock` (ProtoCore.LanguageCodeBlock)
- `callContext` (ProtoCore.CompileTime.Context)
- `sink` (ProtoCore.DebugServices.EventSink)
- `codeBlockNode` (ProtoCore.AST.Node)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Compiler.cs)

## ConsoleOutputStream (class)

Type ConsoleOutputStream

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Constructors
- `void ConsoleOutputStream()` — ConsoleOutputStream.ConsoleOutputStream

### Methods
#### `void Write(ProtoCore.OutputMessage message)`

ConsoleOutputStream.Write

**Parameters:**
- `message` (ProtoCore.OutputMessage)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

## Core (class)

Type Core

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

### Constructors
- `void Core(ProtoCore.Options options)` — Core.Core

### Methods
#### `void AddContextData(System.Collections.Generic.Dictionary<string, object> data)`



**Parameters:**
- `data` (System.Collections.Generic.Dictionary<string, object>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void AddDLLExtensionAppType(System.Type type)`

Core.AddDLLExtensionAppType

**Parameters:**
- `type` (System.Type)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void Cleanup()`

Core.Cleanup

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void GenerateExecutable()`

Core.GenerateExecutable

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void GenerateExprExe()`

Core.GenerateExprExe

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void GenerateExprExeInstructions(int blockScope)`

Core.GenerateExprExeInstructions

**Parameters:**
- `blockScope` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `string GenerateTempLangageVar()`

Core.GenerateTempLangageVar

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `string GenerateTempPropertyVar()`

Core.GenerateTempPropertyVar

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `string GenerateTempVar()`

Core.GenerateTempVar

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `ProtoCore.DSASM.SymbolNode GetFirstVisibleSymbol(string name, int classscope, int function, ProtoCore.DSASM.CodeBlock codeblock)`

Core.GetFirstVisibleSymbol

**Parameters:**
- `name` (string)
- `classscope` (int)
- `function` (int)
- `codeblock` (ProtoCore.DSASM.CodeBlock)

**Returns:** `ProtoCore.DSASM.SymbolNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `bool IsFunctionCodeBlock(ProtoCore.DSASM.CodeBlock cblock)`

Core.IsFunctionCodeBlock

**Parameters:**
- `cblock` (ProtoCore.DSASM.CodeBlock)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `bool IsTempVar(string varName)`

Core.IsTempVar

**Parameters:**
- `varName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void ResetForDeltaExecution()`

Reset the VM state for delta execution.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void ResetForPrecompilation()`

Core.ResetForPrecompilation

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void ResetSSASubscript(System.Guid guid, int subscript)`

Core.ResetSSASubscript

**Parameters:**
- `guid` (System.Guid)
- `subscript` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void SetFunctionInactive(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode functionDef)`

Sets the function to an inactive state where it can no longer be used by the front-end and backend

**Parameters:**
- `functionDef` (ProtoCore.AST.AssociativeAST.FunctionDefinitionNode) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

#### `void SetNewEntryPoint(int pc)`

Core.SetNewEntryPoint

**Parameters:**
- `pc` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

### Properties
- `AssocNode` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — Core.AssocNode
- `BaseOffset` (int, get/set) — Core.BaseOffset
- `BuildStatus` (ProtoCore.BuildStatus, get) — Core.BuildStatus
- `CallsiteGuidMap` (System.Collections.Generic.Dictionary<System.Guid, int>, get/set) — Core.CallsiteGuidMap
- `ClassIndex` (int, get/set) — Core.ClassIndex
- `ClassTable` (ProtoCore.DSASM.ClassTable, get/set) — Core.ClassTable
- `CodeBlockIndex` (int, get/set) — Core.CodeBlockIndex
- `CodeBlockList` (System.Collections.Generic.List<ProtoCore.DSASM.CodeBlock>, get/set) — Core.CodeBlockList
- `Compilers` (System.Collections.Generic.Dictionary<ProtoCore.Language, ProtoCore.Compiler>, get) — Core.Compilers
- `Configurations` (System.Collections.Generic.IDictionary<string, object>, get/set) — Core.Configurations
- `CurrentDSFileName` (string, get/set) — Core.CurrentDSFileName
- `DSExecutable` (ProtoCore.DSASM.Executable, get/set) — Core.DSExecutable
- `DllTypesToLoad` (System.Collections.Generic.List<System.Type>, get) — Core.DllTypesToLoad
- `DynamicFunctionTable` (ProtoCore.DSASM.DynamicFunctionTable, get/set) — Core.DynamicFunctionTable
- `DynamicVariableTable` (ProtoCore.DSASM.DynamicVariableTable, get/set) — Core.DynamicVariableTable
- `ExecutingGraphnode` (ProtoCore.AssociativeGraph.GraphNode, get/set) — Core.ExecutingGraphnode
- `ExecutionLog` (System.IO.TextWriter, get/set) — Core.ExecutionLog
- `ExprInterpreterExe` (ProtoCore.DSASM.Executable, get/set) — Core.ExprInterpreterExe
- `ExpressionUID` (int, get/set) — ExpressionUID is used as the unique id to identify an expression It is incremented by 1 after mapping its current value to an expression
- `ForLoopBlockIndex` (int, get/set) — ForLoopBlockIndex tracks the current number of new for loop blocks created at compile time for every new compile phase It is reset for delta compilation
- `FunctionPointerTable` (ProtoCore.DSASM.FunctionPointerTable, get/set) — Core.FunctionPointerTable
- `FunctionTable` (ProtoCore.Lang.FunctionTable, get) — Properties in under COMPILER_GENERATED_TO_RUNTIME_DATA, are generated at compile time, and passed to RuntimeData/Exe Only Core can initialize these
- `GlobHeapOffset` (int, get/set) — Core.GlobHeapOffset
- `GlobOffset` (int, get/set) — Core.GlobOffset
- `GraphNodeCallList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get/set) — Core.GraphNodeCallList
- `GraphNodeUID` (int, get/set) — Core.GraphNodeUID
- `Heap` (ProtoCore.DSASM.Heap, get) — Core.Heap
- `ImportHandler` (ProtoFFI.ImportModuleHandler, get/set) — Core.ImportHandler
- `ImportNodes` (ProtoCore.AST.AssociativeAST.CodeBlockNode, get/set) — Core.ImportNodes
- `InlineConditionalBodyGraphNodes` (System.Collections.Generic.Stack<System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>>, get/set) — Core.InlineConditionalBodyGraphNodes
- `IsParsingCodeBlockNode` (bool, get/set) — This is set to true when the temporary core is used for precompilation of CBN's.
- `IsParsingPreloadedAssembly` (bool, get/set) — This flag is set true when we call GraphUtilities.PreloadAssembly to load libraries.
- `Options` (ProtoCore.Options, get) — Core.Options
- `ParsingMode` (ProtoCore.ParseMode, get/set) — Core.ParsingMode
- `ProcNode` (ProtoCore.DSASM.ProcedureNode, get/set) — Core.ProcNode
- `ProcTable` (ProtoCore.DSASM.ProcedureTable, get/set) — Core.ProcTable
- `RuntimeTableIndex` (int, get/set) — Core.RuntimeTableIndex
- `SSAExprUID` (int, get/set) — Core.SSAExprUID
- `SSAExpressionUID` (int, get/set) — Core.SSAExpressionUID
- `SSASubscript` (int, get/set) — Core.SSASubscript
- `SSASubscript_GUID` (System.Guid, get/set) — Core.SSASubscript_GUID
- `TypeSystem` (ProtoCore.TypeSystem, get/set) — Core.TypeSystem
- `assocCodegen` (ProtoCore.CodeGen, get/set) — Core.assocCodegen
- `builtInsLoaded` (bool, get/set) — Core.builtInsLoaded
- `deltaCompileStartPC` (int, get/set) — Core.deltaCompileStartPC
- `internalAttributes` (ProtoCore.InternalAttributes, get/set) — Core.internalAttributes
- `newEntryPoint` (int, get) — Core.newEntryPoint
- `watchBaseOffset` (int, get/set) — Core.watchBaseOffset
- `watchFunctionScope` (int, get/set) — Core.watchFunctionScope
- `watchStartPC` (int, get/set) — Core.watchStartPC
- `watchSymbolList` (System.Collections.Generic.List<ProtoCore.DSASM.SymbolNode>, get/set) — Core.watchSymbolList

## DebugFrame (class)

Type DebugFrame

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void DebugFrame()` — DebugFrame.DebugFrame

### Properties
- `Arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — DebugFrame.Arguments
- `DotCallDimensions` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — DebugFrame.DotCallDimensions
- `ExecutingGraphNode` (ProtoCore.AssociativeGraph.GraphNode, get/set) — DebugFrame.ExecutingGraphNode
- `FepRun` (int, get/set) — DebugFrame.FepRun
- `FinalFepChosen` (ProtoCore.FunctionEndPoint, get/set) — DebugFrame.FinalFepChosen
- `FunctionStepOver` (bool, get/set) — DebugFrame.FunctionStepOver
- `HasDebugInfo` (bool, get/set) — DebugFrame.HasDebugInfo
- `IsBaseCall` (bool, get/set) — DebugFrame.IsBaseCall
- `IsDisposeCall` (bool, get/set) — DebugFrame.IsDisposeCall
- `IsDotCall` (bool, get/set) — DebugFrame.IsDotCall
- `IsExternalFunction` (bool, get/set) — DebugFrame.IsExternalFunction
- `IsInlineConditional` (bool, get/set) — DebugFrame.IsInlineConditional
- `IsMemberFunction` (bool, get/set) — DebugFrame.IsMemberFunction
- `IsReplicating` (bool, get/set) — DebugFrame.IsReplicating
- `IsResume` (bool, get/set) — DebugFrame.IsResume
- `ThisPtr` (System.Nullable<ProtoCore.DSASM.StackValue>, get/set) — DebugFrame.ThisPtr

## DebugProperties (class)

Type DebugProperties

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void DebugProperties()` — DebugProperties.DebugProperties

### Methods
#### `bool DebugStackFrameContains(ProtoCore.DebugProperties.StackFrameFlagOptions option)`

DebugProperties.DebugStackFrameContains

**Parameters:**
- `option` (ProtoCore.DebugProperties.StackFrameFlagOptions)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void RestoreCallrForNoBreak(ProtoCore.RuntimeCore runtimeCore, ProtoCore.DSASM.ProcedureNode fNode, bool isReplicating)`

Called only when we step over a function (including replicated and external functions) Pops Debug stackframe and Restores breakpoints

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore) — 
- `fNode` (ProtoCore.DSASM.ProcedureNode) — 
- `isReplicating` (bool) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void SetUpBounce(ProtoCore.DSASM.Executive exec, int exeblock, int returnAddr)`

DebugProperties.SetUpBounce

**Parameters:**
- `exec` (ProtoCore.DSASM.Executive)
- `exeblock` (int)
- `returnAddr` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void SetUpCallrForDebug(ProtoCore.RuntimeCore runtimeCore, ProtoCore.DSASM.Executive exec, ProtoCore.DSASM.ProcedureNode fNode, int pc, bool isBaseCall, ProtoCore.CallSite callsite, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments, System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>> replicationGuides, ProtoCore.DSASM.StackFrame stackFrame, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> dotCallDimensions, bool hasDebugInfo, bool isMember, System.Nullable<ProtoCore.DSASM.StackValue> thisPtr)`

DebugProperties.SetUpCallrForDebug

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)
- `exec` (ProtoCore.DSASM.Executive)
- `fNode` (ProtoCore.DSASM.ProcedureNode)
- `pc` (int)
- `isBaseCall` (bool)
- `callsite` (ProtoCore.CallSite)
- `arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationGuides` (System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>>)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `dotCallDimensions` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `hasDebugInfo` (bool)
- `isMember` (bool)
- `thisPtr` (System.Nullable<ProtoCore.DSASM.StackValue>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

#### `void SetUpStepOverFunctionCalls(ProtoCore.RuntimeCore runtimeCore, ProtoCore.DSASM.ProcedureNode fNode, ProtoCore.AssociativeGraph.GraphNode graphNode, bool hasDebugInfo)`

DebugProperties.SetUpStepOverFunctionCalls

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)
- `fNode` (ProtoCore.DSASM.ProcedureNode)
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)
- `hasDebugInfo` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

### Properties
- `ActiveBreakPoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>, get/set) — DebugProperties.ActiveBreakPoints
- `AllbreakPoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>, get/set) — DebugProperties.AllbreakPoints
- `CurrentBlockId` (int, get/set) — DebugProperties.CurrentBlockId
- `CurrentSymbolName` (string, get/set) — DebugProperties.CurrentSymbolName
- `DebugEntryPC` (int, get/set) — Returns the Program counter. This is only valid when the executive is suspended
- `DebugStackFrame` (System.Collections.Generic.Stack<ProtoCore.DebugFrame>, get/set) — DebugProperties.DebugStackFrame
- `FRStack` (System.Collections.Generic.Stack<bool>, get/set) — DebugProperties.FRStack
- `FirstStackFrame` (ProtoCore.DSASM.StackFrame, get/set) — DebugProperties.FirstStackFrame
- `IsPopmCall` (bool, get/set) — DebugProperties.IsPopmCall
- `ReturnPCFromDispose` (int, get/set) — DebugProperties.ReturnPCFromDispose
- `RunMode` (ProtoCore.Runmode, get/set) — DebugProperties.RunMode
- `StepOutReturnPC` (int, get/set) — DebugProperties.StepOutReturnPC
- `breakOptions` (ProtoCore.DebugProperties.BreakpointOptions, get/set) — DebugProperties.breakOptions
- `deferedGraphnodes` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get/set) — DebugProperties.deferedGraphnodes
- `executingGraphNode` (ProtoCore.AssociativeGraph.GraphNode, get/set) — DebugProperties.executingGraphNode
- `isResume` (bool, get/set) — DebugProperties.isResume

## ExecutionMode (enum)

Type ExecutionMode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

### Values
- `Parallel` = `0`
- `Serial` = `1`

## ExecutionStateEventArgs (class)

Type ExecutionStateEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

### Constructors
- `void ExecutionStateEventArgs(ProtoCore.ExecutionStateEventArgs.State state)` — ExecutionStateEventArgs.ExecutionStateEventArgs

### Properties
- `ExecutionState` (ProtoCore.ExecutionStateEventArgs.State, get) — ExecutionStateEventArgs.ExecutionState

## Executive (class)

Type Executive

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

### Constructors
- `void Executive(ProtoCore.RuntimeCore runtimeCore)` — Executive.Executive

### Methods
#### `ProtoCore.DSASM.StackValue Execute(int codeblock, int entry, bool fepRun, System.Collections.Generic.List<ProtoCore.DSASM.Instruction> breakpoints)`

Executive.Execute

**Parameters:**
- `codeblock` (int)
- `entry` (int)
- `fepRun` (bool)
- `breakpoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

### Properties
- `CurrentDSASMExec` (ProtoCore.DSASM.Executive, get/set) — Executive.CurrentDSASMExec

## FunctionEndPoint (class)

Type FunctionEndPoint

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

### Constructors
- `void FunctionEndPoint()` — FunctionEndPoint.FunctionEndPoint

### Methods
#### `System.Collections.Generic.List<ProtoCore.DSASM.StackValue> CoerceParameters(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, ProtoCore.RuntimeCore runtimeCore)`

Convert the parameters passed to the types specified in this fep

**Parameters:**
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.DSASM.StackValue>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

#### `int ComputeTypeDistance(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> args, ProtoCore.DSASM.ClassTable classTable, ProtoCore.RuntimeCore runtimeCore, bool allowArrayPromotion)`

Compute the number of type transforms needed to turn the current type into the target type Note that this method returns int[] -> char[] as an exact match

**Parameters:**
- `args` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `classTable` (ProtoCore.DSASM.ClassTable) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 
- `allowArrayPromotion` (bool) — 

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

#### `bool DoesPredicateMatch(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions)`

FunctionEndPoint.DoesPredicateMatch

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

#### `bool DoesTypeDeepMatch(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, ProtoCore.RuntimeCore runtimeCore)`

FunctionEndPoint.DoesTypeDeepMatch

**Parameters:**
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

#### `ProtoCore.DSASM.StackValue Execute(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore)`

FunctionEndPoint.Execute

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

#### `int GetConversionDistance(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> reducedParamSVs, ProtoCore.DSASM.ClassTable classTable, bool allowArrayPromotion, ProtoCore.RuntimeCore runtimeCore)`

FunctionEndPoint.GetConversionDistance

**Parameters:**
- `reducedParamSVs` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `classTable` (ProtoCore.DSASM.ClassTable)
- `allowArrayPromotion` (bool)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

#### `string ToString()`

FunctionEndPoint.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionEndPoint.cs)

### Properties
- `BlockScope` (int, get/set) — FunctionEndPoint.BlockScope
- `ClassOwnerIndex` (int, get/set) — FunctionEndPoint.ClassOwnerIndex
- `FormalParams` (ProtoCore.Type[], get/set) — FunctionEndPoint.FormalParams
- `procedureNode` (ProtoCore.DSASM.ProcedureNode, get/set) — FunctionEndPoint.procedureNode

## FunctionGroup (class)

A function group is a collection of overloads to the same method

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

### Constructors
- `void FunctionGroup()` — FunctionGroup.FunctionGroup
- `void FunctionGroup(System.Collections.Generic.List<ProtoCore.FunctionEndPoint> rhs)` — FunctionGroup.FunctionGroup

### Methods
#### `bool CanGetExactMatchStatics(ProtoCore.Runtime.Context context, System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.DSASM.StackValue>> reducedFormalParams, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore, ref System.Collections.Generic.HashSet<ProtoCore.FunctionEndPoint> lookup)`

For a given list of formal parameters, get the function end points that resolve

**Parameters:**
- `context` (ProtoCore.Runtime.Context) — 
- `reducedFormalParams` (System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.DSASM.StackValue>>) — 
- `stackFrame` (ProtoCore.DSASM.StackFrame) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 
- `lookup` (ref System.Collections.Generic.HashSet<ProtoCore.FunctionEndPoint>) — The number of argument sets that couldn't be resolved

**Returns:** `bool` — Returns true, if it can find a matching FEP for all the reduced params. Returns False otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `bool CheckInvalidArrayCoersion(ProtoCore.FunctionEndPoint fep, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> reducedSVs, ProtoCore.DSASM.ClassTable classTable, ProtoCore.RuntimeCore runtimeCore, bool allowArrayPromotion)`

FunctionGroup.CheckInvalidArrayCoersion

**Parameters:**
- `fep` (ProtoCore.FunctionEndPoint)
- `reducedSVs` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `classTable` (ProtoCore.DSASM.ClassTable)
- `runtimeCore` (ProtoCore.RuntimeCore)
- `allowArrayPromotion` (bool)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `void CopyPublic(System.Collections.Generic.List<ProtoCore.FunctionEndPoint> rhs)`

FunctionGroup.CopyPublic

**Parameters:**
- `rhs` (System.Collections.Generic.List<ProtoCore.FunctionEndPoint>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `void CopyVisible(System.Collections.Generic.List<ProtoCore.FunctionEndPoint> rhs)`

FunctionGroup.CopyVisible

**Parameters:**
- `rhs` (System.Collections.Generic.List<ProtoCore.FunctionEndPoint>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `System.Collections.Generic.Dictionary<ProtoCore.FunctionEndPoint, int> GetCastDistances(ProtoCore.Runtime.Context context, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParams, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions, ProtoCore.DSASM.ClassTable classTable, ProtoCore.RuntimeCore runtimeCore)`

FunctionGroup.GetCastDistances

**Parameters:**
- `context` (ProtoCore.Runtime.Context)
- `formalParams` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>)
- `classTable` (ProtoCore.DSASM.ClassTable)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `System.Collections.Generic.Dictionary<ProtoCore.FunctionEndPoint, int>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `System.Collections.Generic.Dictionary<ProtoCore.FunctionEndPoint, int> GetConversionDistances(ProtoCore.Runtime.Context context, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParams, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions, ProtoCore.DSASM.ClassTable classTable, ProtoCore.RuntimeCore runtimeCore, bool allowArrayPromotion)`

Returns a dictionary of the function end points that are type compatible with the costs of the associated conversions

**Parameters:**
- `context` (ProtoCore.Runtime.Context) — 
- `formalParams` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>) — 
- `classTable` (ProtoCore.DSASM.ClassTable) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 
- `allowArrayPromotion` (bool) — 

**Returns:** `System.Collections.Generic.Dictionary<ProtoCore.FunctionEndPoint, int>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `System.Collections.Generic.List<ProtoCore.FunctionEndPoint> GetExactTypeMatches(ProtoCore.Runtime.Context context, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParams, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore)`

Returns a list of all the function end points that are type compliant, there maybe more than one due to pattern matches

**Parameters:**
- `context` (ProtoCore.Runtime.Context)
- `formalParams` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `System.Collections.Generic.List<ProtoCore.FunctionEndPoint>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `System.Collections.Generic.Dictionary<ProtoCore.FunctionEndPoint, int> GetLooseConversionDistances(ProtoCore.Runtime.Context context, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParams, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions, ProtoCore.DSASM.ClassTable classTable, ProtoCore.RuntimeCore runtimeCore)`

Returns a dictionary of the function end points that are type compatible with any branch of replicated parameters.

**Parameters:**
- `context` (ProtoCore.Runtime.Context) — 
- `formalParams` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>) — 
- `classTable` (ProtoCore.DSASM.ClassTable) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.Dictionary<ProtoCore.FunctionEndPoint, int>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

#### `string ToString()`

FunctionGroup.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionGroup.cs)

### Properties
- `FunctionEndPoints` (System.Collections.Generic.List<ProtoCore.FunctionEndPoint>, get) — FunctionGroup.FunctionEndPoints

## IOutputStream (interface)

Type IOutputStream

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Methods
#### `void Write(ProtoCore.OutputMessage message)`

IOutputStream.Write

**Parameters:**
- `message` (ProtoCore.OutputMessage)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

## InlineConditional (struct)

Type InlineConditional

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

## InternalAttributes (class)

These are DS defined class attributes These attributes are used internally by the compiler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/InternalAttributes.cs)

### Constructors
- `void InternalAttributes(ProtoCore.DSASM.ClassTable classTable)` — InternalAttributes.InternalAttributes

### Properties
- `ClassTable` (ProtoCore.DSASM.ClassTable, get) — InternalAttributes.ClassTable

## InterpreterProperties (class)

Type InterpreterProperties

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

### Constructors
- `void InterpreterProperties()` — InterpreterProperties.InterpreterProperties
- `void InterpreterProperties(ProtoCore.InterpreterProperties rhs)` — InterpreterProperties.InterpreterProperties

### Methods
#### `void Reset()`

InterpreterProperties.Reset

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

### Properties
- `DominantStructure` (ProtoCore.Lang.Replication.DominantListStructure, get/set) — InterpreterProperties.DominantStructure
- `executingGraphNode` (ProtoCore.AssociativeGraph.GraphNode, get/set) — InterpreterProperties.executingGraphNode
- `functionCallArguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — InterpreterProperties.functionCallArguments
- `functionCallDotCallDimensions` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — InterpreterProperties.functionCallDotCallDimensions
- `nodeIterations` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get/set) — InterpreterProperties.nodeIterations

## Language (enum)

Type Language

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeBlock.cs)

### Values
- `Associative` = `0`
- `Imperative` = `1`
- `NotSpecified` = `-1`

## LanguageCodeBlock (class)

Type LanguageCodeBlock

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeBlock.cs)

### Constructors
- `void LanguageCodeBlock(ProtoCore.Language lang)` — LanguageCodeBlock.LanguageCodeBlock
- `void LanguageCodeBlock(ProtoCore.LanguageCodeBlock rhs)` — LanguageCodeBlock.LanguageCodeBlock

### Methods
#### `bool Equals(ProtoCore.LanguageCodeBlock other)`

Equality check for properties of a language block

**Parameters:**
- `other` (ProtoCore.LanguageCodeBlock) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeBlock.cs)

### Properties
- `Code` (string, get/set) — LanguageCodeBlock.Code
- `Language` (ProtoCore.Language, get/set) — LanguageCodeBlock.Language

## MigrationRewriter (class)

Type MigrationRewriter

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

### Methods
#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> MigrateMethodNames(System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> astNodes, System.Collections.Generic.IDictionary<string, string> priorNames, ProtoCore.MigrationRewriter.LogWarningHandler logWarningHandler)`

Migrates old method names to new names based on priorNameHints from LibraryServices

**Parameters:**
- `astNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>) — 
- `priorNames` (System.Collections.Generic.IDictionary<string, string>) — dictionary of old names vs. new names for node migration
- `logWarningHandler` (ProtoCore.MigrationRewriter.LogWarningHandler) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — migrated AST nodes after method renaming

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitFunctionCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode node)`

MigrationRewriter.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionCallNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitIdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode node)`

MigrationRewriter.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

## Options (class)

Type Options

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

### Constructors
- `void Options()` — Options.Options

### Properties
- `ApplyUpdate` (bool, get/set) — Options.ApplyUpdate
- `AssociativeToImperativePropagation` (bool, get/set) — Options.AssociativeToImperativePropagation
- `BuildOptErrorAsWarning` (bool, get/set) — Options.BuildOptErrorAsWarning
- `DirectDependencyExecution` (bool, get/set) — Options.DirectDependencyExecution
- `DumpByteCode` (bool, get/set) — Options.DumpByteCode
- `DumpFunctionResolverLogic` (bool, get/set) — Options.DumpFunctionResolverLogic
- `DumpIL` (bool, get) — Options.DumpIL
- `EmitBreakpoints` (bool, get/set) — Options.EmitBreakpoints
- `ExecuteSSA` (bool, get/set) — Options.ExecuteSSA
- `ExecutionMode` (ProtoCore.ExecutionMode, get/set) — Options.ExecutionMode
- `FormatToPrintFloatingPoints` (string, get/set) — Options.FormatToPrintFloatingPoints
- `GCTempVarsOnDebug` (bool, get/set) — Options.GCTempVarsOnDebug
- `GenerateSSA` (bool, get/set) — Options.GenerateSSA
- `IDEDebugMode` (bool, get/set) — Options.IDEDebugMode
- `IncludeDirectories` (System.Collections.Generic.List<string>, get/set) — Options.IncludeDirectories
- `IsDeltaCompile` (bool, get/set) — TODO: Aparajit: This flag is true for Delta AST compilation This will be removed once we make this the default and deprecate "deltaCompileStartPC" which requires recompiling the entire source code for every delta execution
- `IsDeltaExecution` (bool, get/set) — Options.IsDeltaExecution
- `RootModulePathName` (string, get/set) — Options.RootModulePathName
- `RunMode` (ProtoCore.DSASM.InterpreterMode, get/set) — Options.RunMode
- `SuppressBuildOutput` (bool, get/set) — Options.SuppressBuildOutput
- `SuppressFunctionResolutionWarning` (bool, get/set) — Options.SuppressFunctionResolutionWarning
- `Verbose` (bool, get/set) — Options.Verbose
- `WatchTestMode` (bool, get/set) — Options.WatchTestMode
- `dynamicCycleCheck` (bool, get/set) — Options.dynamicCycleCheck
- `staticCycleCheck` (bool, get/set) — Options.staticCycleCheck

## OutputMessage (class)

VM print out message

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Constructors
- `void OutputMessage(ProtoCore.OutputMessage.MessageType type, string message, string filePath, int line, int column)` — OutputMessage.OutputMessage
- `void OutputMessage(string message)` — OutputMessage.OutputMessage

### Properties
- `Column` (int, get) — OutputMessage.Column
- `Continue` (bool, get/set) — OutputMessage.Continue
- `FilePath` (string, get) — OutputMessage.FilePath
- `Line` (int, get) — OutputMessage.Line
- `Message` (string, get) — OutputMessage.Message
- `Type` (ProtoCore.OutputMessage.MessageType, get) — OutputMessage.Type

## ParseMode (enum)

Type ParseMode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

### Values
- `AllowNonAssignment` = `1`
- `None` = `2`
- `Normal` = `0`

## PrimitiveType (enum)

Type PrimitiveType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

### Values
- `Array` = `1`
- `Bool` = `4`
- `Char` = `5`
- `Double` = `2`
- `FunctionPointer` = `10`
- `Integer` = `3`
- `InvalidType` = `-1`
- `MaxPrimitive` = `12`
- `Null` = `0`
- `Pointer` = `9`
- `Return` = `11`
- `String` = `6`
- `Var` = `7`
- `Void` = `8`

## ReasonForExecutionSuspend (enum)

Type ReasonForExecutionSuspend

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

### Values
- `Breakpoint` = `1`
- `EndOfFile` = `4`
- `Exception` = `2`
- `NoEntryPoint` = `5`
- `PreStart` = `0`
- `VMSplit` = `6`
- `Warning` = `3`

## ReplicationGuide (class)

Represents a single replication guide entity that is associated with an argument to a function Given: a = f(i<1>, j<2L>) <1> and <2L> are each represented by a ReplicationGuide instance

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Core.cs)

### Constructors
- `void ReplicationGuide(int guide, bool longest)` — ReplicationGuide.ReplicationGuide

### Properties
- `GuideNumber` (int, get) — ReplicationGuide.GuideNumber
- `IsLongest` (bool, get) — ReplicationGuide.IsLongest

## Runmode (enum)

Type Runmode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DebuggerProperties.cs)

### Values
- `RunTo` = `0`
- `StepIn` = `2`
- `StepNext` = `1`
- `StepOut` = `3`

## RuntimeCore (class)

RuntimeCore is an object that is instantiated once across the lifecycle of the runtime This is the entry point of the runtime VM and its input is a DS Executable format. There will only be one instance of RuntimeCore regardless of how many times instances of a DSASM.Executive (runtime VM) is instantiated. Its properties will be persistent and accessible across all instances of a DSASM.Executive

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

### Constructors
- `void RuntimeCore(ProtoCore.DSASM.Heap heap, ProtoCore.Options options, ProtoCore.DSASM.Executable executable)` — RuntimeCore.RuntimeCore

### Methods
#### `void AddCallSiteGCRoot(System.Guid callSiteID, ProtoCore.DSASM.StackValue sv)`

RuntimeCore.AddCallSiteGCRoot

**Parameters:**
- `callSiteID` (System.Guid)
- `sv` (ProtoCore.DSASM.StackValue)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void Cleanup()`

RuntimeCore.Cleanup

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `int GetCurrentBlockId()`

RuntimeCore.GetCurrentBlockId

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `System.TimeSpan GetCurrentTime()`

RuntimeCore.GetCurrentTime

**Returns:** `System.TimeSpan` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void NotifyExecutionEvent(ProtoCore.ExecutionStateEventArgs.State state)`

RuntimeCore.NotifyExecutionEvent

**Parameters:**
- `state` (ProtoCore.ExecutionStateEventArgs.State)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void OnDispose()`

RuntimeCore.OnDispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void RecordExtecutedGraphNode(ProtoCore.AssociativeGraph.GraphNode graphNode)`

Record the GUID of executed graph node.

**Parameters:**
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void RegisterDllTypes(System.Collections.Generic.List<System.Type> dllTypes)`

Register imported dll types These types are initialzed from Importing dlls

**Parameters:**
- `dllTypes` (System.Collections.Generic.List<System.Type>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void RemoveCallSiteGCRoot(System.Guid callSiteID)`

RuntimeCore.RemoveCallSiteGCRoot

**Parameters:**
- `callSiteID` (System.Guid)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void RemoveExecutedAstGuids()`

Clear all recorded AST guids

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void ResetForDeltaExecution()`

RuntimeCore.ResetForDeltaExecution

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void SetProperties(ProtoCore.Options runtimeOptions, ProtoCore.DSASM.Executable executable, ProtoCore.DebugProperties debugProps, ProtoCore.Runtime.Context context, ProtoCore.DSASM.Executable exprInterpreterExe)`

RuntimeCore.SetProperties

**Parameters:**
- `runtimeOptions` (ProtoCore.Options)
- `executable` (ProtoCore.DSASM.Executable)
- `debugProps` (ProtoCore.DebugProperties)
- `context` (ProtoCore.Runtime.Context)
- `exprInterpreterExe` (ProtoCore.DSASM.Executable)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void SetStartPC(int pc)`

Sets a new entry point pc This can be overrided by another call to SetStartPC

**Parameters:**
- `pc` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void SetupForExecution(ProtoCore.Core compileCore, int globalStackFrameSize)`

Setup before execution This function needs to be called before attempting to execute the RuntimeCore It will initialize the runtime execution data and configuration

**Parameters:**
- `compileCore` (ProtoCore.Core) — 
- `globalStackFrameSize` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void SetupStartPC()`

This function determines what the starting pc should be for the next execution session The StartPC takes precedence if set. Otherwise, the entry pc in the global codeblock is the entry point StartPC is assumed to be reset to kInvalidPC after each execution session

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `void StartTimer()`

RuntimeCore.StartTimer

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

### Properties
- `Breakpoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>, get/set) — RuntimeCore.Breakpoints
- `CallSiteGCRoots` (System.Collections.Generic.IEnumerable<ProtoCore.DSASM.StackValue>, get) — RuntimeCore.CallSiteGCRoots
- `CancellationPending` (bool, get) — RuntimeCore.CancellationPending
- `Context` (ProtoCore.Runtime.Context, get/set) — RuntimeCore.Context
- `ContinuationStruct` (ProtoCore.Lang.ContinuationStructure, get/set) — RuntimeCore.ContinuationStruct
- `CurrentExecutive` (ProtoCore.Executive, get) — RuntimeCore.CurrentExecutive
- `DSExecutable` (ProtoCore.DSASM.Executable, get) — RuntimeCore.DSExecutable
- `DebugProps` (ProtoCore.DebugProperties, get/set) — RuntimeCore.DebugProps
- `ExecutedAstGuids` (System.Collections.Generic.IEnumerable<System.Guid>, get) — RuntimeCore.ExecutedAstGuids
- `ExecutionInstance` (ProtoCore.Executive, get) — RuntimeCore.ExecutionInstance
- `ExecutionState` (int, get/set) — RuntimeCore.ExecutionState
- `ExecutiveProvider` (ProtoCore.DSASM.IExecutiveProvider, get/set) — RuntimeCore.ExecutiveProvider
- `ExprInterpreterExe` (ProtoCore.DSASM.Executable, get) — RuntimeCore.ExprInterpreterExe
- `FunctionCallDepth` (int, get/set) — RuntimeCore.FunctionCallDepth
- `Heap` (ProtoCore.DSASM.Heap, get/set) — RuntimeCore.Heap
- `InterpreterProps` (System.Collections.Generic.Stack<ProtoCore.InterpreterProperties>, get/set) — RuntimeCore.InterpreterProps
- `Mirror` (ProtoCore.DSASM.Mirror.ExecutionMirror, get/set) — RuntimeCore.Mirror
- `Options` (ProtoCore.Options, get) — RuntimeCore.Options
- `ReasonForExecutionSuspend` (ProtoCore.ReasonForExecutionSuspend, get) — Returns the reason why the execution was last suspended
- `RunningBlock` (int, get/set) — The currently executing blockID
- `RuntimeData` (ProtoCore.RuntimeData, get/set) — RuntimeCore.RuntimeData
- `RuntimeMemory` (ProtoCore.Runtime.RuntimeMemory, get/set) — RuntimeCore.RuntimeMemory
- `RuntimeStatus` (ProtoCore.RuntimeStatus, get/set) — RuntimeCore.RuntimeStatus
- `StartPC` (int, get) — RuntimeCore.StartPC
- `WatchSymbolList` (System.Collections.Generic.List<ProtoCore.DSASM.SymbolNode>, get/set) — RuntimeCore.WatchSymbolList
- `watchClassScope` (int, get/set) — RuntimeCore.watchClassScope
- `watchFramePointer` (int, get/set) — RuntimeCore.watchFramePointer
- `watchStack` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — RuntimeCore.watchStack

### Events
#### `Dispose` (`ProtoCore.RuntimeCore.DisposeDelegate`)

**Signature:** `public event ProtoCore.RuntimeCore.DisposeDelegate Dispose`

RuntimeCore.Dispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

#### `ExecutionEvent` (`System.EventHandler<ProtoCore.ExecutionStateEventArgs>`)

**Signature:** `public event System.EventHandler<ProtoCore.ExecutionStateEventArgs> ExecutionEvent`

RuntimeCore.ExecutionEvent

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeCore.cs)

## RuntimeData (class)

The RuntimeData is an object that contains properties that is consumed only by the runtime VM It is instantiated prior to execution and is populated with information gathered from the CompileCore The runtime VM is designed to run independently from the front-end (UI, compiler) and the only 2 properties it needs are the RuntimeData and the DSExecutable. The RuntimeData will also contain properties that are populated at runtime and consumed at runtime.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeData.cs)

### Constructors
- `void RuntimeData()` — RuntimeData.RuntimeData

### Methods
#### `ProtoCore.CallSite GetCallSite(int classScope, string methodName, ProtoCore.DSASM.Executable executable, ProtoCore.RuntimeCore runtimeCore)`

Retrieves an existing instance of a callsite associated with a UID It creates a new callsite if non was found

**Parameters:**
- `classScope` (int)
- `methodName` (string)
- `executable` (ProtoCore.DSASM.Executable)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.CallSite` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeData.cs)

#### `System.Collections.Generic.Dictionary<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite>> GetCallsitesForNodes(System.Collections.Generic.IEnumerable<System.Guid> nodeGuids, ProtoCore.DSASM.Executable executable)`

This API is used by host integrations such as for Revit and C3D. It is used to get the trace data list for all nodes binding to elements in the host.

**Parameters:**
- `nodeGuids` (System.Collections.Generic.IEnumerable<System.Guid>) — 
- `executable` (ProtoCore.DSASM.Executable) — 

**Returns:** `System.Collections.Generic.Dictionary<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeData.cs)

#### `System.Collections.Generic.IDictionary<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite.RawTraceData>> GetTraceDataForNodes(System.Collections.Generic.IEnumerable<System.Guid> nodeGuids, ProtoCore.DSASM.Executable executable)`

Call this method to obtain serialized trace data for a list of nodes.

**Parameters:**
- `nodeGuids` (System.Collections.Generic.IEnumerable<System.Guid>) — A list of System.Guid of nodes whose serialized trace data is to be retrieved. This parameter cannot be null.
- `executable` (ProtoCore.DSASM.Executable) — A container of callsite data for the nodes in the graph.

**Returns:** `System.Collections.Generic.IDictionary<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite.RawTraceData>>` — Returns a dictionary that maps each node Guid to its corresponding list of serialized callsite trace data.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeData.cs)

#### `void SetTraceDataForNodes(System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite.RawTraceData>>> nodeDataPairs)`

Call this method to set the list of serialized trace data, possibly loaded from an external storage.

**Parameters:**
- `nodeDataPairs` (System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<System.Guid, System.Collections.Generic.List<ProtoCore.CallSite.RawTraceData>>>) — A Dictionary that matches a node Guid to its corresponding list of serialized callsite trace data.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeData.cs)

### Properties
- `CallSiteToNodeMap` (System.Collections.Generic.Dictionary<System.Guid, System.Guid>, get) — Map from a callsite's guid to a graph UI node.
- `CallsiteCache` (System.Collections.Generic.IDictionary<string, ProtoCore.CallSite>, get/set) — Map from callsite id to callsite.

## RuntimeStatus (class)

Type RuntimeStatus

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

### Constructors
- `void RuntimeStatus(ProtoCore.RuntimeCore runtimeCore, bool warningAsError, System.IO.TextWriter writer)` — RuntimeStatus.RuntimeStatus

### Methods
#### `void ClearWarningForExpression(int expressionID)`

RuntimeStatus.ClearWarningForExpression

**Parameters:**
- `expressionID` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

#### `void ClearWarningsForAst(int astID)`

RuntimeStatus.ClearWarningsForAst

**Parameters:**
- `astID` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

#### `void ClearWarningsForGraph(System.Guid guid)`

RuntimeStatus.ClearWarningsForGraph

**Parameters:**
- `guid` (System.Guid)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

#### `void LogFunctionGroupNotFoundWarning(string methodName, int classScope, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments)`

Report that the method cannot be found.

**Parameters:**
- `methodName` (string) — The method that cannot be found
- `classScope` (int) — The class scope of object
- `arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — Arguments

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

#### `void LogMethodNotAccessibleWarning(string methodName)`

RuntimeStatus.LogMethodNotAccessibleWarning

**Parameters:**
- `methodName` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

#### `void LogMethodResolutionWarning(ProtoCore.FunctionGroup funcGroup, string methodName, int classScope, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments)`

RuntimeStatus.LogMethodResolutionWarning

**Parameters:**
- `funcGroup` (ProtoCore.FunctionGroup)
- `methodName` (string)
- `classScope` (int)
- `arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

#### `void LogWarning(ProtoCore.Runtime.WarningID ID, string message)`

RuntimeStatus.LogWarning

**Parameters:**
- `ID` (ProtoCore.Runtime.WarningID)
- `message` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

#### `void LogWarning(ProtoCore.Runtime.WarningID ID, string message, string filename, int line, int col)`

RuntimeStatus.LogWarning

**Parameters:**
- `ID` (ProtoCore.Runtime.WarningID)
- `message` (string)
- `filename` (string)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

### Properties
- `MessageHandler` (ProtoCore.IOutputStream, get/set) — RuntimeStatus.MessageHandler
- `WarningCount` (int, get) — RuntimeStatus.WarningCount
- `Warnings` (System.Collections.Generic.List<ProtoCore.Runtime.WarningEntry>, get) — RuntimeStatus.Warnings
- `WebMessageHandler` (ProtoCore.IOutputStream, get/set) — RuntimeStatus.WebMessageHandler

## TextOutputStream (class)

Type TextOutputStream

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Constructors
- `void TextOutputStream(System.Collections.Generic.Dictionary<int, System.Collections.Generic.List<string>> map)` — TextOutputStream.TextOutputStream

### Methods
#### `void Write(ProtoCore.OutputMessage message)`

TextOutputStream.Write

**Parameters:**
- `message` (ProtoCore.OutputMessage)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Properties
- `Map` (System.Collections.Generic.Dictionary<int, System.Collections.Generic.List<string>>, get) — TextOutputStream.Map
- `TextStream` (System.IO.StringWriter, get) — TextOutputStream.TextStream

## Type (struct)

Type Type

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

### Constructors
- `void Type(string TypeName, int TypeRank)` — Constructor of Type using Short string

### Methods
#### `bool Equals(ProtoCore.Type type)`

Type.Equals

**Parameters:**
- `type` (ProtoCore.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `void Initialize()`

Comment Jun: Initialize a type to the default values

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `string ToShortString()`

To its string representation, but using unqualified class class name.

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `string ToString()`

Type.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

### Properties
- `IsIndexable` (bool, get) — Type.IsIndexable

## TypeSystem (class)

Type TypeSystem

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

### Constructors
- `void TypeSystem()` — TypeSystem.TypeSystem

### Methods
#### `void BuildAddressTypeMap()`

TypeSystem.BuildAddressTypeMap

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `ProtoCore.Type BuildPrimitiveTypeObject(ProtoCore.PrimitiveType pType, int rank)`

TypeSystem.BuildPrimitiveTypeObject

**Parameters:**
- `pType` (ProtoCore.PrimitiveType)
- `rank` (int)

**Returns:** `ProtoCore.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `ProtoCore.Type BuildTypeObject(int UID, int rank)`

TypeSystem.BuildTypeObject

**Parameters:**
- `UID` (int)
- `rank` (int)

**Returns:** `ProtoCore.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `ProtoCore.DSASM.StackValue ClassCoerece(ProtoCore.DSASM.StackValue sv, ProtoCore.Type targetType, ProtoCore.RuntimeCore runtimeCore)`

TypeSystem.ClassCoerece

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue)
- `targetType` (ProtoCore.Type)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `ProtoCore.DSASM.StackValue Coerce(ProtoCore.DSASM.StackValue sv, ProtoCore.Type targetType, ProtoCore.RuntimeCore runtimeCore)`

TypeSystem.Coerce

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue)
- `targetType` (ProtoCore.Type)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `ProtoCore.DSASM.StackValue Coerce(ProtoCore.DSASM.StackValue sv, int UID, int rank, ProtoCore.RuntimeCore runtimeCore)`

TypeSystem.Coerce

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue)
- `UID` (int)
- `rank` (int)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `string GetPrimitTypeName(ProtoCore.PrimitiveType type)`

TypeSystem.GetPrimitTypeName

**Parameters:**
- `type` (ProtoCore.PrimitiveType)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `int GetType(ProtoCore.DSASM.StackValue sv)`

TypeSystem.GetType

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `string GetType(int UID)`

TypeSystem.GetType

**Parameters:**
- `UID` (int)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `int GetType(string ident)`

TypeSystem.GetType

**Parameters:**
- `ident` (string)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `bool IsHigherRank(int t1, int t2)`

TypeSystem.IsHigherRank

**Parameters:**
- `t1` (int)
- `t2` (int)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `void SetClassTable(ProtoCore.DSASM.ClassTable table)`

TypeSystem.SetClassTable

**Parameters:**
- `table` (ProtoCore.DSASM.ClassTable)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

#### `void SetTypeSystem()`

TypeSystem.SetTypeSystem

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Type.cs)

### Properties
- `addressTypeClassMap` (System.Collections.Generic.Dictionary<ProtoCore.DSASM.AddressType, int>, get/set) — TypeSystem.addressTypeClassMap
- `classTable` (ProtoCore.DSASM.ClassTable, get) — TypeSystem.classTable

