---
name: rhino-rhino-render-fields
description: This skill encodes the rhino 8.0 surface of the Rhino.Render.Fields namespace — 18 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BoolField, ByteArrayField, Color4fField, DoubleField, DateTimeField, Field, FieldDictionary, FloatField, and 10 more types.
---

# Rhino.Render.Fields

Auto-generated from vendor docs for rhino 8.0. 18 types in this namespace.

## BoolField (class)

bool field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_BoolField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.BoolField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_BoolField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Boolean, get/set) — Gets or sets the field value

## ByteArrayField (class)

ByteArray field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_ByteArrayField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.ByteArrayField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_ByteArrayField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Byte[], get/set) — Gets or sets the field value

## Color4fField (class)

Color4f field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_Color4fField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.Color4fField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Color4fField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `SystemColorValue` (Color, get/set) — Gets or sets the field value
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Color4f, get/set) — Gets or sets the field value

## DateTimeField (class)

DateTime field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_DateTimeField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.DateTimeField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_DateTimeField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (DateTime, get/set) — Gets or sets the field value

## DoubleField (class)

double field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_DoubleField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.DoubleField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_DoubleField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Double, get/set) — Gets or sets the field value

## Field (class)

Generic data fields used to add publicly accessible properties to RenderContent.FieldDictionary. These should be created by calling a FieldDictaionary.Add() method on a Render content object. These are allocated after the RenderContent object's C++ object is created and added to the underlying C++ objects content dictionary, who ever allocates a field is responsible for deleting it so these objects clean up the C++ pointers when they are disposed of.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_Field.htm)

### Constructors
- `protected Field(RenderContent renderContent, IntPtr pointer)` — Initializes a new instance of the Field class
- `protected Field(RenderContent renderContent, string name)` — Initializes a new instance of the Field class
- `[ObsoleteAttribute] protected Field(RenderContent renderContent, string name, string prompt, Object initialValue, bool isTextured)` — Initializes a new instance of the Field class
- `protected Field(RenderContent renderContent, string name, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)` — Initializes a new instance of the Field class
- `protected Field(RenderContent renderContent, string name, string prompt, Object initialValue, bool isTextured, bool treatAsLinear, int sectionId)` — Initializes a new instance of the Field class

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`



**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public abstract Object ValueAsObject()`



**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.Field.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — 
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.

## FieldDictionary (class)

Dictionary containing RenderContent data fields. Add fields to this dictionary in your derived RenderContent classes constructor. Get field values using the TryGet[data type]() methods and set them using the Set() method.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_FieldDictionary.htm)

### Methods
#### `public BoolField Add(string key, bool value)`

Add a new BoolField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Boolean) — Initial value for this field.

**Returns:** `BoolField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_21.htm)

#### `public BoolField Add(string key, bool value, string prompt)`

Add a new BoolField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Boolean) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `BoolField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Boolean,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_22.htm)

#### `public BoolField Add(string key, bool value, string prompt, int sectionId)`

Add a new BoolField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Boolean) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `BoolField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Boolean,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_23.htm)

#### `public ByteArrayField Add(string key, byte[] value)`

AddField a new ByteArrayField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Byte[]) — Initial value for this field.

**Returns:** `ByteArrayField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Byte[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_24.htm)

#### `public Color4fField Add(string key, Color value)`

Add a new Color4fField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Drawing.Color) — Initial value for this field.

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_31.htm)

#### `public Color4fField Add(string key, Color value, string prompt)`

Add a new Color4fField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Drawing.Color) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Drawing.Color,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_32.htm)

#### `public Color4fField Add(string key, Color4f value)`

Add a new Color4fField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Display.Color4f) — Initial value for this field.

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Display.Color4f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add.htm)

#### `public Color4fField Add(string key, Color4f value, string prompt)`

Add a new Color4fField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Display.Color4f) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Display.Color4f,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_1.htm)

#### `public Color4fField Add(string key, Color4f value, string prompt, int sectionId)`

Add a new Color4fField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Display.Color4f) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Display.Color4f,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_2.htm)

#### `public DateTimeField Add(string key, DateTime value)`

Add a new DateTimeField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.DateTime) — Initial value for this field.

**Returns:** `DateTimeField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.DateTime)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_25.htm)

#### `public DateTimeField Add(string key, DateTime value, string prompt)`

Add a new DateTimeField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.DateTime) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `DateTimeField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.DateTime,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_26.htm)

