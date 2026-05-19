---
name: grasshopper-grasshopper-rhinoceros-display
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Rhinoceros.Display namespace — 19 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DisplayColorGradient, DisplayColorStop, ModelDisplayMode, ModelDisplayMode.Attributes, ModelNamedView, ModelNamedView.Attributes, ModelPageViewport, ModelPageViewport.Attributes, and 11 more types.
---

# Grasshopper.Rhinoceros.Display

Auto-generated from vendor docs for grasshopper 8.0. 19 types in this namespace.

## DisplayColorGradient (class)

Represents a gradient colour stop. Wraps the functionality of the ColorGradient type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_DisplayColorGradient.htm)

### Constructors
- `public DisplayColorGradient()` — Blank constructor
- `public DisplayColorGradient(ColorGradient colour)` — Initializes a new instance of the DisplayColorGradient class
- `public DisplayColorGradient(DisplayColorGradient other)` — Initializes a new instance of the DisplayColorGradient class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_CastTo__1.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_Duplicate.htm)

#### `public DisplayColorGradient DuplicateColourGradient()`



**Returns:** `DisplayColorGradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_DuplicateColourGradient.htm)

#### `public virtual IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorGradient_Write.htm)

### Properties
- `ClippingBox` (BoundingBox, get) — 
- `IsValid` (Boolean, get) — Gets the validity of this instance.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.

## DisplayColorStop (class)

Represents a gradient colour stop. Wraps the functionality of the ColorStop type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_DisplayColorStop.htm)

### Constructors
- `public DisplayColorStop()` — Blank constructor
- `public DisplayColorStop(ColorStop colour)` — Initializes a new instance of the DisplayColorStop class
- `public DisplayColorStop(DisplayColorStop other)` — Initializes a new instance of the DisplayColorStop class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorStop_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorStop_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorStop_Duplicate.htm)

#### `public DisplayColorStop DuplicateColorStop()`



**Returns:** `DisplayColorStop` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorStop_DuplicateColorStop.htm)

#### `public virtual IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorStop_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorStop_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_DisplayColorStop_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.

## ModelDisplayMode (class)

Represents a Rhino display mode. Wraps the functionality of the DisplayModeDescription type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelDisplayMode.htm)

### Constructors
- `public ModelDisplayMode()` — Initializes a new instance of the ModelDisplayMode class
- `public ModelDisplayMode(ModelDisplayMode.Attributes attributes)` — Initializes a new instance of the ModelDisplayMode class
- `public ModelDisplayMode(DisplayModeDescription displayMode)` — Initializes a new instance of the ModelDisplayMode class
- `public ModelDisplayMode(Guid id)` — Initializes a new instance of the ModelDisplayMode class

### Methods
#### `public static ModelDisplayMode Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelDisplayMode` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from ModelContent.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelContent other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelContent)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals_1.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `public static implicit operator ModelDisplayMode (DisplayModeDescription mode)`



**Parameters:**
- `mode` (DisplayModeDescription)

**Returns:** `ModelDisplayMode` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_op_Implicit_1.htm)

#### `public static implicit operator ModelDisplayMode (Guid id)`



**Parameters:**
- `id` (System.Guid)

**Returns:** `ModelDisplayMode` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_op_Implicit_2.htm)

#### `public static implicit operator ModelDisplayMode (ModelDisplayMode.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Display.ModelDisplayMode.Attributes)

**Returns:** `ModelDisplayMode` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_op_Implicit.htm)

#### `public static implicit operator ModelDisplayMode (string name)`



**Parameters:**
- `name` (System.String)

**Returns:** `ModelDisplayMode` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_op_Implicit_3.htm)

#### `public ModelDisplayMode.Attributes ToAttributes()`



**Returns:** `ModelDisplayMode.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Overrides ModelContent.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_ToString.htm)

#### `public override string TooltipString()`

(Overrides ModelContent.TooltipString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_TooltipString.htm)

### Properties
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `Unset` (ModelDisplayMode, get) — 

## ModelDisplayMode.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ModelDisplayMode.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelDisplayMode_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelDisplayMode.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.Attributes.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `public virtual string ToDetails()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelContent.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelDisplayMode_Attributes_ToString.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)

