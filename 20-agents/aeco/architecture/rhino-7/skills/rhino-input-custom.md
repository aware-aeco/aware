---
name: rhino-rhino-input-custom
description: This skill encodes the rhino 7.0 surface of the Rhino.Input.Custom namespace — 39 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CommandLineOption, GetArc, GetBaseClass, GetCancel, GetCircle, GetCylinder, GetCone, GetEllipsoid, and 31 more types.
---

# Rhino.Input.Custom

Auto-generated from vendor docs for rhino 7.0. 39 types in this namespace.

## CommandLineOption (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.CommandLineOption"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_CommandLineOption.htm)

### Methods
#### `public static bool IsValidOptionName(string optionName)`

Test a string to see if it can be used as an option name in any of the RhinoGet::AddCommandOption...() functions.

**Parameters:**
- `optionName` (System.String) — The string to be tested.

**Returns:** `Boolean` — true if string can be used as an option name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_CommandLineOption_IsValidOptionName.htm)

#### `public static bool IsValidOptionValueName(string optionValue)`

Test a string to see if it can be used as an option value in RhinoGet::AddCommandOption, RhinoGet::AddCommandOptionToggle, or RhinoGet::AddCommandOptionList.

**Parameters:**
- `optionValue` (System.String) — The string to be tested.

**Returns:** `Boolean` — true if string can be used as an option value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_CommandLineOption_IsValidOptionValueName.htm)

### Properties
- `CurrentListOptionIndex` (Int32, get) — 
- `EnglishName` (String, get) — The English command option name
- `Index` (Int32, get) — 
- `LocalName` (String, get) — The localized command option name
- `StringOptionValue` (String, get) — Assigned by RhinoGet.Get if an option value is specified in a script or by a command window control.

## ConeConstraint (enum)

[Missing <summary> documentation for "T:Rhino.Input.Custom.ConeConstraint"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_ConeConstraint.htm)

### Values
- `None` = `0`
- `Vertical` = `1`
- `AroundCurve` = `2`

## CylinderConstraint (enum)

[Missing <summary> documentation for "T:Rhino.Input.Custom.CylinderConstraint"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_CylinderConstraint.htm)

### Values
- `None` = `0`
- `Vertical` = `1`
- `AroundCurve` = `2`

## GeometryAttributeFilter (enum)

If an object passes the geometry TYPE filter, then the geometry ATTRIBUTE filter is applied.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GeometryAttributeFilter.htm)

### Values
- `WireCurve` = `1` — 3d wire curve If you want to accept only wire or edge curves, then specify wire_curve or edge_curve, otherwise both wire and edge curves will pass the attribute filter.
- `EdgeCurve` = `2` — 3d curve of a surface edge If you want to accept only wire or edge curves, then specify wire_curve or edge_curve, otherwise both wire and edge curves will pass the attribute filter.
- `ClosedCurve` = `4` — Closed Curves and Edges are acceptable If you want to accept only closed or open curves, then specify either closed_curve or open_curve. Otherwise both closed and open curves will pass the attribute filter.
- `OpenCurve` = `8` — Open Curves and Edges are acceptable If you want to accept only closed or open curves, then specify either closed_curve or open_curve. Otherwise both closed and open curves will pass the attribute filter.
- `SeamEdge` = `16` — seam edges are acceptable attributes of acceptable trimming edge objects (associated with an ON_BrepTrim). If none of these attributes are explicitly specified, then any kind of trimming edge will pass the attribute filter.
- `ManifoldEdge` = `32` — edges with 2 different surfaces pass attributes of acceptable trimming edge objects (associated with an ON_BrepTrim). If none of these attributes are explicitly specified, then any kind of trimming edge will pass the attribute filter.
- `NonmanifoldEdge` = `64` — edges with 3 or more surfaces pass attributes of acceptable trimming edge objects (associated with an ON_BrepTrim). If none of these attributes are explicitly specified, then any kind of trimming edge will pass the attribute filter.
- `MatedEdge` = `112` — any mated edge passes attributes of acceptable trimming edge objects (associated with an ON_BrepTrim). If none of these attributes are explicitly specified, then any kind of trimming edge will pass the attribute filter.
- `SurfaceBoundaryEdge` = `128` — boundary edges on surface sides pass attributes of acceptable trimming edge objects (associated with an ON_BrepTrim). If none of these attributes are explicitly specified, then any kind of trimming edge will pass the attribute filter.
- `TrimmingBoundaryEdge` = `256` — boundary edges that trim a surface pass attributes of acceptable trimming edge objects (associated with an ON_BrepTrim). If none of these attributes are explicitly specified, then any kind of trimming edge will pass the attribute filter.
- `BoundaryEdge` = `384` — ant boundary edge passes attributes of acceptable trimming edge objects (associated with an ON_BrepTrim). If none of these attributes are explicitly specified, then any kind of trimming edge will pass the attribute filter.
- `ClosedSurface` = `512` — If you want to accept only closed or open surfaces, then specify either closed_surface or open_surface. Otherwise both closed and open surfaces will pass the attribute filter.
- `OpenSurface` = `1024` — If you want to accept only closed or open surfaces, then specify either closed_surface or open_surface. Otherwise both closed and open surfaces will pass the attribute filter.
- `TrimmedSurface` = `2048` — If you want to accept only trimmed or untrimmed surfaces, then specify either trimmed_surface or untrimmed_surface. Otherwise both trimmed and untrimmed surfaces will pass the attribute filter.
- `UntrimmedSurface` = `4096` — If you want to accept only trimmed or untrimmed surfaces, then specify either trimmed_surface or untrimmed_surface. Otherwise both trimmed and untrimmed surfaces will pass the attribute filter.
- `SubSurface` = `8192` — If you want to accept only sub-surfaces of (multi-surface) polysurface, then specify sub_surface. If you do not want to accept sub-surfaces, then specify top_surface. Otherwise sub-surfaces and top surfaces will pass the attribute filter.
- `TopSurface` = `16384` — If you want to accept only sub-surfaces of (multi-surface) polysurface, then specify sub_surface. If you do not want to accept sub-surfaces, then specify top_surface. Otherwise sub-surfaces and top surfaces will pass the attribute filter.
- `ManifoldPolysrf` = `32768` — If you want to accept only manifold or non-manifold polysurfaces, then specify manifold_polysrf or nonmanifold_polysrf. Otherwise both manifold and non-manifold polysurfaces will pass the attribute filter.
- `NonmanifoldPolysrf` = `65536` — If you want to accept only manifold or non-manifold polysurfaces, then specify manifold_polysrf or nonmanifold_polysrf. Otherwise both manifold and non-manifold polysurfaces will pass the attribute filter.
- `ClosedPolysrf` = `131072` — If you want to accept only closed or open polysurfaces, then specify either closed_polysrf or open_polysrf. Otherwise both closed and open polysurfaces will pass the attribute filter.
- `OpenPolysrf` = `262144` — If you want to accept only closed or open polysurfaces, then specify either closed_polysrf or open_polysrf. Otherwise both closed and open polysurfaces will pass the attribute filter.
- `ClosedMesh` = `524288` — If you want to accept only closed or open meshes, then specify either closed_mesh or open_mesh. Otherwise both closed and open meshes will pass the attribute filter.
- `OpenMesh` = `1048576` — If you want to accept only closed or open meshes, then specify either closed_mesh or open_mesh. Otherwise both closed and open meshes will pass the attribute filter.
- `BoundaryInnerLoop` = `2097152` — all trimming edges are boundary edges.
- `MatedInnerLoop` = `4194304` — all trimming edges are mated.
- `InnerLoop` = `6291456` — any inner loop is acceptable.
- `BoundaryOuterLoop` = `8388608` — all trimming edges are boundary edges.
- `MatedOuterLoop` = `16777216` — all trimming edges are mated.
- `OuterLoop` = `25165824` — any outer loop is acceptable.
- `SpecialLoop` = `33554432` — slit, curve-on-surface, point-on-surface, etc.
- `AcceptAllAttributes` = `4294967295`

## GetArc (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.GetArc"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetArc.htm)

### Constructors
- `public GetArc()` — Initializes a new instance of the GetArc class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetArc_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetArc_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetArc_Finalize.htm)

#### `public Result Get(out Arc arc)`

Perform the 'get' operation.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — [Missing <param name="arc"/> documentation for "M:Rhino.Input.Custom.GetArc.Get(Rhino.Geometry.Arc@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetArc.Get(Rhino.Geometry.Arc@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetArc_Get.htm)

### Properties
- `AllowDeformable` (Boolean, get/set) — Allow for deformable options
- `DefaultRadius` (Double, get/set) — Default radius used for start and end radius
- `Deformable` (Boolean, get/set) — Is the deformable option set
- `DeformableDegree` (Int32, get/set) — 
- `DeformablePointCount` (Int32, get/set) — 

## GetBaseClass (class)

Base class for GetObject, GetPoint, GetSphere, etc. You will never directly create a GetBaseClass but you will use its member functions after calling GetObject.Gets(), GetPoint.Get(), and so on. Provides tools to set command prompt, set command options, and specify if the "get" can optionally accept numbers, nothing (pressing enter), and undo.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetBaseClass.htm)

### Constructors
- `protected GetBaseClass()` — Initializes a new instance of the GetBaseClass class

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`



**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public Object CustomMessage()`



**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

Releases all resources used by the GetBaseClass

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the GetBaseClass and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `public CommandLineOption Option()`



**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public static void PostCustomMessage(Object messageData)`



**Parameters:**
- `messageData` (System.Object) — [Missing <param name="messageData"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.PostCustomMessage(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PostCustomMessage.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

## GetCancel (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.GetCancel"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetCancel.htm)

### Constructors
- `public GetCancel()` — Initializes a new instance of the GetCancel class

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

#### `public Result Wait(Task task, RhinoDoc doc)`

Awaits a particular task to finish.

**Parameters:**
- `task` (System.Threading.Tasks.Task) — The task.
- `doc` (Rhino.RhinoDoc) — A document to set progress reporting.

**Returns:** `Result` — A result enumeration.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCancel_Wait.htm)

#### `public Result Wait<TResult>(Task<TResult> task, RhinoDoc doc)`

Awaits a particular task to finish.

**Parameters:**
- `task` (System.Threading.Tasks.Task<TResult>) — The task.
- `doc` (Rhino.RhinoDoc) — A document to set progress reporting.

**Returns:** `Result` — A result enumeration.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCancel_Wait__1.htm)

#### `public Result WaitAll(IEnumerable<Task> tasks, RhinoDoc doc)`

Awaits some tasks to finish.

**Parameters:**
- `tasks` (System.Collections.Generic.IEnumerable<Task>) — The tasks.
- `doc` (Rhino.RhinoDoc) — A document to set progress reporting.

**Returns:** `Result` — A result enumeration.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCancel_WaitAll.htm)

#### `public Result WaitAll<TResult>(IEnumerable<Task<TResult>> tasks, RhinoDoc doc)`

Awaits some tasks to finish.

**Parameters:**
- `tasks` (System.Collections.Generic.IEnumerable<Task<TResult>>) — The tasks.
- `doc` (Rhino.RhinoDoc) — A document to set progress reporting.

**Returns:** `Result` — A result enumeration.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCancel_WaitAll__1.htm)

### Properties
- `Progress` (IProgress<Double>, get) — 
- `ProgressMessage` (String, get/set) — 
- `ProgressReporting` (Boolean, get/set) — 
- `Token` (CancellationToken, get) — 

### Events
#### `TaskCompleted` (`System.EventHandler<TaskCompleteEventArgs>`)

**Signature:** `public event EventHandler<TaskCompleteEventArgs> TaskCompleted`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetCancel_TaskCompleted.htm)

## GetCircle (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.GetCircle"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetCircle.htm)

### Constructors
- `public GetCircle()` — Initializes a new instance of the GetCircle class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCircle_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCircle_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCircle_Finalize.htm)

#### `public Result Get(out Circle circle)`

Perform the 'get' operation.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — [Missing <param name="circle"/> documentation for "M:Rhino.Input.Custom.GetCircle.Get(Rhino.Geometry.Circle@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetCircle.Get(Rhino.Geometry.Circle@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCircle_Get.htm)

### Properties
- `AllowDeformable` (Boolean, get/set) — Allow for deformable options
- `DefaultSize` (Double, get/set) — Default radius or diameter (based on InDiameterMode)
- `Deformable` (Boolean, get/set) — Is the deformable option set
- `DeformableDegree` (Int32, get/set) — 
- `DeformablePointCount` (Int32, get/set) — 
- `InDiameterMode` (Boolean, get/set) — Determines if the "size" value is representing a radius or diameter

## GetCone (class)

Class provides user interface to define a cone.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetCone.htm)

### Constructors
- `public GetCone()` — Initializes a new instance of the GetCone class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCone_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCone_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCone_Finalize.htm)

#### `public Result Get(out Cone cone)`

Prompt for the getting of a cone.

**Parameters:**
- `cone` (Rhino.Geometry.Cone) — The cone geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCone_Get.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, out Cone cone)`

Prompt for the getting of a mesh cone.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `cone` (Rhino.Geometry.Cone) — The cone geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCone_GetMesh.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, ref int capStyle, out Cone cone)`

Prompt for the getting of a mesh cone.

**Remarks:** The prompt for capStyle will only be seen if it's not zero, aroundFaces is even and the solid option is on.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `capStyle` (System.Int32) — Set to 0 if you don't want the prompt, 3 is triangles, 4 is quads.
- `cone` (Rhino.Geometry.Cone) — The cone geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCone_GetMesh_1.htm)

### Properties
- `AllowInputAngle` (Boolean, get/set) — 
- `ApexAngleDegrees` (Double, get/set) — 
- `BaseAngleDegrees` (Double, get/set) — 
- `Cap` (Boolean, get/set) — Gets or sets whether or not the output should be capped.
- `ConeConstraint` (ConeConstraint, get/set) — State of the cone/cylinder constraint option. When the cone/cylinder option is selected, the circle is being made as a base for a cone/cylinder. By default the vertical cone/cylinder option not available but is not selected. By default the "Vertical" option applies to VerticalCircle.
- `DefaultSize` (Double, get/set) — Default radius or diameter (based on InDiameterMode)
- `Height` (Double, get/set) — 
- `InDiameterMode` (Boolean, get/set) — Determines if the "size" value is representing a radius or diameter

## GetCylinder (class)

Class provides user interface to define a cylinder.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetCylinder.htm)

### Constructors
- `public GetCylinder()` — Initializes a new instance of the GetCylinder class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCylinder_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCylinder_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCylinder_Finalize.htm)

#### `public Result Get(out Cylinder cylinder)`

Prompt for the getting of a cylinder.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — The cylinder geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCylinder_Get.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, out Cylinder cylinder)`

