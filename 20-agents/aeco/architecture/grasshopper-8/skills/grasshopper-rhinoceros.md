---
name: grasshopper-grasshopper-rhinoceros
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Rhinoceros namespace — 16 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelComponentContent, ModelComponentContent.Attributes, ModelContent, ModelContent.Attributes, ModelData, ModelContentEnablerLoader, ModelData.Attributes, ModelUnitSystem, and 8 more types.
---

# Grasshopper.Rhinoceros

Auto-generated from vendor docs for grasshopper 8.0. 16 types in this namespace.

## ModelColor (struct)

Represents a color stored on a model.

**Remarks:** This is an opaque representation of a color with conversion methods to and from other color types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelColor.htm)

### Methods
#### `public int CompareTo(ModelColor other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_CompareTo.htm)

#### `public static bool operator ==(Color left, ModelColor right)`



**Parameters:**
- `left` (System.Drawing.Color)
- `right` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Equality_2.htm)

#### `public static bool operator ==(ModelColor left, Color right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelColor)
- `right` (System.Drawing.Color)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Equality_1.htm)

#### `public static bool operator ==(ModelColor left, ModelColor right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelColor)
- `right` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Equality.htm)

#### `public bool Equals(ModelColor other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_Equals_1.htm)

#### `public static explicit operator ColorRGBA (ModelColor color)`



**Parameters:**
- `color` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `ColorRGBA` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Explicit.htm)

#### `public static ModelColor FromArgb(byte r, byte g, byte b)`



**Parameters:**
- `r` (System.Byte)
- `g` (System.Byte)
- `b` (System.Byte)

**Returns:** `ModelColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_FromArgb.htm)

#### `public static ModelColor FromArgb(byte a, byte r, byte g, byte b)`



**Parameters:**
- `a` (System.Byte)
- `r` (System.Byte)
- `g` (System.Byte)
- `b` (System.Byte)

**Returns:** `ModelColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_FromArgb_1.htm)

#### `public static ModelColor FromArgb(int argb)`



**Parameters:**
- `argb` (System.Int32)

**Returns:** `ModelColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_FromArgb_2.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_GetHashCode.htm)

#### `public static implicit operator ModelColor (Color color)`



**Parameters:**
- `color` (System.Drawing.Color)

**Returns:** `ModelColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Implicit_2.htm)

#### `public static implicit operator ModelColor (ColorRGBA color)`



**Parameters:**
- `color` (ColorRGBA)

**Returns:** `ModelColor` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Implicit_1.htm)

#### `public static implicit operator Color (ModelColor color)`



**Parameters:**
- `color` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `Color` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Implicit.htm)

#### `public static bool operator !=(Color left, ModelColor right)`



**Parameters:**
- `left` (System.Drawing.Color)
- `right` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Inequality_2.htm)

#### `public static bool operator !=(ModelColor left, Color right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelColor)
- `right` (System.Drawing.Color)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Inequality_1.htm)

#### `public static bool operator !=(ModelColor left, ModelColor right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelColor)
- `right` (Grasshopper.Rhinoceros.ModelColor)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_op_Inequality.htm)

#### `public int ToArgb()`



**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_ToArgb.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelColor_ToString.htm)

## ModelComponentContent (class)

Represents a Rhino model component. Wraps the functionality of the ModelComponent type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelComponentContent.htm)

### Methods
#### `protected internal ModelContent AsFrozen(bool deep = true)`

Try to obtain a dereferenced snapshot of itself.

**Parameters:**
- `deep` (System.Boolean) — Whether or not referenced content should also be frozen.

**Returns:** `ModelContent` — Itself if it is already dereferenced.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_AsFrozen.htm)

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

#### `public ModelComponentContent.Attributes ToAttributes()`



