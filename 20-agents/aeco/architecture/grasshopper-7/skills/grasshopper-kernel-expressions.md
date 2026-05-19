---
name: grasshopper-grasshopper-kernel-expressions
description: This skill encodes the grasshopper 7.0 surface of the Grasshopper.Kernel.Expressions namespace — 18 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_CodeString, GH_CodeStringSegment, GH_ExpressionParser, GH_ExpressionSyntaxWriter, GH_ExpressionString, GH_ParserOperator, GH_ParserSymbol, GH_ScriptVariant, and 10 more types.
---

# Grasshopper.Kernel.Expressions

Auto-generated from vendor docs for grasshopper 7.0. 18 types in this namespace.

## GH_CharType (enum)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_CharType.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_CharType.htm)

### Values
- `undefined` = `0`
- `whitespace` = `1`
- `dot` = `2`
- `comma` = `3`
- `colon` = `4`
- `semicolon` = `5`
- `continuation` = `6`
- `newline` = `7`
- `operator` = `20`
- `parenthesis_open` = `21`
- `parenthesis_close` = `22`
- `bracket_open` = `23`
- `bracket_close` = `24`
- `stringstart` = `50`
- `stringend` = `51`
- `stringbody` = `52`
- `commentstart` = `60`
- `commentend` = `61`
- `commentbody` = `62`

## GH_CodeString (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_CodeString.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_CodeString.htm)

### Constructors
- `public GH_CodeString(string input)` — Initializes a new instance of the GH_CodeString class

### Methods
#### `public string Flatten()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeString_Flatten.htm)

#### `public void ParseNewString(string input)`



**Parameters:**
- `input` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeString_ParseNewString.htm)

#### `public void Replace(string search, string replace, bool bIgnoreCase = true, bool bOmitNonCode = true)`



**Parameters:**
- `search` (System.String)
- `replace` (System.String)
- `bIgnoreCase` (System.Boolean)
- `bOmitNonCode` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeString_Replace.htm)

#### `public void ReplaceToken(string search, string replace, bool bIgnoreCase = true, bool bOmitNonCode = true)`



**Parameters:**
- `search` (System.String)
- `replace` (System.String)
- `bIgnoreCase` (System.Boolean)
- `bOmitNonCode` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeString_ReplaceToken.htm)

### Properties
- `Segments` (List<GH_CodeStringSegment>, get) — 
- `DoubleQuote` (Char, get) — 
- `m_Segments` (List<GH_CodeStringSegment>, get) — 

## GH_CodeStringSegment (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_CodeStringSegment.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_CodeStringSegment.htm)

### Constructors
- `public GH_CodeStringSegment(string nString, bool bIsCode)` — Initializes a new instance of the GH_CodeStringSegment class

### Methods
#### `protected bool IsAlpha(char C)`



**Parameters:**
- `C` (System.Char)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeStringSegment_IsAlpha.htm)

#### `public void Replace(string search, string replace, bool bIgnoreCase = true)`



**Parameters:**
- `search` (System.String)
- `replace` (System.String)
- `bIgnoreCase` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeStringSegment_Replace.htm)

#### `public void ReplaceToken(string token, string replace, bool bIgnoreCase = true)`



**Parameters:**
- `token` (System.String)
- `replace` (System.String)
- `bIgnoreCase` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeStringSegment_ReplaceToken.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_CodeStringSegment_ToString.htm)

### Properties
- `IsCode` (Boolean, get/set) — 
- `String` (String, get/set) — 
- `StringValue` (String, get) — 
- `m_IsCode` (Boolean, get) — 
- `m_String` (String, get) — 

## GH_ExpressionParser (class)

Provides a run-time evaluator for Grasshopper expressions.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ExpressionParser.htm)

### Constructors
- `public GH_ExpressionParser()` — Initializes a new instance of the GH_ExpressionParser class
- `public GH_ExpressionParser(bool bThrowExceptions)` — Initializes a new instance of the GH_ExpressionParser class

