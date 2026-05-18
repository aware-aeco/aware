---
name: tekla-tekla-structures-filtering
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Filtering namespace — 24 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BinaryFilterExpression, BinaryFilterExpressionItem, BinaryFilterExpressionCollection, BooleanConstantFilterExpression, BooleanFilterExpression, DataFilterExpression, DateTimeConstantFilterExpression, DateTimeFilterExpression, and 16 more types.
---

# Tekla.Structures.Filtering

Auto-generated from vendor docs for tekla 2026.0. 24 types in this namespace.

## BinaryFilterExpression (class)

The BinaryFilterExpression class represents a binary expression between two filter expressions. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/10ff0aa1-1cac-1cde-56ea-5239f32794b0)

### Constructors
- `public BinaryFilterExpression(BooleanFilterExpression Left, BooleanOperatorType Oper, BooleanConstantFilterExpression Right)` — Initializes a new instance of the BinaryFilterExpression class.
- `public BinaryFilterExpression(DateTimeFilterExpression Left, DateTimeOperatorType Oper, DateTimeConstantFilterExpression Right)` — Initializes a new instance of the BinaryFilterExpression class.
- `public BinaryFilterExpression(NumericFilterExpression Left, NumericOperatorType Oper, NumericConstantFilterExpression Right)` — Initializes a new instance of the BinaryFilterExpression class.
- `public BinaryFilterExpression(StringFilterExpression Left, StringOperatorType Oper, StringConstantFilterExpression Right)` — Initializes a new instance of the BinaryFilterExpression class.

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/fece057a-34f0-fbb1-7184-27f66452ed40)

### Properties
- `IsEnable` (Boolean, get/set) — Gets or sets the enabled state of the filter expression.

## BinaryFilterExpressionCollection (class)

The BinaryFilterExpressionCollection class represents a collection of BinaryFilterItem objects. The class is used to represent linear expressions. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b265602f-8b11-715f-8159-73cbd7464c26)

### Constructors
- `public BinaryFilterExpressionCollection()` — Initializes a new instance of the BinaryFilterExpressionCollection class.

### Methods
#### `public void Add(BinaryFilterExpressionItem Item)`

Adds a BinaryFilterExpressionItem object at the end of the collection.

**Parameters:**
- `Item` (Tekla.Structures.Filtering.BinaryFilterExpressionItem) — The object to add to the collection. This value cannot be null.

