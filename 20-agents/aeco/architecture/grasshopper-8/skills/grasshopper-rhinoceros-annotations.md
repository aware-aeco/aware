---
name: grasshopper-grasshopper-rhinoceros-annotations
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Rhinoceros.Annotations namespace — 17 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AnnotationArrow, AnnotationArrow.Attributes, AnnotationArrowSettings, AnnotationDateTimeFormat, AnnotationDimensionSettings, AnnotationArrowSettings.Attributes, AnnotationDimensionSettings.Attributes, AnnotationLeaderSettings, and 9 more types.
---

# Grasshopper.Rhinoceros.Annotations

Auto-generated from vendor docs for grasshopper 8.0. 17 types in this namespace.

## AnnotationArrow (class)

Represents an arrowhead type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationArrow.htm)

### Constructors
- `public AnnotationArrow()` — Initializes a new instance of the AnnotationArrow class
- `public AnnotationArrow(ArrowType arrowType)` — Initializes a new instance of the AnnotationArrow class
- `public AnnotationArrow(AnnotationArrow.Attributes attributes)` — Initializes a new instance of the AnnotationArrow class

### Methods
#### `public static AnnotationArrow Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `AnnotationArrow` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Cast.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public override int GetHashCode()`

(Inherited from ModelData.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `public static implicit operator AnnotationArrow (AnnotationArrow.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.AnnotationArrow.Attributes)

**Returns:** `AnnotationArrow` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_op_Implicit.htm)

#### `public static implicit operator AnnotationArrow (ArrowType arrowType)`



**Parameters:**
- `arrowType` (ArrowType)

**Returns:** `AnnotationArrow` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_op_Implicit_1.htm)

#### `public ModelData.Attributes ToAttributes()`

(Inherited from ModelData.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

### Properties
- `ArrowType` (Nullable<ArrowType>, get) — 

## AnnotationArrow.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.AnnotationArrow.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the AnnotationArrow.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes_Equals.htm)

#### `protected bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes_Equals_1.htm)

#### `public static AnnotationArrow.Attributes From(ArrowType arrowType)`



**Parameters:**
- `arrowType` (ArrowType)

**Returns:** `AnnotationArrow.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes_GetHashCode.htm)

#### `protected int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes_GetHashCode_1.htm)

#### `public virtual string ToDetails()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelData.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrow_Attributes_ToString.htm)

### Properties
- `ArrowType` (Nullable<ArrowType>, get/set) — 

## AnnotationArrowSettings (class)

Represents an annotation base arrowhead settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings.htm)

### Constructors
- `public AnnotationArrowSettings()` — Initializes a new instance of the AnnotationArrowSettings class
- `public AnnotationArrowSettings(AnnotationArrowSettings.Attributes attributes)` — Initializes a new instance of the AnnotationArrowSettings class
- `public AnnotationArrowSettings(DimensionStyle dimStyle)` — Initializes a new instance of the AnnotationArrowSettings class

### Methods
#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public static explicit operator DimensionStyle (AnnotationArrowSettings settings)`



**Parameters:**
- `settings` (Grasshopper.Rhinoceros.Annotations.AnnotationArrowSettings)

**Returns:** `DimensionStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_op_Explicit.htm)

#### `public void GetDimensionStyle(ref DimensionStyle baseDimensionStyle = null)`



**Parameters:**
- `baseDimensionStyle` (DimensionStyle)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_GetDimensionStyle.htm)

#### `public override int GetHashCode()`

(Inherited from ModelData.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `public static implicit operator AnnotationArrowSettings (AnnotationArrowSettings.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.AnnotationArrowSettings.Attributes)

**Returns:** `AnnotationArrowSettings` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_op_Implicit.htm)

#### `public AnnotationArrowSettings.Attributes ToAttributes()`



