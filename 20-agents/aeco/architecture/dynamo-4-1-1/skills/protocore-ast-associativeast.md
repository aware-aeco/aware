---
name: dynamo-protocore-ast-associativeast
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.AST.AssociativeAST namespace — 42 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AstKind, AssociativeNode, CommentNode, LanguageBlockNode, ReplicationGuideNode, AtLevelNode, ArrayNameNode, GroupExpressionNode, and 34 more types.
---

# ProtoCore.AST.AssociativeAST

Auto-generated from vendor docs for dynamo 4.1.1. 42 types in this namespace.

## ArgumentSignatureNode (class)

Type ArgumentSignatureNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ArgumentSignatureNode()` — ArgumentSignatureNode.ArgumentSignatureNode
- `void ArgumentSignatureNode(ProtoCore.AST.AssociativeAST.ArgumentSignatureNode rhs)` — ArgumentSignatureNode.ArgumentSignatureNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ArgumentSignatureNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `void AddArgument(ProtoCore.AST.AssociativeAST.VarDeclNode arg)`

ArgumentSignatureNode.AddArgument

**Parameters:**
- `arg` (ProtoCore.AST.AssociativeAST.VarDeclNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ArgumentSignatureNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ArgumentSignatureNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ArgumentSignatureNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ArgumentSignatureNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.VarDeclNode>, get/set) — ArgumentSignatureNode.Arguments
- `IsVarArg` (bool, get/set) — ArgumentSignatureNode.IsVarArg
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ArgumentSignatureNode.Kind

## ArrayNameNode (class)

Type ArrayNameNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ArrayNameNode()` — ArrayNameNode.ArrayNameNode
- `void ArrayNameNode(ProtoCore.AST.AssociativeAST.ArrayNameNode rhs)` — ArrayNameNode.ArrayNameNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ArrayNameNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ArrayNameNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ArrayNameNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ArrayNameNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ArrayNameNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `ArrayDimensions` (ProtoCore.AST.AssociativeAST.ArrayNode, get/set) — ArrayNameNode.ArrayDimensions
- `AtLevel` (ProtoCore.AST.AssociativeAST.AtLevelNode, get/set) — ArrayNameNode.AtLevel
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ArrayNameNode.Kind
- `ReplicationGuides` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — ArrayNameNode.ReplicationGuides

## ArrayNode (class)

Type ArrayNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ArrayNode()` — ArrayNode.ArrayNode
- `void ArrayNode(ProtoCore.AST.AssociativeAST.ArrayNode rhs)` — ArrayNode.ArrayNode
- `void ArrayNode(ProtoCore.AST.AssociativeAST.AssociativeNode expr, ProtoCore.AST.AssociativeAST.AssociativeNode type)` — ArrayNode.ArrayNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ArrayNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ArrayNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ArrayNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ArrayNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ArrayNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Expr` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — ArrayNode.Expr
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ArrayNode.Kind
- `Type` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — ArrayNode.Type

## AssociativeNode (class)

Type AssociativeNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void AssociativeNode()` — AssociativeNode.AssociativeNode
- `void AssociativeNode(ProtoCore.AST.AssociativeAST.AssociativeNode rhs)` — AssociativeNode.AssociativeNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

AssociativeNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — AssociativeNode.Kind

## AstExtensions (static-class)

Type AstExtensions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Methods
#### `ProtoCore.AST.ImperativeAST.ImperativeNode ToImperativeAST(ProtoCore.AST.AssociativeAST.AssociativeNode aNode)`

AstExtensions.ToImperativeAST

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.ArrayNameNode ToImperativeNode(ProtoCore.AST.AssociativeAST.ArrayNameNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.ArrayNameNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ArrayNameNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.ArrayNode ToImperativeNode(ProtoCore.AST.AssociativeAST.ArrayNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.ArrayNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ArrayNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.BinaryExpressionNode ToImperativeNode(ProtoCore.AST.AssociativeAST.BinaryExpressionNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.BinaryExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.BinaryExpressionNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.BooleanNode ToImperativeNode(ProtoCore.AST.AssociativeAST.BooleanNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.BooleanNode)

**Returns:** `ProtoCore.AST.ImperativeAST.BooleanNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.CharNode ToImperativeNode(ProtoCore.AST.AssociativeAST.CharNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.CharNode)

**Returns:** `ProtoCore.AST.ImperativeAST.CharNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.CodeBlockNode ToImperativeNode(ProtoCore.AST.AssociativeAST.CodeBlockNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `ProtoCore.AST.ImperativeAST.CodeBlockNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.DoubleNode ToImperativeNode(ProtoCore.AST.AssociativeAST.DoubleNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.DoubleNode)

