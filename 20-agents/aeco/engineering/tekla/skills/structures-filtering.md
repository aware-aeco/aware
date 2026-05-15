---
name: tekla-structures-filtering
description: Tekla Open API - Filtering namespace. Model object filters, filter expressions, search, query, find, select objects by property, BinaryFilterExpression, BinaryFilterOperatorType, StringFilterExpression, NumericFilterExpression. Filter parts, assemblies, bolts, welds, reinforcement by profile, material, class, name, phase, UDA values. This skill should be used when searching, filtering, or querying model objects programmatically.
---

# Tekla.Structures.Filtering API Reference (v2025)

## Overview

The Filtering namespace provides a programmatic way to build object filters that can be used with `ModelObjectSelector.GetObjectsByFilter()` or saved to filter files. Filters are composed of typed expressions (string, numeric, boolean, datetime) combined with binary operators. Filter expression categories provide pre-built expressions for each model object type (parts, assemblies, bolts, welds, etc.).

## Architecture

```
Filter
  └── FilterExpression (tree of expressions)
        ├── BinaryFilterExpression (leaf: left OP right)
        │     ├── Left:  TypedFilterExpression (e.g., PartFilterExpressions.Name)
        │     ├── Oper:  OperatorType (IS_EQUAL, CONTAINS, etc.)
        │     └── Right: ConstantFilterExpression (e.g., StringConstantFilterExpression("BEAM"))
        └── BinaryFilterExpressionCollection (linear chain of BinaryFilterExpressionItems)
              └── BinaryFilterExpressionItem (FilterExpression + AND/OR operator)
```

---

## Core Filter Classes

### Filter
> Top-level filter object. Wraps a FilterExpression and can save/load filter files.

**Constructors:**
- `Filter(FilterExpression FilterExpression)` - create from expression
- `Filter(string FullFileName, IFormatProvider Provider)` - load from file

**Properties:**
- `FilterExpression FilterExpression { get; set; }` - the root expression

**Methods:**
- `string CreateFile(FilterExpressionFileType FilterExpressionFileType, string FullFileName)` - save filter to file

---

### FilterExpression : Expression (abstract)
> Base class for all filter expressions.

**Properties:**
- `bool IsEnable { get; set; }` - enabled state of the expression

---

### Expression (abstract)
> Base class for all expressions in the filtering system.

---

### DataFilterExpression : Expression (abstract)
> Base class for typed data filter expressions (string, numeric, boolean, datetime).

---

### BinaryFilterExpression : FilterExpression
> A comparison between a typed expression (left) and a constant (right) using an operator. This is the primary building block of filters.

**Constructors:**
- `BinaryFilterExpression(BooleanFilterExpression Left, BooleanOperatorType Oper, BooleanConstantFilterExpression Right)`
- `BinaryFilterExpression(StringFilterExpression Left, StringOperatorType Oper, StringConstantFilterExpression Right)`
- `BinaryFilterExpression(NumericFilterExpression Left, NumericOperatorType Oper, NumericConstantFilterExpression Right)`
- `BinaryFilterExpression(DateTimeFilterExpression Left, DateTimeOperatorType Oper, DateTimeConstantFilterExpression Right)`

---

### BinaryFilterExpressionCollection : FilterExpression
> A linear chain of BinaryFilterExpressionItems combined with AND/OR operators. Use this to build multi-condition filters.

**Constructors:** `BinaryFilterExpressionCollection()`

**Properties:**
- `int Count { get; }`
- `bool IsReadOnly { get; }`
- `bool IsSynchronized { get; }`
- `BinaryFilterExpressionItem Item { get; set; }` - indexer
- `object SyncRoot { get; }`

**Methods:**
- `void Add(BinaryFilterExpressionItem Item)`
- `void Clear()`
- `bool Contains(BinaryFilterExpressionItem Item)`
- `void CopyTo(BinaryFilterExpressionItem[] Array, int ArrayIndex)`
- `BinaryFilterExpressionItem GetFirst()`
- `BinaryFilterExpressionItem GetLast()`
- `int IndexOf(BinaryFilterExpressionItem Item)`
- `void Insert(int Index, BinaryFilterExpressionItem Item)`
- `bool IsFirst(BinaryFilterExpressionItem Item)`
- `bool IsLast(BinaryFilterExpressionItem Item)`
- `bool Remove(BinaryFilterExpressionItem Item)`
- `void RemoveAt(int Index)`

---

### BinaryFilterExpressionItem : Expression
> Wraps a FilterExpression with an optional binary operator (AND/OR) for use in BinaryFilterExpressionCollection.

