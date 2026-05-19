---
name: navisworks-autodesk-navisworks-api-schema
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api.Schema namespace — 22 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BooleanField, ComplexField, ConceptCollection, DoubleField, EditAccessor, Field, FieldCollection, GroupField, and 14 more types.
---

# Autodesk.Navisworks.Api.Schema

Auto-generated from vendor docs for navisworks 2026.0. 22 types in this namespace.

## BooleanField (class)

Represents the definition of a Boolean value within a SchemaDefinition.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.BooleanField)

### Constructors
- `BooleanField()` — Default constructor. No identifier specified.
- `BooleanField(string id)` — Constructs the field with a given identifier.

### Methods
#### `bool GetValue(Autodesk.Navisworks.Api.Schema.ReadAccessor accessor)`

Gets a value from a given SchemaData via the associated ReadAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.BooleanField.GetValue%28Autodesk.Navisworks.Api.Schema.ReadAccessor%29)

#### `static Autodesk.Navisworks.Api.Schema.BooleanField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of BooleanField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.BooleanField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.BooleanField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of BooleanField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.BooleanField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void SetValue(Autodesk.Navisworks.Api.Schema.EditAccessor accessor, bool value)`

Sets a value into a given SchemaData via the associated EditAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.EditAccessor)
- `value` (bool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.BooleanField.SetValue%28Autodesk.Navisworks.Api.Schema.EditAccessor%2CSystem.Boolean%29)

### Properties
- `DefaultValue` (bool, get/set) — Default value that created SchemaData objects will contain at creation time.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## ComplexField (class)

Intermediate base class for all complex structural Fields.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.ComplexField)

### Methods
#### `static Autodesk.Navisworks.Api.Schema.ComplexField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ComplexField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.ComplexField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ComplexField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ComplexField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ComplexField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## ConceptCollection (class)

A collection of concept strings.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.ConceptCollection)

### Constructors
- `ConceptCollection()` — Create an empty collection.
- `ConceptCollection(Autodesk.Navisworks.Api.Schema.ConceptCollection from)` — Create one collection from another.

### Methods
#### `void Add(string item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.Add%28System.String%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<string> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<string>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.AddRange%28System.Collections.Generic.IEnumerable%7BSystem.String%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.Clear)

#### `bool Contains(string item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.Contains%28System.String%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<string> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<string>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BSystem.String%7D%29)

#### `void CopyTo(System.Collections.Generic.ICollection<string> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<string>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.CopyTo%28System.Collections.Generic.ICollection%7BSystem.String%7D%29)

#### `void CopyTo(string[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (string[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.CopyTo%28System.String%5B%5D%2CSystem.Int32%29)

#### `System.Collections.Generic.IEnumerator<string> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<string>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.GetEnumerator)

#### `int IndexOf(string item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (string)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.IndexOf%28System.String%29)

#### `void Insert(int index, string item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.Insert%28System.Int32%2CSystem.String%29)

#### `static Autodesk.Navisworks.Api.Schema.ConceptCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ConceptCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.ConceptCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ConceptCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(string item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (string) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.Remove%28System.String%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ConceptCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (string, get/set) — Item property of ConceptCollection.

## DoubleField (class)

Represents the definition of a Double value within a SchemaDefinition.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.DoubleField)

### Constructors
- `DoubleField()` — Default constructor. No identifier specified.
- `DoubleField(string id)` — Constructs the field with a given identifier.

### Methods
#### `double GetValue(Autodesk.Navisworks.Api.Schema.ReadAccessor accessor)`

Gets a value from a given SchemaData via the associated ReadAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.DoubleField.GetValue%28Autodesk.Navisworks.Api.Schema.ReadAccessor%29)

#### `static Autodesk.Navisworks.Api.Schema.DoubleField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of DoubleField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.DoubleField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.DoubleField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of DoubleField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.DoubleField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void SetValue(Autodesk.Navisworks.Api.Schema.EditAccessor accessor, double value)`

