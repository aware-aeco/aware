---
name: grasshopper-grasshopper-rhinoceros-model
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Rhinoceros.Model namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelEarthAnchorPoint, ModelEarthAnchorPoint.Attributes, ModelInstanceDefinition, ModelInstanceDefinition.Attributes, ModelLayer, ModelLayer.Attributes, ModelObject, ModelObject.Attributes.
---

# Grasshopper.Rhinoceros.Model

Auto-generated from vendor docs for grasshopper 8.0. 8 types in this namespace.

## ModelEarthAnchorPoint (class)

Represents a Rhino Earth anchor point. Wraps the functionality of the EarthAnchorPoint type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint.htm)

### Constructors
- `public ModelEarthAnchorPoint()` — Initializes a new instance of the ModelEarthAnchorPoint class
- `public ModelEarthAnchorPoint(ModelEarthAnchorPoint.Attributes attributes)` — Initializes a new instance of the ModelEarthAnchorPoint class
- `public ModelEarthAnchorPoint(EarthAnchorPoint anchorPoint)` — Initializes a new instance of the ModelEarthAnchorPoint class

### Methods
#### `public static ModelEarthAnchorPoint Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelEarthAnchorPoint` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_CastTo__1.htm)

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

#### `public static implicit operator ModelEarthAnchorPoint (EarthAnchorPoint anchorPoint)`



**Parameters:**
- `anchorPoint` (EarthAnchorPoint)

**Returns:** `ModelEarthAnchorPoint` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_op_Implicit_1.htm)

#### `public static implicit operator ModelEarthAnchorPoint (ModelEarthAnchorPoint.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Model.ModelEarthAnchorPoint.Attributes)

**Returns:** `ModelEarthAnchorPoint` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_op_Implicit.htm)

#### `public ModelEarthAnchorPoint.Attributes ToAttributes()`



**Returns:** `ModelEarthAnchorPoint.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Overrides ModelContent.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_ToString.htm)

#### `public override string TooltipString()`

(Overrides ModelContent.TooltipString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_TooltipString.htm)

### Properties
- `Elevation` (Nullable<Double>, get) — Gets a point elevation on earth, in meters.
- `ElevationCoordinateSystem` (Nullable<EarthCoordinateSystem>, get) — Gets the value indicating the zero level convention relating to a location on Earth
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Latitude` (Nullable<Double>, get) — Gets a point latitude on earth, in degrees. [-90, +90]
- `Longitude` (Nullable<Double>, get) — Gets a point longitude on earth, in degrees. [-180, +180]
- `ModelCompass` (Nullable<Plane>, get) — Gets the corresponding compass plane in model coordinates
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)

## ModelEarthAnchorPoint.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.ModelEarthAnchorPoint.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelEarthAnchorPoint.Attributes class

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

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Overrides ModelContent.Attributes.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelEarthAnchorPoint_Attributes_ToString.htm)

### Properties
- `Elevation` (Nullable<Double>, get/set) — 
- `ElevationCoordinateSystem` (Nullable<EarthCoordinateSystem>, get/set) — 
- `Latitude` (Nullable<Double>, get/set) — 
- `Longitude` (Nullable<Double>, get/set) — 
- `ModelCompass` (Nullable<Plane>, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)

## ModelInstanceDefinition (class)

Represents a Rhino block definition. Wraps the functionality of the InstanceDefinition type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition.htm)

### Constructors
- `public ModelInstanceDefinition()` — Initializes a new instance of the ModelInstanceDefinition class
- `public ModelInstanceDefinition(ModelInstanceDefinition.Attributes attributes)` — Initializes a new instance of the ModelInstanceDefinition class
- `public ModelInstanceDefinition(Guid id)` — Initializes a new instance of the ModelInstanceDefinition class
- `public ModelInstanceDefinition(InstanceDefinition block)` — Initializes a new instance of the ModelInstanceDefinition class

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_BakeGeometry.htm)

#### `public static ModelInstanceDefinition Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelInstanceDefinition` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_CastTo__1.htm)

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

#### `public BoundingBox GetBoundingBox(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_GetBoundingBox.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `public static implicit operator ModelInstanceDefinition (InstanceDefinition instanceDefinition)`



**Parameters:**
- `instanceDefinition` (InstanceDefinition)

**Returns:** `ModelInstanceDefinition` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_op_Implicit_1.htm)

#### `public static implicit operator ModelInstanceDefinition (ModelInstanceDefinition.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Model.ModelInstanceDefinition.Attributes)

**Returns:** `ModelInstanceDefinition` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_op_Implicit.htm)

