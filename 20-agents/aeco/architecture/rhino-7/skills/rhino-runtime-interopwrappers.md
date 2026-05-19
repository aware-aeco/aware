---
name: rhino-rhino-runtime-interopwrappers
description: This skill encodes the rhino 7.0 surface of the Rhino.Runtime.InteropWrappers namespace — 31 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ClassArrayOnObjRef, ClassArrayObjRef, ClassArrayString, SimpleArray2dex, SimpleArrayArrayPoint3d, SimpleArrayBinaryArchiveReader, SimpleArrayBrepPointer, SimpleArrayByte, and 23 more types.
---

# Rhino.Runtime.InteropWrappers

Auto-generated from vendor docs for rhino 7.0. 31 types in this namespace.

## ClassArrayObjRef (class)

Represents a wrapper to an unmanaged "array" (list) of CRhinoObjRef instances. Wrapper for a C++ ON_ClassArray of CRhinoObjRef

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_ClassArrayObjRef.htm)

### Constructors
- `public ClassArrayObjRef()` — Initializes a new ClassArrayObjRef instance.
- `public ClassArrayObjRef(IEnumerable<ObjRef> objrefs)` — Initializes a new instances from a set of ObjRefs

### Methods
#### `public void Add(ObjRef objref)`

Adds an ObjRef to the list.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — An ObjRef to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayObjRef_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayObjRef_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayObjRef_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayObjRef_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayObjRef_NonConstPointer.htm)

#### `public ObjRef[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `ObjRef[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayObjRef_ToNonConstArray.htm)

### Properties
- `Count` (Int32, get) — Gets the number of CRhinoObjRef instances in this array.

## ClassArrayOnObjRef (class)

Represents a wrapper to an unmanaged "array" (list) of ON_ObjRef instances. Wrapper for a C++ ON_ClassArray of ON_ObjRef

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef.htm)

### Constructors
- `public ClassArrayOnObjRef()` — Initializes a new ClassArrayOnObjRef instance.
- `public ClassArrayOnObjRef(IEnumerable<ObjRef> objrefs)` — Initializes a new instances from a set of ObjRefs

### Methods
#### `public void Add(ObjRef objref)`

Adds an ObjRef to the list.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — An ObjRef to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef_NonConstPointer.htm)

#### `public ObjRef[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `ObjRef[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef_ToNonConstArray.htm)

#### `public ObjRef[] ToNonConstArray(RhinoDoc doc)`

Copies the unmanaged array to a managed counterpart.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Document containing the array of objects

**Returns:** `ObjRef[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayOnObjRef_ToNonConstArray_1.htm)

### Properties
- `Count` (Int32, get) — Gets the number of ObjRef instances in this array.

## ClassArrayString (class)

Wrapper for a C++ ON_ClassArray<ON_wString> If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_ClassArrayString.htm)

### Constructors
- `public ClassArrayString()` — Initializes a new ClassArrayString instance.

### Methods
#### `public void Add(string s)`

Adds a string to the list.

**Parameters:**
- `s` (System.String) — A string to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayString_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayString_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayString_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the ClassArrayString and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayString_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayString_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayString_NonConstPointer.htm)

#### `public string[] ToArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `String[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_ClassArrayString_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the number of strings in this array.

## CurveSegment (struct)

For internal use only.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_CurveSegment.htm)

### Properties
- `Index` (Int32, get) — The index of the curve used by this boundary element.
- `Reversed` (Boolean, get) — True if this piece of the curve should be reversed.
- `SubDomain` (Interval, get) — The sub-domain of the curve used by this boundary element.

## MeshPointDataStruct (struct)

This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_MeshPointDataStruct.htm)

### Properties
- `m_ci_index` (Int32, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_ci_type` (UInt32, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_edge_index` (Int32, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_et` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_face_index` (Int32, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_Px` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_Py` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_Pz` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_t0` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_t1` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_t2` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_t3` (Double, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.
- `m_Triangle` (Char, get) — This is only needed when passing values to the Rhino C++ core, ignore for .NET plug-ins.

## SimpleArray2dex (class)

Wrapper for ON_SimpleArray<ON_2dex>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArray2dex.htm)

### Constructors
- `public SimpleArray2dex()` — Initializes a new SimpleArray2dex class.
- `public SimpleArray2dex(IEnumerable<IndexPair> values)` — Initializes a new SimpleArray2dex class

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArray2dex_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArray2dex_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArray2dex_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArray2dex_NonConstPointer.htm)