Sets a value into a given SchemaData via the associated EditAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.EditAccessor)
- `value` (double)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.DoubleField.SetValue%28Autodesk.Navisworks.Api.Schema.EditAccessor%2CSystem.Double%29)

### Properties
- `DefaultValue` (double, get/set) — Default value that created SchemaData objects will contain at creation time.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `UnitGroup` (Autodesk.Navisworks.Api.Schema.UnitGroup, get/set) — Represents the unit group (eg. length, volume, angle, etc) of the field value.

## EditAccessor (class)

Provides editing access into a SchemaData via an associated Field. Represents a given position inside SchemaData.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.EditAccessor)

### Methods
#### `static Autodesk.Navisworks.Api.Schema.EditAccessor InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of EditAccessor.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.EditAccessor` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.EditAccessor.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of EditAccessor.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.EditAccessor.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## Field (class)

Base class for all fields used within SchemaDefinitions.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.Field)

### Methods
#### `static Autodesk.Navisworks.Api.Schema.Field InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Field.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.Field` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Field.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Field.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Field.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.Schema.Field value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.Schema.Field)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Field.ValueEquals%28Autodesk.Navisworks.Api.Schema.Field%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.Schema.Field value1, Autodesk.Navisworks.Api.Schema.Field value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Schema.Field)
- `value2` (Autodesk.Navisworks.Api.Schema.Field)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Field.ValueEquals%28Autodesk.Navisworks.Api.Schema.Field%2CAutodesk.Navisworks.Api.Schema.Field%29)

### Properties
- `Concepts` (Autodesk.Navisworks.Api.Schema.ConceptCollection, get) — Field-specific semantics associated with this field.
- `DisplayNameId` (string, get/set) — The field's display name internal ID. Used for looking up the localized display name.
- `Id` (string, get/set) — The identifier for the field.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## FieldCollection (class)

A collection of Fields.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.FieldCollection)

### Constructors
- `FieldCollection()` — Create an empty collection.
- `FieldCollection(Autodesk.Navisworks.Api.Schema.FieldCollection from)` — Create one collection from another.

### Methods
#### `void Add(Autodesk.Navisworks.Api.Schema.Field item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Schema.Field)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.Add%28Autodesk.Navisworks.Api.Schema.Field%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Schema.Field> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Schema.Field>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Schema.Field%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.Schema.Field item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Schema.Field)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.Contains%28Autodesk.Navisworks.Api.Schema.Field%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Schema.Field> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Schema.Field>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Schema.Field%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.Schema.Field[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.Schema.Field[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.CopyTo%28Autodesk.Navisworks.Api.Schema.Field%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Schema.Field> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Schema.Field>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.Schema.Field%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Schema.Field> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Schema.Field>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.Schema.Field item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Schema.Field)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.IndexOf%28Autodesk.Navisworks.Api.Schema.Field%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.Schema.Field item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.Schema.Field)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.Schema.Field%29)

#### `static Autodesk.Navisworks.Api.Schema.FieldCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of FieldCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.FieldCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of FieldCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.Schema.Field item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Schema.Field) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.Remove%28Autodesk.Navisworks.Api.Schema.Field%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.FieldCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.Schema.Field, get/set) — Item property of FieldCollection.

## GroupField (class)

Intermediate base class for all complex group Fields.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.GroupField)

### Methods
#### `static Autodesk.Navisworks.Api.Schema.GroupField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GroupField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.GroupField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.GroupField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GroupField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.GroupField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## GuidField (class)

Represents the definition of a Guid value within a SchemaDefinition.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.GuidField)

### Constructors
- `GuidField(Autodesk.Navisworks.Api.Schema.GuidFieldOwnershipType ownership)` — Default constructor. No identifier specified.
- `GuidField(string id)` — Constructs the field with a given identifier.
- `GuidField(string id, Autodesk.Navisworks.Api.Schema.GuidFieldOwnershipType ownership)` — Constructs the field with a given identifier.

