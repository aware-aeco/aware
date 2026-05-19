---
name: dynamo-protocore-lang-builtinmethods
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.Lang.BuiltInMethods namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: MethodID, BuiltInMethod.
---

# ProtoCore.Lang.BuiltInMethods

Auto-generated from vendor docs for dynamo 4.1.0. 2 types in this namespace.

## BuiltInMethod (class)

Type BuiltInMethod

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/BuiltInMethods.cs)

### Constructors
- `void BuiltInMethod()` — BuiltInMethod.BuiltInMethod

### Properties
- `ID` (ProtoCore.Lang.BuiltInMethods.MethodID, get/set) — BuiltInMethod.ID
- `MethodAttributes` (ProtoCore.AST.AssociativeAST.MethodAttributes, get/set) — BuiltInMethod.MethodAttributes
- `Parameters` (System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, ProtoCore.Type>>, get/set) — BuiltInMethod.Parameters
- `ReturnType` (ProtoCore.Type, get/set) — BuiltInMethod.ReturnType

## MethodID (enum)

Type MethodID

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/BuiltInMethods.cs)

### Values
- `AllFalse` = `0`
- `AllTrue` = `1`
- `Average` = `2`
- `Break` = `47`
- `Concat` = `3`
- `ConditionalIf` = `55`
- `Contains` = `4`
- `ContainsKey` = `51`
- `Count` = `5`
- `CountFalse` = `7`
- `CountTrue` = `6`
- `Difference` = `8`
- `Dot` = `9`
- `Equals` = `10`
- `Evaluate` = `52`
- `Flatten` = `13`
- `GC` = `54`
- `GetElapsedTime` = `11`
- `GetKeys` = `48`
- `GetType` = `12`
- `GetValues` = `49`
- `ImportData` = `14`
- `IndexOf` = `15`
- `InlineConditional` = `46`
- `Insert` = `16`
- `Intersection` = `17`
- `InvalidMethodID` = `-1`
- `IsHomogeneous` = `20`
- `IsRectangular` = `19`
- `IsUniformDepth` = `18`
- `LoadCSV` = `21`
- `Map` = `22`
- `MapTo` = `23`
- `NodeAstFailed` = `53`
- `NormalizeDepth` = `24`
- `Print` = `25`
- `RangeExpression` = `40`
- `Rank` = `26`
- `Remove` = `27`
- `RemoveDuplicates` = `28`
- `RemoveIfNot` = `30`
- `RemoveKey` = `50`
- `RemoveNulls` = `29`
- `Reorder` = `39`
- `Reverse` = `31`
- `Sleep` = `32`
- `SomeFalse` = `33`
- `SomeNulls` = `34`
- `SomeTrue` = `35`
- `Sort` = `36`
- `SortIndexByValue` = `37`
- `SortPointer` = `38`
- `Sum` = `41`
- `ToStringFromArray` = `43`
- `ToStringFromArrayAndFormat` = `57`
- `ToStringFromObject` = `42`
- `ToStringFromObjectAndFormat` = `56`
- `Transpose` = `44`
- `Union` = `45`