### Methods
#### `public void AddVariable(string name, bool val)`



**Parameters:**
- `name` (System.String)
- `val` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable_4.htm)

#### `public void AddVariable(string name, Complex val)`



**Parameters:**
- `name` (System.String)
- `val` (Grasshopper.Kernel.Types.Complex)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable.htm)

#### `public void AddVariable(string name, double val)`



**Parameters:**
- `name` (System.String)
- `val` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable_5.htm)

#### `public void AddVariable(string name, int val)`



**Parameters:**
- `name` (System.String)
- `val` (System.Int32)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable_6.htm)

#### `public void AddVariable(string name, Plane val)`



**Parameters:**
- `name` (System.String)
- `val` (Plane)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable_1.htm)

#### `public void AddVariable(string name, Point3d val)`



**Parameters:**
- `name` (System.String)
- `val` (Point3d)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable_2.htm)

#### `public void AddVariable(string name, string val)`



**Parameters:**
- `name` (System.String)
- `val` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable_7.htm)

#### `public void AddVariable(string name, Vector3d val)`



**Parameters:**
- `name` (System.String)
- `val` (Vector3d)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariable_3.htm)

#### `public void AddVariableEx(string name, GH_Variant val)`



**Parameters:**
- `name` (System.String)
- `val` (Grasshopper.Kernel.Expressions.GH_Variant)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariableEx.htm)

#### `public void AddVariableEx(string name, IGH_Goo val)`



**Parameters:**
- `name` (System.String)
- `val` (Grasshopper.Kernel.Types.IGH_Goo)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_AddVariableEx_1.htm)

#### `public static bool BalancedCharTest(string str, char char_open, char char_close, out int error_at)`

Test a string for balanced open and close chars. You can use this function to see if brackets or parenthesis have been properly used.

**Parameters:**
- `str` (System.String) — String to parse.
- `char_open` (System.Char) — Char that indicates an opening statement.
- `char_close` (System.Char) — Char that indicates a closing statement.
- `error_at` (System.Int32) — The index at which the first error appears or -1 if no error was found.

**Returns:** `Boolean` — True if the chars are balanced, false if they are not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_BalancedCharTest.htm)

#### `public bool CacheSymbols(string Expression)`

Create a Symbols array from the expression. Use this method if you intend to evaluate the same expression multiple times. At this point, the expression has to be in correct syntax format. Use the GH_ExpressionSyntaxWriter.RewriteAll() method to make sure.

**Parameters:**
- `Expression` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_CacheSymbols.htm)

#### `public Queue<GH_ParserSymbol> CachedSymbols()`

Retrieve a copy of the Queue of cached symbols. You can use the cached symbols as an uber-optimization to speed up successive calls to Evaluate() with an identical expression string

**Returns:** `Queue<GH_ParserSymbol>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_CachedSymbols.htm)

#### `public void ClearSymbols()`

Destroy the Symbols cache.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_ClearSymbols.htm)

#### `public void ClearVariables()`

Destroy the variable cache.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_ClearVariables.htm)

#### `public void DisplayFunctionList(IWin32Window wnd = null)`



**Parameters:**
- `wnd` (System.Windows.Forms.IWin32Window)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_DisplayFunctionList.htm)

#### `public GH_Variant Evaluate()`

Evaluate the expression currently loaded in the Symbols cache using the currently loaded variables.

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Evaluate.htm)

#### `public GH_Variant Evaluate(Queue<GH_ParserSymbol> qHint)`

Evaluate the expression queue without overriding any local caches. You can obtain an expression queue by calling CachedSymbols()

**Parameters:**
- `qHint` (System.Collections.Generic.Queue<GH_ParserSymbol>)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Evaluate_1.htm)

#### `public GH_Variant Evaluate(string expression)`

Store a new expression in the Symbols cache and evaluate it using the current variables

**Parameters:**
- `expression` (System.String)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Evaluate_2.htm)

#### `public static bool IsValidVariableName(string name)`

Tests whether a string is a valid variable name for expressions. Valid names must contain at least one character, must start with an alphabetic character, and only contain alphanumeric chars and underscores.

**Parameters:**
- `name` (System.String) — Name to test.

**Returns:** `Boolean` — True if the name is valid (might still conflict with a function name though), false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_IsValidVariableName.htm)

#### `protected GH_Variant Op_BinaryAND(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryAND.htm)

