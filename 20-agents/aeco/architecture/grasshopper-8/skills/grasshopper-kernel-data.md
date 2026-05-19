---
name: grasshopper-grasshopper-kernel-data
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Kernel.Data namespace — 33 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_BracketMismatchException, GH_IndexRanges, GH_IndexRuleSet, GH_Lexer, GH_LexerCombo, GH_Path, GH_Path.PathLengthComparer, GH_PathOffset, and 25 more types.
---

# Grasshopper.Kernel.Data

Auto-generated from vendor docs for grasshopper 8.0. 33 types in this namespace.

## GH_BracketMismatchException (class)

Exception used during Rule Set parsing.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_BracketMismatchException.htm)

### Constructors
- `public GH_BracketMismatchException()` — Initializes a new instance of the GH_BracketMismatchException class
- `public GH_BracketMismatchException(string message)` — Initializes a new instance of the GH_BracketMismatchException class
- `public GH_BracketMismatchException(string message, int location)` — Initializes a new instance of the GH_BracketMismatchException class

### Properties
- `Index` (Int32, get) — 

## GH_ExpandMode (enum)

Enumerates the possible expansion modes for structure paths.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_ExpandMode.htm)

### Values
- `None` = `0` — Indicates no expansion should take place.
- `SimpleReplace` = `1` — Paths are expanded and collisions are erased.
- `SimpleAppend` = `2` — Paths are expanded and merged with colliding paths.
- `Recursive` = `3` — Before a path is expanded, any collision is expanded first.

## GH_GraftMode (enum)

Enumerates all predefined grafting modes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_GraftMode.htm)

### Values
- `None` = `0` — Don't include null items or empty lists.
- `GraftNullItems` = `1` — Null items are included in the grafted tree.
- `GraftEmptyLists` = `2` — Empty lists are included in the grafted tree.
- `GraftAll` = `3` — Null items and empty lists are both included in the grafted tree.

## GH_IndexRange (struct)

Represents a range of indices.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_IndexRange.htm)

### Constructors
- `public GH_IndexRange(int index)` — Create a new singular range.
- `public GH_IndexRange(int index0, int index1)` — Create a new range.

### Methods
#### `public bool AdjacentTo(GH_IndexRange range)`

Tests whether this range is adjacent to another.

**Parameters:**
- `range` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to test.

**Returns:** `Boolean` — True if the ranges are adjacent and not intersecting.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_AdjacentTo.htm)

#### `public bool Contains(GH_IndexRange range)`

Tests whether a specified range is entirely contained within this range.

**Parameters:**
- `range` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to test.

**Returns:** `Boolean` — True if the range is inside this one.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_Contains.htm)

#### `public bool Contains(int index)`

Tests whether a specified index is defined by this range.

**Parameters:**
- `index` (System.Int32) — Index to test.

**Returns:** `Boolean` — True if the index is inside this range.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_Contains_1.htm)

#### `public static GH_IndexRange Intersection(GH_IndexRange range0, GH_IndexRange range1)`

Create the intersection between two ranges.

**Parameters:**
- `range0` (Grasshopper.Kernel.Data.GH_IndexRange) — First range to intersect.
- `range1` (Grasshopper.Kernel.Data.GH_IndexRange) — Second range to intersect.

**Returns:** `GH_IndexRange` — The intersection or an invalid range if the input doesn't intersect.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_Intersection.htm)

#### `public bool IntersectsWith(GH_IndexRange range)`

Tests whether certain indices are described by both this and another range.

**Parameters:**
- `range` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to intersect with.

**Returns:** `Boolean` — True if the ranges intersect.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_IntersectsWith.htm)

#### `public static int Split(GH_IndexRange range, GH_IndexRange splitter, out GH_IndexRange result0, out GH_IndexRange result1)`

Split a range into two using an integer index.

**Parameters:**
- `range` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to split.
- `splitter` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to split with.
- `result0` (Grasshopper.Kernel.Data.GH_IndexRange) — First split result if successful, or an invalid range if not.
- `result1` (Grasshopper.Kernel.Data.GH_IndexRange) — Second split result if successful, or an invalid range if not.

**Returns:** `Int32` — The number of valid split results.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_Split.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_ToString.htm)

#### `public static GH_IndexRange Union(GH_IndexRange range0, GH_IndexRange range1)`

Create a range which contains both input ranges.

**Parameters:**
- `range0` (Grasshopper.Kernel.Data.GH_IndexRange)
- `range1` (Grasshopper.Kernel.Data.GH_IndexRange)

**Returns:** `GH_IndexRange` — Smallest range big enough to contain both range0 and range1.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRange_Union.htm)

### Properties
- `Index0` (Int32, get) — Gets the first index in the range.
- `Index1` (Int32, get) — Gets the last index in the range.
- `InvalidRange` (GH_IndexRange, get) — Gets the predefined invalid range.
- `IsSingular` (Boolean, get) — Gets whether the range has zero length.
- `IsValid` (Boolean, get) — Gets whether this range is valid.
- `Length` (Int32, get) — Gets the length of the range.
- `MaxValue` (GH_IndexRange, get) — 

## GH_IndexRanges (class)

Represents a sorted, non-colliding collection of index ranges.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_IndexRanges.htm)

### Constructors
- `public GH_IndexRanges()` — Initializes a new instance of the GH_IndexRanges class

### Methods
#### `public bool InsertRange(GH_IndexRange range)`

Insert an index range into the collection.

**Parameters:**
- `range` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to insert.

**Returns:** `Boolean` — True if the collection was meaningfully modified.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRanges_InsertRange.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRanges_Read.htm)

#### `public bool RemoveRange(GH_IndexRange range)`

Remove an index range from the collection.

**Parameters:**
- `range` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to remove.

**Returns:** `Boolean` — True if the collection was meaningfully modified.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRanges_RemoveRange.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRanges_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRanges_Write.htm)

### Properties
- `Count` (Int32, get) — Gets the number of ranges stored in this collection.
- `Range` (GH_IndexRange, get) — Gets the range at the given index.

## GH_IndexRuleSet (class)

A collection of index rules.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_IndexRuleSet.htm)

### Constructors
- `public GH_IndexRuleSet()` — Blank constructor. Creates an empty collection of rules.

### Methods
#### `public void AddAnyDigitRule()`

