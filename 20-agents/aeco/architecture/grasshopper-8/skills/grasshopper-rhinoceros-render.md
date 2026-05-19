---
name: grasshopper-grasshopper-rhinoceros-render
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Rhinoceros.Render namespace — 16 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelRenderContent, ModelRenderContent.Attributes, ModelRenderEnvironment, ModelRenderEnvironment.Attributes, ModelRenderMaterial, ModelRenderMaterial.Attributes, ModelRenderSkylight, ModelRenderSkylight.Attributes, and 8 more types.
---

# Grasshopper.Rhinoceros.Render

Auto-generated from vendor docs for grasshopper 8.0. 16 types in this namespace.

## ModelRenderContent (class)

Represents a Rhino render content. Wraps the functionality of the RenderContent type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderContent.htm)

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

#### `protected internal static TContent DuplicateContent<TContent>(TContent renderContent, out string name, out string notes, out string tags) where TContent : RenderContent`



**Parameters:**
- `renderContent` (TContent)
- `name` (System.String)
- `notes` (System.String)
- `tags` (System.String)

**Returns:** `TContent` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_DuplicateContent__1.htm)

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

#### `public ModelRenderContent.Attributes ToAttributes()`



**Returns:** `ModelRenderContent.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_ToAttributes.htm)

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
- `Attribs` (ModelRenderContent.Attributes, get/set) — 
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
- `TypeName` (String, get) — (Overrides ModelData.TypeName.)
- `Serial` (Int32, get) — (Inherited from ModelData.)

## ModelRenderContent.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ModelRenderContent.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes.htm)

### Constructors
- `protected internal Attributes()` — Initializes a new instance of the ModelRenderContent.Attributes class
- `protected Attributes(RenderContent content)` — Initializes a new instance of the ModelRenderContent.Attributes class

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.Attributes.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_GetHashCode.htm)

#### `protected override int GetHashCode(IEqualityComparer comparer)`

(Overrides ModelContent.Attributes.GetHashCode(IEqualityComparer).)

**Parameters:**
- `comparer` (System.Collections.IEqualityComparer)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_GetHashCode.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_ReadData.htm)

#### `protected override ModelData.Attributes SubClone()`

(Overrides ModelData.Attributes.SubClone().)

**Returns:** `ModelData.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_SubClone.htm)

#### `public override string ToDetails()`

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_ToDetails.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_WriteData.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `RenderContent` (RenderContent, get/set) — 
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)

## ModelRenderEnvironment (class)

Represents a Rhino render environment. Wraps the functionality of the RenderEnvironment type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment.htm)

### Constructors
- `public ModelRenderEnvironment()` — Initializes a new instance of the ModelRenderEnvironment class
- `public ModelRenderEnvironment(ModelRenderEnvironment.Attributes attributes)` — Initializes a new instance of the ModelRenderEnvironment class
- `public ModelRenderEnvironment(Guid id)` — Initializes a new instance of the ModelRenderEnvironment class
- `public ModelRenderEnvironment(RenderEnvironment renderEnvironment)` — Initializes a new instance of the ModelRenderEnvironment class

### Methods
#### `public static ModelRenderEnvironment Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelRenderEnvironment` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_Cast.htm)

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

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_CastTo__1.htm)

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

#### `public static implicit operator ModelRenderEnvironment (ColorRGBA rgba)`



**Parameters:**
- `rgba` (ColorRGBA)

**Returns:** `ModelRenderEnvironment` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_op_Implicit_1.htm)

#### `public static implicit operator ModelRenderEnvironment (ModelRenderEnvironment.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Render.ModelRenderEnvironment.Attributes)

**Returns:** `ModelRenderEnvironment` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_op_Implicit.htm)

#### `public static implicit operator ModelRenderEnvironment (RenderEnvironment renderEnvironment)`



**Parameters:**
- `renderEnvironment` (RenderEnvironment)

**Returns:** `ModelRenderEnvironment` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_op_Implicit_2.htm)

#### `public static implicit operator ModelRenderEnvironment (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelRenderEnvironment` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_op_Implicit_3.htm)

#### `public ModelRenderEnvironment.Attributes ToAttributes()`



**Returns:** `ModelRenderEnvironment.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_ToAttributes.htm)

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
- `TypeName` (String, get) — (Overrides ModelRenderContent.TypeName.)

## ModelRenderEnvironment.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ModelRenderEnvironment.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelRenderEnvironment.Attributes class

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

#### `public static implicit operator ModelRenderEnvironment.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelRenderEnvironment.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Inherited from ModelRenderContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelContent.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderEnvironment_Attributes_ToString.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `RenderEnvironment` (RenderEnvironment, get/set) — 
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)

## ModelRenderMaterial (class)

