---
name: dynamo-protocore-compilerdefinitions
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.CompilerDefinitions namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AccessModifier, CompilePass, SubCompilePass.
---

# ProtoCore.CompilerDefinitions

Auto-generated from vendor docs for dynamo 4.1.1. 3 types in this namespace.

## AccessModifier (enum)

Type AccessModifier

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CompilerOptions.cs)

### Values
- `Private` = `2`
- `Protected` = `1`
- `Public` = `0`

## CompilePass (enum)

Type CompilePass

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CompilerOptions.cs)

### Values
- `ClassBaseClass` = `1`
- `ClassHierarchy` = `2`
- `ClassMemFuncBody` = `6`
- `ClassMemFuncSig` = `4`
- `ClassMemVar` = `3`
- `ClassName` = `0`
- `Done` = `9`
- `GlobalFuncBody` = `7`
- `GlobalFuncSig` = `5`
- `GlobalScope` = `8`

## SubCompilePass (enum)

Type SubCompilePass

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CompilerOptions.cs)

### Values
- `None` = `0`
- `UnboundIdentifier` = `1`