[Docs](https://developer.tekla.com/topic/en/18/47/37a55e1e-c3d1-ab03-a52f-1f0856960257)

#### `public void Clear()`

Removes all the elements from the collection.

[Docs](https://developer.tekla.com/topic/en/18/47/d4879053-d92b-b022-d9a3-4d2381f59faa)

#### `public bool Contains(BinaryFilterExpressionItem Item)`

Determines whether an element is in the collection.

**Parameters:**
- `Item` (Tekla.Structures.Filtering.BinaryFilterExpressionItem) — The object to locate in the collection. This value cannot be null.

**Returns:** `Boolean` — True if the object exists.

[Docs](https://developer.tekla.com/topic/en/18/47/692fe680-5d97-a396-86e7-c53e541ca7c8)

#### `public void CopyTo(BinaryFilterExpressionItem[] Array, int ArrayIndex)`

Copies the collection to the input array.

**Parameters:**
- `Array` (.Tekla.Structures.Filtering.BinaryFilterExpressionItem.) — The input array. This value cannot be null.
- `ArrayIndex` (System.Int32) — The starting index.

[Docs](https://developer.tekla.com/topic/en/18/47/4aa802f3-e3bf-08cb-56ec-b5b6109cfbc3)

#### `public BinaryFilterExpressionItem GetFirst()`

Gets the first item of the collection.

**Returns:** `BinaryFilterExpressionItem` — The first item of the collection.

[Docs](https://developer.tekla.com/topic/en/18/47/67080931-72ad-1368-a331-8b823209cc19)

#### `public BinaryFilterExpressionItem GetLast()`

Gets the last item of the collection.

**Returns:** `BinaryFilterExpressionItem` — The last item of the collection.

[Docs](https://developer.tekla.com/topic/en/18/47/ed7ab484-7f6e-9cc3-73af-519a3ebe6d5e)

#### `public int IndexOf(BinaryFilterExpressionItem Item)`

Searches for the specified BinaryFilterExpressionItem in the collection and returns a zero-based index if found.

**Parameters:**
- `Item` (Tekla.Structures.Filtering.BinaryFilterExpressionItem) — The item to search for. This value cannot be null.

**Returns:** `Int32` — The zero-based index of the item in the collection. -1 if not found.

[Docs](https://developer.tekla.com/topic/en/18/47/11f12bd5-c925-b507-86f1-6d1f242ac214)

#### `public void Insert(int Index, BinaryFilterExpressionItem Item)`

Inserts an element in the collection at a specified index.

**Parameters:**
- `Index` (System.Int32) — The zero-based index at which the item shoud be inserted.
- `Item` (Tekla.Structures.Filtering.BinaryFilterExpressionItem) — The item to insert. This value cannot be null.

[Docs](https://developer.tekla.com/topic/en/18/47/ed34cf70-121c-8d04-16fd-3dff463b7ca7)

#### `public bool IsFirst(BinaryFilterExpressionItem BinaryFilterExpressionItem)`

Determines whether an element is the first item of the collection.

**Parameters:**
- `BinaryFilterExpressionItem` (Tekla.Structures.Filtering.BinaryFilterExpressionItem) — The element to evaluate. This value cannot be null.

**Returns:** `Boolean` — True if the element is the first item.

[Docs](https://developer.tekla.com/topic/en/18/47/5cb79128-858c-ecc0-f8e2-aec6aa31db13)

#### `public bool IsLast(BinaryFilterExpressionItem BinaryFilterExpressionItem)`

Determines whether an element is the last item of the collection.

**Parameters:**
- `BinaryFilterExpressionItem` (Tekla.Structures.Filtering.BinaryFilterExpressionItem) — The element to evaluate. This value cannot be null.

**Returns:** `Boolean` — True if the element is the last item.

[Docs](https://developer.tekla.com/topic/en/18/47/980afcfa-dae4-215d-1493-5caba0671bbd)

#### `public bool Remove(BinaryFilterExpressionItem Item)`

Removes the first occurrence of a specific object from the collection.

**Parameters:**
- `Item` (Tekla.Structures.Filtering.BinaryFilterExpressionItem) — The object to remove from the collection. This value cannot be null.

**Returns:** `Boolean` — True if item was successfully removed. False otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/4488a5f8-cfac-c06e-9537-badb4d390f35)

#### `public void RemoveAt(int Index)`

Removes the element at the specified index from the collection.

**Parameters:**
- `Index` (System.Int32) — The index of the item to remove.

[Docs](https://developer.tekla.com/topic/en/18/47/51945f78-fec1-8f3c-1c9e-13a0ef3eeb1b)

#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/fece057a-34f0-fbb1-7184-27f66452ed40)

### Properties
- `Count` (Int32, get) — Gets the number of elements actually contained in the collection.
- `IsEnable` (Boolean, get/set) — Gets or sets the enabled state of the filter expression.
- `IsReadOnly` (Boolean, get) — Gets a value indicating whether the collection is read-only.
- `IsSynchronized` (Boolean, get) — Gets a value indicating whether the collection supports multithreading.
- `Item` (object, get/set) — Gets or sets the item at a specific index.
- `SyncRoot` (Object, get) — Gets the root for synchronization.

## BinaryFilterExpressionItem (class)

The BinaryFilterExpressionItem class represents a data item in a BinaryFilterExpressionCollection. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/861f110e-1c4c-1fb8-8cfa-638e399f5425)

### Constructors
- `public BinaryFilterExpressionItem(FilterExpression FilterExpression)` — Initializes a new instance of the BinaryFilterExpressionItem class.
- `public BinaryFilterExpressionItem(FilterExpression FilterExpression, BinaryFilterOperatorType BinaryFilterItemOperatorType)` — Initializes a new instance of the BinaryFilterExpressionItem class.

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/fece057a-34f0-fbb1-7184-27f66452ed40)

## BinaryFilterOperatorType (enum)

The binary filter operator type defines the operators between two binary filters.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bd8993a4-6571-5c4f-d307-fbc742d65b20)

### Values
- `BOOLEAN_OR` = `0` — The Boolean OR operator.
- `BOOLEAN_AND` = `1` — The Boolean AND operator.
- `EMPTY` = `2` — The empty operator represents the end of the expression.

## BooleanConstantFilterExpression (class)

The BooleanConstantFilterExpression class represents a constant Boolean filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9fb6ce1a-0e8e-c0d0-fcc8-54b3f6a334cc)

### Constructors
- `public BooleanConstantFilterExpression(bool Value)` — Initializes a new instance of the BooleanConstantFilterExpression class.
- `public BooleanConstantFilterExpression(bool Value, IFormatProvider Provider)` — Initializes a new instance of the BooleanConstantFilterExpression class.

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## BooleanFilterExpression (class)

The BooleanFilterExpression class represents a Boolean filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/338c3d33-f44a-342f-13f7-484475847438)

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## BooleanOperatorType (enum)

The Boolean operator type defines the operators between two Boolean filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/58cc9e30-ca28-96dd-dadc-a3616f66acc6)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `BOOLEAN_OR` = `3` — The Boolean OR operator.
- `BOOLEAN_AND` = `4` — The Boolean AND operator.

## DataFilterExpression (class)

The DataFilterExpression class represents a basic data type for a filter expression. This is a base class for other filter expressions and cannot be used directly.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fdcacbf1-dd6d-ecf2-05a1-b34f063e3119)

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## DateTimeConstantFilterExpression (class)

The DateTimeConstantFilterExpression class represents a constant DateTime filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3f04e5be-a631-37df-c552-5e1ce8c0dd5c)

### Constructors
- `public DateTimeConstantFilterExpression(IEnumerable<DateTime> Values)` — Initializes a new instance of the DateTimeConstantFilterExpression class.
- `public DateTimeConstantFilterExpression(DateTime Value)` — Initializes a new instance of the DateTimeConstantFilterExpression class.
- `public DateTimeConstantFilterExpression(IEnumerable<DateTime> Values, IFormatProvider Provider)` — Initializes a new instance of the DateTimeConstantFilterExpression class.
- `public DateTimeConstantFilterExpression(DateTime Value, IFormatProvider Provider)` — Initializes a new instance of the DateTimeConstantFilterExpression class.

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## DateTimeFilterExpression (class)

The DateTimeFilterExpression class represents a DateTime filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c8c4806f-8921-fee5-eac4-1c86d648eb89)

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## DateTimeOperatorType (enum)

The DateTime operator type defines the operators between two DateTime filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/16c4c7db-83ad-5ed5-f606-d681354d5b74)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `EARLIER_THAN` = `15` — The "earlier than" operator.
- `EARLIER_OR_EQUAL` = `16` — The "earlier or equal" operator.
- `LATER_THAN` = `17` — The "later than" operator.
- `LATER_OR_EQUEL` = `18` — The "later or equal" operator.