#### `public static implicit operator ModelInstanceDefinition (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelInstanceDefinition` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_op_Implicit_2.htm)

#### `public ModelInstanceDefinition.Attributes ToAttributes()`



**Returns:** `ModelInstanceDefinition.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_ToAttributes.htm)

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
- `FilePath` (String, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Overrides ModelContent.IsValid.)
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Objects` (IReadOnlyList<ModelObject>, get) — 
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `Unset` (ModelInstanceDefinition, get) — 
- `UpdateType` (Nullable<InstanceDefinitionUpdateType>, get) — 
- `Url` (String, get) — 
- `UrlDescription` (String, get) — 
- `UserText` (ModelUserText, get) — (Inherited from ModelComponentContent.)

## ModelInstanceDefinition.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.ModelInstanceDefinition.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelInstanceDefinition.Attributes class

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

#### `public static implicit operator ModelInstanceDefinition.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelInstanceDefinition.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Overrides ModelComponentContent.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelInstanceDefinition_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `FilePath` (String, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Objects` (IReadOnlyList<ModelObject>, get/set) — 
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `Units` (Nullable<ModelUnitSystem.Value>, get/set) — 
- `UpdateType` (Nullable<InstanceDefinitionUpdateType>, get/set) — 
- `Url` (String, get/set) — 
- `UrlDescription` (String, get/set) — 
- `UserText` (ModelUserText, get/set) — (Inherited from ModelComponentContent.Attributes.)

## ModelLayer (class)

Represents a Rhino model layer. Wraps the functionality of the Layer type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelLayer.htm)

### Constructors
- `public ModelLayer()` — Initializes a new instance of the ModelLayer class
- `public ModelLayer(ModelLayer.Attributes attributes)` — Initializes a new instance of the ModelLayer class
- `public ModelLayer(Guid id)` — Initializes a new instance of the ModelLayer class
- `public ModelLayer(Layer layer)` — Initializes a new instance of the ModelLayer class

### Methods
#### `public static ModelLayer Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelLayer` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_Cast.htm)

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

#### `public static implicit operator ModelLayer (Layer layer)`



**Parameters:**
- `layer` (Layer)

**Returns:** `ModelLayer` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_op_Implicit_1.htm)

#### `public static implicit operator ModelLayer (ModelLayer.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Model.ModelLayer.Attributes)

**Returns:** `ModelLayer` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_op_Implicit.htm)

#### `public static implicit operator ModelLayer (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelLayer` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_op_Implicit_2.htm)

#### `public ModelLayer.Attributes ToAttributes()`



**Returns:** `ModelLayer.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_ToAttributes.htm)

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
- `DisplayColor` (Nullable<ModelColor>, get) — 
- `DraftingColor` (Nullable<ModelColor>, get) — 
- `Hidden` (Nullable<Boolean>, get) — 
- `HiddenOnModel` (Nullable<Boolean>, get) — 
- `HiddenOnNewDetail` (Nullable<Boolean>, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Inherited from ModelContent.)
- `Linetype` (ModelLinetype, get) — 
- `LineWeight` (Nullable<Double>, get) — 
- `Locked` (Nullable<Boolean>, get) — 
- `Material` (ModelRenderMaterial, get) — 
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `Unset` (ModelLayer, get) — 
- `UserText` (ModelUserText, get) — (Inherited from ModelComponentContent.)

## ModelLayer.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.ModelLayer.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelLayer_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelLayer.Attributes class

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

#### `public static implicit operator ModelLayer.Attributes (string path)`



**Parameters:**
- `path` (System.String)

**Returns:** `ModelLayer.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_Attributes_op_Implicit.htm)

#### `public override string ToDetails()`

