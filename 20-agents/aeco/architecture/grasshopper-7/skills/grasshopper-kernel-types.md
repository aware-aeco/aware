---
name: grasshopper-grasshopper-kernel-types
description: This skill encodes the grasshopper 7.0 surface of the Grasshopper.Kernel.Types namespace — 70 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_Arc, GH_Boolean, GH_Box.GH_BoxProxy, GH_Box, GH_Brep, GH_Brep.GH_BrepProxy, GH_Circle.GH_CircleProxy, GH_Circle, and 62 more types.
---

# Grasshopper.Kernel.Types

Auto-generated from vendor docs for grasshopper 7.0. 70 types in this namespace.

## Complex (struct)

Complex number type in Grasshopper. Do not confuse with GH_Complex which is the IGH_Goo implementation of ComplexNumber.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Complex.htm)

### Constructors
- `public Complex(double real_component)` — Initializes a new instance of the Complex class
- `public Complex(double real_component, double imaginary_component)` — Initializes a new instance of the Complex class

### Methods
#### `public Complex ACos()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_ACos.htm)

#### `public Complex ASin()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_ASin.htm)

#### `public Complex ATan()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_ATan.htm)

#### `public static Complex operator +(Complex A, Complex B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Types.Complex)
- `B` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Addition.htm)

#### `public static Complex operator +(Complex A, double B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Types.Complex)
- `B` (System.Double)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Addition_1.htm)

#### `public static Complex operator +(double A, Complex B)`



**Parameters:**
- `A` (System.Double)
- `B` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Addition_2.htm)

#### `public Complex CoTan()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_CoTan.htm)

#### `public Complex Cos()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Cos.htm)

#### `public Complex Cosecant()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Cosecant.htm)

#### `public static Complex operator /(Complex dividend, Complex divisor)`



**Parameters:**
- `dividend` (Grasshopper.Kernel.Types.Complex)
- `divisor` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Division.htm)

#### `public static Complex operator /(Complex dividend, double divisor)`



**Parameters:**
- `dividend` (Grasshopper.Kernel.Types.Complex)
- `divisor` (System.Double)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Division_1.htm)

#### `public static Complex operator /(double dividend, Complex divisor)`



**Parameters:**
- `dividend` (System.Double)
- `divisor` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Division_2.htm)

#### `public Complex Exponential()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Exponential.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_GetHashCode.htm)

#### `public Complex Log()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Log.htm)

#### `public Complex Log10()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Log10.htm)

#### `public double Modulus()`



**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Modulus.htm)

#### `public double ModulusSquared()`



**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_ModulusSquared.htm)

#### `public static Complex operator *(Complex A, Complex B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Types.Complex)
- `B` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Multiply.htm)

#### `public static Complex operator *(Complex A, double B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Types.Complex)
- `B` (System.Double)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Multiply_1.htm)

#### `public static Complex operator *(double A, Complex B)`



**Parameters:**
- `A` (System.Double)
- `B` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Multiply_2.htm)

#### `public Complex Power(Complex exponent)`



**Parameters:**
- `exponent` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Power.htm)

#### `public Complex Root(Complex rootexponent)`



**Parameters:**
- `rootexponent` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Root.htm)

#### `public Complex Secant()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Secant.htm)

#### `public Complex Sin()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Sin.htm)

#### `public Complex Square()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Square.htm)

#### `public Complex SquareRoot()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_SquareRoot.htm)

#### `public static Complex operator -(Complex A, Complex B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Types.Complex)
- `B` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Subtraction.htm)

#### `public static Complex operator -(Complex A, double B)`



**Parameters:**
- `A` (Grasshopper.Kernel.Types.Complex)
- `B` (System.Double)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Subtraction_1.htm)

#### `public static Complex operator -(double A, Complex B)`



**Parameters:**
- `A` (System.Double)
- `B` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_Subtraction_2.htm)

#### `public Complex Tan()`



**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_Tan.htm)

#### `public static Complex operator -(Complex subtrahend)`



**Parameters:**
- `subtrahend` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_UnaryNegation.htm)

#### `public static Complex operator +(Complex summand)`



**Parameters:**
- `summand` (Grasshopper.Kernel.Types.Complex)

**Returns:** `Complex` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Complex_op_UnaryPlus.htm)

### Properties
- `Argument` (Double, get/set) — Gets or sets the complex argument.
- `ComplexUnit` (Complex, get) — 
- `Imaginary` (Double, get/set) — Gets or sets the imaginary component of the Complex number.
- `Infinity` (Complex, get) — 
- `IsReal` (Boolean, get) — Gets whether this complex number only has a real component.
- `IsRealNonNegative` (Boolean, get) — Gets whether this complex number only has a positive real component.
- `IsValid` (Boolean, get) — Gets the validity of this number.
- `IsZero` (Boolean, get) — Gets whether or not this complex number equals {0.0; 0.0}
- `NaN` (Complex, get) — 
- `NegativeInfinity` (Complex, get) — 
- `Real` (Double, get/set) — Gets or sets the real component of the Complex number.
- `Zero` (Complex, get) — 

## GH_Arc (class)

Represents a 3D circular arc. GH_Arc re-implements the OpenNURBS OnArc class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Arc.htm)

### Constructors
- `public GH_Arc()` — Default constructor
- `public GH_Arc(Arc arc)` — Create a duplicate of another arc
- `public GH_Arc(GH_Arc other)` — Create a duplicate of another arc

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

(Overrides GH_GeometricGoo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_GeometricGoo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_CastTo__1.htm)

#### `public virtual void ClearCaches()`

Clears all caches. Typically if the geometry is referenced, this will erase the local copy. If your T is a value-type, you must override this function and specifically unset the local value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_Duplicate.htm)

#### `public GH_Arc DuplicateArc()`

Create a duplicate of this arc.

**Returns:** `GH_Arc` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_DuplicateArc.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

Create a duplicate of this arc.

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_DuplicateGeometry.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this arc. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public virtual bool LoadGeometry(RhinoDoc doc)`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.

**Parameters:**
- `doc` (RhinoDoc) — Document to use for loading.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry_1.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Arc_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — Gets the boundingbox for this geometry.
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — Gets a value indicating whether or not this geometry is currently loaded (assuming it is referenced). Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets the validity of this instance.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — Gets or sets the Guid of the object that is referenced. Not all IGH_GeometricGoo implementations support referenced geometry. The default implementation is to always return Guid.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the value of this type. Note that if the type has a ReferenceID this value might get destroyed in the future.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Boolean (class)

Represents a boolean value. GH_Boolean re-implements the framework System.Boolean type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Boolean.htm)

### Constructors
- `public GH_Boolean()` — Blank constructor
- `public GH_Boolean(bool bool)` — Initializes a new instance of the GH_Boolean class
- `public GH_Boolean(GH_Boolean other)` — Initializes a new instance of the GH_Boolean class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_Duplicate.htm)

#### `public GH_Boolean DuplicateBoolean()`



**Returns:** `GH_Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_DuplicateBoolean.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Boolean_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Booleans are always valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Box (class)

Represents a 3D oriented Box volume.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Box.htm)

### Constructors
- `public GH_Box()` — Initializes a new instance of the GH_Box class
- `public GH_Box(BoundingBox box)` — Initializes a new instance of the GH_Box class
- `public GH_Box(Box box)` — Initializes a new instance of the GH_Box class
- `public GH_Box(Guid id)` — Initializes a new instance of the GH_Box class
- `public GH_Box(GH_Box other)` — Initializes a new instance of the GH_Box class

### Methods
#### `public void AppendRenderGeometry(GH_RenderArgs args, RenderMaterial materialHint)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_RenderArgs)
- `materialHint` (RenderMaterial)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_AppendRenderGeometry.htm)

#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_BakeGeometry.htm)

#### `public Brep Brep()`

Differs from Rhino.Geometry.Brep.FromBox() because it Handles degenerate cases.

**Returns:** `Brep` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Brep.htm)

#### `public override bool CastFrom(Object source)`

(Overrides GH_GeometricGoo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_GeometricGoo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_CastTo__1.htm)

#### `public override void ClearCaches()`

(Overrides GH_GeometricGoo<T>.ClearCaches().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Duplicate.htm)

#### `public GH_Box DuplicateBox()`



**Returns:** `GH_Box` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_DuplicateBox.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

(Overrides GH_GeometricGoo<T>.DuplicateGeometry().)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_DuplicateGeometry.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_GeometricGoo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_EmitProxy.htm)

#### `public bool Geometry(ref Brep brp, ref Curve crv, ref Point pt)`

Convert the box to a GeometryBase instance.

**Parameters:**
- `brp` (Brep) — If the box is at most degenerate in one direction, the brp parameter will be filled out.
- `crv` (Curve) — If the box is degenerate in exactly two directions, the crv parameter will be filled out.
- `pt` (Point) — If the box is degenerate in all three directions, the pt parameter will be filled out.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Geometry.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public override bool LoadGeometry(RhinoDoc doc)`

(Overrides GH_GeometricGoo<T>.LoadGeometry(RhinoDoc).)

**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_LoadGeometry.htm)

#### `public Mesh Mesh()`



**Returns:** `Mesh` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Mesh.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsGeometryLoaded.)
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — (Overrides GH_GeometricGoo<T>.ReferenceID.)
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (Box, get/set) — (Overrides GH_GeometricGoo<T>.Value.)
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Box.GH_BoxProxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_Box.GH_BoxProxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Box_GH_BoxProxy.htm)

### Constructors
- `public GH_BoxProxy(GH_Box Value)` — Initializes a new instance of the GH_Box.GH_BoxProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Box_GH_BoxProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Area` (String, get) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `ObjectID` (String, get/set) — 
- `Origin` (GH_Point3d_Wrapper, get) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `Volume` (String, get) — 
- `XAxis` (GH_Vector3d_Wrapper, get) — 
- `XSize` (GH_Interval_Wrapper, get) — 
- `YAxis` (GH_Vector3d_Wrapper, get) — 
- `YSize` (GH_Interval_Wrapper, get) — 
- `ZAxis` (GH_Vector3d_Wrapper, get) — 
- `ZSize` (GH_Interval_Wrapper, get) — 

## GH_Brep (class)

Represents a 3D polysurface. GH_Brep wraps the functionality of the OpenNURBS OnBrep class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Brep.htm)

### Constructors
- `public GH_Brep()` — Initializes a new instance of the GH_Brep class
- `public GH_Brep(Brep brep)` — Initializes a new instance of the GH_Brep class
- `public GH_Brep(Guid id)` — Initializes a new instance of the GH_Brep class
- `public GH_Brep(GH_Brep other)` — Initializes a new instance of the GH_Brep class

### Methods
#### `public void AppendRenderGeometry(GH_RenderArgs args, RenderMaterial materialHint)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_RenderArgs)
- `materialHint` (RenderMaterial)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_AppendRenderGeometry.htm)

#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_BakeGeometry.htm)

#### `public static BoundingBox BrepTightBoundingBox(Brep in)`

Compute the boundingbox of a Brep object without meshing.

**Parameters:**
- `in` (Brep) — Brep to measure.

**Returns:** `BoundingBox` — The world aligned boundingbox of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_BrepTightBoundingBox.htm)

#### `public override bool CastFrom(Object source)`

(Overrides GH_GeometricGoo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_GeometricGoo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_CastTo__1.htm)

#### `public override void ClearCaches()`

(Overrides GH_GeometricGoo<T>.ClearCaches().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_ClearCaches.htm)

#### `public void DestroyPreviewMeshes()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_DestroyPreviewMeshes.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_Duplicate.htm)

#### `public GH_Brep DuplicateBrep()`



**Returns:** `GH_Brep` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_DuplicateBrep.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

(Overrides GH_GeometricGoo<T>.DuplicateGeometry().)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_DuplicateGeometry.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_GeometricGoo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_GetBoundingBox.htm)

#### `public Mesh[] GetPreviewMeshes()`



**Returns:** `Mesh[]` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_GetPreviewMeshes.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public override bool LoadGeometry(RhinoDoc doc)`

(Overrides GH_GeometricGoo<T>.LoadGeometry(RhinoDoc).)

**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_LoadGeometry.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsGeometryLoaded.)
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — (Overrides GH_GeometricGoo<T>.ReferenceID.)
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (Brep, get/set) — (Overrides GH_GeometricGoo<T>.Value.)
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Brep.GH_BrepProxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_Brep.GH_BrepProxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Brep_GH_BrepProxy.htm)

### Constructors
- `public GH_BrepProxy(GH_Brep Value)` — Initializes a new instance of the GH_Brep.GH_BrepProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_GH_BrepProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public override bool FromString(string in)`

(Overrides GH_GooProxy<T>.FromString(String).)

**Parameters:**
- `in` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Brep_GH_BrepProxy_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Area` (String, get) — 
- `EdgeCount` (Int32, get) — 
- `FaceCount` (Int32, get) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `LoopCount` (Int32, get) — 
- `Manifold` (Boolean, get) — 
- `ObjectID` (String, get/set) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Solid` (Boolean, get) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `VertexCount` (Int32, get) — 
- `Volume` (String, get) — 