Represents a Rhino render material. Wraps the functionality of the RenderMaterial type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderMaterial.htm)

### Constructors
- `public ModelRenderMaterial()` — Initializes a new instance of the ModelRenderMaterial class
- `public ModelRenderMaterial(ModelRenderMaterial.Attributes attributes)` — Initializes a new instance of the ModelRenderMaterial class
- `public ModelRenderMaterial(Guid id)` — Initializes a new instance of the ModelRenderMaterial class
- `public ModelRenderMaterial(Material material)` — Initializes a new instance of the ModelRenderMaterial class
- `public ModelRenderMaterial(RenderMaterial renderMaterial)` — Initializes a new instance of the ModelRenderMaterial class

### Methods
#### `public static ModelRenderMaterial Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_Cast.htm)

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

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_CastTo__1.htm)

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

#### `public static implicit operator ModelRenderMaterial (ColorRGBA rgba)`



**Parameters:**
- `rgba` (ColorRGBA)

**Returns:** `ModelRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_op_Implicit_1.htm)

#### `public static implicit operator ModelRenderMaterial (Material material)`



**Parameters:**
- `material` (Material)

**Returns:** `ModelRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_op_Implicit_2.htm)

#### `public static implicit operator ModelRenderMaterial (ModelRenderMaterial.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Render.ModelRenderMaterial.Attributes)

**Returns:** `ModelRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_op_Implicit.htm)

#### `public static implicit operator ModelRenderMaterial (RenderMaterial renderMaterial)`



**Parameters:**
- `renderMaterial` (RenderMaterial)

**Returns:** `ModelRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_op_Implicit_3.htm)

#### `public static implicit operator ModelRenderMaterial (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_op_Implicit_4.htm)

#### `public ModelRenderMaterial.Attributes ToAttributes()`



**Returns:** `ModelRenderMaterial.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_ToAttributes.htm)

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
- `Name` (ModelContentName, get) — (Overrides ModelContent.Name.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `TypeName` (String, get) — (Overrides ModelRenderContent.TypeName.)
- `Unset` (ModelRenderMaterial, get) — 

## ModelRenderMaterial.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ModelRenderMaterial.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelRenderMaterial.Attributes class

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

#### `public static implicit operator ModelRenderMaterial.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelRenderMaterial.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Inherited from ModelRenderContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelContent.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderMaterial_Attributes_ToString.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `RenderMaterial` (RenderMaterial, get/set) — 
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)

## ModelRenderSkylight (class)

Represents a Rhino skylight. Wraps the functionality of the Skylight type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderSkylight.htm)

### Constructors
- `public ModelRenderSkylight()` — Initializes a new instance of the ModelRenderSkylight class

### Methods
#### `public static ModelRenderSkylight Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelRenderSkylight` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSkylight_Cast.htm)

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

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSkylight_CastTo__1.htm)

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

#### `public static implicit operator ModelRenderSkylight (ModelRenderSkylight.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Render.ModelRenderSkylight.Attributes)

**Returns:** `ModelRenderSkylight` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSkylight_op_Implicit.htm)

#### `public ModelRenderSkylight.Attributes ToAttributes()`



**Returns:** `ModelRenderSkylight.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSkylight_ToAttributes.htm)

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
- `Enabled` (Nullable<Boolean>, get) — 
- `Environment` (ModelRenderEnvironment, get) — 
- `EnvironmentOverride` (Nullable<Boolean>, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)

## ModelRenderSkylight.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ModelRenderSkylight.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderSkylight_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelRenderSkylight.Attributes class

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

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSkylight_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSkylight_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `Enabled` (Nullable<Boolean>, get/set) — 
- `Environment` (ModelRenderEnvironment, get/set) — 
- `EnvironmentOverride` (Nullable<Boolean>, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)

## ModelRenderSun (class)

Represents a Rhino sun. Wraps the functionality of the Sun type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderSun.htm)

### Constructors
- `public ModelRenderSun()` — Initializes a new instance of the ModelRenderSun class

### Methods
#### `public static ModelRenderSun Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelRenderSun` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSun_Cast.htm)

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

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSun_CastTo__1.htm)

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

#### `public static implicit operator ModelRenderSun (ModelRenderSun.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Render.ModelRenderSun.Attributes)

**Returns:** `ModelRenderSun` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSun_op_Implicit.htm)

#### `public ModelRenderSun.Attributes ToAttributes()`