## Expression (class)

The Expression class represents an expression. This is a base class for other expressions and cannot be used directly.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0f6107cc-f4dc-2d5b-2160-8a8912965187)

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/fece057a-34f0-fbb1-7184-27f66452ed40)

## Filter (class)

The Filter class creates a filter file based on the input FilterExpression object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3fe566dd-d704-1455-1c24-352a7d77ec85)

### Constructors
- `public Filter(FilterExpression FilterExpression)` — Initializes a new instance of the Filter class.
- `public Filter(string FullFileName, IFormatProvider Provider = null)` — Initializes a new instance of the Filter class.

### Methods
#### `public string CreateFile(FilterExpressionFileType FilterExpressionFileType, string FullFileName)`

Creates a filter and saves it as a text file.

**Parameters:**
- `FilterExpressionFileType` (Tekla.Structures.Filtering.FilterExpressionFileType) — The FilterExpressionFileType to be generated.
- `FullFileName` (System.String) — The full file name of the file to be saved.

**Returns:** `String` — A string containing the filter's full file name.

[Docs](https://developer.tekla.com/topic/en/18/47/ca0bf551-31d5-5d89-d73d-d247375a025c)

#### `public override string ToString()`

Returns the current FilterExpression as a string.

**Returns:** `String` — A string representation of the current FilterExpression.

[Docs](https://developer.tekla.com/topic/en/18/47/3084a925-8630-a8e8-fbbd-15b8ca9c8841)

### Properties
- `FilterExpression` (FilterExpression, get/set) — Gets the current FilterExpression instance.

## FilterExpression (class)

The FilterExpression class represents a filter expression. This is a base class for other filter expressions and should not be used directly.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2de0e25f-b8f2-75d1-7b6a-d548d2bd3868)

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/fece057a-34f0-fbb1-7184-27f66452ed40)