## GH_Circle (class)

Represents a 3D circle. GH_Circle re-implements the OpenNURBS OnCircle class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Circle.htm)

### Constructors
- `public GH_Circle()` — Default constructor
- `public GH_Circle(Circle circle)` — Create a duplicate of another circle
- `public GH_Circle(GH_Circle other)` — Create a duplicate of another circle.

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

Local to Remote caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_CastTo__1.htm)

#### `public virtual void ClearCaches()`

Clears all caches. Typically if the geometry is referenced, this will erase the local copy. If your T is a value-type, you must override this function and specifically unset the local value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_Duplicate.htm)

#### `public GH_Circle DuplicateCircle()`

Create a duplicate of this circle.

**Returns:** `GH_Circle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_DuplicateCircle.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

Create a duplicate of this circle.

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_DuplicateGeometry.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this circle. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public virtual bool LoadGeometry(RhinoDoc doc)`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.

**Parameters:**
- `doc` (RhinoDoc) — Document to use for loading.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry_1.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the circle using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — Gets the boundingbox for this geometry.
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — Gets a value indicating whether or not this geometry is currently loaded (assuming it is referenced). Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets the validity of this instance. Circles are valid when their base planes are valid and when the radius is larger than zero.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — Gets or sets the Guid of the object that is referenced. Not all IGH_GeometricGoo implementations support referenced geometry. The default implementation is to always return Guid.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the value of this type. Note that if the type has a ReferenceID this value might get destroyed in the future.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Circle.GH_CircleProxy (class)

Proxy description of GH_Circle class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Circle_GH_CircleProxy.htm)

### Constructors
- `public GH_CircleProxy(GH_Circle nOwner)` — Initializes a new instance of the GH_Circle.GH_CircleProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Circle_GH_CircleProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Area` (String, get/set) — 
- `Center` (GH_Point3d_Wrapper, get) — 
- `Circumference` (String, get/set) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Normal` (GH_Vector3d_Wrapper, get) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Radius` (String, get/set) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_Colour (class)

Represents an ARGB colour. GH_Colour re-implements the framework System.Drawing.Color type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Colour.htm)

### Constructors
- `public GH_Colour()` — Blank constructor
- `public GH_Colour(Color colour)` — Initializes a new instance of the GH_Colour class
- `public GH_Colour(GH_Colour other)` — Initializes a new instance of the GH_Colour class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_Duplicate.htm)

#### `public GH_Colour DuplicateColour()`



**Returns:** `GH_Colour` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_DuplicateColour.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Colour_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_ComplexNumber (class)

Wraps up the Complex data type for IGH_Goo adherence.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_ComplexNumber.htm)

### Constructors
- `public GH_ComplexNumber()` — Blank constructor
- `public GH_ComplexNumber(Complex number)` — Initializes a new instance of the GH_ComplexNumber class
- `public GH_ComplexNumber(GH_ComplexNumber other)` — Initializes a new instance of the GH_ComplexNumber class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_Duplicate.htm)

#### `public GH_ComplexNumber DuplicateComplex()`



**Returns:** `GH_ComplexNumber` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_DuplicateComplex.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ComplexNumber_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. If either the real or the imaginary component are NaN, then it is considered invalid
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Culture (class)

Represents a globally unique identifier. GH_Guid re-implements the framework System.Guid type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Culture.htm)

### Constructors
- `public GH_Culture()` — Blank constructor
- `public GH_Culture(CultureInfo culture)` — Initializes a new instance of the GH_Culture class
- `public GH_Culture(GH_Culture other)` — Initializes a new instance of the GH_Culture class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_Duplicate.htm)

#### `public GH_Culture DuplicateCulture()`



**Returns:** `GH_Culture` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_DuplicateCulture.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Culture_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Guids are always valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Curve (class)

Represents a 3D spline curve.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Curve.htm)

### Constructors
- `public GH_Curve()` — Default constructor, creates an invalid curve.
- `public GH_Curve(Curve curve)` — Create a duplicate of an existing curve.
- `public GH_Curve(Guid ref_guid)` — Create a new referenced curve that references a Rhino curve object with the specified ID.
- `public GH_Curve(GH_Curve other)` — Create a duplicate of another GH_Curve. This constructor also copies reference data.
- `public GH_Curve(Guid ref_guid, int ref_edge)` — Create a new references edge curve that references an edge index in a Rhino Brep object.

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

Local to Remote caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_CastTo__1.htm)

#### `public override void ClearCaches()`

Clears all volatile caches for this instance. The boundingbox is cleared, and if the curve is referenced, the local instance of the curve is erased.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_Duplicate.htm)

#### `public GH_Curve DuplicateCurve()`

Create a duplicate of this curve.

**Returns:** `GH_Curve` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_DuplicateCurve.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

Create a duplicate of this curve.

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_DuplicateGeometry.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this curve. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public override bool LoadGeometry(RhinoDoc doc)`

If the curve is referenced and not yet loaded, attempts to load the curve.

**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — True if the curve is loaded (or if it was already loaded), False on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_LoadGeometry.htm)

#### `public bool MakeDeformable()`

Converts the local geometry into deformable nurbs curve geometry. This method will not destroy Reference data.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_MakeDeformable.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_ScriptVariable.htm)

#### `public override string ToString()`

Format the curve using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — Gets the load state of this curve object. The curve is considered to be loaded when the local OnCurve instance is not null.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets the validity of this curve. If the curve is referenced but cannot be loaded, the curve is not valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceEdge` (Int32, get/set) — Gets or sets the edge index for a referenced curve. If the index >= 0 then an edge reference is implied.
- `ReferenceID` (Guid, get/set) — (Overrides GH_GeometricGoo<T>.ReferenceID.)
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (Curve, get/set) — (Overrides GH_GeometricGoo<T>.Value.)
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Curve.GH_CurveProxy (class)

Proxy description of GH_Curve class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Curve_GH_CurveProxy.htm)

### Constructors
- `public GH_CurveProxy(GH_Curve owner)` — Initializes a new instance of the GH_Curve.GH_CurveProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_GH_CurveProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public override bool FromString(string in)`

(Overrides GH_GooProxy<T>.FromString(String).)

**Parameters:**
- `in` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Curve_GH_CurveProxy_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Closed` (Boolean, get) — 
- `Deformable` (Boolean, get) — 
- `Domain` (String, get) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Length` (String, get) — 
- `Linear` (Boolean, get) — 
- `ObjectID` (String, get/set) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `Periodic` (Boolean, get) — 
- `Planar` (Boolean, get) — 
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Type` (String, get) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_DifferentialSolver (enum)

Enumerates all implemented differential samplers.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_DifferentialSolver.htm)

### Values
- `None` = `0` — Unset solver, always returns a zero-length tensor.
- `Euler` = `1` — Euler differential solver.
- `RungeKutta2` = `2` — Runge-Kutta second order solver.
- `RungeKutta3` = `3` — Runge-Kutta third order solver.
- `RungeKutta4` = `4` — Runge-Kutta fourth order solver.

## GH_Field (class)

Represents a field of forces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Field.htm)

### Constructors
- `public GH_Field()` — Blank constructor.
- `public GH_Field(GH_Field other)` — Copy constructor.

### Methods
#### `public bool CastFrom(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_CastFrom.htm)

#### `public bool CastTo<T>(ref T target)`



**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_CastTo__1.htm)

#### `public double CurvatureAt(Point3d location)`



**Parameters:**
- `location` (Point3d)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_CurvatureAt.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_DrawViewportWires.htm)

#### `public IGH_Goo Duplicate()`



**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_Duplicate.htm)

#### `public IGH_GooProxy EmitProxy()`



**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_EmitProxy.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_Read.htm)

#### `public Object ScriptVariable()`



**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_ScriptVariable.htm)

#### `public Vector3d SolveStep(Point3d location, double factor, GH_DifferentialSolver method)`

Move a point through the field.

**Parameters:**
- `location` (Point3d) — Point to move from.
- `factor` (System.Double) — Multiplication factor to apply to sampling. A low factor results in more accurate sampling, but smaller steps.
- `method` (Grasshopper.Kernel.Types.GH_DifferentialSolver) — Estimator to use.

**Returns:** `Vector3d` — New point location.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_SolveStep.htm)

#### `public Point3dList SolveSteps(Point3d location, double accuracy, int count, GH_DifferentialSolver method)`

Move a point through a succession of steps.

**Parameters:**
- `location` (Point3d) — Point to move.
- `accuracy` (System.Double) — Rough accuracy.
- `count` (System.Int32) — Number of samples to take.
- `method` (Grasshopper.Kernel.Types.GH_DifferentialSolver) — Estimator to use.

**Returns:** `Point3dList` — A track (including the original location) of all the samples.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_SolveSteps.htm)

#### `public Vector3d TensorAt(Point3d location)`

Compute the field tensor at the given location as contributed by all the field elements.

**Parameters:**
- `location` (Point3d) — Location to sample field at.

**Returns:** `Vector3d` — The strength and direction of the field at the given location.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_TensorAt.htm)

#### `public string ToString()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Field_Write.htm)

### Properties
- `ClippingBox` (BoundingBox, get) — 
- `Elements` (List<IGH_FieldElement>, get) — Gets the list of elements of this field.
- `IsValid` (Boolean, get) — 
- `IsValidWhyNot` (String, get) — 
- `TypeDescription` (String, get) — 
- `TypeName` (String, get) — 

## GH_FieldElement (class)

Abstract implementation of IGH_FieldElement, derive from this class rather than implementing IGH_FieldElement to save yourself time and effort.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_FieldElement.htm)

### Constructors
- `protected GH_FieldElement()` — Initializes a new instance of the GH_FieldElement class

### Methods
#### `public virtual void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_DrawViewportMeshes.htm)

#### `public virtual void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_DrawViewportWires.htm)

#### `public abstract IGH_FieldElement Duplicate()`



**Returns:** `IGH_FieldElement` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_Duplicate.htm)

#### `public abstract Vector3d Force(Point3d location)`



**Parameters:**
- `location` (Point3d)

**Returns:** `Vector3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_Force.htm)

#### `public virtual bool IsCoincident(Point3d point, double tolerance)`



**Parameters:**
- `point` (Point3d)
- `tolerance` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_IsCoincident.htm)

#### `public virtual bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_Read.htm)

#### `public virtual bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_Write.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — 
- `ClippingBox` (BoundingBox, get) — 
- `ElementGuid` (Guid, get) — 
- `IsLimited` (Boolean, get/set) — 
- `Limits` (Box, get/set) — 

## GH_GeometricGoo<T> (class)

Abstract base implementation of IGH_GeometricGoo. If you implement IGH_GeometricGoo, use this for a booster.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_GeometricGoo_1.htm)

### Constructors
- `protected GH_GeometricGoo()` — Blank constructor, m_value will be set to default (null for reference types, zeroes for value types)
- `protected GH_GeometricGoo(T internal_data)` — Data constructor, m_value will be set to internal_data.

### Methods
#### `public override bool CastFrom(Object source)`

Attempt a cast from generic object.

**Remarks:** If False, the contents of this instance are not to be trusted.

**Parameters:**
- `source` (System.Object) — Reference to source of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public virtual void ClearCaches()`

Clears all caches. Typically if the geometry is referenced, this will erase the local copy. If your T is a value-type, you must override this function and specifically unset the local value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_ClearCaches.htm)

#### `public override IGH_Goo Duplicate()`

Make a complete duplicate of this instance. No shallow copies.

**Remarks:** Classes which implement this interface should also provide type specific Duplicate methods

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_Duplicate.htm)

#### `public abstract IGH_GeometricGoo DuplicateGeometry()`

Make a complete duplicate of this geometry. No shallow copies.

**Remarks:** Each class that implements this interface should also provide a type specific duplication method

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_DuplicateGeometry.htm)

#### `public override IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_EmitProxy.htm)

#### `public abstract BoundingBox GetBoundingBox(Transform xform)`

Compute an aligned boundingbox.

**Parameters:**
- `xform` (Transform) — Transformation to apply to geometry for BoundingBox computation.

**Returns:** `BoundingBox` — The world aligned boundingbox of the transformed geometry.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public virtual bool LoadGeometry(RhinoDoc doc)`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.

**Parameters:**
- `doc` (RhinoDoc) — Document to use for loading.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry_1.htm)

#### `public abstract IGH_GeometricGoo Morph(SpaceMorph xmorph)`

Morph the object or a deformable representation of the object.

**Parameters:**
- `xmorph` (SpaceMorph) — Spatial deform.

**Returns:** `IGH_GeometricGoo` — Deformed geometry. If the local geometry can be deformed accurately, then the returned instance equals this instance. Not all geometry types can be accurately deformed though, if this is the case, this function will return an instance of another IGH_GeometricGoo derived type which can be deformed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_Morph.htm)

#### `public virtual bool Read(GH_IReader reader)`

Default behaviour is to return True.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public abstract string ToString()`

Creates a string description of the current instance value.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ToString.htm)