**Returns:** `ModelRenderSun.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSun_ToAttributes.htm)

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
- `Altitude` (Nullable<Double>, get) — 
- `Azimuth` (Nullable<Double>, get) — 
- `DateTime` (Nullable<DateTime>, get) — 
- `DaylightSavingEnabled` (Nullable<Boolean>, get) — 
- `DaylightSavingMinutes` (Nullable<Int32>, get) — 
- `Direction` (Nullable<Vector3d>, get) — 
- `Enabled` (Nullable<Boolean>, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `Intensity` (Nullable<Double>, get) — 
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Latitude` (Nullable<Double>, get) — 
- `Longitude` (Nullable<Double>, get) — 
- `Manual` (Nullable<Boolean>, get) — 
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `North` (Nullable<Double>, get) — 
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `TimeZone` (Nullable<Double>, get) — 

## ModelRenderSun.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ModelRenderSun.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderSun_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelRenderSun.Attributes class

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

(Overrides ModelData.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSun_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSun_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelContent.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderSun_Attributes_ToString.htm)

### Properties
- `Altitude` (Nullable<Double>, get/set) — 
- `Azimuth` (Nullable<Double>, get/set) — 
- `DateTime` (Nullable<DateTime>, get/set) — 
- `DaylightSavingEnabled` (Nullable<Boolean>, get/set) — 
- `DaylightSavingMinutes` (Nullable<Int32>, get/set) — 
- `Direction` (Nullable<Vector3d>, get/set) — 
- `Enabled` (Nullable<Boolean>, get/set) — 
- `Intensity` (Nullable<Double>, get/set) — 
- `Latitude` (Nullable<Double>, get/set) — 
- `Longitude` (Nullable<Double>, get/set) — 
- `Manual` (Nullable<Boolean>, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `North` (Nullable<Double>, get/set) — 
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `TimeZone` (Nullable<Double>, get/set) — 

## ModelRenderTexture (class)

Represents a Rhino render texture. Wraps the functionality of the RenderTexture type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderTexture.htm)

### Constructors
- `public ModelRenderTexture()` — Initializes a new instance of the ModelRenderTexture class
- `public ModelRenderTexture(ModelRenderTexture.Attributes attributes)` — Initializes a new instance of the ModelRenderTexture class
- `public ModelRenderTexture(Guid id)` — Initializes a new instance of the ModelRenderTexture class
- `public ModelRenderTexture(RenderTexture renderTexture)` — Initializes a new instance of the ModelRenderTexture class

### Methods
#### `public static ModelRenderTexture Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelRenderTexture` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_Cast.htm)

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

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_CastTo__1.htm)

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

#### `public static implicit operator ModelRenderTexture (ColorRGBA rgba)`



**Parameters:**
- `rgba` (ColorRGBA)

**Returns:** `ModelRenderTexture` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_op_Implicit_1.htm)

#### `public static implicit operator ModelRenderTexture (ModelRenderTexture.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Render.ModelRenderTexture.Attributes)

**Returns:** `ModelRenderTexture` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_op_Implicit.htm)

#### `public static implicit operator ModelRenderTexture (RenderTexture renderTexture)`



**Parameters:**
- `renderTexture` (RenderTexture)

**Returns:** `ModelRenderTexture` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_op_Implicit_2.htm)

#### `public static implicit operator ModelRenderTexture (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelRenderTexture` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_op_Implicit_3.htm)

#### `public ModelRenderTexture.Attributes ToAttributes()`



**Returns:** `ModelRenderTexture.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_ToAttributes.htm)

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
- `TypeName` (String, get) — (Overrides ModelRenderContent.TypeName.)

## ModelRenderTexture.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ModelRenderTexture.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ModelRenderTexture_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelRenderTexture.Attributes class

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

#### `public static implicit operator ModelRenderTexture.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelRenderTexture.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Inherited from ModelRenderContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderContent_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelContent.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ModelRenderTexture_Attributes_ToString.htm)

### Properties
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `RenderTexture` (RenderTexture, get/set) — 
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)

## ObjectRender (class)

Represents a Rhino object render attributes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ObjectRender.htm)

### Constructors
- `public ObjectRender()` — Initializes a new instance of the ObjectRender class
- `public ObjectRender(ObjectRender.Attributes attributes)` — Initializes a new instance of the ObjectRender class

### Methods
#### `public static ObjectRender Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectRender` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_Cast.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelValue.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_CastTo__1.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_GetHashCode.htm)

#### `public static implicit operator ObjectRender (ObjectRender.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Render.ObjectRender.Attributes)

**Returns:** `ObjectRender` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_op_Implicit.htm)

#### `public ObjectRender.Attributes ToAttributes()`



**Returns:** `ObjectRender.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_ToAttributes.htm)

#### `public string ToDetails()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_ToDetails.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `CastsShadows` (Nullable<Boolean>, get) — 
- `Material` (Nullable<ObjectRenderMaterial.Value>, get) — 
- `ReceivesShadows` (Nullable<Boolean>, get) — 

## ObjectRender.Attributes (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ObjectRender.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ObjectRender_Attributes.htm)

