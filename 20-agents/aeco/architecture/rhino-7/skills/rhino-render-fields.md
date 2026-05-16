---
name: rhino-common-rhino-render-fields
description: API reference for namespace Rhino.Render.Fields from RhinoCommon.dll
---

# Rhino.Render.Fields

- **BoolField**
  - bool field value class
- **ByteArrayField**
  - ByteArray field value class
- **Color4fField**
  - Color4f field value class
- **DateTimeField**
  - DateTime field value class
- **DoubleField**
  - double field value class
- **Field**
  - Generic data fields used to add publicly accessible properties to             RenderContent.FieldDictionary.  These should be created by calling a             FieldDictaionary.Add() method on a Render content object.  These are             allocated after the RenderContent object's C++ object is created and             added to the underlying C++ objects content dictionary, who ever             allocates a field is responsible for deleting it so these objects clean             up the C++ pointers when they are disposed of.
- **FieldDictionary**
  - Dictionary containing RenderContent data fields. Add fields to this              dictionary in your derived RenderContent classes constructor. Get field              values using the TryGet[data type]() methods and set them using the Set()              method.
- **FloatField**
  - float field value class
- **GuidField**
  - Guid field value class
- **IntField**
  - Integer field value class
- **NullField**
  - Null field value class
- **Point2dField**
  - Point2d field value class
- **Point3dField**
  - Point3d field value class
- **Point4dField**
  - Point4d field value class
- **StringField**
  - String field value class
- **TransformField**
  - Transform field value class
- **Vector2dField**
  - Vector2d field value class
- **Vector3dField**
  - Vector3d field value class
