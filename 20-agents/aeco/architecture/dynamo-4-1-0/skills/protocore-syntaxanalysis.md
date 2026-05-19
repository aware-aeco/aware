---
name: dynamo-protocore-syntaxanalysis
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.SyntaxAnalysis namespace — 9 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AssociativeAstVisitor, AssociativeAstVisitor<TResult>, AssociativeAstReplacer, AstVisitor<TAssociative, TImperative>, AstReplacer, AstTraversal, ImperativeAstVisitor, ImperativeAstVisitor<TResult>, and 1 more types.
---

# ProtoCore.SyntaxAnalysis

Auto-generated from vendor docs for dynamo 4.1.0. 9 types in this namespace.

## AssociativeAstReplacer (class)

Type AssociativeAstReplacer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

### Constructors
- `void AssociativeAstReplacer()` — AssociativeAstReplacer.AssociativeAstReplacer

### Methods
#### `ProtoCore.AST.AssociativeAST.AssociativeNode DefaultVisit(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

AssociativeAstReplacer.DefaultVisit

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitArrayNode(ProtoCore.AST.AssociativeAST.ArrayNode node)`

AssociativeAstReplacer.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitBinaryExpressionNode(ProtoCore.AST.AssociativeAST.BinaryExpressionNode node)`

AssociativeAstReplacer.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BinaryExpressionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitCodeBlockNode(ProtoCore.AST.AssociativeAST.CodeBlockNode node)`

AssociativeAstReplacer.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitExprListNode(ProtoCore.AST.AssociativeAST.ExprListNode node)`

AssociativeAstReplacer.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ExprListNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitFunctionCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode node)`

AssociativeAstReplacer.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionCallNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitFunctionDefinitionNode(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode node)`

AssociativeAstReplacer.VisitFunctionDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDefinitionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitGroupExpressionNode(ProtoCore.AST.AssociativeAST.GroupExpressionNode node)`

AssociativeAstReplacer.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.GroupExpressionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitIdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode node)`

AssociativeAstReplacer.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitIdentifierNode(ProtoCore.AST.AssociativeAST.IdentifierNode node)`

AssociativeAstReplacer.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitInlineConditionalNode(ProtoCore.AST.AssociativeAST.InlineConditionalNode node)`

AssociativeAstReplacer.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.InlineConditionalNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitLanguageBlockNode(ProtoCore.AST.AssociativeAST.LanguageBlockNode node)`

AssociativeAstReplacer.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.LanguageBlockNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> VisitNodeList(System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> nodes)`

AssociativeAstReplacer.VisitNodeList

**Parameters:**
- `nodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitRangeExprNode(ProtoCore.AST.AssociativeAST.RangeExprNode node)`

AssociativeAstReplacer.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.RangeExprNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitTypedIdentifierNode(ProtoCore.AST.AssociativeAST.TypedIdentifierNode node)`

AssociativeAstReplacer.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.TypedIdentifierNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitUnaryExpressionNode(ProtoCore.AST.AssociativeAST.UnaryExpressionNode node)`

AssociativeAstReplacer.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.UnaryExpressionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

## AssociativeAstVisitor (class)

Type AssociativeAstVisitor

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

### Constructors
- `void AssociativeAstVisitor()` — AssociativeAstVisitor.AssociativeAstVisitor

### Methods
#### `bool DefaultVisit(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

AssociativeAstVisitor.DefaultVisit

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool Visit(ProtoCore.AST.Node node)`

AssociativeAstVisitor.Visit

**Parameters:**
- `node` (ProtoCore.AST.Node)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitArgumentSignatureNode(ProtoCore.AST.AssociativeAST.ArgumentSignatureNode node)`

AssociativeAstVisitor.VisitArgumentSignatureNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArgumentSignatureNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitArrayNameNode(ProtoCore.AST.AssociativeAST.ArrayNameNode node)`

AssociativeAstVisitor.VisitArrayNameNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNameNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitArrayNode(ProtoCore.AST.AssociativeAST.ArrayNode node)`

AssociativeAstVisitor.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitAtLevelNode(ProtoCore.AST.AssociativeAST.AtLevelNode node)`

AssociativeAstVisitor.VisitAtLevelNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AtLevelNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitBinaryExpressionNode(ProtoCore.AST.AssociativeAST.BinaryExpressionNode node)`

AssociativeAstVisitor.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BinaryExpressionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitBooleanNode(ProtoCore.AST.AssociativeAST.BooleanNode node)`

AssociativeAstVisitor.VisitBooleanNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BooleanNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitCharNode(ProtoCore.AST.AssociativeAST.CharNode node)`

AssociativeAstVisitor.VisitCharNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CharNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitClassDeclNode(ProtoCore.AST.AssociativeAST.ClassDeclNode node)`

AssociativeAstVisitor.VisitClassDeclNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ClassDeclNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitCodeBlockNode(ProtoCore.AST.AssociativeAST.CodeBlockNode node)`

AssociativeAstVisitor.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitCommentNode(ProtoCore.AST.AssociativeAST.CommentNode node)`

AssociativeAstVisitor.VisitCommentNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CommentNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitConstructorDefinitionNode(ProtoCore.AST.AssociativeAST.ConstructorDefinitionNode node)`