#### `public IndexPair[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `IndexPair[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArray2dex_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.

## SimpleArrayArrayPoint3d (class)

Wrapper for ON_SimpleArray<ON_PolyLine*>, ON_SimpleArray<ON_3dPointArray*> If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayArrayPoint3d.htm)

### Constructors
- `public SimpleArrayArrayPoint3d()` — Initializes a new empty SimpleArrayArrayPoint3d instance.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayArrayPoint3d_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayArrayPoint3d_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayArrayPoint3d_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayArrayPoint3d_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayArrayPoint3d_NonConstPointer.htm)

#### `public int PointCountAt(int index)`

Gets the amount of points in a polyline.

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayArrayPoint3d.PointCountAt(System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayArrayPoint3d.PointCountAt(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayArrayPoint3d_PointCountAt.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of polylines in this array.
- `Item` (Point3d, get) — Gets a point in a polyline.

## SimpleArrayBinaryArchiveReader (class)

Wrapper for ON_SimpleArray<ON_BinaryArchive>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayBinaryArchiveReader.htm)

### Constructors
- `public SimpleArrayBinaryArchiveReader()` — Initializes a new SimpleArrayBinaryArchiveReader class.
- `public SimpleArrayBinaryArchiveReader(IntPtr p)` — Initializes a new SimpleArrayBinaryArchiveReader class.

### Methods
#### `public void Add(BinaryArchiveReader reader)`

Adds a new Interval at the end of this array.

**Parameters:**
- `reader` (Rhino.FileIO.BinaryArchiveReader) — [Missing <param name="reader"/> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayBinaryArchiveReader.Add(Rhino.FileIO.BinaryArchiveReader)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBinaryArchiveReader_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBinaryArchiveReader_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBinaryArchiveReader_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBinaryArchiveReader_Finalize.htm)

#### `public BinaryArchiveReader Get(int index)`

Get the Guid at index

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayBinaryArchiveReader.Get(System.Int32)"]

**Returns:** `BinaryArchiveReader` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayBinaryArchiveReader.Get(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBinaryArchiveReader_Get.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBinaryArchiveReader_NonConstPointer.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.

## SimpleArrayBrepPointer (class)

Wrapper for a C++ ON_SimpleArray<ON_Brep*> or ON_SimpleArray<constant ON_Brep*> If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer.htm)

### Constructors
- `public SimpleArrayBrepPointer()` — Initializes a new SimpleArrayBrepPointer instance.

### Methods
#### `public void Add(Brep brep, bool asConst)`

Adds a brep to the list.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A brep to add.
- `asConst` (System.Boolean) — Whether this brep should be treated as non-modifiable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the SimpleArrayBrepPointer and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer_NonConstPointer.htm)

#### `public Brep[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Brep[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayBrepPointer_ToNonConstArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of breps in this array.

## SimpleArrayByte (class)

Wrapper for ON_SimpleArray<unsigned char>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayByte.htm)

### Constructors
- `public SimpleArrayByte()` — Initializes a new SimpleArrayByte class.
- `public SimpleArrayByte(IEnumerable<byte> values)` — Initializes a new SimpleArrayByte class
- `public SimpleArrayByte(int initialSize)` — Initializes a new SimpleArrayByte class. Initial size of the array - all values are set to zero.
- `public SimpleArrayByte(SimpleArrayByte other)` — Initializes a new SimpleArrayByte with the contents of another SimpleArrayByte.

### Methods
#### `public IntPtr Array()`

Return the raw data.

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayByte.Array"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayByte_Array.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayByte_ConstPointer.htm)

#### `public void CopyTo(SimpleArrayByte other)`

Copies the contents of a SimpleArrayByte into another SimpleArrayByte.

**Parameters:**
- `other` (Rhino.Runtime.InteropWrappers.SimpleArrayByte) — [Missing <param name="other"/> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayByte.CopyTo(Rhino.Runtime.InteropWrappers.SimpleArrayByte)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayByte_CopyTo.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayByte_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayByte_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayByte_NonConstPointer.htm)

