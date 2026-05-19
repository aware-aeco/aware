---
name: rhino-rhino-nodeincode
description: This skill encodes the rhino 7.0 surface of the Rhino.NodeInCode namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Components, ComponentFunctionInfo, NodeInCodeTable.
---

# Rhino.NodeInCode

Auto-generated from vendor docs for rhino 7.0. 3 types in this namespace.

## ComponentFunctionInfo (class)

Defines the base class for a function representing a component. This class is abstract.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_NodeInCode_ComponentFunctionInfo.htm)

### Constructors
- `protected ComponentFunctionInfo(string name, string namespace, string description, IReadOnlyList<string> inputNames, IReadOnlyList<string> inputDescriptions, IReadOnlyList<string> inputTypeNames, IReadOnlyList<bool> inputsOptional, IReadOnlyList<string> outputNames, IReadOnlyList<string> outputDescriptions, IReadOnlyList<string> outputTypeNames, Guid componentGuid)` — Instantiates a new instance of the function class. This is not meant for public consumption.

### Methods
#### `public abstract Object[] Evaluate(IEnumerable args, bool keepTree, out string[] warnings)`

Evaluates the component with a set of arguments. There needs to be an argument for each input param, and each output param gives an entry in the output array.

**Remarks:** For component that store no state, this method is thread safe.

**Parameters:**
- `args` (System.Collections.IEnumerable) — The arguments list. Each item is assigned to each input param, in order.
- `keepTree` (System.Boolean) — A value indicating whether trees should be considered valid inputs, and should be returned. In this case, output variables are not simplified to common types.
- `warnings` (System.String[]) — A possible list of warnings, or null.

**Returns:** `Object[]` — An array of objects, each representing an output result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_ComponentFunctionInfo_Evaluate.htm)

#### `public Object[] Invoke(params Object[] args)`

Runs Evaluate(IEnumerable, Boolean, String[]) with keepTree equal to false, and raises an exception on the first warning.

**Parameters:**
- `args` (System.Object[]) — Arguments. One for each component input.

**Returns:** `Object[]` — Items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_ComponentFunctionInfo_Invoke.htm)

#### `public Object[] InvokeKeepTree(params Object[] args)`

Runs Evaluate(IEnumerable, Boolean, String[]) with keepTree equal to true and raises an exception on the first warning.

**Parameters:**
- `args` (System.Object[]) — Arguments. One for each component input.

**Returns:** `Object[]` — Items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_ComponentFunctionInfo_InvokeKeepTree.htm)

#### `public Object[] InvokeKeepTreeSilenceWarnings(params Object[] args)`

Runs Evaluate(IEnumerable, Boolean, String[]) with keepTree equal to true, and discards warnings (this is a dangerous operation!).

**Parameters:**
- `args` (System.Object[]) — Arguments.

**Returns:** `Object[]` — Array of items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_ComponentFunctionInfo_InvokeKeepTreeSilenceWarnings.htm)

#### `public Object[] InvokeSilenceWarnings(params Object[] args)`

Runs Evaluate(IEnumerable, Boolean, String[]) with keepTree equal to false, then simplifies output with SimplifyTreeOutput(Object[], String[], Boolean) and discards warnings (this is a dangerous operation!).

**Parameters:**
- `args` (System.Object[]) — Arguments. One for each component input.

**Returns:** `Object[]` — Items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_ComponentFunctionInfo_InvokeSilenceWarnings.htm)

#### `public override string ToString()`

Returns a string representing this function.

**Returns:** `String` — The namespace and the name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_ComponentFunctionInfo_ToString.htm)