Append a rule for any single digit.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddAnyDigitRule.htm)

#### `public void AddAnyDigitsRule()`

Append a rule for any amount of digits.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddAnyDigitsRule.htm)

#### `public void AddDigitPatternRule(int firstDigit, int nextDigit)`

Append a rule for filtering digit patterns. This pattern does not have an upper bound.

**Parameters:**
- `firstDigit` (System.Int32) — First digit in pattern.
- `nextDigit` (System.Int32) — Next digit in pattern.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddDigitPatternRule.htm)

#### `public void AddDigitPatternRule(int firstDigit, int nextDigit, int lastDigit)`

Append a rule for filtering digit patterns.

**Parameters:**
- `firstDigit` (System.Int32) — First digit in pattern.
- `nextDigit` (System.Int32) — Next digit in pattern.
- `lastDigit` (System.Int32) — Last digit in pattern.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddDigitPatternRule_1.htm)

#### `public void AddDigitPatternRule(int[] pattern)`

Append a rule for filtering digit patterns.

**Parameters:**
- `pattern` (System.Int32[]) — Complete pattern. A complete pattern consists of at least three integers, the first one signifying the pattern lower bound, the last one signifying the patters upper bound (use Int32.MaxValue is you do not want to enforce an upper bound), and at least one integer in between defining the pattern growth.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddDigitPatternRule_2.htm)

#### `public void AddDigitRule(int digit, bool invert)`

Append a single digit rule.

**Parameters:**
- `digit` (System.Int32) — Digit to filter.
- `invert` (System.Boolean) — If true, the digit will be filtered against rather than for.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddDigitRule.htm)

#### `public void AddRangePatternRule(GH_IndexRange[] pattern)`

Append a rule for filtering range patterns.

**Parameters:**
- `pattern` (Grasshopper.Kernel.Data.GH_IndexRange[]) — Complete pattern. A complete pattern consists of at least three ranges, the first one signifying the pattern lower bound, the last one signifying the patters upper bound (use GH_IndexRange.MaxValue is you do not want to enforce an upper bound), and at least one range in between defining the pattern growth.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddRangePatternRule.htm)

#### `public void AddRangeRule(GH_IndexRange range, bool invert)`

Append a single range rule.

**Parameters:**
- `range` (Grasshopper.Kernel.Data.GH_IndexRange) — Range to filter.
- `invert` (System.Boolean) — If true, the range will be filtered against rather than for.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_AddRangeRule.htm)

#### `public GH_RuleResult Evaluate(int index)`

Evaluate the index given the local rules.

**Parameters:**
- `index` (System.Int32) — Index to evaluate.

**Returns:** `GH_RuleResult` — The result of the evaluation.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_IndexRuleSet_Evaluate.htm)

### Properties
- `Count` (Int32, get) — Gets the number of rules defined in this collection.
- `Rule` (IGH_IndexRule, get) — Gets the rule at the specified index.

## GH_Lexer (class)

Represents a lexical mask for path operations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_Lexer.htm)

### Constructors
- `public GH_Lexer()` — Blank constructor.
- `public GH_Lexer(string mask)` — Create a new mask from a textual string.

### Methods
#### `public bool EvaluatePath(GH_ExpressionParser evaluator, ref GH_Path path, ref int index)`

Evaluate the variables in this mask with the provided expression solver.

**Parameters:**
- `evaluator` (Grasshopper.Kernel.Expressions.GH_ExpressionParser) — Evaluator to use. Must have all variables properly set up.
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path result.
- `index` (System.Int32) — Index result. If the lexer contains no item representation then the index remains unmolested.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Lexer_EvaluatePath.htm)

#### `public static void PerformLexicalReplace<T>(GH_Lexer source, GH_Lexer target, DataTree<T> tree_in, DataTree<T> tree_out)`

Lexical Find..Replace operations.

**Parameters:**
- `source` (Grasshopper.Kernel.Data.GH_Lexer) — Search mask.
- `target` (Grasshopper.Kernel.Data.GH_Lexer) — Replace pattern.
- `tree_in` (Grasshopper.DataTree<T>) — Source data tree.
- `tree_out` (Grasshopper.DataTree<T>) — Target data tree.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Lexer_PerformLexicalReplace__1.htm)

#### `public static void PerformLexicalReplace<T>(GH_Lexer source, GH_Lexer target, IGH_Structure structure_in, GH_Structure<T> structure_out) where T : IGH_Goo`

Lexical Find..Replace operations.

**Parameters:**
- `source` (Grasshopper.Kernel.Data.GH_Lexer) — Search mask.
- `target` (Grasshopper.Kernel.Data.GH_Lexer) — Replace pattern.
- `structure_in` (Grasshopper.Kernel.Data.IGH_Structure) — Source data tree.
- `structure_out` (Grasshopper.Kernel.Data.GH_Structure<T>) — Target data tree.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Lexer_PerformLexicalReplace__1_1.htm)

### Properties
- `IsItem` (Boolean, get) — Gets a value indicating whether or not the item portion of the lexer has been set.
- `IsPath` (Boolean, get) — Gets a value indicating whether or not the path portion of the lexer has been set.
- `Item` (String, get) — Gets the item string that was extracted from the mask. The default is "append"
- `Mask` (String, get) — Gets the original string that represents the user input for this lexer.
- `Path` (List<String>, get) — Gets the list of path strings that was extracted from the mask. If the mask hasn't been parsed yet, it will be parsed now. If the returned list is still nothing, parsing failed.

## GH_LexerCombo (class)

Contains both a source and a target lexer object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_LexerCombo.htm)

### Constructors
- `public GH_LexerCombo()` — Initializes a new instance of the GH_LexerCombo class
- `public GH_LexerCombo(GH_Lexer n_source, GH_Lexer n_target)` — Initializes a new instance of the GH_LexerCombo class
- `public GH_LexerCombo(string n_source, string n_target)` — Initializes a new instance of the GH_LexerCombo class

### Properties
- `Source` (GH_Lexer, get/set) — Gets or sets the lexer object that represents the source of a lexical operation.
- `Target` (GH_Lexer, get/set) — Gets or sets the lexer object that represents the target of a lexical operation.

## GH_Path (class)

Describes the path in structure space of a data item or a list of items. A path consists of a series of integers, each one of which represents an index in a branch structure.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_Path.htm)