Prompt for the getting of a mesh cylinder.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `cylinder` (Rhino.Geometry.Cylinder) — The cylinder geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCylinder_GetMesh.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, ref int capStyle, out Cylinder cylinder)`

Prompt for the getting of a mesh cylinder.

**Remarks:** The prompt for capStyle will only be seen if it's not zero, aroundFaces is even and the solid option is on.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `capStyle` (System.Int32) — Set to 0 if you don't want the prompt, 3 is triangles, 4 is quads.
- `cylinder` (Rhino.Geometry.Cylinder) — The cylinder geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetCylinder_GetMesh_1.htm)

### Properties
- `BothSidesOption` (Boolean, get/set) — Determine if the "both sides" option is enabled
- `Cap` (Boolean, get/set) — Gets or sets whether or not the output should be capped.
- `CylinderConstraint` (CylinderConstraint, get/set) — State of the cone/cylinder constraint option. When the cone/cylinder option is selected, the circle is being made as a base for a cone/cylinder. By default the vertical cone/cylinder option not available but is not selected. By default the "Vertical" option applies to VerticalCircle.
- `DefaultSize` (Double, get/set) — Default radius or diameter (based on InDiameterMode)
- `Height` (Double, get/set) — Height of cylinder
- `InDiameterMode` (Boolean, get/set) — Determines if the "size" value is representing a radius or diameter

## GetEllipsoid (class)

Class provides user interface to define a truncated cone.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetEllipsoid.htm)

### Constructors
- `public GetEllipsoid()` — Initializes a new instance of the GetEllipsoid class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetEllipsoid_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetEllipsoid_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetEllipsoid_Finalize.htm)

#### `public Result Get(out NurbsSurface ellipsoid)`

Prompt for the getting of a ellipsoid.

**Parameters:**
- `ellipsoid` (Rhino.Geometry.NurbsSurface) — The truncated cone in NURB form.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetEllipsoid_Get.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, ref bool quadCaps, out Mesh ellipsoid)`

Prompt for the getting of a mesh ellipsoid.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `quadCaps` (System.Boolean) — Set true to create quad faces at the caps, false for triangles.
- `ellipsoid` (Rhino.Geometry.Mesh) — The ellipsoid in Mesh form.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetEllipsoid_GetMesh_1.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, out Mesh ellipsoid)`

Prompt for the getting of a mesh ellipsoid.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `ellipsoid` (Rhino.Geometry.Mesh) — The ellipsoid in Mesh form.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetEllipsoid_GetMesh.htm)

### Properties
- `FirstPoint` (Point3d, get) — Returns the first point. If in "from foci" mode, then this is the first foci point.
- `IsModeFromFoci` (Boolean, get) — Indicates the ellipsoid was created from foci.
- `MarkFoci` (Boolean, get/set) — Indicates the user wants the ellipsoid foci marked with point objects.
- `SecondPoint` (Point3d, get) — Returns the second point. If in "from foci" mode, then this is the second foci point.

## GetFileNameMode (enum)

[Missing <summary> documentation for "T:Rhino.Input.Custom.GetFileNameMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetFileNameMode.htm)

### Values
- `Open` = `0`
- `OpenTemplate` = `1`
- `OpenImage` = `2`
- `OpenRhinoOnly` = `3`
- `OpenTextFile` = `5`
- `OpenWorksession` = `6`
- `Import` = `7`
- `Attach` = `8`
- `LoadPlugIn` = `9`
- `Save` = `10`
- `SaveSmall` = `11`
- `SaveTemplate` = `12`
- `SaveImage` = `13`
- `Export` = `14`
- `SaveTextFile` = `17`
- `SaveWorksession` = `18`

## GetInteger (class)

Used to get integer numbers.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetInteger.htm)

### Constructors
- `public GetInteger()` — Initializes a new instance of the GetInteger class

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public GetResult Get()`

Call to get an integer.

**Returns:** `GetResult` — If the user chose a number, then Number; another enumeration value otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetInteger_Get.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public int Number()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetInteger.Number"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetInteger_Number.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetLowerLimit(int lowerLimit, bool strictlyGreaterThan)`

Sets a lower limit on the number that can be returned. By default there is no lower limit.

**Parameters:**
- `lowerLimit` (System.Int32) — smallest acceptable number.
- `strictlyGreaterThan` (System.Boolean) — If true, then the returned number will be > lower_limit.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetInteger_SetLowerLimit.htm)

#### `public void SetUpperLimit(int upperLimit, bool strictlyLessThan)`

Sets an upper limit on the number that can be returned. By default there is no upper limit.

**Parameters:**
- `upperLimit` (System.Int32) — largest acceptable number.
- `strictlyLessThan` (System.Boolean) — If true, then the returned number will be < upper_limit.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetInteger_SetUpperLimit.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

## GetLine (class)

Use to interactively get a line. The Rhino "Line" command uses GetLine.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetLine.htm)

### Constructors
- `public GetLine()` — Initializes a new instance of the GetLine class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_Dispose_1.htm)

#### `public void EnableAllVariations(bool on)`

If true, then all line variations are shown if the default line mode is used

**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Input.Custom.GetLine.EnableAllVariations(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_EnableAllVariations.htm)

#### `public void EnableFromBothSidesOption(bool on)`

If true, then the "BothSides" option shows up when the start point is interactively picked.

**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Input.Custom.GetLine.EnableFromBothSidesOption(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_EnableFromBothSidesOption.htm)

#### `public void EnableFromMidPointOption(bool on)`

If true, the "MidPoint" options shows up

**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Input.Custom.GetLine.EnableFromMidPointOption(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_EnableFromMidPointOption.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_Finalize.htm)

#### `public Result Get(out Line line)`

Perform the 'get' operation.

**Parameters:**
- `line` (Rhino.Geometry.Line) — [Missing <param name="line"/> documentation for "M:Rhino.Input.Custom.GetLine.Get(Rhino.Geometry.Line@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetLine.Get(Rhino.Geometry.Line@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_Get.htm)

#### `public void SetFirstPoint(Point3d point)`

Use SetFirstPoint to specify the line's starting point and skip the start point interactive picking

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — [Missing <param name="point"/> documentation for "M:Rhino.Input.Custom.GetLine.SetFirstPoint(Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetLine_SetFirstPoint.htm)

### Properties
- `AcceptZeroLengthLine` (Boolean, get/set) — Controls whether or not a zero length line is acceptable. The default is to require the user to keep picking the end point until we get a point different than the start point.
- `FeedbackColor` (Color, get/set) — If set, the feedback color is used to draw the dynamic line when the second point is begin picked. If not set, the active layer color is used.
- `FirstPointPrompt` (String, get/set) — Prompt when getting first point
- `FixedLength` (Double, get/set) — If FixedLength > 0, the line must have the specified length
- `GetLineMode` (GetLineMode, get/set) — Mode used
- `HaveFeedbackColor` (Boolean, get) — If true, the feedback color is used to draw the dynamic line when the second point is begin picked. If false, the active layer color is used.
- `MidPointPrompt` (String, get/set) — Prompt when getting midpoint
- `SecondPointPrompt` (String, get/set) — Prompt when getting second point

## GetLineMode (enum)

[Missing <summary> documentation for "T:Rhino.Input.Custom.GetLineMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetLineMode.htm)

### Values
- `TwoPoint` = `0`
- `SurfaceNormal` = `1`
- `Angled` = `2`
- `Vertical` = `3`
- `FourPoint` = `4`
- `Bisector` = `5`
- `Perpendicular` = `6`
- `Tangent` = `7`
- `CurveEnd` = `8`
- `CPlaneNormalVector` = `9`

## GetNumber (class)

Used to get double precision numbers.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetNumber.htm)

### Constructors
- `public GetNumber()` — Create a new GetNumber.

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public GetResult Get()`

Call to get a number.

**Returns:** `GetResult` — If the user chose a number, then Number; another enumeration value otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetNumber_Get.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetLowerLimit(double lowerLimit, bool strictlyGreaterThan)`

Sets a lower limit on the number that can be returned. By default there is no lower limit.

**Parameters:**
- `lowerLimit` (System.Double) — smallest acceptable number.
- `strictlyGreaterThan` (System.Boolean) — If true, then the returned number will be > lower_limit.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetNumber_SetLowerLimit.htm)

#### `public void SetUpperLimit(double upperLimit, bool strictlyLessThan)`

Sets an upper limit on the number that can be returned. By default there is no upper limit.

**Parameters:**
- `upperLimit` (System.Double) — largest acceptable number.
- `strictlyLessThan` (System.Boolean) — If true, then the returned number will be < upper_limit.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetNumber_SetUpperLimit.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

## GetObject (class)

The GetObject class is the tool commands use to interactively select objects.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetObject.htm)

### Constructors
- `public GetObject()` — Initializes a new instance of the GetObject class

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public static GetObject ActiveGetObject(RhinoDoc doc)`

Get the currently running GetObject for a given document

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Input.Custom.GetObject.ActiveGetObject(Rhino.RhinoDoc)"]

**Returns:** `GetObject` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetObject.ActiveGetObject(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_ActiveGetObject.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public void AppendToPickList(ObjRef objref)`



**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — [Missing <param name="objref"/> documentation for "M:Rhino.Input.Custom.GetObject.AppendToPickList(Rhino.DocObjects.ObjRef)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_AppendToPickList.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public void ClearObjects()`

Clear possible special object drawing

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_ClearObjects.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public virtual bool CustomGeometryFilter(RhinoObject rhObject, GeometryBase geometry, ComponentIndex componentIndex)`

Checks geometry to see if it can be selected. Override to provide fancy filtering.

**Remarks:** The delegate should not throw exceptions. If an exception is thrown, a message box will show and the filter will be disabled.

**Parameters:**
- `rhObject` (Rhino.DocObjects.RhinoObject) — parent object being considered.
- `geometry` (Rhino.Geometry.GeometryBase) — geometry being considered.
- `componentIndex` (Rhino.Geometry.ComponentIndex) — if >= 0, geometry is a proper sub-part of object->Geometry() with componentIndex.

**Returns:** `Boolean` — The default returns true unless you've set a custom geometry filter. If a custom filter has been set, that delegate is called.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_CustomGeometryFilter.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void DisablePreSelect()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_DisablePreSelect.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void EnableClearObjectsOnEntry(bool enable)`

By default the picked object list is cleared when GetObject.GetObjects() is called. If you are reusing a GetObject class and do not want the existing object list cleared when you call Input, then call EnableClearObjectsOnEntry(false) before calling GetObjects().

**Parameters:**
- `enable` (System.Boolean) — The state to set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnableClearObjectsOnEntry.htm)

#### `public void EnableHighlight(bool enable)`

By default, any object post-pick selected by GetObjects() is highlighted. If you want to post-pick objects and not have them automatically highlight, then call EnableHighlight = false.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetObject.EnableHighlight(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnableHighlight.htm)

#### `public void EnableIgnoreGrips(bool enable)`

By default, post selection will select objects with grips on. If you do not want to be able to post select objects with grips on, then call EnableIgnoreGrips = false. The ability to preselect an object with grips on is determined by the value returned by the virtual RhinoObject.IsSelectableWithGripsOn.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetObject.EnableIgnoreGrips(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnableIgnoreGrips.htm)

#### `public void EnablePostSelect(bool enable)`

Control the availability of post selection in GetObjects.

**Remarks:** By default, if no valid input is pre-selected when GetObjects is called, then the user is given the chance to post select. If you want to force the user to pre-select, then call EnablePostSelect(false).

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetObject.EnablePostSelect(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnablePostSelect.htm)

#### `public void EnablePreSelect(bool enable, bool ignoreUnacceptablePreselectedObjects)`

Control the pre-selection behavior GetObjects.

**Remarks:** By default, if valid input is pre-selected when GetObjects() is called, then that input is returned and the user is not given the opportunity to post-select. If you want to force the user to post-select, then call EnablePreSelect(false).

**Parameters:**
- `enable` (System.Boolean) — if true, pre-selection is enabled.
- `ignoreUnacceptablePreselectedObjects` (System.Boolean) — If true and some acceptable objects are pre-selected, then any unacceptable pre-selected objects are ignored. If false and any unacceptable are pre-selected, then the user is forced to post-select.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnablePreSelect.htm)

#### `public void EnablePressEnterWhenDonePrompt(bool enable)`

By default, when GetObject.GetObjects is called with minimumNumber > 0 and maximumNumber = 0, the command prompt automatically includes "Press Enter when done" after the user has selected at least minimumNumber of objects. If you want to prohibit the addition of the "Press Enter when done", then call EnablePressEnterWhenDonePrompt = false;

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetObject.EnablePressEnterWhenDonePrompt(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnablePressEnterWhenDonePrompt.htm)

#### `public void EnableSelPrevious(bool enable)`