### Properties
- `IsEnable` (Boolean, get/set) — Gets or sets the enabled state of the filter expression.

## FilterExpressionFileType (enum)

The filter expression file type defines the filter expression file types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/85c8b3b7-5a40-6a3b-a4d8-23be21cd57fd)

### Values
- `OBJECT_GROUP_SELECTION` = `0` — The object group selection filter.
- `OBJECT_GROUP_VIEW` = `1` — The object group view filter.
- `DRAWING_SINGLE_PART` = `2` — The drawing single part filter.
- `DRAWING_ASSEMBLY` = `3` — The drawing assembly filter.
- `DRAWING_CAST_UNIT` = `4` — The drawing cast unit filter.
- `DRAWING_GENERAL` = `5` — The drawing general filter.

## InvalidFilterExpressionException (class)

The InvalidFilterExpressionException class represents an error that occurred during the expression evaluation. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/12183379-26b4-46ac-2d00-a72bfe9473e3)

### Constructors
- `public InvalidFilterExpressionException(Expression Expression, InvalidFilterExpressionExceptionReasonsType InvalidFilterExpressionExceptionReasonsType)` — Initializes a new instance of the InvalidFilterExpressionException class.
- `public InvalidFilterExpressionException(Expression Expression, InvalidFilterExpressionExceptionReasonsType InvalidFilterExpressionExceptionReasonsType, int MaximumExpressionNumber)` — Initializes a new instance of the InvalidFilterExpressionException class.
- `public InvalidFilterExpressionException(Expression LeftExpression, OperatorType OperatorType, Expression RightExpression, InvalidFilterExpressionExceptionReasonsType InvalidFilterExpressionExceptionReasonsType)` — Initializes a new instance of the InvalidFilterExpressionException class.

### Properties
- `Expression` (Expression, get) — Gets the invalid expression.
- `InvalidFilterExpressionExceptionReasonsType` (InvalidFilterExpressionExceptionReasonsType, get) — Gets the reason why the exception is thrown.
- `LeftExpression` (Expression, get) — Gets the invalid expression's left operand.
- `OperatorType` (OperatorType, get) — Gets the invalid expression's operator.
- `RightExpression` (Expression, get) — Gets the invalid expression's right operand.

## InvalidFilterExpressionExceptionReasonsType (enum)

The invalid filter expression exception reasons type defines the possible reasons for the InvalidFilterExpressionException.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/857010f3-9b55-4a4b-4c96-d0d53ece3e56)

### Values
- `TOO_MANY_NESTED_COLLECTIONS` = `0` — There are too many nested collections.

## NumericConstantFilterExpression (class)

The NumericConstantFilterExpression class represents a constant numeric filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6c797a9c-2d1c-a857-d1f0-82e78555e217)

