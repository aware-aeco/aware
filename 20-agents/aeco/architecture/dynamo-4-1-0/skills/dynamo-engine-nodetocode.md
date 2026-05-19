---
name: dynamo-dynamo-engine-nodetocode
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Engine.NodeToCode namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: INamingProvider.
---

# Dynamo.Engine.NodeToCode

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## INamingProvider (interface)

Node to code will create some variables, and if it is able to know the type of expression, it could give a more meaningful variable prefix.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/NodeToCode/NodeToCode.cs)

### Methods
#### `string GetTypeDependentName(ProtoCore.Type type)`

Returns a name for specified type. This name will be used as the prefix for variable that created in node to code. It should return a empty string if fails to generate one.

**Parameters:**
- `type` (ProtoCore.Type) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/NodeToCode/NodeToCode.cs)

