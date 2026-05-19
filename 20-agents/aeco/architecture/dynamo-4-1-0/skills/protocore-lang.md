---
name: dynamo-protocore-lang
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.Lang namespace — 9 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuiltInFunctionEndPoint, BuiltInMethods, ContinuationStructure, FFIActivationRecord, FFIFunctionEndPoint, FunctionTable, JILActivationRecord, JILFunctionEndPoint, and 1 more types.
---

# ProtoCore.Lang

Auto-generated from vendor docs for dynamo 4.1.0. 9 types in this namespace.

## BuiltInFunctionEndPoint (class)

Type BuiltInFunctionEndPoint

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/BuiltInFunctionEndPoint.cs)

### Constructors
- `void BuiltInFunctionEndPoint(ProtoCore.Lang.BuiltInMethods.MethodID id)` — BuiltInFunctionEndPoint.BuiltInFunctionEndPoint

### Methods
#### `bool DoesPredicateMatch(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions)`

BuiltInFunctionEndPoint.DoesPredicateMatch

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/BuiltInFunctionEndPoint.cs)

#### `ProtoCore.DSASM.StackValue Execute(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore)`

BuiltInFunctionEndPoint.Execute

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/BuiltInFunctionEndPoint.cs)

## BuiltInMethods (class)

Type BuiltInMethods

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/BuiltInMethods.cs)

### Constructors
- `void BuiltInMethods(ProtoCore.Core core)` — BuiltInMethods.BuiltInMethods

### Methods
#### `string GetMethodName(ProtoCore.Lang.BuiltInMethods.MethodID id)`

BuiltInMethods.GetMethodName

**Parameters:**
- `id` (ProtoCore.Lang.BuiltInMethods.MethodID)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/BuiltInMethods.cs)

### Properties
- `Methods` (System.Collections.Generic.List<ProtoCore.Lang.BuiltInMethods.BuiltInMethod>, get/set) — BuiltInMethods.Methods

## ContinuationStructure (class)

Continuation structure holds the data for a partial execution of a replicated callsite

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/ContinuationStructure.cs)

### Constructors
- `void ContinuationStructure()` — ContinuationStructure.ContinuationStructure

### Properties
- `Done` (bool, get/set) — True iff execution is complete
- `InitialArguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — ContinuationStructure.InitialArguments
- `InitialDepth` (int, get/set) — The depth of the replicating member function used in determining thisPtr
- `InitialDotCallDimensions` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — These are cached at the first replicating call so that they can be used in GC when the replication is Done
- `InitialPC` (int, get/set) — The instruction point of the replicating CALLR instruction needs to be cached in order to return to the next instruction at end of replication
- `IsFirstCall` (bool, get/set) — This flag indicates whether the current FEP call is a continuation of a replicating call
- `NextDispatchArgs` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — This is the resolved list of arguments that should be used to execute the function end point
- `Result` (ProtoCore.DSASM.StackValue, get/set) — This represents the result. If execution of the callsite is complete this is the retun value, otherwise it is a partial value
- `RunningResult` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get/set) — 

## FFIActivationRecord (class)

Type FFIActivationRecord

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FFIFunctionEndPoint.cs)

### Constructors
- `void FFIActivationRecord()` — FFIActivationRecord.FFIActivationRecord

## FFIFunctionEndPoint (class)

Type FFIFunctionEndPoint

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FFIFunctionEndPoint.cs)

### Constructors
- `void FFIFunctionEndPoint()` — FFIFunctionEndPoint.FFIFunctionEndPoint
- `void FFIFunctionEndPoint(ProtoCore.Lang.FFIActivationRecord record)` — FFIFunctionEndPoint.FFIFunctionEndPoint

### Methods
#### `bool DoesPredicateMatch(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions)`

FFIFunctionEndPoint.DoesPredicateMatch

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FFIFunctionEndPoint.cs)

#### `ProtoCore.DSASM.StackValue Execute(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore)`

FFIFunctionEndPoint.Execute

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FFIFunctionEndPoint.cs)

## FunctionTable (class)

Type FunctionTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionTable.cs)

### Constructors
- `void FunctionTable()` — FunctionTable.FunctionTable

### Methods
#### `void InitGlobalFunctionEntry(int classIndexAtCallsite)`

Initialize the global function table entry for a class The argument is the index of the class of functions to initialize + 1, which is the index expected at callsite

**Parameters:**
- `classIndexAtCallsite` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/FunctionTable.cs)

### Properties
- `FunctionList` (System.Collections.Generic.Dictionary<string, ProtoCore.FunctionGroup>, get) — FunctionTable.FunctionList
- `GlobalFuncTable` (System.Collections.Generic.Dictionary<int, System.Collections.Generic.Dictionary<string, ProtoCore.FunctionGroup>>, get) — FunctionTable.GlobalFuncTable

## JILActivationRecord (class)

Type JILActivationRecord

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/JILFunctionEndPoint.cs)

### Constructors
- `void JILActivationRecord()` — JILActivationRecord.JILActivationRecord

## JILFunctionEndPoint (class)

Type JILFunctionEndPoint

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/JILFunctionEndPoint.cs)

### Constructors
- `void JILFunctionEndPoint()` — JILFunctionEndPoint.JILFunctionEndPoint
- `void JILFunctionEndPoint(ProtoCore.Lang.JILActivationRecord activation)` — JILFunctionEndPoint.JILFunctionEndPoint

### Methods
#### `bool DoesPredicateMatch(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions)`

JILFunctionEndPoint.DoesPredicateMatch

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/JILFunctionEndPoint.cs)

#### `ProtoCore.DSASM.StackValue Execute(ProtoCore.Runtime.Context c, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParameters, ProtoCore.DSASM.StackFrame stackFrame, ProtoCore.RuntimeCore runtimeCore)`

JILFunctionEndPoint.Execute

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `formalParameters` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `stackFrame` (ProtoCore.DSASM.StackFrame)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/JILFunctionEndPoint.cs)

## Obj (class)

Type Obj

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Obj.cs)

### Constructors
- `void Obj()` — Obj.Obj
- `void Obj(ProtoCore.DSASM.StackValue dsasmValue)` — Obj.Obj

### Methods
#### `string ToString()`

Obj.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/Obj.cs)

### Properties
- `DsasmValue` (ProtoCore.DSASM.StackValue, get) — This represents a tieback value into the DSASM execution engine

