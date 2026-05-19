---
name: grasshopper-grasshopper-rhinoceros-drafting
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Rhinoceros.Drafting namespace — 15 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelHatchLine, ModelHatchPattern, ModelHatchPattern.Attributes, ModelLinetype, ModelLinetype.Attributes, ModelLineWidth, ObjectDrafting, ObjectDraftingColor, and 7 more types.
---

# Grasshopper.Rhinoceros.Drafting

Auto-generated from vendor docs for grasshopper 8.0. 15 types in this namespace.

## ModelHatchLine (class)

Represents a Rhino model hatch line.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ModelHatchLine.htm)

### Constructors
- `public ModelHatchLine()` — Initializes a new instance of the ModelHatchLine class
- `public ModelHatchLine(ModelHatchLine.Attributes attributes)` — Initializes a new instance of the ModelHatchLine class
- `public ModelHatchLine(HatchLine hatchLine)` — Initializes a new instance of the ModelHatchLine class

### Methods
#### `public static ModelHatchLine Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelHatchLine` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Cast.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_GetHashCode.htm)

#### `public static implicit operator ModelHatchLine (HatchLine hatchLine)`



**Parameters:**
- `hatchLine` (HatchLine)

**Returns:** `ModelHatchLine` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_op_Implicit_1.htm)

#### `public static implicit operator ModelHatchLine (ModelHatchLine.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Drafting.ModelHatchLine.Attributes)

**Returns:** `ModelHatchLine` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_op_Implicit.htm)

#### `public ModelHatchLine.Attributes ToAttributes()`



**Returns:** `ModelHatchLine.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_ToAttributes.htm)

#### `public HatchLine ToHatchLine()`



**Returns:** `HatchLine` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_ToHatchLine.htm)

#### `public override string ToString()`

(Overrides ModelValue.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Angle` (Double, get) — 
- `Base` (Point2d, get) — 
- `Offset` (Vector2d, get) — 
- `Segments` (IReadOnlyList<Double>, get) — 

## ModelHatchLine.Attributes (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Drafting.ModelHatchLine.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Attributes.htm)

### Methods
#### `public bool Equals(ModelHatchLine.Attributes other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ModelHatchLine.Attributes)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Attributes_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Attributes_Equals_1.htm)

#### `public static ModelHatchLine.Attributes From(HatchLine modelHatchLine)`



**Parameters:**
- `modelHatchLine` (HatchLine)

**Returns:** `ModelHatchLine.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Attributes_GetHashCode.htm)

#### `public HatchLine ToHatchLine()`



**Returns:** `HatchLine` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchLine_Attributes_ToHatchLine.htm)

### Properties
- `Angle` (Double, get/set) — 
- `Base` (Point2d, get/set) — 
- `Offset` (Vector2d, get/set) — 
- `Segments` (IReadOnlyList<Double>, get/set) — 

## ModelHatchPattern (class)

Represents a Rhino model hatch pattern. Wraps the functionality of the HatchPattern type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern.htm)

### Constructors
- `public ModelHatchPattern()` — Initializes a new instance of the ModelHatchPattern class
- `public ModelHatchPattern(ModelHatchPattern.Attributes attributes)` — Initializes a new instance of the ModelHatchPattern class
- `public ModelHatchPattern(Guid id)` — Initializes a new instance of the ModelHatchPattern class
- `public ModelHatchPattern(HatchPattern pattern)` — Initializes a new instance of the ModelHatchPattern class

### Methods
#### `public static ModelHatchPattern Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelHatchPattern` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_Cast.htm)

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

#### `public static implicit operator ModelHatchPattern (HatchPattern pattern)`



**Parameters:**
- `pattern` (HatchPattern)

**Returns:** `ModelHatchPattern` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_op_Implicit_1.htm)

#### `public static implicit operator ModelHatchPattern (ModelHatchPattern.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Drafting.ModelHatchPattern.Attributes)

**Returns:** `ModelHatchPattern` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_op_Implicit.htm)

#### `public static implicit operator ModelHatchPattern (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelHatchPattern` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_op_Implicit_2.htm)

#### `public ModelHatchPattern.Attributes ToAttributes()`