#### `public abstract IGH_GeometricGoo Transform(Transform xform)`

Transforms the object or a deformable representation of the object.

**Parameters:**
- `xform` (Transform) — Transformation matrix.

**Returns:** `IGH_GeometricGoo` — Transformed geometry. If the local geometry can be transformed accurately, then the returned instance equals this instance. Not all geometry types can be accurately transformed under all circumstances though, if this is the case, this function will return an instance of another IGH_GeometricGoo derived type which can be transformed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_Transform.htm)

#### `public virtual bool Write(GH_IWriter writer)`

Default behaviour is to return True.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — Gets the boundingbox for this geometry.
- `IsGeometryLoaded` (Boolean, get) — Gets a value indicating whether or not this geometry is currently loaded (assuming it is referenced). Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets a value indicating whether or not the current value is valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — Gets or sets the Guid of the object that is referenced. Not all IGH_GeometricGoo implementations support referenced geometry. The default implementation is to always return Guid.Empty.
- `TypeDescription` (String, get) — Gets a description of the type of the implementation.
- `TypeName` (String, get) — Gets the name of the type of the implementation.
- `Value` (T, get/set) — Gets or sets the value of this type. Note that if the type has a ReferenceID this value might get destroyed in the future.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_GeometricGooWrapper (class)

Utility class for maintaining all kinds of IGH_GeometricGoo types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_GeometricGooWrapper.htm)

### Constructors
- `public GH_GeometricGooWrapper()` — Initializes a new instance of the GH_GeometricGooWrapper class
- `public GH_GeometricGooWrapper(IGH_GeometricGoo goo)` — Initializes a new instance of the GH_GeometricGooWrapper class

### Methods
#### `public bool CastFrom(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_CastFrom.htm)

#### `public bool CastTo<T>(ref T target)`



**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_CastTo__1.htm)

#### `public void ClearCaches()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_ClearCaches.htm)

#### `public IGH_Goo Duplicate()`



**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_Duplicate.htm)

#### `public IGH_GeometricGoo DuplicateGeometry()`



**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_DuplicateGeometry.htm)

#### `public IGH_GooProxy EmitProxy()`



**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_EmitProxy.htm)

#### `public BoundingBox GetBoundingBox(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_GetBoundingBox.htm)

#### `public bool LoadGeometry()`



**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_LoadGeometry.htm)

#### `public bool LoadGeometry(RhinoDoc doc)`



**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_LoadGeometry_1.htm)

#### `public IGH_GeometricGoo Morph(SpaceMorph xmorph)`



**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_Morph.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_Read.htm)

#### `public Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disapears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The object that represents this IGH_Goo instance in a script.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_ToString.htm)

#### `public IGH_GeometricGoo Transform(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_Transform.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGooWrapper_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — 
- `InternalGoo` (IGH_GeometricGoo, get) — 
- `IsGeometryLoaded` (Boolean, get) — 
- `IsReferencedGeometry` (Boolean, get) — 
- `IsValid` (Boolean, get) — 
- `IsValidWhyNot` (String, get) — 
- `ReferenceID` (Guid, get/set) — 
- `TypeDescription` (String, get) — 
- `TypeName` (String, get) — 

## GH_GeometryGroup (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_GeometryGroup.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_GeometryGroup.htm)

### Constructors
- `public GH_GeometryGroup()` — Initializes a new instance of the GH_GeometryGroup class

### Methods
#### `public void AppendRenderGeometry(GH_RenderArgs args, RenderMaterial material)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_RenderArgs)
- `material` (RenderMaterial)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_AppendRenderGeometry.htm)

#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_BakeGeometry.htm)

#### `public bool CastFrom(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_CastFrom.htm)

#### `public bool CastTo<T>(ref T target)`



**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_CastTo__1.htm)

#### `public void ClearCaches()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_DrawViewportWires.htm)

#### `public IGH_Goo Duplicate()`



**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_Duplicate.htm)

#### `public IGH_GeometricGoo DuplicateGeometry()`



**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_DuplicateGeometry.htm)

#### `public IGH_GooProxy EmitProxy()`



**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_EmitProxy.htm)

#### `public BoundingBox GetBoundingBox(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_GetBoundingBox.htm)

#### `public bool LoadGeometry()`



**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_LoadGeometry.htm)

#### `public bool LoadGeometry(RhinoDoc doc)`



**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_LoadGeometry_1.htm)

#### `public IGH_GeometricGoo Morph(SpaceMorph xmorph)`



**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_Morph.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_Read.htm)

#### `public Object ScriptVariable()`



**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_ToString.htm)

#### `public IGH_GeometricGoo Transform(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_Transform.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometryGroup_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — 
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — 
- `IsReferencedGeometry` (Boolean, get) — 
- `IsValid` (Boolean, get) — 
- `IsValidWhyNot` (String, get) — 
- `Objects` (List<IGH_GeometricGoo>, get) — 
- `ReferenceID` (Guid, get/set) — 
- `TypeDescription` (String, get) — 
- `TypeName` (String, get) — 

## GH_GeometryGroup.GH_GeometryGroupProxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_GeometryGroup.GH_GeometryGroupProxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_GeometryGroup_GH_GeometryGroupProxy.htm)

### Constructors
- `public GH_GeometryGroupProxy(GH_GeometryGroup owner)` — Initializes a new instance of the GH_GeometryGroup.GH_GeometryGroupProxy class

### Methods
#### `public virtual void Construct()`

Override this method to supply a custom UI during proxy construction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_Goo<T> (class)

Base class for IGH_Goo implementation. Takes care of some default behaviour.

**Remarks:** Feel free to implement IGH_Goo directly though.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Goo_1.htm)

### Constructors
- `protected GH_Goo()` — Blank constructor, m_value will be set to default (null for reference types, zeroes for value types)
- `protected GH_Goo(T internal_data)` — Data constructor, m_value will be set to internal_data.
- `protected GH_Goo(GH_Goo<T> other)` — Copy constructor, copies the data from another GH_Goo(Of T) instance. Reference types will not be duplicated.

### Methods
#### `public virtual bool CastFrom(Object source)`

Attempt a cast from generic data.

**Remarks:** If False, the contents of this instance are not to be trusted.

**Parameters:**
- `source` (System.Object) — Reference to data that should be cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public abstract IGH_Goo Duplicate()`

Make a complete duplicate of this instance. No shallow copies.

**Remarks:** Classes which implement this interface should also provide type specific Duplicate methods

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_Duplicate.htm)

#### `public virtual IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_EmitProxy.htm)

#### `public virtual bool Read(GH_IReader reader)`

Default behaviour is to return True.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public abstract string ToString()`

Creates a string description of the current instance value.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ToString.htm)

#### `public virtual bool Write(GH_IWriter writer)`

Default behaviour is to return True.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets a value indicating whether or not the current value is valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — Gets a description of the type of the implementation.
- `TypeName` (String, get) — Gets the name of the type of the implementation.
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — 

## GH_GooProxy<T> (class)

Abstract base implementation of IGH_GooProxy

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_GooProxy_1.htm)

### Constructors
- `protected GH_GooProxy(T owner)` — Initializes a new instance of the GH_GooProxy<T> class

### Methods
#### `public virtual void Construct()`

Override this method to supply a custom UI during proxy construction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`



**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`



**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Owner` (T, get) — 
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — 
- `TypeName` (String, get) — 
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_Guid (class)

Represents a globally unique identifier. GH_Guid re-implements the framework System.Guid type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Guid.htm)

### Constructors
- `public GH_Guid()` — Blank constructor
- `public GH_Guid(Guid id)` — Initializes a new instance of the GH_Guid class
- `public GH_Guid(GH_Guid other)` — Initializes a new instance of the GH_Guid class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_Duplicate.htm)

#### `public GH_Guid DuplicateGuid()`



**Returns:** `GH_Guid` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_DuplicateGuid.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Guid_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Guids are always valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Integer (class)

Represents a 32-bit signed integer. GH_Integer re-implements the framework System.Int32 type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Integer.htm)

### Constructors
- `public GH_Integer()` — Blank constructor
- `public GH_Integer(int number)` — Initializes a new instance of the GH_Integer class
- `public GH_Integer(GH_Integer other)` — Initializes a new instance of the GH_Integer class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_Duplicate.htm)

#### `public GH_Integer DuplicateInteger()`



**Returns:** `GH_Integer` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_DuplicateInteger.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Integers are always valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Interval (class)

Represents a one-dimensional numeric domain.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Interval.htm)

### Constructors
- `public GH_Interval()` — Default constructor
- `public GH_Interval(Interval interval)` — Create a duplicate of another interval
- `public GH_Interval(GH_Interval other)` — Create a duplicate from another interval.

### Methods
#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

Local to Remote caster function. this stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

Create a duplicate of this interval.

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_Duplicate.htm)

#### `public GH_Interval DuplicateInterval()`

Create a duplicate of this interval.

**Returns:** `GH_Interval` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_DuplicateInterval.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this interval. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the interval using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Intervals are invalid if either of the extremes is a #NaN.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Interval2D (class)

Represents a two-dimensional numeric domain.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Interval2D.htm)

### Constructors
- `public GH_Interval2D()` — Default constructor. Creates two empty intervals.
- `public GH_Interval2D(GH_Interval2D other)` — Create a duplicate of another two-dimensional interval
- `public GH_Interval2D(UVInterval uv)` — Duplicates a one-dimensional interval in both directions.

### Methods
#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(ref T target)`

Local to Remote caster function. this stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

Create a duplicate of this interval.

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_Duplicate.htm)

#### `public GH_Interval2D DuplicateInterval()`

Create a duplicate of this interval.

**Returns:** `GH_Interval2D` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_DuplicateInterval.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this interval. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the interval using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Interval2D_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Valid intervals contain no #NaN extremes
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Interval2D.GH_Interval2DProxy (class)

Proxy description of GH_Interval class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Interval2D_GH_Interval2DProxy.htm)

### Constructors
- `public GH_Interval2DProxy(GH_Interval2D nValue)` — Initializes a new instance of the GH_Interval2D.GH_Interval2DProxy class

### Methods
#### `public virtual void Construct()`

Override this method to supply a custom UI during proxy construction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Area` (Double, get) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `U0` (Double, get/set) — 
- `U1` (Double, get/set) — 
- `UIsIncreasing` (Boolean, get/set) — 
- `UIsNormalized` (Boolean, get/set) — 
- `ULength` (Double, get) — 
- `UMiddle` (Double, get) — 
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `V0` (Double, get/set) — 
- `V1` (Double, get/set) — 
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `VIsIncreasing` (Boolean, get/set) — 
- `VIsNormalized` (Boolean, get/set) — 
- `VLength` (Double, get) — 
- `VMiddle` (Double, get) — 

## GH_Line (class)

Represents a 3D line segment. GH_Line re-implements the OpenNURBS OnLine class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Line.htm)

### Constructors
- `public GH_Line()` — Default constructor
- `public GH_Line(Line line)` — Create a duplicate of another line.
- `public GH_Line(GH_Line other)` — Create a duplicate of another line.

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

Local to Remote caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_CastTo__1.htm)

#### `public virtual void ClearCaches()`

Clears all caches. Typically if the geometry is referenced, this will erase the local copy. If your T is a value-type, you must override this function and specifically unset the local value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_Duplicate.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

Create a duplicate of this line.

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_DuplicateGeometry.htm)

#### `public GH_Line DuplicateLine()`

Create a duplicate of this line.

**Returns:** `GH_Line` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_DuplicateLine.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this line. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public virtual bool LoadGeometry(RhinoDoc doc)`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.

**Parameters:**
- `doc` (RhinoDoc) — Document to use for loading.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry_1.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the line using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — Gets a value indicating whether or not this geometry is currently loaded (assuming it is referenced). Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets the validity of this instance. Lines with zero length are not considered to be valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — Gets or sets the Guid of the object that is referenced. Not all IGH_GeometricGoo implementations support referenced geometry. The default implementation is to always return Guid.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the value of this type. Note that if the type has a ReferenceID this value might get destroyed in the future.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Line.GH_LineProxy (class)

Proxy description of GH_Line class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Line_GH_LineProxy.htm)

### Constructors
- `public GH_LineProxy(GH_Line nOwner)` — Initializes a new instance of the GH_Line.GH_LineProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Line_GH_LineProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Ax` (String, get/set) — 
- `Ay` (String, get/set) — 
- `Az` (String, get/set) — 
- `Bx` (String, get/set) — 
- `By` (String, get/set) — 
- `Bz` (String, get/set) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Length` (String, get/set) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_LineCharge (class)

Line charge implementation for IGH_Fields.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_LineCharge.htm)

### Constructors
- `public GH_LineCharge()` — Initializes a new instance of the GH_LineCharge class

### Methods
#### `public virtual void DrawViewportMeshes(GH_PreviewMeshArgs args)`

(Inherited from GH_FieldElement.)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_DrawViewportMeshes.htm)

#### `public override void DrawViewportWires(GH_PreviewWireArgs args)`

(Overrides GH_FieldElement.DrawViewportWires(GH_PreviewWireArgs).)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LineCharge_DrawViewportWires.htm)