**Returns:** `ModelComponentContent.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelComponentContent_ToAttributes.htm)

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
- `Attribs` (ModelComponentContent.Attributes, get/set) — 
- `DefaultAttributes` (ModelData.Attributes, get) — (Inherited from ModelData.)
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsReferencedData` (Boolean, get) — (Inherited from ModelContent.)
- `IsReferencedDataLoaded` (Boolean, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `IsValidWhyNot` (String, get) — (Inherited from ModelContent.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `TypeDescription` (String, get) — (Inherited from ModelData.)
- `UserText` (ModelUserText, get) — 
- `Serial` (Int32, get) — (Inherited from ModelData.)

## ModelComponentContent.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.ModelComponentContent.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelComponentContent_Attributes.htm)

### Constructors
- `protected Attributes()` — Initializes a new instance of the ModelComponentContent.Attributes class

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelComponentContent_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.Attributes.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `protected override int GetHashCode(IEqualityComparer comparer)`

(Overrides ModelContent.Attributes.GetHashCode(IEqualityComparer).)

**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelComponentContent_Attributes_GetHashCode.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelComponentContent_Attributes_ReadData.htm)

#### `protected virtual ModelData.Attributes SubClone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_SubClone.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelComponentContent_Attributes_ToDetails.htm)

#### `public abstract ModelData ToModelData()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToModelData.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelComponentContent_Attributes_WriteData.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `UserText` (ModelUserText, get/set) — 

## ModelContent (class)

Represents a Rhino model element.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelContent.htm)

### Methods
#### `protected internal ModelContent AsFrozen(bool deep = true)`

Try to obtain a dereferenced snapshot of itself.

**Parameters:**
- `deep` (System.Boolean) — Whether or not referenced content should also be frozen.

**Returns:** `ModelContent` — Itself if it is already dereferenced.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_AsFrozen.htm)

#### `public static ModelContent Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelContent` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Cast.htm)

#### `protected internal static T Cast<A, T>(Object source) where A : new(), ModelContent.Attributes where T : ModelContent`



**Parameters:**
- `source` (System.Object)

**Returns:** `T` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Cast__2.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelData.CastTo<T>(T).)

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

(Overrides ModelData.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Equals_1.htm)

#### `public static ModelContent FromId(RhinoDoc document, Guid guid)`



**Parameters:**
- `document` (RhinoDoc)
- `guid` (System.Guid)

**Returns:** `ModelContent` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_FromId.htm)

#### `public override int GetHashCode()`

(Overrides ModelData.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `protected bool InternaliseData()`



**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_InternaliseData.htm)

#### `protected virtual bool LoadReferencedData()`



**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_LoadReferencedData.htm)

#### `protected override bool Read(GH_IReader reader)`

(Overrides ModelData.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Read.htm)

#### `protected internal Guid? RuntimeId(RhinoDoc document)`



**Parameters:**
- `document` (RhinoDoc)

**Returns:** `Nullable<Guid>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_RuntimeId.htm)

#### `public ModelContent.Attributes ToAttributes()`



**Returns:** `ModelContent.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Overrides ModelData.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_ToString.htm)

#### `public override string TooltipString()`

(Overrides ModelData.TooltipString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_TooltipString.htm)

#### `protected virtual void UnloadReferencedData()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_UnloadReferencedData.htm)

#### `protected override bool Write(GH_IWriter writer)`

(Overrides ModelData.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Write.htm)

### Properties
- `Attribs` (ModelContent.Attributes, get/set) — 
- `DefaultAttributes` (ModelData.Attributes, get) — (Inherited from ModelData.)
- `Id` (Nullable<Guid>, get) — 
- `IsReferencedData` (Boolean, get) — 
- `IsReferencedDataLoaded` (Boolean, get) — 
- `IsValid` (Boolean, get) — (Overrides ModelData.IsValid.)
- `IsValidWhyNot` (String, get) — (Overrides ModelData.IsValidWhyNot.)
- `Name` (ModelContentName, get) — 
- `Notes` (String, get) — 
- `Parent` (ModelContentName, get) — 
- `Path` (ModelContentName, get) — 
- `Tags` (ModelTags, get) — 
- `TypeDescription` (String, get) — (Inherited from ModelData.)
- `Serial` (Int32, get) — (Inherited from ModelData.)

## ModelContent.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.ModelContent.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelContent_Attributes.htm)

### Constructors
- `protected Attributes()` — Initializes a new instance of the ModelContent.Attributes class

### Methods
#### `protected internal virtual ModelContent.Attributes AsFrozen()`



**Returns:** `ModelContent.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_AsFrozen.htm)

#### `public ModelData.Attributes Clone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `protected internal static A CloneOnWrite<A>(ref bool cloned, ref A attributes) where A : ModelContent.Attributes`