## ModelNamedView (class)

Represents a Rhino named view. Wraps a ViewInfo instance in the named views table.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelNamedView.htm)

### Methods
#### `public static ModelNamedView Cast(Object data)`



**Parameters:**
- `data` (System.Object)

**Returns:** `ModelNamedView` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from ModelContent.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelContent other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelContent)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals_1.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `public static implicit operator ModelNamedView (ModelNamedView.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Display.ModelNamedView.Attributes)

**Returns:** `ModelNamedView` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_op_Implicit.htm)

#### `public static implicit operator ModelNamedView (ViewInfo viewportInfo)`



**Parameters:**
- `viewportInfo` (ViewInfo)

**Returns:** `ModelNamedView` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_op_Implicit_1.htm)

#### `public ModelNamedView.Attributes ToAttributes()`



**Returns:** `ModelNamedView.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_ToString.htm)

#### `public override string TooltipString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_TooltipString.htm)

### Properties
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)

## ModelNamedView.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ModelNamedView.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelNamedView_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelNamedView.Attributes class

### Methods
#### `protected internal virtual ModelContent.Attributes AsFrozen()`

(Inherited from ModelContent.Attributes.)

**Returns:** `ModelContent.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_AsFrozen.htm)

#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Equals.htm)

#### `protected override bool Equals(Object other, IEqualityComparer comparer)`

(Overrides ModelContent.Attributes.Equals(Object, IEqualityComparer).)

**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.Attributes.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `protected override int GetHashCode(IEqualityComparer comparer)`

(Overrides ModelContent.Attributes.GetHashCode(IEqualityComparer).)

**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_Attributes_GetHashCode.htm)

#### `protected internal virtual bool IsValidContentName(ModelContentName value, bool? parent)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `value` (Grasshopper.Rhinoceros.ModelContentName)
- `parent` (System.Nullable<Boolean>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_IsValidContentName.htm)

#### `protected internal override sealed bool Read(GH_IReader reader)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Read.htm)

#### `protected override bool ReadData(GH_IReader reader)`

(Overrides ModelContent.Attributes.ReadData(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_Attributes_ReadData.htm)

#### `protected virtual ModelData.Attributes SubClone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_SubClone.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

#### `protected internal override sealed bool Write(GH_IWriter writer)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Write.htm)

#### `protected override bool WriteData(GH_IWriter writer)`

(Overrides ModelContent.Attributes.WriteData(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelNamedView_Attributes_WriteData.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `View` (ModelView, get/set) — 

## ModelPageViewport (class)

Represents a Rhino page layout. Wraps the main viewport of a RhinoPageView instance.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelPageViewport.htm)

### Constructors
- `public ModelPageViewport()` — Initializes a new instance of the ModelPageViewport class
- `public ModelPageViewport(ModelPageViewport.Attributes attributes)` — Initializes a new instance of the ModelPageViewport class
- `public ModelPageViewport(Guid id)` — Initializes a new instance of the ModelPageViewport class
- `public ModelPageViewport(RhinoViewport viewport)` — Initializes a new instance of the ModelPageViewport class

### Methods
#### `public static ModelPageViewport Cast(Object data)`



**Parameters:**
- `data` (System.Object)

**Returns:** `ModelPageViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelPageViewport_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelViewport.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelPageViewport_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from ModelViewport.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from ModelContent.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelContent other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelContent)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals_1.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `public static implicit operator ModelPageViewport (ModelPageViewport.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Display.ModelPageViewport.Attributes)

**Returns:** `ModelPageViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelPageViewport_op_Implicit.htm)

#### `public static implicit operator ModelPageViewport (RhinoViewport viewport)`



**Parameters:**
- `viewport` (RhinoViewport)

**Returns:** `ModelPageViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelPageViewport_op_Implicit_1.htm)

#### `public ModelPageViewport.Attributes ToAttributes()`



**Returns:** `ModelPageViewport.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelPageViewport_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_ToString.htm)

#### `public override string TooltipString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_TooltipString.htm)