#### `public override IGH_FieldElement Duplicate()`

(Overrides GH_FieldElement.Duplicate().)

**Returns:** `IGH_FieldElement` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LineCharge_Duplicate.htm)

#### `public override Vector3d Force(Point3d location)`

(Overrides GH_FieldElement.Force(Point3d).)

**Parameters:**
- `location` (Point3d)

**Returns:** `Vector3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LineCharge_Force.htm)

#### `public override bool IsCoincident(Point3d point, double tolerance)`

(Overrides GH_FieldElement.IsCoincident(Point3d, Double).)

**Parameters:**
- `point` (Point3d)
- `tolerance` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LineCharge_IsCoincident.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_FieldElement.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LineCharge_Read.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_FieldElement.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LineCharge_Write.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — (Overrides GH_FieldElement.BoundingBox.)
- `Charge` (Double, get/set) — 
- `ClippingBox` (BoundingBox, get) — (Inherited from GH_FieldElement.)
- `ElementGuid` (Guid, get) — (Overrides GH_FieldElement.ElementGuid.)
- `IsLimited` (Boolean, get/set) — (Inherited from GH_FieldElement.)
- `Limits` (Box, get/set) — (Inherited from GH_FieldElement.)
- `Segment` (Line, get/set) — 

## GH_LonLatCoordinate (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_LonLatCoordinate.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_LonLatCoordinate.htm)

### Constructors
- `public GH_LonLatCoordinate()` — Initializes a new instance of the GH_LonLatCoordinate class
- `public GH_LonLatCoordinate(double longitude, double latitude)` — Initializes a new instance of the GH_LonLatCoordinate class

### Methods
#### `public bool CastFrom(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_CastFrom.htm)

#### `public bool CastTo<T>(ref T target)`



**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_CastTo__1.htm)

#### `public IGH_Goo Duplicate()`



**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_Duplicate.htm)

#### `public IGH_GooProxy EmitProxy()`



**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_EmitProxy.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_Read.htm)

#### `public Object ScriptVariable()`



**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_LonLatCoordinate_Write.htm)

### Properties
- `IsValid` (Boolean, get) — 
- `IsValidWhyNot` (String, get) — 
- `Latitude` (Double, get/set) — Gets or sets the longitude (in degrees) of the location.
- `Longitude` (Double, get/set) — Gets or sets the longitude (in degrees) of the location.
- `SphereUParameter` (Double, get) — Gets the spherical U parameter.
- `SphereVParameter` (Double, get) — Gets the spherical V parameter.
- `TypeDescription` (String, get) — 
- `TypeName` (String, get) — 

## GH_Material (class)

Represents a implementation of the Rhino material

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Material.htm)

### Constructors
- `public GH_Material()` — Initializes a new instance of the GH_Material class
- `public GH_Material(DisplayMaterial material)` — Create a new Material.
- `public GH_Material(RenderMaterial material)` — Create a new Material.
- `public GH_Material(Color diffuse)` — Create a new material based on a colour. The colour is used to control Diffuse, Emission and Transparency. Ambient, Specular and Shine are always default.
- `public GH_Material(Guid rdkId)` — Create a new Material from an RDK material ID.
- `public GH_Material(string rdkXml)` — Create a new Material from an RDK xml string.
- `public GH_Material(GH_Material other)` — Copy another GH_Material instance.

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(ref T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_CastTo__1.htm)

#### `public static Material CreateRhinoMaterial(Color colour)`

Creates a standard Rhino material from a colour.

**Parameters:**
- `colour` (System.Drawing.Color) — Base colour for material.

**Returns:** `Material` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_CreateRhinoMaterial.htm)

#### `public static DisplayMaterial CreateStandardMaterial(Color colour)`

Creates a standard Grasshopper material from a colour.

**Parameters:**
- `colour` (System.Drawing.Color) — Base colour for material.

**Returns:** `DisplayMaterial` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_CreateStandardMaterial.htm)

#### `public override IGH_Goo Duplicate()`

Duplicate this material.

**Returns:** `IGH_Goo` — An exact duplicate of this material.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_Duplicate.htm)

#### `public GH_Material DuplicateMaterial()`

Duplicate this material.

**Returns:** `GH_Material` — An exact duplicate of this material.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_DuplicateMaterial.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_EmitProxy.htm)

#### `public RenderMaterial MaterialBestGuess()`

Gets the best guess RenderMaterial for this shader.

**Returns:** `RenderMaterial` — Material which best describes this shader.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_MaterialBestGuess.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Material_Write.htm)

### Properties
- `IsValid` (Boolean, get) — (Overrides GH_Goo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `RdkMaterialId` (Guid, get/set) — Gets or sets the RDK material ID this material is associated with.
- `RdkMaterialRmtl` (String, get/set) — Gets or sets the RDK material RMTL file path.
- `RdkMaterialXml` (String, get/set) — Gets or sets the RDK material Xml string.
- `Type` (GH_Material.MaterialType, get) — Gets the implied type based on what fields have been assigned.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Material.GH_Material_Proxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_Material.GH_Material_Proxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Material_GH_Material_Proxy.htm)

### Constructors
- `public GH_Material_Proxy(GH_Material nValue)` — Initializes a new instance of the GH_Material.GH_Material_Proxy class

### Methods
#### `public virtual void Construct()`

Override this method to supply a custom UI during proxy construction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Diffuse` (Color, get/set) — 
- `Emission` (Color, get/set) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `IsRDKMaterial` (Boolean, get) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Shine` (Double, get/set) — 
- `Specular` (Color, get/set) — 
- `Transparency` (Double, get/set) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_Material.MaterialType (enum)

Enumerates the possible material types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Material_MaterialType.htm)

### Values
- `Shader` = `0` — Display shader, very basic colour information, nothing more.
- `RhinoMaterial` = `1` — A material from the Rhino document, identified by ID.
- `XmlMaterial` = `2` — A material defined by an RDK Xml string.
- `RmtlMaterial` = `3` — A material loaded from an RMTL file.

## GH_Matrix (class)

Represents a rectangular numeric matrix. GH_Integer re-implements Rhino.Geometry.Matrix.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Matrix.htm)

### Constructors
- `public GH_Matrix()` — Blank constructor
- `public GH_Matrix(Matrix matrix)` — Initializes a new instance of the GH_Matrix class
- `public GH_Matrix(GH_Matrix other)` — Initializes a new instance of the GH_Matrix class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_CastTo__1.htm)

#### `public static Matrix CloneMatrix(Matrix matrix)`

Duplicates a Matrix.

**Parameters:**
- `matrix` (Matrix)

**Returns:** `Matrix` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_CloneMatrix.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_Duplicate.htm)

#### `public GH_Matrix DuplicateMatrix()`



**Returns:** `GH_Matrix` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_DuplicateMatrix.htm)

#### `public virtual IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Matrix_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Integers are always valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Mesh (class)

Represents a 3D polygonal mesh composed of quads and tris. GH_MEsh wraps the functionality of the OpenNURBS OnMesh class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Mesh.htm)

### Constructors
- `public GH_Mesh()` — Initializes a new instance of the GH_Mesh class
- `public GH_Mesh(Mesh mesh)` — Initializes a new instance of the GH_Mesh class
- `public GH_Mesh(Guid id)` — Initializes a new instance of the GH_Mesh class
- `public GH_Mesh(GH_Mesh other)` — Initializes a new instance of the GH_Mesh class

### Methods
#### `public void AppendRenderGeometry(GH_RenderArgs args, RenderMaterial materialHint)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_RenderArgs)
- `materialHint` (RenderMaterial)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_AppendRenderGeometry.htm)

#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

(Overrides GH_GeometricGoo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_GeometricGoo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_CastTo__1.htm)

#### `public override void ClearCaches()`

(Overrides GH_GeometricGoo<T>.ClearCaches().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

Make a complete duplicate of this instance. No shallow copies.

**Remarks:** Classes which implement this interface should also provide type specific Duplicate methods

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_Duplicate.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

(Overrides GH_GeometricGoo<T>.DuplicateGeometry().)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_DuplicateGeometry.htm)

#### `public GH_Mesh DuplicateMesh()`



**Returns:** `GH_Mesh` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_DuplicateMesh.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_GeometricGoo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public override bool LoadGeometry(RhinoDoc doc)`

(Overrides GH_GeometricGoo<T>.LoadGeometry(RhinoDoc).)

**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_LoadGeometry.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsGeometryLoaded.)
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — (Overrides GH_GeometricGoo<T>.ReferenceID.)
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the value of this type. Note that if the type has a ReferenceID this value might get destroyed in the future.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Mesh.GH_MeshProxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_Mesh.GH_MeshProxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Mesh_GH_MeshProxy.htm)

### Constructors
- `public GH_MeshProxy(GH_Mesh Value)` — Initializes a new instance of the GH_Mesh.GH_MeshProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Mesh_GH_MeshProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Area` (String, get) — 
- `EdgeCount` (Int32, get) — 
- `FaceCount` (Int32, get) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Manifold` (Boolean, get) — 
- `ObjectID` (String, get/set) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Solid` (Boolean, get) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `VertexCount` (Int32, get) — 
- `Volume` (String, get) — 

## GH_MeshFace (class)

Represents a tri or quad mesh face. GH_MeshFace works similar to the OpenNURBS OnMeshFace class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_MeshFace.htm)

### Constructors
- `public GH_MeshFace()` — Default constructor. Sets all corners to 0.
- `public GH_MeshFace(MeshFace other)` — Create a duplicate of another mesh face.
- `public GH_MeshFace(GH_MeshFace other)` — Create a duplicate of another mesh face
- `public GH_MeshFace(int nA, int nB, int nC)` — Create a triangular mesh face from 3 corners indices.
- `public GH_MeshFace(int nA, int nB, int nC, int nD)` — Create a quadrangular mesh face from 4 corners indices.

### Methods
#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(ref T target)`

Local to Remote caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

Create a duplicate of this Mesh face.

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_Duplicate.htm)

#### `public GH_MeshFace DuplicateMeshFace()`

Create a duplicate of this Mesh face.

**Returns:** `GH_MeshFace` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_DuplicateMeshFace.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this mesh face. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the mesh face using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshFace_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Test for validity. No negative indices are allowed and only C and D are allowed to be identical.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_MeshFace.GH_MeshFaceProxy (class)

Proxy description of GH_MeshFace class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_MeshFace_GH_MeshFaceProxy.htm)

### Constructors
- `public GH_MeshFaceProxy(GH_MeshFace nValue)` — Initializes a new instance of the GH_MeshFace.GH_MeshFaceProxy class

### Methods
#### `public virtual void Construct()`

Override this method to supply a custom UI during proxy construction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `A` (Int32, get/set) — 
- `B` (Int32, get/set) — 
- `C` (Int32, get/set) — 
- `D` (Int32, get/set) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Kind` (String, get) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_MeshingParameters (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_MeshingParameters.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_MeshingParameters.htm)

### Constructors
- `public GH_MeshingParameters()` — Initializes a new instance of the GH_MeshingParameters class
- `public GH_MeshingParameters(MeshingParameters other)` — Initializes a new instance of the GH_MeshingParameters class

### Methods
#### `public virtual bool CastFrom(Object source)`

Attempt a cast from generic data.

**Remarks:** If False, the contents of this instance are not to be trusted.

**Parameters:**
- `source` (System.Object) — Reference to data that should be cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshingParameters_Duplicate.htm)

#### `public GH_MeshingParameters DuplicateMesherSettings()`



**Returns:** `GH_MeshingParameters` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshingParameters_DuplicateMesherSettings.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshingParameters_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshingParameters_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshingParameters_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_MeshingParameters_Write.htm)

### Properties
- `IsValid` (Boolean, get) — (Overrides GH_Goo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_MeshingParameters.RhMesherSettings_Proxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_MeshingParameters.RhMesherSettings_Proxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_MeshingParameters_RhMesherSettings_Proxy.htm)

### Constructors
- `public RhMesherSettings_Proxy(GH_MeshingParameters owner)` — Initializes a new instance of the GH_MeshingParameters.RhMesherSettings_Proxy class

### Methods
#### `public virtual void Construct()`

Override this method to supply a custom UI during proxy construction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `AngleTolerance` (Double, get/set) — 
- `AspectRatio` (Double, get/set) — 
- `GridMaxCount` (Int32, get/set) — 
- `GridMinCount` (Int32, get/set) — 
- `IsParsable` (Boolean, get) — (Overrides GH_GooProxy<T>.IsParsable.)
- `JaggedSeams` (Boolean, get/set) — 
- `MaxEdgeLength` (Double, get/set) — 
- `MinEdgeLength` (Double, get/set) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Refine` (Boolean, get/set) — 
- `SimplePlanes` (Boolean, get/set) — 
- `Tolerance` (Double, get/set) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## GH_Number (class)

Represents a double-precision floating point number. GH_Number re-implements the framework System.Double type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Number.htm)