**Parameters:**
- `cloned` (System.Boolean)
- `attributes` (A)

**Returns:** `A` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_CloneOnWrite__1.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Equals.htm)

#### `protected virtual bool Equals(Object other, IEqualityComparer comparer)`



**Parameters:**
- `other` (System.Object)
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Equals_1.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `protected virtual int GetHashCode(IEqualityComparer comparer)`



**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode_1.htm)

#### `protected internal virtual bool IsValidContentName(ModelContentName value, bool? parent)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.ModelContentName)
- `parent` (System.Nullable<Boolean>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_IsValidContentName.htm)

#### `protected internal override sealed bool Read(GH_IReader reader)`

(Overrides ModelData.Attributes.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Read.htm)

#### `protected virtual bool ReadData(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ReadData.htm)

#### `protected virtual ModelData.Attributes SubClone()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_SubClone.htm)

#### `public virtual string ToDetails()`

(Inherited from ModelData.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToDetails.htm)

#### `public abstract ModelData ToModelData()`

(Inherited from ModelData.Attributes.)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelData.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

#### `protected internal override sealed bool Write(GH_IWriter writer)`

(Overrides ModelData.Attributes.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_Write.htm)

#### `protected virtual bool WriteData(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_WriteData.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — 
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — 

## ModelContentEnablerLoader (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.ModelContentEnablerLoader.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelContentEnablerLoader.htm)

### Constructors
- `public ModelContentEnablerLoader()` — Initializes a new instance of the ModelContentEnablerLoader class

### Methods
#### `public override GH_LoadingInstruction PriorityLoad()`

(Overrides GH_AssemblyPriority.PriorityLoad().)

**Returns:** `GH_LoadingInstruction` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentEnablerLoader_PriorityLoad.htm)

## ModelContentName (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.ModelContentName.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelContentName.htm)

### Methods
#### `public static ModelContentName Combine(params ModelContentName[] paths)`



**Parameters:**
- `paths` (Grasshopper.Rhinoceros.ModelContentName[])

**Returns:** `ModelContentName` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_Combine.htm)

#### `public int CompareTo(ModelContentName other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_CompareTo.htm)

#### `public int CompareTo(Object obj)`



**Parameters:**
- `obj` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_CompareTo_1.htm)

#### `public static ModelContentName operator /(ModelContentName left, ModelContentName right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelContentName)
- `right` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `ModelContentName` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Division.htm)

#### `public static bool operator ==(ModelContentName left, ModelContentName right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelContentName)
- `right` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Equality.htm)

#### `public static bool operator ==(ModelContentName left, string right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelContentName)
- `right` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Equality_1.htm)

#### `public static bool operator ==(string left, ModelContentName right)`



**Parameters:**
- `left` (System.String)
- `right` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Equality_2.htm)

#### `public bool Equals(ModelContentName other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_Equals.htm)

#### `public override bool Equals(Object obj)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `obj` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_Equals_1.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_GetHashCode.htm)

#### `public static implicit operator string (ModelContentName value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Implicit.htm)

#### `public static implicit operator ModelContentName (string value)`



**Parameters:**
- `value` (System.String)

**Returns:** `ModelContentName` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Implicit_1.htm)

#### `public static bool operator !=(ModelContentName left, ModelContentName right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelContentName)
- `right` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Inequality.htm)

#### `public static bool operator !=(ModelContentName left, string right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelContentName)
- `right` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Inequality_1.htm)

#### `public static bool operator !=(string left, ModelContentName right)`



**Parameters:**
- `left` (System.String)
- `right` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_Inequality_2.htm)

#### `public static ModelContentName operator ~(ModelContentName value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.ModelContentName)

**Returns:** `ModelContentName` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_op_OnesComplement.htm)

#### `public string[] Split()`



**Returns:** `String[]` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_Split.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContentName_ToString.htm)

### Properties
- `IsDefault` (Boolean, get) — 
- `IsDefaultOrEmpty` (Boolean, get) — 
- `IsEmpty` (Boolean, get) — 
- `Parent` (ModelContentName, get) — 
- `Separator` (String, get) — 
- `Stem` (ModelContentName, get) — 
- `Comparer` (StringComparer, get) — 
- `Empty` (ModelContentName, get) — 