**Constructors:**
- `BinaryFilterExpressionItem(FilterExpression FilterExpression)` - last item (no operator)
- `BinaryFilterExpressionItem(FilterExpression FilterExpression, BinaryFilterOperatorType BinaryFilterItemOperatorType)` - item with AND/OR

---

## Operator Enums

### BinaryFilterOperatorType
> Operators between two filter items in a collection.

| Value | Int | Description |
|-------|-----|-------------|
| BOOLEAN_OR | 0 | OR |
| BOOLEAN_AND | 1 | AND |
| EMPTY | 2 | End of expression |

### StringOperatorType
| Value | Int | Description |
|-------|-----|-------------|
| IS_EQUAL | 1 | Equals |
| IS_NOT_EQUAL | 2 | Not equals |
| CONTAINS | 9 | Contains substring |
| NOT_CONTAINS | 10 | Does not contain |
| STARTS_WITH | 11 | Starts with |
| NOT_STARTS_WITH | 12 | Does not start with |
| ENDS_WITH | 13 | Ends with |
| NOT_ENDS_WITH | 14 | Does not end with |

### NumericOperatorType
| Value | Int | Description |
|-------|-----|-------------|
| IS_EQUAL | 1 | Equals |
| IS_NOT_EQUAL | 2 | Not equals |
| SMALLER_THAN | 5 | Less than |
| SMALLER_OR_EQUAL | 6 | Less than or equal |
| GREATER_THAN | 7 | Greater than |
| GREATER_OR_EQUAL | 8 | Greater than or equal |

### BooleanOperatorType
| Value | Int | Description |
|-------|-----|-------------|
| IS_EQUAL | 1 | Equals |
| IS_NOT_EQUAL | 2 | Not equals |
| BOOLEAN_OR | 3 | OR |
| BOOLEAN_AND | 4 | AND |

### DateTimeOperatorType
| Value | Int | Description |
|-------|-----|-------------|
| IS_EQUAL | 1 | Equals |
| IS_NOT_EQUAL | 2 | Not equals |
| EARLIER_THAN | 15 | Earlier than |
| EARLIER_OR_EQUAL | 16 | Earlier or equal |
| LATER_THAN | 17 | Later than |
| LATER_OR_EQUEL | 18 | Later or equal (note: typo in API) |

### OperatorType (unified)
> All operators in a single enum (used by InvalidFilterExpressionException).

| Value | Int | Description |
|-------|-----|-------------|
| IS_EQUAL | 1 | Equals |
| IS_NOT_EQUAL | 2 | Not equals |
| BOOLEAN_OR | 3 | OR |
| BOOLEAN_AND | 4 | AND |
| SMALLER_THAN | 5 | Less than |
| SMALLER_OR_EQUAL | 6 | Less than or equal |
| GREATER_THAN | 7 | Greater than |
| GREATER_OR_EQUAL | 8 | Greater than or equal |
| CONTAINS | 9 | Contains |
| NOT_CONTAINS | 10 | Not contains |
| STARTS_WITH | 11 | Starts with |
| NOT_STARTS_WITH | 12 | Not starts with |
| ENDS_WITH | 13 | Ends with |
| NOT_ENDS_WITH | 14 | Not ends with |
| EARLIER_THAN | 15 | Earlier than |
| EARLIER_OR_EQUAL | 16 | Earlier or equal |
| LATER_THAN | 17 | Later than |
| LATER_OR_EQUAL | 18 | Later or equal |

### FilterExpressionFileType
> Filter file types for saving.

| Value | Int | Description |
|-------|-----|-------------|
| OBJECT_GROUP_SELECTION | 0 | Selection filter |
| OBJECT_GROUP_VIEW | 1 | View filter |
| DRAWING_SINGLE_PART | 2 | Drawing single part filter |
| DRAWING_ASSEMBLY | 3 | Drawing assembly filter |
| DRAWING_CAST_UNIT | 4 | Drawing cast unit filter |
| DRAWING_GENERAL | 5 | Drawing general filter |

---

## Typed Expression Base Classes

### StringFilterExpression : DataFilterExpression
> Base for string filter expressions. Subclassed by category expressions like `PartFilterExpressions.Name`.

### NumericFilterExpression : DataFilterExpression
> Base for numeric filter expressions. Subclassed by category expressions like `PartFilterExpressions.Phase`.

### BooleanFilterExpression : DataFilterExpression
> Base for boolean filter expressions.

### DateTimeFilterExpression : DataFilterExpression
> Base for datetime filter expressions.

---

## Constant Expression Classes

### StringConstantFilterExpression : DataFilterExpression
> Constant string value for the right side of a BinaryFilterExpression.

