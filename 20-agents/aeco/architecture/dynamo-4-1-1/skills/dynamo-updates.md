---
name: dynamo-dynamo-updates
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Updates namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BinaryVersion.
---

# Dynamo.Updates

Auto-generated from vendor docs for dynamo 4.1.1. 1 types in this namespace.

## BinaryVersion (class)

This class represents current version of the product.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

### Methods
#### `bool Equals(object other)`

Determines whether this instance and another specified Dynamo.Updates.BinaryVersion object are equal by reference

**Parameters:**
- `other` (object) — The Dynamo.Updates.BinaryVersion to compare to this instance.

**Returns:** `bool` — True if references are equal

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `Dynamo.Updates.BinaryVersion FromString(string version)`

Parses a given string version representation

**Parameters:**
- `version` (string) — Version to parse

**Returns:** `Dynamo.Updates.BinaryVersion` — Result Dynamo.Updates.BinaryVersion object

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `int GetHashCode()`

Returns the hash code for this Dynamo.Updates.BinaryVersion

**Returns:** `int` — The hash code

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `string ToString()`

Returns string representation of the Dynamo.Updates.BinaryVersion

**Returns:** `string` — String representation of the version

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `bool op_Equality(Dynamo.Updates.BinaryVersion lhs, Dynamo.Updates.BinaryVersion rhs)`

Compares two Dynamo.Updates.BinaryVersion objects. The result specifies if the versions are the same

**Parameters:**
- `lhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.
- `rhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.

**Returns:** `bool` — True if the versions are the same

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `bool op_GreaterThan(Dynamo.Updates.BinaryVersion lhs, Dynamo.Updates.BinaryVersion rhs)`

Compares two Dynamo.Updates.BinaryVersion objects. The result specifies if the first given version is newer than the second one

**Parameters:**
- `lhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.
- `rhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.

**Returns:** `bool` — True if the first version is newer than the second one

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `bool op_GreaterThanOrEqual(Dynamo.Updates.BinaryVersion lhs, Dynamo.Updates.BinaryVersion rhs)`

Compares two Dynamo.Updates.BinaryVersion objects. The result specifies if the first given version is newer or the same as the second one

**Parameters:**
- `lhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.
- `rhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.

**Returns:** `bool` — True if the first version is newer or the same as the second one

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `bool op_Inequality(Dynamo.Updates.BinaryVersion lhs, Dynamo.Updates.BinaryVersion rhs)`

Compares two Dynamo.Updates.BinaryVersion objects. The result specifies if the versions are different

**Parameters:**
- `lhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.
- `rhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.

**Returns:** `bool` — True if the versions are different

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `bool op_LessThan(Dynamo.Updates.BinaryVersion lhs, Dynamo.Updates.BinaryVersion rhs)`

Compares two Dynamo.Updates.BinaryVersion objects. The result specifies if the first given version is older than the second one

**Parameters:**
- `lhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.
- `rhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.

**Returns:** `bool` — True if the first version is older than the second one

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

#### `bool op_LessThanOrEqual(Dynamo.Updates.BinaryVersion lhs, Dynamo.Updates.BinaryVersion rhs)`

Compares two Dynamo.Updates.BinaryVersion objects. The result specifies if the first given version is older or the same as the second one

**Parameters:**
- `lhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.
- `rhs` (Dynamo.Updates.BinaryVersion) — A Dynamo.Updates.BinaryVersion to compare.

**Returns:** `bool` — True if the first version is older or the same as the second one

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Updates/BinaryVersion.cs)