### Properties
- `DisplayMode` (ModelDisplayMode, get) — (Inherited from ModelViewport.)
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `PageNumber` (Nullable<Int32>, get) — 
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Space` (ActiveSpace, get) — (Overrides ModelViewport.Space.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `ViewportType` (ViewportType, get) — (Overrides ModelViewport.ViewportType.)

## ModelPageViewport.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ModelPageViewport.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelPageViewport_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelPageViewport.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.Attributes.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `public override string ToDetails()`

(Overrides ModelViewport.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelPageViewport_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelViewport.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelPageViewport_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `DisplayMode` (ModelDisplayMode, get/set) — (Inherited from ModelViewport.Attributes.)
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `PageNumber` (Nullable<Int32>, get/set) — 
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `UserText` (ModelUserText, get/set) — (Inherited from ModelViewport.Attributes.)
- `View` (ModelView, get/set) — (Inherited from ModelViewport.Attributes.)

## ModelStandardViewport (class)

Represents a Rhino standard modeling viewport. Wraps the main viewport of a modeling RhinoView instance.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelStandardViewport.htm)

### Constructors
- `public ModelStandardViewport()` — Initializes a new instance of the ModelStandardViewport class
- `public ModelStandardViewport(ModelStandardViewport.Attributes attributes)` — Initializes a new instance of the ModelStandardViewport class
- `public ModelStandardViewport(Guid id)` — Initializes a new instance of the ModelStandardViewport class
- `public ModelStandardViewport(RhinoViewport viewport)` — Initializes a new instance of the ModelStandardViewport class

### Methods
#### `public static ModelStandardViewport Cast(Object data)`



**Parameters:**
- `data` (System.Object)

**Returns:** `ModelStandardViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelStandardViewport_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelViewport.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelStandardViewport_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from ModelViewport.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from ModelContent.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelContent other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelContent)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals_1.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `public static implicit operator ModelStandardViewport (ModelStandardViewport.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Display.ModelStandardViewport.Attributes)

**Returns:** `ModelStandardViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelStandardViewport_op_Implicit.htm)

#### `public static implicit operator ModelStandardViewport (RhinoViewport viewport)`



**Parameters:**
- `viewport` (RhinoViewport)

**Returns:** `ModelStandardViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelStandardViewport_op_Implicit_1.htm)

#### `public ModelStandardViewport.Attributes ToAttributes()`



**Returns:** `ModelStandardViewport.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelStandardViewport_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_ToString.htm)

#### `public override string TooltipString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_TooltipString.htm)

### Properties
- `DisplayMode` (ModelDisplayMode, get) — (Inherited from ModelViewport.)
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Space` (ActiveSpace, get) — (Overrides ModelViewport.Space.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `ViewportType` (ViewportType, get) — (Overrides ModelViewport.ViewportType.)

## ModelStandardViewport.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ModelStandardViewport.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelStandardViewport_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelStandardViewport.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.Attributes.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `public override string ToDetails()`

(Overrides ModelViewport.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelStandardViewport_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelViewport.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelStandardViewport_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `DisplayMode` (ModelDisplayMode, get/set) — (Inherited from ModelViewport.Attributes.)
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `UserText` (ModelUserText, get/set) — (Inherited from ModelViewport.Attributes.)
- `View` (ModelView, get/set) — (Inherited from ModelViewport.Attributes.)

## ModelView (class)

Represents a 3D view frustum. Wraps the functionality of the ViewportInfo type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelView.htm)

### Constructors
- `public ModelView()` — Initializes a new instance of the ModelView class
- `public ModelView(Guid ref_guid)` — Create a new referenced view that references a RhinoView or NamedView object with the specified ref_guid.
- `public ModelView(ModelView other)` — Initializes a new instance of the ModelView class
- `public ModelView(RhinoView view)` — Initializes a new instance of the ModelView class
- `public ModelView(RhinoViewport viewport)` — Initializes a new instance of the ModelView class
- `public ModelView(ViewInfo viewInfo)` — Initializes a new instance of the ModelView class
- `public ModelView(ViewportInfo viewportInfo)` — Initializes a new instance of the ModelView class
- `public ModelView(ViewportInfo viewportInfo, string title)` — Initializes a new instance of the ModelView class

### Methods
#### `public static ModelView Cast(Object data)`



**Parameters:**
- `data` (System.Object)

**Returns:** `ModelView` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_Cast.htm)

#### `internal bool CastTo<Q>(out Q target)`



**Parameters:**
- `target` (Q)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_CastTo__1.htm)

#### `public bool Equals(ModelView other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Display.ModelView)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_Equals_1.htm)

