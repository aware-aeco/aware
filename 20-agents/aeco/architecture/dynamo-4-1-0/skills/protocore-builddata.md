---
name: dynamo-protocore-builddata
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.BuildData namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ErrorType, WarningID, ErrorEntry, WarningEntry.
---

# ProtoCore.BuildData

Auto-generated from vendor docs for dynamo 4.1.0. 4 types in this namespace.

## ErrorEntry (struct)

Type ErrorEntry

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

## ErrorType (enum)

Type ErrorType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Values
- `MaxErrorID` = `2`
- `SemanticError` = `1`
- `SyntaxError` = `0`

## WarningEntry (struct)

Type WarningEntry

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

## WarningID (enum)

Type WarningID

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/BuildStatus.cs)

### Values
- `AccessViolation` = `0`
- `CallingConstructorInConstructor` = `1`
- `CallingConstructorOnInstance` = `2`
- `CallingNonStaticMethodOnClass` = `3`
- `FileNotFound` = `15`
- `FunctionAbnormalExit` = `4`
- `FunctionAlreadyDefined` = `5`
- `FunctionNotFound` = `6`
- `IdUnboundIdentifier` = `7`
- `InvalidRangeExpression` = `9`
- `InvalidStaticCyclicDependency` = `8`
- `InvalidThis` = `10`
- `MissingReturnStatement` = `11`
- `MultipleSymbolFound` = `16`
- `MultipleSymbolFoundFromName` = `17`
- `Parsing` = `12`
- `PropertyNotFound` = `14`
- `TypeUndefined` = `13`