**Returns:** `AnnotationArrowSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

### Properties
- `Arrow1` (AnnotationArrow, get) — 
- `Arrow2` (AnnotationArrow, get) — 
- `ArrowSize` (Nullable<Double>, get) — 
- `DimensionInline` (Nullable<Boolean>, get) — 
- `FitArrow` (Nullable<ArrowFit>, get) — 
- `LeaderArrow` (AnnotationArrow, get) — 
- `LeaderArrowSize` (Nullable<Double>, get) — 

## AnnotationArrowSettings.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.AnnotationArrowSettings.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the AnnotationArrowSettings.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes_Equals.htm)

#### `protected bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes_Equals_1.htm)

#### `public static AnnotationArrowSettings.Attributes From(DimensionStyle dimStyle)`



**Parameters:**
- `dimStyle` (DimensionStyle)

**Returns:** `AnnotationArrowSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes_GetHashCode.htm)

#### `protected int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes_GetHashCode_1.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationArrowSettings_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToString.htm)

### Properties
- `Arrow1` (AnnotationArrow, get/set) — 
- `Arrow2` (AnnotationArrow, get/set) — 
- `ArrowSize` (Nullable<Double>, get/set) — 
- `DimensionInline` (Nullable<Boolean>, get/set) — 
- `FitArrow` (Nullable<ArrowFit>, get/set) — 
- `LeaderArrow` (AnnotationArrow, get/set) — 
- `LeaderArrowSize` (Nullable<Double>, get/set) — 

## AnnotationDateTimeFormat (class)

Represents a Date Time format.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationDateTimeFormat.htm)

### Constructors
- `public AnnotationDateTimeFormat()` — Initializes a new instance of the AnnotationDateTimeFormat class
- `public AnnotationDateTimeFormat(GH_String value)` — Initializes a new instance of the AnnotationDateTimeFormat class
- `public AnnotationDateTimeFormat(string value)` — Initializes a new instance of the AnnotationDateTimeFormat class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_String.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDateTimeFormat_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Inherited from GH_String.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_String.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDateTimeFormat_Duplicate.htm)

#### `public GH_String DuplicateString()`

(Inherited from GH_String.)

**Returns:** `GH_String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_DuplicateString.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Inherited from GH_String.)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_EmitProxy.htm)

#### `public bool Equals(AnnotationDateTimeFormat other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Annotations.AnnotationDateTimeFormat)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDateTimeFormat_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDateTimeFormat_Equals_1.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDateTimeFormat_GetHashCode.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_String.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Inherited from GH_String.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Inherited from GH_String.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_Write.htm)

### Properties
- `DisplayName` (String, get) — 
- `IsValid` (Boolean, get) — Gets the validity of this instance. String are valid if they are not null.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Inherited from GH_String.)
- `TypeName` (String, get) — (Inherited from GH_String.)
- `Value` (T, get/set) — Gets or sets the internal data.

## AnnotationDimensionSettings (class)

Represents annotation dimension settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings.htm)

### Constructors
- `public AnnotationDimensionSettings()` — Initializes a new instance of the AnnotationDimensionSettings class
- `public AnnotationDimensionSettings(AnnotationDimensionSettings.Attributes attributes)` — Initializes a new instance of the AnnotationDimensionSettings class
- `public AnnotationDimensionSettings(DimensionStyle dimStyle)` — Initializes a new instance of the AnnotationDimensionSettings class

### Methods
#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public static explicit operator DimensionStyle (AnnotationDimensionSettings setings)`



**Parameters:**
- `setings` (Grasshopper.Rhinoceros.Annotations.AnnotationDimensionSettings)

**Returns:** `DimensionStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_op_Explicit.htm)

#### `public void GetDimensionStyle(ref DimensionStyle baseDimensionStyle = null)`



**Parameters:**
- `baseDimensionStyle` (DimensionStyle)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_GetDimensionStyle.htm)

#### `public override int GetHashCode()`

(Inherited from ModelData.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `public static implicit operator AnnotationDimensionSettings (AnnotationDimensionSettings.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.AnnotationDimensionSettings.Attributes)