**Constructors:** `StringConstantFilterExpression(string Value)`, `StringConstantFilterExpression(IEnumerable<string> Values)`

### NumericConstantFilterExpression : DataFilterExpression
> Constant numeric value. Accepts many numeric types and TeklaStructuresDatabaseTypeEnum.

**Constructors (key overloads):**
- `NumericConstantFilterExpression(int Value)`
- `NumericConstantFilterExpression(double Value)`
- `NumericConstantFilterExpression(TeklaStructuresDatabaseTypeEnum ObjectType)`
- `NumericConstantFilterExpression(IEnumerable<int> Values)`
- `NumericConstantFilterExpression(IEnumerable<double> Values)`
- `NumericConstantFilterExpression(IEnumerable<TeklaStructuresDatabaseTypeEnum> ObjectTypes)`
- Also accepts: Int16, UInt16, UInt32, long, UInt64, and IEnumerable variants, all with optional IFormatProvider

### BooleanConstantFilterExpression : DataFilterExpression
> Constant boolean value.

**Constructors:** `BooleanConstantFilterExpression(bool Value)`, `BooleanConstantFilterExpression(bool Value, IFormatProvider Provider)`

### DateTimeConstantFilterExpression : DataFilterExpression
> Constant DateTime value.

**Constructors:**
- `DateTimeConstantFilterExpression(DateTime Value)`
- `DateTimeConstantFilterExpression(IEnumerable<DateTime> Values)`
- `DateTimeConstantFilterExpression(DateTime Value, IFormatProvider Provider)`
- `DateTimeConstantFilterExpression(IEnumerable<DateTime> Values, IFormatProvider Provider)`

---

## Error Handling

### InvalidFilterExpressionException : Exception
> Thrown when a filter expression is invalid.

**Properties:**
- `Expression Expression { get; }` - the invalid expression
- `InvalidFilterExpressionExceptionReasonsType InvalidFilterExpressionExceptionReasonsType { get; }`
- `Expression LeftExpression { get; }` - left operand
- `OperatorType OperatorType { get; }` - operator
- `Expression RightExpression { get; }` - right operand

### InvalidFilterExpressionExceptionReasonsType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| TOO_MANY_NESTED_COLLECTIONS | 0 | Too many nested collections |

---

## Filter Expression Categories

All category classes live in `Tekla.Structures.Filtering.Categories`. Each category provides typed expression classes for specific model object properties. Every category also includes `CustomBoolean(string)`, `CustomDateTime(string)`, `CustomNumber(string)`, and `CustomString(string)` for user-defined attributes (UDAs).

### PartFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Class` | NumericFilterExpression | Part class number |
| `Finish` | StringFilterExpression | Surface finish |
| `Lot` | NumericFilterExpression | Lot number |
| `Material` | StringFilterExpression | Material name |
| `Name` | StringFilterExpression | Part name |
| `NumberingSeries` | StringFilterExpression | Numbering series |
| `Phase` | NumericFilterExpression | Phase number |
| `PositionNumber` | StringFilterExpression | Position number |
| `PourPhase` | NumericFilterExpression | Pour phase number |
| `Prefix` | StringFilterExpression | Position prefix |
| `PrimaryPart` | StringFilterExpression | Primary part flag |
| `Profile` | StringFilterExpression | Profile name |
| `StartNumber` | NumericFilterExpression | Start number |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### AssemblyFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Guid` | StringFilterExpression | Assembly GUID |
| `IdNumber` | StringFilterExpression | Identifier number |
| `Level` | StringFilterExpression | Level |
| `Name` | StringFilterExpression | Assembly name |
| `Phase` | StringFilterExpression | Phase |
| `PositionNumber` | StringFilterExpression | Position number |
| `Prefix` | StringFilterExpression | Position prefix |
| `Series` | StringFilterExpression | Series |
| `StartNumber` | StringFilterExpression | Start number |
| `Type` | StringFilterExpression | Assembly type |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### BoltFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Length` | NumericFilterExpression | Bolt length |
| `Phase` | NumericFilterExpression | Phase number |
| `SiteWorkshop` | StringFilterExpression | Site or workshop |
| `Size` | NumericFilterExpression | Bolt size |
| `Standard` | StringFilterExpression | Bolt standard |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### WeldFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Phase` | NumericFilterExpression | Phase number |
| `PositionNumber` | StringFilterExpression | Position number |
| `ReferenceText` | StringFilterExpression | Reference text |
| `SizeAboveLine` | NumericFilterExpression | Weld size above line |
| `SizeBelowLine` | NumericFilterExpression | Weld size below line |
| `TypeAboveLine` | NumericFilterExpression | Weld type above line |
| `TypeBelowLine` | NumericFilterExpression | Weld type below line |
| `WeldingSite` | NumericFilterExpression | Welding site |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### ReinforcingBarFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Class` | NumericFilterExpression | Rebar class |
| `Diameter` | NumericFilterExpression | Bar diameter |
| `JoinType` | StringFilterExpression | Join type |
| `Length` | NumericFilterExpression | Bar length |
| `Material` | StringFilterExpression | Material |
| `Name` | StringFilterExpression | Rebar name |
| `NumberingSeries` | StringFilterExpression | Numbering series |
| `Phase` | NumericFilterExpression | Phase number |
| `Position` | StringFilterExpression | Position |
| `PositionNumber` | StringFilterExpression | Position number |
| `Prefix` | StringFilterExpression | Position prefix |
| `Shape` | StringFilterExpression | Bar shape |
| `Size` | StringFilterExpression | Bar size |
| `StartNumber` | NumericFilterExpression | Start number |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### ComponentFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `ConnectionCode` | StringFilterExpression | Connection code |
| `Name` | StringFilterExpression | Component name |
| `Phase` | NumericFilterExpression | Phase number |
| `RunningNumber` | StringFilterExpression | Running number |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### ObjectFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Guid` | StringFilterExpression | Object GUID |
| `IdNumber` | NumericFilterExpression | Identifier number |
| `IsComponent` | NumericFilterExpression | Whether object is a component |
| `Phase` | NumericFilterExpression | Phase number |
| `Type` | NumericFilterExpression | Object type (use TeklaStructuresDatabaseTypeEnum) |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### ObjectTypesFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `CategoryName` | StringFilterExpression | Category name |
| `EntityName` | StringFilterExpression | Entity name |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### TaskFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `ActualEndDate` | DateTimeFilterExpression | Actual end date |
| `ActualStartDate` | DateTimeFilterExpression | Actual start date |
| `Completeness` | NumericFilterExpression | Task completeness % |
| `Critical` | StringFilterExpression | Critical flag |
| `Local` | StringFilterExpression | Local flag |
| `Name` | StringFilterExpression | Task name |
| `PlannedEndDate` | DateTimeFilterExpression | Planned end date |
| `PlannedStartDate` | DateTimeFilterExpression | Planned start date |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### SurfaceFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Class` | NumericFilterExpression | Surface class |
| `Name` | StringFilterExpression | Surface name |
| `Type` | StringFilterExpression | Surface type |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### ConstructionObjectFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Phase` | NumericFilterExpression | Phase number |
| `Type` | NumericFilterExpression | Object type |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### LoadFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Group` | StringFilterExpression | Load group |
| `Phase` | NumericFilterExpression | Phase number |
| `Type` | StringFilterExpression | Load type |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### LogicalAreaFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Building` | StringFilterExpression | Building name |
| `Section` | StringFilterExpression | Section name |
| `Site` | StringFilterExpression | Site name |
| `Story` | StringFilterExpression | Story name |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### PourObjectFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `ConcreteMixture` | StringFilterExpression | Concrete mixture |
| `Material` | StringFilterExpression | Material |
| `PourNumber` | StringFilterExpression | Pour number |
| `PourPhase` | StringFilterExpression | Pour phase |
| `PourType` | StringFilterExpression | Pour type |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### PourUnitFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| `Guid` | StringFilterExpression | Pour unit GUID |
| `Name` | StringFilterExpression | Pour unit name |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### PourBreakFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| *(no built-in expressions)* | | |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### ReferenceObjectFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| *(no built-in expressions)* | | |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

### TemplateFilterExpressions

| Expression | Base Type | Description |
|-----------|-----------|-------------|
| *(no built-in expressions)* | | |
| **Custom UDAs** | | |
| `CustomBoolean(string)` | BooleanFilterExpression | Custom boolean UDA |
| `CustomDateTime(string)` | DateTimeFilterExpression | Custom datetime UDA |
| `CustomNumber(string)` | NumericFilterExpression | Custom numeric UDA |
| `CustomString(string)` | StringFilterExpression | Custom string UDA |

---

## Common Usage Patterns

### Simple filter: parts with name "BEAM"
```csharp
var filter = new Filter(
    new BinaryFilterExpression(
        new PartFilterExpressions.Name(),
        StringOperatorType.IS_EQUAL,
        new StringConstantFilterExpression("BEAM")));
```