#### `public DateTimeField Add(string key, DateTime value, string prompt, int sectionId)`

Add a new DateTimeField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.DateTime) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `DateTimeField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.DateTime,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_27.htm)

#### `public DoubleField Add(string key, double value)`

AddField a new DoubleField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Double) — Initial value for this field.

**Returns:** `DoubleField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_28.htm)

#### `public DoubleField Add(string key, double value, string prompt)`

Add a new DoubleField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Double) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `DoubleField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Double,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_29.htm)

#### `public DoubleField Add(string key, double value, string prompt, int sectionId)`

Add a new DoubleField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Double) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `DoubleField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Double,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_30.htm)

#### `public GuidField Add(string key, Guid value)`

Add a new GuidField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Guid) — Initial value for this field.

**Returns:** `GuidField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_33.htm)

#### `public GuidField Add(string key, Guid value, string prompt)`

Add a new GuidField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Guid) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `GuidField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Guid,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_34.htm)

#### `public GuidField Add(string key, Guid value, string prompt, int sectionId)`

Add a new GuidField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Guid) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `GuidField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Guid,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_35.htm)

#### `public IntField Add(string key, int value)`

Add a new IntField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Int32) — Initial value for this field.

**Returns:** `IntField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_36.htm)

#### `public IntField Add(string key, int value, string prompt)`

Add a new IntField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Int32) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `IntField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Int32,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_37.htm)

#### `public IntField Add(string key, int value, string prompt, int sectionId)`

Add a new IntField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Int32) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `IntField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Int32,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_38.htm)

#### `public Point2dField Add(string key, Point2d value)`

Add a new Point2dField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point2d) — Initial value for this field.

**Returns:** `Point2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_3.htm)

#### `public Point2dField Add(string key, Point2d value, string prompt)`

Add a new Point2dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Point2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point2d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_4.htm)

#### `public Point2dField Add(string key, Point2d value, string prompt, int sectionId)`

Add a new Point2dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Point2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point2d,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_5.htm)

#### `public Point3dField Add(string key, Point3d value)`

Add a new Point3dField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point3d) — Initial value for this field.

**Returns:** `Point3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_6.htm)

#### `public Point3dField Add(string key, Point3d value, string prompt)`

Add a new Point3dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Point3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point3d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_7.htm)

#### `public Point3dField Add(string key, Point3d value, string prompt, int sectionId)`

Add a new Point3dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Point3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point3d,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_8.htm)

#### `public Point4dField Add(string key, Point4d value)`

Add a new Point4dField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point4d) — Initial value for this field.

**Returns:** `Point4dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point4d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_9.htm)

#### `public Point4dField Add(string key, Point4d value, string prompt)`

Add a new Point4dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point4d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Point4dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point4d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_10.htm)

#### `public Point4dField Add(string key, Point4d value, string prompt, int sectionId)`

Add a new Point4dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point4d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Point4dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Point4d,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_11.htm)

#### `public FloatField Add(string key, float value)`

Add a new FloatField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Single) — Initial value for this field.

**Returns:** `FloatField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_39.htm)

#### `public FloatField Add(string key, float value, string prompt)`

AddField a new FloatField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Single) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `FloatField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Single,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_40.htm)

#### `public FloatField Add(string key, float value, string prompt, int sectionId)`

AddField a new FloatField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above
- `value` (System.Single) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `FloatField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.Single,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_41.htm)

#### `public StringField Add(string key, string value)`

Add a new StringField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.String) — Initial value for this field.

**Returns:** `StringField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_42.htm)

#### `public NullField Add(string key, string prompt, int sectionId)`

Add a new NullField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — Id of the section containing the field

**Returns:** `NullField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_43.htm)

#### `public StringField Add(string key, string value, string prompt)`

Add a new StringField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.String) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `StringField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_44.htm)

#### `public StringField Add(string key, string value, string prompt, int sectionId)`

Add a new StringField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.String) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `StringField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,System.String,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_45.htm)

#### `public TransformField Add(string key, Transform value)`

Add a new TransformField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Transform) — Initial value for this field.

**Returns:** `TransformField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_12.htm)

#### `public TransformField Add(string key, Transform value, string prompt)`

Add a new TransformField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Transform) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `TransformField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Transform,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_13.htm)

#### `public TransformField Add(string key, Transform value, string prompt, int sectionId)`