## ModelData (class)

Represents a Rhino model chunk of data.

**Remarks:** This type is the base type of a family of immutable types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelData.htm)

### Methods
#### `protected static T Cast<A, T>(Object source) where A : ModelData.Attributes where T : ModelData`



**Parameters:**
- `source` (System.Object)

**Returns:** `T` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Cast__2.htm)

#### `public virtual bool CastTo<T>(out T target)`



**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_CastTo__1.htm)

#### `public bool Equals(ModelData other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelData)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Equals_1.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_GetHashCode.htm)

#### `protected virtual bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Read.htm)

#### `public ModelData.Attributes ToAttributes()`



**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToAttributes.htm)

#### `public string ToDetails()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToString.htm)

#### `public virtual string TooltipString()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_TooltipString.htm)

#### `protected virtual bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Write.htm)

### Properties
- `Attribs` (ModelData.Attributes, get/set) — 
- `DefaultAttributes` (ModelData.Attributes, get) — 
- `IsValidWhyNot` (String, get) — 
- `TypeDescription` (String, get) — 
- `Serial` (Int32, get) — 

## ModelData.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.ModelData.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelData_Attributes.htm)

### Constructors
- `protected Attributes()` — Initializes a new instance of the ModelData.Attributes class

### Methods
#### `public ModelData.Attributes Clone()`



**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Clone.htm)

#### `protected internal virtual bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Read.htm)

#### `protected virtual ModelData.Attributes SubClone()`



**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_SubClone.htm)

#### `public virtual string ToDetails()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToDetails.htm)

#### `public abstract ModelData ToModelData()`



**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_ToString.htm)

#### `protected internal virtual bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_Attributes_Write.htm)

## ModelFont (class)

Represents a typography font. Wraps the functionality of the Font type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelFont.htm)

### Constructors
- `public ModelFont()` — Initializes a new instance of the ModelFont class
- `public ModelFont(Font font)` — Initializes a new instance of the ModelFont class
- `public ModelFont(string familyName)` — Initializes a new instance of the ModelFont class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_Duplicate.htm)

#### `public virtual IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_EmitProxy.htm)

#### `public bool Equals(ModelFont other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelFont)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_Equals_1.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_GetHashCode.htm)

#### `public static implicit operator ModelFont (Font font)`



**Parameters:**
- `font` (Font)

**Returns:** `ModelFont` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_op_Implicit.htm)

#### `public static implicit operator ModelFont (string familyName)`



**Parameters:**
- `familyName` (System.String)

**Returns:** `ModelFont` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_op_Implicit_1.htm)

#### `public static bool IsFamilyNameInstalled(string familyName)`



**Parameters:**
- `familyName` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_IsFamilyNameInstalled.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelFont_Write.htm)

### Properties
- `FamilyName` (String, get) — 
- `InstalledFamilies` (IReadOnlyList<String>, get) — 
- `InstalledFonts` (IReadOnlyList<ModelFont>, get) — 
- `IsInstalled` (Boolean, get) — 
- `IsValid` (Boolean, get) — (Overrides GH_Goo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.

## ModelMeshingParameters (class)

Represents some meshing parameters. Wraps the functionality of the MeshingParameters type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelMeshingParameters.htm)

### Constructors
- `public ModelMeshingParameters()` — Initializes a new instance of the ModelMeshingParameters class

### Methods
#### `public static ModelMeshingParameters Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelMeshingParameters` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelMeshingParameters_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelMeshingParameters_CastTo__1.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelMeshingParameters_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelMeshingParameters_GetHashCode.htm)

#### `public static implicit operator ModelMeshingParameters (MeshingParameters parameters)`



**Parameters:**
- `parameters` (MeshingParameters)

**Returns:** `ModelMeshingParameters` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelMeshingParameters_op_Implicit.htm)

#### `public MeshingParameters ToMeshingParameters(RhinoDoc document = null)`



**Parameters:**
- `document` (RhinoDoc)

**Returns:** `MeshingParameters` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelMeshingParameters_ToMeshingParameters.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `DefaultAnalysis` (ModelMeshingParameters, get) — 
- `DefaultRender` (ModelMeshingParameters, get) — 
- `FastRender` (ModelMeshingParameters, get) — 
- `QualityRender` (ModelMeshingParameters, get) — 
- `Unset` (ModelMeshingParameters, get) — 