#### `public byte[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `Byte[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayByte_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.

## SimpleArrayClippingPlaneObjectPointer (class)

ON_SimpleArray of CRhinoClippingPlaneObject*

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayClippingPlaneObjectPointer.htm)

### Constructors
- `public SimpleArrayClippingPlaneObjectPointer()` — Initializes a new SimpleArrayClippingPlaneObjectPointer instance.

### Methods
#### `public void Add(ClippingPlaneObject clippingplane, bool asConst)`

Adds a clipping plane to the list.

**Parameters:**
- `clippingplane` (Rhino.DocObjects.ClippingPlaneObject) — A clipping plane to add.
- `asConst` (System.Boolean) — Whether this clipping plane should be treated as non-modifiable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayClippingPlaneObjectPointer_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayClippingPlaneObjectPointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayClippingPlaneObjectPointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the SimpleArrayClippingPlaneObjectPointer and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayClippingPlaneObjectPointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayClippingPlaneObjectPointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayClippingPlaneObjectPointer_NonConstPointer.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of clipping planes in this array.

## SimpleArrayCurvePointer (class)

Wrapper for a C++ ON_SimpleArray of ON_Curve* or constant ON_Curve*. If you are not writing C++ code, then you can ignore this class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayCurvePointer.htm)

### Constructors
- `public SimpleArrayCurvePointer()` — Initializes a new SimpleArrayCurvePointer instance.
- `public SimpleArrayCurvePointer(IEnumerable<Curve> curves)` — Initializes a new SimpleArrayCurvePointer instance, from a set of input curves.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayCurvePointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayCurvePointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayCurvePointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayCurvePointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayCurvePointer_NonConstPointer.htm)

#### `public Curve[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Curve[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayCurvePointer_ToNonConstArray.htm)

## SimpleArrayDouble (class)

Wrapper for ON_SimpleArray<double>. If you are not writing C++ code, then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayDouble.htm)

### Constructors
- `public SimpleArrayDouble()` — Initializes a new SimpleArrayDouble instance.
- `public SimpleArrayDouble(IEnumerable<double> items)` — Initializes a new SimpleArrayDouble instance, with items.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayDouble_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayDouble_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayDouble_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayDouble_NonConstPointer.htm)

#### `public double[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `Double[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayDouble_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.

## SimpleArrayExtrusionPointer (class)

Wrapper for a C++ ON_SimpleArray<ON_Extrusion*> or ON_SimpleArray<constant ON_Extrusion*> If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer.htm)

### Constructors
- `public SimpleArrayExtrusionPointer()` — Initializes a new SimpleArrayExtrusionPointer instance.

### Methods
#### `public void Add(Extrusion extrusion, bool asConst)`

Adds a extrusion to the list.

**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — A extrusion to add.
- `asConst` (System.Boolean) — Whether this extrusion should be treated as non-modifiable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the SimpleArrayExtrusionPointer and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer_NonConstPointer.htm)

#### `public Extrusion[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Extrusion[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayExtrusionPointer_ToNonConstArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of Extrusions in this array.

## SimpleArrayFloat (class)

Wrapper for ON_SimpleArray<float>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayFloat.htm)

### Constructors
- `public SimpleArrayFloat()` — Initializes a new SimpleArrayFloat class.
- `public SimpleArrayFloat(IEnumerable<float> values)` — Initializes a new SimpleArrayFloat class
- `public SimpleArrayFloat(int initialSize)` — Initializes a new SimpleArrayFloat class. Initial size of the array - all values are set to zero.
- `public SimpleArrayFloat(SimpleArrayFloat other)` — Initializes a new SimpleArrayFloat with the contents of another SimpleArrayFloat.

### Methods
#### `public IntPtr Array()`

Return the raw data.

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayFloat.Array"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayFloat_Array.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayFloat_ConstPointer.htm)

#### `public void CopyTo(SimpleArrayFloat other)`

Copies the contents of a SimpleArrayFloat into another SimpleArrayFloat.

