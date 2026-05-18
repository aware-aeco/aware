---
name: tekla-tekla-structures-filtering
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Filtering namespace — 24 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BinaryFilterExpression, BinaryFilterExpressionCollection, BinaryFilterExpressionItem, BooleanConstantFilterExpression, BooleanFilterExpression, DataFilterExpression, DateTimeConstantFilterExpression, Expression, and 16 more types.
---

# Tekla.Structures.Filtering

Auto-generated from vendor docs for tekla 2025.0. 24 types in this namespace.

## BinaryFilterExpression (class)

The BinaryFilterExpression class represents a binary expression between two filter expressions. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/10ff0aa1-1cac-1cde-56ea-5239f32794b0)

### Constructors
- `BinaryFilterExpression(...)` — Initializes a new instance of the BinaryFilterExpression class.
- `BinaryFilterExpression(...)` — Initializes a new instance of the BinaryFilterExpression class.
- `BinaryFilterExpression(...)` — Initializes a new instance of the BinaryFilterExpression class.
- `BinaryFilterExpression(...)` — Initializes a new instance of the BinaryFilterExpression class.

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/fece057a-34f0-fbb1-7184-27f66452ed40)

### Properties
- `IsEnable` (object, get/set) — Gets or sets the enabled state of the filter expression.

## BinaryFilterExpressionCollection (class)

The BinaryFilterExpressionCollection class represents a collection of BinaryFilterItem objects. The class is used to represent linear expressions. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b265602f-8b11-715f-8159-73cbd7464c26)

### Constructors
- `BinaryFilterExpressionCollection(...)` — Initializes a new instance of the BinaryFilterExpressionCollection class.

### Methods
#### `Add(...)`

Adds a BinaryFilterExpressionItem object at the end of the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/37a55e1e-c3d1-ab03-a52f-1f0856960257)

#### `Clear(...)`

Removes all the elements from the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/d4879053-d92b-b022-d9a3-4d2381f59faa)

#### `Contains(...)`

Determines whether an element is in the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/692fe680-5d97-a396-86e7-c53e541ca7c8)

#### `CopyTo(...)`

Copies the collection to the input array.

[Docs](https://developer.tekla.com/topic/en/18/43/4aa802f3-e3bf-08cb-56ec-b5b6109cfbc3)

#### `GetFirst(...)`

Gets the first item of the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/67080931-72ad-1368-a331-8b823209cc19)

#### `GetLast(...)`

Gets the last item of the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/ed7ab484-7f6e-9cc3-73af-519a3ebe6d5e)

#### `IndexOf(...)`

Searches for the specified BinaryFilterExpressionItem in the collection and returns a zero-based index if found.

[Docs](https://developer.tekla.com/topic/en/18/43/11f12bd5-c925-b507-86f1-6d1f242ac214)

#### `Insert(...)`

Inserts an element in the collection at a specified index.

[Docs](https://developer.tekla.com/topic/en/18/43/ed34cf70-121c-8d04-16fd-3dff463b7ca7)

#### `IsFirst(...)`

Determines whether an element is the first item of the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/5cb79128-858c-ecc0-f8e2-aec6aa31db13)

#### `IsLast(...)`

Determines whether an element is the last item of the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/980afcfa-dae4-215d-1493-5caba0671bbd)

#### `Remove(...)`

Removes the first occurrence of a specific object from the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/4488a5f8-cfac-c06e-9537-badb4d390f35)

#### `RemoveAt(...)`

Removes the element at the specified index from the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/51945f78-fec1-8f3c-1c9e-13a0ef3eeb1b)

#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/fece057a-34f0-fbb1-7184-27f66452ed40)

### Properties
- `Count` (object, get/set) — Gets the number of elements actually contained in the collection.
- `IsEnable` (object, get/set) — Gets or sets the enabled state of the filter expression.
- `IsReadOnly` (object, get/set) — Gets a value indicating whether the collection is read-only.
- `IsSynchronized` (object, get/set) — Gets a value indicating whether the collection supports multithreading.
- `Item` (object, get/set) — Gets or sets the item at a specific index.
- `SyncRoot` (object, get/set) — Gets the root for synchronization.

## BinaryFilterExpressionItem (class)

The BinaryFilterExpressionItem class represents a data item in a BinaryFilterExpressionCollection. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/861f110e-1c4c-1fb8-8cfa-638e399f5425)