Add a new TransformField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Transform) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `TransformField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Transform,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_14.htm)

#### `public Vector2dField Add(string key, Vector2d value)`

Add a new Vector2dField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector2d) — Initial value for this field.

**Returns:** `Vector2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Vector2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_15.htm)

#### `public Vector2dField Add(string key, Vector2d value, string prompt)`

Add a new Vector2dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Vector2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Vector2d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_16.htm)

#### `public Vector2dField Add(string key, Vector2d value, string prompt, int sectionId)`

Add a new Vector2dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Vector2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Vector2d,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_17.htm)

#### `public Vector3dField Add(string key, Vector3d value)`

Add a new Vector3dField to the dictionary. This will be a data only field and not show up in the content browsers.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector3d) — Initial value for this field.

**Returns:** `Vector3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Vector3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_18.htm)

#### `public Vector3dField Add(string key, Vector3d value, string prompt)`

Add a new Vector3dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Vector3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Vector3d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_19.htm)

#### `public Vector3dField Add(string key, Vector3d value, string prompt, int sectionId)`

Add a new Vector3dField to the dictionary.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Vector3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.Add(System.String,Rhino.Geometry.Vector3d,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Add_20.htm)

#### `public StringField AddFilename(string key, string value, string prompt, int sectionId)`

Add a new StringField to the dictionary which will reference a filename. A file watcher will automatically be attached to the referenced file, and the content will be considered changed when the file is changed on disk.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.String) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `StringField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddFilename(System.String,System.String,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddFilename.htm)

#### `public BoolField AddTextured(string key, bool value, string prompt)`

Add a new BoolField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Boolean) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `BoolField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Boolean,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_21.htm)

#### `public BoolField AddTextured(string key, bool value, string prompt, bool treatAsLinear)`

Add a new BoolField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Boolean) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `BoolField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Boolean,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_22.htm)

#### `public BoolField AddTextured(string key, bool value, string prompt, bool treatAsLinear, int sectionId)`

Add a new BoolField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Boolean) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `BoolField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Boolean,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_23.htm)

#### `public Color4fField AddTextured(string key, Color value, string prompt)`

Add a new Color4fField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Drawing.Color) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Drawing.Color,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_30.htm)

#### `public Color4fField AddTextured(string key, Color value, string prompt, bool treatAsLinear)`

Add a new Color4fField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Drawing.Color) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Drawing.Color,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_31.htm)

#### `[ObsoleteAttribute] public Color4fField AddTextured(string key, Color4f value, string prompt)`

Obsolete.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Display.Color4f,System.String)"]
- `value` (Rhino.Display.Color4f) — [Missing <param name="value"/> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Display.Color4f,System.String)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Display.Color4f,System.String)"]

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Display.Color4f,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured.htm)

#### `public Color4fField AddTextured(string key, Color4f value, string prompt, bool treatAsLinear)`

Add a new Color4fField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Display.Color4f) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Display.Color4f,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_1.htm)

#### `public Color4fField AddTextured(string key, Color4f value, string prompt, bool treatAsLinear, int sectionId)`

Add a new Color4fField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Display.Color4f) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Color4fField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Display.Color4f,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_2.htm)

#### `public DateTimeField AddTextured(string key, DateTime value, string prompt)`

Add a new DateTimeField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.DateTime) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `DateTimeField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.DateTime,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_24.htm)

#### `public DateTimeField AddTextured(string key, DateTime value, string prompt, bool treatAsLinear)`

Add a new DateTimeField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.DateTime) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `DateTimeField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.DateTime,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_25.htm)

#### `public DateTimeField AddTextured(string key, DateTime value, string prompt, bool treatAsLinear, int sectionId)`

Add a new DateTimeField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.DateTime) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `DateTimeField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.DateTime,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_26.htm)

#### `public DoubleField AddTextured(string key, double value, string prompt)`

Add a new DoubleField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Double) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `DoubleField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Double,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_27.htm)

#### `public DoubleField AddTextured(string key, double value, string prompt, bool treatAsLinear)`

Add a new DoubleField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Double) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `DoubleField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Double,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_28.htm)

#### `public DoubleField AddTextured(string key, double value, string prompt, bool treatAsLinear, int sectionId)`

