---
name: dynamo-protocore-ast-imperativeast
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.AST.ImperativeAST namespace — 31 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DictionaryExpressionBuilder, AstKind, ImperativeNode, LanguageBlockNode, ArrayNameNode, GroupExpressionNode, IdentifierNode, TypedIdentifierNode, and 23 more types.
---

# ProtoCore.AST.ImperativeAST

Auto-generated from vendor docs for dynamo 4.1.0. 31 types in this namespace.

## ArrayNameNode (class)

Type ArrayNameNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ArrayNameNode()` — ArrayNameNode.ArrayNameNode
- `void ArrayNameNode(ProtoCore.AST.ImperativeAST.ArrayNameNode rhs)` — ArrayNameNode.ArrayNameNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

ArrayNameNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ArrayNameNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ArrayNameNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ArrayNameNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ArrayNameNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `ArrayDimensions` (ProtoCore.AST.ImperativeAST.ArrayNode, get/set) — ArrayNameNode.ArrayDimensions
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — ArrayNameNode.Kind

## ArrayNode (class)

Type ArrayNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ArrayNode()` — ArrayNode.ArrayNode
- `void ArrayNode(ProtoCore.AST.ImperativeAST.ArrayNode rhs)` — ArrayNode.ArrayNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

ArrayNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ArrayNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ArrayNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ArrayNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ArrayNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Expr` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — ArrayNode.Expr
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — ArrayNode.Kind
- `Type` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — ArrayNode.Type

## AstFactory (static-class)

Type AstFactory

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Methods
#### `ProtoCore.AST.ImperativeAST.ImperativeNode BuildForLoopIndexExpression(ProtoCore.AST.ImperativeAST.ImperativeNode value, ProtoCore.AST.ImperativeAST.ImperativeNode index)`

AstFactory.BuildForLoopIndexExpression

**Parameters:**
- `value` (ProtoCore.AST.ImperativeAST.ImperativeNode)
- `index` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode BuildFunctionCall(string className, string functionName, System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode> args)`

AstFactory.BuildFunctionCall

**Parameters:**
- `className` (string)
- `functionName` (string)
- `args` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode BuildIndexExpression(ProtoCore.AST.ImperativeAST.ImperativeNode value, ProtoCore.AST.ImperativeAST.ImperativeNode index)`

AstFactory.BuildIndexExpression

**Parameters:**
- `value` (ProtoCore.AST.ImperativeAST.ImperativeNode)
- `index` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

## AstKind (enum)

Type AstKind

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Values
- `Array` = `1`
- `ArrayName` = `0`
- `BinaryExpression` = `2`
- `Boolean` = `3`
- `Break` = `4`
- `Char` = `5`
- `CodeBlock` = `6`
- `Continue` = `7`
- `Double` = `8`
- `ElseIf` = `9`
- `ExpressionList` = `10`
- `ForLoop` = `11`
- `FunctionCall` = `12`
- `GroupExpression` = `13`
- `Identifier` = `14`
- `IdentifierList` = `15`
- `If` = `16`
- `IfPosition` = `17`
- `InlineConditional` = `18`
- `Integer` = `19`
- `LanguageBlock` = `20`
- `Null` = `21`
- `RangeExpression` = `22`
- `String` = `23`
- `TypedIdentifier` = `24`
- `UnaryExpression` = `25`
- `While` = `26`

## BinaryExpressionNode (class)

Type BinaryExpressionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void BinaryExpressionNode()` — BinaryExpressionNode.BinaryExpressionNode
- `void BinaryExpressionNode(ProtoCore.AST.ImperativeAST.BinaryExpressionNode rhs)` — BinaryExpressionNode.BinaryExpressionNode
- `void BinaryExpressionNode(ProtoCore.AST.ImperativeAST.ImperativeNode left, ProtoCore.AST.ImperativeAST.ImperativeNode right, ProtoCore.DSASM.Operator optr)` — BinaryExpressionNode.BinaryExpressionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

BinaryExpressionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

BinaryExpressionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

BinaryExpressionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

BinaryExpressionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

BinaryExpressionNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — BinaryExpressionNode.Kind
- `LeftNode` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — BinaryExpressionNode.LeftNode
- `Optr` (ProtoCore.DSASM.Operator, get/set) — BinaryExpressionNode.Optr
- `RightNode` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — BinaryExpressionNode.RightNode
- `guid` (System.Guid, get/set) — BinaryExpressionNode.guid

## BooleanNode (class)