### Constructors
- `public GH_Path()` — Default constructor, creates a path with zero elements.
- `public GH_Path(GH_Path Other)` — Creates an exact copy of another path.
- `public GH_Path(int index)` — Creates a new path with a single element.
- `public GH_Path(params int[] args)` — Creates a new path from a series of elements.

### Methods
#### `public GH_Path AppendElement(int index)`

Create a new path by appending a new index value to this path.

**Parameters:**
- `index` (System.Int32) — Index value to append.

**Returns:** `GH_Path` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_AppendElement.htm)

#### `public int Compare(GH_Path x, GH_Path y)`

Compare two paths. This function determines the Sorting behaviour of paths.

**Parameters:**
- `x` (Grasshopper.Kernel.Data.GH_Path)
- `y` (Grasshopper.Kernel.Data.GH_Path)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_Compare.htm)

#### `public int CompareTo(GH_Path other)`

Compare this path to another path. This function determines the Sorting behaviour of paths.

**Parameters:**
- `other` (Grasshopper.Kernel.Data.GH_Path)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_CompareTo.htm)

#### `public GH_Path CullElement()`

Removes the last index value from the path. If the path is already empty, nothing will happen.

**Returns:** `GH_Path` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_CullElement.htm)

#### `public GH_Path CullFirstElement()`

Removes the first index value from the path. If the path is already empty, nothing will happen.

**Returns:** `GH_Path` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_CullFirstElement.htm)

#### `public static bool operator ==(GH_Path A, GH_Path B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Data.GH_Path)
- `B` (Grasshopper.Kernel.Data.GH_Path)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_op_Equality.htm)

#### `public string Format(string format_provider, string separator)`

Format a path.

**Parameters:**
- `format_provider` (System.String) — The string describing the format. You are allowed to use {0} as a format placeholder. The placeholder will be filled up with a semi-colon separated list: 0;0;2;4
- `separator` (System.String) — Character to use for element separators. The default separator is a semi-colon.

**Returns:** `String` — Formatted string.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_Format.htm)

#### `public bool FromString(string s)`

Try to deserialize a GH_Path from a String.

**Parameters:**
- `s` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_FromString.htm)

#### `public override int GetHashCode()`

Specialized Hash code pattern.

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_GetHashCode.htm)

#### `public static bool operator >(GH_Path A, GH_Path B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Data.GH_Path)
- `B` (Grasshopper.Kernel.Data.GH_Path)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_op_GreaterThan.htm)

#### `public GH_Path Increment(int index)`

Increment a specific index in this path by one.

**Parameters:**
- `index` (System.Int32) — Index to increment. Valid indices are (-Length+1) to (Length-1).

**Returns:** `GH_Path` — A new path with incremented index.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_Increment.htm)

#### `public GH_Path Increment(int index, int offset)`

Increment a specific index in this path by one.

**Parameters:**
- `index` (System.Int32) — Index to increment. Valis indices are (-Length+1) to (Length-1).
- `offset` (System.Int32) — Amount to increment.

**Returns:** `GH_Path` — A new path with incremented index.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_Increment_1.htm)

#### `public static bool operator !=(GH_Path A, GH_Path B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Data.GH_Path)
- `B` (Grasshopper.Kernel.Data.GH_Path)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_op_Inequality.htm)

#### `public bool IsAncestor(GH_Path potential_ancestor, ref int additional_generations)`

Test another path to see if it is an ancestor of this path. For a path to be considered an ancestor, it must share the initial dimensions.

**Parameters:**
- `potential_ancestor` (Grasshopper.Kernel.Data.GH_Path) — Parental path to test.
- `additional_generations` (System.Int32) — This value will hold the number of generations between parent and child path.

**Returns:** `Boolean` — True if other overlaps completely with this path while being at least one element shorter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_IsAncestor.htm)

#### `public bool IsCoincident(GH_Path other)`

Test to see if this path is coincident with another path.

**Parameters:**
- `other` (Grasshopper.Kernel.Data.GH_Path) — Path to compare to.

**Returns:** `Boolean` — True if both paths are coincident (identical), false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_IsCoincident.htm)

#### `public bool IsCoincident(params int[] index_list)`

Test to see if this path is coincident with set of integers.

**Parameters:**
- `index_list` (System.Int32[]) — Integers to compare to.

**Returns:** `Boolean` — True if both paths are coincident (identical), false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_IsCoincident_1.htm)

#### `public static bool operator <(GH_Path A, GH_Path B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Data.GH_Path)
- `B` (Grasshopper.Kernel.Data.GH_Path)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_op_LessThan.htm)

#### `public GH_Path PrependElement(int index)`

Create a new path by prepending a new index value to this path.

**Parameters:**
- `index` (System.Int32) — Index value to prepend.

**Returns:** `GH_Path` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_PrependElement.htm)

#### `public bool Read(GH_IReader reader)`

Read this path from an archive.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Object to read with.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_Read.htm)

#### `public static bool SplitPathLikeString(string s, out string[] path_segments, out string index_segment)`

Try to split up a path-like formatted string "{A;B;C}(i)" into component parts.

**Parameters:**
- `s` (System.String) — String to parse.
- `path_segments` (System.String[]) — Output - Strings inside the path portion of s.
- `index_segment` (System.String) — Output - String inside the index portion of s.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_SplitPathLikeString.htm)

#### `public override string ToString()`

Concatenates the indices in the path.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_ToString.htm)

#### `public string ToString(bool includeBrackets)`

Concatenates the indices in the path.

**Parameters:**
- `includeBrackets` (System.Boolean) — If True, brackets will be prepended and appended to the path string.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_ToString_1.htm)

#### `public bool Write(GH_IWriter writer)`

Write this path to an archive.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Object to write with.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_Write.htm)

### Properties
- `Indices` (Int32[], get/set) — Gets or sets the entire index space; the path that identifies an element in structure space. You should not change the index space when the path is used inside a structure since it will invalidate the sort order. If you don't know what you're doing, for Pete's sake don't touch this.
- `Length` (Int32, get) — Returns the number of dimensions in the path.
- `Valid` (Boolean, get) — Gets whether this path is valid. Invalid paths either have no elements or negative elements.

## GH_Path.PathLengthComparer (class)