### Methods
#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_Attributes_Equals_1.htm)

#### `public bool Equals(ObjectRender.Attributes other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Render.ObjectRender.Attributes)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_Attributes_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRender_Attributes_GetHashCode.htm)

### Properties
- `CastsShadows` (Nullable<Boolean>, get/set) — 
- `Material` (Nullable<ObjectRenderMaterial.Value>, get/set) — 
- `MeshingParameters` (ModelMeshingParameters, get/set) — 
- `ReceivesShadows` (Nullable<Boolean>, get/set) — 

## ObjectRenderMaterial (class)

Represents a Rhino object material.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial.htm)

### Constructors
- `public ObjectRenderMaterial()` — Initializes a new instance of the ObjectRenderMaterial class
- `public ObjectRenderMaterial(ObjectRenderMaterial.Value value)` — Initializes a new instance of the ObjectRenderMaterial class

### Methods
#### `public static ObjectRenderMaterial Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ObjectRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Cast.htm)

#### `public virtual bool CastTo<T>(out T target)`

(Inherited from ModelValue.)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelValue.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_CastTo__1.htm)

#### `public override bool Equals(Object other)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Equals.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_GetHashCode.htm)

#### `public static implicit operator ObjectRenderMaterial.Value (ObjectRenderMaterial data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Render.ObjectRenderMaterial)

**Returns:** `ObjectRenderMaterial.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_op_Implicit.htm)

#### `public static implicit operator ObjectRenderMaterial (ObjectRenderMaterial.Value data)`



**Parameters:**
- `data` (Grasshopper.Rhinoceros.Render.ObjectRenderMaterial.Value)

**Returns:** `ObjectRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_op_Implicit_1.htm)

#### `public override string ToString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_ToString.htm)

#### `public virtual string TooltipString()`

(Inherited from ModelValue.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelValue_TooltipString.htm)

### Properties
- `Material` (ModelRenderMaterial, get) — 
- `Source` (ObjectMaterialSource, get) — 

## ObjectRenderMaterial.Value (struct)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Render.ObjectRenderMaterial.Value.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value.htm)

### Constructors
- `public Value(ModelRenderMaterial value)` — Initializes a new instance of the ObjectRenderMaterial.Value class
- `public Value(ObjectMaterialSource source)` — Initializes a new instance of the ObjectRenderMaterial.Value class
- `public Value(ObjectMaterialSource source, ModelRenderMaterial value)` — Initializes a new instance of the ObjectRenderMaterial.Value class

### Methods
#### `public int CompareTo(Object obj)`



**Parameters:**
- `obj` (System.Object)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_CompareTo_1.htm)

#### `public int CompareTo(ObjectRenderMaterial.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Render.ObjectRenderMaterial.Value)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_CompareTo.htm)

#### `public void Deconstruct(out ObjectMaterialSource source, out ModelRenderMaterial value)`



**Parameters:**
- `source` (ObjectMaterialSource)
- `value` (Grasshopper.Rhinoceros.Render.ModelRenderMaterial)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_Deconstruct.htm)

#### `public override bool Equals(Object other)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `other` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_Equals_1.htm)

#### `public bool Equals(ObjectRenderMaterial.Value other)`



**Parameters:**
- `other` (Grasshopper.Rhinoceros.Render.ObjectRenderMaterial.Value)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_Equals.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_GetHashCode.htm)

#### `public static implicit operator ObjectRenderMaterial.Value (ModelRenderMaterial color)`



**Parameters:**
- `color` (Grasshopper.Rhinoceros.Render.ModelRenderMaterial)

**Returns:** `ObjectRenderMaterial.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_op_Implicit.htm)

#### `public static implicit operator ObjectRenderMaterial.Value (ObjectMaterialSource source)`



**Parameters:**
- `source` (ObjectMaterialSource)

**Returns:** `ObjectRenderMaterial.Value` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_op_Implicit_3.htm)

#### `public static implicit operator ModelRenderMaterial (ObjectRenderMaterial.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Render.ObjectRenderMaterial.Value)

**Returns:** `ModelRenderMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_op_Implicit_2.htm)

#### `public static implicit operator ObjectMaterialSource (ObjectRenderMaterial.Value value)`



**Parameters:**
- `value` (Grasshopper.Rhinoceros.Render.ObjectRenderMaterial.Value)

**Returns:** `ObjectMaterialSource` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_op_Implicit_1.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Render_ObjectRenderMaterial_Value_ToString.htm)

### Properties
- `ByLayer` (ObjectRenderMaterial.Value, get) — 
- `ByParent` (ObjectRenderMaterial.Value, get) — 
- `Default` (ObjectRenderMaterial.Value, get) — 
- `Material` (ModelRenderMaterial, get) — 
- `Source` (ObjectMaterialSource, get) — 

