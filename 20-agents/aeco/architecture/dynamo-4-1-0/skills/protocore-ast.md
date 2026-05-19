---
name: dynamo-protocore-ast
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.AST namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Node, DictionaryExpressionBuilderBase<T>, NodeEnumerableExtensions.
---

# ProtoCore.AST

Auto-generated from vendor docs for dynamo 4.1.0. 3 types in this namespace.

## DictionaryExpressionBuilderBase<T> (class)

Dictionary syntax like { "foo" : 12 } isn't part of the AST. Instead, this helper class exists to build a function call to build a Dictionary via zero touch.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

### Constructors
- `void DictionaryExpressionBuilderBase()` — DictionaryExpressionBuilderBase.DictionaryExpressionBuilderBase

### Methods
#### `void AddKey(T value)`

DictionaryExpressionBuilderBase.AddKey

**Parameters:**
- `value` (T)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

#### `void AddValue(T value)`

DictionaryExpressionBuilderBase.AddValue

**Parameters:**
- `value` (T)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

#### `void SetNodeEndLocation(ProtoCore.DesignScriptParser.Token token)`

DictionaryExpressionBuilderBase.SetNodeEndLocation

**Parameters:**
- `token` (ProtoCore.DesignScriptParser.Token)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

#### `void SetNodeStartLocation(ProtoCore.DesignScriptParser.Token token)`

DictionaryExpressionBuilderBase.SetNodeStartLocation

**Parameters:**
- `token` (ProtoCore.DesignScriptParser.Token)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

#### `T ToFunctionCall()`

DictionaryExpressionBuilderBase.ToFunctionCall

**Returns:** `T` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

## Node (class)

Type Node

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AST.cs)

### Constructors
- `void Node()` — Node.Node
- `void Node(ProtoCore.AST.Node rhs)` — Node.Node

### Methods
#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

Node.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AST.cs)

#### `void InheritID(int id)`

An explicit mechanism to manually set the ID of an AST node

**Parameters:**
- `id` (int) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AST.cs)

### Properties
- `ID` (int, get) — Node.ID
- `Name` (string, get/set) — Node.Name
- `charPos` (int, get/set) — Node.charPos
- `col` (int, get/set) — Node.col
- `endCharPos` (int, get/set) — Node.endCharPos
- `endCol` (int, get/set) — Node.endCol
- `endLine` (int, get/set) — Node.endLine
- `line` (int, get/set) — Node.line
- `skipMe` (bool, get/set) — Node.skipMe

## NodeEnumerableExtensions (static-class)

Type NodeEnumerableExtensions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Methods
#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> AsEnumerable(ProtoCore.AST.Node item)`

NodeEnumerableExtensions.AsEnumerable

**Parameters:**
- `item` (ProtoCore.AST.Node)

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Concat(System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> list, ProtoCore.AST.Node item)`

NodeEnumerableExtensions.Concat

**Parameters:**
- `list` (System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>)
- `item` (ProtoCore.AST.Node)

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