AssociativeAstVisitor.VisitConstructorDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ConstructorDefinitionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitDefaultArgNode(ProtoCore.AST.AssociativeAST.DefaultArgNode node)`

AssociativeAstVisitor.VisitDefaultArgNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DefaultArgNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitDoubleNode(ProtoCore.AST.AssociativeAST.DoubleNode node)`

AssociativeAstVisitor.VisitDoubleNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DoubleNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitDynamicBlockNode(ProtoCore.AST.AssociativeAST.DynamicBlockNode node)`

AssociativeAstVisitor.VisitDynamicBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DynamicBlockNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitDynamicNode(ProtoCore.AST.AssociativeAST.DynamicNode node)`

AssociativeAstVisitor.VisitDynamicNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DynamicNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitExprListNode(ProtoCore.AST.AssociativeAST.ExprListNode node)`

AssociativeAstVisitor.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ExprListNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitFunctionCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode node)`

AssociativeAstVisitor.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionCallNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitFunctionDefinitionNode(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode node)`

AssociativeAstVisitor.VisitFunctionDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDefinitionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitFunctionDotCallNode(ProtoCore.AST.AssociativeAST.FunctionDotCallNode node)`

AssociativeAstVisitor.VisitFunctionDotCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDotCallNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitGroupExpressionNode(ProtoCore.AST.AssociativeAST.GroupExpressionNode node)`

AssociativeAstVisitor.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.GroupExpressionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitIdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode node)`

AssociativeAstVisitor.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitIdentifierNode(ProtoCore.AST.AssociativeAST.IdentifierNode node)`

AssociativeAstVisitor.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitImportNode(ProtoCore.AST.AssociativeAST.ImportNode node)`

AssociativeAstVisitor.VisitImportNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ImportNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitInlineConditionalNode(ProtoCore.AST.AssociativeAST.InlineConditionalNode node)`

AssociativeAstVisitor.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.InlineConditionalNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitIntNode(ProtoCore.AST.AssociativeAST.IntNode node)`

AssociativeAstVisitor.VisitIntNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IntNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitLanguageBlockNode(ProtoCore.AST.AssociativeAST.LanguageBlockNode node)`

AssociativeAstVisitor.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.LanguageBlockNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitNullNode(ProtoCore.AST.AssociativeAST.NullNode node)`

AssociativeAstVisitor.VisitNullNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.NullNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitRangeExprNode(ProtoCore.AST.AssociativeAST.RangeExprNode node)`

AssociativeAstVisitor.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.RangeExprNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitReplicationGuideNode(ProtoCore.AST.AssociativeAST.ReplicationGuideNode node)`

AssociativeAstVisitor.VisitReplicationGuideNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ReplicationGuideNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitStringNode(ProtoCore.AST.AssociativeAST.StringNode node)`

AssociativeAstVisitor.VisitStringNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.StringNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitThisPointerNode(ProtoCore.AST.AssociativeAST.ThisPointerNode node)`

AssociativeAstVisitor.VisitThisPointerNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ThisPointerNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitTypedIdentifierNode(ProtoCore.AST.AssociativeAST.TypedIdentifierNode node)`

AssociativeAstVisitor.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.TypedIdentifierNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitUnaryExpressionNode(ProtoCore.AST.AssociativeAST.UnaryExpressionNode node)`

AssociativeAstVisitor.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.UnaryExpressionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `bool VisitVarDeclNode(ProtoCore.AST.AssociativeAST.VarDeclNode node)`

AssociativeAstVisitor.VisitVarDeclNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.VarDeclNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

## AssociativeAstVisitor<TResult> (class)

Type AssociativeAstVisitor<TResult>

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

### Constructors
- `void AssociativeAstVisitor()` — AssociativeAstVisitor.AssociativeAstVisitor

### Methods
#### `TResult DefaultVisit(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

AssociativeAstVisitor.DefaultVisit

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult Visit(ProtoCore.AST.Node node)`

AssociativeAstVisitor.Visit

**Parameters:**
- `node` (ProtoCore.AST.Node)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitArgumentSignatureNode(ProtoCore.AST.AssociativeAST.ArgumentSignatureNode node)`

AssociativeAstVisitor.VisitArgumentSignatureNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArgumentSignatureNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitArrayNameNode(ProtoCore.AST.AssociativeAST.ArrayNameNode node)`

AssociativeAstVisitor.VisitArrayNameNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNameNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitArrayNode(ProtoCore.AST.AssociativeAST.ArrayNode node)`

AssociativeAstVisitor.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitAtLevelNode(ProtoCore.AST.AssociativeAST.AtLevelNode node)`

AssociativeAstVisitor.VisitAtLevelNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AtLevelNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitBinaryExpressionNode(ProtoCore.AST.AssociativeAST.BinaryExpressionNode node)`

AssociativeAstVisitor.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BinaryExpressionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitBooleanNode(ProtoCore.AST.AssociativeAST.BooleanNode node)`

AssociativeAstVisitor.VisitBooleanNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BooleanNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitCharNode(ProtoCore.AST.AssociativeAST.CharNode node)`

AssociativeAstVisitor.VisitCharNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CharNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitClassDeclNode(ProtoCore.AST.AssociativeAST.ClassDeclNode node)`

AssociativeAstVisitor.VisitClassDeclNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ClassDeclNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitCodeBlockNode(ProtoCore.AST.AssociativeAST.CodeBlockNode node)`