By default, any object selected during a command becomes part of the "previous selection set" and can be reselected by the SelPrev command. If you need to select objects but do not want them to be selected by a subsequent call to SelPrev, then call EnableSelPrev = false.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetObject.EnableSelPrevious(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnableSelPrevious.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `public void EnableUnselectObjectsOnExit(bool enable)`

By default any objects in the object list are unselected when GetObject.GetObjects() exits with any return code besides Object. If you want to leave the objects selected when non-object input is returned, then call EnableUnselectObjectsOnExit(false) before calling GetObjects().

**Parameters:**
- `enable` (System.Boolean) — The state to set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_EnableUnselectObjectsOnExit.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public GetResult Get()`

Call to select a single object.

**Returns:** `GetResult` — GetResult.Object if an object was selected. GetResult.Cancel if the user pressed ESCAPE to cancel the selection. See GetResults for other possible values that may be returned when options, numbers, etc., are acceptable responses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_Get.htm)

#### `public GetResult GetMultiple(int minimumNumber, int maximumNumber)`

Call to select objects.

**Parameters:**
- `minimumNumber` (System.Int32) — minimum number of objects to select.
- `maximumNumber` (System.Int32) — maximum number of objects to select. If 0, then the user must press enter to finish object selection. If -1, then object selection stops as soon as there are at least minimumNumber of object selected. If >0, then the picking stops when there are maximumNumber objects. If a window pick, crossing pick, or Sel* command attempts to add more than maximumNumber, then the attempt is ignored.

**Returns:** `GetResult` — GetResult.Object if one or more objects were selected. GetResult.Cancel if the user pressed ESCAPE to cancel the selection. See GetResults for other possible values that may be returned when options, numbers, etc., are acceptable responses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_GetMultiple.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `public ObjRef Object(int index)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Input.Custom.GetObject.Object(System.Int32)"]

**Returns:** `ObjRef` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetObject.Object(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_Object.htm)

#### `public ObjRef[] Objects()`



**Returns:** `ObjRef[]` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetObject.Objects"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_Objects.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public bool PassesGeometryAttributeFilter(RhinoObject rhObject, GeometryBase geometry, ComponentIndex componentIndex)`

Checks geometry to see if it passes the basic GeometryAttributeFilter.

**Parameters:**
- `rhObject` (Rhino.DocObjects.RhinoObject) — parent object being considered.
- `geometry` (Rhino.Geometry.GeometryBase) — geometry being considered.
- `componentIndex` (Rhino.Geometry.ComponentIndex) — if >= 0, geometry is a proper sub-part of object->Geometry() with componentIndex.

**Returns:** `Boolean` — true if the geometry passes the filter returned by GeometryAttributeFilter().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_PassesGeometryAttributeFilter.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetCustomGeometryFilter(GetObjectGeometryFilter filter)`

Set filter callback function that will be called by the CustomGeometryFilter

**Parameters:**
- `filter` (Rhino.Input.Custom.GetObjectGeometryFilter) — [Missing <param name="filter"/> documentation for "M:Rhino.Input.Custom.GetObject.SetCustomGeometryFilter(Rhino.Input.Custom.GetObjectGeometryFilter)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_SetCustomGeometryFilter.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetPressEnterWhenDonePrompt(string prompt)`

The default prompt when EnablePressEnterWhenDonePrompt is enabled is "Press Enter when done". Use this function to specify a different string to be appended.

**Parameters:**
- `prompt` (System.String) — The text that will be displayed just after the prompt, after the selection has been made.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetObject_SetPressEnterWhenDonePrompt.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

### Properties
- `AlreadySelectedObjectSelect` (Boolean, get/set) — Allow selecting objects that are already selected. By default, GetObjects() disallows selection of objects that are already selected to avoid putting the same object in the selection set more than once. Calling EnableAlreadySelectedObjectSelect = true overrides that restriction and allows selected objects to be selected and returned by GetObjects. This is useful because, coupled with the return immediately mode of GetObjects(1, -1), it is possible to select a selected object to deselect when the selected objects are being managed outside GetObjects() as in the case of CRhinoPolyEdge::GetEdge().
- `BottomObjectPreference` (Boolean, get/set) — By default, if a call to Input is permitted to select different parts of the same object, like a polysurface, a surface and an edge, then the top-most object is preferred. (polysurface beats face beats edge). If you want the bottom most object to be preferred, then call EnableBottomObjectPreference = true before calling GetObjects().
- `ChooseOneQuestion` (Boolean, get/set) — By default, if a call to Input is permitted to select different parts of the same object, like a polysurface and an edge of that polysurface, then the top-most object is automatically selected. If you want the choose-one-object mechanism to include pop up in these cases, then call EnableChooseOneQuestion = true before calling GetObjects().
- `DeselectAllBeforePostSelect` (Boolean, get/set) — true if pre-selected input will be deselected before post-selection begins when no pre-selected input is valid.
- `GeometryAttributeFilter` (GeometryAttributeFilter, get/set) — The geometry attribute filter provides a secondary filter that can be used to restrict which objects can be selected. Control of the type of geometry (points, curves, surfaces, meshes, etc.) is provided by GetObject.SetGeometryFilter. The geometry attribute filter is used to require the selected geometry to have certain attributes (open, closed, etc.). The default attribute filter permits selection of all types of geometry.
- `GeometryFilter` (ObjectType, get/set) — The geometry type filter controls which types of geometry (points, curves, surfaces, meshes, etc.) can be selected. The default geometry type filter permits selection of all types of geometry. NOTE: the filter can be a bitwise combination of multiple ObjectTypes.
- `GroupSelect` (Boolean, get/set) — By default, groups are ignored in GetObject. If you want your call to GetObjects() to select every object in a group that has any objects selected, then enable group selection.
- `InactiveDetailPickEnabled` (Boolean, get/set) — By default, objects in inactive details are not permitted to be picked. In a few rare cases this is used (ex. picking circles during DimRadius)
- `ObjectCount` (Int32, get) — Gets the number of objects that were selected.
- `ObjectsWerePreselected` (Boolean, get) — 
- `OneByOnePostSelect` (Boolean, get/set) — In one-by-one post selection, the user is forced to select objects by post picking them one at a time.
- `ProxyBrepFromSubD` (Boolean, get/set) — If a subd (or a subd component) cannot be selected, but a brep (or brep component) can be selected, then automatically create and use a proxy brep.
- `ReferenceObjectSelect` (Boolean, get/set) — By default, reference objects can be selected. If you do not want to be able to select reference objects, then call EnableReferenceObjectSelect=false.
- `SerialNumber` (UInt32, get) — Each instance of GetObject has a unique runtime serial number that is used to identify object selection events associated with that instance.
- `SubObjectSelect` (Boolean, get/set) — By default, GetObject.Input will permit a user to select sub-objects (like a curve in a b-rep or a curve in a group). If you only want the user to select "top" level objects, then call EnableSubObjectSelect = false.

## GetObjectGeometryFilter (delegate)

[Missing <summary> documentation for "T:Rhino.Input.Custom.GetObjectGeometryFilter"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetObjectGeometryFilter.htm)

## GetOption (class)

If you want to explicitly get string input, then use GetString class with options. If you only want to get options, then use this class (GetOption)

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetOption.htm)

### Constructors
- `public GetOption()` — Initializes a new instance of the GetOption class

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public GetResult Get()`

Call to get an option. A return value of "option" means the user selected a valid option. Use Option() the determine which option.

**Returns:** `GetResult` — If the user chose an option, then Option; another enumeration value otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetOption_Get.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

## GetPoint (class)

Used to interactively get a point.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetPoint.htm)

### Constructors
- `public GetPoint()` — Create a new GetPoint.

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddConstructionPoint(Point3d point)`

Adds a point to the list of construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point to be added.

**Returns:** `Int32` — Total number of construction points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddConstructionPoint.htm)

#### `public int AddConstructionPoints(Point3d[] points)`

Adds points to the list of construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

**Parameters:**
- `points` (Rhino.Geometry.Point3d[]) — An array of points to be added.

**Returns:** `Int32` — Total number of construction points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddConstructionPoints.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public int AddSnapPoint(Point3d point)`

Adds a point to the list of osnap points.

**Remarks:** When point osnap is enabled, GetPoint will snap to points in the Rhino model. If you want the user to be able to snap to additional points, then use GetPoint::AddSnapPoints to specify the locations of these additional points.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point.

**Returns:** `Int32` — Total number of snap points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddSnapPoint.htm)

#### `public int AddSnapPoints(Point3d[] points)`

Adds points to the list of osnap points.

**Remarks:** When point osnap is enabled, GetPoint will snap to points in the Rhino model. If you want the user to be able to snap to additional points, then use GetPoint::AddSnapPoints to specify the locations of these additional points.

**Parameters:**
- `points` (Rhino.Geometry.Point3d[]) — An array of points to snap onto.

**Returns:** `Int32` — Total number of snap points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddSnapPoints.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearConstraints()`

Removes any explicit constraints added by calls to GetPoint::Constraint() and enable the built-in constraint options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ClearConstraints.htm)

#### `public void ClearConstructionPoints()`

Remove all construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ClearConstructionPoints.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public void ClearSnapPoints()`

Remove all snap points.

**Remarks:** When point osnap is enabled, GetPoint will snap to points in the Rhino model. If you want the user to be able to snap to additional points, then use GetPoint::AddSnapPoints to specify the locations of these additional points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ClearSnapPoints.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public bool Constrain(Arc arc)`

Constrains the picked point to lie on an arc.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — An arc to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain.htm)

#### `public bool Constrain(Brep brep, int wireDensity, int faceIndex, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a brep.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A brep to use as constraint.
- `wireDensity` (System.Int32) — When wire_density<0, isocurve intersection snapping is turned off, when wire_density>=0, the value defines the isocurve density used for isocurve intersection snapping.
- `faceIndex` (System.Int32) — When face_index <0, constrain to whole brep. When face_index >=0, constrain to individual face.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_1.htm)

#### `public bool Constrain(Circle circle)`

Constrains the picked point to lie on a circle.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — A circle to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_2.htm)

#### `public bool Constrain(Curve curve, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a curve.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A curve to use as constraint.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_3.htm)

#### `public bool Constrain(Cylinder cylinder)`

Constrains the picked point to lie on a cylinder.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — A cylinder to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_4.htm)

#### `public bool Constrain(Line line)`

Constrains the picked point to lie on a line.

**Parameters:**
- `line` (Rhino.Geometry.Line) — A line to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_5.htm)

#### `public bool Constrain(Mesh mesh, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to use as constraint.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_6.htm)

#### `public bool Constrain(Plane plane, bool allowElevator)`

constrain the picked point to lie on a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane to use as constraint.
- `allowElevator` (System.Boolean) — true if elevator mode should be allowed at user request.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_7.htm)

#### `public bool Constrain(Point3d from, Point3d to)`

Constrains the picked point to lie on a line.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — The start point of constraint.
- `to` (Rhino.Geometry.Point3d) — The end point of constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_8.htm)

#### `public bool Constrain(Sphere sphere)`

Constrains the picked point to lie on a sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — A sphere to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_9.htm)

#### `public bool Constrain(Surface surface, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a surface.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — A surface to use as constraint.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_10.htm)

#### `public void ConstrainDistanceFromBasePoint(double distance)`

Sets distance constraint from base point.

**Remarks:** If the base point is set and the distance from base point constraint is > 0, then the picked point is constrained to be this distance from the base point.

**Parameters:**
- `distance` (System.Double) — pass UnsetValue to clear this constraint. Pass 0.0 to disable the ability to set this constraint by typing a number during GetPoint.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainDistanceFromBasePoint.htm)

#### `public bool ConstrainToConstructionPlane(bool throughBasePoint)`

If enabled, the picked point is constrained to be on the active construction plane. If the base point is set, then the point is constrained to be on the plane that contains the base point and is parallel to the active construction plane. By default this constraint is enabled.

**Parameters:**
- `throughBasePoint` (System.Boolean) — true if the base point should be used as compulsory level reference.

**Returns:** `Boolean` — If true and the base point is set, then the point is constrained to be on the plane parallel to the construction plane that passes through the base point, even when planar mode is off. If throughBasePoint is false, then the base point shift only happens if planar mode is on.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainToConstructionPlane.htm)

#### `public void ConstrainToTargetPlane()`

Constrains point to lie on a plane that is parallel to the viewing plane and passes through the view's target point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainToTargetPlane.htm)

#### `public bool ConstrainToVirtualCPlaneIntersection(Plane plane)`

If enabled, the picked point is constrained to be on the intersection of the plane and the virtual CPlane going through the plane origin. If the planes are parallel, the constraint works just like planar constraint.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The plane used for the plane - virtual CPlane intersection.

**Returns:** `Boolean` — true if the operation succeeded; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainToVirtualCPlaneIntersection.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void DrawLineFromPoint(Point3d startPoint, bool showDistanceInStatusBar)`

Use DrawLineFromPoint() if you want a dynamic line drawn from a point to the point being picked.

**Remarks:** Calling DrawLineFromPoint automatically enables drawing the line. Use EnableDrawLineFromPoint() to toggle the line drawing state.

**Parameters:**
- `startPoint` (Rhino.Geometry.Point3d) — The line is drawn from startPoint to the point being picked. If the base point has not been set, then it is set to startPoint.
- `showDistanceInStatusBar` (System.Boolean) — if true, the distance from the basePoint to the point begin picked is shown in the status bar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_DrawLineFromPoint.htm)

#### `public void EnableCurveSnapArrow(bool drawDirectionArrowAtSnapPoint, bool reverseArrow)`

Controls display of the curve snap arrow icon.

**Remarks:** The tangent bar is drawn by GetPoint::DynamicDraw. If you override GetPoint::DynamicDraw, then you must call the base class function.

**Parameters:**
- `drawDirectionArrowAtSnapPoint` (System.Boolean) — true to draw arrow icon whenever GetPoint snaps to a curve.
- `reverseArrow` (System.Boolean) — true if arrow icon direction should be the reverse of the first derivative direction.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableCurveSnapArrow.htm)

#### `public void EnableCurveSnapPerpBar(bool drawPerpBarAtSnapPoint, bool drawEndPoints)`

Controls display of the curve snap perpendicular bar icon.

**Parameters:**
- `drawPerpBarAtSnapPoint` (System.Boolean) — true to draw a tangent bar icon whenever GetPoint snaps to a curve.
- `drawEndPoints` (System.Boolean) — true to draw points at the end of the tangent bar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableCurveSnapPerpBar.htm)

#### `public void EnableCurveSnapTangentBar(bool drawTangentBarAtSnapPoint, bool drawEndPoints)`

Controls display of the curve snap tangent bar icon.

**Remarks:** The tangent bar is drawn by GetPoint::DynamicDraw. If you override GetPoint::DynamicDraw, then you must call the base class function.

**Parameters:**
- `drawTangentBarAtSnapPoint` (System.Boolean) — true to draw a tangent bar icon whenever GetPoint snaps to a curve.
- `drawEndPoints` (System.Boolean) — true to draw points at the end of the tangent bar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableCurveSnapTangentBar.htm)

#### `public void EnableDrawLineFromPoint(bool enable)`

Controls drawing of dynamic a line from the start point.

**Parameters:**
- `enable` (System.Boolean) — if true, a dynamic line is drawn from the DrawLineFromPoint startPoint to the point being picked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableDrawLineFromPoint.htm)

#### `public void EnableNoRedrawOnExit(bool noRedraw)`

The default functionality of the GetPoint operation is to perform a redraw on exit. Calling this function with true turns off automatic redraw at the end of GetPoint. May be needed in some commands for flicker free feedback. When set to true, the caller is responsible for cleaning up the screen after GetPoint.

**Parameters:**
- `noRedraw` (System.Boolean) — [Missing <param name="noRedraw"/> documentation for "M:Rhino.Input.Custom.GetPoint.EnableNoRedrawOnExit(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableNoRedrawOnExit.htm)

#### `public void EnableObjectSnapCursors(bool enable)`

Enables or disables object snap cursors. By default, object snap cursors are enabled.

**Parameters:**
- `enable` (System.Boolean) — If true then object snap cursors (plus sign with "near", "end", etc.) are used when the point snaps to a object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableObjectSnapCursors.htm)

#### `public void EnableSnapToCurves(bool enable)`

If you want GetPoint() to try to snap to curves when the mouse is near a curve (like the center point in the Circle command when the AroundCurve option is on), then enable the snap to curves option.

**Parameters:**
- `enable` (System.Boolean) — Whether points should be enabled.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableSnapToCurves.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public GetResult Get()`