**Returns:** `ModelHatchPattern.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_ToAttributes.htm)

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
- `FillType` (Nullable<HatchPatternFillType>, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Lines` (IReadOnlyList<ModelHatchLine>, get) — 
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `Unset` (ModelHatchPattern, get) — 
- `UserText` (ModelUserText, get) — (Inherited from ModelComponentContent.)

## ModelHatchPattern.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Drafting.ModelHatchPattern.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelHatchPattern.Attributes class

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

#### `public static implicit operator ModelHatchPattern.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelHatchPattern.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Overrides ModelComponentContent.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelHatchPattern_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `Lines` (IReadOnlyList<ModelHatchLine>, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `UserText` (ModelUserText, get/set) — (Inherited from ModelComponentContent.Attributes.)

## ModelLineWidth (class)

Represents a Rhino model linetype.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ModelLineWidth.htm)

### Constructors
- `public ModelLineWidth()` — Initializes a new instance of the ModelLineWidth class
- `public ModelLineWidth(double value)` — Initializes a new instance of the ModelLineWidth class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Number.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLineWidth_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from GH_Number.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Number.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLineWidth_Duplicate.htm)

#### `public GH_Number DuplicateNumber()`

(Inherited from GH_Number.)

**Returns:** `GH_Number` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_DuplicateNumber.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Inherited from GH_Number.)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_EmitProxy.htm)

#### `public bool Equals(ModelLineWidth other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ModelLineWidth)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLineWidth_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLineWidth_Equals_1.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLineWidth_GetHashCode.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_Number.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Number.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLineWidth_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Inherited from GH_Number.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. If the number is NaN, then it is considered invalid
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Number.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Number.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.

## ModelLinetype (class)

Represents a Rhino model linetype. Wraps the functionality of the Linetype type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ModelLinetype.htm)

### Constructors
- `public ModelLinetype()` — Initializes a new instance of the ModelLinetype class
- `public ModelLinetype(ModelLinetype.Attributes attributes)` — Initializes a new instance of the ModelLinetype class
- `public ModelLinetype(Guid id)` — Initializes a new instance of the ModelLinetype class
- `public ModelLinetype(Linetype linetype)` — Initializes a new instance of the ModelLinetype class

### Methods
#### `public static ModelLinetype Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelLinetype` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_Cast.htm)

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

#### `public static implicit operator ModelLinetype (Linetype linetype)`



**Parameters:**
- `linetype` (Linetype)

**Returns:** `ModelLinetype` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_op_Implicit_1.htm)

#### `public static implicit operator ModelLinetype (ModelLinetype.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Drafting.ModelLinetype.Attributes)

**Returns:** `ModelLinetype` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_op_Implicit.htm)

#### `public static implicit operator ModelLinetype (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelLinetype` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_op_Implicit_2.htm)

#### `public ModelLinetype.Attributes ToAttributes()`



**Returns:** `ModelLinetype.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_ToAttributes.htm)

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
- `CapStyle` (Nullable<LineCapStyle>, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `JoinStyle` (Nullable<LineJoinStyle>, get) — 
- `Name` (ModelContentName, get) — (Overrides ModelContent.Name.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Segments` (IReadOnlyList<Double>, get) — 
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `TaperPoints` (IReadOnlyList<Point2d>, get) — 
- `Unset` (ModelLinetype, get) — 
- `UserText` (ModelUserText, get) — (Inherited from ModelComponentContent.)
- `Width` (Nullable<Double>, get) — 
- `WidthUnits` (Nullable<ModelUnitSystem.Value>, get) — 

## ModelLinetype.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Drafting.ModelLinetype.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ModelLinetype_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelLinetype.Attributes class

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

#### `public static implicit operator ModelLinetype.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelLinetype.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Overrides ModelComponentContent.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelContent.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ModelLinetype_Attributes_ToString.htm)