**Returns:** `AnnotationDimensionSettings` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_op_Implicit.htm)

#### `public AnnotationDimensionSettings.Attributes ToAttributes()`



**Returns:** `AnnotationDimensionSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

### Properties
- `BaselineSpacing` (Nullable<Double>, get) — 
- `CentermarkSize` (Nullable<Double>, get) — 
- `CentermarkStyle` (Nullable<CenterMarkStyle>, get) — 
- `DecimalSeparator` (Nullable<Char>, get) — 
- `DimensionLineExtension` (Nullable<Double>, get) — 
- `ExtensionLineExtension` (Nullable<Double>, get) — 
- `ExtensionLineOffset` (Nullable<Double>, get) — 
- `FixedExtension` (Nullable<Boolean>, get) — 
- `FixedExtensionLength` (Nullable<Double>, get) — 
- `RadialTextAngleType` (Nullable<LeaderContentAngleStyle>, get) — 
- `RadialTextLocation` (Nullable<TextLocation>, get) — 
- `RadialTextOrientation` (Nullable<TextOrientation>, get) — 
- `SuppressExtension1` (Nullable<Boolean>, get) — 
- `SuppressExtension2` (Nullable<Boolean>, get) — 
- `TextAngleType` (Nullable<LeaderContentAngleStyle>, get) — 
- `TextLocation` (Nullable<TextLocation>, get) — 
- `TextOrientation` (Nullable<TextOrientation>, get) — 

## AnnotationDimensionSettings.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.AnnotationDimensionSettings.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the AnnotationDimensionSettings.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes_Equals.htm)

#### `protected bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes_Equals_1.htm)

#### `public static AnnotationDimensionSettings.Attributes From(DimensionStyle dimStyle)`



**Parameters:**
- `dimStyle` (DimensionStyle)

**Returns:** `AnnotationDimensionSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes_GetHashCode.htm)

#### `protected int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes_GetHashCode_1.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationDimensionSettings_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToString.htm)

### Properties
- `BaselineSpacing` (Nullable<Double>, get/set) — 
- `CentermarkSize` (Nullable<Double>, get/set) — 
- `CentermarkStyle` (Nullable<CenterMarkStyle>, get/set) — 
- `DecimalSeparator` (Nullable<Char>, get/set) — 
- `DimensionLineExtension` (Nullable<Double>, get/set) — 
- `ExtensionLineExtension` (Nullable<Double>, get/set) — 
- `ExtensionLineOffset` (Nullable<Double>, get/set) — 
- `FixedExtension` (Nullable<Boolean>, get/set) — 
- `FixedExtensionLength` (Nullable<Double>, get/set) — 
- `RadialTextAngleType` (Nullable<LeaderContentAngleStyle>, get/set) — 
- `RadialTextLocation` (Nullable<TextLocation>, get/set) — 
- `RadialTextOrientation` (Nullable<TextOrientation>, get/set) — 
- `SuppressExtension1` (Nullable<Boolean>, get/set) — 
- `SuppressExtension2` (Nullable<Boolean>, get/set) — 
- `TextAngleType` (Nullable<LeaderContentAngleStyle>, get/set) — 
- `TextLocation` (Nullable<TextLocation>, get/set) — 
- `TextOrientation` (Nullable<TextOrientation>, get/set) — 

## AnnotationLeaderSettings (class)

Represents annotation leader settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings.htm)

### Constructors
- `public AnnotationLeaderSettings()` — Initializes a new instance of the AnnotationLeaderSettings class
- `public AnnotationLeaderSettings(AnnotationLeaderSettings.Attributes attributes)` — Initializes a new instance of the AnnotationLeaderSettings class
- `public AnnotationLeaderSettings(DimensionStyle dimStyle)` — Initializes a new instance of the AnnotationLeaderSettings class

### Methods
#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public static explicit operator DimensionStyle (AnnotationLeaderSettings settings)`