AssociativeAstVisitor.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitCommentNode(ProtoCore.AST.AssociativeAST.CommentNode node)`

AssociativeAstVisitor.VisitCommentNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CommentNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitConstructorDefinitionNode(ProtoCore.AST.AssociativeAST.ConstructorDefinitionNode node)`

AssociativeAstVisitor.VisitConstructorDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ConstructorDefinitionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitDefaultArgNode(ProtoCore.AST.AssociativeAST.DefaultArgNode node)`

AssociativeAstVisitor.VisitDefaultArgNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DefaultArgNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitDoubleNode(ProtoCore.AST.AssociativeAST.DoubleNode node)`

AssociativeAstVisitor.VisitDoubleNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DoubleNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitDynamicBlockNode(ProtoCore.AST.AssociativeAST.DynamicBlockNode node)`

AssociativeAstVisitor.VisitDynamicBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DynamicBlockNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitDynamicNode(ProtoCore.AST.AssociativeAST.DynamicNode node)`

AssociativeAstVisitor.VisitDynamicNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DynamicNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitExprListNode(ProtoCore.AST.AssociativeAST.ExprListNode node)`

AssociativeAstVisitor.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ExprListNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitFunctionCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode node)`

AssociativeAstVisitor.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionCallNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitFunctionDefinitionNode(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode node)`

AssociativeAstVisitor.VisitFunctionDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDefinitionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitFunctionDotCallNode(ProtoCore.AST.AssociativeAST.FunctionDotCallNode node)`

AssociativeAstVisitor.VisitFunctionDotCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDotCallNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitGroupExpressionNode(ProtoCore.AST.AssociativeAST.GroupExpressionNode node)`

AssociativeAstVisitor.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.GroupExpressionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitIdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode node)`

AssociativeAstVisitor.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitIdentifierNode(ProtoCore.AST.AssociativeAST.IdentifierNode node)`

AssociativeAstVisitor.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitImportNode(ProtoCore.AST.AssociativeAST.ImportNode node)`

AssociativeAstVisitor.VisitImportNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ImportNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitInlineConditionalNode(ProtoCore.AST.AssociativeAST.InlineConditionalNode node)`

AssociativeAstVisitor.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.InlineConditionalNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitIntNode(ProtoCore.AST.AssociativeAST.IntNode node)`

AssociativeAstVisitor.VisitIntNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IntNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitLanguageBlockNode(ProtoCore.AST.AssociativeAST.LanguageBlockNode node)`

AssociativeAstVisitor.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.LanguageBlockNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitNullNode(ProtoCore.AST.AssociativeAST.NullNode node)`

AssociativeAstVisitor.VisitNullNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.NullNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitRangeExprNode(ProtoCore.AST.AssociativeAST.RangeExprNode node)`

AssociativeAstVisitor.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.RangeExprNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitReplicationGuideNode(ProtoCore.AST.AssociativeAST.ReplicationGuideNode node)`

AssociativeAstVisitor.VisitReplicationGuideNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ReplicationGuideNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitStringNode(ProtoCore.AST.AssociativeAST.StringNode node)`

AssociativeAstVisitor.VisitStringNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.StringNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitThisPointerNode(ProtoCore.AST.AssociativeAST.ThisPointerNode node)`

AssociativeAstVisitor.VisitThisPointerNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ThisPointerNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitTypedIdentifierNode(ProtoCore.AST.AssociativeAST.TypedIdentifierNode node)`

AssociativeAstVisitor.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.TypedIdentifierNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitUnaryExpressionNode(ProtoCore.AST.AssociativeAST.UnaryExpressionNode node)`

AssociativeAstVisitor.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.UnaryExpressionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

#### `TResult VisitVarDeclNode(ProtoCore.AST.AssociativeAST.VarDeclNode node)`

AssociativeAstVisitor.VisitVarDeclNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.VarDeclNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AssociativeAstVisitor.cs)

## AstReplacer (class)

Type AstReplacer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

### Constructors
- `void AstReplacer()` — AstReplacer.AstReplacer

### Methods
#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitArrayNode(ProtoCore.AST.AssociativeAST.ArrayNode node)`

AstReplacer.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitArrayNode(ProtoCore.AST.ImperativeAST.ArrayNode node)`

AstReplacer.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitAssociativeNode(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

AstReplacer.VisitAssociativeNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitBinaryExpressionNode(ProtoCore.AST.AssociativeAST.BinaryExpressionNode node)`

AstReplacer.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BinaryExpressionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitBinaryExpressionNode(ProtoCore.AST.ImperativeAST.BinaryExpressionNode node)`

AstReplacer.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BinaryExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitCodeBlockNode(ProtoCore.AST.ImperativeAST.CodeBlockNode node)`

AstReplacer.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CodeBlockNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitElseIfNode(ProtoCore.AST.ImperativeAST.ElseIfBlock node)`

AstReplacer.VisitElseIfNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ElseIfBlock)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitExprListNode(ProtoCore.AST.AssociativeAST.ExprListNode node)`

AstReplacer.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ExprListNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitExprListNode(ProtoCore.AST.ImperativeAST.ExprListNode node)`

AstReplacer.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ExprListNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitForLoopNode(ProtoCore.AST.ImperativeAST.ForLoopNode node)`

AstReplacer.VisitForLoopNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ForLoopNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitFunctionCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode node)`

AstReplacer.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionCallNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitFunctionCallNode(ProtoCore.AST.ImperativeAST.FunctionCallNode node)`