### Methods
#### `System.Guid GetValue(Autodesk.Navisworks.Api.Schema.ReadAccessor accessor)`

Gets a value from a given SchemaData via the associated ReadAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor)

**Returns:** `System.Guid` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.GuidField.GetValue%28Autodesk.Navisworks.Api.Schema.ReadAccessor%29)

#### `static Autodesk.Navisworks.Api.Schema.GuidField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GuidField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.GuidField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.GuidField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GuidField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.GuidField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void SetValue(Autodesk.Navisworks.Api.Schema.EditAccessor accessor, System.Guid value)`

Sets a value into a given SchemaData via the associated EditAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.EditAccessor)
- `value` (System.Guid)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.GuidField.SetValue%28Autodesk.Navisworks.Api.Schema.EditAccessor%2CSystem.Guid%29)

### Properties
- `DefaultValue` (System.Guid, get/set) — Default value that created SchemaData objects will contain at creation time.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `OwnershipSemantics` (Autodesk.Navisworks.Api.Schema.GuidFieldOwnershipType, get/set) — Ownership semantics for this GUID.
- `TargetConcept` (string, get/set) — Target concept, if any, that we expect the referenced resource to implement.

## GuidFieldOwnershipType (enum)

Ownership semantics for a given GUID.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.GuidFieldOwnershipType)

### Values
- `Generic` = `0` — GUID has no implied meaning.
- `Identifier` = `1` — Uniquely identifies a given resource. There are no ownership implications as this is on the resource itself.
- `UniqueReference` = `2` — A unique reference to another resource. Ownership is implicit as no other resource can own this reference.
- `SharedReference` = `3` — A shared reference to another resource. Ownership is akin to reference counting.
- `WeakReference` = `4` — A reference to another resource that explicitly has no ownership. The reference may be invalid (i.e. the resource could be missing).
- `OwnerReference` = `5` — A reference to another resource that explicitly has direct ownership. Similar to UniqueReference but may be referenced by WeakReference GUIDs.
- `ExternalReference` = `6` — A reference to an external resource that is not specified via Identifier.

## Int32Field (class)

Represents the definition of a Int32 value within a SchemaDefinition.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.Int32Field)

### Constructors
- `Int32Field()` — Default constructor. No identifier specified.
- `Int32Field(string id)` — Constructs the field with a given identifier.

### Methods
#### `int GetValue(Autodesk.Navisworks.Api.Schema.ReadAccessor accessor)`

Gets a value from a given SchemaData via the associated ReadAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Int32Field.GetValue%28Autodesk.Navisworks.Api.Schema.ReadAccessor%29)

#### `static Autodesk.Navisworks.Api.Schema.Int32Field InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Int32Field.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.Int32Field` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Int32Field.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Int32Field.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Int32Field.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void SetValue(Autodesk.Navisworks.Api.Schema.EditAccessor accessor, int value)`

Sets a value into a given SchemaData via the associated EditAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.EditAccessor)
- `value` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.Int32Field.SetValue%28Autodesk.Navisworks.Api.Schema.EditAccessor%2CSystem.Int32%29)

### Properties
- `DefaultValue` (int, get/set) — Default value that created SchemaData objects will contain at creation time.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## ReadAccessor (class)

Provides read-only access into a SchemaData via an associated Field. Represents a given position inside SchemaData.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.ReadAccessor)

### Methods
#### `static Autodesk.Navisworks.Api.Schema.ReadAccessor InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ReadAccessor.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.ReadAccessor` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ReadAccessor.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ReadAccessor.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ReadAccessor.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool IsValid(Autodesk.Navisworks.Api.Schema.SchemaData instance)`

Tests to see if this accessor is associated with a given SchemaData.