### Properties
- `CapStyle` (Nullable<LineCapStyle>, get/set) — 
- `JoinStyle` (Nullable<LineJoinStyle>, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Segments` (IReadOnlyList<Double>, get/set) — 
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `TaperPoints` (IReadOnlyList<Point2d>, get/set) — 
- `UserText` (ModelUserText, get/set) — (Inherited from ModelComponentContent.Attributes.)
- `Width` (Nullable<Double>, get/set) — 
- `WidthUnits` (Nullable<ModelUnitSystem.Value>, get/set) — 

## ObjectDrafting (class)

Represents a Rhino object drafting attributes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDrafting.htm)

### Constructors
- `public ObjectDrafting()` — Initializes a new instance of the ObjectDrafting class
- `public ObjectDrafting(ObjectDrafting.Attributes attributes)` — Initializes a new instance of the ObjectDrafting class

### Methods
#### `public static ObjectDrafting Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectDrafting` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_CastTo__1.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_GetHashCode.htm)

#### `public static implicit operator ObjectDrafting (ObjectDrafting.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Drafting.ObjectDrafting.Attributes)

**Returns:** `ObjectDrafting` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_op_Implicit.htm)

#### `public ObjectDrafting.Attributes ToAttributes()`



**Returns:** `ObjectDrafting.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_ToAttributes.htm)

#### `public string ToDetails()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Color` (Nullable<ObjectDraftingColor.Value>, get) — 
- `LineScale` (Nullable<Double>, get) — 
- `Linetype` (Nullable<ObjectDraftingLinetype.Value>, get) — 
- `LineWidth` (Nullable<ObjectDraftingLineWidth.Value>, get) — 
- `Order` (Nullable<Int32>, get) — 

## ObjectDrafting.Attributes (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Drafting.ObjectDrafting.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_Attributes.htm)

### Methods
#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_Attributes_Equals_1.htm)

#### `public bool Equals(ObjectDrafting.Attributes other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ObjectDrafting.Attributes)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDrafting_Attributes_GetHashCode.htm)

### Properties
- `Color` (Nullable<ObjectDraftingColor.Value>, get/set) — 
- `LineScale` (Nullable<Double>, get/set) — 
- `Linetype` (Nullable<ObjectDraftingLinetype.Value>, get/set) — 
- `LineWidth` (Nullable<ObjectDraftingLineWidth.Value>, get/set) — 
- `Order` (Nullable<Int32>, get/set) — 

## ObjectDraftingColor (class)

Represents a Rhino object drafting color.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor.htm)

### Constructors
- `public ObjectDraftingColor()` — Initializes a new instance of the ObjectDraftingColor class
- `public ObjectDraftingColor(ObjectDraftingColor.Value value)` — Initializes a new instance of the ObjectDraftingColor class

### Methods
#### `public static ObjectDraftingColor Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectDraftingColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_CastTo__1.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_GetHashCode.htm)

#### `public static implicit operator ObjectDraftingColor.Value (ObjectDraftingColor data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingColor)

**Returns:** `ObjectDraftingColor.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_op_Implicit.htm)

#### `public static implicit operator ObjectDraftingColor (ObjectDraftingColor.Value data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingColor.Value)

**Returns:** `ObjectDraftingColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_op_Implicit_1.htm)

#### `public ObjectDraftingColor.Value ToAttributes()`



**Returns:** `ObjectDraftingColor.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_ToAttributes.htm)

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
- `Source` (ObjectPlotColorSource, get) — 

## ObjectDraftingColor.Value (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Drafting.ObjectDraftingColor.Value.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value.htm)

### Constructors
- `public Value(ModelColor color)` — Initializes a new instance of the ObjectDraftingColor.Value class
- `public Value(ObjectPlotColorSource source)` — Initializes a new instance of the ObjectDraftingColor.Value class
- `public Value(ObjectPlotColorSource source, ModelColor color)` — Initializes a new instance of the ObjectDraftingColor.Value class

### Methods
#### `public int CompareTo(Object other)`



**Parameters:**
- `other` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_CompareTo_1.htm)

#### `public int CompareTo(ObjectDraftingColor.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingColor.Value)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_CompareTo.htm)

#### `public void Deconstruct(out ObjectPlotColorSource source, out ModelColor value)`



**Parameters:**
- `source` (ObjectPlotColorSource)
- `value` (Grasshopper.Rhinoceros.ModelColor)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_Deconstruct.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_Equals_1.htm)

