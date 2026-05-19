---
name: dynamo-protocore-designscriptparser
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.DesignScriptParser namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Parser, Errors, FatalError, Token, Buffer, UTF8Buffer, Scanner.
---

# ProtoCore.DesignScriptParser

Auto-generated from vendor docs for dynamo 4.1.1. 7 types in this namespace.

## Buffer (class)

Type Buffer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

### Constructors
- `void Buffer(ProtoCore.DesignScriptParser.Buffer b)` — Buffer.Buffer
- `void Buffer(System.IO.Stream s, bool isUserStream)` — Buffer.Buffer

### Methods
#### `void Close()`

Buffer.Close

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

#### `void Finalize()`

Buffer.Finalize

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

#### `string GetString(int beg, int end)`

Buffer.GetString

**Parameters:**
- `beg` (int)
- `end` (int)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

#### `int Peek()`

Buffer.Peek

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

#### `int Read()`

Buffer.Read

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

### Properties
- `Pos` (int, get/set) — Buffer.Pos

## Errors (class)

Type Errors

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

### Constructors
- `void Errors()` — Errors.Errors

### Methods
#### `void SemErr(int line, int col, string s)`

Errors.SemErr

**Parameters:**
- `line` (int)
- `col` (int)
- `s` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

#### `void SemErr(string s)`

Errors.SemErr

**Parameters:**
- `s` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

#### `void SynErr(int line, int col, int n)`

Errors.SynErr

**Parameters:**
- `line` (int)
- `col` (int)
- `n` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

#### `void Warning(int line, int col, string s)`

Errors.Warning

**Parameters:**
- `line` (int)
- `col` (int)
- `s` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

#### `void Warning(string s)`

Errors.Warning

**Parameters:**
- `s` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

## FatalError (class)

Type FatalError

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

### Constructors
- `void FatalError(string m)` — FatalError.FatalError

## Parser (class)

Type Parser

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

### Constructors
- `void Parser(ProtoCore.DesignScriptParser.Scanner scanner, ProtoCore.Core coreObj, bool _builtinMethodsLoaded)` — Parser.Parser

### Methods
#### `ProtoCore.AST.ImperativeAST.IdentifierNode BuildImperativeIdentifier(string name, ProtoCore.PrimitiveType type)`

Parser.BuildImperativeIdentifier

**Parameters:**
- `name` (string)
- `type` (ProtoCore.PrimitiveType)

**Returns:** `ProtoCore.AST.ImperativeAST.IdentifierNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

#### `void Parse()`

Parser.Parse

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

#### `void SemErr(string msg)`

Parser.SemErr

**Parameters:**
- `msg` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Parser.cs)

### Properties
- `ImportModuleHandler` (ProtoFFI.ImportModuleHandler, get/set) — Parser.ImportModuleHandler
- `commentNode` (ProtoCore.AST.AssociativeAST.CodeBlockNode, get/set) — Parser.commentNode
- `root` (ProtoCore.AST.Node, get/set) — Parser.root

## Scanner (class)

Type Scanner

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

### Constructors
- `void Scanner(System.IO.Stream s)` — Scanner.Scanner
- `void Scanner(string fileName)` — Scanner.Scanner

### Methods
#### `ProtoCore.DesignScriptParser.Token Peek()`

Scanner.Peek

**Returns:** `ProtoCore.DesignScriptParser.Token` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

#### `void ResetPeek()`

Scanner.ResetPeek

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

#### `ProtoCore.DesignScriptParser.Token Scan()`

Scanner.Scan

**Returns:** `ProtoCore.DesignScriptParser.Token` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

## Token (class)

Type Token

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

### Constructors
- `void Token()` — Token.Token

## UTF8Buffer (class)

Type UTF8Buffer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

### Constructors
- `void UTF8Buffer(ProtoCore.DesignScriptParser.Buffer b)` — UTF8Buffer.UTF8Buffer

### Methods
#### `int Read()`

UTF8Buffer.Read

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Parser/Scanner.cs)