#### `protected GH_Variant Op_BinaryAddition(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryAddition.htm)

#### `protected GH_Variant Op_BinaryAmpersand(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryAmpersand.htm)

#### `protected GH_Variant Op_BinaryAngle(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryAngle.htm)

#### `protected GH_Variant Op_BinaryCircumflex(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryCircumflex.htm)

#### `protected GH_Variant Op_BinaryCrossProduct(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryCrossProduct.htm)

#### `protected GH_Variant Op_BinaryDistance(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryDistance.htm)

#### `protected GH_Variant Op_BinaryDivision(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryDivision.htm)

#### `protected GH_Variant Op_BinaryEquality(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryEquality.htm)

#### `protected GH_Variant Op_BinaryIntegerDivision(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryIntegerDivision.htm)

#### `protected GH_Variant Op_BinaryLargerThan(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryLargerThan.htm)

#### `protected GH_Variant Op_BinaryLargerThanOrEqual(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryLargerThanOrEqual.htm)

#### `protected GH_Variant Op_BinaryModulus(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryModulus.htm)

#### `protected GH_Variant Op_BinaryMultiplication(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryMultiplication.htm)

#### `protected GH_Variant Op_BinaryNearEquality(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryNearEquality.htm)

#### `protected GH_Variant Op_BinaryOR(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryOR.htm)

#### `protected GH_Variant Op_BinaryPull(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryPull.htm)

#### `protected GH_Variant Op_BinaryPush(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryPush.htm)

#### `protected GH_Variant Op_BinarySmallerThan(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinarySmallerThan.htm)

#### `protected GH_Variant Op_BinarySmallerThanOrEqual(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinarySmallerThanOrEqual.htm)

#### `protected GH_Variant Op_BinarySubtraction(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinarySubtraction.htm)

#### `protected GH_Variant Op_BinaryXOR(GH_Variant A, GH_Variant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_Variant)
- `B` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_BinaryXOR.htm)

#### `protected GH_Variant Op_UnaryBang(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryBang.htm)

#### `protected GH_Variant Op_UnaryCube(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryCube.htm)

#### `protected GH_Variant Op_UnaryDeg2Rad(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryDeg2Rad.htm)

#### `protected GH_Variant Op_UnaryImaginary(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryImaginary.htm)

#### `protected GH_Variant Op_UnaryMinus(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryMinus.htm)

#### `protected GH_Variant Op_UnaryNOT(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryNOT.htm)

#### `protected GH_Variant Op_UnaryOComponent(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryOComponent.htm)

#### `protected GH_Variant Op_UnaryPlus(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryPlus.htm)

#### `protected GH_Variant Op_UnarySquare(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnarySquare.htm)

#### `protected GH_Variant Op_UnaryXComponent(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryXComponent.htm)

#### `protected GH_Variant Op_UnaryYComponent(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryYComponent.htm)

#### `protected GH_Variant Op_UnaryZComponent(GH_Variant V)`



**Parameters:**
- `V` (Grasshopper.Kernel.Expressions.GH_Variant)

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionParser_Op_UnaryZComponent.htm)

### Properties
- `ThrowExceptions` (Boolean, get/set) — 
- `Variables` (SortedDictionary<String,GH_Variant>, get) — 

## GH_ExpressionString (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ExpressionString.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ExpressionString.htm)

### Constructors
- `public GH_ExpressionString(string in)` — Initializes a new instance of the GH_ExpressionString class