AstReplacer.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.FunctionCallNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitFunctionDefinitionNode(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode node)`

AstReplacer.VisitFunctionDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDefinitionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitGroupExpressionNode(ProtoCore.AST.AssociativeAST.GroupExpressionNode node)`

AstReplacer.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.GroupExpressionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitGroupExpressionNode(ProtoCore.AST.ImperativeAST.GroupExpressionNode node)`

AstReplacer.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.GroupExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitIdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode node)`

AstReplacer.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitIdentifierListNode(ProtoCore.AST.ImperativeAST.IdentifierListNode node)`

AstReplacer.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierListNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitIdentifierNode(ProtoCore.AST.AssociativeAST.IdentifierNode node)`

AstReplacer.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitIdentifierNode(ProtoCore.AST.ImperativeAST.IdentifierNode node)`

AstReplacer.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitIfStatementNode(ProtoCore.AST.ImperativeAST.IfStmtNode node)`

AstReplacer.VisitIfStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitImperativeNode(ProtoCore.AST.ImperativeAST.ImperativeNode node)`

AstReplacer.VisitImperativeNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitInlineConditionalNode(ProtoCore.AST.AssociativeAST.InlineConditionalNode node)`

AstReplacer.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.InlineConditionalNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitInlineConditionalNode(ProtoCore.AST.ImperativeAST.InlineConditionalNode node)`

AstReplacer.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.InlineConditionalNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitLanguageBlockNode(ProtoCore.AST.AssociativeAST.LanguageBlockNode node)`

AstReplacer.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.LanguageBlockNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitLanguageBlockNode(ProtoCore.AST.ImperativeAST.LanguageBlockNode node)`

AstReplacer.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.LanguageBlockNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.Node VisitNode(ProtoCore.AST.Node node)`

AstReplacer.VisitNode

**Parameters:**
- `node` (ProtoCore.AST.Node)

**Returns:** `ProtoCore.AST.Node` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> VisitNodeList(System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> nodes)`

AstReplacer.VisitNodeList

**Parameters:**
- `nodes` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `System.Collections.Generic.List<ProtoCore.AST.Node> VisitNodeList(System.Collections.Generic.List<ProtoCore.AST.Node> nodes)`

AstReplacer.VisitNodeList

**Parameters:**
- `nodes` (System.Collections.Generic.List<ProtoCore.AST.Node>)

**Returns:** `System.Collections.Generic.List<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitRangeExprNode(ProtoCore.AST.AssociativeAST.RangeExprNode node)`

AstReplacer.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.RangeExprNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitRangeExprNode(ProtoCore.AST.ImperativeAST.RangeExprNode node)`

AstReplacer.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.RangeExprNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitTypedIdentifierNode(ProtoCore.AST.AssociativeAST.TypedIdentifierNode node)`

AstReplacer.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.TypedIdentifierNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitTypedIdentifierNode(ProtoCore.AST.ImperativeAST.TypedIdentifierNode node)`

AstReplacer.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.TypedIdentifierNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitUnaryExpressionNode(ProtoCore.AST.AssociativeAST.UnaryExpressionNode node)`

AstReplacer.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.UnaryExpressionNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitUnaryExpressionNode(ProtoCore.AST.ImperativeAST.UnaryExpressionNode node)`

