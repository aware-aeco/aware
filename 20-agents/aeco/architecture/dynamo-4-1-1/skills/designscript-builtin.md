---
name: dynamo-designscript-builtin
description: This skill encodes the dynamo 4.1.1 surface of the DesignScript.Builtin namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Get, Dictionary, KeyNotFoundException, IndexOutOfRangeException, StringOverIndexingException, BuiltinNullReferenceException.
---

# DesignScript.Builtin

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## BuiltinNullReferenceException (class)

Null reference exception thrown with null DS builtin types: lists, dictionaries and strings.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/IndexingExceptions.cs)

### Constructors
- `void BuiltinNullReferenceException(string message)` — BuiltinNullReferenceException.BuiltinNullReferenceException

## Dictionary (class)

Type Dictionary

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Dictionary.cs)

### Methods
#### `DesignScript.Builtin.Dictionary ByKeysValues(System.Collections.Generic.IList<string> keys, System.Collections.Generic.IList<object> values)`

Produces a Dictionary with the supplied keys and values. The number of entries is the shorter of keys or values.

**Parameters:**
- `keys` (System.Collections.Generic.IList<string>) — Keys of dictionary
- `values` (System.Collections.Generic.IList<object>) — Values of dictionary

**Returns:** `DesignScript.Builtin.Dictionary` — Dictionary from keys and values

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Dictionary.cs)

#### `System.Collections.Generic.IDictionary<string, object> Components()`

Produces the components of a Dictionary. The reverse of Dictionary.ByKeysValues.

**Returns:** `System.Collections.Generic.IDictionary<string, object>` — Values of the dictionary

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Dictionary.cs)

#### `DesignScript.Builtin.Dictionary RemoveKeys(System.Collections.Generic.IList<string> keys)`

Produce a new Dictionary with the given keys removed.

**Parameters:**
- `keys` (System.Collections.Generic.IList<string>) — The key in the Dictionary to remove

**Returns:** `DesignScript.Builtin.Dictionary` — New dictionary with keys removed

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Dictionary.cs)

#### `DesignScript.Builtin.Dictionary SetValueAtKeys(System.Collections.Generic.IList<string> keys, System.Collections.Generic.IList<object> values)`

Produce a new Dictionary with a list of keys set to the new values, possibly overwriting existing key-value pairs. These two lists are expected to be of the same length. If not, the shorter of the two bounds the number of insertions.

**Parameters:**
- `keys` (System.Collections.Generic.IList<string>) — The keys in the Dictionary to set. If the same key already exists, the value at that key will be modified.
- `values` (System.Collections.Generic.IList<object>) — The corresponding values to insert.

**Returns:** `DesignScript.Builtin.Dictionary` — New dictionary with the entries inserted

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Dictionary.cs)

#### `string ToString()`

Returns a friendly string representation of the dictionary.

**Returns:** `string` — String representation of the dictionary.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Dictionary.cs)

#### `object ValueAtKey(string key)`

Obtain the value at a specified key

**Parameters:**
- `key` (string) — The key in the Dictionary to obtain value for

**Returns:** `object` — Value at the specified key or null if it is not set

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Dictionary.cs)

### Properties
- `Count` (int, get) — The number of key value pairs in a Dictionary.
- `Keys` (System.Collections.Generic.IEnumerable<string>, get) — Produces the keys in a Dictionary.
- `Values` (System.Collections.Generic.IEnumerable<object>, get) — Produces the values in a Dictionary.

## Get (static-class)

Type Get

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Builtin.cs)

### Methods
#### `object ValueAtIndex(DesignScript.Builtin.Dictionary dictionary, string key)`

Get.ValueAtIndex

**Parameters:**
- `dictionary` (DesignScript.Builtin.Dictionary)
- `key` (string)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Builtin.cs)

#### `object ValueAtIndex(System.Collections.IList list, int index)`

Get.ValueAtIndex

**Parameters:**
- `list` (System.Collections.IList)
- `index` (int)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Builtin.cs)

#### `object ValueAtIndex(string stringList, int index)`

Get.ValueAtIndex

**Parameters:**
- `stringList` (string)
- `index` (int)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Builtin.cs)

#### `object ValueAtIndexInForLoop(System.Collections.IList list, int index)`

Get.ValueAtIndexInForLoop

**Parameters:**
- `list` (System.Collections.IList)
- `index` (int)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Builtin.cs)

#### `object ValueAtIndexInForLoop(string stringList, int index)`

Get.ValueAtIndexInForLoop

**Parameters:**
- `stringList` (string)
- `index` (int)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/Builtin.cs)

## IndexOutOfRangeException (class)

Type IndexOutOfRangeException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/IndexingExceptions.cs)

### Constructors
- `void IndexOutOfRangeException(string message)` — IndexOutOfRangeException.IndexOutOfRangeException

## KeyNotFoundException (class)

Type KeyNotFoundException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/IndexingExceptions.cs)

### Constructors
- `void KeyNotFoundException(string message)` — KeyNotFoundException.KeyNotFoundException

## StringOverIndexingException (class)

Type StringOverIndexingException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Libraries/DesignScriptBuiltin/IndexingExceptions.cs)

### Constructors
- `void StringOverIndexingException(string message)` — StringOverIndexingException.StringOverIndexingException