Add a new DoubleField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Double) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `DoubleField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Double,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_29.htm)

#### `public GuidField AddTextured(string key, Guid value, string prompt)`

Add a new GuidField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Guid) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `GuidField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Guid,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_32.htm)

#### `public GuidField AddTextured(string key, Guid value, string prompt, bool treatAsLinear)`

Add a new GuidField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Guid) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `GuidField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Guid,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_33.htm)

#### `public GuidField AddTextured(string key, Guid value, string prompt, bool treatAsLinear, int sectionId)`

Add a new GuidField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Guid) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `GuidField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Guid,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_34.htm)

#### `public IntField AddTextured(string key, int value, string prompt)`

Add a new IntField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Int32) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `IntField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Int32,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_35.htm)

#### `public IntField AddTextured(string key, int value, string prompt, bool treatAsLinear)`

Add a new IntField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Int32) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `IntField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Int32,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_36.htm)

#### `public IntField AddTextured(string key, int value, string prompt, bool treatAsLinear, int sectionId)`

Add a new IntField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Int32) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `IntField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Int32,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_37.htm)

#### `public Point2dField AddTextured(string key, Point2d value, string prompt)`

Add a new Point2dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Point2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point2d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_3.htm)

#### `public Point2dField AddTextured(string key, Point2d value, string prompt, bool treatAsLinear)`

Add a new Point2dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `Point2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point2d,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_4.htm)

#### `public Point2dField AddTextured(string key, Point2d value, string prompt, bool treatAsLinear, int sectionId)`

Add a new Point2dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Point2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point2d,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_5.htm)

#### `public Point3dField AddTextured(string key, Point3d value, string prompt)`

Add a new Point3dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Point3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point3d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_6.htm)

#### `public Point3dField AddTextured(string key, Point3d value, string prompt, bool treatAsLinear)`

Add a new Point3dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `Point3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point3d,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_7.htm)

#### `public Point3dField AddTextured(string key, Point3d value, string prompt, bool treatAsLinear, int sectionId)`

Add a new Point3dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Point3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point3d,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_8.htm)

#### `public Point4dField AddTextured(string key, Point4d value, string prompt)`

Add a new Point4dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point4d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Point4dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point4d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_9.htm)

#### `public Point4dField AddTextured(string key, Point4d value, string prompt, bool treatAsLinear)`

Add a new Point4dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point4d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `Point4dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point4d,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_10.htm)

#### `public Point4dField AddTextured(string key, Point4d value, string prompt, bool treatAsLinear, int sectionId)`

Add a new Point4dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Point4d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Point4dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Point4d,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_11.htm)

#### `public FloatField AddTextured(string key, float value, string prompt)`

Add a new FloatField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Single) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `FloatField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Single,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_38.htm)

#### `public FloatField AddTextured(string key, float value, string prompt, bool treatAsLinear)`

Add a new FloatField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.Single) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `FloatField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Single,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_39.htm)

#### `public FloatField AddTextured(string key, float value, string prompt, bool treatAsLinear, int sectionId)`

Add a new FloatField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above
- `value` (System.Single) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `FloatField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.Single,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_40.htm)

#### `public NullField AddTextured(string key, string prompt)`

Add a new NullField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `NullField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_41.htm)

#### `public NullField AddTextured(string key, string prompt, bool treatAsLinear)`

Add a new NullField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `NullField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_42.htm)

#### `public NullField AddTextured(string key, string prompt, bool treatAsLinear, int sectionId)`

Add a new NullField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `NullField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_43.htm)

#### `public StringField AddTextured(string key, string value, string prompt)`

Add a new StringField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.String) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `StringField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_44.htm)

#### `public StringField AddTextured(string key, string value, string prompt, bool treatAsLinear)`

Add a new StringField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.String) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `StringField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.String,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_45.htm)

#### `public StringField AddTextured(string key, string value, string prompt, bool treatAsLinear, int sectionId)`

Add a new StringField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (System.String) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `StringField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,System.String,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_46.htm)

#### `public TransformField AddTextured(string key, Transform value, string prompt)`

Add a new TransformField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Transform) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `TransformField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Transform,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_12.htm)

#### `public TransformField AddTextured(string key, Transform value, string prompt, bool treatAsLinear)`

Add a new TransformField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Transform) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `TransformField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Transform,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_13.htm)

#### `public TransformField AddTextured(string key, Transform value, string prompt, bool treatAsLinear, int sectionId)`

Add a new TransformField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Transform) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `TransformField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Transform,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_14.htm)