**Parameters:**
- `other` (Rhino.Runtime.InteropWrappers.SimpleArrayFloat) — [Missing <param name="other"/> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayFloat.CopyTo(Rhino.Runtime.InteropWrappers.SimpleArrayFloat)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayFloat_CopyTo.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayFloat_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayFloat_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayFloat_NonConstPointer.htm)

#### `public float[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `Single[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayFloat_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.

## SimpleArrayGeometryPointer (class)

Wrapper for a C++ ON_SimpleArray<ON_Geometry*>* or ON_SimpleArray<constant ON_Geometry*>. If you are not writing C++ code, then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayGeometryPointer.htm)

### Constructors
- `public SimpleArrayGeometryPointer()` — Initializes a new SimpleArrayGeometryPointer instance.
- `public SimpleArrayGeometryPointer(IEnumerable geometry)` — Expects all of the items in the IEnumerable to be GeometryBase types
- `public SimpleArrayGeometryPointer(IEnumerable<GeometryBase> geometry)` — Create an ON_SimpleArray<ON_Geometry*> filled with items in geometry

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGeometryPointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGeometryPointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGeometryPointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGeometryPointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGeometryPointer_NonConstPointer.htm)

#### `public GeometryBase[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `GeometryBase[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGeometryPointer_ToNonConstArray.htm)

## SimpleArrayGuid (class)

Wrapper for ON_SimpleArray<ON_UUID>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayGuid.htm)

### Constructors
- `public SimpleArrayGuid()` — Initializes a new SimpleArrayGuid class.
- `public SimpleArrayGuid(IEnumerable<Guid> values)` — Initializes a new SimpleArrayGuid class

### Methods
#### `public void Append(Guid uuid)`

Appends a new Guid at the end of this array.

**Parameters:**
- `uuid` (System.Guid) — [Missing <param name="uuid"/> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayGuid.Append(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuid_Append.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuid_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuid_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuid_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuid_NonConstPointer.htm)

#### `public Guid[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `Guid[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuid_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.
- `Item` (Guid, get) — Get the Guid at index

## SimpleArrayGuidPointer (class)

Wrapper for ON_SimpleArray<ON_UUID>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayGuidPointer.htm)

### Constructors
- `public SimpleArrayGuidPointer()` — Initializes a new SimpleArrayGuidPointer class.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuidPointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuidPointer_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuidPointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuidPointer_NonConstPointer.htm)

#### `public Guid[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `Guid[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayGuidPointer_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.
- `Item` (Guid, get) — Get the Guid at index

## SimpleArrayInt (class)

Wrapper for ON_SimpleArray<int>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayInt.htm)

### Constructors
- `public SimpleArrayInt()` — Initializes a new SimpleArrayInt class.
- `public SimpleArrayInt(IEnumerable<int> values)` — Initializes a new SimpleArrayInt class

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInt_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInt_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInt_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInt_NonConstPointer.htm)

#### `public int[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `Int32[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInt_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.

## SimpleArrayInterval (class)

Wrapper for ON_SimpleArray<ON_Interval>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayInterval.htm)

### Constructors
- `public SimpleArrayInterval()` — Initializes a new SimpleArrayInterval class.

### Methods
#### `public void Add(Interval interval)`

Adds a new Interval at the end of this array.

**Parameters:**
- `interval` (Rhino.Geometry.Interval) — [Missing <param name="interval"/> documentation for "M:Rhino.Runtime.InteropWrappers.SimpleArrayInterval.Add(Rhino.Geometry.Interval)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInterval_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInterval_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInterval_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInterval_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInterval_NonConstPointer.htm)

#### `public Interval[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `Interval[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayInterval_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of elements in this array.

## SimpleArrayLine (class)

Wrapper for ON_SimpleArray<ON_Line>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayLine.htm)

### Constructors
- `public SimpleArrayLine()` — Initializes a new SimpleArrayLine instance.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLine_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLine_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLine_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLine_NonConstPointer.htm)

#### `public Line[] ToArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Line[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLine_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of lines in this array.

## SimpleArrayLinetypePointer (class)

Wrapper for a C++ ON_SimpleArray<ON_Linetype*> If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayLinetypePointer.htm)

### Constructors
- `public SimpleArrayLinetypePointer()` — Initializes a new SimpleArrayLinetypePointer instance.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLinetypePointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLinetypePointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Dispose

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLinetypePointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLinetypePointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLinetypePointer_NonConstPointer.htm)