After setting up options and so on, call GetPoint::Get to get a 3d point. The point is retrieved when the mouse goes down.

**Returns:** `GetResult` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetPoint.Get"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Get.htm)

#### `public GetResult Get(bool onMouseUp)`

After setting up options and so on, call this method to get a 3d point.

**Parameters:**
- `onMouseUp` (System.Boolean) — If false, the point is returned when the left mouse button goes down. If true, the point is returned when the left mouse button goes up.

**Returns:** `GetResult` — Point if the user chose a point; other enumeration value otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Get_1.htm)

#### `public GetResult Get(bool onMouseUp, bool get2DPoint)`

After setting up options and so on, call this method to get a 2d or 3d point.

**Parameters:**
- `onMouseUp` (System.Boolean) — If false, the point is returned when the left mouse button goes down. If true, the point is returned when the left mouse button goes up.
- `get2DPoint` (System.Boolean) — If true then get a 2d point otherwise get a 2d point

**Returns:** `GetResult` — Point if the user chose a 3d point; Point2d if the user chose a 2d point; other enumeration value otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Get_2.htm)

#### `public Point3d[] GetConstructionPoints()`

Gets current construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

**Returns:** `Point3d[]` — An array of points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_GetConstructionPoints.htm)

#### `public bool GetPlanarConstraint(ref RhinoViewport vp, out Plane plane)`



**Parameters:**
- `vp` (Rhino.Display.RhinoViewport) — [Missing <param name="vp"/> documentation for "M:Rhino.Input.Custom.GetPoint.GetPlanarConstraint(Rhino.Display.RhinoViewport@,Rhino.Geometry.Plane@)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.Input.Custom.GetPoint.GetPlanarConstraint(Rhino.Display.RhinoViewport@,Rhino.Geometry.Plane@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetPoint.GetPlanarConstraint(Rhino.Display.RhinoViewport@,Rhino.Geometry.Plane@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_GetPlanarConstraint.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public Point3d[] GetSnapPoints()`

Gets current snap points.

**Returns:** `Point3d[]` — An array of points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_GetSnapPoints.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public bool InterruptMouseMove()`

If you have lengthy computations in OnMouseMove() and/or DymanicDraw() overrides, then periodically call InterruptMouseMove() to see if you should interrupt your work because the mouse has moved again.

**Returns:** `Boolean` — true if you should interrupt your work; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_InterruptMouseMove.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `protected virtual void OnDynamicDraw(GetPointDrawEventArgs e)`

Default calls the DynamicDraw event.

**Parameters:**
- `e` (Rhino.Input.Custom.GetPointDrawEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnDynamicDraw.htm)

#### `protected virtual void OnMouseDown(GetPointMouseEventArgs e)`

Default calls the MouseDown event.

**Parameters:**
- `e` (Rhino.Input.Custom.GetPointMouseEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnMouseDown.htm)

#### `protected virtual void OnMouseMove(GetPointMouseEventArgs e)`

Calls the MouseMove event and can/should be called by overriding implementation.

**Parameters:**
- `e` (Rhino.Input.Custom.GetPointMouseEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnMouseMove.htm)

#### `protected virtual void OnPostDrawObjects(DrawEventArgs e)`

In the "rare" case that you need to draw some depth buffered geometry during a GetPoint operation, override the OnPostDrawObjects function. NOTE!! Overriding this function comes with a significant performance penalty because the scene needs to be fully regenerated every frame where the standard DynamicDraw event draws temporary decorations (geometry) on top of a static scene.

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnPostDrawObjects.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public void PermitConstraintOptions(bool permit)`

Control the availability of the built-in linear, planar, curve, and surface constraint options like "Along", "AlongPerp", "AlongTan", "AlongParallel", "Between", "OnCrv", "OnSrf", ".x", ".y", ".z", ".xy", etc.

**Remarks:** By default, these built-in constraint options are available unless an explicit constraint is added by calling one of the GetPoint::Constrain functions. Calling GetPoint::ClearConstraints automatically enables the built-in constraint options. The built-in constraint options are never visible on the command line and the user must type the complete option name to activate these options.

**Parameters:**
- `permit` (System.Boolean) — if true, then the built-in constraint options are automatically available in GetPoint.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitConstraintOptions.htm)

#### `public void PermitElevatorMode(int permitMode)`

Permits the use of the control key to define a line constraint.

**Parameters:**
- `permitMode` (System.Int32) — 0: no elevator modes are permitted 1: fixed plane elevator mode (like the Line command) 2: cplane elevator mode (like object dragging)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitElevatorMode.htm)

#### `public void PermitFromOption(bool permit)`

Control the availability of the built-in "From" option. By default, the "From" option is enabled.

**Remarks:** The GetPoint "From" option is never visible on the command line and the user must type the complete option name to activate the "From" option. When the GetPoint "From" snap is enabled, the user set/change the base point during GetPoint by typing "From" and picking a point. A related option is the built-in distance from base point constraint that is can be set before GetPoint is called by passing a value to GetPoint::ConstrainDistanceFromBasePoint or during GetPoint by entering a number.

**Parameters:**
- `permit` (System.Boolean) — if true, then the "From" option is automatically available in GetPoint.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitFromOption.htm)

#### `public void PermitObjectSnap(bool permit)`

By default, object snaps like "end", "near", etc. are controlled by the user. If you want to disable this ability, then call PermitObjectSnap(false).

**Parameters:**
- `permit` (System.Boolean) — true to permit snapping to objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitObjectSnap.htm)

#### `public void PermitOrthoSnap(bool permit)`

Controls availability of ortho snap. Default is true.

**Parameters:**
- `permit` (System.Boolean) — if true, then GetPoint pays attention to the Rhino "ortho snap" and "planar snap" settings reported by ModelAidSettings.Ortho and ModelAidSettings.Planar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitOrthoSnap.htm)

#### `public void PermitTabMode(bool permit)`

Permits the use of the tab key to define a line constraint.

**Remarks:** By default, use of the tab key is supported.

**Parameters:**
- `permit` (System.Boolean) — If true, then the built-in tab key mode is available.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitTabMode.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public BrepFace PointOnBrep(out double u, out double v)`

Use to determine if point was on a Brep face. If the point was on a Brep face, then the (u,v) are the face parameters for the point.

**Parameters:**
- `u` (System.Double) — If the point was on a Brep face, then the u parameter.
- `v` (System.Double) — If the point was on a Brep face, then the v parameter.

**Returns:** `BrepFace` — The Brep face or null if the point was not on a Brep face.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnBrep.htm)

#### `public Curve PointOnCurve(out double t)`

Use to determine is point was on a curve.

**Parameters:**
- `t` (System.Double) — If the point was on a curve, then the t is the curve parameter for the point. The point returned by Point() is the same as curve.PointAt(t).

**Returns:** `Curve` — A curve at a specified parameter value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnCurve.htm)

#### `public ObjRef PointOnObject()`

Call this function to see if the point was on an object. If the point was on an object an ObjRef is returned; otherwise null is returned.

**Returns:** `ObjRef` — A point object reference.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnObject.htm)

#### `public Surface PointOnSurface(out double u, out double v)`

Use to determine if point was on a surface. If the point was on a surface, then the (u,v) are the surface parameters for the point. The point returned by Point() is the same as surface.PointAt(u,v).

**Parameters:**
- `u` (System.Double) — If the point was on a surface, then the u parameter.
- `v` (System.Double) — If the point was on a surface, then the v parameter.

**Returns:** `Surface` — The surface or null if the point was not on a surface.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnSurface.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetBasePoint(Point3d basePoint, bool showDistanceInStatusBar)`

Sets a base point used by ortho snap, from snap, planar snap, etc.

**Parameters:**
- `basePoint` (Rhino.Geometry.Point3d) — The new base point.
- `showDistanceInStatusBar` (System.Boolean) — If true, then the distance from base_point to the current point will be in the status bar distance pane.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_SetBasePoint.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetCursor(CursorStyle cursor)`

Sets cursor that will be used when Get() is called and snap is not happening.

**Parameters:**
- `cursor` (Rhino.UI.CursorStyle) — [Missing <param name="cursor"/> documentation for "M:Rhino.Input.Custom.GetPoint.SetCursor(Rhino.UI.CursorStyle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_SetCursor.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public bool TryGetBasePoint(out Point3d basePoint)`



**Parameters:**
- `basePoint` (Rhino.Geometry.Point3d) — [Missing <param name="basePoint"/> documentation for "M:Rhino.Input.Custom.GetPoint.TryGetBasePoint(Rhino.Geometry.Point3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetPoint.TryGetBasePoint(Rhino.Geometry.Point3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_TryGetBasePoint.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

### Properties
- `DynamicDrawColor` (Color, get/set) — Color used by CRhinoGetPoint::DynamicDraw to draw the current point and the line from the base point to the current point.
- `FullFrameRedrawDuringGet` (Boolean, get/set) — In the "RARE" case that you need to draw some depth buffered geometry during a Get() operation, setting this value to true will force entire frames to be redrawn while the user moves the mouse. This allows DisplayPipeline events to be triggered as well as OnPostDrawObjects NOTE!! Setting this value to true comes with a significant performance penalty because the scene needs to be fully regenerated every frame where the standard DynamicDraw event draws temporary decorations (geometry) on top of a static scene.
- `OsnapEventType` (OsnapModes, get) — Gets the type of object snap used to obtain the point.
- `Tag` (Object, get/set) — Gets or sets an arbitrary object that can be attached to this GetPoint instance. Useful for passing some/ information that you may need in a DynamicDraw event since you can get at this Tag from the GetPointDrawEventArgs.

### Events
#### `DynamicDraw` (`System.EventHandler<GetPointDrawEventArgs>`)

**Signature:** `public event EventHandler<GetPointDrawEventArgs> DynamicDraw`

Event to use if you want to dynamically draw things as the mouse/digitizer moves. Every time the mouse moves, DynamicDraw will be called once per viewport. The calls to DynamicDraw happen AFTER the call to MouseMove. If you are drawing anything that takes a long time, periodically call InterruptMouseMove() to see if you should stop.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_DynamicDraw.htm)

#### `MouseDown` (`System.EventHandler<GetPointMouseEventArgs>`)

**Signature:** `public event EventHandler<GetPointMouseEventArgs> MouseDown`

Called during Get2dRectangle, Get2dLine, and GetPoint(..,true) when the mouse down event for the initial point occurs. This function is not called during ordinary point getting because the mouse down event terminates an ordinary point get and returns a GetResult.Point result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_MouseDown.htm)

#### `MouseMove` (`System.EventHandler<GetPointMouseEventArgs>`)

**Signature:** `public event EventHandler<GetPointMouseEventArgs> MouseMove`

Called every time the mouse moves. MouseMove is called once per mouse move and is called BEFORE any calls to OnDynamicDraw. If you are doing anything that takes a long time, periodically call InterruptMouseMove() to see if you should stop. If the view is such that the 2d screen point can't be mapped to a 3d point, the 'point' argument will be Unset.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_MouseMove.htm)

#### `PostDrawObjects` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public event EventHandler<DrawEventArgs> PostDrawObjects`

Same as the DisplayPipeline.PostDrawObjects, but only works during the operation of the Get() function. NOTE: You must set FullFrameRedrawDuringGet to true in order for this event to be called.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_PostDrawObjects.htm)

## GetPointDrawEventArgs (class)

Arguments for drawing during point getting.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetPointDrawEventArgs.htm)

### Properties
- `CurrentPoint` (Point3d, get) — 
- `Display` (DisplayPipeline, get) — (Inherited from DrawEventArgs.)
- `RhinoDoc` (RhinoDoc, get) — (Inherited from DrawEventArgs.)
- `Source` (GetPoint, get) — GetPoint class that this draw event originated from.
- `Viewport` (RhinoViewport, get) — (Inherited from DrawEventArgs.)

## GetPointMouseEventArgs (class)

Arguments for mouse information during point getting.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetPointMouseEventArgs.htm)

### Properties
- `ControlKeyDown` (Boolean, get) — 
- `LeftButtonDown` (Boolean, get) — 
- `MiddleButtonDown` (Boolean, get) — 
- `Point` (Point3d, get) — 
- `RightButtonDown` (Boolean, get) — 
- `ShiftKeyDown` (Boolean, get) — 
- `Source` (GetPoint, get) — 
- `Viewport` (RhinoViewport, get) — 
- `WindowPoint` (Point, get) — 

## GetPolyline (class)

Use to interactively get a polyline.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetPolyline.htm)

### Constructors
- `public GetPolyline()` — Initializes a new instance of the GetPolyline class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPolyline_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPolyline_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPolyline_Finalize.htm)

#### `public Result Get(out Polyline polyline)`

Perform the 'get' operation.