Type BooleanNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void BooleanNode(ProtoCore.AST.ImperativeAST.BooleanNode rhs)` — BooleanNode.BooleanNode
- `void BooleanNode(bool value)` — BooleanNode.BooleanNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

BooleanNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

BooleanNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

BooleanNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

BooleanNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

BooleanNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — BooleanNode.Kind
- `Value` (bool, get/set) — BooleanNode.Value

## BreakNode (class)

Type BreakNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

### Constructors
- `void BreakNode()` — BreakNode.BreakNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

BreakNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

BreakNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `string ToString()`

BreakNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — BreakNode.Kind

## CharNode (class)

Type CharNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void CharNode()` — CharNode.CharNode
- `void CharNode(ProtoCore.AST.ImperativeAST.CharNode rhs)` — CharNode.CharNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

CharNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

CharNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

CharNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

CharNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

CharNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — CharNode.Kind
- `Value` (string, get/set) — CharNode.Value

## CodeBlockNode (class)

Type CodeBlockNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void CodeBlockNode()` — CodeBlockNode.CodeBlockNode
- `void CodeBlockNode(ProtoCore.AST.ImperativeAST.CodeBlockNode rhs)` — CodeBlockNode.CodeBlockNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

CodeBlockNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

CodeBlockNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

CodeBlockNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

CodeBlockNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

CodeBlockNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Body` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — CodeBlockNode.Body
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — CodeBlockNode.Kind

## ContinueNode (class)

Type ContinueNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

### Constructors
- `void ContinueNode()` — ContinueNode.ContinueNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

ContinueNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ContinueNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `string ToString()`

ContinueNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — ContinueNode.Kind

## DictionaryExpressionBuilder (class)

Type DictionaryExpressionBuilder

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

### Constructors
- `void DictionaryExpressionBuilder()` — DictionaryExpressionBuilder.DictionaryExpressionBuilder

### Methods
#### `ProtoCore.AST.ImperativeAST.ImperativeNode ToFunctionCall()`

DictionaryExpressionBuilder.ToFunctionCall

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

## DoubleNode (class)

Type DoubleNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void DoubleNode(ProtoCore.AST.ImperativeAST.DoubleNode rhs)` — DoubleNode.DoubleNode
- `void DoubleNode(double value)` — DoubleNode.DoubleNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

DoubleNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

DoubleNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

DoubleNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

DoubleNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

DoubleNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — DoubleNode.Kind
- `Value` (double, get/set) — DoubleNode.Value

## ElseIfBlock (class)

Type ElseIfBlock

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Constructors
- `void ElseIfBlock()` — ElseIfBlock.ElseIfBlock
- `void ElseIfBlock(ProtoCore.AST.ImperativeAST.ElseIfBlock rhs)` — ElseIfBlock.ElseIfBlock

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

ElseIfBlock.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ElseIfBlock.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `bool Equals(object other)`

ElseIfBlock.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `int GetHashCode()`

ElseIfBlock.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `string ToString()`

ElseIfBlock.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Properties
- `Body` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — ElseIfBlock.Body
- `ElseIfBodyPosition` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — ElseIfBlock.ElseIfBodyPosition
- `Expr` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — ElseIfBlock.Expr
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — ElseIfBlock.Kind

## ExprListNode (class)

Type ExprListNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ExprListNode()` — ExprListNode.ExprListNode
- `void ExprListNode(ProtoCore.AST.ImperativeAST.ExprListNode rhs)` — ExprListNode.ExprListNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

ExprListNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ExprListNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ExprListNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ExprListNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ExprListNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Exprs` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — ExprListNode.Exprs
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — ExprListNode.Kind

## ForLoopNode (class)

Type ForLoopNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

### Constructors
- `void ForLoopNode()` — ForLoopNode.ForLoopNode
- `void ForLoopNode(ProtoCore.AST.ImperativeAST.ForLoopNode rhs)` — ForLoopNode.ForLoopNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

ForLoopNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ForLoopNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `bool Equals(object other)`

ForLoopNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `int GetHashCode()`

ForLoopNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

#### `string ToString()`

ForLoopNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeGraph.cs)

### Properties
- `Body` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — ForLoopNode.Body
- `Expression` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — ForLoopNode.Expression
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — ForLoopNode.Kind
- `KwForCol` (int, get/set) — ForLoopNode.KwForCol
- `KwForLine` (int, get/set) — ForLoopNode.KwForLine
- `KwInCol` (int, get/set) — ForLoopNode.KwInCol
- `KwInLine` (int, get/set) — ForLoopNode.KwInLine
- `LoopVariable` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — ForLoopNode.LoopVariable

## FunctionCallNode (class)

Type FunctionCallNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void FunctionCallNode()` — FunctionCallNode.FunctionCallNode
- `void FunctionCallNode(ProtoCore.AST.ImperativeAST.FunctionCallNode rhs)` — FunctionCallNode.FunctionCallNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

FunctionCallNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

FunctionCallNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

FunctionCallNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

FunctionCallNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

FunctionCallNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `FormalArguments` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — FunctionCallNode.FormalArguments
- `Function` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — FunctionCallNode.Function
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — FunctionCallNode.Kind

## GroupExpressionNode (class)

Type GroupExpressionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void GroupExpressionNode()` — GroupExpressionNode.GroupExpressionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

GroupExpressionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

GroupExpressionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

GroupExpressionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Expression` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — GroupExpressionNode.Expression
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — GroupExpressionNode.Kind

## IdentifierListNode (class)

Type IdentifierListNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void IdentifierListNode()` — IdentifierListNode.IdentifierListNode
- `void IdentifierListNode(ProtoCore.AST.ImperativeAST.IdentifierListNode rhs)` — IdentifierListNode.IdentifierListNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

IdentifierListNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

IdentifierListNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

IdentifierListNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

IdentifierListNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

IdentifierListNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — IdentifierListNode.Kind
- `LeftNode` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — IdentifierListNode.LeftNode
- `Optr` (ProtoCore.DSASM.Operator, get/set) — IdentifierListNode.Optr
- `RightNode` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — IdentifierListNode.RightNode

## IdentifierNode (class)

Type IdentifierNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void IdentifierNode()` — IdentifierNode.IdentifierNode
- `void IdentifierNode(ProtoCore.AST.ImperativeAST.IdentifierNode rhs)` — IdentifierNode.IdentifierNode
- `void IdentifierNode(string identName)` — IdentifierNode.IdentifierNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

IdentifierNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

IdentifierNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

IdentifierNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

IdentifierNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

IdentifierNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `ArrayName` (string, get/set) — IdentifierNode.ArrayName
- `DataType` (ProtoCore.Type, get/set) — IdentifierNode.DataType
- `IsLocal` (bool, get/set) — IdentifierNode.IsLocal
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — IdentifierNode.Kind
- `Value` (string, get/set) — IdentifierNode.Value

## IfStmtNode (class)

Type IfStmtNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Constructors
- `void IfStmtNode()` — IfStmtNode.IfStmtNode
- `void IfStmtNode(ProtoCore.AST.ImperativeAST.IfStmtNode rhs)` — IfStmtNode.IfStmtNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

IfStmtNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

IfStmtNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `bool Equals(object other)`

IfStmtNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `int GetHashCode()`

IfStmtNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `string ToString()`

IfStmtNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Properties
- `ElseBody` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — IfStmtNode.ElseBody
- `ElseBodyPosition` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — IfStmtNode.ElseBodyPosition
- `ElseIfList` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ElseIfBlock>, get/set) — IfStmtNode.ElseIfList
- `IfBody` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — IfStmtNode.IfBody
- `IfBodyPosition` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — IfStmtNode.IfBodyPosition
- `IfExprNode` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — IfStmtNode.IfExprNode
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — IfStmtNode.Kind

## IfStmtPositionNode (class)

Type IfStmtPositionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Constructors
- `void IfStmtPositionNode()` — IfStmtPositionNode.IfStmtPositionNode
- `void IfStmtPositionNode(ProtoCore.AST.ImperativeAST.IfStmtPositionNode rhs)` — IfStmtPositionNode.IfStmtPositionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

IfStmtPositionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

IfStmtPositionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — IfStmtPositionNode.Kind

## ImperativeNode (class)

Type ImperativeNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Constructors
- `void ImperativeNode()` — ImperativeNode.ImperativeNode
- `void ImperativeNode(ProtoCore.AST.ImperativeAST.ImperativeNode rhs)` — ImperativeNode.ImperativeNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

ImperativeNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — ImperativeNode.Kind

## InlineConditionalNode (class)