#### `public Linetype[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Linetype[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayLinetypePointer_ToNonConstArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of linetypes in this array.

## SimpleArrayMeshPointer (class)

Represents a wrapper to an unmanaged array of mesh pointers. Wrapper for a C++ ON_SimpleArray of ON_Mesh* or constant ON_Mesh*. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer.htm)

### Constructors
- `public SimpleArrayMeshPointer()` — Initializes a new SimpleArrayMeshPointer instance.

### Methods
#### `public void Add(Mesh mesh, bool asConst)`

Adds a mesh to the list.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to add.
- `asConst` (System.Boolean) — Whether this mesh should be treated as non-modifiable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the SimpleArrayMeshPointer and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer_NonConstPointer.htm)

#### `public Mesh[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Mesh[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayMeshPointer_ToNonConstArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of meshes in this array.

## SimpleArrayPlane (class)

Wrapper for ON_SimpleArray<ON_Plane>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayPlane.htm)

### Constructors
- `public SimpleArrayPlane()` — Initializes a new SimpleArrayLine instance.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPlane_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPlane_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPlane_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPlane_NonConstPointer.htm)

#### `public Plane[] ToArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Plane[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPlane_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of lines in this array.

## SimpleArrayPoint2d (class)

ON_SimpleArray<ON_2dPoint> class wrapper. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayPoint2d.htm)

### Constructors
- `public SimpleArrayPoint2d()` — Initializes a new empty SimpleArrayPoint3d instance.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint2d_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint2d_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint2d_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint2d_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint2d_NonConstPointer.htm)

#### `public Point2d[] ToArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Point2d[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint2d_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of points in this array.

## SimpleArrayPoint3d (class)

ON_SimpleArray<ON_3dPoint>, ON_3dPointArray, ON_PolyLine all have the same size This class wraps all of these C++ versions. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayPoint3d.htm)

### Constructors
- `public SimpleArrayPoint3d()` — Initializes a new empty SimpleArrayPoint3d instance.
- `public SimpleArrayPoint3d(IEnumerable<Point3d> pts)` — Initializes a new SimpleArrayPoint3d instance from a set of points

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint3d_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint3d_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint3d_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint3d_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint3d_NonConstPointer.htm)

#### `public Point3d[] ToArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `Point3d[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayPoint3d_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of points in this array.

## SimpleArraySubDPointer (class)

Wrapper for a C++ ON_SimpleArray<ON_SubD*> or ON_SimpleArray<constant ON_SubD*> If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer.htm)

### Constructors
- `public SimpleArraySubDPointer()` — Initializes a new SimpleArraySubDPointer instance.

### Methods
#### `public void Add(SubD subd, bool asConst)`

Adds a subd to the list.

**Parameters:**
- `subd` (Rhino.Geometry.SubD) — A subd to add.
- `asConst` (System.Boolean) — Whether this subd should be treated as non-modifiable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer_Add.htm)

#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the SimpleArraySubDPointer and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer_NonConstPointer.htm)

#### `public SubD[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart.

**Returns:** `SubD[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySubDPointer_ToNonConstArray.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of subds in this array.

## SimpleArraySurfacePointer (class)

Wrapper for a C++ ON_SimpleArray of ON_Surface* or constant ON_Surface*. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArraySurfacePointer.htm)

### Constructors
- `public SimpleArraySurfacePointer()` — Initializes a new SimpleArraySurfacePointer instance.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySurfacePointer_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySurfacePointer_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySurfacePointer_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySurfacePointer_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySurfacePointer_NonConstPointer.htm)

#### `public Surface[] ToNonConstArray()`

Copies the unmanaged array to a managed counterpart. Elements are made non-constant.

**Returns:** `Surface[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArraySurfacePointer_ToNonConstArray.htm)

## SimpleArrayUint (class)

Wrapper for ON_SimpleArray<unsigned int>. If you are not writing C++ code then this class is not for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_SimpleArrayUint.htm)

### Constructors
- `public SimpleArrayUint()` — Initializes a new SimpleArrayInt class.
- `public SimpleArrayUint(IEnumerable<uint> values)` — Initializes a new SimpleArrayInt class.