**Parameters:**
- `polyline` (Rhino.Geometry.Polyline) — [Missing <param name="polyline"/> documentation for "M:Rhino.Input.Custom.GetPolyline.Get(Rhino.Geometry.Polyline@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetPolyline.Get(Rhino.Geometry.Polyline@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPolyline_Get.htm)

#### `public void SetFirstPoint(Point3d point)`

Use SetFirstPoint to specify the line's starting point and skip the start point interactive picking

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — [Missing <param name="point"/> documentation for "M:Rhino.Input.Custom.GetPolyline.SetFirstPoint(Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPolyline_SetFirstPoint.htm)

### Properties
- `FirstPointPrompt` (String, get/set) — Prompt when getting first point
- `FourthPointPrompt` (String, get/set) — Prompt when getting fourth point
- `MaxPointCount` (Int32, get/set) — 
- `MinPointCount` (Int32, get/set) — 
- `SecondPointPrompt` (String, get/set) — Prompt when getting second point
- `ThirdPointPrompt` (String, get/set) — Prompt when getting third point

## GetSphere (class)

Class provides user interface to define a sphere.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetSphere.htm)

### Constructors
- `public GetSphere()` — Initializes a new instance of the GetSphere class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetSphere_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetSphere_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetSphere_Finalize.htm)

#### `public Result Get(out Sphere sphere)`

Prompt for the getting of a sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — The sphere geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetSphere_Get.htm)

#### `public Result GetMesh(ref GetSphere.MeshSphereStyle style, ref int verticalFaces, ref int aroundFaces, ref int triangleSubdivisions, ref int quadSubdivisions, out Sphere sphere)`

Prompt for the getting of a mesh sphere.

**Parameters:**
- `style` (Rhino.Input.Custom.GetSphere.MeshSphereStyle) — The style of the mesh sphere.
- `verticalFaces` (System.Int32) — The number of UV mesh faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of UV mesh faces in the around direction.
- `triangleSubdivisions` (System.Int32) — The number of triangle mesh subdivisions.
- `quadSubdivisions` (System.Int32) — The number of quad mesh subdivisions.
- `sphere` (Rhino.Geometry.Sphere) — The sphere geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetSphere_GetMesh.htm)

### Properties
- `DefaultSize` (Double, get/set) — Default radius or diameter (based on InDiameterMode)
- `InDiameterMode` (Boolean, get/set) — Determines if the "size" value is representing a radius or diameter

## GetSphere.MeshSphereStyle (enum)

Mesh sphere styles enumeration.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetSphere_MeshSphereStyle.htm)

### Values
- `UV` = `0` — UV Sphere.
- `Triangle` = `1` — Icosahedron Sphere.
- `Quad` = `2` — Quad Sphere.

## GetString (class)

Used to get strings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetString.htm)

### Constructors
- `public GetString()` — Constructs a new GetString.

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public GetResult Get()`

Returns the string that the user typed. By default, space stops the string input.

**Returns:** `GetResult` — The result type. If the user typed a string, this is String.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetString_Get.htm)

#### `public GetResult GetLiteralString()`

Returns the string that the user typed. By default, space does not stop input.

**Returns:** `GetResult` — The result type. If the user typed a string, this is String.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetString_GetLiteralString.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

## GetTorus (class)

Class provides user interface to define a torus.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetTorus.htm)

### Constructors
- `public GetTorus()` — Initializes a new instance of the GetTorus class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTorus_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTorus_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTorus_Finalize.htm)

#### `public Result Get(out Torus torus)`

Prompt for the getting of a torus.

**Parameters:**
- `torus` (Rhino.Geometry.Torus) — The torus geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTorus_Get.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, out Torus torus)`

Prompt for the getting of a mesh torus.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `torus` (Rhino.Geometry.Torus) — The torus geometry defined by the user.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTorus_GetMesh.htm)

### Properties
- `AroundDirectionCount` (Int32, get/set) — The number of faces in the around direction.
- `AroundDirectionMinimumCount` (Int32, get/set) — The minumum number of faces in the around direction.
- `DefaultSize` (Double, get/set) — Default radius or diameter (based on InDiameterMode)
- `FixInnerRadius` (Boolean, get/set) — Second radius option. The first radius chosen sets the inner dimension of the torus and the second radius is constrained to be outside of the first radius.
- `InDiameterMode` (Boolean, get/set) — Determines if the "size" value is representing a radius or diameter
- `InSecondDiameterMode` (Boolean, get/set) — Second radius option. Determines if the second "size" value is representing a radius or diameter
- `PromptForMeshDensity` (Boolean, get/set) — Set true if you are prompting for a mesh or subd torus.
- `SecondSize` (Double, get/set) — Second radius or diameter (based on InSecondDiameterMode)
- `VerticalDirectionCount` (Int32, get/set) — The number of faces in the vertical direction.
- `VerticalDirectionMinimumCount` (Int32, get/set) — The minumum number of faces in the vertical direction.

## GetTransform (class)

Used for getting a Transform

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetTransform.htm)

### Constructors
- `protected GetTransform()` — Initializes a new instance of the GetTransform class

### Methods
#### `public void AcceptColor(bool enable)`

If you want to allow the user to be able to type in a color r,g,b or name during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true) before calling GetPoint()/GetObject(). If the user chooses to type in a color, then the result code GetResult.Color is returned and you can use RhinoGet.Color() to get the value of the color. If the get accepts points, then the user will not be able to type in r,g,b colors but will be able to type color names.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptColor.htm)

#### `public void AcceptCustomMessage(bool enable)`

(Inherited from GetBaseClass.)

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptCustomMessage(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptCustomMessage.htm)

#### `public void AcceptEnterWhenDone(bool enable)`

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing(). On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog can correctly enable the Done button.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AcceptEnterWhenDone(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptEnterWhenDone.htm)

#### `public void AcceptNothing(bool enable)`

If you want to allow the user to be able to press enter in order to skip selecting a something in GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNothing( true ) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to press enter in order to skip selecting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNothing.htm)

#### `public void AcceptNumber(bool enable, bool acceptZero)`

If you want to allow the user to be able to type in a number during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptNumber() beforehand. If the user chooses to type in a number, then the result code GetResult.Number is returned and you can use RhinoGet.Number() to get the value of the number. If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a number.
- `acceptZero` (System.Boolean) — If you are using GetPoint and you want "0" to return (0,0,0) instead of the number zero, then set acceptZero = false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptNumber.htm)

#### `public void AcceptPoint(bool enable)`

If you want to allow the user to be able to type in a point then call AcceptPoint(true) before calling GetPoint()/GetObject(). If the user chooses to type in a number, then the result code GetResult.Point is returned and you can use RhinoGet.Point() to get the value of the point.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type in a point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptPoint.htm)

#### `public void AcceptString(bool enable)`

If you want to allow the user to be able to type in a string during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptString(true) before calling GetPoint()/GetObject(). If the user chooses to type in a string, then the result code GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to type a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptString.htm)

#### `public void AcceptUndo(bool enable)`

If you want to allow the user to have an 'undo' option in GetPoint.Get(), GetObject.GetObjects(), etc., then call AcceptUndo(true) beforehand.

**Parameters:**
- `enable` (System.Boolean) — true if user is able to choose the 'Undo' option.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AcceptUndo.htm)

#### `public int AddConstructionPoint(Point3d point)`

Adds a point to the list of construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point to be added.

**Returns:** `Int32` — Total number of construction points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddConstructionPoint.htm)

#### `public int AddConstructionPoints(Point3d[] points)`

Adds points to the list of construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

**Parameters:**
- `points` (Rhino.Geometry.Point3d[]) — An array of points to be added.

**Returns:** `Int32` — Total number of construction points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddConstructionPoints.htm)

#### `public int AddOption(LocalizeStringPair optionName)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_1.htm)

#### `public int AddOption(LocalizeStringPair optionName, LocalizeStringPair optionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `optionValue` (Rhino.UI.LocalizeStringPair) — The localized value visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_2.htm)

#### `public int AddOption(string englishOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_3.htm)

#### `public int AddOption(string englishOption, string englishOptionValue)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_4.htm)

#### `public int AddOption(string englishOption, string englishOptionValue, bool hiddenOption)`

Adds a command line option.

**Parameters:**
- `englishOption` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes).
- `englishOptionValue` (System.String) — The option value in English, visualized after an equality sign.
- `hiddenOption` (System.Boolean) — If true, the option is not displayed on the command line and the full option name must be typed in order to activate the option.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOption_5.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor.htm)

#### `public int AddOptionColor(LocalizeStringPair optionName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — option description.
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — option prompt shown if the user selects this option

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_1.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_2.htm)

#### `public int AddOptionColor(string englishName, ref OptionColor colorValue, string prompt)`

Add a command line option to get colors and automatically save the value.

**Parameters:**
- `englishName` (System.String) — option description
- `colorValue` (Rhino.Input.Custom.OptionColor) — The current color value.
- `prompt` (System.String) — The command prompt will show this during picking.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionColor_3.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble.htm)

#### `public int AddOptionDouble(LocalizeStringPair optionName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically saves the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_1.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — The current number value.

**Returns:** `Int32` — Option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_2.htm)

#### `public int AddOptionDouble(string englishName, ref OptionDouble numberValue, string prompt)`

Adds a command line option to get numbers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `numberValue` (Rhino.Input.Custom.OptionDouble) — Current value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionDouble_3.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1.htm)

#### `public int AddOptionEnumList<T>(string englishOptionName, T defaultValue, T[] include) where T : struct, new(), IConvertible`

Adds a choice of enumerated values as list option. Allows to include only some enumerated values.

**Parameters:**
- `englishOptionName` (System.String) — The name of the option
- `defaultValue` (T) — The default value
- `include` (T[]) — An array of enumerated values to use. This argument can also be null; in this case, the whole enumerated is used.

**Returns:** `Int32` — Option index

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumList__1_1.htm)

#### `public int AddOptionEnumSelectionList<T>(string englishOptionName, IEnumerable<T> enumSelection, int listCurrentIndex) where T : struct, new(), IConvertible`

Adds a list of enumerated values as option list. Use enumSelection[go.Option.CurrentListOptionIndex] to retrieve selection.

**Parameters:**
- `englishOptionName` (System.String) — [Missing <param name="englishOptionName"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `enumSelection` (System.Collections.Generic.IEnumerable<T>) — [Missing <param name="enumSelection"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]
- `listCurrentIndex` (System.Int32) — [Missing <param name="listCurrentIndex"/> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.AddOptionEnumSelectionList``1(System.String,System.Collections.Generic.IEnumerable{``0},System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionEnumSelectionList__1.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger.htm)

#### `public int AddOptionInteger(LocalizeStringPair optionName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_1.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_2.htm)

#### `public int AddOptionInteger(string englishName, ref OptionInteger intValue, string prompt)`

Adds a command line option to get integers and automatically save the value.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `intValue` (Rhino.Input.Custom.OptionInteger) — The current integer value.
- `prompt` (System.String) — option prompt shown if the user selects this option. If null or empty, then the option name is used as the get number prompt.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionInteger_3.htm)

#### `public int AddOptionList(LocalizeStringPair optionName, IEnumerable<LocalizeStringPair> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<LocalizeStringPair>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList.htm)

#### `public int AddOptionList(string englishOptionName, IEnumerable<string> listValues, int listCurrentIndex)`

Adds a command line list option.

**Parameters:**
- `englishOptionName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `listValues` (System.Collections.Generic.IEnumerable<String>) — The string values.
- `listCurrentIndex` (System.Int32) — Zero based index of current option.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionList_1.htm)

#### `public int AddOptionToggle(LocalizeStringPair optionName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `optionName` (Rhino.UI.LocalizeStringPair) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle.htm)

#### `public int AddOptionToggle(string englishName, ref OptionToggle toggleValue)`

Adds a command line option to toggle a setting.

**Parameters:**
- `englishName` (System.String) — Must only consist of letters and numbers (no characters list periods, spaces, or dashes)
- `toggleValue` (Rhino.Input.Custom.OptionToggle) — The current toggle value.

**Returns:** `Int32` — option index value (>0) or 0 if option cannot be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_AddOptionToggle_1.htm)

#### `public int AddSnapPoint(Point3d point)`

Adds a point to the list of osnap points.

**Remarks:** When point osnap is enabled, GetPoint will snap to points in the Rhino model. If you want the user to be able to snap to additional points, then use GetPoint::AddSnapPoints to specify the locations of these additional points.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point.

**Returns:** `Int32` — Total number of snap points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddSnapPoint.htm)

#### `public int AddSnapPoints(Point3d[] points)`

Adds points to the list of osnap points.

**Remarks:** When point osnap is enabled, GetPoint will snap to points in the Rhino model. If you want the user to be able to snap to additional points, then use GetPoint::AddSnapPoints to specify the locations of these additional points.

**Parameters:**
- `points` (Rhino.Geometry.Point3d[]) — An array of points to snap onto.

**Returns:** `Int32` — Total number of snap points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_AddSnapPoints.htm)

#### `public void AddTransformObjects(TransformObjectList list)`

Adds any objects you want transformed and grips you want transformed. Make sure no duplicates are in the list and that no grip owners are passed in as objects.

**Parameters:**
- `list` (Rhino.Collections.TransformObjectList) — A custom transform object list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTransform_AddTransformObjects.htm)

#### `public abstract Transform CalculateTransform(RhinoViewport viewport, Point3d point)`

Retrieves the final transformation. Override this virtual function to provide your own custom transformation method.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — A Rhino viewport that the user is using.
- `point` (Rhino.Geometry.Point3d) — A point that the user is selecting.

**Returns:** `Transform` — A transformation matrix value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTransform_CalculateTransform.htm)

#### `public void ClearCommandOptions()`

Clear all command options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearCommandOptions.htm)

#### `public void ClearConstraints()`

Removes any explicit constraints added by calls to GetPoint::Constraint() and enable the built-in constraint options.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ClearConstraints.htm)

#### `public void ClearConstructionPoints()`

Remove all construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ClearConstructionPoints.htm)

#### `public void ClearDefault()`

Clears any defaults set using SetDefaultPoint, SetDefaultNumber, SetDefaultString, or SetCommandPromptDefault.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_ClearDefault.htm)

#### `public void ClearSnapPoints()`

Remove all snap points.

**Remarks:** When point osnap is enabled, GetPoint will snap to points in the Rhino model. If you want the user to be able to snap to additional points, then use GetPoint::AddSnapPoints to specify the locations of these additional points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ClearSnapPoints.htm)

#### `public Color Color()`

Gets a color if Get*() returns GetResult.Color.