**Parameters:**
- `instance` (Autodesk.Navisworks.Api.Schema.SchemaData)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.ReadAccessor.IsValid%28Autodesk.Navisworks.Api.Schema.SchemaData%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## SchemaBuilder (class)

Enables construction of SchemaDefinitions.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.SchemaBuilder)

### Constructors
- `SchemaBuilder()` — Default constructor.

### Methods
#### `void AddConcept(string concpt)`

Adds a given concept string onto the schema definition under construction.

**Parameters:**
- `concpt` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaBuilder.AddConcept%28System.String%29)

#### `void AddField(Autodesk.Navisworks.Api.Schema.Field field)`

Copies & adds in the field to the builder's internal collection.

**Parameters:**
- `field` (Autodesk.Navisworks.Api.Schema.Field)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaBuilder.AddField%28Autodesk.Navisworks.Api.Schema.Field%29)

#### `Autodesk.Navisworks.Api.Schema.SchemaDefinition BuildDefinition()`

Constructs a schema definition matching what has been passed into Field) & String).

**Returns:** `Autodesk.Navisworks.Api.Schema.SchemaDefinition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaBuilder.BuildDefinition)

#### `static Autodesk.Navisworks.Api.Schema.SchemaBuilder InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SchemaBuilder.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.SchemaBuilder` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaBuilder.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SchemaBuilder.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaBuilder.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## SchemaData (class)

Contains data values defined by a given schema definition. May be traversed manually or accessed via NET DLR dynamic typing.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.SchemaData)

### Methods
#### `Autodesk.Navisworks.Api.Schema.SchemaData Copy()`

Creates a copy of the current state of this data object.

**Returns:** `Autodesk.Navisworks.Api.Schema.SchemaData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaData.Copy)

#### `static Autodesk.Navisworks.Api.Schema.SchemaData InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SchemaData.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.SchemaData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaData.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SchemaData.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaData.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `int ValueCompares(Autodesk.Navisworks.Api.Schema.SchemaData rhs)`

Performs a value comparison.

**Parameters:**
- `rhs` (Autodesk.Navisworks.Api.Schema.SchemaData)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaData.ValueCompares%28Autodesk.Navisworks.Api.Schema.SchemaData%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.Schema.SchemaData value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.Schema.SchemaData)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaData.ValueEquals%28Autodesk.Navisworks.Api.Schema.SchemaData%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.Schema.SchemaData value1, Autodesk.Navisworks.Api.Schema.SchemaData value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Schema.SchemaData)
- `value2` (Autodesk.Navisworks.Api.Schema.SchemaData)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaData.ValueEquals%28Autodesk.Navisworks.Api.Schema.SchemaData%2CAutodesk.Navisworks.Api.Schema.SchemaData%29)

### Properties
- `Definition` (Autodesk.Navisworks.Api.Schema.SchemaDefinition, get) — Schema definition that defines this data object.
- `EditAccessor` (Autodesk.Navisworks.Api.Schema.EditAccessor, get) — Accessor for editing values when beginning a query into this data object.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `ReadAccessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor, get) — Read-only accessor for beginning a query into this data object.

## SchemaDefinition (class)

Schema definitions are used to define data structures that are then created as schema data objects for use with various Navisworks API calls.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.SchemaDefinition)

### Methods
#### `Autodesk.Navisworks.Api.Schema.SchemaData CreateData()`

Creates a concrete data object to hold values structured according to the Fields within this definition.

**Returns:** `Autodesk.Navisworks.Api.Schema.SchemaData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinition.CreateData)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinition.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinition.GetHashCode)