### Methods
#### `public void BuildLUT()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionString_BuildLUT.htm)

#### `public bool IsWhiteSpace(int index)`



**Parameters:**
- `index` (System.Int32)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionString_IsWhiteSpace.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionString_ToString.htm)

### Properties
- `Char` (Char, get) — 
- `Next` (Char, get) — 
- `Prev` (Char, get) — 
- `bracket_closed` (Char, get) — 
- `bracket_open` (Char, get) — 
- `colon` (Char, get) — 
- `comma` (Char, get) — 
- `dot` (Char, get) — 
- `dquote` (Char, get) — 
- `invalid` (Char, get) — 
- `linefeed` (Char, get) — 
- `m_t` (List<GH_CharType>, get) — 
- `paren_close` (Char, get) — 
- `paren_open` (Char, get) — 
- `quote` (Char, get) — 
- `return` (Char, get) — 
- `scolon` (Char, get) — 
- `space` (Char, get) — 
- `tab` (Char, get) — 
- `underbar` (Char, get) — 

## GH_ExpressionSyntaxWriter (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ExpressionSyntaxWriter.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ExpressionSyntaxWriter.htm)

### Methods
#### `public static string RewriteAll(string Expression)`

Apply all syntax rulesets to the expression.

**Parameters:**
- `Expression` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionSyntaxWriter_RewriteAll.htm)

#### `public static void RewriteForEvaluator(GH_CodeString sCode)`

Rewrite the expression so that all temporary keywords and symbols are replaced by evaluator-specific chars. The expression will become less readable, do not let the user see the result of this function. You need to rewrite the expression with this function if you intend to feed it into the Evaluator.

**Parameters:**
- `sCode` (Grasshopper.Kernel.Expressions.GH_CodeString)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionSyntaxWriter_RewriteForEvaluator.htm)

#### `public static string RewriteForEvaluator(string Expression)`

Rewrite the expression so that all temporary keywords and symbols are replaced by evaluator-specific chars. The expression will become less readable, do not let the user see the result of this function. You need to rewrite the expression with this function if you intend to feed it into the Evaluator.

**Parameters:**
- `Expression` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionSyntaxWriter_RewriteForEvaluator_1.htm)

#### `public static void RewriteForGraphicInterface(GH_CodeString code)`

Rewrite the expression so that tags are replaced by the complex characters that make up the esoteric operators, functions and constants. The expression should become more readable for humans.

**Parameters:**
- `code` (Grasshopper.Kernel.Expressions.GH_CodeString)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionSyntaxWriter_RewriteForGraphicInterface.htm)

#### `public static string RewriteForGraphicInterface(string expression)`

Rewrite the expression so that tags are replaced by the complex characters that make up the esoteric operators, functions and constants. The expression should become more readable

**Parameters:**
- `expression` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ExpressionSyntaxWriter_RewriteForGraphicInterface_1.htm)

## GH_OperatorType (enum)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_OperatorType.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_OperatorType.htm)

### Values
- `UnaryOnLeft` = `0`
- `UnaryOnRight` = `1`
- `Binary` = `2`

## GH_ParserOperator (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ParserOperator.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ParserOperator.htm)

### Constructors
- `public GH_ParserOperator(string name, char symbol, GH_ParserPrecedence precedence, GH_OperatorType type, string description)` — Initializes a new instance of the GH_ParserOperator class

### Methods
#### `public int CompareTo(GH_ParserOperator other)`



**Parameters:**
- `other` (Grasshopper.Kernel.Expressions.GH_ParserOperator)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ParserOperator_CompareTo.htm)

#### `public override bool Equals(Object obj)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `obj` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ParserOperator_Equals.htm)

### Properties
- `m_description` (String, get) — 
- `m_name` (String, get) — 
- `m_precedence` (GH_ParserPrecedence, get) — 
- `m_symbol` (Char, get) — 
- `m_type` (GH_OperatorType, get) — 