**Parameters:**
- `settings` (Grasshopper.Rhinoceros.Annotations.AnnotationLeaderSettings)

**Returns:** `DimensionStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_op_Explicit.htm)

#### `public void GetDimensionStyle(ref DimensionStyle baseDimensionStyle = null)`



**Parameters:**
- `baseDimensionStyle` (DimensionStyle)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_GetDimensionStyle.htm)

#### `public override int GetHashCode()`

(Inherited from ModelData.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `public static implicit operator AnnotationLeaderSettings (AnnotationLeaderSettings.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.AnnotationLeaderSettings.Attributes)

**Returns:** `AnnotationLeaderSettings` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_op_Implicit.htm)

#### `public AnnotationLeaderSettings.Attributes ToAttributes()`



**Returns:** `AnnotationLeaderSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

### Properties
- `ContentAngleType` (Nullable<LeaderContentAngleStyle>, get) — 
- `CurveType` (Nullable<LeaderCurveStyle>, get) — 
- `HasLanding` (Nullable<Boolean>, get) — 
- `LandingLength` (Nullable<Double>, get) — 
- `TextHorizontalAlignment` (Nullable<TextHorizontalAlignment>, get) — 
- `TextOrientation` (Nullable<TextOrientation>, get) — 
- `TextVerticalAlignment` (Nullable<TextVerticalAlignment>, get) — 

## AnnotationLeaderSettings.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.AnnotationLeaderSettings.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the AnnotationLeaderSettings.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes_Equals.htm)

#### `protected bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes_Equals_1.htm)

#### `public static AnnotationLeaderSettings.Attributes From(DimensionStyle dimStyle)`



**Parameters:**
- `dimStyle` (DimensionStyle)

**Returns:** `AnnotationLeaderSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes_GetHashCode.htm)

#### `protected int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes_GetHashCode_1.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationLeaderSettings_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToString.htm)

### Properties
- `ContentAngleType` (Nullable<LeaderContentAngleStyle>, get/set) — 
- `CurveType` (Nullable<LeaderCurveStyle>, get/set) — 
- `HasLanding` (Nullable<Boolean>, get/set) — 
- `LandingLength` (Nullable<Double>, get/set) — 
- `TextHorizontalAlignment` (Nullable<TextHorizontalAlignment>, get/set) — 
- `TextOrientation` (Nullable<TextOrientation>, get/set) — 
- `TextVerticalAlignment` (Nullable<TextVerticalAlignment>, get/set) — 

## AnnotationTextSettings (class)

Represents model text settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings.htm)

### Constructors
- `public AnnotationTextSettings()` — Initializes a new instance of the AnnotationTextSettings class
- `public AnnotationTextSettings(AnnotationTextSettings.Attributes attributes)` — Initializes a new instance of the AnnotationTextSettings class
- `public AnnotationTextSettings(DimensionStyle dimStyle)` — Initializes a new instance of the AnnotationTextSettings class

### Methods
#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public static explicit operator DimensionStyle (AnnotationTextSettings settings)`



**Parameters:**
- `settings` (Grasshopper.Rhinoceros.Annotations.AnnotationTextSettings)

**Returns:** `DimensionStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_op_Explicit.htm)

#### `public void GetDimensionStyle(ref DimensionStyle baseDimensionStyle = null)`



**Parameters:**
- `baseDimensionStyle` (DimensionStyle)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_GetDimensionStyle.htm)

#### `public override int GetHashCode()`

(Inherited from ModelData.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `public static implicit operator AnnotationTextSettings (AnnotationTextSettings.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.AnnotationTextSettings.Attributes)

**Returns:** `AnnotationTextSettings` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_op_Implicit.htm)

#### `public AnnotationTextSettings.Attributes ToAttributes()`