AstReplacer.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.UnaryExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitWhileStatementNode(ProtoCore.AST.ImperativeAST.WhileStmtNode node)`

AstReplacer.VisitWhileStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.WhileStmtNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

## AstTraversal (class)

AstTraversal visits all nodes of the AST unless the result of a Visit* method is false or you override one of the methods such that it doesn't visit all of the Node's child nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

### Constructors
- `void AstTraversal()` — AstTraversal.AstTraversal

### Methods
#### `bool VisitAllChildren(ProtoCore.AST.Node node)`

AstTraversal.VisitAllChildren

**Parameters:**
- `node` (ProtoCore.AST.Node)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `bool VisitAssociativeNode(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

AstTraversal.VisitAssociativeNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `bool VisitImperativeNode(ProtoCore.AST.ImperativeAST.ImperativeNode node)`

AstTraversal.VisitImperativeNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

## AstVisitor<TAssociative, TImperative> (class)

Type AstVisitor<TAssociative, TImperative>

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

### Constructors
- `void AstVisitor()` — AstVisitor.AstVisitor

### Methods
#### `TAssociative VisitArgumentSignatureNode(ProtoCore.AST.AssociativeAST.ArgumentSignatureNode node)`

AstVisitor.VisitArgumentSignatureNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArgumentSignatureNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitArrayNameNode(ProtoCore.AST.AssociativeAST.ArrayNameNode node)`

AstVisitor.VisitArrayNameNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNameNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitArrayNameNode(ProtoCore.AST.ImperativeAST.ArrayNameNode node)`

AstVisitor.VisitArrayNameNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNameNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitArrayNode(ProtoCore.AST.AssociativeAST.ArrayNode node)`

AstVisitor.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ArrayNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitArrayNode(ProtoCore.AST.ImperativeAST.ArrayNode node)`

AstVisitor.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitAssociativeNode(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

AstVisitor.VisitAssociativeNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitAtLevelNode(ProtoCore.AST.AssociativeAST.AtLevelNode node)`

AstVisitor.VisitAtLevelNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AtLevelNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitBinaryExpressionNode(ProtoCore.AST.AssociativeAST.BinaryExpressionNode node)`

AstVisitor.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BinaryExpressionNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitBinaryExpressionNode(ProtoCore.AST.ImperativeAST.BinaryExpressionNode node)`

AstVisitor.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BinaryExpressionNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitBooleanNode(ProtoCore.AST.AssociativeAST.BooleanNode node)`

AstVisitor.VisitBooleanNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BooleanNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitBooleanNode(ProtoCore.AST.ImperativeAST.BooleanNode node)`

AstVisitor.VisitBooleanNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BooleanNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitBreakNode(ProtoCore.AST.ImperativeAST.BreakNode node)`

AstVisitor.VisitBreakNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BreakNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitCharNode(ProtoCore.AST.AssociativeAST.CharNode node)`

AstVisitor.VisitCharNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CharNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitCharNode(ProtoCore.AST.ImperativeAST.CharNode node)`

AstVisitor.VisitCharNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CharNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitClassDeclNode(ProtoCore.AST.AssociativeAST.ClassDeclNode node)`

AstVisitor.VisitClassDeclNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ClassDeclNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitCodeBlockNode(ProtoCore.AST.AssociativeAST.CodeBlockNode node)`

AstVisitor.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitCodeBlockNode(ProtoCore.AST.ImperativeAST.CodeBlockNode node)`

AstVisitor.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CodeBlockNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitCommentNode(ProtoCore.AST.AssociativeAST.CommentNode node)`

AstVisitor.VisitCommentNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CommentNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitConstructorDefinitionNode(ProtoCore.AST.AssociativeAST.ConstructorDefinitionNode node)`

AstVisitor.VisitConstructorDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ConstructorDefinitionNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitContinueNode(ProtoCore.AST.ImperativeAST.ContinueNode node)`

AstVisitor.VisitContinueNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ContinueNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitDefaultArgNode(ProtoCore.AST.AssociativeAST.DefaultArgNode node)`

AstVisitor.VisitDefaultArgNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DefaultArgNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitDoubleNode(ProtoCore.AST.AssociativeAST.DoubleNode node)`

AstVisitor.VisitDoubleNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DoubleNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitDoubleNode(ProtoCore.AST.ImperativeAST.DoubleNode node)`

AstVisitor.VisitDoubleNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.DoubleNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitDynamicBlockNode(ProtoCore.AST.AssociativeAST.DynamicBlockNode node)`

AstVisitor.VisitDynamicBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DynamicBlockNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitDynamicNode(ProtoCore.AST.AssociativeAST.DynamicNode node)`

AstVisitor.VisitDynamicNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.DynamicNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitElseIfNode(ProtoCore.AST.ImperativeAST.ElseIfBlock node)`

AstVisitor.VisitElseIfNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ElseIfBlock)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitExprListNode(ProtoCore.AST.AssociativeAST.ExprListNode node)`

AstVisitor.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ExprListNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitExprListNode(ProtoCore.AST.ImperativeAST.ExprListNode node)`

AstVisitor.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ExprListNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitForLoopNode(ProtoCore.AST.ImperativeAST.ForLoopNode node)`

AstVisitor.VisitForLoopNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ForLoopNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitFunctionCallNode(ProtoCore.AST.AssociativeAST.FunctionCallNode node)`

AstVisitor.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionCallNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitFunctionCallNode(ProtoCore.AST.ImperativeAST.FunctionCallNode node)`

AstVisitor.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.FunctionCallNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitFunctionDefinitionNode(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode node)`

AstVisitor.VisitFunctionDefinitionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDefinitionNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitFunctionDotCallNode(ProtoCore.AST.AssociativeAST.FunctionDotCallNode node)`

AstVisitor.VisitFunctionDotCallNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.FunctionDotCallNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitGroupExpressionNode(ProtoCore.AST.AssociativeAST.GroupExpressionNode node)`

AstVisitor.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.GroupExpressionNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitGroupExpressionNode(ProtoCore.AST.ImperativeAST.GroupExpressionNode node)`

AstVisitor.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.GroupExpressionNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitIdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode node)`

AstVisitor.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitIdentifierListNode(ProtoCore.AST.ImperativeAST.IdentifierListNode node)`

AstVisitor.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierListNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitIdentifierNode(ProtoCore.AST.AssociativeAST.IdentifierNode node)`

AstVisitor.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitIdentifierNode(ProtoCore.AST.ImperativeAST.IdentifierNode node)`

AstVisitor.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitIfStatementNode(ProtoCore.AST.ImperativeAST.IfStmtNode node)`

AstVisitor.VisitIfStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitIfStmtPositionNode(ProtoCore.AST.ImperativeAST.IfStmtPositionNode node)`

AstVisitor.VisitIfStmtPositionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtPositionNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitImperativeNode(ProtoCore.AST.ImperativeAST.ImperativeNode node)`

AstVisitor.VisitImperativeNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitImportNode(ProtoCore.AST.AssociativeAST.ImportNode node)`

AstVisitor.VisitImportNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ImportNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitInlineConditionalNode(ProtoCore.AST.AssociativeAST.InlineConditionalNode node)`

AstVisitor.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.InlineConditionalNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitInlineConditionalNode(ProtoCore.AST.ImperativeAST.InlineConditionalNode node)`

AstVisitor.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.InlineConditionalNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitIntNode(ProtoCore.AST.AssociativeAST.IntNode node)`

AstVisitor.VisitIntNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IntNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitIntNode(ProtoCore.AST.ImperativeAST.IntNode node)`

AstVisitor.VisitIntNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IntNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitLanguageBlockNode(ProtoCore.AST.AssociativeAST.LanguageBlockNode node)`

AstVisitor.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.LanguageBlockNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitLanguageBlockNode(ProtoCore.AST.ImperativeAST.LanguageBlockNode node)`

AstVisitor.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.LanguageBlockNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitNullNode(ProtoCore.AST.AssociativeAST.NullNode node)`

AstVisitor.VisitNullNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.NullNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitNullNode(ProtoCore.AST.ImperativeAST.NullNode node)`

AstVisitor.VisitNullNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.NullNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitRangeExprNode(ProtoCore.AST.AssociativeAST.RangeExprNode node)`

AstVisitor.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.RangeExprNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitRangeExprNode(ProtoCore.AST.ImperativeAST.RangeExprNode node)`

AstVisitor.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.RangeExprNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitReplicationGuideNode(ProtoCore.AST.AssociativeAST.ReplicationGuideNode node)`

AstVisitor.VisitReplicationGuideNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ReplicationGuideNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitStringNode(ProtoCore.AST.AssociativeAST.StringNode node)`

AstVisitor.VisitStringNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.StringNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitStringNode(ProtoCore.AST.ImperativeAST.StringNode node)`

AstVisitor.VisitStringNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.StringNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitThisPointerNode(ProtoCore.AST.AssociativeAST.ThisPointerNode node)`

AstVisitor.VisitThisPointerNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.ThisPointerNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitTypedIdentifierNode(ProtoCore.AST.AssociativeAST.TypedIdentifierNode node)`

AstVisitor.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.TypedIdentifierNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitTypedIdentifierNode(ProtoCore.AST.ImperativeAST.TypedIdentifierNode node)`

AstVisitor.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.TypedIdentifierNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitUnaryExpressionNode(ProtoCore.AST.AssociativeAST.UnaryExpressionNode node)`

AstVisitor.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.UnaryExpressionNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitUnaryExpressionNode(ProtoCore.AST.ImperativeAST.UnaryExpressionNode node)`

AstVisitor.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.UnaryExpressionNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TAssociative VisitVarDeclNode(ProtoCore.AST.AssociativeAST.VarDeclNode node)`

AstVisitor.VisitVarDeclNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.VarDeclNode)