#### `public bool Equals(ObjectDraftingColor.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingColor.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_GetHashCode.htm)

#### `public static implicit operator ObjectDraftingColor.Value (ModelColor color)`



**Parameters:**
- `color` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `ObjectDraftingColor.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_op_Implicit_2.htm)

#### `public static implicit operator ModelColor (ObjectDraftingColor.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingColor.Value)

**Returns:** `ModelColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_op_Implicit_1.htm)

#### `public static implicit operator ObjectPlotColorSource (ObjectDraftingColor.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingColor.Value)

**Returns:** `ObjectPlotColorSource` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_op_Implicit.htm)

#### `public static implicit operator ObjectDraftingColor.Value (ObjectPlotColorSource source)`



**Parameters:**
- `source` (ObjectPlotColorSource)

**Returns:** `ObjectDraftingColor.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_op_Implicit_3.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingColor_Value_ToString.htm)

### Properties
- `ByDisplay` (ObjectDraftingColor.Value, get) — 
- `ByLayer` (ObjectDraftingColor.Value, get) — 
- `ByParent` (ObjectDraftingColor.Value, get) — 
- `Color` (ModelColor, get) — 
- `Source` (ObjectPlotColorSource, get) — 

## ObjectDraftingLineWidth (class)

Represents a Rhino object drafintg linetype.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth.htm)

### Constructors
- `public ObjectDraftingLineWidth()` — Initializes a new instance of the ObjectDraftingLineWidth class
- `public ObjectDraftingLineWidth(ObjectDraftingLineWidth.Value value)` — Initializes a new instance of the ObjectDraftingLineWidth class

### Methods
#### `public static ObjectDraftingLineWidth Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectDraftingLineWidth` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_CastTo__1.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_GetHashCode.htm)

#### `public static implicit operator ObjectDraftingLineWidth.Value (ObjectDraftingLineWidth data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLineWidth)

**Returns:** `ObjectDraftingLineWidth.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_op_Implicit.htm)

#### `public static implicit operator ObjectDraftingLineWidth (ObjectDraftingLineWidth.Value data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLineWidth.Value)

**Returns:** `ObjectDraftingLineWidth` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_op_Implicit_1.htm)

#### `public ObjectDraftingLineWidth.Value ToAttributes()`



**Returns:** `ObjectDraftingLineWidth.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_ToAttributes.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Source` (ObjectPlotWeightSource, get) — 
- `Width` (Nullable<Double>, get) — 

## ObjectDraftingLineWidth.Value (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Drafting.ObjectDraftingLineWidth.Value.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value.htm)

### Constructors
- `public Value(double value)` — Initializes a new instance of the ObjectDraftingLineWidth.Value class
- `public Value(ObjectPlotWeightSource source)` — Initializes a new instance of the ObjectDraftingLineWidth.Value class
- `public Value(ObjectPlotWeightSource source, double value)` — Initializes a new instance of the ObjectDraftingLineWidth.Value class

### Methods
#### `public int CompareTo(Object obj)`



**Parameters:**
- `obj` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_CompareTo_1.htm)

#### `public int CompareTo(ObjectDraftingLineWidth.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLineWidth.Value)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_CompareTo.htm)

#### `public void Deconstruct(out ObjectPlotWeightSource source, out double value)`



**Parameters:**
- `source` (ObjectPlotWeightSource)
- `value` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_Deconstruct.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_Equals_1.htm)

#### `public bool Equals(ObjectDraftingLineWidth.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLineWidth.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_GetHashCode.htm)

#### `public static implicit operator ObjectDraftingLineWidth.Value (double color)`



**Parameters:**
- `color` (System.Double)

**Returns:** `ObjectDraftingLineWidth.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_op_Implicit_3.htm)

#### `public static implicit operator double (ObjectDraftingLineWidth.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLineWidth.Value)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_op_Implicit_1.htm)

#### `public static implicit operator ObjectPlotWeightSource (ObjectDraftingLineWidth.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLineWidth.Value)

**Returns:** `ObjectPlotWeightSource` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_op_Implicit.htm)