**Returns:** `Color` — The color chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Color.htm)

#### `public Result CommandResult()`

Helper method for getting command result value from getter results.

**Returns:** `Result` — The converted command result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CommandResult.htm)

#### `public bool Constrain(Arc arc)`

Constrains the picked point to lie on an arc.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — An arc to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain.htm)

#### `public bool Constrain(Brep brep, int wireDensity, int faceIndex, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a brep.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A brep to use as constraint.
- `wireDensity` (System.Int32) — When wire_density<0, isocurve intersection snapping is turned off, when wire_density>=0, the value defines the isocurve density used for isocurve intersection snapping.
- `faceIndex` (System.Int32) — When face_index <0, constrain to whole brep. When face_index >=0, constrain to individual face.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_1.htm)

#### `public bool Constrain(Circle circle)`

Constrains the picked point to lie on a circle.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — A circle to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_2.htm)

#### `public bool Constrain(Curve curve, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a curve.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A curve to use as constraint.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_3.htm)

#### `public bool Constrain(Cylinder cylinder)`

Constrains the picked point to lie on a cylinder.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — A cylinder to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_4.htm)

#### `public bool Constrain(Line line)`

Constrains the picked point to lie on a line.

**Parameters:**
- `line` (Rhino.Geometry.Line) — A line to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_5.htm)

#### `public bool Constrain(Mesh mesh, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to use as constraint.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_6.htm)

#### `public bool Constrain(Plane plane, bool allowElevator)`

constrain the picked point to lie on a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane to use as constraint.
- `allowElevator` (System.Boolean) — true if elevator mode should be allowed at user request.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_7.htm)

#### `public bool Constrain(Point3d from, Point3d to)`

Constrains the picked point to lie on a line.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — The start point of constraint.
- `to` (Rhino.Geometry.Point3d) — The end point of constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_8.htm)

#### `public bool Constrain(Sphere sphere)`

Constrains the picked point to lie on a sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — A sphere to use as constraint.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_9.htm)

#### `public bool Constrain(Surface surface, bool allowPickingPointOffObject)`

Constrains the picked point to lie on a surface.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — A surface to use as constraint.
- `allowPickingPointOffObject` (System.Boolean) — defines whether the point pick is allowed to happen off object. When false, a "no no" cursor is shown when the cursor is not on the object. When true, a normal point picking cursor is used and the marker is visible also when the cursor is not on the object.

**Returns:** `Boolean` — true if constraint could be applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Constrain_10.htm)

#### `public void ConstrainDistanceFromBasePoint(double distance)`

Sets distance constraint from base point.

**Remarks:** If the base point is set and the distance from base point constraint is > 0, then the picked point is constrained to be this distance from the base point.

**Parameters:**
- `distance` (System.Double) — pass UnsetValue to clear this constraint. Pass 0.0 to disable the ability to set this constraint by typing a number during GetPoint.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainDistanceFromBasePoint.htm)

#### `public bool ConstrainToConstructionPlane(bool throughBasePoint)`

If enabled, the picked point is constrained to be on the active construction plane. If the base point is set, then the point is constrained to be on the plane that contains the base point and is parallel to the active construction plane. By default this constraint is enabled.

**Parameters:**
- `throughBasePoint` (System.Boolean) — true if the base point should be used as compulsory level reference.

**Returns:** `Boolean` — If true and the base point is set, then the point is constrained to be on the plane parallel to the construction plane that passes through the base point, even when planar mode is off. If throughBasePoint is false, then the base point shift only happens if planar mode is on.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainToConstructionPlane.htm)

#### `public void ConstrainToTargetPlane()`

Constrains point to lie on a plane that is parallel to the viewing plane and passes through the view's target point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainToTargetPlane.htm)

#### `public bool ConstrainToVirtualCPlaneIntersection(Plane plane)`

If enabled, the picked point is constrained to be on the intersection of the plane and the virtual CPlane going through the plane origin. If the planes are parallel, the constraint works just like planar constraint.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The plane used for the plane - virtual CPlane intersection.

**Returns:** `Boolean` — true if the operation succeeded; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_ConstrainToVirtualCPlaneIntersection.htm)

#### `public Object CustomMessage()`

(Inherited from GetBaseClass.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.CustomMessage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_CustomMessage.htm)

#### `public void Dispose()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

(Inherited from GetBaseClass.)

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Dispose_1.htm)

#### `public void DrawLineFromPoint(Point3d startPoint, bool showDistanceInStatusBar)`

Use DrawLineFromPoint() if you want a dynamic line drawn from a point to the point being picked.

**Remarks:** Calling DrawLineFromPoint automatically enables drawing the line. Use EnableDrawLineFromPoint() to toggle the line drawing state.

**Parameters:**
- `startPoint` (Rhino.Geometry.Point3d) — The line is drawn from startPoint to the point being picked. If the base point has not been set, then it is set to startPoint.
- `showDistanceInStatusBar` (System.Boolean) — if true, the distance from the basePoint to the point begin picked is shown in the status bar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_DrawLineFromPoint.htm)

#### `public void EnableCurveSnapArrow(bool drawDirectionArrowAtSnapPoint, bool reverseArrow)`

Controls display of the curve snap arrow icon.

**Remarks:** The tangent bar is drawn by GetPoint::DynamicDraw. If you override GetPoint::DynamicDraw, then you must call the base class function.

**Parameters:**
- `drawDirectionArrowAtSnapPoint` (System.Boolean) — true to draw arrow icon whenever GetPoint snaps to a curve.
- `reverseArrow` (System.Boolean) — true if arrow icon direction should be the reverse of the first derivative direction.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableCurveSnapArrow.htm)

#### `public void EnableCurveSnapPerpBar(bool drawPerpBarAtSnapPoint, bool drawEndPoints)`

Controls display of the curve snap perpendicular bar icon.

**Parameters:**
- `drawPerpBarAtSnapPoint` (System.Boolean) — true to draw a tangent bar icon whenever GetPoint snaps to a curve.
- `drawEndPoints` (System.Boolean) — true to draw points at the end of the tangent bar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableCurveSnapPerpBar.htm)

#### `public void EnableCurveSnapTangentBar(bool drawTangentBarAtSnapPoint, bool drawEndPoints)`

Controls display of the curve snap tangent bar icon.

**Remarks:** The tangent bar is drawn by GetPoint::DynamicDraw. If you override GetPoint::DynamicDraw, then you must call the base class function.

**Parameters:**
- `drawTangentBarAtSnapPoint` (System.Boolean) — true to draw a tangent bar icon whenever GetPoint snaps to a curve.
- `drawEndPoints` (System.Boolean) — true to draw points at the end of the tangent bar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableCurveSnapTangentBar.htm)

#### `public void EnableDrawLineFromPoint(bool enable)`

Controls drawing of dynamic a line from the start point.

**Parameters:**
- `enable` (System.Boolean) — if true, a dynamic line is drawn from the DrawLineFromPoint startPoint to the point being picked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableDrawLineFromPoint.htm)

#### `public void EnableNoRedrawOnExit(bool noRedraw)`

The default functionality of the GetPoint operation is to perform a redraw on exit. Calling this function with true turns off automatic redraw at the end of GetPoint. May be needed in some commands for flicker free feedback. When set to true, the caller is responsible for cleaning up the screen after GetPoint.

**Parameters:**
- `noRedraw` (System.Boolean) — [Missing <param name="noRedraw"/> documentation for "M:Rhino.Input.Custom.GetPoint.EnableNoRedrawOnExit(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableNoRedrawOnExit.htm)

#### `public void EnableObjectSnapCursors(bool enable)`

Enables or disables object snap cursors. By default, object snap cursors are enabled.

**Parameters:**
- `enable` (System.Boolean) — If true then object snap cursors (plus sign with "near", "end", etc.) are used when the point snaps to a object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableObjectSnapCursors.htm)

#### `public void EnableSnapToCurves(bool enable)`

If you want GetPoint() to try to snap to curves when the mouse is near a curve (like the center point in the Circle command when the AroundCurve option is on), then enable the snap to curves option.

**Parameters:**
- `enable` (System.Boolean) — Whether points should be enabled.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_EnableSnapToCurves.htm)

#### `public void EnableTransparentCommands(bool enable)`

Control the availability of transparent commands during the get.

**Remarks:** Some Rhino commands are "transparent" and can be run inside of other commands. Examples of transparent commands include the view manipulation commands like ZoomExtents, Top, etc., and the selection commands like SelAll, SelPoint, etc. By default transparent commands can be run during any get. If you want to disable this feature, then call EnableTransparentCommands(false) before calling GetString, GetPoint, GetObject, etc.

**Parameters:**
- `enable` (System.Boolean) — If true, then transparent commands can be run during the get. If false, then transparent commands cannot be run during the get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_EnableTransparentCommands.htm)

#### `protected override void Finalize()`

(Inherited from GetBaseClass.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Finalize.htm)

#### `public GetResult Get()`

After setting up options and so on, call GetPoint::Get to get a 3d point. The point is retrieved when the mouse goes down.

**Returns:** `GetResult` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetPoint.Get"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Get.htm)

#### `public GetResult Get(bool onMouseUp)`

After setting up options and so on, call this method to get a 3d point.

**Parameters:**
- `onMouseUp` (System.Boolean) — If false, the point is returned when the left mouse button goes down. If true, the point is returned when the left mouse button goes up.

**Returns:** `GetResult` — Point if the user chose a point; other enumeration value otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Get_1.htm)

#### `public GetResult Get(bool onMouseUp, bool get2DPoint)`

After setting up options and so on, call this method to get a 2d or 3d point.

**Parameters:**
- `onMouseUp` (System.Boolean) — If false, the point is returned when the left mouse button goes down. If true, the point is returned when the left mouse button goes up.
- `get2DPoint` (System.Boolean) — If true then get a 2d point otherwise get a 2d point

**Returns:** `GetResult` — Point if the user chose a 3d point; Point2d if the user chose a 2d point; other enumeration value otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_Get_2.htm)

#### `public Point3d[] GetConstructionPoints()`

Gets current construction points.

**Remarks:** Construction points are like snap points except that they get snapped to even when point osnap is off. Typically, there are only a few construction points while there can be many snap points. For example, when polylines are drawn the start point is a construction point and the other points are snap points.

**Returns:** `Point3d[]` — An array of points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_GetConstructionPoints.htm)

#### `public bool GetPlanarConstraint(ref RhinoViewport vp, out Plane plane)`

(Inherited from GetPoint.)

**Parameters:**
- `vp` (Rhino.Display.RhinoViewport) — [Missing <param name="vp"/> documentation for "M:Rhino.Input.Custom.GetPoint.GetPlanarConstraint(Rhino.Display.RhinoViewport@,Rhino.Geometry.Plane@)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.Input.Custom.GetPoint.GetPlanarConstraint(Rhino.Display.RhinoViewport@,Rhino.Geometry.Plane@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetPoint.GetPlanarConstraint(Rhino.Display.RhinoViewport@,Rhino.Geometry.Plane@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_GetPlanarConstraint.htm)

#### `public T GetSelectedEnumValue<T>() where T : struct, new(), IConvertible`

Returns the selected enumerated value. Use this in combination with AddOptionEnumList<T>(String, T). This must be called directly after having called a Get method, and having obtained a Option value.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValue__1.htm)

#### `public T GetSelectedEnumValueFromSelectionList<T>(IEnumerable<T> selectionList) where T : struct, new(), IConvertible`

Returns the selected enumerated value by looking at the list of values from which to select. Use this in combination with AddOptionEnumSelectionList<T>(String, IEnumerable<T>, Int32)

**Parameters:**
- `selectionList` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.GetSelectedEnumValueFromSelectionList``1(System.Collections.Generic.IEnumerable{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GetSelectedEnumValueFromSelectionList__1.htm)

#### `public Point3d[] GetSnapPoints()`

Gets current snap points.

**Returns:** `Point3d[]` — An array of points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_GetSnapPoints.htm)

#### `public GetResult GetXform()`

Gets the Transformation. Call this after having set up options and so on.

**Returns:** `GetResult` — The result based on user choice.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTransform_GetXform.htm)

#### `public bool GotDefault()`

Returns true if user pressed Enter to accept a default point, number, or string set using SetDefaultPoint, SetDefaultNumber, or SetDefaultString.

**Returns:** `Boolean` — true if the result if the default point, number or string set. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_GotDefault.htm)

#### `public bool InterruptMouseMove()`

If you have lengthy computations in OnMouseMove() and/or DymanicDraw() overrides, then periodically call InterruptMouseMove() to see if you should interrupt your work because the mouse has moved again.

**Returns:** `Boolean` — true if you should interrupt your work; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_InterruptMouseMove.htm)

#### `public Point[] Line2d()`

Returns two points defining the location in the view window of the 2d line selected in GetPoint::Get2dLine(). (0,0) = upper left corner of window.

**Returns:** `Point[]` — An array with two 2D points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Line2d.htm)

#### `public double Number()`

Gets a number if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.Number.

**Returns:** `Double` — The number chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Number.htm)

#### `protected virtual void OnDynamicDraw(GetPointDrawEventArgs e)`

Default calls the DynamicDraw event.

**Parameters:**
- `e` (Rhino.Input.Custom.GetPointDrawEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnDynamicDraw.htm)

#### `protected virtual void OnMouseDown(GetPointMouseEventArgs e)`

Default calls the MouseDown event.

**Parameters:**
- `e` (Rhino.Input.Custom.GetPointMouseEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnMouseDown.htm)

#### `protected virtual void OnMouseMove(GetPointMouseEventArgs e)`

Calls the MouseMove event and can/should be called by overriding implementation.

**Parameters:**
- `e` (Rhino.Input.Custom.GetPointMouseEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnMouseMove.htm)

#### `protected virtual void OnPostDrawObjects(DrawEventArgs e)`

In the "rare" case that you need to draw some depth buffered geometry during a GetPoint operation, override the OnPostDrawObjects function. NOTE!! Overriding this function comes with a significant performance penalty because the scene needs to be fully regenerated every frame where the standard DynamicDraw event draws temporary decorations (geometry) on top of a static scene.

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — Current argument for the event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_OnPostDrawObjects.htm)

#### `public CommandLineOption Option()`

(Inherited from GetBaseClass.)

**Returns:** `CommandLineOption` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.Option"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Option.htm)