### Multi-condition filter: steel beams in phase 1
```csharp
var expressions = new BinaryFilterExpressionCollection();

expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new PartFilterExpressions.Name(),
        StringOperatorType.IS_EQUAL,
        new StringConstantFilterExpression("BEAM")),
    BinaryFilterOperatorType.BOOLEAN_AND));

expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new PartFilterExpressions.Phase(),
        NumericOperatorType.IS_EQUAL,
        new NumericConstantFilterExpression(1))));

var filter = new Filter(expressions);
```

### Filter by object type (e.g., only parts)
```csharp
var filter = new Filter(
    new BinaryFilterExpression(
        new ObjectFilterExpressions.Type(),
        NumericOperatorType.IS_EQUAL,
        new NumericConstantFilterExpression(
            TeklaStructuresDatabaseTypeEnum.PART)));
```

### Filter by custom UDA
```csharp
var filter = new Filter(
    new BinaryFilterExpression(
        new PartFilterExpressions.CustomString("USER_FIELD_1"),
        StringOperatorType.CONTAINS,
        new StringConstantFilterExpression("FOUNDATION")));
```

### Filter by profile name starting with "HEA"
```csharp
var filter = new Filter(
    new BinaryFilterExpression(
        new PartFilterExpressions.Profile(),
        StringOperatorType.STARTS_WITH,
        new StringConstantFilterExpression("HEA")));
```

### Save filter to file
```csharp
var filter = new Filter(expression);
filter.CreateFile(
    FilterExpressionFileType.OBJECT_GROUP_SELECTION,
    Path.Combine(modelPath, "attributes", "MyFilter.SObjGrp"));
```

### Load filter from file
```csharp
var filter = new Filter(
    Path.Combine(modelPath, "attributes", "MyFilter.SObjGrp"),
    CultureInfo.InvariantCulture);
FilterExpression expression = filter.FilterExpression;
```

---

## Advanced Filter Patterns

### Complete Filter + Collect Results Workflow

```csharp
var model = new Model();
var selector = model.GetModelObjectSelector();

// Build filter
var filter = new Filter(
    new BinaryFilterExpression(
        new PartFilterExpressions.Profile(),
        StringOperatorType.STARTS_WITH,
        new StringConstantFilterExpression("HEA")));

// Collect all matching objects
var results = new List<Part>();
var enumerator = selector.GetObjectsByFilter(filter.FilterExpression);
while (enumerator.MoveNext())
{
    if (enumerator.Current is Part part)
    {
        part.Select();
        results.Add(part);
    }
}
```

### Combine Multiple Object Type Filters (Beams OR Plates)

```csharp
var expressions = new BinaryFilterExpressionCollection();

// Beams
expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new ObjectFilterExpressions.Type(),
        NumericOperatorType.IS_EQUAL,
        new NumericConstantFilterExpression(
            TeklaStructuresDatabaseTypeEnum.PART)),
    BinaryFilterOperatorType.BOOLEAN_AND));

// AND profile starts with HEA (covers both beams and columns with HEA profiles)
expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new PartFilterExpressions.Profile(),
        StringOperatorType.STARTS_WITH,
        new StringConstantFilterExpression("HEA"))));

var filter = new Filter(expressions);
var enumerator = selector.GetObjectsByFilter(filter.FilterExpression);
```

### Filter by UDA + Built-in Property Combination

```csharp
var expressions = new BinaryFilterExpressionCollection();

// Built-in: material is S355JR
expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new PartFilterExpressions.Material(),
        StringOperatorType.IS_EQUAL,
        new StringConstantFilterExpression("S355JR")),
    BinaryFilterOperatorType.BOOLEAN_AND));

// UDA: custom string field contains "FOUNDATION"
expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new PartFilterExpressions.CustomString("USER_FIELD_1"),
        StringOperatorType.CONTAINS,
        new StringConstantFilterExpression("FOUNDATION"))));

var filter = new Filter(expressions);
```

### Per-Object Filter Check

```csharp
// Check if a single object matches a filter without enumerating
var filter = new Filter(filterExpression);

// ObjectMatchesToFilter works on individual objects
bool matches = Operation.ObjectMatchesToFilter(modelObject, filter);
if (matches)
{
    // Object passes the filter criteria
}
```

### Save and Load Filter Files

```csharp
// Save filter to selection filter file (.SObjGrp)
var model = new Model();
var modelPath = model.GetInfo().ModelPath;
var filterPath = Path.Combine(modelPath, "attributes", "MyCustomFilter.SObjGrp");

var filter = new Filter(filterExpression);
filter.CreateFile(FilterExpressionFileType.OBJECT_GROUP_SELECTION, filterPath);

// Load filter from file
var loadedFilter = new Filter(filterPath, CultureInfo.InvariantCulture);
var enumerator = selector.GetObjectsByFilter(loadedFilter.FilterExpression);
```