## GH_ParserPrecedence (enum)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ParserPrecedence.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ParserPrecedence.htm)

### Values
- `Invalid` = `-1`
- `None` = `0`
- `Level0` = `1`
- `Level1` = `2`
- `Level2` = `3`
- `Level3` = `4`
- `Level4` = `5`
- `Level5` = `6`

## GH_ParserSymbol (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ParserSymbol.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ParserSymbol.htm)

### Constructors
- `public GH_ParserSymbol()` — Initializes a new instance of the GH_ParserSymbol class
- `public GH_ParserSymbol(string token, GH_ParserTokenClass class, GH_ParserPrecedence level)` — Initializes a new instance of the GH_ParserSymbol class

### Methods
#### `public int CompareTo(GH_ParserSymbol other)`



**Parameters:**
- `other` (Grasshopper.Kernel.Expressions.GH_ParserSymbol)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ParserSymbol_CompareTo.htm)

#### `public override bool Equals(Object obj)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `obj` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ParserSymbol_Equals.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ParserSymbol_ToString.htm)

### Properties
- `m_class` (GH_ParserTokenClass, get) — 
- `m_level` (GH_ParserPrecedence, get) — 
- `m_token` (String, get) — 

## GH_ParserTokenClass (enum)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ParserTokenClass.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ParserTokenClass.htm)

### Values
- `Keyword` = `1`
- `Identifier` = `2`
- `Numeric` = `3`
- `Literal` = `4`
- `Operator` = `5`
- `Punctuation` = `6`

## GH_ScriptVariant (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ScriptVariant.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ScriptVariant.htm)

### Constructors
- `public GH_ScriptVariant()` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(Plane val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(Point3d val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(Vector3d val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(bool val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(byte val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(DateTime val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(double val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(int val)` — Initializes a new instance of the GH_ScriptVariant class
- `public GH_ScriptVariant(string val)` — Initializes a new instance of the GH_ScriptVariant class

### Methods
#### `public static GH_ScriptVariant operator +(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Addition.htm)

#### `public static bool operator &(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_BitwiseAnd.htm)

#### `public static bool operator |(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_BitwiseOr.htm)

#### `C# does not support this operator.`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Concatenate.htm)

#### `public static GH_ScriptVariant operator /(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Division.htm)

#### `public static bool operator ==(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Equality.htm)

#### `public static bool operator ^(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_ExclusiveOr.htm)

#### `public static explicit operator bool (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit.htm)

#### `public static explicit operator DateTime (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `DateTime` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit_4.htm)

#### `public static explicit operator double (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit_2.htm)

#### `public static explicit operator int (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit_1.htm)

#### `public static explicit operator Plane (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Plane` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit_7.htm)

#### `public static explicit operator Point3d (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Point3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit_5.htm)

#### `public static explicit operator string (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit_3.htm)

#### `public static explicit operator Vector3d (GH_ScriptVariant in)`



**Parameters:**
- `in` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Vector3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Explicit_6.htm)

#### `C# does not support this operator.`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Exponent.htm)

#### `public static bool operator false(GH_ScriptVariant A)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_False.htm)

#### `public static bool operator >(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_GreaterThan.htm)

#### `public static bool operator >=(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_GreaterThanOrEqual.htm)

#### `public static implicit operator GH_ScriptVariant (bool in)`



**Parameters:**
- `in` (System.Boolean)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit_3.htm)

#### `public static implicit operator GH_ScriptVariant (DateTime in)`



**Parameters:**
- `in` (System.DateTime)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit_4.htm)

#### `public static implicit operator GH_ScriptVariant (double in)`



**Parameters:**
- `in` (System.Double)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit_5.htm)

#### `public static implicit operator GH_ScriptVariant (int in)`



**Parameters:**
- `in` (System.Int32)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit_6.htm)

#### `public static implicit operator GH_ScriptVariant (Plane in)`



**Parameters:**
- `in` (Plane)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit.htm)

#### `public static implicit operator GH_ScriptVariant (Point3d in)`



**Parameters:**
- `in` (Point3d)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit_1.htm)