Use this comparer to sort lists of paths using a topological approach. Shorter paths are favoured over longer paths. Equal length paths use the default comparer.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_Path_PathLengthComparer.htm)

### Constructors
- `public PathLengthComparer()` — Initializes a new instance of the GH_Path.PathLengthComparer class

### Methods
#### `public int Compare(GH_Path x, GH_Path y)`



**Parameters:**
- `x` (Grasshopper.Kernel.Data.GH_Path)
- `y` (Grasshopper.Kernel.Data.GH_Path)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Path_PathLengthComparer_Compare.htm)

## GH_PathOffset (class)

Represents a relative offset within a data structure.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_PathOffset.htm)

### Constructors
- `public GH_PathOffset()` — Initializes a new instance of the GH_PathOffset class
- `public GH_PathOffset(IEnumerable<int> pathShift)` — Initializes a new instance of the GH_PathOffset class
- `public GH_PathOffset(IEnumerable<int> pathShift, int itemShift)` — Initializes a new instance of the GH_PathOffset class
- `public GH_PathOffset(IEnumerable<int> pathShift, int itemShift, bool pathWrap, bool itemWrap)` — Initializes a new instance of the GH_PathOffset class

### Methods
#### `public bool OffsetPath(GH_Path path, int index, ref GH_Path rel_path, ref int rel_index)`

Offset a path + index value without wrapping.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to offset.
- `index` (System.Int32) — Item to offset.
- `rel_path` (Grasshopper.Kernel.Data.GH_Path) — Offsetted path.
- `rel_index` (System.Int32) — Offsetted index.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_PathOffset_OffsetPath.htm)

#### `public bool OffsetPath(GH_Path path, int index, IGH_Structure tree, out GH_Path relativePath, out int relativeIndex)`

Offset a path + index value with wrapping and bounds checking.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to offset.
- `index` (System.Int32) — Item to offset.
- `tree` (Grasshopper.Kernel.Data.IGH_Structure) — Structure context for offset.
- `relativePath` (Grasshopper.Kernel.Data.GH_Path) — Offsetted path.
- `relativeIndex` (System.Int32) — Offsetted index.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_PathOffset_OffsetPath_1.htm)

#### `public static GH_PathOffset ParseString(string mask)`



**Parameters:**
- `mask` (System.String)

**Returns:** `GH_PathOffset` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_PathOffset_ParseString.htm)

#### `public override string ToString()`

Format the Path Offset.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_PathOffset_ToString.htm)

### Properties
- `ItemOffset` (Int32, get/set) — Gets or sets the relative offset for branch items.
- `ItemWrap` (Boolean, get/set) — Gets or sets the ItemWrap flag.
- `PathOffset` (List<Int32>, get) — Gets access to the relative offsets for path indices.
- `PathWrap` (Boolean, get/set) — Gets or sets the PathWrap flag.

## GH_RuleAnyNumber (class)

(No description provided in vendor docs for Grasshopper.Kernel.Data.GH_RuleAnyNumber.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleAnyNumber.htm)

### Constructors
- `public GH_RuleAnyNumber()` — Initializes a new instance of the GH_RuleAnyNumber class

### Methods
#### `public bool Apply(int number)`



**Parameters:**
- `number` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_RuleAnyNumber_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — 
- `Notation` (String, get) — 

## GH_RuleAnyNumbers (class)

(No description provided in vendor docs for Grasshopper.Kernel.Data.GH_RuleAnyNumbers.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleAnyNumbers.htm)

### Constructors
- `public GH_RuleAnyNumbers()` — Initializes a new instance of the GH_RuleAnyNumbers class

### Methods
#### `public bool Apply(int number)`



**Parameters:**
- `number` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_RuleAnyNumbers_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — 
- `Notation` (String, get) — 

## GH_RuleComplex (class)

(No description provided in vendor docs for Grasshopper.Kernel.Data.GH_RuleComplex.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleComplex.htm)

### Constructors
- `public GH_RuleComplex(IEnumerable<IGH_Rule> fragments, IEnumerable<GH_RuleOperator> operators)` — Initializes a new instance of the GH_RuleComplex class

### Methods
#### `public bool Apply(int number)`



**Parameters:**
- `number` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_RuleComplex_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — 
- `Notation` (String, get) — 

## GH_RuleGroup (class)

(No description provided in vendor docs for Grasshopper.Kernel.Data.GH_RuleGroup.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleGroup.htm)

### Constructors
- `public GH_RuleGroup(IEnumerable<int> numbers, bool negate)` — Initializes a new instance of the GH_RuleGroup class

### Methods
#### `public bool Apply(int number)`



**Parameters:**
- `number` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_RuleGroup_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — 
- `Notation` (String, get) — 

## GH_RuleKind (enum)

Represents all possible element types in a Path Pattern.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleKind.htm)

### Values
- `None` = `0` — Represents an invalid, default value.
- `AnyNumber` = `1` — Any single number
- `AnyNumbers` = `2` — Any (or no) collection of numbers
- `Number` = `3` — Single number
- `Group` = `4` — A collection of non-related integers.
- `Range` = `5` — A range of digits delimited by two extremes.
- `Sequence` = `6` — A collection of increasing non-related integers which repeat either indefinitely or up to a limit
- `Complex` = `7` — Represents a complex of multiple pattern types.

## GH_RuleNumber (class)

(No description provided in vendor docs for Grasshopper.Kernel.Data.GH_RuleNumber.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleNumber.htm)

### Constructors
- `public GH_RuleNumber(int number, bool negate)` — Initializes a new instance of the GH_RuleNumber class

### Methods
#### `public bool Apply(int number)`



**Parameters:**
- `number` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_RuleNumber_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — 
- `Notation` (String, get) — 

## GH_RuleOperator (enum)

Enumerate the possible ways rules can be strung together.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleOperator.htm)

### Values
- `Conjunction` = `0` — Represents an and/also operator.
- `Disjunction` = `1` — Represents an or/else operator.

## GH_RuleRange (class)

(No description provided in vendor docs for Grasshopper.Kernel.Data.GH_RuleRange.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleRange.htm)

### Constructors
- `public GH_RuleRange(int min, int max, bool negate)` — Initializes a new instance of the GH_RuleRange class

### Methods
#### `public bool Apply(int number)`