#### `static Autodesk.Navisworks.Api.Schema.SchemaDefinition InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SchemaDefinition.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.SchemaDefinition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinition.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SchemaDefinition.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinition.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.Schema.SchemaDefinition value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.Schema.SchemaDefinition)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinition.ValueEquals%28Autodesk.Navisworks.Api.Schema.SchemaDefinition%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.Schema.SchemaDefinition value1, Autodesk.Navisworks.Api.Schema.SchemaDefinition value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Schema.SchemaDefinition)
- `value2` (Autodesk.Navisworks.Api.Schema.SchemaDefinition)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinition.ValueEquals%28Autodesk.Navisworks.Api.Schema.SchemaDefinition%2CAutodesk.Navisworks.Api.Schema.SchemaDefinition%29)

### Properties
- `Concepts` (Autodesk.Navisworks.Api.Schema.ConceptCollection, get) — The concepts that this definition implements.
- `Fields` (Autodesk.Navisworks.Api.Schema.FieldCollection, get) — The top level of Fields within this definition.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## SchemaDefinitionCollection (class)

A collection of Definitions.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.SchemaDefinitionCollection)

### Constructors
- `SchemaDefinitionCollection()` — Create an empty collection.
- `SchemaDefinitionCollection(Autodesk.Navisworks.Api.Schema.SchemaDefinitionCollection from)` — Create one collection from another.

### Methods
#### `bool Contains(Autodesk.Navisworks.Api.Schema.SchemaDefinition item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Schema.SchemaDefinition)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinitionCollection.Contains%28Autodesk.Navisworks.Api.Schema.SchemaDefinition%29)

#### `void CopyTo(Autodesk.Navisworks.Api.Schema.SchemaDefinition[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.Schema.SchemaDefinition[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinitionCollection.CopyTo%28Autodesk.Navisworks.Api.Schema.SchemaDefinition%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Schema.SchemaDefinition> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Schema.SchemaDefinition>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinitionCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.Schema.SchemaDefinition%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Schema.SchemaDefinition> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Schema.SchemaDefinition>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinitionCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.Schema.SchemaDefinition item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Schema.SchemaDefinition)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinitionCollection.IndexOf%28Autodesk.Navisworks.Api.Schema.SchemaDefinition%29)

#### `static Autodesk.Navisworks.Api.Schema.SchemaDefinitionCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SchemaDefinitionCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.SchemaDefinitionCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinitionCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SchemaDefinitionCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SchemaDefinitionCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.Schema.SchemaDefinition, get) — Item property of SchemaDefinitionCollection.

## SimpleField (class)

Intermediate base class for all simple value Fields.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.SimpleField)

### Methods
#### `static Autodesk.Navisworks.Api.Schema.SimpleField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SimpleField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.SimpleField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SimpleField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SimpleField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.SimpleField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## StringField (class)

Represents the definition of a String value within a SchemaDefinition.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.StringField)

### Constructors
- `StringField()` — Default constructor. No identifier specified.
- `StringField(string id)` — Constructs the field with a given identifier.

### Methods
#### `string GetValue(Autodesk.Navisworks.Api.Schema.ReadAccessor accessor)`

Gets a value from a given SchemaData via the associated ReadAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.StringField.GetValue%28Autodesk.Navisworks.Api.Schema.ReadAccessor%29)

#### `static Autodesk.Navisworks.Api.Schema.StringField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of StringField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.StringField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.StringField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of StringField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.StringField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void SetValue(Autodesk.Navisworks.Api.Schema.EditAccessor accessor, string value)`

Sets a value into a given SchemaData via the associated EditAccessor.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.EditAccessor)
- `value` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.StringField.SetValue%28Autodesk.Navisworks.Api.Schema.EditAccessor%2CSystem.String%29)

### Properties
- `DefaultValue` (string, get/set) — Default value that created SchemaData objects will contain at creation time.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## StructField (class)

Represents the definition of a structure within a SchemaDefinition.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.StructField)

### Constructors
- `StructField()` — Default constructor. No identifier specified.
- `StructField(string id)` — Constructs the field with a given identifier.

### Methods
#### `static Autodesk.Navisworks.Api.Schema.StructField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of StructField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.StructField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.StructField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of StructField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.StructField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Fields` (Autodesk.Navisworks.Api.Schema.FieldCollection, get) — The fields that comprise the struct content.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## UnitGroup (enum)