#### `public static implicit operator GH_ScriptVariant (string in)`



**Parameters:**
- `in` (System.String)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit_7.htm)

#### `public static implicit operator GH_ScriptVariant (Vector3d in)`



**Parameters:**
- `in` (Vector3d)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Implicit_2.htm)

#### `public static bool operator !=(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Inequality.htm)

#### `C# does not support this operator.`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_IntegerDivision.htm)

#### `public static GH_ScriptVariant operator <<(GH_ScriptVariant A, int B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (System.Int32)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_LeftShift.htm)

#### `public static bool operator <(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_LessThan.htm)

#### `public static bool operator <=(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_LessThanOrEqual.htm)

#### `public static GH_ScriptVariant operator %(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Modulus.htm)

#### `public static GH_ScriptVariant operator *(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Multiply.htm)

#### `public static GH_ScriptVariant operator ~(GH_ScriptVariant A)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_OnesComplement.htm)

#### `public static GH_ScriptVariant operator >>(GH_ScriptVariant A, int B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (System.Int32)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_RightShift.htm)

#### `public static GH_ScriptVariant operator -(GH_ScriptVariant A, GH_ScriptVariant B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_Subtraction.htm)

#### `protected static void ThrowNullOperatorException(string sName)`



**Parameters:**
- `sName` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_ThrowNullOperatorException.htm)

#### `protected static void ThrowOperatorException(string op_name, GH_ScriptVariantType A)`



**Parameters:**
- `op_name` (System.String)
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariantType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_ThrowOperatorException.htm)

#### `protected static void ThrowOperatorException(string op_name, GH_ScriptVariantType A, GH_ScriptVariantType B)`



**Parameters:**
- `op_name` (System.String)
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariantType)
- `B` (Grasshopper.Kernel.Expressions.GH_ScriptVariantType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_ThrowOperatorException_1.htm)

#### `protected static void ThrowOperatorException(string op_name, string A)`



**Parameters:**
- `op_name` (System.String)
- `A` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_ThrowOperatorException_2.htm)

#### `protected static void ThrowOperatorException(string op_name, string A, string B)`



**Parameters:**
- `op_name` (System.String)
- `A` (System.String)
- `B` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_ThrowOperatorException_3.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_ToString.htm)

#### `public static bool operator true(GH_ScriptVariant A)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_True.htm)

#### `public static GH_ScriptVariant operator -(GH_ScriptVariant A)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_UnaryNegation.htm)

#### `public static GH_ScriptVariant operator +(GH_ScriptVariant A)`



**Parameters:**
- `A` (Grasshopper.Kernel.Expressions.GH_ScriptVariant)