### Properties
- `ComponentGuid` (Guid, get) — The unique identifier of the Grasshopper component. It is the original developer's responsibility to ensure that this ID is unique.
- `Delegate` (Delegate, get) — Returns a delegate that can be directly invoked using a list of arguments. This flattens trees.
- `DelegateNoWarnings` (Delegate, get) — Returns a delegate that can be directly invoked using a list of arguments. This flattens trees.
- `DelegateTree` (Delegate, get) — Returns a delegate that can be directly invoked using a list of arguments. This considers trees and simplifies single-output components.
- `DelegateTreeNoWarnings` (Delegate, get) — Returns a delegate that can be directly invoked using a list of arguments. This considers trees and simplifies single-output components.
- `Description` (String, get) — The function description.
- `FullName` (String, get) — Returns the name of the component prefixed by, if existing, the namespace.
- `FullScriptingName` (String, get) — Shows the full name of the component, including optional periods. Removes spaces and common operator signs.
- `InputDescriptions` (IReadOnlyList<String>, get) — The function input parameter descriptions.
- `InputNames` (IReadOnlyList<String>, get) — The function input parameter names.
- `InputsOptional` (IReadOnlyList<Boolean>, get) — Indications for each function input parameter whether it is optional.
- `InputTypeNames` (IReadOnlyList<String>, get) — The function input type names.
- `IsDefault` (Boolean, get) — Determines if the component is a default one.
- `Name` (String, get) — The function name.
- `Namespace` (String, get) — The function namespace.
- `OutputDescriptions` (IReadOnlyList<String>, get) — Grasshopper returns several items in general. This property returns the output descriptions.
- `OutputNames` (IReadOnlyList<String>, get) — Grasshopper returns several items in general. This property returns the output names.
- `OutputTypeNames` (IReadOnlyList<String>, get) — Grasshopper returns several items in general. This property returns the output type names.

## Components (class)

Provides access to all Grasshopper runtime components.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_NodeInCode_Components.htm)

### Constructors
- `protected Components()` — Initializes a new instance of the Components class

### Methods
#### `public static ComponentFunctionInfo FindComponent(string fullName)`

Finds a component given its full name.

**Parameters:**
- `fullName` (System.String) — The name, including its library name and a period if it is made by a third-party.

**Returns:** `ComponentFunctionInfo` — [Missing <returns> documentation for "M:Rhino.NodeInCode.Components.FindComponent(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_Components_FindComponent.htm)

#### `protected virtual NodeInCodeTable __getAllFunctions()`

Returns a collection with all component functions. For infrastructure usage only.

**Returns:** `NodeInCodeTable` — [Missing <returns> documentation for "M:Rhino.NodeInCode.Components.__getAllFunctions"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_Components___getAllFunctions.htm)

### Properties
- `NodeInCodeFunctions` (NodeInCodeTable, get) — Returns a collection with all component functions.

## NodeInCodeTable (class)

Permits rapid access to references to all Grasshopper functions.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_NodeInCode_NodeInCodeTable.htm)

### Constructors
- `public NodeInCodeTable(IEnumerable<ComponentFunctionInfo> items = null)` — Instantiates the table. Users of RhinoCommon do not typically need to call this constructor.

### Methods
#### `public void Add(ComponentFunctionInfo item)`

Adds, or replaces a new instance of component function information.

**Parameters:**
- `item` (Rhino.NodeInCode.ComponentFunctionInfo) — [Missing <param name="item"/> documentation for "M:Rhino.NodeInCode.NodeInCodeTable.Add(Rhino.NodeInCode.ComponentFunctionInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_NodeInCodeTable_Add.htm)

#### `public override IEnumerable<string> GetDynamicMemberNames()`

Returns all additional names in the table.

**Returns:** `IEnumerable<String>` — [Missing <returns> documentation for "M:Rhino.NodeInCode.NodeInCodeTable.GetDynamicMemberNames"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_NodeInCodeTable_GetDynamicMemberNames.htm)

#### `public override bool TryGetIndex(GetIndexBinder binder, Object[] indexes, out Object result)`

Gets the ComponentFunctionInfo at

**Parameters:**
- `binder` (System.Dynamic.GetIndexBinder) — The binder.
- `indexes` (System.Object[]) — ONE string index.
- `result` (System.Object) — The bound info.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.NodeInCode.NodeInCodeTable.TryGetIndex(System.Dynamic.GetIndexBinder,System.Object[],System.Object@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_NodeInCodeTable_TryGetIndex.htm)

#### `public override bool TryGetMember(GetMemberBinder binder, out Object result)`

Dynamically binds the table to property-like access via its item names.

**Parameters:**
- `binder` (System.Dynamic.GetMemberBinder) — The dynamic binder.
- `result` (System.Object) — Returns the result.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.NodeInCode.NodeInCodeTable.TryGetMember(System.Dynamic.GetMemberBinder,System.Object@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_NodeInCodeTable_TryGetMember.htm)

#### `public override bool TryInvokeMember(InvokeMemberBinder binder, Object[] args, out Object result)`

Dynamically invokes a member of the table.

**Parameters:**
- `binder` (System.Dynamic.InvokeMemberBinder) — The binder.
- `args` (System.Object[]) — The arguments.
- `result` (System.Object) — The result.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_NodeInCode_NodeInCodeTable_TryInvokeMember.htm)

### Properties
- `Count` (Int32, get) — Returns the amount of items in this table.