#### `public BoundingBox GetBoundingBox(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_GetBoundingBox.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_GetHashCode.htm)

#### `public IGH_GeometricGoo Morph(SpaceMorph xmorph)`



**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_Morph.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_ToString.htm)

#### `public ViewportInfo ToViewportInfo()`



**Returns:** `ViewportInfo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_ToViewportInfo.htm)

#### `public IGH_GeometricGoo Transform(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelView_Transform.htm)

### Properties
- `IsValid` (Boolean, get) — 
- `Title` (String, get) — 

## ModelViewport (class)

Represents a Rhino viewport. Wraps the main viewport of a RhinoView instance.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelViewport.htm)

### Methods
#### `protected internal ModelContent AsFrozen(bool deep = true)`

Try to obtain a dereferenced snapshot of itself.

**Parameters:**
- `deep` (System.Boolean) — Whether or not referenced content should also be frozen.

**Returns:** `ModelContent` — Itself if it is already dereferenced.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_AsFrozen.htm)

#### `public static ModelViewport Cast(Object data)`



**Parameters:**
- `data` (System.Object)

**Returns:** `ModelViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from ModelContent.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelContent other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelContent)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals_1.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `public static implicit operator ModelViewport (ModelViewport.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Display.ModelViewport.Attributes)

**Returns:** `ModelViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_op_Implicit.htm)

#### `public static implicit operator ModelViewport (RhinoViewport viewport)`



**Parameters:**
- `viewport` (RhinoViewport)

**Returns:** `ModelViewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_op_Implicit_1.htm)

#### `protected bool InternaliseData()`

(Inherited from ModelContent.)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_InternaliseData.htm)

#### `protected virtual bool LoadReferencedData()`

(Inherited from ModelContent.)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_LoadReferencedData.htm)

#### `protected override bool Read(GH_IReader reader)`

(Inherited from ModelContent.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Read.htm)

#### `protected internal Guid? RuntimeId(RhinoDoc document)`

(Inherited from ModelContent.)

**Parameters:**
- `document` (RhinoDoc)

**Returns:** `Nullable<Guid>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_RuntimeId.htm)

#### `public ModelViewport.Attributes ToAttributes()`



**Returns:** `ModelViewport.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_ToString.htm)

#### `public override string TooltipString()`

(Inherited from ModelContent.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_TooltipString.htm)

#### `protected virtual void UnloadReferencedData()`

(Inherited from ModelContent.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_UnloadReferencedData.htm)

#### `protected override bool Write(GH_IWriter writer)`

(Inherited from ModelContent.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Write.htm)