### Constructors
- `public GH_Number()` — Blank constructor
- `public GH_Number(double number)` — Initializes a new instance of the GH_Number class
- `public GH_Number(GH_Number other)` — Initializes a new instance of the GH_Number class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_Duplicate.htm)

#### `public GH_Number DuplicateNumber()`



**Returns:** `GH_Number` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_DuplicateNumber.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Number_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. If the number is NaN, then it is considered invalid
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_ObjectWrapper (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_ObjectWrapper.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_ObjectWrapper.htm)

### Constructors
- `public GH_ObjectWrapper()` — Initializes a new instance of the GH_ObjectWrapper class
- `public GH_ObjectWrapper(Object obj)` — Initializes a new instance of the GH_ObjectWrapper class

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_CastTo__1.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_Duplicate.htm)

#### `public GH_ObjectWrapper DuplicateObject()`



**Returns:** `GH_ObjectWrapper` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_DuplicateObject.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_ObjectWrapper_ToString.htm)

#### `public virtual bool Write(GH_IWriter writer)`

Default behaviour is to return True.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_Write.htm)

### Properties
- `ClippingBox` (BoundingBox, get) — 
- `IsValid` (Boolean, get) — (Overrides GH_Goo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Plane (class)

Represents a 3D plane. GH_Plane re-implements the OpenNURBS OnPlane class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Plane.htm)

### Constructors
- `public GH_Plane()` — Default constructor. Creates a world XY plane.
- `public GH_Plane(Plane plane)` — Create a duplicate of another plane.
- `public GH_Plane(Guid id)` — Initializes a new instance of the GH_Plane class
- `public GH_Plane(GH_Plane other)` — Create a duplicate of another plane.

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

Local to Remote caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_CastTo__1.htm)

#### `public virtual void ClearCaches()`

Clears all caches. Typically if the geometry is referenced, this will erase the local copy. If your T is a value-type, you must override this function and specifically unset the local value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_ClearCaches.htm)

#### `public static void DrawPlane(DisplayPipeline display, Plane plane)`

Draw a plane with default colours and default size.

**Parameters:**
- `display` (DisplayPipeline)
- `plane` (Plane)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DrawPlane.htm)

#### `public static void DrawPlane(DisplayPipeline display, Plane plane, double size, int frequency)`



**Parameters:**
- `display` (DisplayPipeline)
- `plane` (Plane)
- `size` (System.Double)
- `frequency` (System.Int32)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DrawPlane_1.htm)

#### `public static void DrawPlane(DisplayPipeline display, Plane plane, double size, int frequency, Color grid_color, Color x_color, Color y_color)`



**Parameters:**
- `display` (DisplayPipeline)
- `plane` (Plane)
- `size` (System.Double)
- `frequency` (System.Int32)
- `grid_color` (System.Drawing.Color)
- `x_color` (System.Drawing.Color)
- `y_color` (System.Drawing.Color)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DrawPlane_2.htm)

#### `public static void DrawPlane(DisplayPipeline display, Plane plane, double size, int frequency, Color grid_color, Color x_color, Color y_color, Color back_color)`



**Parameters:**
- `display` (DisplayPipeline)
- `plane` (Plane)
- `size` (System.Double)
- `frequency` (System.Int32)
- `grid_color` (System.Drawing.Color)
- `x_color` (System.Drawing.Color)
- `y_color` (System.Drawing.Color)
- `back_color` (System.Drawing.Color)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DrawPlane_3.htm)

#### `public static void DrawPlaneIcon(DisplayPipeline display, Plane plane, double size, Color edgeColour, Color fillColour)`



**Parameters:**
- `display` (DisplayPipeline)
- `plane` (Plane)
- `size` (System.Double)
- `edgeColour` (System.Drawing.Color)
- `fillColour` (System.Drawing.Color)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DrawPlaneIcon.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_Duplicate.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

Create a duplicate of this plane.

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DuplicateGeometry.htm)

#### `public GH_Plane DuplicatePlane()`

Create a duplicate of this plane.

**Returns:** `GH_Plane` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_DuplicatePlane.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this plane. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public override bool LoadGeometry(RhinoDoc doc)`

(Overrides GH_GeometricGoo<T>.LoadGeometry(RhinoDoc).)

**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_LoadGeometry.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the plane using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — Gets a value indicating whether or not this geometry is currently loaded (assuming it is referenced). Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets the validity of the plane. If the plane is invalid, attempts are made to fix the data. If the plane remains invalid, False is returned.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — (Overrides GH_GeometricGoo<T>.ReferenceID.)
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the value of this type. Note that if the type has a ReferenceID this value might get destroyed in the future.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Plane.GH_PlaneProxy (class)

Proxy description of GH_Plane class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Plane_GH_PlaneProxy.htm)

### Constructors
- `public GH_PlaneProxy(GH_Plane nOwner)` — Initializes a new instance of the GH_Plane.GH_PlaneProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Plane_GH_PlaneProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Origin` (GH_Point3d_Wrapper, get) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `XAxis` (GH_Vector3d_Wrapper, get) — 
- `YAxis` (GH_Vector3d_Wrapper, get) — 
- `ZAxis` (GH_Vector3d_Wrapper, get) — 

## GH_Point (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_Point.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Point.htm)

### Constructors
- `public GH_Point()` — Initializes a new instance of the GH_Point class
- `public GH_Point(Point3d pt)` — Initializes a new instance of the GH_Point class
- `public GH_Point(Guid rh_obj_id)` — Initializes a new instance of the GH_Point class
- `public GH_Point(GH_Point iOther)` — Initializes a new instance of the GH_Point class

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_BakeGeometry.htm)

#### `public bool CastFrom(Object source)`



**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CastFrom.htm)

#### `public bool CastTo<T>(out T target)`



**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CastTo__1.htm)

#### `public void ClearCaches()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ClearCaches.htm)

#### `public void CreateFromCoordinate(Point3d pt)`

Create a new point geometry instance based on a fixed coordinate

**Remarks:** This 'constructor' merely sets fields, it doesn't load the geometry and it doesn't perform validity checking.

**Parameters:**
- `pt` (Point3d) — The coordinate for the point

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CreateFromCoordinate.htm)

#### `public void CreateFromCurveDistance(Guid crv_id, Curve crv, double t, bool bFromStart)`

Create new point geometry instance based on a curve distance parameter

**Remarks:** This 'constructor' merely sets fields, it doesn't load the geometry and it doesn't perform validity checking.

**Parameters:**
- `crv_id` (System.Guid) — ID of the curve this object references
- `crv` (Curve) — The curve to act on
- `t` (System.Double) — Parameter on the curve that controls the distance measurement
- `bFromStart` (System.Boolean) — True if the distance is to be measured from the start of the curve

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CreateFromCurveDistance.htm)

#### `public void CreateFromCurveRatio(Guid crv_id, Curve crv, double t)`

Create new point geometry instance based on a curve parameter ratio

**Remarks:** This 'constructor' merely sets fields, it doesn't load the geometry and it doesn't perform validity checking.

**Parameters:**
- `crv_id` (System.Guid) — ID of the curve this object references
- `crv` (Curve)
- `t` (System.Double) — Parameter on curve in domain space

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CreateFromCurveRatio.htm)

#### `public void CreateFromEdgeDistance(Guid brp_id, Curve edge, int e_index, double t, bool bFromStart)`

Create new point geometry instance based on a curve distance parameter

**Remarks:** This 'constructor' merely sets fields, it doesn't load the geometry and it doesn't perform validity checking.

**Parameters:**
- `brp_id` (System.Guid) — ID of the curve this object references
- `edge` (Curve) — The edge curve
- `e_index` (System.Int32) — Index of edge to reference
- `t` (System.Double) — Parameter along the curve in curve domain space
- `bFromStart` (System.Boolean) — True if the distance is to be measured from the start of the curve

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CreateFromEdgeDistance.htm)

#### `public void CreateFromEdgeRatio(Guid brp_id, Curve edge, int e_index, double t)`

Create new point geometry instance based on an edge parameter ratio

**Remarks:** This 'constructor' merely sets fields, it doesn't load the geometry and it doesn't perform validity checking.

**Parameters:**
- `brp_id` (System.Guid) — ID of the brep this object references
- `edge` (Curve) — The edge curve
- `e_index` (System.Int32) — Index of edge to reference
- `t` (System.Double) — Parameter on edge in edge domain space

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CreateFromEdgeRatio.htm)

#### `public void CreateFromPointObject(Guid id)`

Create a new point geometry instance based on point object ID. Technically this now also accepts Dot object IDs.

**Remarks:** This 'constructor' merely sets fields, it doesn't load the geometry and it doesn't perform validity checking.

**Parameters:**
- `id` (System.Guid) — The identifier of the point object

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CreateFromPointObject.htm)

#### `public void CreateFromSurfaceParam(Guid brp_id, int f_index, Surface srf, double u, double v)`

Create new point geometry instance based on a curve distance parameter

**Remarks:** This 'constructor' merely sets fields, it doesn't load the geometry and it doesn't perform validity checking.

**Parameters:**
- `brp_id` (System.Guid) — ID of the curve this object references
- `f_index` (System.Int32) — Index of edge to reference
- `srf` (Surface) — Surface to work on
- `u` (System.Double) — Parameter in surface U direction
- `v` (System.Double) — Parameter in surface V direction

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_CreateFromSurfaceParam.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_DrawViewportWires.htm)

#### `public IGH_Goo Duplicate()`



**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_Duplicate.htm)

#### `public IGH_GeometricGoo DuplicateGeometry()`



**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_DuplicateGeometry.htm)

#### `public GH_Point DuplicatePoint()`



**Returns:** `GH_Point` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_DuplicatePoint.htm)

#### `public IGH_GooProxy EmitProxy()`



**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_EmitProxy.htm)

#### `public void EnsureReferenceData()`

Creates new Reference data if it doesn't already exists

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_EnsureReferenceData.htm)

#### `public BoundingBox GetBoundingBox(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_GetBoundingBox.htm)

#### `public bool LoadGeometry()`



**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_LoadGeometry.htm)

#### `public bool LoadGeometry(RhinoDoc doc)`



**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_LoadGeometry_1.htm)

#### `public IGH_GeometricGoo Morph(SpaceMorph xmorph)`



**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_Morph.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_Read.htm)

#### `public Curve ReferenceCurve()`



**Returns:** `Curve` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceCurve.htm)

#### `public Curve ReferenceCurve(RhinoObject ref)`



**Parameters:**
- `ref` (RhinoObject)

**Returns:** `Curve` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceCurve_1.htm)

#### `public int ReferenceIndex()`

Retrieve the index of the point reference data.

**Remarks:** If there exists no reference data, -1 is returned.

**Returns:** `Int32` — The index of the reference data. This index describes the sub-object of the reference.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceIndex.htm)

#### `public void ReferenceIndex(int new_index)`

Sets the index of the point reference data.

**Remarks:** If there is no reference data, this function will create a new instance.

**Parameters:**
- `new_index` (System.Int32) — The index to set. This index describes the sub-object of the reference.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceIndex_1.htm)

#### `public double ReferenceParam(int index)`

Retrieve the indexed param of the point reference data.

**Remarks:** If there exists no reference data, NaN is returned.

**Parameters:**
- `index` (System.Int32) — Index of parameter. Only 0 and 1 are valid values.

**Returns:** `Double` — The indexed param of the reference data.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceParam.htm)

#### `public void ReferenceParam(int index, double new_param)`

Sets the index of the point reference data.

**Remarks:** If there is no reference data, this function will create a new instance.

**Parameters:**
- `index` (System.Int32) — The index of the parameter. Only 0 and 1 are valid values.
- `new_param` (System.Double) — The new value of the parameter

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceParam_1.htm)

#### `public BrepFace ReferenceSurface()`



**Returns:** `BrepFace` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceSurface.htm)

#### `public GH_PointRefType ReferenceType()`

Retrieve the reference type of this point object.

**Returns:** `GH_PointRefType` — The reference type as specified by the Reference Data. If there is no reference data, either Unset or Coordinate is returned depending on whether m_Point is a null reference.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceType.htm)

#### `public void ReferenceType(GH_PointRefType new_type)`

Sets the type of the point reference data.

**Remarks:** If there is no reference data, this function will create a new instance.

**Parameters:**
- `new_type` (Grasshopper.Kernel.Types.GH_PointRefType) — The new type, if type = Coordinate or Unset, then the Reference data will be erased.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferenceType_1.htm)

#### `public bool ReferencesCurve()`

Returns true if this point is based on a curve reference

**Remarks:** See AlsoReferenceGH_Point ClassGrasshopper.Kernel.Types Namespace

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferencesCurve.htm)

#### `public bool ReferencesEdge()`

Returns True if this point is based on an edge reference

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ReferencesEdge.htm)

#### `public Object ScriptVariable()`



**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ScriptVariable.htm)

#### `public void SetReferenceParams(double u, double v)`

Sets both param values of the point reference

**Remarks:** If there is no reference data, this function will create a new instance.

**Parameters:**
- `u` (System.Double) — The new u parameter
- `v` (System.Double) — The new v parameter

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_SetReferenceParams.htm)