#### `public Vector2dField AddTextured(string key, Vector2d value, string prompt)`

Add a new Vector2dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Vector2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Vector2d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_15.htm)

#### `public Vector2dField AddTextured(string key, Vector2d value, string prompt, bool treatAsLinear)`

Add a new Vector2dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `Vector2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Vector2d,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_16.htm)

#### `public Vector2dField AddTextured(string key, Vector2d value, string prompt, bool treatAsLinear, int sectionId)`

Add a new Vector2dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector2d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Vector2dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Vector2d,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_17.htm)

#### `public Vector3dField AddTextured(string key, Vector3d value, string prompt)`

Add a new Vector3dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.

**Returns:** `Vector3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Vector3d,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_18.htm)

#### `public Vector3dField AddTextured(string key, Vector3d value, string prompt, bool treatAsLinear)`

Add a new Vector3dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).

**Returns:** `Vector3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Vector3d,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_19.htm)

#### `public Vector3dField AddTextured(string key, Vector3d value, string prompt, bool treatAsLinear, int sectionId)`

Add a new Vector3dField to the dictionary. This overload will cause the field to be tagged as "textured" so that the texturing UI will appear in automatic UIs.

**Parameters:**
- `key` (System.String) — Key name for the field value to add. See important note above.
- `value` (Rhino.Geometry.Vector3d) — Initial value for this field.
- `prompt` (System.String) — Prompt to display in the user interface (Content Browsers) if this is null or an empty string the this field is a data only field and will not appear in the user interface.
- `treatAsLinear` (System.Boolean) — Determines whether the texture in this slot will be treated as linear by rendering engines (ie - not gamma packed).
- `sectionId` (System.Int32) — The id of the section containing the field

**Returns:** `Vector3dField` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.AddTextured(System.String,Rhino.Geometry.Vector3d,System.String,System.Boolean,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_AddTextured_20.htm)

#### `public bool ContainsField(string fieldName)`

Call this method to determine if a this FieldsList contains a field with the specified field name.

**Parameters:**
- `fieldName` (System.String) — Field to search for

**Returns:** `Boolean` — Returns true if a field with that matches fieldName is found or false if it is not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_ContainsField.htm)

#### `public IEnumerator<Field> GetEnumerator()`



**Returns:** `IEnumerator<Field>` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FieldDictionary.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_GetEnumerator.htm)

#### `public Field GetField(string fieldName)`

Call this method to get the field with the specified name.

**Parameters:**
- `fieldName` (System.String) — Field name to search for.

**Returns:** `Field` — If the field exists in the Fields dictionary then the field is returned otherwise; null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_GetField.htm)

#### `public void RemoveField(string fieldName)`



**Parameters:**
- `fieldName` (System.String) — [Missing <param name="fieldName"/> documentation for "M:Rhino.Render.Fields.FieldDictionary.RemoveField(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_RemoveField.htm)

#### `public void Set(string key, bool value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.Boolean) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_7.htm)

#### `public void Set(string key, byte[] value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.Byte[]) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_8.htm)

#### `public void Set(string key, Color value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.Drawing.Color) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_11.htm)

#### `public void Set(string key, Color4f value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (Rhino.Display.Color4f) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set.htm)

#### `public void Set(string key, DateTime value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.DateTime) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_9.htm)

#### `public void Set(string key, double value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.Double) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_10.htm)

#### `public void Set(string key, Guid value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.Guid) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_12.htm)

#### `public void Set(string key, int value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.Int32) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_13.htm)

#### `public void Set(string key, Point2d value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (Rhino.Geometry.Point2d) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_1.htm)

#### `public void Set(string key, Point3d value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (Rhino.Geometry.Point3d) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_2.htm)

#### `public void Set(string key, Point4d value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (Rhino.Geometry.Point4d) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_3.htm)

#### `public void Set(string key, float value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.Single) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_14.htm)

#### `public void Set(string key, string value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (System.String) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_15.htm)

#### `public void Set(string key, Transform value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (Rhino.Geometry.Transform) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_4.htm)

#### `public void Set(string key, Vector2d value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (Rhino.Geometry.Vector2d) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_5.htm)

#### `public void Set(string key, Vector3d value)`

Set the field value and send the appropriate change notification to the render SDK. Will throw a InvalidOperationException exception if the key name is not valid.