### Constructors
- `public NumericConstantFilterExpression(IEnumerable<double> Values)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<short> Values)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<int> Values)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<long> Values)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<ushort> Values)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<uint> Values)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<ulong> Values)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(double Value)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(short Value)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(int Value)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(long Value)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(ushort Value)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(uint Value)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(ulong Value)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<TeklaStructuresDatabaseTypeEnum> ObjectTypes)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(TeklaStructuresDatabaseTypeEnum ObjectType)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<double> Values, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<short> Values, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<int> Values, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<long> Values, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<ushort> Values, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<uint> Values, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(IEnumerable<ulong> Values, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(double Value, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(short Value, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(int Value, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(long Value, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(ushort Value, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(uint Value, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `public NumericConstantFilterExpression(ulong Value, IFormatProvider Provider)` — Initializes a new instance of the NumericConstantFilterExpression class.

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## NumericFilterExpression (class)

The NumericFilterExpression class represents a numeric filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/13b949a0-ebf7-827a-54e3-f44b5ada00f8)

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## NumericOperatorType (enum)

The numeric operator type defines the operators between two numeric filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a6a61b2f-6808-ae6d-3c9c-009922276362)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `SMALLER_THAN` = `5` — The "smaller than" operator.
- `SMALLER_OR_EQUAL` = `6` — The "smaller or equal" operator.
- `GREATER_THAN` = `7` — The "greater than" operator.
- `GREATER_OR_EQUAL` = `8` — The "greater or equal" operator.

## OperatorType (enum)

The operator type defines the operators between two filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/774cc6c1-7fab-2418-4d35-59069d90d7c3)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `BOOLEAN_OR` = `3` — The Boolean OR operator.
- `BOOLEAN_AND` = `4` — The Boolean AND operator.
- `SMALLER_THAN` = `5` — The "smaller than" operator.
- `SMALLER_OR_EQUAL` = `6` — The "smaller or equal" operator.
- `GREATER_THAN` = `7` — The "greater than" operator.
- `GREATER_OR_EQUAL` = `8` — The "greater or equal" operator.
- `CONTAINS` = `9` — The "contains" string operator.
- `NOT_CONTAINS` = `10` — The "not contains" string operator.
- `STARTS_WITH` = `11` — The "starts with" string operator.
- `NOT_STARTS_WITH` = `12` — The "not starts with" string operator.
- `ENDS_WITH` = `13` — The "ends with" string operator.
- `NOT_ENDS_WITH` = `14` — The "not ends with" string operator.
- `EARLIER_THAN` = `15` — The "earlier than" operator.
- `EARLIER_OR_EQUAL` = `16` — The "earlier or equal" operator.
- `LATER_THAN` = `17` — The "later than" operator.
- `LATER_OR_EQUAL` = `18` — The "later or equal" operator.

## StringConstantFilterExpression (class)

The StringConstantFilterExpression class represents a constant string filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1ec9a93f-5ba3-47a6-2336-7185597983dc)

### Constructors
- `public StringConstantFilterExpression(IEnumerable<string> Values)` — Initializes a new instance of the StringConstantFilterExpression class.
- `public StringConstantFilterExpression(string Value)` — Initializes a new instance of the StringConstantFilterExpression class.

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## StringFilterExpression (class)

The StringFilterExpression class represents a string filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2ad423fa-08aa-b2e7-0053-07ee753a700d)

### Methods
#### `public override string ToString()`

Creates a string representation of the current object.

**Returns:** `String` — A new string representing the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## StringOperatorType (enum)

The string operator type defines the operators between two string filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8942df33-7ed7-d235-7edc-2a45579ef569)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `CONTAINS` = `9` — The "contains" string operator.
- `NOT_CONTAINS` = `10` — The "not contains" string operator.
- `STARTS_WITH` = `11` — The "starts with" string operator.
- `NOT_STARTS_WITH` = `12` — The "not starts with" string operator.
- `ENDS_WITH` = `13` — The "ends with" string operator.
- `NOT_ENDS_WITH` = `14` — The "not ends with" string operator.

