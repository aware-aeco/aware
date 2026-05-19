---
name: dynamo-protocore-runtime
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.Runtime namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Context, RuntimeMemory, WarningID, WarningEntry, InfoEntry.
---

# ProtoCore.Runtime

Auto-generated from vendor docs for dynamo 4.1.0. 5 types in this namespace.

## Context (class)

Type Context

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Context.cs)

### Constructors
- `void Context()` — Context.Context

### Properties
- `IsImplicitCall` (bool, get/set) — Context.IsImplicitCall
- `IsReplicating` (bool, get/set) — Context.IsReplicating

## InfoEntry (struct)

Type InfoEntry

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeStatus.cs)

## RuntimeMemory (class)

Type RuntimeMemory

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

### Constructors
- `void RuntimeMemory(ProtoCore.DSASM.Heap heap)` — RuntimeMemory.RuntimeMemory

### Methods
#### `void AlignStackForExprInterpreter()`

RuntimeMemory.AlignStackForExprInterpreter

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `ProtoCore.DSASM.StackValue GetAtRelative(int relativeOffset)`

RuntimeMemory.GetAtRelative

**Parameters:**
- `relativeOffset` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `ProtoCore.DSASM.StackValue GetMemberData(int symbolindex, int scope, ProtoCore.DSASM.Executable exe)`

RuntimeMemory.GetMemberData

**Parameters:**
- `symbolindex` (int)
- `scope` (int)
- `exe` (ProtoCore.DSASM.Executable)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `int GetRelative(int index)`

RuntimeMemory.GetRelative

**Parameters:**
- `index` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `int GetStackIndex(int offset)`

Returns stack index of symbol for specified frame.

**Parameters:**
- `offset` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `ProtoCore.DSASM.StackValue GetSymbolValue(ProtoCore.DSASM.SymbolNode symbol)`

Returns the value of symbol on current stack frame.

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `ProtoCore.DSASM.StackValue GetSymbolValueOnFrame(ProtoCore.DSASM.SymbolNode symbol, int framePointer)`

Returns the value of symbol on specified frame.

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode) — 
- `framePointer` (int) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `ProtoCore.DSASM.StackValue Pop()`

RuntimeMemory.Pop

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void PopConstructBlockId()`

RuntimeMemory.PopConstructBlockId

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void PopFrame(int size)`

Remove the specified number of items from the stack.

**Parameters:**
- `size` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void Push(ProtoCore.DSASM.StackValue data)`

RuntimeMemory.Push

**Parameters:**
- `data` (ProtoCore.DSASM.StackValue)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void PushConstructBlockId(int id)`

Push the block ID of the block that will be executed

**Parameters:**
- `id` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void PushFrameForGlobals(int size)`

Reserve stack for global variables.

**Parameters:**
- `size` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void PushFrameForLocals(int size)`

Reserve specified number of stack slots for local variables and initialize them to Null.

**Parameters:**
- `size` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void PushStackFrame(ProtoCore.DSASM.StackFrame stackFrame)`

RuntimeMemory.PushStackFrame

**Parameters:**
- `stackFrame` (ProtoCore.DSASM.StackFrame)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void RestoreStackForExprInterpreter()`

RuntimeMemory.RestoreStackForExprInterpreter

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void SetAtRelative(int offset, ProtoCore.DSASM.StackValue data)`

RuntimeMemory.SetAtRelative

**Parameters:**
- `offset` (int)
- `data` (ProtoCore.DSASM.StackValue)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `void SetSymbolValue(ProtoCore.DSASM.SymbolNode symbol, ProtoCore.DSASM.StackValue data)`

Set the value for symbol on current stack frame.

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode) — 
- `data` (ProtoCore.DSASM.StackValue) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

#### `bool ValidateStackFrame()`

RuntimeMemory.ValidateStackFrame

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/RuntimeMemory.cs)

### Properties
- `ConstructBlockIds` (System.Collections.Generic.List<int>, get) — RuntimeMemory.ConstructBlockIds
- `CurrentConstructBlockId` (int, get) — RuntimeMemory.CurrentConstructBlockId
- `CurrentStackFrame` (ProtoCore.DSASM.StackFrame, get) — Current stack frame.
- `CurrentStackFrameClassScope` (int, get) — Current stack frame ClassScope.
- `CurrentStackFrameFunctionBlock` (int, get) — Current stack frame FunctionScope.
- `CurrentStackFrameFunctionScope` (int, get) — Current stack frame FunctionScope.
- `CurrentStackFrameThisPtr` (ProtoCore.DSASM.StackValue, get) — Current stack frame ClassScope.
- `FramePointer` (int, get/set) — RuntimeMemory.FramePointer
- `GlobOffset` (int, get) — RuntimeMemory.GlobOffset
- `Heap` (ProtoCore.DSASM.Heap, get) — RuntimeMemory.Heap
- `IsCurrentStackFrameNull` (bool, get) — RuntimeMemory.IsCurrentStackFrameNull
- `Stack` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get) — RuntimeMemory.Stack

## WarningEntry (struct)

Type WarningEntry

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

## WarningID (enum)

Type WarningID

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Values
- `AccessViolation` = `1`
- `AmbiguousMethodDispatch` = `2`
- `AurgumentIsNotExpected` = `3`
- `CallingConstructorOnInstance` = `4`
- `ConversionNotPossible` = `5`
- `CyclicDependency` = `11`
- `Default` = `0`
- `DereferencingNonPointer` = `6`
- `FileNotExist` = `7`
- `IndexOutOfRange` = `8`
- `IntegerOverflow` = `24`
- `InvalidArguments` = `10`
- `InvalidArrayIndexType` = `23`
- `InvalidIndexing` = `17`
- `InvalidRecursion` = `9`
- `InvalidType` = `19`
- `MethodResolutionFailure` = `12`
- `ModuloByZero` = `18`
- `MoreThanOneDominantList` = `21`
- `OverIndexing` = `13`
- `RangeExpressionOutOfMemory` = `20`
- `ReplicationWarning` = `16`
- `RunOutOfMemory` = `22`
- `TypeConvertionCauseInfoLoss` = `14`
- `TypeMismatch` = `15`