#### `public DialogResult ShowReferenceDialog()`



**Returns:** `DialogResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ShowReferenceDialog.htm)

#### `public DialogResult ShowReferenceDialog(IWin32Window owner)`



**Parameters:**
- `owner` (System.Windows.Forms.IWin32Window)

**Returns:** `DialogResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ShowReferenceDialog_1.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_ToString.htm)

#### `public IGH_GeometricGoo Transform(Transform xform)`



**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_Transform.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — 
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — 
- `IsReferencedGeometry` (Boolean, get) — 
- `IsValid` (Boolean, get) — 
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceData` (GH_PointRefData, get/set) — Gets or sets the reference data of this point geometry.
- `ReferenceID` (Guid, get/set) — 
- `TypeDescription` (String, get) — 
- `TypeName` (String, get) — 
- `Value` (Point3d, get/set) — 

## GH_Point.GH_PointProxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_Point.GH_PointProxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Point_GH_PointProxy.htm)

### Constructors
- `public GH_PointProxy(GH_Point Value)` — Initializes a new instance of the GH_Point.GH_PointProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_GH_PointProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected void SrfRefChanged(GH_PointRefUV_Wrapper sender, GH_PointRefData ref)`



**Parameters:**
- `sender` (GH_PointRefUV_Wrapper)
- `ref` (Grasshopper.Kernel.Types.GH_PointRefData)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Point_GH_PointProxy_SrfRefChanged.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Distance` (String, get/set) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Referenced` (Boolean, get) — 
- `ReferenceID` (String, get) — 
- `ReferenceType` (String, get) — 
- `TParameterRatio` (String, get/set) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `UVParameterRatio` (GH_PointRefUV_Wrapper, get) — 
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `X` (String, get/set) — 
- `Y` (String, get/set) — 
- `Z` (String, get/set) — 

## GH_PointCharge (class)

Point charge implementation for IGH_Fields.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_PointCharge.htm)

### Constructors
- `public GH_PointCharge()` — Initializes a new instance of the GH_PointCharge class

### Methods
#### `public virtual void DrawViewportMeshes(GH_PreviewMeshArgs args)`

(Inherited from GH_FieldElement.)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_DrawViewportMeshes.htm)

#### `public override void DrawViewportWires(GH_PreviewWireArgs args)`

(Overrides GH_FieldElement.DrawViewportWires(GH_PreviewWireArgs).)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointCharge_DrawViewportWires.htm)

#### `public override IGH_FieldElement Duplicate()`

(Overrides GH_FieldElement.Duplicate().)

**Returns:** `IGH_FieldElement` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointCharge_Duplicate.htm)

#### `public override Vector3d Force(Point3d location)`

(Overrides GH_FieldElement.Force(Point3d).)

**Parameters:**
- `location` (Point3d)

**Returns:** `Vector3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointCharge_Force.htm)

#### `public override bool IsCoincident(Point3d point, double tolerance)`

(Overrides GH_FieldElement.IsCoincident(Point3d, Double).)

**Parameters:**
- `point` (Point3d)
- `tolerance` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointCharge_IsCoincident.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_FieldElement.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointCharge_Read.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_FieldElement.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointCharge_Write.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — (Overrides GH_FieldElement.BoundingBox.)
- `Charge` (Double, get/set) — 
- `ClippingBox` (BoundingBox, get) — (Inherited from GH_FieldElement.)
- `Decay` (Double, get/set) — 
- `ElementGuid` (Guid, get) — (Overrides GH_FieldElement.ElementGuid.)
- `IsLimited` (Boolean, get/set) — (Inherited from GH_FieldElement.)
- `Limits` (Box, get/set) — (Inherited from GH_FieldElement.)
- `Location` (Point3d, get/set) — 

## GH_PointRefData (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_PointRefData.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_PointRefData.htm)

### Constructors
- `public GH_PointRefData()` — Initializes a new instance of the GH_PointRefData class
- `public GH_PointRefData(GH_PointRefData iOther)` — Initializes a new instance of the GH_PointRefData class

### Methods
#### `public Point3d EvCurve(Curve c)`

Evaluate this reference structure when applied to a curve

**Remarks:** Evaluation depends on the value of m_RefType

**Parameters:**
- `c` (Curve) — The curve to act on

**Returns:** `Point3d` — The point on the curve if successful, or Nothing

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_EvCurve.htm)

#### `public double EvCurveParam(Curve c)`

Evaluate this reference structure when applied to a curve

**Remarks:** Evaluation depends on the value of m_RefType

**Parameters:**
- `c` (Curve) — The curve to act on

**Returns:** `Double` — The parameter on the curve that corresponds to this reference, or NaN

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_EvCurveParam.htm)

#### `public Point3d EvSurface(Surface s)`

Evaluate this reference structure when applied to the surface

**Remarks:** Surface is evaluated at normalized {uv} parameters

**Parameters:**
- `s` (Surface) — The surface to act on.

**Returns:** `Point3d` — A point if successful, or Nothing

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_EvSurface.htm)

#### `protected bool SetCurveDistFromEndParam(Curve c, double t)`



**Parameters:**
- `c` (Curve)
- `t` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_SetCurveDistFromEndParam.htm)

#### `protected bool SetCurveDistFromStartParam(Curve c, double t)`



**Parameters:**
- `c` (Curve)
- `t` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_SetCurveDistFromStartParam.htm)

#### `public bool SetCurveParam(Curve c, double t)`

Set the curve reference parameter based on the value of m_RefType and the curve parameter

**Parameters:**
- `c` (Curve) — The curve to act on
- `t` (System.Double) — The parameter on the curve domain which is to be translated

**Returns:** `Boolean` — True on success

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_SetCurveParam.htm)

#### `protected bool SetCurveRatioParam(Curve c, double t)`



**Parameters:**
- `c` (Curve)
- `t` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_SetCurveRatioParam.htm)

#### `public bool SetSurfaceParam(Surface srf, double u, double v)`

Set the surface reference parameters

**Parameters:**
- `srf` (Surface) — The surface to act on
- `u` (System.Double) — The parameter on the surface u domain
- `v` (System.Double) — The parameter on the surface v domain

**Returns:** `Boolean` — True on success

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointRefData_SetSurfaceParam.htm)

### Properties
- `m_RefID` (Guid, get) — 
- `m_RefIndex` (Int32, get) — 
- `m_RefParam` (Double[], get) — 
- `m_RefType` (GH_PointRefType, get) — 

## GH_PointRefType (enum)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_PointRefType.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_PointRefType.htm)

### Values
- `coordinate` = `1`
- `point_object` = `2`
- `curve_ratio` = `10`
- `curve_dist_start` = `11`
- `curve_dist_end` = `12`
- `srf_param` = `20`

## GH_PointUtil (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_PointUtil.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_PointUtil.htm)

### Methods
#### `public static Plane FitPlaneThroughPoints(IEnumerable<GH_Point> pts)`

Fit a least-squares plane through a collection of points.

**Parameters:**
- `pts` (System.Collections.Generic.IEnumerable<GH_Point>) — Points to fit, may contain nulls.

**Returns:** `Plane` — Best fit plane or null if insufficient points were supplied.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointUtil_FitPlaneThroughPoints.htm)

#### `public static List<GH_Point> ProjectPointsToPlane(IEnumerable<GH_Point> pts, Plane plane, bool include_nulls)`

Project a list of points onto the plane. The result will be points in plane-uv coordinates.

**Parameters:**
- `pts` (System.Collections.Generic.IEnumerable<GH_Point>) — Points to project.
- `plane` (Plane) — Plane to project onto.
- `include_nulls` (System.Boolean) — If True, nulls in the pts list are maintained.

**Returns:** `List<GH_Point>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointUtil_ProjectPointsToPlane.htm)

#### `public static List<GH_Point> PullPointsToPlane(IEnumerable<GH_Point> pts, Plane plane, bool include_nulls)`

Pull a list of points onto the plane. The result will be points in 3D world coordinates which are coincident with the given plane.

**Parameters:**
- `pts` (System.Collections.Generic.IEnumerable<GH_Point>) — Points to pull.
- `plane` (Plane) — Plane to pull onto.
- `include_nulls` (System.Boolean) — If True, nulls in the pts list are maintained.

**Returns:** `List<GH_Point>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointUtil_PullPointsToPlane.htm)

#### `public static List<GH_Point> RemapPointsToPlane(IEnumerable<GH_Point> pts, Plane plane, bool include_nulls)`

Remap a list of points onto the plane. This operation equals a ChangeBasisXForm for each point.

**Parameters:**
- `pts` (System.Collections.Generic.IEnumerable<GH_Point>) — Points to remap.
- `plane` (Plane) — Plane to remap with.
- `include_nulls` (System.Boolean) — If True, nulls in the pts list are maintained.

**Returns:** `List<GH_Point>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_PointUtil_RemapPointsToPlane.htm)

## GH_Rectangle (class)

Represents a 3D rectangle. GH_Rectangle re-implements the RhinoCommon Rectangle3d.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Rectangle.htm)

### Constructors
- `public GH_Rectangle()` — Default constructor
- `public GH_Rectangle(Rectangle3d rec)` — Create a duplicate of another rectangle.
- `public GH_Rectangle(GH_Rectangle other)` — Create a duplicate of another circle.

### Methods
#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

Local to Remote caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_CastTo__1.htm)

#### `public virtual void ClearCaches()`

Clears all caches. Typically if the geometry is referenced, this will erase the local copy. If your T is a value-type, you must override this function and specifically unset the local value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_ClearCaches.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_Duplicate.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

Create a duplicate of this rectangle.

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_DuplicateGeometry.htm)

#### `public GH_Rectangle DuplicateRectangle()`

Create a duplicate of this rectangle.

**Returns:** `GH_Rectangle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_DuplicateRectangle.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this rectangle. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_GetBoundingBox.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public virtual bool LoadGeometry(RhinoDoc doc)`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.

**Parameters:**
- `doc` (RhinoDoc) — Document to use for loading.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry_1.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the circle using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — Gets the boundingbox for this geometry.
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — Gets a value indicating whether or not this geometry is currently loaded (assuming it is referenced). Not all IGH_GeometricGoo implementations support referenced geometry. The default is to always return True.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets the validity of this instance.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — Gets or sets the Guid of the object that is referenced. Not all IGH_GeometricGoo implementations support referenced geometry. The default implementation is to always return Guid.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the value of this type. Note that if the type has a ReferenceID this value might get destroyed in the future.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Rectangle.GH_RectangleProxy (class)

Proxy description of GH_Rectangle class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Rectangle_GH_RectangleProxy.htm)

### Constructors
- `public GH_RectangleProxy(GH_Rectangle nOwner)` — Initializes a new instance of the GH_Rectangle.GH_RectangleProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Rectangle_GH_RectangleProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public virtual bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Area` (String, get) — 
- `Circumference` (String, get) — 
- `Height` (GH_Interval_Wrapper, get) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `Origin` (GH_Point3d_Wrapper, get) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `Width` (GH_Interval_Wrapper, get) — 
- `XAxis` (GH_Vector3d_Wrapper, get) — 
- `YAxis` (GH_Vector3d_Wrapper, get) — 
- `ZAxis` (GH_Vector3d_Wrapper, get) — 

## GH_SpinForce (class)

Rotational spin force for IGH_Fields.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_SpinForce.htm)

### Constructors
- `public GH_SpinForce()` — Initializes a new instance of the GH_SpinForce class

### Methods
#### `public virtual void DrawViewportMeshes(GH_PreviewMeshArgs args)`

(Inherited from GH_FieldElement.)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_DrawViewportMeshes.htm)

#### `public override void DrawViewportWires(GH_PreviewWireArgs args)`

(Overrides GH_FieldElement.DrawViewportWires(GH_PreviewWireArgs).)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SpinForce_DrawViewportWires.htm)

#### `public override IGH_FieldElement Duplicate()`

(Overrides GH_FieldElement.Duplicate().)

**Returns:** `IGH_FieldElement` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SpinForce_Duplicate.htm)

#### `public override Vector3d Force(Point3d location)`

(Overrides GH_FieldElement.Force(Point3d).)

**Parameters:**
- `location` (Point3d)

**Returns:** `Vector3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SpinForce_Force.htm)

#### `public virtual bool IsCoincident(Point3d point, double tolerance)`

(Inherited from GH_FieldElement.)

**Parameters:**
- `point` (Point3d)
- `tolerance` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_IsCoincident.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_FieldElement.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SpinForce_Read.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_FieldElement.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SpinForce_Write.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — (Overrides GH_FieldElement.BoundingBox.)
- `ClippingBox` (BoundingBox, get) — (Inherited from GH_FieldElement.)
- `Decay` (Double, get/set) — 
- `ElementGuid` (Guid, get) — (Overrides GH_FieldElement.ElementGuid.)
- `IsLimited` (Boolean, get/set) — (Inherited from GH_FieldElement.)
- `Limits` (Box, get/set) — (Inherited from GH_FieldElement.)
- `Plane` (Plane, get/set) — 
- `Radius` (Double, get/set) — 
- `Strength` (Double, get/set) — 