**Returns:** `AnnotationTextSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

### Properties
- `DrawForward` (Nullable<Boolean>, get) — 
- `DrawTextMask` (Nullable<Boolean>, get) — 
- `FitText` (Nullable<TextFit>, get) — 
- `Font` (Font, get) — 
- `MaskColor` (Nullable<Color>, get) — 
- `MaskColorSource` (Nullable<MaskType>, get) — 
- `MaskFrame` (Nullable<MaskFrame>, get) — 
- `MaskOffset` (Nullable<Double>, get) — 
- `TextGap` (Nullable<Double>, get) — 
- `TextHeight` (Nullable<Double>, get) — 
- `TextHorizontalAlignment` (Nullable<TextHorizontalAlignment>, get) — 
- `TextOrientation` (Nullable<TextOrientation>, get) — 
- `TextVerticalAlignment` (Nullable<TextVerticalAlignment>, get) — 

## AnnotationTextSettings.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.AnnotationTextSettings.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the AnnotationTextSettings.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes_Equals.htm)

#### `protected bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes_Equals_1.htm)

#### `public static AnnotationTextSettings.Attributes From(DimensionStyle dimStyle)`



**Parameters:**
- `dimStyle` (DimensionStyle)

**Returns:** `AnnotationTextSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes_GetHashCode.htm)

#### `protected int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes_GetHashCode_1.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationTextSettings_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToString.htm)

### Properties
- `DrawForward` (Nullable<Boolean>, get/set) — 
- `DrawTextMask` (Nullable<Boolean>, get/set) — 
- `FitText` (Nullable<TextFit>, get/set) — 
- `Font` (Font, get/set) — 
- `MaskColor` (Nullable<Color>, get/set) — 
- `MaskColorSource` (Nullable<MaskType>, get/set) — 
- `MaskFrame` (Nullable<MaskFrame>, get/set) — 
- `MaskOffset` (Nullable<Double>, get/set) — 
- `TextGap` (Nullable<Double>, get/set) — 
- `TextHeight` (Nullable<Double>, get/set) — 
- `TextHorizontalAlignment` (Nullable<TextHorizontalAlignment>, get/set) — 
- `TextOrientation` (Nullable<TextOrientation>, get/set) — 
- `TextVerticalAlignment` (Nullable<TextVerticalAlignment>, get/set) — 

## AnnotationToleranceSettings (class)

Represents annotation tolerance settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings.htm)

### Constructors
- `public AnnotationToleranceSettings()` — Initializes a new instance of the AnnotationToleranceSettings class
- `public AnnotationToleranceSettings(AnnotationToleranceSettings.Attributes attributes)` — Initializes a new instance of the AnnotationToleranceSettings class
- `public AnnotationToleranceSettings(DimensionStyle dimStyle)` — Initializes a new instance of the AnnotationToleranceSettings class

### Methods
#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public static explicit operator DimensionStyle (AnnotationToleranceSettings settings)`



**Parameters:**
- `settings` (Grasshopper.Rhinoceros.Annotations.AnnotationToleranceSettings)

**Returns:** `DimensionStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_op_Explicit.htm)

#### `public void GetDimensionStyle(ref DimensionStyle baseDimensionStyle = null)`



**Parameters:**
- `baseDimensionStyle` (DimensionStyle)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_GetDimensionStyle.htm)

#### `public override int GetHashCode()`

(Inherited from ModelData.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `public static implicit operator AnnotationToleranceSettings (AnnotationToleranceSettings.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.AnnotationToleranceSettings.Attributes)

**Returns:** `AnnotationToleranceSettings` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_op_Implicit.htm)

#### `public AnnotationToleranceSettings.Attributes ToAttributes()`



**Returns:** `AnnotationToleranceSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

### Properties
- `AlternateToleranceResolution` (Nullable<Int32>, get) — 
- `ToleranceFormat` (Nullable<ToleranceDisplayFormat>, get) — 
- `ToleranceHeightScale` (Nullable<Double>, get) — 
- `ToleranceLowerValue` (Nullable<Double>, get) — 
- `ToleranceResolution` (Nullable<Int32>, get) — 
- `ToleranceUpperValue` (Nullable<Double>, get) — 

## AnnotationToleranceSettings.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.AnnotationToleranceSettings.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the AnnotationToleranceSettings.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes_Equals.htm)

#### `protected bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes_Equals_1.htm)