### Properties
- `Attribs` (ModelViewport.Attributes, get/set) — 
- `DefaultAttributes` (ModelData.Attributes, get) — (Overrides ModelData.DefaultAttributes.)
- `DisplayMode` (ModelDisplayMode, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsReferencedData` (Boolean, get) — (Inherited from ModelContent.)
- `IsReferencedDataLoaded` (Boolean, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `IsValidWhyNot` (String, get) — (Inherited from ModelContent.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Space` (ActiveSpace, get) — 
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `TypeDescription` (String, get) — (Inherited from ModelData.)
- `Unset` (ModelViewport, get) — 
- `ViewportType` (ViewportType, get) — 
- `Serial` (Int32, get) — (Inherited from ModelData.)

## ModelViewport.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ModelViewport.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ModelViewport_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelViewport.Attributes class

### Methods
#### `protected internal virtual ModelContent.Attributes AsFrozen()`

(Inherited from ModelContent.Attributes.)

**Returns:** `ModelContent.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_AsFrozen.htm)

#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Equals.htm)

#### `protected override bool Equals(Object other, IEqualityComparer comparer)`

(Overrides ModelContent.Attributes.Equals(Object, IEqualityComparer).)

**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.Attributes.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `protected override int GetHashCode(IEqualityComparer comparer)`

(Overrides ModelContent.Attributes.GetHashCode(IEqualityComparer).)

**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_Attributes_GetHashCode.htm)

#### `protected internal virtual bool IsValidContentName(ModelContentName value, bool? parent)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `value` (Grasshopper.Rhinoceros.ModelContentName)
- `parent` (System.Nullable<Boolean>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_IsValidContentName.htm)

#### `protected internal override sealed bool Read(GH_IReader reader)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Read.htm)

#### `protected override bool ReadData(GH_IReader reader)`

(Overrides ModelContent.Attributes.ReadData(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_Attributes_ReadData.htm)

#### `protected virtual ModelData.Attributes SubClone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_SubClone.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

#### `protected internal override sealed bool Write(GH_IWriter writer)`

(Inherited from ModelContent.Attributes.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Write.htm)

#### `protected override bool WriteData(GH_IWriter writer)`

(Overrides ModelContent.Attributes.WriteData(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ModelViewport_Attributes_WriteData.htm)

### Properties
- `DisplayMode` (ModelDisplayMode, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `UserText` (ModelUserText, get/set) — 
- `View` (ModelView, get/set) — 

## ObjectDisplay (class)

Represents a Rhino object display attributes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ObjectDisplay.htm)

### Constructors
- `public ObjectDisplay()` — Initializes a new instance of the ObjectDisplay class
- `public ObjectDisplay(ObjectDisplay.Attributes attributes)` — Initializes a new instance of the ObjectDisplay class

### Methods
#### `public static ObjectDisplay Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectDisplay` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelValue.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_CastTo__1.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_GetHashCode.htm)

#### `public static implicit operator ObjectDisplay (ObjectDisplay.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Display.ObjectDisplay.Attributes)

**Returns:** `ObjectDisplay` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_op_Implicit.htm)

#### `public ObjectDisplay.Attributes ToAttributes()`



**Returns:** `ObjectDisplay.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_ToAttributes.htm)

#### `public string ToDetails()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Color` (Nullable<ObjectDisplayColor.Value>, get) — 
- `WireDensity` (Nullable<Int32>, get) — 

## ObjectDisplay.Attributes (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ObjectDisplay.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ObjectDisplay_Attributes.htm)

### Methods
#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_Attributes_Equals_1.htm)

#### `public bool Equals(ObjectDisplay.Attributes other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Display.ObjectDisplay.Attributes)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplay_Attributes_GetHashCode.htm)

### Properties
- `Color` (Nullable<ObjectDisplayColor.Value>, get/set) — 
- `Mode` (ModelDisplayMode, get/set) — 
- `WireDensity` (Nullable<Int32>, get/set) — 

## ObjectDisplayColor (class)

Represents a Rhino object display color.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ObjectDisplayColor.htm)

### Constructors
- `public ObjectDisplayColor()` — Initializes a new instance of the ObjectDisplayColor class
- `public ObjectDisplayColor(ObjectDisplayColor.Value value)` — Initializes a new instance of the ObjectDisplayColor class

### Methods
#### `public static ObjectDisplayColor Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectDisplayColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelValue.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_CastTo__1.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_GetHashCode.htm)

#### `public static implicit operator ObjectDisplayColor.Value (ObjectDisplayColor data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Display.ObjectDisplayColor)

**Returns:** `ObjectDisplayColor.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_op_Implicit.htm)

#### `public static implicit operator ObjectDisplayColor (ObjectDisplayColor.Value data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Display.ObjectDisplayColor.Value)