## ModelTags (struct)

Represents an immutable set of strings.

**Remarks:** Comparison is not case sensitive.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelTags.htm)

### Constructors
- `public ModelTags(IEnumerable<string> source)` — Initializes a new instance of the ModelTags class

### Methods
#### `public ModelTags AddRange(IEnumerable<string> source)`



**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelTags` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_AddRange.htm)

#### `public static ModelTags operator +(ModelTags left, IEnumerable<string> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelTags)
- `right` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelTags` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_op_Addition.htm)

#### `public int CompareTo(ModelTags other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelTags)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_CompareTo.htm)

#### `public int CompareTo(Object obj)`



**Parameters:**
- `obj` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_CompareTo_1.htm)

#### `public bool Contains(string value)`



**Parameters:**
- `value` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_Contains.htm)

#### `public static bool operator ==(ModelTags left, ModelTags right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelTags)
- `right` (Grasshopper.Rhinoceros.ModelTags)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_op_Equality.htm)

#### `public bool Equals(ModelTags other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelTags)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_Equals.htm)

#### `public override bool Equals(Object obj)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `obj` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_Equals_1.htm)

#### `public IEnumerator<string> GetEnumerator()`



**Returns:** `IEnumerator<String>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_GetEnumerator.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_GetHashCode.htm)

#### `public static implicit operator string (ModelTags value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.ModelTags)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_op_Implicit.htm)

#### `public static implicit operator ModelTags (string value)`



**Parameters:**
- `value` (System.String)

**Returns:** `ModelTags` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_op_Implicit_1.htm)

#### `public static bool operator !=(ModelTags left, ModelTags right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelTags)
- `right` (Grasshopper.Rhinoceros.ModelTags)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_op_Inequality.htm)

#### `public ModelTags RemoveRange(IEnumerable<string> source)`



**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelTags` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_RemoveRange.htm)

#### `public static ModelTags operator -(ModelTags left, IEnumerable<string> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelTags)
- `right` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelTags` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_op_Subtraction.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelTags_ToString.htm)

### Properties
- `Count` (Int32, get) — 
- `IsDefault` (Boolean, get) — 
- `IsDefaultOrEmpty` (Boolean, get) — 
- `IsEmpty` (Boolean, get) — 
- `Empty` (ModelTags, get) — 

## ModelUnitSystem (class)

Represents a Rhino Unit System. Wraps the functionality of the UnitSystem type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelUnitSystem.htm)

### Constructors
- `public ModelUnitSystem()` — Initializes a new instance of the ModelUnitSystem class
- `public ModelUnitSystem(ModelUnitSystem.Value attributes)` — Initializes a new instance of the ModelUnitSystem class
- `public ModelUnitSystem(ActiveSpace space, RhinoDoc document)` — Initializes a new instance of the ModelUnitSystem class

### Methods
#### `public void AdjustDocumentUnitSystem(RhinoDoc document, ActiveSpace space, bool scale = false)`



**Parameters:**
- `document` (RhinoDoc)
- `space` (ActiveSpace)
- `scale` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_AdjustDocumentUnitSystem.htm)

#### `public static ModelUnitSystem Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelUnitSystem` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_CastTo__1.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelValue.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_CastTo__1.htm)

#### `public static void Copy(RhinoDoc source, RhinoDoc target, ActiveSpace space)`



**Parameters:**
- `source` (RhinoDoc)
- `target` (RhinoDoc)
- `space` (ActiveSpace)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Copy.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_GetHashCode.htm)

#### `public static implicit operator ModelUnitSystem.Value (ModelUnitSystem us)`



**Parameters:**
- `us` (Grasshopper.Rhinoceros.ModelUnitSystem)

**Returns:** `ModelUnitSystem.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_op_Implicit.htm)

#### `public static implicit operator ModelUnitSystem (ModelUnitSystem.Value attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.ModelUnitSystem.Value)