**Parameters:**
- `number` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_RuleRange_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — 
- `Notation` (String, get) — 

## GH_RuleResult (enum)

Enumerates the possible opinions a rule can have.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleResult.htm)

### Values
- `NoOpinion` = `0` — The rule does not prohibit or enforce any behaviour.
- `Include` = `1` — The rule claims a certain index ought to be included.
- `Exclude` = `-1` — The rule claims a certain index ought to be exluded.

## GH_RuleSequence (class)

(No description provided in vendor docs for Grasshopper.Kernel.Data.GH_RuleSequence.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_RuleSequence.htm)

### Constructors
- `public GH_RuleSequence(IEnumerable<int> sequence, int limit, bool negate)` — Initializes a new instance of the GH_RuleSequence class

### Methods
#### `public bool Apply(int number)`



**Parameters:**
- `number` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_RuleSequence_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — 
- `Notation` (String, get) — 

## GH_SimplificationMode (enum)

Enumerates the possible data structure simplification modes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_SimplificationMode.htm)

### Values
- `None` = `0` — Specifies no simplification is to take place.
- `CollapseLeadingOverlaps` = `1` — Only overlaps at the leading portion of paths will be simplified.
- `CollapseAllOverlaps` = `2` — Overlaps along the entire span of the paths will be simplified.

## GH_StringMismatchException (class)

Exception used during Rule Set parsing.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_StringMismatchException.htm)

### Constructors
- `public GH_StringMismatchException()` — Initializes a new instance of the GH_StringMismatchException class
- `public GH_StringMismatchException(string message)` — Initializes a new instance of the GH_StringMismatchException class

## GH_Structure<T> (class)

Represents a data tree where each branch has a unique path

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_Structure_1.htm)

### Constructors
- `public GH_Structure()` — Default constructor, create a blank data structure.
- `public GH_Structure(GH_Structure<T> other, bool shallowCopy)` — Copy constructor, duplicate another data structure.

### Methods
#### `public IGH_StructureEnumerator AllData(bool skipNulls)`

Gets an enumerator for all the data items in this structure.

**Parameters:**
- `skipNulls` (System.Boolean) — If True, null items will be skipped.

**Returns:** `IGH_StructureEnumerator` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_AllData.htm)

#### `public void Append(T data)`

Append a data item to the last branch in the structure. If no branch exist yet, a new one will be created with [path = {0}]

**Parameters:**
- `data` (T) — Data to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Append.htm)

#### `public void Append(T data, GH_Path path)`

Append a data item to the specified branch in the structure. If the branch doesn't exist yet, it will be created.

**Parameters:**
- `data` (T) — Data to append
- `path` (Grasshopper.Kernel.Data.GH_Path) — Data path

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Append_1.htm)

#### `public void AppendRange(IEnumerable<T> data)`

Append a list of data items to the last branch in the structure

**Parameters:**
- `data` (System.Collections.Generic.IEnumerable<T>) — Data to append

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_AppendRange.htm)

#### `public void AppendRange(IEnumerable<T> data, GH_Path path)`

Append a list of data items to the specified branch in the structure. If the branch doesn't exist yet, it will be created.

**Parameters:**
- `data` (System.Collections.Generic.IEnumerable<T>) — Data to append
- `path` (Grasshopper.Kernel.Data.GH_Path) — Data path

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_AppendRange_1.htm)

#### `public void Clear()`

Clears the entire structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Clear.htm)

#### `public void ClearData()`

Removes all data from all paths without affecting the structure topology.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_ClearData.htm)

#### `public string DataDescription(bool includeIndices, bool includePaths)`

Gets a description of the data contained in this structure.

**Parameters:**
- `includeIndices` (System.Boolean) — If true, every data item will be prefixed by its index.
- `includePaths` (System.Boolean) — If true, every branch of data will be preceded by the path.

**Returns:** `String` — A string describing the data.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_DataDescription.htm)

#### `public GH_Structure<T> Duplicate()`

Create an exact duplicate of this structure. All data items are copied as well.

**Returns:** `GH_Structure<T>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Duplicate.htm)

#### `public GH_Structure<Q> DuplicateCast<Q>(GH_Structure<T>.ConversionDelegate<T, Q> conversion) where Q : IGH_Goo`

Create a duplicate of this structure in a different type.

**Parameters:**
- `conversion` (Grasshopper.Kernel.Data.GH_Structure<T>.ConversionDelegate<T,Q>) — Conversion delegate.

**Returns:** `GH_Structure<Q>` — A duplicate of this structure with all valid casts.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_DuplicateCast__1.htm)

#### `public void EnsureCapacity(int capacity)`

Ensures that all current branches have a certain capacity.

**Parameters:**
- `capacity` (System.Int32) — Capacity per branch.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_EnsureCapacity.htm)

#### `public List<T> EnsurePath(GH_Path path)`

Create a new branch with the specified path if it doesn't already exists.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path of the branch to ensure.

**Returns:** `List<T>` — The list that belongs to the specified path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_EnsurePath.htm)

#### `public List<T> EnsurePath(params int[] path)`

Create a new branch with the specified path if it doesn't already exists.

**Parameters:**
- `path` (System.Int32[]) — Path of the branch to ensure.

**Returns:** `List<T>` — The list that belongs to the specified path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_EnsurePath_1.htm)

#### `public void ExpandPath(GH_Path path, int element, GH_ExpandMode mode)`

Expand a path in this structure by appending an element.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to expand, if the path doesn't exist, nothing will happen.
- `element` (System.Int32) — Element to append to the path.
- `mode` (Grasshopper.Kernel.Data.GH_ExpandMode) — Expansion mode.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_ExpandPath.htm)

#### `public void Flatten(GH_Path flat_path = null)`

Flattens the entire structure into a single path.

**Parameters:**
- `flat_path` (Grasshopper.Kernel.Data.GH_Path) — The path of the flattened data. If null, the path will be {0}

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Flatten.htm)

#### `public List<T> FlattenData()`

Collects all data in the structure under a single list. Does not alter the state of this structure.

**Returns:** `List<T>` — A linear list containing all data items in all branches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_FlattenData.htm)

#### `public IEnumerator GetEnumerator()`



**Returns:** `IEnumerator` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_GetEnumerator.htm)

#### `public IEnumerator<T> GetEnumerator_Generic()`



**Returns:** `IEnumerator<T>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_GetEnumerator_Generic.htm)