#### `public static AnnotationToleranceSettings.Attributes From(DimensionStyle dimStyle)`



**Parameters:**
- `dimStyle` (DimensionStyle)

**Returns:** `AnnotationToleranceSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes_GetHashCode.htm)

#### `protected int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes_GetHashCode_1.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationToleranceSettings_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToString.htm)

### Properties
- `AlternateToleranceResolution` (Nullable<Int32>, get/set) — 
- `ToleranceFormat` (Nullable<ToleranceDisplayFormat>, get/set) — 
- `ToleranceHeightScale` (Nullable<Double>, get/set) — 
- `ToleranceLowerValue` (Nullable<Double>, get/set) — 
- `ToleranceResolution` (Nullable<Int32>, get/set) — 
- `ToleranceUpperValue` (Nullable<Double>, get/set) — 

## AnnotationUnitsSettings (class)

Represents annotation units settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings.htm)

### Constructors
- `public AnnotationUnitsSettings()` — Initializes a new instance of the AnnotationUnitsSettings class
- `public AnnotationUnitsSettings(AnnotationUnitsSettings.Attributes attributes)` — Initializes a new instance of the AnnotationUnitsSettings class
- `public AnnotationUnitsSettings(DimensionStyle dimStyle)` — Initializes a new instance of the AnnotationUnitsSettings class

### Methods
#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelData.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Inherited from ModelData.)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public static explicit operator DimensionStyle (AnnotationUnitsSettings settings)`



**Parameters:**
- `settings` (Grasshopper.Rhinoceros.Annotations.AnnotationUnitsSettings)

**Returns:** `DimensionStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_op_Explicit.htm)

#### `public void GetDimensionStyle(ref DimensionStyle baseDimensionStyle = null)`



**Parameters:**
- `baseDimensionStyle` (DimensionStyle)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_GetDimensionStyle.htm)

#### `public override int GetHashCode()`

(Inherited from ModelData.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `public static implicit operator AnnotationUnitsSettings (AnnotationUnitsSettings.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.AnnotationUnitsSettings.Attributes)

**Returns:** `AnnotationUnitsSettings` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_op_Implicit.htm)

#### `public AnnotationUnitsSettings.Attributes ToAttributes()`



**Returns:** `AnnotationUnitsSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

### Properties
- `AngleFormat` (Nullable<AngleDisplayFormat>, get) — 
- `AngleResolution` (Nullable<Int32>, get) — 
- `AngleRoundoff` (Nullable<Double>, get) — 
- `AngleZeroSuppression` (Nullable<ZeroSuppression>, get) — 
- `LengthDisplay` (Nullable<LengthDisplay>, get) — 
- `LengthFactor` (Nullable<Double>, get) — 
- `LengthPrefix` (String, get) — 
- `LengthResolution` (Nullable<Int32>, get) — 
- `LengthRoundoff` (Nullable<Double>, get) — 
- `LengthSuffix` (String, get) — 
- `LengthZeroSuppression` (Nullable<ZeroSuppression>, get) — 
- `StackFractionFormat` (Nullable<StackDisplayFormat>, get) — 
- `StackHeightScale` (Nullable<Double>, get) — 

## AnnotationUnitsSettings.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.AnnotationUnitsSettings.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the AnnotationUnitsSettings.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes_Equals.htm)

#### `protected bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes_Equals_1.htm)

#### `public static AnnotationUnitsSettings.Attributes From(DimensionStyle dimStyle)`



**Parameters:**
- `dimStyle` (DimensionStyle)

**Returns:** `AnnotationUnitsSettings.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes_From.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes_GetHashCode.htm)

#### `protected int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes_GetHashCode_1.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_AnnotationUnitsSettings_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToString.htm)