**Returns:** `ModelUnitSystem` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_op_Implicit_1.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public override string TooltipString()`

(Overrides ModelValue.TooltipString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_TooltipString.htm)

### Properties
- `ActiveSpace` (ActiveSpace, get) — 
- `BaseSystem` (UnitSystem, get) — 
- `Factor` (Double, get) — 
- `IsValid` (Boolean, get) — (Overrides ModelValue.IsValid.)
- `MetersPerUnit` (Double, get) — 
- `Name` (String, get) — 

## ModelUnitSystem.Value (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.ModelUnitSystem.Value.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelUnitSystem_Value.htm)

### Constructors
- `public Value(ActiveSpace activeSpace, string name = null)` — Initializes a new instance of the ModelUnitSystem.Value class
- `public Value(UnitSystem unitSystem, string name = null)` — Initializes a new instance of the ModelUnitSystem.Value class
- `public Value(UnitSystem unitSystem, string name, double factor)` — Initializes a new instance of the ModelUnitSystem.Value class
- `public Value(ModelUnitSystem.Value other, string name = null, double? factor = null)` — Initializes a new instance of the ModelUnitSystem.Value class

### Methods
#### `public void AdjustDocumentUnitSystem(RhinoDoc document, ActiveSpace space, bool scale = false)`



**Parameters:**
- `document` (RhinoDoc)
- `space` (ActiveSpace)
- `scale` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_AdjustDocumentUnitSystem.htm)

#### `public int CompareTo(ModelUnitSystem.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelUnitSystem.Value)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_CompareTo.htm)

#### `public int CompareTo(Object obj)`



**Parameters:**
- `obj` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_CompareTo_1.htm)

#### `public static bool operator ==(ModelUnitSystem.Value left, ModelUnitSystem.Value right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUnitSystem.Value)
- `right` (Grasshopper.Rhinoceros.ModelUnitSystem.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_op_Equality.htm)

#### `public bool Equals(ModelUnitSystem.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelUnitSystem.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_Equals.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_Equals_1.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_GetHashCode.htm)

#### `public static bool operator !=(ModelUnitSystem.Value left, ModelUnitSystem.Value right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUnitSystem.Value)
- `right` (Grasshopper.Rhinoceros.ModelUnitSystem.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_op_Inequality.htm)

#### `public UnitSystem ToNonCustomUnitSystem(out double factor)`



**Parameters:**
- `factor` (System.Double)

**Returns:** `UnitSystem` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_ToNonCustomUnitSystem.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_ToString.htm)

#### `public UnitSystem ToUnitSystem(out double meterPerUnit)`



**Parameters:**
- `meterPerUnit` (System.Double)

**Returns:** `UnitSystem` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUnitSystem_Value_ToUnitSystem.htm)

### Properties
- `ActiveSpace` (ActiveSpace, get) — 
- `BaseSystem` (UnitSystem, get) — 
- `Factor` (Double, get) — 
- `Feet` (ModelUnitSystem.Value, get) — 
- `Inches` (ModelUnitSystem.Value, get) — 
- `Meters` (ModelUnitSystem.Value, get) — 
- `MetersPerUnit` (Double, get) — 
- `Millimeters` (ModelUnitSystem.Value, get) — 
- `ModelUnits` (ModelUnitSystem.Value, get) — 
- `Name` (String, get) — 
- `None` (ModelUnitSystem.Value, get) — 
- `PageUnits` (ModelUnitSystem.Value, get) — 
- `Symbol` (String, get) — 
- `Unset` (ModelUnitSystem.Value, get) — 
- `_Factor` (Double, get) — 

## ModelUserText (struct)

Represents an immutable collection of string/string pairs.

**Remarks:** Key comparison is not case sensitive.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelUserText.htm)

### Constructors
- `public ModelUserText(IEnumerable<KeyValuePair<string, string>> source)` — Initializes a new instance of the ModelUserText class

### Methods
#### `public static ModelUserText operator +(ModelUserText left, IEnumerable<KeyValuePair<string, string>> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_Addition.htm)

#### `public static ModelUserText operator +(ModelUserText left, IEnumerable<string> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_Addition_1.htm)

#### `public static ModelUserText operator &(ModelUserText left, IEnumerable<KeyValuePair<string, string>> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_BitwiseAnd.htm)

#### `public static ModelUserText operator &(ModelUserText left, IEnumerable<string> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_BitwiseAnd_1.htm)