#### `public static implicit operator ObjectDraftingLineWidth.Value (ObjectPlotWeightSource source)`



**Parameters:**
- `source` (ObjectPlotWeightSource)

**Returns:** `ObjectDraftingLineWidth.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_op_Implicit_2.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLineWidth_Value_ToString.htm)

### Properties
- `ByLayer` (ObjectDraftingLineWidth.Value, get) — 
- `ByParent` (ObjectDraftingLineWidth.Value, get) — 
- `Source` (ObjectPlotWeightSource, get) — 
- `Width` (Double, get) — 

## ObjectDraftingLinetype (class)

Represents a Rhino object drafintg linetype.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype.htm)

### Constructors
- `public ObjectDraftingLinetype()` — Initializes a new instance of the ObjectDraftingLinetype class
- `public ObjectDraftingLinetype(ObjectDraftingLinetype.Value value)` — Initializes a new instance of the ObjectDraftingLinetype class

### Methods
#### `public static ObjectDraftingLinetype Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectDraftingLinetype` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_CastTo__1.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_GetHashCode.htm)

#### `public static implicit operator ObjectDraftingLinetype.Value (ObjectDraftingLinetype data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLinetype)

**Returns:** `ObjectDraftingLinetype.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_op_Implicit.htm)

#### `public static implicit operator ObjectDraftingLinetype (ObjectDraftingLinetype.Value data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLinetype.Value)

**Returns:** `ObjectDraftingLinetype` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_op_Implicit_1.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Linetype` (ModelLinetype, get) — 
- `Source` (ObjectLinetypeSource, get) — 

## ObjectDraftingLinetype.Value (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Drafting.ObjectDraftingLinetype.Value.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value.htm)

### Constructors
- `public Value(ModelLinetype value)` — Initializes a new instance of the ObjectDraftingLinetype.Value class
- `public Value(ObjectLinetypeSource source)` — Initializes a new instance of the ObjectDraftingLinetype.Value class
- `public Value(ObjectLinetypeSource source, ModelLinetype value)` — Initializes a new instance of the ObjectDraftingLinetype.Value class

### Methods
#### `public int CompareTo(Object obj)`



**Parameters:**
- `obj` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_CompareTo_1.htm)

#### `public int CompareTo(ObjectDraftingLinetype.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLinetype.Value)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_CompareTo.htm)

#### `public void Deconstruct(out ObjectLinetypeSource source, out ModelLinetype value)`



**Parameters:**
- `source` (ObjectLinetypeSource)
- `value` (Grasshopper.Rhinoceros.Drafting.ModelLinetype)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_Deconstruct.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_Equals_1.htm)

#### `public bool Equals(ObjectDraftingLinetype.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLinetype.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_GetHashCode.htm)

#### `public static implicit operator ObjectDraftingLinetype.Value (ModelLinetype color)`



**Parameters:**
- `color` (Grasshopper.Rhinoceros.Drafting.ModelLinetype)

**Returns:** `ObjectDraftingLinetype.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_op_Implicit.htm)

#### `public static implicit operator ModelLinetype (ObjectDraftingLinetype.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLinetype.Value)

**Returns:** `ModelLinetype` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_op_Implicit_2.htm)

#### `public static implicit operator ObjectLinetypeSource (ObjectDraftingLinetype.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Drafting.ObjectDraftingLinetype.Value)

**Returns:** `ObjectLinetypeSource` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_op_Implicit_1.htm)

#### `public static implicit operator ObjectDraftingLinetype.Value (ObjectLinetypeSource source)`



**Parameters:**
- `source` (ObjectLinetypeSource)

**Returns:** `ObjectDraftingLinetype.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_op_Implicit_3.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Drafting_ObjectDraftingLinetype_Value_ToString.htm)

### Properties
- `ByLayer` (ObjectDraftingLinetype.Value, get) — 
- `ByParent` (ObjectDraftingLinetype.Value, get) — 
- `Continuous` (ObjectDraftingLinetype.Value, get) — 
- `Linetype` (ModelLinetype, get) — 
- `Source` (ObjectLinetypeSource, get) — 