**Returns:** `GH_ScriptVariant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_ScriptVariant_op_UnaryPlus.htm)

### Properties
- `Boolean` (Boolean, get) — 
- `DateTime` (DateTime, get) — 
- `Double` (Double, get) — 
- `Integer` (Int32, get) — 
- `Object` (Object, get) — 
- `Plane` (Plane, get) — 
- `Point` (Point3d, get) — 
- `String` (String, get) — 
- `Type` (GH_ScriptVariantType, get) — 
- `Vector` (Vector3d, get) — 
- `m_type` (GH_ScriptVariantType, get) — 
- `m_value` (Object, get) — 

## GH_ScriptVariantType (enum)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_ScriptVariantType.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_ScriptVariantType.htm)

### Values
- `nothing` = `0`
- `boolean` = `1`
- `integer` = `2`
- `double` = `3`
- `string` = `5`
- `datetime` = `6`
- `point` = `10`
- `vector` = `11`
- `plane` = `12`
- `object` = `20`

## GH_SignatureException (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_SignatureException.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_SignatureException.htm)

### Constructors
- `public GH_SignatureException(List<GH_Variant> args, string name)` — Initializes a new instance of the GH_SignatureException class

### Properties
- `Message` (String, get) — (Overrides Exception.Message.)

## GH_SolverException (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_SolverException.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_SolverException.htm)

### Constructors
- `public GH_SolverException(string nMessage)` — Initializes a new instance of the GH_SolverException class

## GH_SyntaxException (class)

(No description provided in vendor docs for Grasshopper.Kernel.Expressions.GH_SyntaxException.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_SyntaxException.htm)

### Constructors
- `public GH_SyntaxException(string nMessage)` — Initializes a new instance of the GH_SyntaxException class

## GH_Variant (class)

Variant data used in Grasshopper Expressions.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_Variant.htm)

### Constructors
- `public GH_Variant()` — Create a new Null variant.
- `public GH_Variant(Plane value)` — Create a new Plane variant.
- `public GH_Variant(Point3d value)` — Create a new Point variant.
- `public GH_Variant(Vector3d value)` — Create a new Point variant.
- `public GH_Variant(bool value)` — Create a new Bool variant.
- `public GH_Variant(double value)` — Create a new Double variant.
- `public GH_Variant(int value)` — Create a new Integer variant.
- `public GH_Variant(string value)` — Create a new String variant.
- `public GH_Variant(Complex value)` — Create a new Complex variant.
- `public GH_Variant(GH_Variant other)` — Duplicate a Variant.

### Methods
#### `public T Data<T>()`

Perform a straight cast of the data inside this Variant.

**Returns:** `T` — The value inside this Variant cast to T.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_Variant_Data__1.htm)

#### `public GH_Variant Duplicate()`

Duplicate this Variant. If the type of this Variant is [unknown] the contents are not guaranteed to be duplicated.

**Returns:** `GH_Variant` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_Variant_Duplicate.htm)

#### `public bool Evaluate()`

If this variant represents a string, this function will attempt to evaluate that string and replace the data inside this variant.

**Returns:** `Boolean` — True if the string was evaluated, false if otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_Variant_Evaluate.htm)

#### `public IGH_Goo ToGoo()`

Attempt to convert a Variant to a Grasshopper IGH_Goo type.

**Returns:** `IGH_Goo` — An instance of a class implementing IGH_Goo or null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_Variant_ToGoo.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Expressions_GH_Variant_ToString.htm)

### Properties
- `_Bool` (Boolean, get) — Gets the contents of this Variant as a bool.
- `_Complex` (Complex, get) — Gets the contents of this Variant as a complex number.
- `_Double` (Double, get) — Gets the contents of this Variant as a double. If the Variant is of type Integer this cast will also work.
- `_Int` (Int32, get) — Gets the contents of this Variant as an integer.
- `_Plane` (Plane, get) — Gets the contents of this Variant as a Plane.
- `_Point` (Point3d, get) — Gets the contents of this Variant as a Point.
- `_String` (String, get) — Gets the contents of this Variant as a String. This function will work for all Variant types.
- `_Vector` (Vector3d, get) — Gets the contents of this Variant as a Vector.
- `IsNumeric` (Boolean, get) — Gets a value indicating whether or not the Variant is a numeric type. Only Doubles and Integers are considered to be Numeric.
- `Type` (GH_VariantType, get) — Gets the Type of this Variant.

## GH_VariantType (enum)

Lists all possible variable types supported in the Grasshopper Expression Parser.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Expressions_GH_VariantType.htm)

### Values
- `unknown` = `-1` — Unknown type.
- `null` = `0` — Variant is a null reference.
- `bool` = `1` — Variant is of type System.Boolean
- `int` = `2` — Variant is of type System.Int32
- `double` = `4` — Variant is of type System.Double
- `string` = `8` — Variant is of type System.String
- `point` = `16` — Variant is of type Rhino.Geometry.Point3d
- `plane` = `32` — Variant is of type Rhino.Geometry.Plane
- `complex` = `64` — Variant is of type Grasshopper.Types.GH_Complex