#### `public void Graft(GH_GraftMode mode)`

Grafts all paths by reallocating all data into child paths.

**Parameters:**
- `mode` (Grasshopper.Kernel.Data.GH_GraftMode) — Graft mode.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Graft.htm)

#### `public void Graft(GH_GraftMode mode, GH_Path path)`

Grafts a specific path by reallocating all data into child paths. If a grafted path coincides with an existing path, the data item in question will be appended to the existing path.

**Parameters:**
- `mode` (Grasshopper.Kernel.Data.GH_GraftMode) — Graft mode.
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to graft

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Graft_1.htm)

#### `public void Insert(T data, GH_Path path, int index)`

Insert a data item into the specified branch in the structure. If the branch doesn't exist yet, it will be created.

**Parameters:**
- `data` (T) — Data to insert.
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path of branch to insert into.
- `index` (System.Int32) — Insertion index.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Insert.htm)

#### `public int LongestPathIndex()`

Finds the path in this structure with the most dimensions. In case of multiple equally long longest paths, the last one will be returned.

**Returns:** `Int32` — The index of the longest path or -1 if there are no paths.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_LongestPathIndex.htm)

#### `public void MergeStructure(GH_Structure<T> other)`

Merges two structures together. Data inside similar paths will be merged into single lists and unique paths will be appended. The other structure will not be altered, so beware that data is now shared among both structures.

**Parameters:**
- `other` (Grasshopper.Kernel.Data.GH_Structure<T>) — Structure to merge

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_MergeStructure.htm)

#### `public bool PathExists(GH_Path path)`

Returns True if the specified path is already defined inside the structure.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The data path to search for

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_PathExists.htm)

#### `public void PathIndex(GH_Path path, ref int idx0, ref int idx1)`

Find the indices that surround the given path domain. Indices may be out of bounds.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to search for (it need not exist in the structure).
- `idx0` (System.Int32) — The index of the path itself or the first smaller path if the path is not defined.
- `idx1` (System.Int32) — The index of th path itself of the first larger path if the path is not defined.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_PathIndex.htm)

#### `public bool Read(GH_IReader reader)`

Read the entire GH_Structure from an archive. This method relies on the the serialization of the IGH_Goo items it contains. Also, the IGH_Goo T generic must have a public constructor that takes no arguments.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Archive to read from.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Read.htm)

#### `public void RemoveData(T data)`

Removes the first occurence of a data instance in the tree.

**Parameters:**
- `data` (T) — Data to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_RemoveData.htm)

#### `public void RemovePath(GH_Path path)`

Removes a path and all associated data from the structure. If the path doesn't exist, nothing will happen.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_RemovePath.htm)

#### `public void ReplacePath(GH_Path find, GH_Path replace)`

Replace an existing path with a different one. If the operation is successfull, then the 'find' path will be deleted. If the 'replace' path is already defined, the items in 'find' will be appended to the existing path.

**Parameters:**
- `find` (Grasshopper.Kernel.Data.GH_Path) — The path to search for.
- `replace` (Grasshopper.Kernel.Data.GH_Path) — The new path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_ReplacePath.htm)

#### `public void Reverse()`

Reverse the order of items in all branches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Reverse.htm)

#### `public GH_Structure<T> ShallowDuplicate()`

Create a structure with a similar topology to this one with pointers to the same data items.