### Properties
- `AngleFormat` (Nullable<AngleDisplayFormat>, get/set) — 
- `AngleResolution` (Nullable<Int32>, get/set) — 
- `AngleRoundoff` (Nullable<Double>, get/set) — 
- `AngleZeroSuppression` (Nullable<ZeroSuppression>, get/set) — 
- `LengthDisplay` (Nullable<LengthDisplay>, get/set) — 
- `LengthFactor` (Nullable<Double>, get/set) — 
- `LengthPrefix` (String, get/set) — 
- `LengthResolution` (Nullable<Int32>, get/set) — 
- `LengthRoundoff` (Nullable<Double>, get/set) — 
- `LengthSuffix` (String, get/set) — 
- `LengthZeroSuppression` (Nullable<ZeroSuppression>, get/set) — 
- `StackFractionFormat` (Nullable<StackDisplayFormat>, get/set) — 
- `StackHeightScale` (Nullable<Double>, get/set) — 

## ModelAnnotationStyle (class)

Represents a Rhino model annotation style. Wraps the functionality of the DimensionStyle type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle.htm)

### Constructors
- `public ModelAnnotationStyle()` — Initializes a new instance of the ModelAnnotationStyle class
- `public ModelAnnotationStyle(ModelAnnotationStyle.Attributes attributes)` — Initializes a new instance of the ModelAnnotationStyle class
- `public ModelAnnotationStyle(DimensionStyle dimStyle)` — Initializes a new instance of the ModelAnnotationStyle class
- `public ModelAnnotationStyle(Guid id)` — Initializes a new instance of the ModelAnnotationStyle class

### Methods
#### `public static ModelAnnotationStyle Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelAnnotationStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_CastTo__1.htm)

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

#### `public static implicit operator ModelAnnotationStyle (DimensionStyle style)`



**Parameters:**
- `style` (DimensionStyle)

**Returns:** `ModelAnnotationStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_op_Implicit_1.htm)

#### `public static implicit operator ModelAnnotationStyle (ModelAnnotationStyle.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Annotations.ModelAnnotationStyle.Attributes)

**Returns:** `ModelAnnotationStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_op_Implicit.htm)

#### `public static implicit operator ModelAnnotationStyle (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelAnnotationStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_op_Implicit_2.htm)

#### `public ModelAnnotationStyle.Attributes ToAttributes()`



**Returns:** `ModelAnnotationStyle.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_ToAttributes.htm)

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
- `ArrowSettings` (AnnotationArrowSettings, get) — 
- `DimensionScale` (Nullable<Double>, get) — 
- `DimensionSettings` (AnnotationDimensionSettings, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `LeaderSettings` (AnnotationLeaderSettings, get) — 
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `TextSettings` (AnnotationTextSettings, get) — 
- `ToleranceSettings` (AnnotationToleranceSettings, get) — 
- `UnitsSettings` (AnnotationUnitsSettings, get) — 
- `Unset` (ModelAnnotationStyle, get) — 
- `UserText` (ModelUserText, get) — (Inherited from ModelComponentContent.)

## ModelAnnotationStyle.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Annotations.ModelAnnotationStyle.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelAnnotationStyle.Attributes class

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

#### `public static implicit operator ModelAnnotationStyle.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelAnnotationStyle.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Overrides ModelComponentContent.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Annotations_ModelAnnotationStyle_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `ArrowSettings` (AnnotationArrowSettings, get/set) — 
- `DimensionScale` (Nullable<Double>, get/set) — 
- `DimensionSettings` (AnnotationDimensionSettings, get/set) — 
- `LeaderSettings` (AnnotationLeaderSettings, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `TextSettings` (AnnotationTextSettings, get/set) — 
- `ToleranceSettings` (AnnotationToleranceSettings, get/set) — 
- `UnitsSettings` (AnnotationUnitsSettings, get/set) — 
- `UserText` (ModelUserText, get/set) — (Inherited from ModelComponentContent.Attributes.)