### Methods
#### `public IntPtr ConstPointer()`

Gets the constant (immutable) pointer of this array.

**Returns:** `IntPtr` — The constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayUint_ConstPointer.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayUint_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayUint_Finalize.htm)

#### `public IntPtr NonConstPointer()`

Gets the non-constant pointer (for modification) of this array.

**Returns:** `IntPtr` — The non-constant pointer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayUint_NonConstPointer.htm)

#### `public uint[] ToArray()`

Returns the managed counterpart of the unmanaged array.

**Returns:** `UInt32[]` — The managed array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_SimpleArrayUint_ToArray.htm)

### Properties
- `Count` (Int32, get) — Gets the number of elements in this array.
- `UnsignedCount` (UInt32, get) — Gets the number of elements in this array.

## StringHolder (class)

This class is used to pass strings back and forth between managed and unmanaged code. This should not be needed by plug-ins. If you are just dealing with an ON_wString*, use StringWrapper

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_StringHolder.htm)

### Constructors
- `public StringHolder()` — Constructor

### Methods
#### `public IntPtr ConstPointer()`

C++ pointer used to access the ON_wString, managed plug-ins should never need this.

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.StringHolder.ConstPointer"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringHolder_ConstPointer.htm)

#### `public void Dispose()`

IDispose implementation

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringHolder_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Called by Dispose and finalizer

**Parameters:**
- `disposing` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringHolder_Dispose_1.htm)

#### `protected override void Finalize()`

Destructor

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringHolder_Finalize.htm)

#### `public static string GetString(IntPtr pStringHolder)`

Gets managed string from unmanaged ON_wString pointer.

**Parameters:**
- `pStringHolder` (System.IntPtr)

**Returns:** `String` — Null if pStringHolder has no reference, otherwise, the string. This may be an empty string, if setting an empty string is possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringHolder_GetString.htm)

#### `public IntPtr NonConstPointer()`

C++ pointer used to access the ON_wString, managed plug-ins should never need this.

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.StringHolder.NonConstPointer"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringHolder_NonConstPointer.htm)

#### `public override string ToString()`

Marshals unmanaged ON_wString to a managed .NET string

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.StringHolder.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringHolder_ToString.htm)

## StringWrapper (class)

Represents a wrapper to an unmanaged OpenNurbs string. Wraps a C++ ON_wString*.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_InteropWrappers_StringWrapper.htm)

### Constructors
- `public StringWrapper()` — Initializes a new empty unmanaged string (ON_wString*).
- `public StringWrapper(string s)` — Initializes a new unmanaged string with an initial value. The string s can be null.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringWrapper_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringWrapper_Finalize.htm)

#### `public static string GetStringFromPointer(IntPtr pConstON_wString)`

Get string from an ON_wString*

**Parameters:**
- `pConstON_wString` (System.IntPtr) — [Missing <param name="pConstON_wString"/> documentation for "M:Rhino.Runtime.InteropWrappers.StringWrapper.GetStringFromPointer(System.IntPtr)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.InteropWrappers.StringWrapper.GetStringFromPointer(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringWrapper_GetStringFromPointer.htm)

#### `public void SetString(string s)`

Set contents of this string.

**Parameters:**
- `s` (System.String) — The new string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringWrapper_SetString.htm)

#### `public static void SetStringOnPointer(IntPtr pON_wString, string s)`

Set contents of an ON_wString*

**Parameters:**
- `pON_wString` (System.IntPtr) — [Missing <param name="pON_wString"/> documentation for "M:Rhino.Runtime.InteropWrappers.StringWrapper.SetStringOnPointer(System.IntPtr,System.String)"]
- `s` (System.String) — [Missing <param name="s"/> documentation for "M:Rhino.Runtime.InteropWrappers.StringWrapper.SetStringOnPointer(System.IntPtr,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringWrapper_SetStringOnPointer.htm)

#### `public override string ToString()`

Returns the string contents of this wrapper.

**Returns:** `String` — A managed string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_InteropWrappers_StringWrapper_ToString.htm)

### Properties
- `ConstPointer` (IntPtr, get) — Gets the constant pointer (constant ON_wString*).
- `NonConstPointer` (IntPtr, get) — Gets the non-constant pointer (ON_wString*).