**Returns:** `GH_Structure<T>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_ShallowDuplicate.htm)

#### `public int ShortestPathIndex()`

Finds the path in this structure with the least dimensions. In case of multiple equally long longest paths, the first one will be returned.

**Returns:** `Int32` — The index of the shortest path or -1 if there are no paths.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_ShortestPathIndex.htm)

#### `public void Simplify(GH_SimplificationMode mode)`

Simplify the data structure by collapsing path indices shared by all branches.

**Parameters:**
- `mode` (Grasshopper.Kernel.Data.GH_SimplificationMode) — Simplification algorithm to use.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Simplify.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_ToString.htm)

#### `public void TrimExcess()`

Trims the excess allocated memory in all branches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_TrimExcess.htm)

#### `public bool Write(GH_IWriter writer)`

Write the entire GH_Structure to an archive. This method relies on the the serialization of the IGH_Goo items it contains.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Archive to write to.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_Structure_1_Write.htm)

### Properties
- `Branch[GH_Path]` (IList, get) — Gets a limited access pointer to the data list associated with a certain path.
- `Branch[Int32]` (IList, get) — Gets a limited access pointer to the data list at the specified index
- `Branches` (IList<List<T>>, get) — Gets a list of all the data-arrays in this structure
- `DataCount` (Int32, get) — Gets the total number of data items stored in all paths.
- `DataItem[Int32]` (T, get) — Gets the item at a given offset. The structure is treated as a linear list in this case. If index is out of range, null is returned. For repeated indexed retrieval, consider using a For Each loop since the enumerator is more efficient.
- `DataItem[GH_Path, Int32]` (T, get) — Gets a direct pointer to a data item under a specific path and index.
- `DataList[GH_Path]` (List<T>, get) — Gets a direct pointer to the list of data under a specific path.
- `DataList[Int32]` (List<T>, get) — Gets a direct pointer to the list of data under the specified index.
- `FirstItem` (T, get) — Gets the first item in the structure. If no such item exists, null is returned.
- `IsEmpty` (Boolean, get) — Gets the Empty state of the structure. If the structure is Empty it contains no paths and no branches.
- `LastItem` (T, get) — Gets the last item in the structure. If no such item exists, null is returned.
- `NonNulls` (IEnumerable<T>, get) — Get an enumerator for this structure that allow linear iteration over the data hierarchy while skipping all Null entries. This Enumerator has been optimized, do not cache instances of it.
- `Path` (GH_Path, get) — Gets the data path at the specified index.
- `PathCount` (Int32, get) — Gets the number of paths defined in this structure.
- `Paths` (IList<GH_Path>, get) — Gets a list of all the paths in this structure.
- `StructureProxy` (IList<IList>, get) — Gets a proxy list of all the data-arrays in this structure.
- `TopologyDescription` (String, get) — Gets a description of the topology of the structure. Useful for debugging purposes.

## GH_Structure<T>.ConversionDelegate<Tfrom, Tto> (delegate)

Represents a conversion between two types of IGH_Goo.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_Structure_1_ConversionDelegate_2.htm)

## GH_TreeFilter (class)

Represents a collection of rules for validating DataTree paths and indices.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_TreeFilter.htm)

### Constructors
- `public GH_TreeFilter()` — Initializes a new instance of the GH_TreeFilter class

### Methods
#### `public static bool FindItemBrackets(string text, out int index0, out int index1)`

Locate the two parenthesis that delineate the index segment of an Index Rule set.

**Parameters:**
- `text` (System.String) — String to search.
- `index0` (System.Int32) — Index of opening parenthesis.
- `index1` (System.Int32) — Index of closing parenthesis.

**Returns:** `Boolean` — True if valid parenthesis were found, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeFilter_FindItemBrackets.htm)

#### `public static int FindNextLevelChar(string text, int index, char char)`

Find the next occurance of a char in a string that is level with the search start index.

**Parameters:**
- `text` (System.String) — String to search.
- `index` (System.Int32) — Index to search from.
- `char` (System.Char) — Char to match.

**Returns:** `Int32` — The index of the first level character match or -1 if no such character could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeFilter_FindNextLevelChar.htm)

#### `public static bool FindPathBrackets(string text, out int index0, out int index1)`

Locate the two curly brackets that delineate the path segment of an Index Rule set.

**Parameters:**
- `text` (System.String) — String to search.
- `index0` (System.Int32) — Index of opening curly bracket.
- `index1` (System.Int32) — Index of closing curly bracket.

**Returns:** `Boolean` — True if valid brackets were found, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeFilter_FindPathBrackets.htm)

#### `public static int FindPrevLevelChar(string text, int index, char char)`

Find the previous occurance of a char in a string that is level with the search start index.

**Parameters:**
- `text` (System.String) — String to search.
- `index` (System.Int32) — Index to search from.
- `char` (System.Char) — Char to match.

**Returns:** `Int32` — The index of the first level character match or -1 if no such character could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeFilter_FindPrevLevelChar.htm)

#### `public static GH_TreeFilter ParsePattern(string filter)`

Parses a textual filter and returns an instance of the GH_TreeFilter. If the filter cannot be parsed an exception will be thrown.

**Parameters:**
- `filter` (System.String) — Filter to parse.

**Returns:** `GH_TreeFilter` — An instance of GH_TreeFilter describing the textual filter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeFilter_ParsePattern.htm)

#### `public static List<string> SplitStringWithExpressions(string text, char separator)`

Split a string into segments using level-aware split. This method throws exceptions if the input string is not correctly formatted.

**Parameters:**
- `text` (System.String) — String to split.
- `separator` (System.Char) — Splitting character.

**Returns:** `List<String>` — A list of string segments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeFilter_SplitStringWithExpressions.htm)

### Properties
- `ItemClose` (Char, get) — 
- `ItemOpen` (Char, get) — 
- `PathClose` (Char, get) — 
- `PathOpen` (Char, get) — 
- `PathSeparator` (Char, get) — 
- `SegmentSeparator` (Char, get) — 
- `StringDelimeter` (Char, get) — 

## GH_TreeIndex (struct)

Represents a single unique location in a DataTree.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_TreeIndex.htm)

### Constructors
- `public GH_TreeIndex(GH_Path path, int item)` — Create a new instance of the GH_TreeIndex structure with preset path and item index.

### Properties
- `Item` (Int32, get/set) — Gets or sets the item index.
- `Path` (GH_Path, get/set) — Gets or sets the Path identifier.

## GH_TreeRules (class)

Represents an entire data tree rule set.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_GH_TreeRules.htm)

### Constructors
- `public GH_TreeRules(IEnumerable<IGH_Rule> pathRules, IGH_Rule indexRule)` — Constructs a new rule set for testing data trees. Unless you know exactly what you are doing, use the FromString() static method instead.

### Methods
#### `public bool Apply(GH_Path path)`

Apply all the local rules to a path.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to test.

**Returns:** `Boolean` — True if path passes all the rule tests.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeRules_Apply.htm)

#### `public bool Apply(GH_Path path, int index)`

Apply all the local rules to a path+index combination.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to test.
- `index` (System.Int32) — Index to test.

**Returns:** `Boolean` — True if path+index pass all the rule tests.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeRules_Apply_1.htm)

#### `public static GH_TreeRules FromString(string text, ref string message = null)`

Attempt to parse a Pattern string.

**Parameters:**
- `text` (System.String) — String to parse.
- `message` (System.String) — If this function fails, msg will contain a message describing the error.

**Returns:** `GH_TreeRules` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeRules_FromString.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_GH_TreeRules_ToString.htm)

### Properties
- `HasItemRule` (Boolean, get) — Gets whether this rule set has a defined index rule.
- `HasPathRules` (Boolean, get) — Gets whether this rule set has any path rules.
- `PathRuleCount` (Int32, get) — Gets the number of defined path rules.
- `AllowedChars` (String, get) — 
- `AndOperator` (Char, get) — 
- `AnyNumbersSymbol` (Char, get) — 
- `AnyNumberSymbol` (Char, get) — 
- `ItemBrackets` (String, get) — 
- `ItemCloseBracket` (Char, get) — 
- `ItemOpenBracket` (Char, get) — 
- `NotOperator` (Char, get) — 
- `OrOperator` (Char, get) — 
- `PathBrackets` (String, get) — 
- `PathCloseBracket` (Char, get) — 
- `PathOpenBracket` (Char, get) — 
- `PathSeparator` (Char, get) — 
- `RangeSymbol` (Char, get) — 
- `RuleBrackets` (String, get) — 
- `RuleCloseBracket` (Char, get) — 
- `RuleOpenBracket` (Char, get) — 
- `Separator` (Char, get) — 
- `SequenceCode` (String, get) — 
- `SequenceSymbol` (Char, get) — 

## IGH_DataTree (interface)

Utility interface for detection of DataTree generic instances.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_IGH_DataTree.htm)

### Methods
#### `bool MergeWithParameter(IGH_Param param)`

Utility function for assigning Script Data Trees to parameters.

**Parameters:**
- `param` (Grasshopper.Kernel.IGH_Param) — Parameter to merge with.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_DataTree_MergeWithParameter.htm)

## IGH_IndexRule (interface)

Interface for index rules. An index rule is a way to determine whether a specific integer is considered valid, invalid or neutral. Index rules are used for selecting elements in DataTrees.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_IGH_IndexRule.htm)

### Methods
#### `GH_RuleResult Evaluate(int index)`



**Parameters:**
- `index` (System.Int32)

**Returns:** `GH_RuleResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_IndexRule_Evaluate.htm)