**Returns:** `ObjectDisplayColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_op_Implicit_1.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Color` (Nullable<ModelColor>, get) — 
- `Source` (ObjectColorSource, get) — 

## ObjectDisplayColor.Value (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ObjectDisplayColor.Value.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value.htm)

### Constructors
- `public Value(ModelColor color)` — Initializes a new instance of the ObjectDisplayColor.Value class
- `public Value(ObjectColorSource source)` — Initializes a new instance of the ObjectDisplayColor.Value class
- `public Value(ObjectColorSource source, ModelColor color)` — Initializes a new instance of the ObjectDisplayColor.Value class

### Methods
#### `public int CompareTo(Object other)`



**Parameters:**
- `other` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_CompareTo_1.htm)

#### `public int CompareTo(ObjectDisplayColor.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Display.ObjectDisplayColor.Value)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_CompareTo.htm)

#### `public void Deconstruct(out ObjectColorSource source, out ModelColor value)`



**Parameters:**
- `source` (ObjectColorSource)
- `value` (Grasshopper.Rhinoceros.ModelColor)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_Deconstruct.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_Equals_1.htm)

#### `public bool Equals(ObjectDisplayColor.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Display.ObjectDisplayColor.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_GetHashCode.htm)

#### `public static implicit operator ObjectDisplayColor.Value (ModelColor color)`



**Parameters:**
- `color` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `ObjectDisplayColor.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_op_Implicit_2.htm)

#### `public static implicit operator ObjectDisplayColor.Value (ObjectColorSource source)`



**Parameters:**
- `source` (ObjectColorSource)

**Returns:** `ObjectDisplayColor.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_op_Implicit_3.htm)

#### `public static implicit operator ModelColor (ObjectDisplayColor.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Display.ObjectDisplayColor.Value)

**Returns:** `ModelColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_op_Implicit_1.htm)

#### `public static implicit operator ObjectColorSource (ObjectDisplayColor.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Display.ObjectDisplayColor.Value)

**Returns:** `ObjectColorSource` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_op_Implicit.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectDisplayColor_Value_ToString.htm)

### Properties
- `ByLayer` (ObjectDisplayColor.Value, get) — 
- `ByMaterial` (ObjectDisplayColor.Value, get) — 
- `ByParent` (ObjectDisplayColor.Value, get) — 
- `Color` (ModelColor, get) — 
- `Source` (ObjectColorSource, get) — 

## ObjectVisibility (class)

Represents a Rhino object visibility attributes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ObjectVisibility.htm)

### Constructors
- `public ObjectVisibility()` — Initializes a new instance of the ObjectVisibility class
- `public ObjectVisibility(ObjectVisibility.Attributes attributes)` — Initializes a new instance of the ObjectVisibility class

### Methods
#### `public static ObjectVisibility Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectVisibility` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelValue.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_CastTo__1.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_GetHashCode.htm)

#### `public static implicit operator ObjectVisibility (ObjectVisibility.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Display.ObjectVisibility.Attributes)

**Returns:** `ObjectVisibility` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_op_Implicit.htm)

#### `public ObjectVisibility.Attributes ToAttributes()`



**Returns:** `ObjectVisibility.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_ToAttributes.htm)

#### `public string ToDetails()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Hidden` (Nullable<Boolean>, get) — 
- `Locked` (Nullable<Boolean>, get) — 
- `Viewport` (ModelViewport, get) — 

## ObjectVisibility.Attributes (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Display.ObjectVisibility.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Display_ObjectVisibility_Attributes.htm)

### Methods
#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_Attributes_Equals_1.htm)

#### `public bool Equals(ObjectVisibility.Attributes other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Display.ObjectVisibility.Attributes)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Display_ObjectVisibility_Attributes_GetHashCode.htm)

### Properties
- `Hidden` (Nullable<Boolean>, get/set) — 
- `Locked` (Nullable<Boolean>, get/set) — 
- `Viewport` (ModelViewport, get/set) — 