Type InlineConditionalNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void InlineConditionalNode()` — InlineConditionalNode.InlineConditionalNode
- `void InlineConditionalNode(ProtoCore.AST.ImperativeAST.InlineConditionalNode rhs)` — InlineConditionalNode.InlineConditionalNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

InlineConditionalNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

InlineConditionalNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

InlineConditionalNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

InlineConditionalNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

InlineConditionalNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `ConditionExpression` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — InlineConditionalNode.ConditionExpression
- `FalseExpression` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — InlineConditionalNode.FalseExpression
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — InlineConditionalNode.Kind
- `TrueExpression` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — InlineConditionalNode.TrueExpression

## IntNode (class)

Type IntNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void IntNode(ProtoCore.AST.ImperativeAST.IntNode rhs)` — IntNode.IntNode
- `void IntNode(long value)` — IntNode.IntNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

IntNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

IntNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

IntNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

IntNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

IntNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — IntNode.Kind
- `Value` (long, get/set) — IntNode.Value

## LanguageBlockNode (class)

Type LanguageBlockNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void LanguageBlockNode()` — LanguageBlockNode.LanguageBlockNode
- `void LanguageBlockNode(ProtoCore.AST.ImperativeAST.LanguageBlockNode rhs)` — LanguageBlockNode.LanguageBlockNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

LanguageBlockNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

LanguageBlockNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

LanguageBlockNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

LanguageBlockNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

LanguageBlockNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Attributes` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — LanguageBlockNode.Attributes
- `CodeBlockNode` (ProtoCore.AST.Node, get/set) — LanguageBlockNode.CodeBlockNode
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — LanguageBlockNode.Kind
- `codeblock` (ProtoCore.LanguageCodeBlock, get/set) — LanguageBlockNode.codeblock

## NullNode (class)

Type NullNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void NullNode()` — NullNode.NullNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

NullNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

NullNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

NullNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

NullNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

NullNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — NullNode.Kind

## RangeExprNode (class)

Type RangeExprNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void RangeExprNode()` — RangeExprNode.RangeExprNode
- `void RangeExprNode(ProtoCore.AST.ImperativeAST.RangeExprNode rhs)` — RangeExprNode.RangeExprNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

RangeExprNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

RangeExprNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

RangeExprNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

RangeExprNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

RangeExprNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `From` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — RangeExprNode.From
- `HasRangeAmountOperator` (bool, get/set) — RangeExprNode.HasRangeAmountOperator
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — RangeExprNode.Kind
- `Step` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — RangeExprNode.Step
- `StepOperator` (ProtoCore.DSASM.RangeStepOperator, get/set) — RangeExprNode.StepOperator
- `To` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — RangeExprNode.To

## StringNode (class)

Type StringNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void StringNode()` — StringNode.StringNode
- `void StringNode(ProtoCore.AST.ImperativeAST.StringNode rhs)` — StringNode.StringNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

StringNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

StringNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

StringNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

StringNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

StringNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — StringNode.Kind
- `Value` (string, get/set) — StringNode.Value

## TypedIdentifierNode (class)

Type TypedIdentifierNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void TypedIdentifierNode()` — TypedIdentifierNode.TypedIdentifierNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

TypedIdentifierNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

TypedIdentifierNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — TypedIdentifierNode.Kind

## UnaryExpressionNode (class)

Type UnaryExpressionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void UnaryExpressionNode()` — UnaryExpressionNode.UnaryExpressionNode
- `void UnaryExpressionNode(ProtoCore.AST.ImperativeAST.UnaryExpressionNode rhs)` — UnaryExpressionNode.UnaryExpressionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

UnaryExpressionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

UnaryExpressionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

UnaryExpressionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

UnaryExpressionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Expression` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — UnaryExpressionNode.Expression
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — UnaryExpressionNode.Kind
- `Operator` (ProtoCore.DSASM.UnaryOperator, get/set) — UnaryExpressionNode.Operator

## WhileStmtNode (class)

Type WhileStmtNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Constructors
- `void WhileStmtNode()` — WhileStmtNode.WhileStmtNode
- `void WhileStmtNode(ProtoCore.AST.ImperativeAST.WhileStmtNode rhs)` — WhileStmtNode.WhileStmtNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult> visitor)`

WhileStmtNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Imperative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

WhileStmtNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `bool Equals(object other)`

WhileStmtNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `int GetHashCode()`

WhileStmtNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

#### `string ToString()`

WhileStmtNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Parser/ImperativeAST.cs)

### Properties
- `Body` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>, get/set) — WhileStmtNode.Body
- `Expr` (ProtoCore.AST.ImperativeAST.ImperativeNode, get/set) — WhileStmtNode.Expr
- `Kind` (ProtoCore.AST.ImperativeAST.AstKind, get) — WhileStmtNode.Kind