## GH_String (class)

Represents a literal string. GH_String re-implements the framework System.String type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_String.htm)

### Constructors
- `public GH_String()` — Blank constructor
- `public GH_String(string str)` — Initializes a new instance of the GH_String class
- `public GH_String(GH_String other)` — Initializes a new instance of the GH_String class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_Duplicate.htm)

#### `public GH_String DuplicateString()`



**Returns:** `GH_String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_DuplicateString.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_String_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. String are valid if they are not null.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_StructurePath (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_StructurePath.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_StructurePath.htm)

### Constructors
- `public GH_StructurePath()` — Blank constructor
- `public GH_StructurePath(GH_Path path)` — Initializes a new instance of the GH_StructurePath class
- `public GH_StructurePath(GH_StructurePath other)` — Initializes a new instance of the GH_StructurePath class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_Duplicate.htm)

#### `public GH_StructurePath DuplicatePath()`



**Returns:** `GH_StructurePath` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_DuplicatePath.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_StructurePath_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Integers are always valid
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (GH_Path, get/set) — (Overrides GH_Goo<T>.Value.)
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_SubD (class)

Represents a 3D subdivision surface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_SubD.htm)

### Constructors
- `public GH_SubD()` — Initializes a new instance of the GH_SubD class
- `public GH_SubD(SubD subd)` — Initializes a new instance of the GH_SubD class
- `public GH_SubD(Guid id)` — Initializes a new instance of the GH_SubD class
- `public GH_SubD(GH_SubD other)` — Initializes a new instance of the GH_SubD class

### Methods
#### `public void AppendRenderGeometry(GH_RenderArgs args, RenderMaterial materialHint)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_RenderArgs)
- `materialHint` (RenderMaterial)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_AppendRenderGeometry.htm)

#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

(Overrides GH_GeometricGoo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_GeometricGoo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_CastTo__1.htm)

#### `public override void ClearCaches()`

(Overrides GH_GeometricGoo<T>.ClearCaches().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_ClearCaches.htm)

#### `public void DestroyPreviewMeshes()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_DestroyPreviewMeshes.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_Duplicate.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

(Overrides GH_GeometricGoo<T>.DuplicateGeometry().)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_DuplicateGeometry.htm)

#### `public GH_SubD DuplicateSubD()`



**Returns:** `GH_SubD` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_DuplicateSubD.htm)

#### `public override IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_GetBoundingBox.htm)

#### `public Mesh[] GetPreviewMeshes()`



**Returns:** `Mesh[]` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_GetPreviewMeshes.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public override bool LoadGeometry(RhinoDoc doc)`

(Overrides GH_GeometricGoo<T>.LoadGeometry(RhinoDoc).)

**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_LoadGeometry.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_Transform.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_SubD_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `IsGeometryLoaded` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsGeometryLoaded.)
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — (Overrides GH_GeometricGoo<T>.ReferenceID.)
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (SubD, get/set) — (Overrides GH_GeometricGoo<T>.Value.)
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Surface (class)

Represents a 3D (trimmed) surface. GH_Surface wraps the functionality of the OpenNURBS OnBrepFace class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Surface.htm)

### Constructors
- `public GH_Surface()` — Initializes a new instance of the GH_Surface class
- `public GH_Surface(Brep brep)` — Initializes a new instance of the GH_Surface class
- `public GH_Surface(Surface srf)` — Initializes a new instance of the GH_Surface class
- `public GH_Surface(Guid id)` — Initializes a new instance of the GH_Surface class
- `public GH_Surface(GH_Surface other)` — Initializes a new instance of the GH_Surface class

### Methods
#### `public void AppendRenderGeometry(GH_RenderArgs args, RenderMaterial materialHint)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_RenderArgs)
- `materialHint` (RenderMaterial)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_AppendRenderGeometry.htm)

#### `public bool BakeGeometry(RhinoDoc doc, ObjectAttributes att, ref Guid obj_guid)`



**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `obj_guid` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_BakeGeometry.htm)

#### `public override bool CastFrom(Object source)`

(Overrides GH_GeometricGoo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_CastFrom.htm)

#### `public override bool CastTo<Q>(out Q target)`

Attempt a cast to type T.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_CastTo__1.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_GeometricGoo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_CastTo__1.htm)

#### `public override void ClearCaches()`

(Overrides GH_GeometricGoo<T>.ClearCaches().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_ClearCaches.htm)

#### `public void DestroyPreviewMeshes()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_DestroyPreviewMeshes.htm)

#### `public void DrawViewportMeshes(GH_PreviewMeshArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(GH_PreviewWireArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_DrawViewportWires.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_GeometricGoo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_Duplicate.htm)

#### `public override IGH_GeometricGoo DuplicateGeometry()`

(Overrides GH_GeometricGoo<T>.DuplicateGeometry().)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_DuplicateGeometry.htm)

#### `public GH_Surface DuplicateSurface()`



**Returns:** `GH_Surface` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_DuplicateSurface.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_GeometricGoo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_EmitProxy.htm)

#### `public override BoundingBox GetBoundingBox(Transform xform)`

(Overrides GH_GeometricGoo<T>.GetBoundingBox(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `BoundingBox` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_GetBoundingBox.htm)

#### `public Mesh[] GetPreviewMeshes()`



**Returns:** `Mesh[]` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_GetPreviewMeshes.htm)

#### `public bool IsPointOnDomain(double u, double v)`



**Parameters:**
- `u` (System.Double)
- `v` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_IsPointOnDomain.htm)

#### `public bool IsPointOnFace(double u, double v)`



**Parameters:**
- `u` (System.Double)
- `v` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_IsPointOnFace.htm)

#### `public virtual bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GeometricGoo_1_LoadGeometry.htm)

#### `public override bool LoadGeometry(RhinoDoc doc)`

(Overrides GH_GeometricGoo<T>.LoadGeometry(RhinoDoc).)

**Parameters:**
- `doc` (RhinoDoc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_LoadGeometry.htm)

#### `public override IGH_GeometricGoo Morph(SpaceMorph xmorph)`

(Overrides GH_GeometricGoo<T>.Morph(SpaceMorph).)

**Parameters:**
- `xmorph` (SpaceMorph)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_Morph.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_ToString.htm)

#### `public override IGH_GeometricGoo Transform(Transform xform)`

(Overrides GH_GeometricGoo<T>.Transform(Transform).)

**Parameters:**
- `xform` (Transform)

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_Transform.htm)

#### `public bool Untrim()`



**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_Untrim.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_Write.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — (Overrides GH_GeometricGoo<T>.Boundingbox.)
- `ClippingBox` (BoundingBox, get) — 
- `Face` (BrepFace, get) — 
- `IsGeometryLoaded` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsGeometryLoaded.)
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — (Overrides GH_GeometricGoo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — (Overrides GH_GeometricGoo<T>.ReferenceID.)
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (Brep, get/set) — (Overrides GH_GeometricGoo<T>.Value.)
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Surface.GH_SurfaceProxy (class)

(No description provided in vendor docs for Grasshopper.Kernel.Types.GH_Surface.GH_SurfaceProxy.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Surface_GH_SurfaceProxy.htm)

### Constructors
- `public GH_SurfaceProxy(GH_Surface Value)` — Initializes a new instance of the GH_Surface.GH_SurfaceProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_GH_SurfaceProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public override bool FromString(string in)`

(Overrides GH_GooProxy<T>.FromString(String).)

**Parameters:**
- `in` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Surface_GH_SurfaceProxy_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Inherited from GH_GooProxy<T>.)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_ToString.htm)

### Properties
- `Area` (String, get) — 
- `Closed` (Boolean, get) — 
- `Deformable` (Boolean, get) — 
- `EdgeCount` (Int32, get) — 
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `LoopCount` (Int32, get) — 
- `ObjectID` (String, get/set) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `Periodic` (Boolean, get) — 
- `Planar` (Boolean, get) — 
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `Singular` (Boolean, get) — 
- `Solid` (Boolean, get) — 
- `Trimmed` (Boolean, get) — 
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `VertexCount` (Int32, get) — 
- `Volume` (String, get) — 

## GH_Time (class)

Represents a date and time. GH_Time re-implements the framework System.DateTime type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Time.htm)

### Constructors
- `public GH_Time()` — Blank constructor
- `public GH_Time(DateTime time)` — Initializes a new instance of the GH_Time class
- `public GH_Time(GH_Time other)` — Initializes a new instance of the GH_Time class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_Duplicate.htm)

#### `public GH_Time DuplicateDateTime()`



**Returns:** `GH_Time` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_DuplicateDateTime.htm)

#### `public override IGH_GooProxy EmitProxy()`

(Overrides GH_Goo<T>.EmitProxy().)

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Time_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets the validity of this instance. Datetime is valid if it's not equal to DateTime.MaxValue
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Transform (class)

Represents a collection of transformations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Transform.htm)

### Constructors
- `public GH_Transform()` — Initializes a new instance of the GH_Transform class
- `public GH_Transform(Transform transform)` — Initializes a new instance of the GH_Transform class
- `public GH_Transform(GH_Transform other)` — Initializes a new instance of the GH_Transform class
- `public GH_Transform(ITransform transform)` — Initializes a new instance of the GH_Transform class

### Methods
#### `public override bool CastFrom(Object source)`

(Overrides GH_Goo<T>.CastFrom(Object).)

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<Q>(ref Q target)`

(Overrides GH_Goo<T>.CastTo<Q>(Q).)

**Parameters:**
- `target` (Q)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_CastTo__1.htm)

#### `public void ClearCaches()`

Erase the compound transformation caches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_ClearCaches.htm)

#### `public override IGH_Goo Duplicate()`

(Overrides GH_Goo<T>.Duplicate().)

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_Duplicate.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this transform. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_EmitProxy.htm)

#### `public GH_Transform GetInverse()`



**Returns:** `GH_Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_GetInverse.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_Read.htm)

#### `public override Object ScriptVariable()`

(Overrides GH_Goo<T>.ScriptVariable().)

**Returns:** `Object` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_ScriptVariable.htm)

#### `public override string ToString()`

(Overrides GH_Goo<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Transform_Write.htm)

### Properties
- `CompoundTransforms` (List<ITransform>, get) — Gets the internal set of transforms. If you modify the compound transformations, you must call ClearCaches().
- `IsValid` (Boolean, get) — (Overrides GH_Goo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (Transform, get/set) — (Overrides GH_Goo<T>.Value.)
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Vector (class)

Represents a 3D vector. GH_Vector re-implements the RhinoCommon Vector3d struct.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Vector.htm)

### Constructors
- `public GH_Vector()` — Default constructor. Creates a zero-length vector.
- `public GH_Vector(Vector3d vector)` — Create a vector from x, y and z components.
- `public GH_Vector(GH_Vector other)` — Create a duplicate of another vector

### Methods
#### `public override bool CastFrom(Object source)`

Remote to Local caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `source` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_CastFrom.htm)

#### `public virtual bool CastTo<Q>(ref Q target)`

Attempt a cast to type Q.

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (Q) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_CastTo__1.htm)

#### `public override bool CastTo<T>(out T target)`

Local to Remote caster function. This stuff is complex, don't concern yourself with casting logic.

**Parameters:**
- `target` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_CastTo__1.htm)

#### `public override IGH_Goo Duplicate()`

Create a duplicate of this vector.

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_Duplicate.htm)

#### `public GH_Vector DuplicateVector()`

Create a duplicate of this vector.

**Returns:** `GH_Vector` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_DuplicateVector.htm)

#### `public override IGH_GooProxy EmitProxy()`

Returns a proxy that represents this vector. Do not call this function unless you're

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_EmitProxy.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_Goo<T>.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_Read.htm)

#### `public virtual Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disappears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The default implementation of this function returns the Value instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Goo_1_ScriptVariable.htm)

#### `public override string ToString()`

Format the vector using default grasshopper formatting logic.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_ToString.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_Goo<T>.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_Write.htm)

### Properties
- `IsValid` (Boolean, get) — (Overrides GH_Goo<T>.IsValid.)
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — (Overrides GH_Goo<T>.TypeDescription.)
- `TypeName` (String, get) — (Overrides GH_Goo<T>.TypeName.)
- `Value` (T, get/set) — Gets or sets the internal data.
- `m_value` (T, get) — (Inherited from GH_Goo<T>.)

## GH_Vector.GH_VectorProxy (class)

Proxy description of GH_Vector class. This class is used by the Generic Multivalue interfaces.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_Vector_GH_VectorProxy.htm)

### Constructors
- `public GH_VectorProxy(GH_Vector nOwner)` — Initializes a new instance of the GH_Vector.GH_VectorProxy class

### Methods
#### `public override void Construct()`

(Overrides GH_GooProxy<T>.Construct().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_GH_VectorProxy_Construct.htm)

#### `public virtual string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation must be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_FormatInstance.htm)

#### `public override bool FromString(string in)`

(Overrides GH_GooProxy<T>.FromString(String).)

**Parameters:**
- `in` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_GH_VectorProxy_FromString.htm)