### Constructors
- `BinaryFilterExpressionItem(...)` — Initializes a new instance of the BinaryFilterExpressionItem class.
- `BinaryFilterExpressionItem(...)` — Initializes a new instance of the BinaryFilterExpressionItem class.

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/fece057a-34f0-fbb1-7184-27f66452ed40)

## BinaryFilterOperatorType (enum)

The binary filter operator type defines the operators between two binary filters.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bd8993a4-6571-5c4f-d307-fbc742d65b20)

### Values
- `BOOLEAN_OR` = `0` — The Boolean OR operator.
- `BOOLEAN_AND` = `1` — The Boolean AND operator.
- `EMPTY` = `2` — The empty operator represents the end of the expression.

## BooleanConstantFilterExpression (class)

The BooleanConstantFilterExpression class represents a constant Boolean filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9fb6ce1a-0e8e-c0d0-fcc8-54b3f6a334cc)

### Constructors
- `BooleanConstantFilterExpression(...)` — Initializes a new instance of the BooleanConstantFilterExpression class.
- `BooleanConstantFilterExpression(...)` — Initializes a new instance of the BooleanConstantFilterExpression class.

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## BooleanFilterExpression (class)

The BooleanFilterExpression class represents a Boolean filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/338c3d33-f44a-342f-13f7-484475847438)

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## BooleanOperatorType (enum)

The Boolean operator type defines the operators between two Boolean filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/58cc9e30-ca28-96dd-dadc-a3616f66acc6)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `BOOLEAN_OR` = `3` — The Boolean OR operator.
- `BOOLEAN_AND` = `4` — The Boolean AND operator.

## DataFilterExpression (class)

The DataFilterExpression class represents a basic data type for a filter expression. This is a base class for other filter expressions and cannot be used directly.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fdcacbf1-dd6d-ecf2-05a1-b34f063e3119)

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## DateTimeConstantFilterExpression (class)

The DateTimeConstantFilterExpression class represents a constant DateTime filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3f04e5be-a631-37df-c552-5e1ce8c0dd5c)

### Constructors
- `DateTimeConstantFilterExpression(...)` — Initializes a new instance of the DateTimeConstantFilterExpression class.
- `DateTimeConstantFilterExpression(...)` — Initializes a new instance of the DateTimeConstantFilterExpression class.
- `DateTimeConstantFilterExpression(...)` — Initializes a new instance of the DateTimeConstantFilterExpression class.
- `DateTimeConstantFilterExpression(...)` — Initializes a new instance of the DateTimeConstantFilterExpression class.

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## DateTimeFilterExpression (class)

The DateTimeFilterExpression class represents a DateTime filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c8c4806f-8921-fee5-eac4-1c86d648eb89)

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## DateTimeOperatorType (enum)

The DateTime operator type defines the operators between two DateTime filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/16c4c7db-83ad-5ed5-f606-d681354d5b74)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `EARLIER_THAN` = `15` — The "earlier than" operator.
- `EARLIER_OR_EQUAL` = `16` — The "earlier or equal" operator.
- `LATER_THAN` = `17` — The "later than" operator.
- `LATER_OR_EQUEL` = `18` — The "later or equal" operator.

## Expression (class)

The Expression class represents an expression. This is a base class for other expressions and cannot be used directly.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0f6107cc-f4dc-2d5b-2160-8a8912965187)

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/fece057a-34f0-fbb1-7184-27f66452ed40)

## Filter (class)

The Filter class creates a filter file based on the input FilterExpression object.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3fe566dd-d704-1455-1c24-352a7d77ec85)

### Constructors
- `Filter(...)` — Initializes a new instance of the Filter class.
- `Filter(...)` — Initializes a new instance of the Filter class.

### Methods
#### `CreateFile(...)`

Creates a filter and saves it as a text file.

[Docs](https://developer.tekla.com/topic/en/18/43/ca0bf551-31d5-5d89-d73d-d247375a025c)

#### `ToString(...)`

Returns the current FilterExpression as a string.

[Docs](https://developer.tekla.com/topic/en/18/43/3084a925-8630-a8e8-fbbd-15b8ca9c8841)

### Properties
- `FilterExpression` (object, get/set) — Gets the current FilterExpression instance.

## FilterExpression (class)

The FilterExpression class represents a filter expression. This is a base class for other filter expressions and should not be used directly.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2de0e25f-b8f2-75d1-7b6a-d548d2bd3868)

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/fece057a-34f0-fbb1-7184-27f66452ed40)

### Properties
- `IsEnable` (object, get/set) — Gets or sets the enabled state of the filter expression.