(Overrides ModelComponentContent.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelLayer_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `DisplayColor` (Nullable<ModelColor>, get/set) — 
- `DraftingColor` (Nullable<ModelColor>, get/set) — 
- `Hidden` (Nullable<Boolean>, get/set) — 
- `HiddenOnModel` (Nullable<Boolean>, get/set) — 
- `HiddenOnNewDetail` (Nullable<Boolean>, get/set) — 
- `Linetype` (ModelLinetype, get/set) — 
- `LineWeight` (Nullable<Double>, get/set) — 
- `Locked` (Nullable<Boolean>, get/set) — 
- `Material` (ModelRenderMaterial, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `UserText` (ModelUserText, get/set) — (Inherited from ModelComponentContent.Attributes.)

## ModelObject (class)

Represents a Rhino model object. Wraps the functionality of the RhinoObject type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelObject.htm)

### Constructors
- `public ModelObject()` — Initializes a new instance of the ModelObject class
- `public ModelObject(ModelObject.Attributes attributes)` — Initializes a new instance of the ModelObject class
- `public ModelObject(Guid id)` — Initializes a new instance of the ModelObject class
- `public ModelObject(RhinoObject rhinoObject)` — Initializes a new instance of the ModelObject class
- `public ModelObject(RhinoDoc document, ObjectAttributes objectAttributes)` — Initializes a new instance of the ModelObject class
- `public ModelObject(RhinoDoc document, ObjectAttributes objectAttributes, GeometryBase geometry)` — Initializes a new instance of the ModelObject class

### Methods
#### `public static ModelObject Cast(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `ModelObject` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_Cast.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides ModelContent.CastTo<T>(T).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_CastTo__1.htm)

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

#### `public BoundingBox GetBoundingBox(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_GetBoundingBox.htm)

#### `public override int GetHashCode()`

(Inherited from ModelContent.)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_GetHashCode.htm)

#### `public static implicit operator ModelObject (ModelObject.Attributes attributes)`



**Parameters:**
- `attributes` (Grasshopper.Rhinoceros.Model.ModelObject.Attributes)

**Returns:** `ModelObject` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_op_Implicit.htm)

#### `public static implicit operator ModelObject (RhinoObject object)`



**Parameters:**
- `object` (RhinoObject)

**Returns:** `ModelObject` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_op_Implicit_1.htm)

#### `public ModelObject.Attributes ToAttributes()`



**Returns:** `ModelObject.Attributes` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_ToAttributes.htm)

#### `public string ToDetails()`

(Inherited from ModelData.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelData_ToDetails.htm)

#### `public override string ToString()`

(Overrides ModelContent.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_ToString.htm)

#### `public override string TooltipString()`

(Overrides ModelContent.TooltipString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_TooltipString.htm)

### Properties
- `Display` (ObjectDisplay, get) — 
- `Drafting` (ObjectDrafting, get) — 
- `Id` (Nullable<Guid>, get) — (Inherited from ModelContent.)
- `IsValid` (Boolean, get) — (Overrides ModelContent.IsValid.)
- `Layer` (ModelLayer, get) — 
- `Name` (ModelContentName, get) — (Inherited from ModelContent.)
- `Notes` (String, get) — (Inherited from ModelContent.)
- `ObjectType` (ObjectType, get) — 
- `Parent` (ModelContentName, get) — (Inherited from ModelContent.)
- `Path` (ModelContentName, get) — (Inherited from ModelContent.)
- `Render` (ObjectRender, get) — 
- `Tags` (ModelTags, get) — (Inherited from ModelContent.)
- `TypeName` (String, get) — (Overrides ModelData.TypeName.)
- `Url` (String, get) — 
- `UserText` (ModelUserText, get) — (Inherited from ModelComponentContent.)
- `Visibility` (ObjectVisibility, get) — 

## ModelObject.Attributes (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.ModelObject.Attributes.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_ModelObject_Attributes.htm)

### Constructors
- `public Attributes()` — Initializes a new instance of the ModelObject.Attributes class

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

(Overrides ModelComponentContent.Attributes.ToDetails().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_Attributes_ToDetails.htm)

#### `public override ModelData ToModelData()`

(Overrides ModelData.Attributes.ToModelData().)

**Returns:** `ModelData` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_ModelObject_Attributes_ToModelData.htm)

#### `public override string ToString()`

(Inherited from ModelContent.Attributes.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_ModelContent_Attributes_ToString.htm)

### Properties
- `Display` (ObjectDisplay, get/set) — 
- `Drafting` (ObjectDrafting, get/set) — 
- `Frame` (Nullable<Plane>, get/set) — 
- `Layer` (ModelLayer, get/set) — 
- `Name` (ModelContentName, get/set) — The last component of the element Path.
- `Notes` (String, get/set) — (Inherited from ModelContent.Attributes.)
- `Parent` (ModelContentName, get/set) — Path of the parent element in 'Root::Parent' format if nested.
- `Path` (ModelContentName, get/set) — Full content path in 'Root::Parent::Name' format if is nested.
- `Render` (ObjectRender, get/set) — 
- `Tags` (ModelTags, get/set) — (Inherited from ModelContent.Attributes.)
- `Url` (String, get/set) — 
- `UserText` (ModelUserText, get/set) — (Inherited from ModelComponentContent.Attributes.)
- `Visibility` (ObjectVisibility, get/set) — 