**Returns:** `ProtoCore.AST.ImperativeAST.DoubleNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.ExprListNode ToImperativeNode(ProtoCore.AST.AssociativeAST.ExprListNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.ExprListNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ExprListNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.FunctionCallNode ToImperativeNode(ProtoCore.AST.AssociativeAST.FunctionCallNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.FunctionCallNode)

**Returns:** `ProtoCore.AST.ImperativeAST.FunctionCallNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.GroupExpressionNode ToImperativeNode(ProtoCore.AST.AssociativeAST.GroupExpressionNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.GroupExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.GroupExpressionNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.IdentifierListNode ToImperativeNode(ProtoCore.AST.AssociativeAST.IdentifierListNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `ProtoCore.AST.ImperativeAST.IdentifierListNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.IdentifierNode ToImperativeNode(ProtoCore.AST.AssociativeAST.IdentifierNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.IdentifierNode)

**Returns:** `ProtoCore.AST.ImperativeAST.IdentifierNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.InlineConditionalNode ToImperativeNode(ProtoCore.AST.AssociativeAST.InlineConditionalNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.InlineConditionalNode)

**Returns:** `ProtoCore.AST.ImperativeAST.InlineConditionalNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.IntNode ToImperativeNode(ProtoCore.AST.AssociativeAST.IntNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.IntNode)

**Returns:** `ProtoCore.AST.ImperativeAST.IntNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.LanguageBlockNode ToImperativeNode(ProtoCore.AST.AssociativeAST.LanguageBlockNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.LanguageBlockNode)

**Returns:** `ProtoCore.AST.ImperativeAST.LanguageBlockNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.NullNode ToImperativeNode(ProtoCore.AST.AssociativeAST.NullNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.NullNode)

**Returns:** `ProtoCore.AST.ImperativeAST.NullNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.RangeExprNode ToImperativeNode(ProtoCore.AST.AssociativeAST.RangeExprNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.RangeExprNode)

**Returns:** `ProtoCore.AST.ImperativeAST.RangeExprNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.StringNode ToImperativeNode(ProtoCore.AST.AssociativeAST.StringNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.StringNode)

**Returns:** `ProtoCore.AST.ImperativeAST.StringNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.TypedIdentifierNode ToImperativeNode(ProtoCore.AST.AssociativeAST.TypedIdentifierNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.TypedIdentifierNode)

**Returns:** `ProtoCore.AST.ImperativeAST.TypedIdentifierNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.UnaryExpressionNode ToImperativeNode(ProtoCore.AST.AssociativeAST.UnaryExpressionNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.AssociativeAST.UnaryExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.UnaryExpressionNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.BreakNode ToImperativeNode(ProtoCore.AST.ImperativeAST.BreakNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.ImperativeAST.BreakNode)

**Returns:** `ProtoCore.AST.ImperativeAST.BreakNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.ImperativeAST.ContinueNode ToImperativeNode(ProtoCore.AST.ImperativeAST.ContinueNode aNode)`

AstExtensions.ToImperativeNode

**Parameters:**
- `aNode` (ProtoCore.AST.ImperativeAST.ContinueNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ContinueNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

## AstFactory (class)

Type AstFactory

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void AstFactory()` — AstFactory.AstFactory

### Methods
#### `ProtoCore.AST.AssociativeAST.AssociativeNode AddAtLevel(ProtoCore.AST.AssociativeAST.AssociativeNode node, int level, bool shouldKeepListStructure)`

Create a copy of node with at-level. E.g., xs@-2

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode) — 
- `level` (int) — 
- `shouldKeepListStructure` (bool) — 

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode AddReplicationGuide(ProtoCore.AST.AssociativeAST.AssociativeNode node, System.Collections.Generic.List<int> guides, bool isLongest)`

Create a copy of the node with replication guide added.

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode) — Associative AST node.
- `guides` (System.Collections.Generic.List<int>) — Replication guide.
- `isLongest` (bool) — If use the Longest replication strategy.

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.BinaryExpressionNode BuildAssignment(ProtoCore.AST.AssociativeAST.AssociativeNode lhs, ProtoCore.AST.AssociativeAST.AssociativeNode rhs)`

AstFactory.BuildAssignment

**Parameters:**
- `lhs` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `rhs` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.BinaryExpressionNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.BinaryExpressionNode BuildBinaryExpression(ProtoCore.AST.AssociativeAST.AssociativeNode lhs, ProtoCore.AST.AssociativeAST.AssociativeNode rhs, ProtoCore.DSASM.Operator op)`

AstFactory.BuildBinaryExpression

**Parameters:**
- `lhs` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `rhs` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `op` (ProtoCore.DSASM.Operator)

**Returns:** `ProtoCore.AST.AssociativeAST.BinaryExpressionNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.BooleanNode BuildBooleanNode(bool value)`

AstFactory.BuildBooleanNode

**Parameters:**
- `value` (bool)

**Returns:** `ProtoCore.AST.AssociativeAST.BooleanNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.InlineConditionalNode BuildConditionalNode(ProtoCore.AST.AssociativeAST.AssociativeNode condition, ProtoCore.AST.AssociativeAST.AssociativeNode trueExpr, ProtoCore.AST.AssociativeAST.AssociativeNode falseExpr)`

AstFactory.BuildConditionalNode

**Parameters:**
- `condition` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `trueExpr` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `falseExpr` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.InlineConditionalNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.DoubleNode BuildDoubleNode(double value)`

AstFactory.BuildDoubleNode

**Parameters:**
- `value` (double)

**Returns:** `ProtoCore.AST.AssociativeAST.DoubleNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.ExprListNode BuildExprList(System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> nodes)`

AstFactory.BuildExprList

**Parameters:**
- `nodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `ProtoCore.AST.AssociativeAST.ExprListNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.ExprListNode BuildExprList(System.Collections.Generic.List<string> exprs)`

AstFactory.BuildExprList

**Parameters:**
- `exprs` (System.Collections.Generic.List<string>)

**Returns:** `ProtoCore.AST.AssociativeAST.ExprListNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall(System.Action a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>(System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, T9>(System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5, T6, T7, T8, T9>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8>(System.Action<T1, T2, T3, T4, T5, T6, T7, T8> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5, T6, T7, T8>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7>(System.Action<T1, T2, T3, T4, T5, T6, T7> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5, T6, T7>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6>(System.Action<T1, T2, T3, T4, T5, T6> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5, T6>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5>(System.Action<T1, T2, T3, T4, T5> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4, T5>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4>(System.Action<T1, T2, T3, T4> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3, T4>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3>(System.Action<T1, T2, T3> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2, T3>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2>(System.Action<T1, T2> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1, T2>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1>(System.Action<T1> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Action<T1>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, TRetType>(System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, TRetType>(System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, T9, TRetType>(System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, T5, T6, T7, T8, T9, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, T8, TRetType>(System.Func<T1, T2, T3, T4, T5, T6, T7, T8, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, T5, T6, T7, T8, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, T7, TRetType>(System.Func<T1, T2, T3, T4, T5, T6, T7, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, T5, T6, T7, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, T6, TRetType>(System.Func<T1, T2, T3, T4, T5, T6, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, T5, T6, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, T5, TRetType>(System.Func<T1, T2, T3, T4, T5, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, T5, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, T4, TRetType>(System.Func<T1, T2, T3, T4, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, T4, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, T3, TRetType>(System.Func<T1, T2, T3, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, T3, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, T2, TRetType>(System.Func<T1, T2, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, T2, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<T1, TRetType>(System.Func<T1, TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<T1, TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall<TRetType>(System.Func<TRetType> a, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `a` (System.Func<TRetType>)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall(string functionName, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `functionName` (string)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionCall(string className, string functionName, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments, ProtoCore.Core core)`

AstFactory.BuildFunctionCall

**Parameters:**
- `className` (string)
- `functionName` (string)
- `arguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionObject(ProtoCore.AST.AssociativeAST.AssociativeNode functionNode, int numParams, System.Collections.Generic.IEnumerable<int> connectedIndices, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputs)`

AstFactory.BuildFunctionObject

**Parameters:**
- `functionNode` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `numParams` (int)
- `connectedIndices` (System.Collections.Generic.IEnumerable<int>)
- `inputs` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildFunctionObject(string functionName, int numParams, System.Collections.Generic.IEnumerable<int> connectedIndices, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> inputs)`

AstFactory.BuildFunctionObject

**Parameters:**
- `functionName` (string)
- `numParams` (int)
- `connectedIndices` (System.Collections.Generic.IEnumerable<int>)
- `inputs` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierListNode BuildIdentList(ProtoCore.AST.AssociativeAST.AssociativeNode leftNode, ProtoCore.AST.AssociativeAST.AssociativeNode rightNode)`

AstFactory.BuildIdentList

**Parameters:**
- `leftNode` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `rightNode` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierListNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierNode BuildIdentifier(string name)`

AstFactory.BuildIdentifier

**Parameters:**
- `name` (string)

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierNode BuildIdentifier(string name, ProtoCore.AST.AssociativeAST.AssociativeNode arrayIndex)`

AstFactory.BuildIdentifier

**Parameters:**
- `name` (string)
- `arrayIndex` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildIndexExpression(ProtoCore.AST.AssociativeAST.AssociativeNode value, ProtoCore.AST.AssociativeAST.AssociativeNode index)`

AstFactory.BuildIndexExpression

**Parameters:**
- `value` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `index` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.IntNode BuildIntNode(int value)`

Build a DesignScript Int AST node from a 32 bit int input value.

**Parameters:**
- `value` (int) — 

**Returns:** `ProtoCore.AST.AssociativeAST.IntNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.IntNode BuildIntNode(long value)`

Build a DesignScript Int AST node from a long input value.

**Parameters:**
- `value` (long) — 

**Returns:** `ProtoCore.AST.AssociativeAST.IntNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.NullNode BuildNullNode()`

AstFactory.BuildNullNode

**Returns:** `ProtoCore.AST.AssociativeAST.NullNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.VarDeclNode BuildParamNode(string paramName)`

AstFactory.BuildParamNode

**Parameters:**
- `paramName` (string)

**Returns:** `ProtoCore.AST.AssociativeAST.VarDeclNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.VarDeclNode BuildParamNode(string paramName, ProtoCore.Type type)`

AstFactory.BuildParamNode

**Parameters:**
- `paramName` (string)
- `type` (ProtoCore.Type)

**Returns:** `ProtoCore.AST.AssociativeAST.VarDeclNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode BuildPrimitiveNodeFromObject(object value)`

Builds a integer, double, string, boolean or null node depending on input value type.

**Parameters:**
- `value` (object) — Input value

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — AssociativeNode

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.BinaryExpressionNode BuildReturnStatement(ProtoCore.AST.AssociativeAST.AssociativeNode rhs)`

AstFactory.BuildReturnStatement

**Parameters:**
- `rhs` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.BinaryExpressionNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.StringNode BuildStringNode(string str)`

AstFactory.BuildStringNode

**Parameters:**
- `str` (string)

**Returns:** `ProtoCore.AST.AssociativeAST.StringNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

## AstKind (enum)

Type AstKind

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Values
- `ArgumentSignature` = `2`
- `Array` = `0`
- `ArrayName` = `1`
- `AtLevel` = `3`
- `BinaryExpression` = `4`
- `Boolean` = `5`
- `Char` = `6`
- `ClassDeclaration` = `7`
- `CodeBlock` = `8`
- `Comment` = `9`
- `Constructor` = `10`
- `DefaultArgument` = `11`
- `Double` = `12`
- `Dynamic` = `13`
- `DynamicBlock` = `14`
- `ExpressionList` = `15`
- `FunctionCall` = `16`
- `FunctionDefintion` = `17`
- `FunctionDotCall` = `18`
- `GroupExpression` = `19`
- `Identifier` = `21`
- `IdentifierList` = `20`
- `If` = `22`
- `Import` = `23`
- `InlineConditional` = `24`
- `Integer` = `25`
- `LanguageBlock` = `26`
- `Null` = `27`
- `RangeExpression` = `28`
- `ReplicationGuide` = `29`
- `String` = `30`
- `ThisPointer` = `31`
- `TypedIdentifier` = `32`
- `UnaryExpression` = `33`
- `VariableDeclaration` = `34`

## AtLevelNode (class)

Type AtLevelNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void AtLevelNode()` — AtLevelNode.AtLevelNode
- `void AtLevelNode(ProtoCore.AST.AssociativeAST.AtLevelNode rhs)` — AtLevelNode.AtLevelNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

AtLevelNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

AtLevelNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

AtLevelNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

AtLevelNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

AtLevelNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `IsDominant` (bool, get/set) — AtLevelNode.IsDominant
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — AtLevelNode.Kind
- `Level` (long, get/set) — AtLevelNode.Level

## BinaryExpressionNode (class)

Type BinaryExpressionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void BinaryExpressionNode(ProtoCore.AST.AssociativeAST.AssociativeNode left, ProtoCore.AST.AssociativeAST.AssociativeNode right, ProtoCore.DSASM.Operator optr)` — Create a Binary assignment node from a given lhs identifier and given right node with line and col properties of rhs node
- `void BinaryExpressionNode(ProtoCore.AST.AssociativeAST.BinaryExpressionNode rhs)` — Create a Binary assignment node from a given lhs identifier and given right node with line and col properties of rhs node
- `void BinaryExpressionNode(ProtoCore.AST.AssociativeAST.IdentifierNode lhs, ProtoCore.AST.AssociativeAST.AssociativeNode rhs)` — Create a Binary assignment node from a given lhs identifier and given right node with line and col properties of rhs node

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

BinaryExpressionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

BinaryExpressionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

BinaryExpressionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

BinaryExpressionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

BinaryExpressionNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `ExpressionUID` (int, get/set) — BinaryExpressionNode.ExpressionUID
- `IsFirstIdentListNode` (bool, get/set) — BinaryExpressionNode.IsFirstIdentListNode
- `IsInputExpression` (bool, get/set) — BinaryExpressionNode.IsInputExpression
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — BinaryExpressionNode.Kind
- `LeftNode` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — BinaryExpressionNode.LeftNode
- `Optr` (ProtoCore.DSASM.Operator, get/set) — BinaryExpressionNode.Optr
- `OriginalAstID` (int, get/set) — BinaryExpressionNode.OriginalAstID
- `RightNode` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — BinaryExpressionNode.RightNode
- `SSAExpressionUID` (int, get/set) — BinaryExpressionNode.SSAExpressionUID
- `SSASubExpressionID` (int, get/set) — BinaryExpressionNode.SSASubExpressionID
- `guid` (System.Guid, get/set) — BinaryExpressionNode.guid
- `isMultipleAssign` (bool, get/set) — BinaryExpressionNode.isMultipleAssign
- `isSSAAssignment` (bool, get/set) — BinaryExpressionNode.isSSAAssignment
- `isSSAFirstAssignment` (bool, get/set) — BinaryExpressionNode.isSSAFirstAssignment
- `isSSAPointerAssignment` (bool, get/set) — BinaryExpressionNode.isSSAPointerAssignment

## BooleanNode (class)

Type BooleanNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void BooleanNode(ProtoCore.AST.AssociativeAST.BooleanNode rhs)` — BooleanNode.BooleanNode
- `void BooleanNode(bool value)` — BooleanNode.BooleanNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

BooleanNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

BooleanNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

BooleanNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

BooleanNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

BooleanNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — BooleanNode.Kind
- `Value` (bool, get/set) — BooleanNode.Value

## CharNode (class)

Type CharNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void CharNode()` — CharNode.CharNode
- `void CharNode(ProtoCore.AST.AssociativeAST.CharNode rhs)` — CharNode.CharNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

CharNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

CharNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

CharNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

CharNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

CharNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — CharNode.Kind
- `Value` (string, get/set) — CharNode.Value

## ClassAttributes (class)

Type ClassAttributes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ClassAttributes(string msg, string preferredShortName)` — ClassAttributes.ClassAttributes

### Properties
- `HiddenInLibrary` (bool, get) — ClassAttributes.HiddenInLibrary
- `IsObsolete` (bool, get) — ClassAttributes.IsObsolete
- `ObsoleteMessage` (string, get) — ClassAttributes.ObsoleteMessage
- `PreferredShortName` (string, get) — ClassAttributes.PreferredShortName

## ClassDeclNode (class)

Type ClassDeclNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ClassDeclNode()` — ClassDeclNode.ClassDeclNode
- `void ClassDeclNode(ProtoCore.AST.AssociativeAST.ClassDeclNode rhs)` — ClassDeclNode.ClassDeclNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ClassDeclNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ClassDeclNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ClassDeclNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ClassDeclNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ClassDeclNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Attributes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — ClassDeclNode.Attributes
- `BaseClass` (string, get/set) — ClassDeclNode.BaseClass
- `ClassAttributes` (ProtoCore.AST.AssociativeAST.ClassAttributes, get/set) — ClassDeclNode.ClassAttributes
- `ClassName` (string, get/set) — ClassDeclNode.ClassName
- `ExternLibName` (string, get/set) — ClassDeclNode.ExternLibName
- `Interfaces` (System.Collections.Generic.List<string>, get/set) — ClassDeclNode.Interfaces
- `IsExternLib` (bool, get/set) — ClassDeclNode.IsExternLib
- `IsImportedClass` (bool, get/set) — ClassDeclNode.IsImportedClass
- `IsInterface` (bool, get/set) — ClassDeclNode.IsInterface
- `IsStatic` (bool, get/set) — ClassDeclNode.IsStatic
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ClassDeclNode.Kind
- `Procedures` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — ClassDeclNode.Procedures
- `Variables` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — ClassDeclNode.Variables

## CodeBlockNode (class)

Type CodeBlockNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void CodeBlockNode()` — CodeBlockNode.CodeBlockNode
- `void CodeBlockNode(ProtoCore.AST.AssociativeAST.CodeBlockNode rhs)` — CodeBlockNode.CodeBlockNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

CodeBlockNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

CodeBlockNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

CodeBlockNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

CodeBlockNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

CodeBlockNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Body` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — CodeBlockNode.Body
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — CodeBlockNode.Kind
- `ProcTable` (ProtoCore.DSASM.ProcedureTable, get/set) — CodeBlockNode.ProcTable
- `Symbols` (ProtoCore.DSASM.SymbolTable, get/set) — CodeBlockNode.Symbols

## CommentNode (class)

Type CommentNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void CommentNode(int col, int line, string value, ProtoCore.AST.AssociativeAST.CommentNode.CommentType type)` — CommentNode.CommentNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

CommentNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

CommentNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — CommentNode.Kind
- `Type` (ProtoCore.AST.AssociativeAST.CommentNode.CommentType, get) — CommentNode.Type
- `Value` (string, get) — CommentNode.Value

## ConstructorDefinitionNode (class)

Type ConstructorDefinitionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ConstructorDefinitionNode()` — ConstructorDefinitionNode.ConstructorDefinitionNode
- `void ConstructorDefinitionNode(ProtoCore.AST.AssociativeAST.ConstructorDefinitionNode rhs)` — ConstructorDefinitionNode.ConstructorDefinitionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ConstructorDefinitionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ConstructorDefinitionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ConstructorDefinitionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ConstructorDefinitionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ConstructorDefinitionNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Access` (ProtoCore.CompilerDefinitions.AccessModifier, get/set) — ConstructorDefinitionNode.Access
- `BaseConstructor` (ProtoCore.AST.AssociativeAST.FunctionCallNode, get/set) — ConstructorDefinitionNode.BaseConstructor
- `ExternLibName` (string, get/set) — ConstructorDefinitionNode.ExternLibName
- `FunctionBody` (ProtoCore.AST.AssociativeAST.CodeBlockNode, get/set) — ConstructorDefinitionNode.FunctionBody
- `IsExternLib` (bool, get/set) — ConstructorDefinitionNode.IsExternLib
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ConstructorDefinitionNode.Kind
- `LocalVariableCount` (int, get/set) — ConstructorDefinitionNode.LocalVariableCount
- `MethodAttributes` (ProtoCore.AST.AssociativeAST.MethodAttributes, get/set) — ConstructorDefinitionNode.MethodAttributes
- `ReturnType` (ProtoCore.Type, get/set) — ConstructorDefinitionNode.ReturnType
- `Signature` (ProtoCore.AST.AssociativeAST.ArgumentSignatureNode, get/set) — ConstructorDefinitionNode.Signature

## DefaultArgNode (class)

Type DefaultArgNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void DefaultArgNode()` — DefaultArgNode.DefaultArgNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

DefaultArgNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

DefaultArgNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — DefaultArgNode.Kind

## DictionaryExpressionBuilder (class)

Type DictionaryExpressionBuilder

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

### Constructors
- `void DictionaryExpressionBuilder()` — DictionaryExpressionBuilder.DictionaryExpressionBuilder

### Methods
#### `ProtoCore.AST.AssociativeAST.AssociativeNode ToFunctionCall()`

DictionaryExpressionBuilder.ToFunctionCall

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/DictionaryBuilder.cs)

## DoubleNode (class)

Type DoubleNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void DoubleNode(ProtoCore.AST.AssociativeAST.DoubleNode rhs)` — DoubleNode.DoubleNode
- `void DoubleNode(double value)` — DoubleNode.DoubleNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

DoubleNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

DoubleNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

DoubleNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

DoubleNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

DoubleNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — DoubleNode.Kind
- `Value` (double, get/set) — DoubleNode.Value

## DynamicBlockNode (class)

Type DynamicBlockNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void DynamicBlockNode(int blockId)` — DynamicBlockNode.DynamicBlockNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

DynamicBlockNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

DynamicBlockNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

DynamicBlockNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

DynamicBlockNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — DynamicBlockNode.Kind
- `block` (int, get/set) — DynamicBlockNode.block

## DynamicNode (class)

Type DynamicNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void DynamicNode()` — DynamicNode.DynamicNode
- `void DynamicNode(ProtoCore.AST.AssociativeAST.DynamicNode rhs)` — DynamicNode.DynamicNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

DynamicNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

DynamicNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — DynamicNode.Kind

## ExprListNode (class)

Type ExprListNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ExprListNode()` — ExprListNode.ExprListNode
- `void ExprListNode(ProtoCore.AST.AssociativeAST.ExprListNode rhs)` — ExprListNode.ExprListNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ExprListNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ExprListNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ExprListNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ExprListNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ExprListNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Exprs` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — ExprListNode.Exprs
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ExprListNode.Kind

## ExternalAttributes (class)

Type ExternalAttributes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ExternalAttributes()` — ExternalAttributes.ExternalAttributes

### Methods
#### `void AddAttribute(string attribute, object value)`

ExternalAttributes.AddAttribute

**Parameters:**
- `attribute` (string)
- `value` (object)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool TryGetAttribute(string attribute, ref object value)`

ExternalAttributes.TryGetAttribute

**Parameters:**
- `attribute` (string)
- `value` (ref object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

## FunctionCallNode (class)

Type FunctionCallNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void FunctionCallNode()` — FunctionCallNode.FunctionCallNode
- `void FunctionCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode rhs)` — FunctionCallNode.FunctionCallNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

FunctionCallNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

FunctionCallNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

FunctionCallNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

FunctionCallNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

FunctionCallNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `DynamicTableIndex` (int, get/set) — FunctionCallNode.DynamicTableIndex
- `FormalArguments` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — FunctionCallNode.FormalArguments
- `Function` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — FunctionCallNode.Function
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — FunctionCallNode.Kind

## FunctionDefinitionNode (class)

Type FunctionDefinitionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void FunctionDefinitionNode()` — FunctionDefinitionNode.FunctionDefinitionNode
- `void FunctionDefinitionNode(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode rhs)` — FunctionDefinitionNode.FunctionDefinitionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

FunctionDefinitionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

FunctionDefinitionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

FunctionDefinitionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

FunctionDefinitionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

FunctionDefinitionNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Access` (ProtoCore.CompilerDefinitions.AccessModifier, get/set) — FunctionDefinitionNode.Access
- `Attributes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — FunctionDefinitionNode.Attributes
- `BuiltInMethodId` (ProtoCore.Lang.BuiltInMethods.MethodID, get/set) — FunctionDefinitionNode.BuiltInMethodId
- `ExternLibName` (string, get/set) — FunctionDefinitionNode.ExternLibName
- `FunctionBody` (ProtoCore.AST.AssociativeAST.CodeBlockNode, get/set) — FunctionDefinitionNode.FunctionBody
- `IsAssocOperator` (bool, get/set) — FunctionDefinitionNode.IsAssocOperator
- `IsAutoGenerated` (bool, get/set) — FunctionDefinitionNode.IsAutoGenerated
- `IsAutoGeneratedThisProc` (bool, get/set) — FunctionDefinitionNode.IsAutoGeneratedThisProc
- `IsBuiltIn` (bool, get/set) — FunctionDefinitionNode.IsBuiltIn
- `IsExternLib` (bool, get/set) — FunctionDefinitionNode.IsExternLib
- `IsStatic` (bool, get/set) — FunctionDefinitionNode.IsStatic
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — FunctionDefinitionNode.Kind
- `MethodAttributes` (ProtoCore.AST.AssociativeAST.MethodAttributes, get/set) — FunctionDefinitionNode.MethodAttributes
- `ReturnType` (ProtoCore.Type, get/set) — FunctionDefinitionNode.ReturnType
- `Signature` (ProtoCore.AST.AssociativeAST.ArgumentSignatureNode, get/set) — FunctionDefinitionNode.Signature

## FunctionDotCallNode (class)

Type FunctionDotCallNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void FunctionDotCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode callNode, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> arguments)` — FunctionDotCallNode.FunctionDotCallNode
- `void FunctionDotCallNode(ProtoCore.AST.AssociativeAST.FunctionDotCallNode rhs)` — FunctionDotCallNode.FunctionDotCallNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

FunctionDotCallNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

FunctionDotCallNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

FunctionDotCallNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

FunctionDotCallNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `ProtoCore.AST.AssociativeAST.IdentifierListNode GetIdentList()`

FunctionDotCallNode.GetIdentList

**Returns:** `ProtoCore.AST.AssociativeAST.IdentifierListNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

FunctionDotCallNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Arguments` (System.Collections.Generic.IList<ProtoCore.AST.AssociativeAST.AssociativeNode>, get) — FunctionDotCallNode.Arguments
- `FunctionCall` (ProtoCore.AST.AssociativeAST.FunctionCallNode, get/set) — FunctionDotCallNode.FunctionCall
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — FunctionDotCallNode.Kind
- `isLastSSAIdentListFactor` (bool, get/set) — FunctionDotCallNode.isLastSSAIdentListFactor

## GroupExpressionNode (class)

Type GroupExpressionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void GroupExpressionNode()` — GroupExpressionNode.GroupExpressionNode
- `void GroupExpressionNode(ProtoCore.AST.AssociativeAST.GroupExpressionNode rhs)` — GroupExpressionNode.GroupExpressionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

GroupExpressionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

GroupExpressionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

GroupExpressionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

GroupExpressionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

GroupExpressionNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Expression` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — GroupExpressionNode.Expression
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — GroupExpressionNode.Kind

## IdentifierListNode (class)

Type IdentifierListNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void IdentifierListNode()` — IdentifierListNode.IdentifierListNode
- `void IdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode rhs)` — IdentifierListNode.IdentifierListNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

IdentifierListNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

IdentifierListNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

IdentifierListNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

IdentifierListNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

IdentifierListNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `IsLastSSAIdentListFactor` (bool, get/set) — IdentifierListNode.IsLastSSAIdentListFactor
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — IdentifierListNode.Kind
- `LeftNode` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — IdentifierListNode.LeftNode
- `Optr` (ProtoCore.DSASM.Operator, get/set) — IdentifierListNode.Optr
- `RightNode` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — IdentifierListNode.RightNode

## IdentifierNode (class)

Type IdentifierNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void IdentifierNode(ProtoCore.AST.AssociativeAST.IdentifierNode rhs)` — IdentifierNode.IdentifierNode
- `void IdentifierNode(string identName)` — IdentifierNode.IdentifierNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

IdentifierNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

IdentifierNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

IdentifierNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

IdentifierNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `IsLocal` (bool, get/set) — IdentifierNode.IsLocal
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — IdentifierNode.Kind
- `Value` (string, get/set) — IdentifierNode.Value
- `datatype` (ProtoCore.Type, get/set) — IdentifierNode.datatype

## ImportNode (class)

Type ImportNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ImportNode()` — ImportNode.ImportNode
- `void ImportNode(ProtoCore.AST.AssociativeAST.ImportNode rhs)` — ImportNode.ImportNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ImportNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ImportNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ImportNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ImportNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ImportNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `CodeNode` (ProtoCore.AST.AssociativeAST.CodeBlockNode, get/set) — ImportNode.CodeNode
- `HasBeenImported` (bool, get/set) — ImportNode.HasBeenImported
- `Identifiers` (System.Collections.Generic.HashSet<string>, get/set) — ImportNode.Identifiers
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ImportNode.Kind
- `ModuleName` (string, get/set) — ImportNode.ModuleName
- `ModulePathFileName` (string, get/set) — ImportNode.ModulePathFileName

## InlineConditionalNode (class)

Type InlineConditionalNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void InlineConditionalNode()` — InlineConditionalNode.InlineConditionalNode
- `void InlineConditionalNode(ProtoCore.AST.AssociativeAST.InlineConditionalNode rhs)` — InlineConditionalNode.InlineConditionalNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

InlineConditionalNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

InlineConditionalNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

InlineConditionalNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

InlineConditionalNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

InlineConditionalNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `ConditionExpression` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — InlineConditionalNode.ConditionExpression
- `FalseExpression` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — InlineConditionalNode.FalseExpression
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — InlineConditionalNode.Kind
- `TrueExpression` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — InlineConditionalNode.TrueExpression

## IntNode (class)

Type IntNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void IntNode(ProtoCore.AST.AssociativeAST.IntNode rhs)` — IntNode.IntNode
- `void IntNode(long value)` — IntNode.IntNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

IntNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

IntNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

IntNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

IntNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

IntNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — IntNode.Kind
- `Value` (long, get/set) — IntNode.Value

## LanguageBlockNode (class)

Type LanguageBlockNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void LanguageBlockNode()` — LanguageBlockNode.LanguageBlockNode
- `void LanguageBlockNode(ProtoCore.AST.AssociativeAST.LanguageBlockNode rhs)` — LanguageBlockNode.LanguageBlockNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

LanguageBlockNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

LanguageBlockNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

LanguageBlockNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

LanguageBlockNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

LanguageBlockNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Attributes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>, get/set) — LanguageBlockNode.Attributes
- `CodeBlockNode` (ProtoCore.AST.Node, get/set) — LanguageBlockNode.CodeBlockNode
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — LanguageBlockNode.Kind
- `codeblock` (ProtoCore.LanguageCodeBlock, get/set) — LanguageBlockNode.codeblock

## MethodAttributes (class)

Type MethodAttributes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void MethodAttributes(bool hiddenInLibrary, bool canUpdatePeriodically, string msg)` — MethodAttributes.MethodAttributes

### Properties
- `AllowArrayPromotion` (bool, get) — MethodAttributes.AllowArrayPromotion
- `CanUpdatePeriodically` (bool, get) — MethodAttributes.CanUpdatePeriodically
- `Description` (string, get/set) — Returns/Sets description for the method.
- `HiddenInLibrary` (bool, get) — MethodAttributes.HiddenInLibrary
- `IsLacingDisabled` (bool, get) — MethodAttributes.IsLacingDisabled
- `IsObsolete` (bool, get) — MethodAttributes.IsObsolete
- `ObsoleteMessage` (string, get) — MethodAttributes.ObsoleteMessage
- `ReturnKeys` (System.Collections.Generic.IEnumerable<string>, get) — MethodAttributes.ReturnKeys

## NullNode (class)

Type NullNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void NullNode()` — NullNode.NullNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

NullNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

NullNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

NullNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

NullNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

NullNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — NullNode.Kind

## RangeExprNode (class)

Type RangeExprNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void RangeExprNode()` — RangeExprNode.RangeExprNode
- `void RangeExprNode(ProtoCore.AST.AssociativeAST.RangeExprNode rhs)` — RangeExprNode.RangeExprNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

RangeExprNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

RangeExprNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

RangeExprNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

RangeExprNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

RangeExprNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `From` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — RangeExprNode.From
- `HasRangeAmountOperator` (bool, get/set) — RangeExprNode.HasRangeAmountOperator
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — RangeExprNode.Kind
- `Step` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — RangeExprNode.Step
- `StepOperator` (ProtoCore.DSASM.RangeStepOperator, get/set) — RangeExprNode.StepOperator
- `To` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — RangeExprNode.To

## ReplicationGuideNode (class)

Type ReplicationGuideNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ReplicationGuideNode()` — ReplicationGuideNode.ReplicationGuideNode
- `void ReplicationGuideNode(ProtoCore.AST.AssociativeAST.ReplicationGuideNode rhs)` — ReplicationGuideNode.ReplicationGuideNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ReplicationGuideNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ReplicationGuideNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ReplicationGuideNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ReplicationGuideNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ReplicationGuideNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `IsLongest` (bool, get/set) — ReplicationGuideNode.IsLongest
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ReplicationGuideNode.Kind
- `RepGuide` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — ReplicationGuideNode.RepGuide

## StringNode (class)

Type StringNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void StringNode()` — StringNode.StringNode
- `void StringNode(ProtoCore.AST.AssociativeAST.StringNode rhs)` — StringNode.StringNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

StringNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

StringNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

StringNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

StringNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

StringNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — StringNode.Kind
- `Value` (string, get/set) — StringNode.Value

## ThisPointerNode (class)

Type ThisPointerNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void ThisPointerNode()` — ThisPointerNode.ThisPointerNode
- `void ThisPointerNode(ProtoCore.AST.AssociativeAST.ThisPointerNode rhs)` — ThisPointerNode.ThisPointerNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

ThisPointerNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

ThisPointerNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

ThisPointerNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

ThisPointerNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

ThisPointerNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — ThisPointerNode.Kind

## TypedIdentifierNode (class)

Type TypedIdentifierNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void TypedIdentifierNode()` — TypedIdentifierNode.TypedIdentifierNode
- `void TypedIdentifierNode(ProtoCore.AST.AssociativeAST.IdentifierNode rhs)` — TypedIdentifierNode.TypedIdentifierNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

TypedIdentifierNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

TypedIdentifierNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — TypedIdentifierNode.Kind
- `TypeAlias` (string, get/set) — TypedIdentifierNode.TypeAlias

## UnaryExpressionNode (class)

Type UnaryExpressionNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void UnaryExpressionNode()` — UnaryExpressionNode.UnaryExpressionNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

UnaryExpressionNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

UnaryExpressionNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

UnaryExpressionNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

UnaryExpressionNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Expression` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — UnaryExpressionNode.Expression
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — UnaryExpressionNode.Kind
- `Operator` (ProtoCore.DSASM.UnaryOperator, get/set) — UnaryExpressionNode.Operator

## VarDeclNode (class)

Type VarDeclNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Constructors
- `void VarDeclNode()` — VarDeclNode.VarDeclNode
- `void VarDeclNode(ProtoCore.AST.AssociativeAST.VarDeclNode rhs)` — VarDeclNode.VarDeclNode

### Methods
#### `TResult Accept<TResult>(ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult> visitor)`

VarDeclNode.Accept

**Parameters:**
- `visitor` (ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> Children()`

VarDeclNode.Children

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `bool Equals(object other)`

VarDeclNode.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `int GetHashCode()`

VarDeclNode.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

#### `string ToString()`

VarDeclNode.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/AssociativeAST.cs)

### Properties
- `Access` (ProtoCore.CompilerDefinitions.AccessModifier, get/set) — VarDeclNode.Access
- `ArgumentType` (ProtoCore.Type, get/set) — VarDeclNode.ArgumentType
- `ExternalAttributes` (ProtoCore.AST.AssociativeAST.ExternalAttributes, get/set) — VarDeclNode.ExternalAttributes
- `IsStatic` (bool, get/set) — VarDeclNode.IsStatic
- `Kind` (ProtoCore.AST.AssociativeAST.AstKind, get) — VarDeclNode.Kind
- `NameNode` (ProtoCore.AST.AssociativeAST.AssociativeNode, get/set) — VarDeclNode.NameNode