**Returns:** `TAssociative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

#### `TImperative VisitWhileStatementNode(ProtoCore.AST.ImperativeAST.WhileStmtNode node)`

AstVisitor.VisitWhileStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.WhileStmtNode)

**Returns:** `TImperative` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/AstVisitor.cs)

## ImperativeAstReplacer (class)

Type ImperativeAstReplacer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

### Constructors
- `void ImperativeAstReplacer()` — ImperativeAstReplacer.ImperativeAstReplacer

### Methods
#### `ProtoCore.AST.ImperativeAST.ImperativeNode DefaultVisit(ProtoCore.AST.ImperativeAST.ImperativeNode node)`

ImperativeAstReplacer.DefaultVisit

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitArrayNode(ProtoCore.AST.ImperativeAST.ArrayNode node)`

ImperativeAstReplacer.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitBinaryExpressionNode(ProtoCore.AST.ImperativeAST.BinaryExpressionNode node)`

ImperativeAstReplacer.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BinaryExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitCodeBlockNode(ProtoCore.AST.ImperativeAST.CodeBlockNode node)`

ImperativeAstReplacer.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CodeBlockNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitElseIfNode(ProtoCore.AST.ImperativeAST.ElseIfBlock node)`

ImperativeAstReplacer.VisitElseIfNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ElseIfBlock)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitExprListNode(ProtoCore.AST.ImperativeAST.ExprListNode node)`

ImperativeAstReplacer.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ExprListNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitForLoopNode(ProtoCore.AST.ImperativeAST.ForLoopNode node)`

ImperativeAstReplacer.VisitForLoopNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ForLoopNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitFunctionCallNode(ProtoCore.AST.ImperativeAST.FunctionCallNode node)`

ImperativeAstReplacer.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.FunctionCallNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitGroupExpressionNode(ProtoCore.AST.ImperativeAST.GroupExpressionNode node)`

ImperativeAstReplacer.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.GroupExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitIdentifierListNode(ProtoCore.AST.ImperativeAST.IdentifierListNode node)`

ImperativeAstReplacer.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierListNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitIdentifierNode(ProtoCore.AST.ImperativeAST.IdentifierNode node)`

ImperativeAstReplacer.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitIfStatementNode(ProtoCore.AST.ImperativeAST.IfStmtNode node)`

ImperativeAstReplacer.VisitIfStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitInlineConditionalNode(ProtoCore.AST.ImperativeAST.InlineConditionalNode node)`

ImperativeAstReplacer.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.InlineConditionalNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitLanguageBlockNode(ProtoCore.AST.ImperativeAST.LanguageBlockNode node)`