**Parameters:**
- `key` (System.String) — Key name for the field value to change. See important note above.
- `value` (Rhino.Geometry.Vector3d) — New value for this field.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_Set_6.htm)

#### `public bool TryGetValue(string key, out bool value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.Boolean) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_7.htm)

#### `public bool TryGetValue(string key, out byte[] value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.Byte[]) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_8.htm)

#### `public bool TryGetValue(string key, out Color value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.Drawing.Color) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_11.htm)

#### `public bool TryGetValue(string key, out Color4f value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (Rhino.Display.Color4f) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue.htm)

#### `public bool TryGetValue(string key, out DateTime value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.DateTime) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_9.htm)

#### `public bool TryGetValue(string key, out double value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.Double) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_10.htm)

#### `public bool TryGetValue(string key, out Guid value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.Guid) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_12.htm)

#### `public bool TryGetValue(string key, out int value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.Int32) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_13.htm)

#### `public bool TryGetValue(string key, out Point2d value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (Rhino.Geometry.Point2d) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_1.htm)

#### `public bool TryGetValue(string key, out Point3d value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (Rhino.Geometry.Point3d) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_2.htm)

#### `public bool TryGetValue(string key, out Point4d value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (Rhino.Geometry.Point4d) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_3.htm)

#### `public bool TryGetValue(string key, out float value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.Single) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_14.htm)

#### `public bool TryGetValue(string key, out string value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (System.String) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_15.htm)

#### `public bool TryGetValue(string key, out Transform value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (Rhino.Geometry.Transform) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_4.htm)

#### `public bool TryGetValue(string key, out Vector2d value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (Rhino.Geometry.Vector2d) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_5.htm)

#### `public bool TryGetValue(string key, out Vector3d value)`

Find a field with the specified key and get its value if found.

**Parameters:**
- `key` (System.String) — Key name of the field to get a value for.
- `value` (Rhino.Geometry.Vector3d) — Output parameter which will receive the field value.

**Returns:** `Boolean` — Returns true if the key is found and the value parameter is set to the field value. Returns false if the field was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue_6.htm)

#### `public bool TryGetValue<T>(string key, out T value)`

Parametrized version of TryGetValue.

**Parameters:**
- `key` (System.String) — Name of field to find.
- `value` (T) — out parameter to be set.

**Returns:** `Boolean` — true if field was found. If false out parameter value will be set to default(T).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FieldDictionary_TryGetValue__1.htm)

## FloatField (class)

float field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_FloatField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.FloatField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_FloatField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Single, get/set) — Gets or sets the field value

## GuidField (class)

Guid field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_GuidField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.GuidField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_GuidField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Guid, get/set) — Gets or sets the field value

## IntField (class)

Integer field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_IntField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.IntField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_IntField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Int32, get/set) — Gets or sets the field value

## NullField (class)

Null field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_NullField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.NullField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_NullField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Boolean, get/set) — Gets or sets the field value

## Point2dField (class)

Point2d field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_Point2dField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.Point2dField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Point2dField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Point2d, get/set) — Gets or sets the field value

## Point3dField (class)

Point3d field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_Point3dField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.Point3dField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Point3dField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Point3d, get/set) — Gets or sets the field value

## Point4dField (class)

Point4d field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_Point4dField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.Point4dField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Point4dField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Point4d, get/set) — Gets or sets the field value

## StringField (class)

String field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_StringField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.StringField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_StringField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (String, get/set) — Gets or sets the field value

## TransformField (class)

Transform field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_TransformField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.TransformField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_TransformField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Transform, get/set) — Gets or sets the field value

## Vector2dField (class)

Vector2d field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_Vector2dField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.Vector2dField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Vector2dField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Vector2d, get/set) — Gets or sets the field value

## Vector3dField (class)

Vector3d field value class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_Fields_Vector3dField.htm)

### Methods
#### `protected void CreateCppPointer(RenderContent content, string key, string prompt, Object initialValue, bool isTextured, bool treatAsLinear)`

(Inherited from Field.)