## FilterExpressionFileType (enum)

The filter expression file type defines the filter expression file types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/85c8b3b7-5a40-6a3b-a4d8-23be21cd57fd)

### Values
- `OBJECT_GROUP_SELECTION` = `0` — The object group selection filter.
- `OBJECT_GROUP_VIEW` = `1` — The object group view filter.
- `DRAWING_SINGLE_PART` = `2` — The drawing single part filter.
- `DRAWING_ASSEMBLY` = `3` — The drawing assembly filter.
- `DRAWING_CAST_UNIT` = `4` — The drawing cast unit filter.
- `DRAWING_GENERAL` = `5` — The drawing general filter.

## InvalidFilterExpressionException (class)

The InvalidFilterExpressionException class represents an error that occurred during the expression evaluation. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/12183379-26b4-46ac-2d00-a72bfe9473e3)

### Constructors
- `InvalidFilterExpressionException(...)` — Initializes a new instance of the InvalidFilterExpressionException class.
- `InvalidFilterExpressionException(...)` — Initializes a new instance of the InvalidFilterExpressionException class.
- `InvalidFilterExpressionException(...)` — Initializes a new instance of the InvalidFilterExpressionException class.

### Properties
- `Expression` (object, get/set) — Gets the invalid expression.
- `InvalidFilterExpressionExceptionReasonsType` (object, get/set) — Gets the reason why the exception is thrown.
- `LeftExpression` (object, get/set) — Gets the invalid expression's left operand.
- `OperatorType` (object, get/set) — Gets the invalid expression's operator.
- `RightExpression` (object, get/set) — Gets the invalid expression's right operand.

## InvalidFilterExpressionExceptionReasonsType (enum)

The invalid filter expression exception reasons type defines the possible reasons for the InvalidFilterExpressionException.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/857010f3-9b55-4a4b-4c96-d0d53ece3e56)

### Values
- `TOO_MANY_NESTED_COLLECTIONS` = `0` — There are too many nested collections.

## NumericConstantFilterExpression (class)

The NumericConstantFilterExpression class represents a constant numeric filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6c797a9c-2d1c-a857-d1f0-82e78555e217)

### Constructors
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.
- `NumericConstantFilterExpression(...)` — Initializes a new instance of the NumericConstantFilterExpression class.

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## NumericFilterExpression (class)

The NumericFilterExpression class represents a numeric filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/13b949a0-ebf7-827a-54e3-f44b5ada00f8)

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## NumericOperatorType (enum)

The numeric operator type defines the operators between two numeric filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a6a61b2f-6808-ae6d-3c9c-009922276362)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `SMALLER_THAN` = `5` — The "smaller than" operator.
- `SMALLER_OR_EQUAL` = `6` — The "smaller or equal" operator.
- `GREATER_THAN` = `7` — The "greater than" operator.
- `GREATER_OR_EQUAL` = `8` — The "greater or equal" operator.

## OperatorType (enum)

The operator type defines the operators between two filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/774cc6c1-7fab-2418-4d35-59069d90d7c3)

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

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1ec9a93f-5ba3-47a6-2336-7185597983dc)

### Constructors
- `StringConstantFilterExpression(...)` — Initializes a new instance of the StringConstantFilterExpression class.
- `StringConstantFilterExpression(...)` — Initializes a new instance of the StringConstantFilterExpression class.

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## StringFilterExpression (class)

The StringFilterExpression class represents a string filter expression.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2ad423fa-08aa-b2e7-0053-07ee753a700d)

### Methods
#### `ToString(...)`

Creates a string representation of the current object.

[Docs](https://developer.tekla.com/topic/en/18/43/c0294f6e-dcc4-d468-68e5-c1563f7927c4)

## StringOperatorType (enum)

The string operator type defines the operators between two string filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8942df33-7ed7-d235-7edc-2a45579ef569)

### Values
- `IS_EQUAL` = `1` — The "is equal" operator.
- `IS_NOT_EQUAL` = `2` — The "is not equal" operator.
- `CONTAINS` = `9` — The "contains" string operator.
- `NOT_CONTAINS` = `10` — The "not contains" string operator.
- `STARTS_WITH` = `11` — The "starts with" string operator.
- `NOT_STARTS_WITH` = `12` — The "not starts with" string operator.
- `ENDS_WITH` = `13` — The "ends with" string operator.
- `NOT_ENDS_WITH` = `14` — The "not ends with" string operator.