ImperativeAstReplacer.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.LanguageBlockNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode> VisitNodeList(System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode> nodes)`

ImperativeAstReplacer.VisitNodeList

**Parameters:**
- `nodes` (System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>)

**Returns:** `System.Collections.Generic.List<ProtoCore.AST.ImperativeAST.ImperativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitRangeExprNode(ProtoCore.AST.ImperativeAST.RangeExprNode node)`

ImperativeAstReplacer.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.RangeExprNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitTypedIdentifierNode(ProtoCore.AST.ImperativeAST.TypedIdentifierNode node)`

ImperativeAstReplacer.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.TypedIdentifierNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitUnaryExpressionNode(ProtoCore.AST.ImperativeAST.UnaryExpressionNode node)`

ImperativeAstReplacer.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.UnaryExpressionNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode VisitWhileStatementNode(ProtoCore.AST.ImperativeAST.WhileStmtNode node)`

ImperativeAstReplacer.VisitWhileStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.WhileStmtNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

## ImperativeAstVisitor (class)

Type ImperativeAstVisitor

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

### Constructors
- `void ImperativeAstVisitor()` — ImperativeAstVisitor.ImperativeAstVisitor

### Methods
#### `bool DefaultVisit(ProtoCore.AST.ImperativeAST.ImperativeNode node)`

ImperativeAstVisitor.DefaultVisit

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool Visit(ProtoCore.AST.Node node)`

ImperativeAstVisitor.Visit

**Parameters:**
- `node` (ProtoCore.AST.Node)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitArrayNameNode(ProtoCore.AST.ImperativeAST.ArrayNameNode node)`

ImperativeAstVisitor.VisitArrayNameNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNameNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitArrayNode(ProtoCore.AST.ImperativeAST.ArrayNode node)`

ImperativeAstVisitor.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitBinaryExpressionNode(ProtoCore.AST.ImperativeAST.BinaryExpressionNode node)`

ImperativeAstVisitor.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BinaryExpressionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitBooleanNode(ProtoCore.AST.ImperativeAST.BooleanNode node)`

ImperativeAstVisitor.VisitBooleanNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BooleanNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitBreakNode(ProtoCore.AST.ImperativeAST.BreakNode node)`

ImperativeAstVisitor.VisitBreakNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BreakNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitCharNode(ProtoCore.AST.ImperativeAST.CharNode node)`

ImperativeAstVisitor.VisitCharNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CharNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitCodeBlockNode(ProtoCore.AST.ImperativeAST.CodeBlockNode node)`

ImperativeAstVisitor.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CodeBlockNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitContinueNode(ProtoCore.AST.ImperativeAST.ContinueNode node)`

ImperativeAstVisitor.VisitContinueNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ContinueNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitDoubleNode(ProtoCore.AST.ImperativeAST.DoubleNode node)`

ImperativeAstVisitor.VisitDoubleNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.DoubleNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitElseIfNode(ProtoCore.AST.ImperativeAST.ElseIfBlock node)`

ImperativeAstVisitor.VisitElseIfNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ElseIfBlock)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitExprListNode(ProtoCore.AST.ImperativeAST.ExprListNode node)`

ImperativeAstVisitor.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ExprListNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitForLoopNode(ProtoCore.AST.ImperativeAST.ForLoopNode node)`

ImperativeAstVisitor.VisitForLoopNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ForLoopNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitFunctionCallNode(ProtoCore.AST.ImperativeAST.FunctionCallNode node)`

ImperativeAstVisitor.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.FunctionCallNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitGroupExpressionNode(ProtoCore.AST.ImperativeAST.GroupExpressionNode node)`

ImperativeAstVisitor.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.GroupExpressionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitIdentifierListNode(ProtoCore.AST.ImperativeAST.IdentifierListNode node)`

ImperativeAstVisitor.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierListNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitIdentifierNode(ProtoCore.AST.ImperativeAST.IdentifierNode node)`

ImperativeAstVisitor.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitIfStatementNode(ProtoCore.AST.ImperativeAST.IfStmtNode node)`

ImperativeAstVisitor.VisitIfStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitIfStmtPositionNode(ProtoCore.AST.ImperativeAST.IfStmtPositionNode node)`

ImperativeAstVisitor.VisitIfStmtPositionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtPositionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitInlineConditionalNode(ProtoCore.AST.ImperativeAST.InlineConditionalNode node)`

ImperativeAstVisitor.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.InlineConditionalNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitIntNode(ProtoCore.AST.ImperativeAST.IntNode node)`

ImperativeAstVisitor.VisitIntNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IntNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitLanguageBlockNode(ProtoCore.AST.ImperativeAST.LanguageBlockNode node)`

ImperativeAstVisitor.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.LanguageBlockNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitNullNode(ProtoCore.AST.ImperativeAST.NullNode node)`

ImperativeAstVisitor.VisitNullNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.NullNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitRangeExprNode(ProtoCore.AST.ImperativeAST.RangeExprNode node)`

ImperativeAstVisitor.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.RangeExprNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitStringNode(ProtoCore.AST.ImperativeAST.StringNode node)`

ImperativeAstVisitor.VisitStringNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.StringNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitTypedIdentifierNode(ProtoCore.AST.ImperativeAST.TypedIdentifierNode node)`