## IGH_Rule (interface)

Interface for individual pattern elements.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_IGH_Rule.htm)

### Methods
#### `bool Apply(int number)`

Applies this rule to an integer.

**Parameters:**
- `number` (System.Int32) — Integer to match.

**Returns:** `Boolean` — True if the integer passes the rule test.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Rule_Apply.htm)

### Properties
- `Kind` (GH_RuleKind, get) — Gets the type of rule.
- `Notation` (String, get) — Gets the string notation (representation) for this rule.

## IGH_Structure (interface)

Base interface for all GH_Structure types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_IGH_Structure.htm)

### Methods
#### `IGH_StructureEnumerator AllData(bool skipNulls)`

Gets an enumerator for all the data items in this structure.

**Parameters:**
- `skipNulls` (System.Boolean) — If True, null items will be skipped.

**Returns:** `IGH_StructureEnumerator` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_AllData.htm)

#### `void Clear()`

Clears the entire structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_Clear.htm)

#### `void ClearData()`

Removes all data from all paths without affecting the structure topology.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_ClearData.htm)

#### `string DataDescription(bool includeIndices, bool includePaths)`

Gets a description of the data contained in this structure.

**Parameters:**
- `includeIndices` (System.Boolean) — If true, every data item will be prefixed by its index.
- `includePaths` (System.Boolean) — If true, every branch of data will be preceded by the path.

**Returns:** `String` — A string describing the data.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_DataDescription.htm)

#### `void EnsureCapacity(int capacity)`

Ensures that all branches have a certain capacity

**Parameters:**
- `capacity` (System.Int32) — Capacity per branch.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_EnsureCapacity.htm)

#### `void ExpandPath(GH_Path path, int element, GH_ExpandMode mode)`

Expand a path in this structure by appending an element.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to expand, if the path doesn't exist, nothing will happen.
- `element` (System.Int32) — Element to append to the path.
- `mode` (Grasshopper.Kernel.Data.GH_ExpandMode) — Expansion mode.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_ExpandPath.htm)

#### `void Flatten(GH_Path path = null)`

Flattens the entire structure into a single path.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The path of the flattened data. If null, the path will be {0}

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_Flatten.htm)

#### `void Graft(GH_GraftMode mode)`

Grafts all paths by reallocating all data into child paths.

**Parameters:**
- `mode` (Grasshopper.Kernel.Data.GH_GraftMode) — Graft mode.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_Graft.htm)

#### `void Graft(GH_GraftMode mode, GH_Path path)`

Grafts a specific path by reallocating all data into child paths. If a grafted path coincides with an existing path, the data item in question will be appended to the existing path.

**Parameters:**
- `mode` (Grasshopper.Kernel.Data.GH_GraftMode) — Graft mode.
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to graft

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_Graft_1.htm)

#### `int LongestPathIndex()`

Finds the path in this structure with the most dimensions. In case of multiple equally long longest paths, the last one will be returned.

**Returns:** `Int32` — The index of the longest path or -1 if there are no paths.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_LongestPathIndex.htm)

#### `bool PathExists(GH_Path path)`

Returns True if the specified path is already defined inside the structure.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The data path to search for

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_PathExists.htm)

#### `void PathIndex(GH_Path path, ref int idx0, ref int idx1)`

Find the indices that delineate the given path domain.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to search for (it need not exist in the structure).
- `idx0` (System.Int32) — The index of the path itself or the first smaller path if the path is not defined.
- `idx1` (System.Int32) — The index of the path itself of the first larger path if the path is not defined.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_PathIndex.htm)

#### `void RemovePath(GH_Path path)`

Removes a path and all associated data from the structure. If the path doesn't exist, nothing will happen.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path the remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_RemovePath.htm)

#### `void ReplacePath(GH_Path find, GH_Path replace)`

Replace an existing path with a different one. If the operation is successfull, then the 'find' path will be deleted. If the 'replace' path is already defined, the items in 'find' will be appended to the existing path.

**Parameters:**
- `find` (Grasshopper.Kernel.Data.GH_Path) — The path to search for.
- `replace` (Grasshopper.Kernel.Data.GH_Path) — The new path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_ReplacePath.htm)

#### `int ShortestPathIndex()`

Finds the path in this structure with the least dimensions. In case of multiple equally long longest paths, the first one will be returned.

**Returns:** `Int32` — The index of the shortest path or -1 if there are no paths.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_ShortestPathIndex.htm)

#### `void Simplify(GH_SimplificationMode mode)`

Simplify the data structure by removing path indices shared by all branches.

**Parameters:**
- `mode` (Grasshopper.Kernel.Data.GH_SimplificationMode) — Simplification algorithm to use.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_Simplify.htm)

#### `void TrimExcess()`

Trims the excess allocated memory in all branches

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Data_IGH_Structure_TrimExcess.htm)

### Properties
- `Branch[GH_Path]` (IList, get) — Gets a limited access pointer to the data list associated with a certain path.
- `Branch[Int32]` (IList, get) — Gets a limited access pointer to the data list at the specified index.
- `DataCount` (Int32, get) — Gets the total number of data items stored in all paths.
- `IsEmpty` (Boolean, get) — Gets the Empty state of the structure. If the structure is Empty when it contains no paths and no branches.
- `Path` (GH_Path, get) — Gets the data path at the specified index.
- `PathCount` (Int32, get) — Gets the number of paths defined in this structure.
- `Paths` (IList<GH_Path>, get) — Gets a list of all the paths in this structure.
- `StructureProxy` (IList<IList>, get) — Gets a proxy list of all the data-arrays in this structure
- `TopologyDescription` (String, get) — Gets a description of the topology of the structure. Useful for debugging purposes.

## IGH_StructureEnumerator (interface)

Defines a minimum interface for iterating over all data in an IGH_Structure.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Data_IGH_StructureEnumerator.htm)