#### `public int OptionIndex()`

(Inherited from GetBaseClass.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetBaseClass.OptionIndex"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_OptionIndex.htm)

#### `public void PermitConstraintOptions(bool permit)`

Control the availability of the built-in linear, planar, curve, and surface constraint options like "Along", "AlongPerp", "AlongTan", "AlongParallel", "Between", "OnCrv", "OnSrf", ".x", ".y", ".z", ".xy", etc.

**Remarks:** By default, these built-in constraint options are available unless an explicit constraint is added by calling one of the GetPoint::Constrain functions. Calling GetPoint::ClearConstraints automatically enables the built-in constraint options. The built-in constraint options are never visible on the command line and the user must type the complete option name to activate these options.

**Parameters:**
- `permit` (System.Boolean) — if true, then the built-in constraint options are automatically available in GetPoint.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitConstraintOptions.htm)

#### `public void PermitElevatorMode(int permitMode)`

Permits the use of the control key to define a line constraint.

**Parameters:**
- `permitMode` (System.Int32) — 0: no elevator modes are permitted 1: fixed plane elevator mode (like the Line command) 2: cplane elevator mode (like object dragging)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitElevatorMode.htm)

#### `public void PermitFromOption(bool permit)`

Control the availability of the built-in "From" option. By default, the "From" option is enabled.

**Remarks:** The GetPoint "From" option is never visible on the command line and the user must type the complete option name to activate the "From" option. When the GetPoint "From" snap is enabled, the user set/change the base point during GetPoint by typing "From" and picking a point. A related option is the built-in distance from base point constraint that is can be set before GetPoint is called by passing a value to GetPoint::ConstrainDistanceFromBasePoint or during GetPoint by entering a number.

**Parameters:**
- `permit` (System.Boolean) — if true, then the "From" option is automatically available in GetPoint.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitFromOption.htm)

#### `public void PermitObjectSnap(bool permit)`

By default, object snaps like "end", "near", etc. are controlled by the user. If you want to disable this ability, then call PermitObjectSnap(false).

**Parameters:**
- `permit` (System.Boolean) — true to permit snapping to objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitObjectSnap.htm)

#### `public void PermitOrthoSnap(bool permit)`

Controls availability of ortho snap. Default is true.

**Parameters:**
- `permit` (System.Boolean) — if true, then GetPoint pays attention to the Rhino "ortho snap" and "planar snap" settings reported by ModelAidSettings.Ortho and ModelAidSettings.Planar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitOrthoSnap.htm)

#### `public void PermitTabMode(bool permit)`

Permits the use of the tab key to define a line constraint.

**Remarks:** By default, use of the tab key is supported.

**Parameters:**
- `permit` (System.Boolean) — If true, then the built-in tab key mode is available.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PermitTabMode.htm)

#### `public Rectangle PickRectangle()`

If the get was a GetObjects() and the mouse was used to select the objects, then the returned rectangle has left < right and top < bottom. This rectangle is the Windows GDI screen coordinates of the picking rectangle. RhinoViewport.GetPickXform( pick_rect, pick_xform ) will calculate the picking transformation that was used. In all other cases, left=right=top=bottom=0;

**Returns:** `Rectangle` — The picking rectangle; or 0 in the specified cases.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_PickRectangle.htm)

#### `public Point3d Point()`

Gets a point if Get*() returns GetResult.Point.

**Returns:** `Point3d` — The point chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point.htm)

#### `public Point Point2d()`

Returns location in view of point in selected in GetPoint::Get() or GetPoint::Get2dPoint(). (0,0) = upper left corner of window.

**Returns:** `Point` — The location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Point2d.htm)

#### `public BrepFace PointOnBrep(out double u, out double v)`

Use to determine if point was on a Brep face. If the point was on a Brep face, then the (u,v) are the face parameters for the point.

**Parameters:**
- `u` (System.Double) — If the point was on a Brep face, then the u parameter.
- `v` (System.Double) — If the point was on a Brep face, then the v parameter.

**Returns:** `BrepFace` — The Brep face or null if the point was not on a Brep face.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnBrep.htm)

#### `public Curve PointOnCurve(out double t)`

Use to determine is point was on a curve.

**Parameters:**
- `t` (System.Double) — If the point was on a curve, then the t is the curve parameter for the point. The point returned by Point() is the same as curve.PointAt(t).

**Returns:** `Curve` — A curve at a specified parameter value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnCurve.htm)

#### `public ObjRef PointOnObject()`

Call this function to see if the point was on an object. If the point was on an object an ObjRef is returned; otherwise null is returned.

**Returns:** `ObjRef` — A point object reference.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnObject.htm)

#### `public Surface PointOnSurface(out double u, out double v)`

Use to determine if point was on a surface. If the point was on a surface, then the (u,v) are the surface parameters for the point. The point returned by Point() is the same as surface.PointAt(u,v).

**Parameters:**
- `u` (System.Double) — If the point was on a surface, then the u parameter.
- `v` (System.Double) — If the point was on a surface, then the v parameter.

**Returns:** `Surface` — The surface or null if the point was not on a surface.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_PointOnSurface.htm)

#### `public Rectangle Rectangle2d()`

Returns the location in the view of the 2d rectangle selected in GetPoint::Get2dRectangle(). rect.left < rect.right and rect.top < rect.bottom (0,0) = upper left corner of window.

**Returns:** `Rectangle` — The rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Rectangle2d.htm)

#### `public GetResult Result()`

Returns result of the Get*() call.

**Returns:** `GetResult` — The result of the last Get*() call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Result.htm)

#### `public void SetBasePoint(Point3d basePoint, bool showDistanceInStatusBar)`

Sets a base point used by ortho snap, from snap, planar snap, etc.

**Parameters:**
- `basePoint` (Rhino.Geometry.Point3d) — The new base point.
- `showDistanceInStatusBar` (System.Boolean) — If true, then the distance from base_point to the current point will be in the status bar distance pane.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_SetBasePoint.htm)

#### `public void SetCommandPrompt(string prompt)`

Sets prompt message that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — command prompt message.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPrompt.htm)

#### `public void SetCommandPromptDefault(string defaultValue)`

Sets message that describes what default value will be used if the user presses enter. This description appears in angle brackets <> in the command prompt window. You do not need to provide a default value description unless you explicitly enable AcceptNothing.

**Remarks:** If you have a simple default point, number, or string, it is easier to use SetDefaultPoint, SetDefaultNumber, or SetDefaultString. SetCommandPromptDefault and AcceptNothing can be used for providing more advanced UI behavior.

**Parameters:**
- `defaultValue` (System.String) — description of default value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetCommandPromptDefault.htm)

#### `public void SetCursor(CursorStyle cursor)`

Sets cursor that will be used when Get() is called and snap is not happening.

**Parameters:**
- `cursor` (Rhino.UI.CursorStyle) — [Missing <param name="cursor"/> documentation for "M:Rhino.Input.Custom.GetPoint.SetCursor(Rhino.UI.CursorStyle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_SetCursor.htm)

#### `public void SetDefaultColor(Color defaultColor)`

Sets a color as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultColor will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default color, GetResult.Color is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultColor will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultColor` (System.Drawing.Color) — value for default color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultColor.htm)

#### `public void SetDefaultInteger(int defaultValue)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultInteger will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default integer, GetResult.Number is returned and CRhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.Int32) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultInteger.htm)

#### `public void SetDefaultNumber(double defaultNumber)`

Sets a number as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultNumber will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default number, GetResult.Number is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultNumber will clear any previous calls to SetDefaultString or SetDefaultPoint.

**Parameters:**
- `defaultNumber` (System.Double) — value for default number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultNumber.htm)

#### `public void SetDefaultPoint(Point3d point)`

Sets a point as default value that will be returned if the user presses the ENTER key during the get.

**Remarks:** Calling SetDefaultPoint will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses enter to accept the default point, GetResult.Point is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultPoint will clear any previous calls to SetDefaultString or SetDefaultNumber.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — value for default point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultPoint.htm)

#### `public void SetDefaultString(string defaultValue)`

Sets a string as default value that will be returned if the user presses ENTER key during the get.

**Remarks:** Calling SetDefaultString will automatically handle setting the command prompt default and reacting to the user pressing ENTER. If the user presses ENTER to accept the default string, GetResult.String is returned and RhinoGet.GotDefault() will return true. Calling SetDefaultString will clear any previous calls to SetDefaultNumber or SetDefaultPoint.

**Parameters:**
- `defaultValue` (System.String) — value for default string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetDefaultString.htm)

#### `public void SetWaitDuration(int milliseconds)`

Sets the wait duration (in milliseconds) of the getter. If the duration passes without the user making a decision, the GetResult.Timeout code is returned.

**Parameters:**
- `milliseconds` (System.Int32) — Number of milliseconds to wait.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_SetWaitDuration.htm)

#### `public string StringResult()`

Gets a string if GetPoint.Get(), GetObject.GetObjects(), etc., returns GetResult.String.

**Returns:** `String` — The string chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_StringResult.htm)

#### `public bool TryGetBasePoint(out Point3d basePoint)`

(Inherited from GetPoint.)

**Parameters:**
- `basePoint` (Rhino.Geometry.Point3d) — [Missing <param name="basePoint"/> documentation for "M:Rhino.Input.Custom.GetPoint.TryGetBasePoint(Rhino.Geometry.Point3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.GetPoint.TryGetBasePoint(Rhino.Geometry.Point3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetPoint_TryGetBasePoint.htm)

#### `public Vector3d Vector()`

Gets a direction if Get*() returns GetResult.Point (Set by some digitizers, but in general it's (0,0,0).

**Returns:** `Vector3d` — The vector chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_Vector.htm)

#### `public RhinoView View()`

Gets a view the user clicked in during GetPoint.Get(), GetObject.GetObjects(), etc.

**Returns:** `RhinoView` — The view chosen by the user.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetBaseClass_View.htm)

### Properties
- `DynamicDrawColor` (Color, get/set) — Color used by CRhinoGetPoint::DynamicDraw to draw the current point and the line from the base point to the current point.
- `FullFrameRedrawDuringGet` (Boolean, get/set) — In the "RARE" case that you need to draw some depth buffered geometry during a Get() operation, setting this value to true will force entire frames to be redrawn while the user moves the mouse. This allows DisplayPipeline events to be triggered as well as OnPostDrawObjects NOTE!! Setting this value to true comes with a significant performance penalty because the scene needs to be fully regenerated every frame where the standard DynamicDraw event draws temporary decorations (geometry) on top of a static scene.
- `HaveTransform` (Boolean, get/set) — 
- `ObjectList` (TransformObjectList, get) — 
- `OsnapEventType` (OsnapModes, get) — Gets the type of object snap used to obtain the point.
- `Tag` (Object, get/set) — Gets or sets an arbitrary object that can be attached to this GetPoint instance. Useful for passing some/ information that you may need in a DynamicDraw event since you can get at this Tag from the GetPointDrawEventArgs.
- `Transform` (Transform, get/set) — 

### Events
#### `DynamicDraw` (`System.EventHandler<GetPointDrawEventArgs>`)

**Signature:** `public event EventHandler<GetPointDrawEventArgs> DynamicDraw`

Event to use if you want to dynamically draw things as the mouse/digitizer moves. Every time the mouse moves, DynamicDraw will be called once per viewport. The calls to DynamicDraw happen AFTER the call to MouseMove. If you are drawing anything that takes a long time, periodically call InterruptMouseMove() to see if you should stop.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_DynamicDraw.htm)

#### `MouseDown` (`System.EventHandler<GetPointMouseEventArgs>`)

**Signature:** `public event EventHandler<GetPointMouseEventArgs> MouseDown`

Called during Get2dRectangle, Get2dLine, and GetPoint(..,true) when the mouse down event for the initial point occurs. This function is not called during ordinary point getting because the mouse down event terminates an ordinary point get and returns a GetResult.Point result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_MouseDown.htm)

#### `MouseMove` (`System.EventHandler<GetPointMouseEventArgs>`)

**Signature:** `public event EventHandler<GetPointMouseEventArgs> MouseMove`

Called every time the mouse moves. MouseMove is called once per mouse move and is called BEFORE any calls to OnDynamicDraw. If you are doing anything that takes a long time, periodically call InterruptMouseMove() to see if you should stop. If the view is such that the 2d screen point can't be mapped to a 3d point, the 'point' argument will be Unset.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_MouseMove.htm)

#### `PostDrawObjects` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public event EventHandler<DrawEventArgs> PostDrawObjects`

Same as the DisplayPipeline.PostDrawObjects, but only works during the operation of the Get() function. NOTE: You must set FullFrameRedrawDuringGet to true in order for this event to be called.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Input_Custom_GetPoint_PostDrawObjects.htm)

## GetTruncatedCone (class)

Class provides user interface to define a truncated cone.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_GetTruncatedCone.htm)

### Constructors
- `public GetTruncatedCone()` — Initializes a new instance of the GetTruncatedCone class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTruncatedCone_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTruncatedCone_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTruncatedCone_Finalize.htm)

#### `public Result Get(out Brep truncatedCone)`

Prompt for the getting of a truncated cone.