ImperativeAstVisitor.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.TypedIdentifierNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitUnaryExpressionNode(ProtoCore.AST.ImperativeAST.UnaryExpressionNode node)`

ImperativeAstVisitor.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.UnaryExpressionNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `bool VisitWhileStatementNode(ProtoCore.AST.ImperativeAST.WhileStmtNode node)`

ImperativeAstVisitor.VisitWhileStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.WhileStmtNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

## ImperativeAstVisitor<TResult> (class)

Type ImperativeAstVisitor<TResult>

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

### Constructors
- `void ImperativeAstVisitor()` — ImperativeAstVisitor.ImperativeAstVisitor

### Methods
#### `TResult DefaultVisit(ProtoCore.AST.ImperativeAST.ImperativeNode node)`

ImperativeAstVisitor.DefaultVisit

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult Visit(ProtoCore.AST.Node node)`

ImperativeAstVisitor.Visit

**Parameters:**
- `node` (ProtoCore.AST.Node)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitArrayNameNode(ProtoCore.AST.ImperativeAST.ArrayNameNode node)`

ImperativeAstVisitor.VisitArrayNameNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNameNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitArrayNode(ProtoCore.AST.ImperativeAST.ArrayNode node)`

ImperativeAstVisitor.VisitArrayNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ArrayNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitBinaryExpressionNode(ProtoCore.AST.ImperativeAST.BinaryExpressionNode node)`

ImperativeAstVisitor.VisitBinaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BinaryExpressionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitBooleanNode(ProtoCore.AST.ImperativeAST.BooleanNode node)`

ImperativeAstVisitor.VisitBooleanNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BooleanNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitBreakNode(ProtoCore.AST.ImperativeAST.BreakNode node)`

ImperativeAstVisitor.VisitBreakNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BreakNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitCharNode(ProtoCore.AST.ImperativeAST.CharNode node)`

ImperativeAstVisitor.VisitCharNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CharNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitCodeBlockNode(ProtoCore.AST.ImperativeAST.CodeBlockNode node)`

ImperativeAstVisitor.VisitCodeBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.CodeBlockNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitContinueNode(ProtoCore.AST.ImperativeAST.ContinueNode node)`

ImperativeAstVisitor.VisitContinueNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ContinueNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitDoubleNode(ProtoCore.AST.ImperativeAST.DoubleNode node)`

ImperativeAstVisitor.VisitDoubleNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.DoubleNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitElseIfNode(ProtoCore.AST.ImperativeAST.ElseIfBlock node)`

ImperativeAstVisitor.VisitElseIfNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ElseIfBlock)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitExprListNode(ProtoCore.AST.ImperativeAST.ExprListNode node)`

ImperativeAstVisitor.VisitExprListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ExprListNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitForLoopNode(ProtoCore.AST.ImperativeAST.ForLoopNode node)`

ImperativeAstVisitor.VisitForLoopNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ForLoopNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitFunctionCallNode(ProtoCore.AST.ImperativeAST.FunctionCallNode node)`

ImperativeAstVisitor.VisitFunctionCallNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.FunctionCallNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitGroupExpressionNode(ProtoCore.AST.ImperativeAST.GroupExpressionNode node)`

ImperativeAstVisitor.VisitGroupExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.GroupExpressionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitIdentifierListNode(ProtoCore.AST.ImperativeAST.IdentifierListNode node)`

ImperativeAstVisitor.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierListNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitIdentifierNode(ProtoCore.AST.ImperativeAST.IdentifierNode node)`

ImperativeAstVisitor.VisitIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IdentifierNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitIfStatementNode(ProtoCore.AST.ImperativeAST.IfStmtNode node)`

ImperativeAstVisitor.VisitIfStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitIfStmtPositionNode(ProtoCore.AST.ImperativeAST.IfStmtPositionNode node)`

ImperativeAstVisitor.VisitIfStmtPositionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IfStmtPositionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitInlineConditionalNode(ProtoCore.AST.ImperativeAST.InlineConditionalNode node)`

ImperativeAstVisitor.VisitInlineConditionalNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.InlineConditionalNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitIntNode(ProtoCore.AST.ImperativeAST.IntNode node)`

ImperativeAstVisitor.VisitIntNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.IntNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitLanguageBlockNode(ProtoCore.AST.ImperativeAST.LanguageBlockNode node)`

ImperativeAstVisitor.VisitLanguageBlockNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.LanguageBlockNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitNullNode(ProtoCore.AST.ImperativeAST.NullNode node)`

ImperativeAstVisitor.VisitNullNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.NullNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitRangeExprNode(ProtoCore.AST.ImperativeAST.RangeExprNode node)`

ImperativeAstVisitor.VisitRangeExprNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.RangeExprNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitStringNode(ProtoCore.AST.ImperativeAST.StringNode node)`

ImperativeAstVisitor.VisitStringNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.StringNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitTypedIdentifierNode(ProtoCore.AST.ImperativeAST.TypedIdentifierNode node)`

ImperativeAstVisitor.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.TypedIdentifierNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitUnaryExpressionNode(ProtoCore.AST.ImperativeAST.UnaryExpressionNode node)`

ImperativeAstVisitor.VisitUnaryExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.UnaryExpressionNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

#### `TResult VisitWhileStatementNode(ProtoCore.AST.ImperativeAST.WhileStmtNode node)`

ImperativeAstVisitor.VisitWhileStatementNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.WhileStmtNode)

**Returns:** `TResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SyntaxAnalysis/ImperativeAstVisitor.cs)