**Parameters:**
- `content` (Rhino.Render.RenderContent) — [Missing <param name="content"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `initialValue` (System.Object) — [Missing <param name="initialValue"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `isTextured` (System.Boolean) — [Missing <param name="isTextured"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]
- `treatAsLinear` (System.Boolean) — [Missing <param name="treatAsLinear"/> documentation for "M:Rhino.Render.Fields.Field.CreateCppPointer(Rhino.Render.RenderContent,System.String,System.String,System.Object,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_CreateCppPointer.htm)

#### `public T GetValue<T>()`

Parametrized version of GetValue calling appropriate ValueAs* methods.

**Returns:** `T` — Value of type T of the field

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_GetValue__1.htm)

#### `protected bool ValueAsBool()`

Return field value as a bool.

**Returns:** `Boolean` — Returns field value as a bool.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsBool.htm)

#### `protected byte[] ValueAsByteArray()`

Return field as a byte array.

**Returns:** `Byte[]` — Return field as a byte array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsByteArray.htm)

#### `protected Color4f ValueAsColor4f()`

Return field as a Rhino.Display.Color4f color value.

**Returns:** `Color4f` — Return field as a Rhino.Display.Color4f color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsColor4f.htm)

#### `protected DateTime ValueAsDateTime()`

Return field as a DateTime value.

**Returns:** `DateTime` — Return field as a DateTime value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDateTime.htm)

#### `protected double ValueAsDouble()`

Return field value as a double precision number.

**Returns:** `Double` — Return the field value as a double precision number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsDouble.htm)

#### `protected float ValueAsFloat()`

Return field value as floating point number.

**Returns:** `Single` — Return the field value as an floating point number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsFloat.htm)

#### `protected Guid ValueAsGuid()`

Return field value as Guid.

**Returns:** `Guid` — Return the field value as an Guid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsGuid.htm)

#### `protected int ValueAsInt()`

Return field value as integer.

**Returns:** `Int32` — Return the field value as an integer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsInt.htm)

#### `public override Object ValueAsObject()`

(Overrides Field.ValueAsObject().)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.Render.Fields.Vector3dField.ValueAsObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Vector3dField_ValueAsObject.htm)

#### `protected Point2d ValueAsPoint2d()`

Return field as a Rhino.Geometry.Point2d color value.

**Returns:** `Point2d` — Return field as a Rhino.Geometry.Point2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint2d.htm)

#### `protected Point3d ValueAsPoint3d()`

Return field as a Rhino.Geometry.Point3d color value.

**Returns:** `Point3d` — Return field as a Rhino.Geometry.Point3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint3d.htm)

#### `protected Point4d ValueAsPoint4d()`

Return field as a Rhino.Geometry.Point4d color value.

**Returns:** `Point4d` — Return field as a Rhino.Geometry.Point4d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsPoint4d.htm)

#### `protected string ValueAsString()`

Get field value as a string.

**Returns:** `String` — Returns the field value as a string if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsString.htm)

#### `protected Transform ValueAsTransform()`

Return field as a Rhino.Geometry.Transform color value.

**Returns:** `Transform` — Return field as a Rhino.Geometry.Transform color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsTransform.htm)

#### `protected Vector2d ValueAsVector2d()`

Return field as a Rhino.Geometry.Vector2d color value.

**Returns:** `Vector2d` — Return field as a Rhino.Geometry.Vector2d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector2d.htm)

#### `protected Vector3d ValueAsVector3d()`

Return field as a Rhino.Geometry.Vector3d color value.

**Returns:** `Vector3d` — Return field as a Rhino.Geometry.Vector3d color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_Fields_Field_ValueAsVector3d.htm)

### Properties
- `AutomaticRegisteredProperty` (Boolean, get/set) — (Inherited from Field.)
- `IsHiddenInAutoUI` (Boolean, get/set) — When fields are used by the automatic UI, they can be hidden from it by calling this method.
- `Name` (String, get) — Field name value string passed to the constructor.
- `Tag` (Object, get/set) — Gets or sets an object that contains data to associate with the field.
- `TextureAmountMax` (Double, get/set) — Set Max value for Texture amount
- `TextureAmountMin` (Double, get/set) — Set Min value for Texture amount
- `UseTextureAmount` (Boolean, get/set) — True if 'texture amount' is in use, otherwise false. The 'texture amount' is represented as a numeric stepper in the UI. If true, then the stepper is shown. If false, then the stepper is hidden.
- `UseTextureOn` (Boolean, get/set) — True if 'texture on' is in use, otherwise false. In the UI 'texture on' is represented as a checkbox. If true then the checbox is shown. If false then the checkbox is not shown.
- `Value` (Vector3d, get/set) — Gets or sets the field value