#### `public static ModelUserText operator |(ModelUserText left, IEnumerable<KeyValuePair<string, string>> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_BitwiseOr.htm)

#### `public static ModelUserText operator |(ModelUserText left, IEnumerable<string> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_BitwiseOr_1.htm)

#### `public bool ContainsKey(string key)`



**Parameters:**
- `key` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_ContainsKey.htm)

#### `public ModelUserText EnsureRange(IEnumerable<KeyValuePair<string, string>> source)`



**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_EnsureRange.htm)

#### `public ModelUserText EnsureRange(IEnumerable<string> keys)`



**Parameters:**
- `keys` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_EnsureRange_1.htm)

#### `public static bool operator ==(ModelUserText left, ModelUserText right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (Grasshopper.Rhinoceros.ModelUserText)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_Equality.htm)

#### `public bool Equals(ModelUserText other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.ModelUserText)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_Equals.htm)

#### `public override bool Equals(Object obj)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `obj` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_Equals_1.htm)

#### `public IEnumerator<KeyValuePair<string, string>> GetEnumerator()`



**Returns:** `IEnumerator<KeyValuePair<String,String>>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_GetEnumerator.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_GetHashCode.htm)

#### `public static bool operator !=(ModelUserText left, ModelUserText right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (Grasshopper.Rhinoceros.ModelUserText)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_Inequality.htm)

#### `public ModelUserText MergeRange(IEnumerable<KeyValuePair<string, string>> source)`



**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_MergeRange.htm)

#### `public ModelUserText MergeRange(IEnumerable<string> source)`



**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_MergeRange_1.htm)

#### `public ModelUserText RemoveRange(IEnumerable<KeyValuePair<string, string>> source)`



**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_RemoveRange.htm)

#### `public ModelUserText RemoveRange(IEnumerable<string> keys)`



**Parameters:**
- `keys` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_RemoveRange_1.htm)

#### `public static ModelUserText operator -(ModelUserText left, IEnumerable<KeyValuePair<string, string>> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_Subtraction.htm)

#### `public static ModelUserText operator -(ModelUserText left, IEnumerable<string> right)`



**Parameters:**
- `left` (Grasshopper.Rhinoceros.ModelUserText)
- `right` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_op_Subtraction_1.htm)

#### `public bool TryGetValue(string key, out string value)`



**Parameters:**
- `key` (System.String)
- `value` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_TryGetValue.htm)

#### `public ModelUserText UpdateRange(IEnumerable<KeyValuePair<string, string>> source)`



**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<KeyValuePair<String,String>>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_UpdateRange.htm)

#### `public ModelUserText UpdateRange(IEnumerable<string> keys)`



**Parameters:**
- `keys` (System.Collections.Generic.IEnumerable<String>)

**Returns:** `ModelUserText` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelUserText_UpdateRange_1.htm)

### Properties
- `Count` (Int32, get) — 
- `IsDefault` (Boolean, get) — 
- `IsDefaultOrEmpty` (Boolean, get) — 
- `IsEmpty` (Boolean, get) — 
- `Item[Int32]` (KeyValuePair<String,String>, get) — 
- `Item[String]` (String, get) — 
- `Keys` (IEnumerable<String>, get) — 
- `Values` (IEnumerable<String>, get) — 
- `Empty` (ModelUserText, get) — 

## ModelValue (class)

Represents a value in a Rhino model.

**Remarks:** This type is the base type of a family of immutable types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_ModelValue.htm)

### Methods
#### `public virtual bool CastTo<T>(out T target)`



**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_CastTo__1.htm)

#### `protected virtual Object Clone()`



**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_Clone.htm)

#### `protected internal abstract bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_Read.htm)

#### `protected internal static bool ReadValue<T>(GH_IReader reader, out T serializable) where T : struct, new(), GH_ISerializable`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)
- `serializable` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ReadValue__1.htm)

#### `protected internal abstract Object ScriptVariable()`



**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

#### `protected internal abstract bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_Write.htm)

#### `protected internal static bool WriteValue<T>(GH_IWriter writer, in T serializable) where T : struct, new(), GH_ISerializable`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)
- `serializable` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_WriteValue__1.htm)

### Properties
- `IsValidWhyNot` (String, get) — 
- `TypeDescription` (String, get) — 