**Parameters:**
- `truncatedCone` (Rhino.Geometry.Brep) — The truncated cone in Brep form.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTruncatedCone_Get.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, ref int capStyle, out Mesh truncatedCone)`

Prompt for the getting of a mesh truncated cone.

**Remarks:** The prompt for capStyle will only be seen if it's not zero, aroundFaces is even and the solid option is on.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `capStyle` (System.Int32) — Set to 0 if you don't want the prompt, 3 is triangles, 4 is quads.
- `truncatedCone` (Rhino.Geometry.Mesh) — The truncated cone in Mesh form.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTruncatedCone_GetMesh_1.htm)

#### `public Result GetMesh(ref int verticalFaces, ref int aroundFaces, out Mesh truncatedCone)`

Prompt for the getting of a mesh truncated cone.

**Parameters:**
- `verticalFaces` (System.Int32) — The number of faces in the vertical direction.
- `aroundFaces` (System.Int32) — The number of faces in the around direction
- `truncatedCone` (Rhino.Geometry.Mesh) — The truncated cone in Mesh form.

**Returns:** `Result` — The result of the getting operation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_GetTruncatedCone_GetMesh.htm)

### Properties
- `Cap` (Boolean, get/set) — Gets or sets whether or not the output should be capped.
- `CylinderConstraint` (CylinderConstraint, get/set) — State of the cone/cylinder constraint option. When the cone/cylinder option is selected, the circle is being made as a base for a cone/cylinder. By default the vertical cone/cylinder option not available but is not selected. By default the "Vertical" option applies to VerticalCircle.
- `DefaultSize` (Double, get/set) — Default radius or diameter (based on InDiameterMode)
- `Height` (Double, get/set) — Height of truncated cone.
- `InDiameterMode` (Boolean, get/set) — Determines if the "size" value is representing a radius or diameter
- `SecondRadius` (Double, get/set) — Radius of second circle.

## OptionColor (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.OptionColor"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_OptionColor.htm)

### Constructors
- `public OptionColor(Color initialValue)` — Initializes a new instance of the OptionColor class

### Methods
#### `public void Dispose()`

Releases all resources used by the OptionColor

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionColor_Dispose.htm)

#### `protected void Dispose(bool disposing)`

Releases the unmanaged resources used by the OptionColor and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionColor_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionColor_Finalize.htm)

### Properties
- `CurrentValue` (Color, get/set) — 
- `InitialValue` (Color, get) — 

## OptionDouble (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.OptionDouble"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_OptionDouble.htm)

### Constructors
- `public OptionDouble(double initialValue)` — Initializes a new instance of the OptionDouble class
- `public OptionDouble(double initialValue, bool setLowerLimit, double limit)` — Initializes a new instance of the double option class.
- `public OptionDouble(double initialValue, double lowerLimit, double upperLimit)` — Initializes a new instance of the OptionDouble class with lower and upper limits.

### Methods
#### `public void Dispose()`

Releases all resources used by the OptionDouble

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionDouble_Dispose.htm)

#### `protected void Dispose(bool disposing)`

Releases the unmanaged resources used by the OptionDouble and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionDouble_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionDouble_Finalize.htm)

### Properties
- `CurrentValue` (Double, get/set) — 
- `InitialValue` (Double, get) — 

## OptionInteger (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.OptionInteger"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_OptionInteger.htm)

### Constructors
- `public OptionInteger(int initialValue)` — Initializes a new instance of the OptionInteger class
- `public OptionInteger(int initialValue, bool setLowerLimit, int limit)` — Initializes a new instance of the OptionInteger class.
- `public OptionInteger(int initialValue, int lowerLimit, int upperLimit)` — Initializes a new instance of the OptionInteger class with both lower and upper limits.

### Methods
#### `public void Dispose()`

Releases all resources used by the OptionInteger

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionInteger_Dispose.htm)

#### `protected void Dispose(bool disposing)`

Releases the unmanaged resources used by the OptionInteger and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionInteger_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionInteger_Finalize.htm)

### Properties
- `CurrentValue` (Int32, get/set) — 
- `InitialValue` (Int32, get) — 

## OptionToggle (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.OptionToggle"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_OptionToggle.htm)

### Constructors
- `public OptionToggle(bool initialValue, LocalizeStringPair offValue, LocalizeStringPair onValue)` — Initializes a new instance of the OptionToggle class
- `public OptionToggle(bool initialValue, string offValue, string onValue)` — Initializes a new instance of the OptionToggle class

### Methods
#### `public void Dispose()`

Releases all resources used by the OptionToggle

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionToggle_Dispose.htm)

#### `protected void Dispose(bool disposing)`

Releases the unmanaged resources used by the OptionToggle and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionToggle_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_OptionToggle_Finalize.htm)

### Properties
- `CurrentValue` (Boolean, get/set) — 
- `InitialValue` (Boolean, get) — 

## PickContext (class)

Utility for determining if objects are picked

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_PickContext.htm)

### Constructors
- `public PickContext()` — Initializes a new instance of the PickContext class

### Methods
#### `public void Dispose()`

Releases all resources used by the PickContext

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the PickContext and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_Finalize.htm)

#### `public bool PickFrustumTest(BezierCurve bezier, out double t, out double depth, out double distance)`



**Parameters:**
- `bezier` (Rhino.Geometry.BezierCurve) — [Missing <param name="bezier"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.BezierCurve,System.Double@,System.Double@,System.Double@)"]
- `t` (System.Double) — [Missing <param name="t"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.BezierCurve,System.Double@,System.Double@,System.Double@)"]
- `depth` (System.Double) — [Missing <param name="depth"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.BezierCurve,System.Double@,System.Double@,System.Double@)"]
- `distance` (System.Double) — [Missing <param name="distance"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.BezierCurve,System.Double@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.BezierCurve,System.Double@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest.htm)

#### `public bool PickFrustumTest(BoundingBox box, out bool boxCompletelyInFrustum)`

Fast test to check if a bounding box intersects a pick frustum.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — [Missing <param name="box"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.BoundingBox,System.Boolean@)"]
- `boxCompletelyInFrustum` (System.Boolean) — Set to true if the box is completely contained in the pick frustum. When doing a window or crossing pick, you can immediately return a hit if the object's bounding box is completely inside of the pick frustum.

**Returns:** `Boolean` — False if bounding box is invalid or box does not intersect the pick frustum

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_1.htm)

#### `public bool PickFrustumTest(Line line, out double t, out double depth, out double distance)`



**Parameters:**
- `line` (Rhino.Geometry.Line) — [Missing <param name="line"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Line,System.Double@,System.Double@,System.Double@)"]
- `t` (System.Double) — [Missing <param name="t"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Line,System.Double@,System.Double@,System.Double@)"]
- `depth` (System.Double) — [Missing <param name="depth"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Line,System.Double@,System.Double@,System.Double@)"]
- `distance` (System.Double) — [Missing <param name="distance"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Line,System.Double@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Line,System.Double@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_2.htm)

#### `public bool PickFrustumTest(Mesh mesh, PickContext.MeshPickStyle pickStyle, out Point3d hitPoint, out double depth, out double distance, out PickContext.MeshHitFlag hitFlag, out int hitIndex)`

Utility for picking meshes

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — mesh to test
- `pickStyle` (Rhino.Input.Custom.PickContext.MeshPickStyle) — mode used for pick test
- `hitPoint` (Rhino.Geometry.Point3d) — location returned here for point picks
- `depth` (System.Double) — depth returned here for point picks LARGER values are NEARER to the camera. SMALLER values are FARTHER from the camera.
- `distance` (System.Double) — planar distance returned here for point picks. SMALLER values are CLOSER to the pick point
- `hitFlag` (Rhino.Input.Custom.PickContext.MeshHitFlag) — For point picks, How to interpret the hitIndex (vertex hit, edge hit, or face hit)
- `hitIndex` (System.Int32) — index of vertex/edge/face that was hit. Use hitFlag to determine what this index corresponds to

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Mesh,Rhino.Input.Custom.PickContext.MeshPickStyle,Rhino.Geometry.Point3d@,System.Double@,System.Double@,Rhino.Input.Custom.PickContext.MeshHitFlag@,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_4.htm)

#### `public bool PickFrustumTest(Mesh mesh, PickContext.MeshPickStyle pickStyle, out Point3d hitPoint, out Point2d hitSurfaceUV, out Point2d hitTextureCoordinate, out double depth, out double distance, out PickContext.MeshHitFlag hitFlag, out int hitIndex)`

Utility for picking meshes

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — mesh to test
- `pickStyle` (Rhino.Input.Custom.PickContext.MeshPickStyle) — mode used for pick test
- `hitPoint` (Rhino.Geometry.Point3d) — location returned here for point picks
- `hitSurfaceUV` (Rhino.Geometry.Point2d) — If the mesh has surface parameters, set to the surface parameters of the hit point
- `hitTextureCoordinate` (Rhino.Geometry.Point2d) — If the mesh has texture coordinates, set to the texture coordinate of the hit point. Note that the texture coordinates can be set in many different ways and this information is useless unless you know how the texture coordinates are set on this particular mesh.
- `depth` (System.Double) — depth returned here for point picks LARGER values are NEARER to the camera. SMALLER values are FARTHER from the camera.
- `distance` (System.Double) — planar distance returned here for point picks. SMALLER values are CLOSER to the pick point
- `hitFlag` (Rhino.Input.Custom.PickContext.MeshHitFlag) — For point picks, How to interpret the hitIndex (vertex hit, edge hit, or face hit)
- `hitIndex` (System.Int32) — index of vertex/edge/face that was hit. Use hitFlag to determine what this index corresponds to

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Mesh,Rhino.Input.Custom.PickContext.MeshPickStyle,Rhino.Geometry.Point3d@,Rhino.Geometry.Point2d@,Rhino.Geometry.Point2d@,System.Double@,System.Double@,Rhino.Input.Custom.PickContext.MeshHitFlag@,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_3.htm)

#### `public bool PickFrustumTest(NurbsCurve curve, out double t, out double depth, out double distance)`



**Parameters:**
- `curve` (Rhino.Geometry.NurbsCurve) — [Missing <param name="curve"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.NurbsCurve,System.Double@,System.Double@,System.Double@)"]
- `t` (System.Double) — [Missing <param name="t"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.NurbsCurve,System.Double@,System.Double@,System.Double@)"]
- `depth` (System.Double) — [Missing <param name="depth"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.NurbsCurve,System.Double@,System.Double@,System.Double@)"]
- `distance` (System.Double) — [Missing <param name="distance"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.NurbsCurve,System.Double@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.NurbsCurve,System.Double@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_5.htm)

#### `public bool PickFrustumTest(Point3d point, out double depth, out double distance)`

Utility for picking 3d point

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — [Missing <param name="point"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Point3d,System.Double@,System.Double@)"]
- `depth` (System.Double) — depth returned here for point picks. LARGER values are NEARER to the camera. SMALLER values are FARTHER from the camera.
- `distance` (System.Double) — planar distance returned here for point picks. SMALLER values are CLOSER to the pick point

**Returns:** `Boolean` — true if there is a hit

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_6.htm)

#### `public bool PickFrustumTest(Point3d[] points, out int pointIndex, out double depth, out double distance)`



**Parameters:**
- `points` (Rhino.Geometry.Point3d[]) — [Missing <param name="points"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Point3d[],System.Int32@,System.Double@,System.Double@)"]
- `pointIndex` (System.Int32) — [Missing <param name="pointIndex"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Point3d[],System.Int32@,System.Double@,System.Double@)"]
- `depth` (System.Double) — [Missing <param name="depth"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Point3d[],System.Int32@,System.Double@,System.Double@)"]
- `distance` (System.Double) — [Missing <param name="distance"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Point3d[],System.Int32@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.Point3d[],System.Int32@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_7.htm)

#### `public bool PickFrustumTest(PointCloud cloud, out int pointIndex, out double depth, out double distance)`



**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — [Missing <param name="cloud"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.PointCloud,System.Int32@,System.Double@,System.Double@)"]
- `pointIndex` (System.Int32) — [Missing <param name="pointIndex"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.PointCloud,System.Int32@,System.Double@,System.Double@)"]
- `depth` (System.Double) — [Missing <param name="depth"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.PointCloud,System.Int32@,System.Double@,System.Double@)"]
- `distance` (System.Double) — [Missing <param name="distance"/> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.PointCloud,System.Int32@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.Custom.PickContext.PickFrustumTest(Rhino.Geometry.PointCloud,System.Int32@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickFrustumTest_8.htm)

#### `public int[] PickMeshTopologyVertices(Mesh mesh)`

Utility for picking mesh vertices

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.Input.Custom.PickContext.PickMeshTopologyVertices(Rhino.Geometry.Mesh)"]

**Returns:** `Int32[]` — indices of mesh topology vertices that were picked

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_PickMeshTopologyVertices.htm)

#### `public void SetPickTransform(Transform transform)`



**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.Input.Custom.PickContext.SetPickTransform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_SetPickTransform.htm)

#### `public void UpdateClippingPlanes()`

Updates the clipping plane information in pick region. The SetClippingPlanes and View fields must be called before calling UpdateClippingPlanes().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_Custom_PickContext_UpdateClippingPlanes.htm)

### Properties
- `GetObjectUsed` (GetObject, get) — 
- `PickGroupsEnabled` (Boolean, get/set) — True if GroupObjects should be added to the pick list
- `PickLine` (Line, get/set) — pick chord starts on near clipping plane and ends on far clipping plane.
- `PickMode` (PickMode, get/set) — 
- `PickStyle` (PickStyle, get/set) — 
- `SubObjectSelectionEnabled` (Boolean, get/set) — True if the user had activated sub-object selection
- `View` (RhinoView, get/set) — This view can be a model view or a page view. When view is a page view, then you need to distinguish between the viewports MainViewport() and ActiveViewport(). When m_view is a model view, both MainViewport() and ActiveViewport() return the world view's viewport.

## PickContext.MeshHitFlag (enum)

[Missing <summary> documentation for "T:Rhino.Input.Custom.PickContext.MeshHitFlag"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_PickContext_MeshHitFlag.htm)

### Values
- `Invalid` = `-1`
- `Vertex` = `0`
- `Edge` = `1`
- `Face` = `2`

## PickContext.MeshPickStyle (enum)

[Missing <summary> documentation for "T:Rhino.Input.Custom.PickContext.MeshPickStyle"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_PickContext_MeshPickStyle.htm)

### Values
- `WireframePicking` = `0` — Checks for vertex and edge hits
- `ShadedModePicking` = `1` — Checks for face hits
- `VertexOnlyPicking` = `2` — Returns false if no vertices are hit

## PickMode (enum)

Picking can happen in wireframe or shaded display mode

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_PickMode.htm)

### Values
- `Wireframe` = `1`
- `Shaded` = `2`

## PickStyle (enum)

Provides picking values that describe common CAD picking behavior.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_PickStyle.htm)

### Values
- `None` = `0`
- `PointPick` = `1`
- `WindowPick` = `2`
- `CrossingPick` = `3`

## TaskCompleteEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Input.Custom.TaskCompleteEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_Custom_TaskCompleteEventArgs.htm)

### Constructors
- `public TaskCompleteEventArgs(Task task, RhinoDoc doc)` — Initializes a new instance of the TaskCompleteEventArgs class

### Properties
- `Doc` (RhinoDoc, get/set) — 
- `Redraw` (Boolean, get/set) — 
- `Task` (Task, get/set) — 