#### `public virtual string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_MutateString.htm)

#### `protected string NumberToString(double number)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `number` (System.Double)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_NumberToString.htm)

#### `protected double StringToNumber(string text)`

(Inherited from GH_GooProxy<T>.)

**Parameters:**
- `text` (System.String)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_GooProxy_1_StringToNumber.htm)

#### `public override string ToString()`

(Overrides GH_GooProxy<T>.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Vector_GH_VectorProxy_ToString.htm)

### Properties
- `IsParsable` (Boolean, get) — (Overrides GH_GooProxy<T>.IsParsable.)
- `Length` (String, get/set) — 
- `Owner` (T, get) — (Inherited from GH_GooProxy<T>.)
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `TypeDescription` (String, get) — (Inherited from GH_GooProxy<T>.)
- `TypeName` (String, get) — (Inherited from GH_GooProxy<T>.)
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.
- `X` (String, get/set) — 
- `Y` (String, get/set) — 
- `Z` (String, get/set) — 

## GH_VectorForce (class)

Linear vector force implementation for IGH_Fields.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_GH_VectorForce.htm)

### Constructors
- `public GH_VectorForce()` — Initializes a new instance of the GH_VectorForce class

### Methods
#### `public virtual void DrawViewportMeshes(GH_PreviewMeshArgs args)`

(Inherited from GH_FieldElement.)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_DrawViewportMeshes.htm)

#### `public override void DrawViewportWires(GH_PreviewWireArgs args)`

(Overrides GH_FieldElement.DrawViewportWires(GH_PreviewWireArgs).)

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_VectorForce_DrawViewportWires.htm)

#### `public override IGH_FieldElement Duplicate()`

(Overrides GH_FieldElement.Duplicate().)

**Returns:** `IGH_FieldElement` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_VectorForce_Duplicate.htm)

#### `public override Vector3d Force(Point3d location)`

(Overrides GH_FieldElement.Force(Point3d).)

**Parameters:**
- `location` (Point3d)

**Returns:** `Vector3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_VectorForce_Force.htm)

#### `public virtual bool IsCoincident(Point3d point, double tolerance)`

(Inherited from GH_FieldElement.)

**Parameters:**
- `point` (Point3d)
- `tolerance` (System.Double)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_FieldElement_IsCoincident.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_FieldElement.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_VectorForce_Read.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_FieldElement.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_VectorForce_Write.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — (Overrides GH_FieldElement.BoundingBox.)
- `Chord` (Line, get/set) — 
- `ClippingBox` (BoundingBox, get) — (Inherited from GH_FieldElement.)
- `ElementGuid` (Guid, get) — (Overrides GH_FieldElement.ElementGuid.)
- `IsLimited` (Boolean, get/set) — (Inherited from GH_FieldElement.)
- `Limits` (Box, get/set) — (Inherited from GH_FieldElement.)

## IGH_FieldElement (interface)

Represents the basic interface for field elements.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_IGH_FieldElement.htm)

### Methods
#### `void DrawViewportMeshes(GH_PreviewMeshArgs args)`

Implement this function to draw all shaded meshes. If the viewport does not support shading, this function will not be called.

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewMeshArgs) — Drawing arguments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_IGH_PreviewData_DrawViewportMeshes.htm)

#### `void DrawViewportWires(GH_PreviewWireArgs args)`

Implement this function to draw all wire and point previews.

**Parameters:**
- `args` (Grasshopper.Kernel.GH_PreviewWireArgs) — Drawing arguments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_IGH_PreviewData_DrawViewportWires.htm)

#### `IGH_FieldElement Duplicate()`

Create an exact copy of this element.

**Returns:** `IGH_FieldElement` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_FieldElement_Duplicate.htm)

#### `Vector3d Force(Point3d location)`

Compute the force at a given location.

**Parameters:**
- `location` (Point3d)

**Returns:** `Vector3d` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_FieldElement_Force.htm)

#### `bool IsCoincident(Point3d point, double tolerance)`

Computes whether the point can be considered coincident with the element.

**Parameters:**
- `point` (Point3d) — Point to test.
- `tolerance` (System.Double) — Tolerance to use.

**Returns:** `Boolean` — True if the point is coincident within tolerance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_FieldElement_IsCoincident.htm)

#### `bool Read(GH_IReader reader)`

This method is called whenever the instance is required to deserialize itself.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Reader object to deserialize from.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Read.htm)

#### `bool Write(GH_IWriter writer)`

This method is called whenever the instance is required to serialize itself.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer object to serialize with.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Write.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — Gets the visual boundingbox of all the geometry used to represent this element in 3D.
- `ClippingBox` (BoundingBox, get) — Gets the clipping box for this data. The clipping box is typically the same as the boundingbox.
- `ElementGuid` (Guid, get) — Gets the element Guid for this element.
- `IsLimited` (Boolean, get/set) — Gets or sets whether this element affects a limited region.
- `Limits` (Box, get/set) — Gets or sets the limits of this region. Unless IsLimited equals true, the Limits property is ignored.

## IGH_GeometricGoo (interface)

Base interface for all Data inside Grasshoper that could pass for Geometry

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_IGH_GeometricGoo.htm)

### Methods
#### `bool CastFrom(Object source)`

Attempt a cast from generic object

**Remarks:** If False, the contents of this instance are not to be trusted.

**Parameters:**
- `source` (System.Object) — Reference to source of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_CastFrom.htm)

#### `bool CastTo<T>(out T target)`

Attempt a cast to type T

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (T) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_CastTo__1.htm)

#### `void ClearCaches()`

Clears all caches. Typically if the geometry is referenced, this will erase the local copy.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GeometricGoo_ClearCaches.htm)

#### `IGH_Goo Duplicate()`

Make a complete duplicate of this instance. No shallow copies.

**Remarks:** Classes which implement this interface should also provide type specific Duplicate methods

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_Duplicate.htm)

#### `IGH_GeometricGoo DuplicateGeometry()`

Make a complete duplicate of this geometry. No shallow copies.

**Remarks:** Each class that implements this interface should also provide a type specific duplication method

**Returns:** `IGH_GeometricGoo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GeometricGoo_DuplicateGeometry.htm)

#### `IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_EmitProxy.htm)

#### `BoundingBox GetBoundingBox(Transform xform)`

Compute an aligned boundingbox.

**Parameters:**
- `xform` (Transform) — Transformation to apply to geometry for BoundingBox computation.

**Returns:** `BoundingBox` — The world aligned boundingbox of the transformed geometry.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GeometricGoo_GetBoundingBox.htm)

#### `bool LoadGeometry()`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GeometricGoo_LoadGeometry.htm)

#### `bool LoadGeometry(RhinoDoc doc)`

If the geometry is referenced and currently unloaded, forces loading of the geometry. Not all IGH_GeometricGoo implementations support referenced geometry.

**Parameters:**
- `doc` (RhinoDoc) — Document to use for loading.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GeometricGoo_LoadGeometry_1.htm)

#### `IGH_GeometricGoo Morph(SpaceMorph xmorph)`

Morph the object or a deformable representation of the object.

**Parameters:**
- `xmorph` (SpaceMorph) — Spatial deform.

**Returns:** `IGH_GeometricGoo` — Deformed geometry. If the local geometry can be deformed accurately, then the returned instance equals this instance. Not all geometry types can be accurately deformed though, if this is the case, this function will return an instance of another IGH_GeometricGoo derived type which can be deformed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GeometricGoo_Morph.htm)

#### `Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disapears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The object that represents this IGH_Goo instance in a script.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_ScriptVariable.htm)

#### `string ToString()`

Creates a string description of the current instance value

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_ToString.htm)

#### `IGH_GeometricGoo Transform(Transform xform)`

Transforms the object or a deformable representation of the object.

**Parameters:**
- `xform` (Transform) — Transformation matrix.

**Returns:** `IGH_GeometricGoo` — Transformed geometry. If the local geometry can be transformed accurately, then the returned instance equals this instance. Not all geometry types can be accurately transformed under all circumstances though, if this is the case, this function will return an instance of another IGH_GeometricGoo derived type which can be transformed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GeometricGoo_Transform.htm)

### Properties
- `Boundingbox` (BoundingBox, get) — Gets the (cached) boundingbox for this geometry. Not all geometry types cache boundingbox results.
- `IsGeometryLoaded` (Boolean, get) — Gets a value indicating whether or not this geometry is currently loaded (assuming it is referenced). Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsReferencedGeometry` (Boolean, get) — Gets a value indicating whether or not this geometry is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `IsValid` (Boolean, get) — Gets a value indicating whether or not the current value is valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `ReferenceID` (Guid, get/set) — Gets or sets the Guid of the object that is referenced. Not all IGH_GeometricGoo implementations support referenced geometry.
- `TypeDescription` (String, get) — Gets a description of the type of the implementation.
- `TypeName` (String, get) — Gets the name of the type of the implementation.

## IGH_Goo (interface)

Base interface for all Data inside Grasshopper. Every parameter must implement a type of Goo.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_IGH_Goo.htm)

### Methods
#### `bool CastFrom(Object source)`

Attempt a cast from generic object

**Remarks:** If False, the contents of this instance are not to be trusted.

**Parameters:**
- `source` (System.Object) — Reference to source of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_CastFrom.htm)

#### `bool CastTo<T>(out T target)`

Attempt a cast to type T

**Remarks:** If false, the target instance contents are not guaranteed to be valid.

**Parameters:**
- `target` (T) — Pointer to target of cast.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_CastTo__1.htm)

#### `IGH_Goo Duplicate()`

Make a complete duplicate of this instance. No shallow copies.

**Remarks:** Classes which implement this interface should also provide type specific Duplicate methods

**Returns:** `IGH_Goo` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_Duplicate.htm)

#### `IGH_GooProxy EmitProxy()`

Create a new proxy for this instance. Return Null if this class does not support proxies.

**Returns:** `IGH_GooProxy` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_EmitProxy.htm)

#### `bool Read(GH_IReader reader)`

This method is called whenever the instance is required to deserialize itself.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Reader object to deserialize from.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Read.htm)

#### `Object ScriptVariable()`

This function will be called when the local IGH_Goo instance disapears into a user Script. This would be an excellent place to cast your IGH_Goo type to a simple data type.

**Returns:** `Object` — The object that represents this IGH_Goo instance in a script.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_ScriptVariable.htm)

#### `string ToString()`

Creates a string description of the current instance value

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_Goo_ToString.htm)

#### `bool Write(GH_IWriter writer)`

This method is called whenever the instance is required to serialize itself.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer object to serialize with.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Write.htm)

### Properties
- `IsValid` (Boolean, get) — Gets a value indicating whether or not the current value is valid.
- `IsValidWhyNot` (String, get) — Gets a string describing the state of "invalidness". If the instance is valid, then this property should return Nothing or String.Empty.
- `TypeDescription` (String, get) — Gets a description of the type of the implementation.
- `TypeName` (String, get) — Gets the name of the type of the implementation.

## IGH_GooProxy (interface)

Base interface for all type proxies.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_IGH_GooProxy.htm)

### Methods
#### `void Construct()`

This method will be called when a new instance of this type is constructed. It allows implementers to supply a customized UI to be shown during construction. This method should never throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GooProxy_Construct.htm)

#### `string FormatInstance()`

Returns a String description of the current value.

**Remarks:** If the implementation IsParsable(), then this representation will be in parsable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GooProxy_FormatInstance.htm)

#### `bool FromString(string in)`

If IsParsable(), then attempts to convert the string to a generic type value

**Parameters:**
- `in` (System.String) — The String to parse.

**Returns:** `Boolean` — True on success, false on failure. This method should not throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GooProxy_FromString.htm)

#### `string MutateString(string in)`

Munge a string to remove obvious errors on account of the user.

**Parameters:**
- `in` (System.String)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_IGH_GooProxy_MutateString.htm)

### Properties
- `IsParsable` (Boolean, get) — Gets a value indicating whether or not the type can be instantiated from a String.
- `ProxyOwner` (IGH_Goo, get) — Gets the piece of Grasshopper data that spawned this proxy.
- `UserString` (String, get/set) — Gets or sets the user-defined string that describes this proxy. This only really makes sense if the Proxy is Parsable.
- `Valid` (Boolean, get) — Gets a value indicating whether this proxy represents valid data.

## UVInterval (struct)

UVInterval data type. Do not confuse this with GH_Interval2D.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_UVInterval.htm)

### Constructors
- `public UVInterval(Interval newU, Interval newV)` — Initializes a new instance of the UVInterval class

### Properties
- `IsValid` (Boolean, get) — Gets a value indicating whether or not this UVInterval is valid.
- `U` (Interval, get/set) — Gets or sets the U interval.
- `U0` (Double, get/set) — Gets or sets the T0 value of the U domain.
- `U1` (Double, get/set) — Gets or sets the T1 value of the U domain.
- `V` (Interval, get/set) — Gets or sets the V interval.
- `V0` (Double, get/set) — Gets or sets the T0 value of the V domain.
- `V1` (Double, get/set) — Gets or sets the T1 value of the V domain.