Represents the unit group (eg. length, volume, angle, etc) of the field value.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.UnitGroup)

### Values
- `None` = `0` — No unit group specified (default).
- `Length` = `8192` — Unit group is length.
- `Area` = `12288` — Unit group is area.
- `Volume` = `16384` — Unit group is volume.
- `Angle` = `32768` — Unit group is angle.

## VectorField (class)

Represents the definition of a vector within a SchemaDefinition.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.VectorField)

### Constructors
- `VectorField(Autodesk.Navisworks.Api.Schema.Field value)` — Default constructor. No identifier specified.
- `VectorField(string id, Autodesk.Navisworks.Api.Schema.Field value)` — Constructs the field with a given identifier.

### Methods
#### `int GetCount(Autodesk.Navisworks.Api.Schema.ReadAccessor accessor)`

Obtains the number of values held within a distinct SchemaData object.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor) — Accessor provided by parent object e.g. from SchemaData, or a containing VectorField's IndexAccessor.

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.VectorField.GetCount%28Autodesk.Navisworks.Api.Schema.ReadAccessor%29)

#### `Autodesk.Navisworks.Api.Schema.EditAccessor GetIndexAccessor(Autodesk.Navisworks.Api.Schema.EditAccessor accessor, int index)`

Obtains the appropriate accessor for a given index into the vector.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.EditAccessor) — Accessor provided by parent object e.g. from SchemaData, or a containing VectorField's IndexAccessor.
- `index` (int)

**Returns:** `Autodesk.Navisworks.Api.Schema.EditAccessor` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.VectorField.GetIndexAccessor%28Autodesk.Navisworks.Api.Schema.EditAccessor%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.Schema.ReadAccessor GetIndexAccessor(Autodesk.Navisworks.Api.Schema.ReadAccessor accessor, int index)`

Obtains the appropriate accessor for a given index into the vector.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.ReadAccessor) — Accessor provided by parent object e.g. from SchemaData, or a containing VectorField's IndexAccessor.
- `index` (int)

**Returns:** `Autodesk.Navisworks.Api.Schema.ReadAccessor` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.VectorField.GetIndexAccessor%28Autodesk.Navisworks.Api.Schema.ReadAccessor%2CSystem.Int32%29)

#### `static Autodesk.Navisworks.Api.Schema.VectorField InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of VectorField.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Schema.VectorField` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.VectorField.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of VectorField.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.VectorField.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void SetCount(Autodesk.Navisworks.Api.Schema.EditAccessor accessor, int count)`

Sets the number of values held within a distinct SchemaData object. Each index will be initialised to its default value.

**Parameters:**
- `accessor` (Autodesk.Navisworks.Api.Schema.EditAccessor) — Accessor provided by parent object e.g. from SchemaData, or a containing VectorField's IndexAccessor.
- `count` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Schema.VectorField.SetCount%28Autodesk.Navisworks.Api.Schema.EditAccessor%2CSystem.Int32%29)

### Properties
- `CollectionType` (Autodesk.Navisworks.Api.Schema.VectorFieldCollectionType, get/set) — Represents collection metadata, including whether the collection is ordered, and whether the elements are unique.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `ValueField` (Autodesk.Navisworks.Api.Schema.Field, get) — A field object that identifies the type used within the vector & provides access to each index.

## VectorFieldCollectionType (enum)

Represents collection metadata, including whether the collection is ordered, and whether the elements are unique.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Schema.VectorFieldCollectionType)

### Values
- `Unordered` = `0` — Preserving element order is not important. (Default)
- `Ordered` = `1` — Preserving element order is important.
- `UnorderedUnique` = `2` — Preserving element order is not important and elements must be unique.
- `OrderedUnique` = `3` — Preserving element order is important and elements must be unique.

