---
name: rhino-rhino-collections
description: This skill encodes the rhino 8.0 surface of the Rhino.Collections namespace — 10 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ArchivableDictionary, Point3dList, CurveList, Point3dList.XAccess, Point3dList.ZAccess, Point3dList.YAccess, RhinoList, RhinoList<T>, and 2 more types.
---

# Rhino.Collections

Auto-generated from vendor docs for rhino 8.0. 10 types in this namespace.

## ArchivableDictionary (class)

Represents a dictionary class that can be attached to objects and can be serialized (saved) at necessity.See remarks for layout.

**Remarks:** This is the layout of this object:.BEGINCHUNK (TCODE_ANONYMOUS_CHUNK)|- version (int)|- entry count (int) for entry count entries |- BEGINCHUNK (TCODE_ANONYMOUS_CHUNK) |- key (string) |- entry contents |- ENDCHUNK (TCODE_ANONYMOUS_CHUNK)ENDCHUNK (TCODE_ANONYMOUS_CHUNK)

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_ArchivableDictionary.htm)

### Constructors
- `public ArchivableDictionary()` — Initializes an instance of a dictionary for writing to a 3dm archive.
- `public ArchivableDictionary(int version)` — Initializes an instance of a dictionary for writing to a 3dm archive.
- `public ArchivableDictionary(UserData parentUserData)` — Initializes an instance of a dictionary for writing to a 3dm archive
- `public ArchivableDictionary(int version, string name)` — Initializes an instance of a dictionary for writing to a 3dm archive.
- `protected ArchivableDictionary(SerializationInfo info, StreamingContext context)` — Protected constructor for internal use.

### Methods
#### `public bool AddContentsFrom(ArchivableDictionary source)`

Add the contents from the source dictionary.

**Parameters:**
- `source` (Rhino.Collections.ArchivableDictionary) — [Missing <param name="source"/> documentation for "M:Rhino.Collections.ArchivableDictionary.AddContentsFrom(Rhino.Collections.ArchivableDictionary)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.AddContentsFrom(Rhino.Collections.ArchivableDictionary)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_AddContentsFrom.htm)

#### `public void Clear()`

Removes all keys and values from the dictionary.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Clear.htm)

#### `public ArchivableDictionary Clone()`

Constructs a deep copy of this object.

**Returns:** `ArchivableDictionary` — The copy of this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Clone.htm)

#### `public bool ContainsKey(string key)`

Determines whether the dictionary contains the specified key.

**Parameters:**
- `key` (System.String) — The key to locate.

**Returns:** `Boolean` — true if the dictionary contains an element with the specified key; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_ContainsKey.htm)

#### `public bool GetBool(string key)`

Get value as Boolean, will only succeed if value was created using Set(string key, Boolean value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBool(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBool(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetBool.htm)

#### `public bool GetBool(string key, bool defaultValue)`

Get value as Boolean, will return defaultValue unless value was created using Set(string key, Boolean value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBool(System.String,System.Boolean)"]
- `defaultValue` (System.Boolean) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBool(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBool(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetBool_1.htm)

#### `public byte[] GetBytes(string key)`

Get value as byte[], will only succeed if value was created using Set(string key, byte[] value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBytes(System.String)"]

**Returns:** `Byte[]` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBytes(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetBytes.htm)

#### `public byte[] GetBytes(string key, byte[] defaultValue)`

Get value as byte[], will return defaultValue unless value was created using Set(string key, byte[] value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBytes(System.String,System.Byte[])"]
- `defaultValue` (System.Byte[]) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBytes(System.String,System.Byte[])"]

**Returns:** `Byte[]` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetBytes(System.String,System.Byte[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetBytes_1.htm)

#### `public ArchivableDictionary GetDictionary(string key)`

Get value as ArchivableDictionary, will only succeed if value was created using Set(string key, ArchivableDictionary value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDictionary(System.String)"]

**Returns:** `ArchivableDictionary` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDictionary(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetDictionary.htm)

#### `public ArchivableDictionary GetDictionary(string key, ArchivableDictionary defaultValue)`

Get value as ArchivableDictionary, will return defaultValue unless value was created using Set(string key, ArchivableDictionary value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDictionary(System.String,Rhino.Collections.ArchivableDictionary)"]
- `defaultValue` (Rhino.Collections.ArchivableDictionary) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDictionary(System.String,Rhino.Collections.ArchivableDictionary)"]

**Returns:** `ArchivableDictionary` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDictionary(System.String,Rhino.Collections.ArchivableDictionary)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetDictionary_1.htm)

#### `public double GetDouble(string key)`

Get value as double, will only succeed if value was created using Set(string key, double value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDouble(System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDouble(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetDouble.htm)

#### `public double GetDouble(string key, double defaultValue)`

Get value as double, will only succeed if value was created using Set(string key, double value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDouble(System.String,System.Double)"]
- `defaultValue` (System.Double) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDouble(System.String,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetDouble(System.String,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetDouble_1.htm)

#### `public T GetEnumValue<T>() where T : struct, new(), IConvertible`

Get an enum value

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetEnumValue__1.htm)

#### `public T GetEnumValue<T>(string key) where T : struct, new(), IConvertible`

Get an enum value from the dictionary using a custom key.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetEnumValue``1(System.String)"]

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetEnumValue``1(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetEnumValue__1_1.htm)

#### `public IEnumerator<KeyValuePair<string, Object>> GetEnumerator()`

Gets the enumerator of this dictionary.

**Returns:** `IEnumerator<KeyValuePair<String,Object>>` — A IEnumerator<T>, where T is an instance of KeyValuePair<TKey, TValue>, with T0 set as string, and T1 as System.Object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetEnumerator.htm)

#### `public float GetFloat(string key)`

Get value as float, will only succeed if value was created using Set(string key, float value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetFloat(System.String)"]

**Returns:** `Single` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetFloat(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetFloat.htm)

#### `public float GetFloat(string key, float defaultValue)`

Get value as float, will return defaultValue unless value was created using Set(string key, float value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetFloat(System.String,System.Single)"]
- `defaultValue` (System.Single) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetFloat(System.String,System.Single)"]

**Returns:** `Single` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetFloat(System.String,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetFloat_1.htm)

#### `public Guid GetGuid(string key)`

Get value as Guid, will only succeed if value was created using Set(string key, Guid value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetGuid(System.String)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetGuid(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetGuid.htm)

#### `public Guid GetGuid(string key, Guid defaultValue)`

Get value as Guid, will return defaultValue unless value was created using Set(string key, Guid value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetGuid(System.String,System.Guid)"]
- `defaultValue` (System.Guid) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetGuid(System.String,System.Guid)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetGuid(System.String,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetGuid_1.htm)

#### `public int GetInteger(string key)`

Get value as int, will only succeed if value was created using Set(string key, int value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetInteger(System.String)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetInteger(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetInteger.htm)

#### `public int GetInteger(string key, int defaultValue)`

Get value as int, will return defaultValue unless value was created using Set(string key, int value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetInteger(System.String,System.Int32)"]
- `defaultValue` (System.Int32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetInteger(System.String,System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetInteger(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetInteger_1.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetObjectData.htm)

#### `public Plane GetPlane(string key)`

Get value as Plane, will return defaultValue unless value was created using Set(string key, Plane value)

**Parameters:**
- `key` (System.String) — The key.

**Returns:** `Plane` — The value as Plane.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetPlane.htm)

#### `public Plane GetPlane(string key, Plane defaultValue)`

Get value as Plane, will return defaultValue unless value was created using Set(string key, Plane value)

**Parameters:**
- `key` (System.String) — The key.
- `defaultValue` (Rhino.Geometry.Plane) — The default value.

**Returns:** `Plane` — The value as Plane.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetPlane_1.htm)

#### `public Point3d GetPoint3d(string key)`

Get value as Point3d, will only succeed if value was created using Set(string key, Point3d value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3d(System.String)"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3d(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetPoint3d.htm)

#### `public Point3d GetPoint3d(string key, Point3d defaultValue)`

Get value as Point3d, will return defaultValue unless value was created using Set(string key, Point3d value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3d(System.String,Rhino.Geometry.Point3d)"]
- `defaultValue` (Rhino.Geometry.Point3d) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3d(System.String,Rhino.Geometry.Point3d)"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3d(System.String,Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetPoint3d_1.htm)

#### `public Point3f GetPoint3f(string key)`

Get value as Point3f, will only succeed if value was created using Set(string key, Point3f value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3f(System.String)"]

**Returns:** `Point3f` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3f(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetPoint3f.htm)

#### `public Point3f GetPoint3f(string key, Point3f defaultValue)`

Get value as Point3f, will return defaultValue unless value was created using Set(string key, Point3f value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3f(System.String,Rhino.Geometry.Point3f)"]
- `defaultValue` (Rhino.Geometry.Point3f) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3f(System.String,Rhino.Geometry.Point3f)"]

**Returns:** `Point3f` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetPoint3f(System.String,Rhino.Geometry.Point3f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetPoint3f_1.htm)

#### `public string GetString(string key)`

Get value as string, will only succeed if value was created using Set(string key, string value)

**Parameters:**
- `key` (System.String) — The key which points to the string

**Returns:** `String` — The string

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetString.htm)

#### `public string GetString(string key, string defaultValue)`

Get value as string, will return defaultValue unless value was created using Set(string key, string value)

**Parameters:**
- `key` (System.String) — The key which points to the string
- `defaultValue` (System.String) — The string

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetString(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetString_1.htm)

#### `public Vector3d GetVector3d(string key)`

Get value as Vector3d, will only succeed if value was created using Set(string key, Vector3d value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetVector3d(System.String)"]

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetVector3d(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetVector3d.htm)

#### `public Vector3d GetVector3d(string key, Vector3d defaultValue)`

Get value as Vector3d, will return defaultValue unless value was created using Set(string key, Vector3d value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetVector3d(System.String,Rhino.Geometry.Vector3d)"]
- `defaultValue` (Rhino.Geometry.Vector3d) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.GetVector3d(System.String,Rhino.Geometry.Vector3d)"]

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.GetVector3d(System.String,Rhino.Geometry.Vector3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_GetVector3d_1.htm)

#### `public int Getint(string key, int defaultValue)`

Get value as int, will return defaultValue unless value was created using Set(string key, int value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.Getint(System.String,System.Int32)"]
- `defaultValue` (System.Int32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.Getint(System.String,System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Getint(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Getint.htm)

#### `public bool Remove(string key)`

Removes the value with the specified key from the dictionary.

**Parameters:**
- `key` (System.String) — The key of the element to remove.

**Returns:** `Boolean` — true if the element is successfully found and removed; otherwise, false. This method returns false if key is not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Remove.htm)

#### `public bool RemoveEnumValue<T>() where T : struct, new(), IConvertible`

Remove an enum value from the dictionary.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.RemoveEnumValue``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_RemoveEnumValue__1.htm)

#### `public bool ReplaceContentsWith(ArchivableDictionary source)`

Replace the contents of the dictionary with that of the given source dictionary.

**Parameters:**
- `source` (Rhino.Collections.ArchivableDictionary) — [Missing <param name="source"/> documentation for "M:Rhino.Collections.ArchivableDictionary.ReplaceContentsWith(Rhino.Collections.ArchivableDictionary)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.ReplaceContentsWith(Rhino.Collections.ArchivableDictionary)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_ReplaceContentsWith.htm)

#### `public bool Set(string key, ArchivableDictionary val)`

Sets another ArchivableDictionary as entry in this dictionary.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Collections.ArchivableDictionary) — An object for that key. Because this class is a reference type and is mutable, changes to this object will propagate to the object inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Collections.ArchivableDictionary)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set.htm)

#### `public bool Set(string key, bool val)`

Sets a Boolean.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Boolean) — A Boolean value. Because Boolean has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_17.htm)

#### `public bool Set(string key, BoundingBox val)`

Sets a BoundingBox.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.BoundingBox) — A value for that key. Because BoundingBox has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.BoundingBox)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_2.htm)

#### `public bool Set(string key, byte val)`

Sets a Byte.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Byte) — A Byte. Because Byte has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_18.htm)

#### `public bool Set(string key, Color val)`

Sets a Color.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.Color) — A value for that key. Because Color has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_31.htm)

#### `public bool Set(string key, double val)`

Sets a Double.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Double) — A Double. Because Double has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_30.htm)

#### `public bool Set(string key, Font val)`

Sets a Font.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.Font) — A value for that key. Because Font is immutable, it is not possible to modify the object while it is in this dictionary.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Drawing.Font)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_32.htm)

#### `public bool Set(string key, GeometryBase val)`

Sets any class deriving from the GeometryBase base class.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.GeometryBase) — A geometry object for that key. Because this class is a reference type and is mutable, changes to this object will propagate to the object inside the dictionary.It is up to the user to clone this entry when appropriate. You can use Duplicate() for this.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.GeometryBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_3.htm)

#### `public bool Set(string key, Guid val)`

Sets a Guid.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Guid) — A Guid. Because Guid has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_39.htm)

#### `public bool Set(string key, IEnumerable<bool> val)`

Sets a list, an array or any enumerable of Boolean.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<Boolean>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_21.htm)

#### `public bool Set(string key, IEnumerable<byte> val)`

Sets a list, an array or any enumerable of Byte.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<Byte>) — A value for that key Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_22.htm)

#### `public bool Set(string key, IEnumerable<double> val)`

Sets a list, an array or any enumerable of Double.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<Double>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_23.htm)

#### `public bool Set(string key, IEnumerable<GeometryBase> val)`

Sets an array of GeometryBase

**Parameters:**
- `key` (System.String) — A text key
- `val` (System.Collections.Generic.IEnumerable<GeometryBase>) — An object for that key Because this class is a reference type and is mutable, changes to this object will propagate to the object inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_20.htm)

#### `public bool Set(string key, IEnumerable<Guid> val)`

Sets a list, an array or any enumerable of Guid.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<Guid>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_24.htm)

#### `public bool Set(string key, IEnumerable<short> val)`

Sets a list, an array or any enumerable of Int16.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<Int16>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_25.htm)

#### `public bool Set(string key, IEnumerable<int> val)`

Sets a list, an array or any enumerable of Int32.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<Int32>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_26.htm)

#### `public bool Set(string key, IEnumerable<ObjRef> val)`

Sets an array of ObjRef

**Parameters:**
- `key` (System.String) — A text key
- `val` (System.Collections.Generic.IEnumerable<ObjRef>) — An object for that key Because this class is a reference type and is mutable, changes to this object will propagate to the object inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_19.htm)

#### `public bool Set(string key, IEnumerable<sbyte> val)`

Sets a list, an array or any enumerable of SByte.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<SByte>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_27.htm)

#### `public bool Set(string key, IEnumerable<float> val)`

Sets a list, an array or any enumerable of Single.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<Single>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_28.htm)

#### `public bool Set(string key, IEnumerable<string> val)`

Sets a list, an array or any enumerable of String.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Collections.Generic.IEnumerable<String>) — A value for that key. Because this interface is a reference type, changes to the assigned object will modify this entry inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_29.htm)

#### `public bool Set(string key, short val)`

Sets a Int16.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Int16) — A Int16. Because Int16 has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_40.htm)

#### `public bool Set(string key, int val)`

Sets a Int32.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Int32) — A Int32. Because Int32 has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_41.htm)

#### `public bool Set(string key, long val)`

Sets a Int64.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Int64) — A Int64. Because Int64 has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_42.htm)

#### `public bool Set(string key, Interval val)`

Sets an Interval.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Interval) — A value for that key. Because Interval has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Interval)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_4.htm)

#### `public bool Set(string key, Line val)`

Sets a Line.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Line) — A value for that key. Because Line has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Line)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_5.htm)

#### `public bool Set(string key, MeshingParameters val)`

Sets a MeshingParameters.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.MeshingParameters) — An object for that key. Because this class is a reference type and is mutable, changes to this object will propagate to the object inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.MeshingParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_6.htm)

#### `public bool Set(string key, ObjRef val)`

Sets a ObjRef

**Parameters:**
- `key` (System.String) — A text key
- `val` (Rhino.DocObjects.ObjRef) — An object for that key Because this class is a reference type and is mutable, changes to this object will propagate to the object inside the dictionary.It is up to the user to clone this entry when appropriate.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.DocObjects.ObjRef)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_1.htm)

#### `public bool Set(string key, Plane val)`

Sets a Plane.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Plane) — A plane for that key. Because Plane has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Plane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_7.htm)

#### `public bool Set(string key, Point val)`

Sets a Point.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.Point) — A value for that key. Because Point has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_33.htm)

#### `public bool Set(string key, Point2d val)`

Sets a Point2d.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Point2d) — A point for that key. Because Point2d has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_8.htm)

#### `public bool Set(string key, Point3d val)`

Sets a Point3d.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Point3d) — A point for that key. Because Point3d has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_9.htm)

#### `public bool Set(string key, Point3f val)`

Sets a Point3f.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Point3f) — A value for that key. Because Point3f has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Point3f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_10.htm)

#### `public bool Set(string key, Point4d val)`

Sets a Point4d.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Point4d) — A value for that key. Because Point4d has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Point4d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_11.htm)

#### `public bool Set(string key, PointF val)`

Sets a PointF.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.PointF) — A value for that key. Because PointF has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Drawing.PointF)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_34.htm)

#### `public bool Set(string key, Ray3d val)`

Sets a Ray3d.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Ray3d) — A value for that key. Because Ray3d has value semantics and is immutable, no changes to this object are possible.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Ray3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_12.htm)

#### `public bool Set(string key, Rectangle val)`

Sets a Rectangle.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.Rectangle) — A value for that key. Because Rectangle has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Drawing.Rectangle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_35.htm)

#### `public bool Set(string key, RectangleF val)`

Sets a RectangleF.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.RectangleF) — A value for that key. Because RectangleF has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Drawing.RectangleF)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_36.htm)

#### `public bool Set(string key, sbyte val)`

Sets a SByte.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.SByte) — A SByte. Because SByte has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_43.htm)

#### `public bool Set(string key, float val)`

Sets a Single.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.Single) — A Single. Because Single has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_44.htm)

#### `public bool Set(string key, Size val)`

Sets a Size.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.Size) — A value for that key. Because Size has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_37.htm)

#### `public bool Set(string key, SizeF val)`

Sets a SizeF.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (System.Drawing.SizeF) — A value for that key. Because SizeF has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,System.Drawing.SizeF)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_38.htm)

#### `public bool Set(string key, string val)`

Sets a String.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.String) — A String. Because String is immutable, it is not possible to modify the object while it is in this dictionary.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_45.htm)

#### `public bool Set(string key, Transform val)`

Sets a Transform.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Transform) — A transform for that key. Because Transform has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_13.htm)

#### `public bool Set(string key, ushort val)`

Sets a UInt16.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.UInt16) — A UInt16. Because UInt16 has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_46.htm)

#### `public bool Set(string key, uint val)`

Sets a UInt32.

**Parameters:**
- `key` (System.String) — The text key.
- `val` (System.UInt32) — A UInt32. Because UInt32 has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — true if set operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_47.htm)

#### `public bool Set(string key, Vector2d val)`

Sets a Vector2d.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Vector2d) — A value for that key. Because Vector2d has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Vector2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_14.htm)

#### `public bool Set(string key, Vector3d val)`

Sets a Vector3d.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Vector3d) — A value for that key. Because Vector3d has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Vector3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_15.htm)

#### `public bool Set(string key, Vector3f val)`

Sets a Vector3f.

**Parameters:**
- `key` (System.String) — A text key.
- `val` (Rhino.Geometry.Vector3f) — A value for that key. Because Vector3f has value semantics, changes to the assigning value will leave this entry unchanged.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.Set(System.String,Rhino.Geometry.Vector3f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_Set_16.htm)

#### `public bool SetEnumValue<T>(string key, T enumValue) where T : struct, new(), IConvertible`

Set an enum value in the dictionary with a custom key.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.SetEnumValue``1(System.String,``0)"]
- `enumValue` (T) — [Missing <param name="enumValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.SetEnumValue``1(System.String,``0)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.SetEnumValue``1(System.String,``0)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_SetEnumValue__1.htm)

#### `public bool SetEnumValue<T>(T enumValue) where T : struct, new(), IConvertible`

Set an enum value

**Parameters:**
- `enumValue` (T) — [Missing <param name="enumValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.SetEnumValue``1(``0)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.SetEnumValue``1(``0)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_SetEnumValue__1_1.htm)

#### `public bool TryGetBool(string key, out bool value)`

Get value as Boolean, will only succeed if value was created using Set(string key, Boolean value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetBool(System.String,System.Boolean@)"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetBool(System.String,System.Boolean@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetBool(System.String,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetBool.htm)

#### `public bool TryGetBytes(string key, out byte[] value)`

Get value as byte[], will only succeed if value was created using Set(string key, byte[] value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetBytes(System.String,System.Byte[]@)"]
- `value` (System.Byte[]) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetBytes(System.String,System.Byte[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetBytes(System.String,System.Byte[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetBytes.htm)

#### `public bool TryGetDictionary(string key, out ArchivableDictionary value)`

Get value as ArchivableDictionary, will only succeed if value was created using Set(string key, ArchivableDictionary value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetDictionary(System.String,Rhino.Collections.ArchivableDictionary@)"]
- `value` (Rhino.Collections.ArchivableDictionary) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetDictionary(System.String,Rhino.Collections.ArchivableDictionary@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetDictionary(System.String,Rhino.Collections.ArchivableDictionary@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetDictionary.htm)

#### `public bool TryGetDouble(string key, out double value)`

Get value as double, will only succeed if value was created using Set(string key, double value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetDouble(System.String,System.Double@)"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetDouble(System.String,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetDouble(System.String,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetDouble.htm)

#### `public bool TryGetEnumValue<T>(string key, out T enumValue) where T : struct, new(), IConvertible`

Attempt to get an enum value from the dictionary using a custom key.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetEnumValue``1(System.String,``0@)"]
- `enumValue` (T) — [Missing <param name="enumValue"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetEnumValue``1(System.String,``0@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetEnumValue``1(System.String,``0@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetEnumValue__1.htm)

#### `public bool TryGetFloat(string key, out float value)`

Get value as float, will only succeed if value was created using Set(string key, float value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetFloat(System.String,System.Single@)"]
- `value` (System.Single) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetFloat(System.String,System.Single@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetFloat(System.String,System.Single@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetFloat.htm)

#### `public bool TryGetGuid(string key, out Guid value)`

Get value as Guid, will only succeed if value was created using Set(string key, Guid value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetGuid(System.String,System.Guid@)"]
- `value` (System.Guid) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetGuid(System.String,System.Guid@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetGuid(System.String,System.Guid@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetGuid.htm)

#### `public bool TryGetInteger(string key, out int value)`

Get value as int, will only succeed if value was created using Set(string key, int value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetInteger(System.String,System.Int32@)"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetInteger(System.String,System.Int32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetInteger(System.String,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetInteger.htm)

#### `public bool TryGetPlane(string key, out Plane value)`

Get value as Plane, will only succeed if value was created using Set(string key, Plane value)

**Parameters:**
- `key` (System.String) — The key.
- `value` (Rhino.Geometry.Plane) — The value.

**Returns:** `Boolean` — The value as Plane.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetPlane.htm)

#### `public bool TryGetPoint3d(string key, out Point3d value)`

Get value as Point3d, will only succeed if value was created using Set(string key, Point3d value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@)"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetPoint3d.htm)

#### `public bool TryGetPoint3f(string key, out Point3f value)`

Get value as Point3f, will only succeed if value was created using Set(string key, Point3f value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetPoint3f(System.String,Rhino.Geometry.Point3f@)"]
- `value` (Rhino.Geometry.Point3f) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetPoint3f(System.String,Rhino.Geometry.Point3f@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetPoint3f(System.String,Rhino.Geometry.Point3f@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetPoint3f.htm)

#### `public bool TryGetString(string key, out string value)`

Get value as string, will only succeed if value was created using Set(string key, string value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetString(System.String,System.String@)"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetString(System.String,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetString(System.String,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetString.htm)

#### `public bool TryGetValue(string key, out Object value)`

Gets the value associated with the specified key.

**Parameters:**
- `key` (System.String) — The key of the value to get.
- `value` (System.Object) — When this method returns and if the key is found, contains the value associated with the specified key; otherwise, null. This parameter is passed uninitialized.

**Returns:** `Boolean` — true if the dictionary contains an element with the specified key; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetValue.htm)

#### `public bool TryGetVector3d(string key, out Vector3d value)`

Get value as Vector3d, will only succeed if value was created using Set(string key, Vector3d value)

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetVector3d(System.String,Rhino.Geometry.Vector3d@)"]
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetVector3d(System.String,Rhino.Geometry.Vector3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.ArchivableDictionary.TryGetVector3d(System.String,Rhino.Geometry.Vector3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_ArchivableDictionary_TryGetVector3d.htm)

### Properties
- `ChangeSerialNumber` (UInt32, get) — Retrieve current change serial number. This is a number that gets increased each time a datum is set or changed.
- `Count` (Int32, get) — Gets the number of key/value pairs contained in the dictionary.
- `Item` (Object, get/set) — Gets the value associated with the specified key.
- `Keys` (String[], get) — Gets all entry names or keys.
- `Name` (String, get/set) — Gets or sets the name string of this ArchivableDictionary.
- `ParentUserData` (UserData, get) — If this dictionary is part of user-data (or is a UserDictionary), then this is the parent user data. null if this dictionary is not part of user-data
- `Values` (Object[], get) — Gets all values in this dictionary.
- `Version` (Int32, get/set) — Gets or sets the version of this ArchivableDictionary.

## CurveList (class)

Represents a list of curves.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_CurveList.htm)

### Constructors
- `public CurveList()` — Initializes a new empty list of curves.
- `public CurveList(IEnumerable<Curve> collection)` — Initializes a new list that is filled with all items of the input enumerable. Input items are not explicitly duplicated (this is a shallow copy).
- `public CurveList(int initialCapacity)` — Initializes a new empty list of curves with a predefined capacity. This is the amount of items the list will accept before resizing.

### Methods
#### `public void Add(Arc arc)`

Adds an arc to this list.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — An arc value that will be the model of the new internal curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Add.htm)

#### `public void Add(Circle circle)`

Adds a circle to this list.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — A circle value that will be the model of the new internal curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Add_1.htm)

#### `public void Add(Ellipse ellipse)`

Adds an ellipse to this list.

**Parameters:**
- `ellipse` (Rhino.Geometry.Ellipse) — An ellipse that will be the model of the new internal curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Add_2.htm)

#### `public void Add(IEnumerable<Point3d> polyline)`

Adds a polyline to this list.

**Parameters:**
- `polyline` (System.Collections.Generic.IEnumerable<Point3d>) — A polyline value that will be copied in a new polyline. This argument can be null, an array, a list or any enumerable set of Point3d.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Add_4.htm)

#### `public void Add(Line line)`

Adds a line to this list.

**Parameters:**
- `line` (Rhino.Geometry.Line) — A line value that will be the model of the new internal curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Add_3.htm)

#### `public void Add(T item)`

Adds an object to the end of the List.

**Parameters:**
- `item` (T) — Item to append.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Add.htm)

#### `public void AddRange(IEnumerable collection)`

Adds the elements of the specified collection to the end of the List.

**Parameters:**
- `collection` (System.Collections.IEnumerable) — The collection whose elements should be added to the end of the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic). Objects in collection which cannot be represented as T will throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AddRange_1.htm)

#### `public void AddRange(IEnumerable<T> collection)`

Adds the elements of the specified collection to the end of the List.

**Parameters:**
- `collection` (System.Collections.Generic.IEnumerable<T>) — The collection whose elements should be added to the end of the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic), if type T is a reference type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AddRange.htm)

#### `public ReadOnlyCollection<T> AsReadOnly()`

Constructs a read-only wrapper of this class.

**Returns:** `ReadOnlyCollection<T>` — A wrapper.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AsReadOnly.htm)

#### `public int BinarySearch(int index, int count, T item, IComparer<T> comparer)`

Searches the entire sorted List for an element using the specified comparer and returns the zero-based index of the element.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to search.
- `count` (System.Int32) — The length of the range to search.
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements. Or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T)::Default.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch.htm)

#### `public int BinarySearch(T item)`

Searches the entire sorted List for an element using the default comparer and returns the zero-based index of the element.

**Parameters:**
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch_1.htm)

#### `public int BinarySearch(T item, IComparer<T> comparer)`

Searches the entire sorted List for an element using the specified comparer and returns the zero-based index of the element.

**Parameters:**
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements. Or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T)::Default.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch_2.htm)

#### `public void Clear()`

Removes all elements from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Clear.htm)

#### `public bool Contains(T item)`

Determines whether an element is in the List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Boolean` — true if item is found in the List; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Contains.htm)

#### `public RhinoList<TOutput> ConvertAll<TOutput>(Converter<T, TOutput> converter)`

Aggregates all results of a conversion function over this table into a new list.

**Parameters:**
- `converter` (System.Converter<T,TOutput>) — A conversion function that can transform from T to TOutput.

**Returns:** `RhinoList<TOutput>` — The new list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ConvertAll__1.htm)

#### `public void CopyTo(int index, T[] array, int arrayIndex, int count)`

Copies a range of elements from the List to a compatible one-dimensional array, starting at the specified index of the target array.

**Parameters:**
- `index` (System.Int32) — The zero-based index in the source List at which copying begins.
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.
- `count` (System.Int32) — The number of elements to copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo.htm)

#### `public void CopyTo(T[] array)`

Copies the entire List to a compatible one-dimensional array, starting at the beginning of the target array.

**Parameters:**
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo_1.htm)

#### `public void CopyTo(T[] array, int arrayIndex)`

Copies the entire List to a compatible one-dimensional array, starting at the specified index of the target array.

**Parameters:**
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo_2.htm)

#### `public RhinoList<T> Duplicate()`

Returns a shallow copy of this instance. If the generic type is comprised of only value types (struct, enum, ptr), then the result will be a deep copy.

**Returns:** `RhinoList<T>` — The duplicated list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Duplicate.htm)

#### `public bool Exists(Predicate<T> match)`

Determines whether the List contains elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to search for.

**Returns:** `Boolean` — true if the List contains one or more elements that match the conditions defined by the specified predicate; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Exists.htm)

#### `public T Find(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the first occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `T` — The first element that matches the conditions defined by the specified predicate, if found; otherwise, the default value for type T.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Find.htm)

#### `public RhinoList<T> FindAll(Predicate<T> match)`

Retrieves all the elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to search for.

**Returns:** `RhinoList<T>` — A ON_List(T) containing all the elements that match the conditions defined by the specified predicate, if found; otherwise, an empty ON_List(T).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindAll.htm)

#### `public int FindIndex(int startIndex, int count, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the range of elements in the List that extends from the specified index to the last element.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the search.
- `count` (System.Int32) — The number of elements in the section to search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex.htm)

#### `public int FindIndex(int startIndex, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex_1.htm)

#### `public int FindIndex(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex_2.htm)

#### `public T FindLast(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the last occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `T` — The last element that matches the conditions defined by the specified predicate, if found; otherwise, the default value for type T.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLast.htm)

#### `public int FindLastIndex(int startIndex, int count, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the backward search.
- `count` (System.Int32) — The number of elements in the section to search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex.htm)

#### `public int FindLastIndex(int startIndex, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the backward search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex_1.htm)

#### `public int FindLastIndex(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex_2.htm)

#### `public void ForEach(Action<T> action)`

Performs the specified action on each element of the List.

**Parameters:**
- `action` (System.Action<T>) — The Action(T) delegate to perform on each element of the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ForEach.htm)

#### `public IEnumerator<T> GetEnumerator()`

Constructs an enumerator that is capable of iterating over all items in this list.

**Returns:** `IEnumerator<T>` — The new enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_GetEnumerator.htm)

#### `public RhinoList<T> GetRange(int index, int count)`

Constructs a shallow copy of a range of elements in the source List.

**Parameters:**
- `index` (System.Int32) — The zero-based List index at which the range starts.
- `count` (System.Int32) — The number of elements in the range.

**Returns:** `RhinoList<T>` — A shallow copy of a range of elements in the source List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_GetRange.htm)

#### `public int IndexOf(T item)`

Searches for the specified object and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf.htm)

#### `public int IndexOf(T item, int index)`

Searches for the specified object and returns the zero-based index of the first occurrence within the range of elements in the List that extends from the specified index to the last element.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the search.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf_1.htm)

#### `public int IndexOf(T item, int index, int count)`

Searches for the specified object and returns the zero-based index of the first occurrence within the range of elements in the List that starts at the specified index and contains the specified number of elements.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the search.
- `count` (System.Int32) — The number of elements in the section to search.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf_2.htm)

#### `public void Insert(int index, Arc arc)`

Inserts an arc at a given index of this list.

**Parameters:**
- `index` (System.Int32) — A 0-based position in the list.
- `arc` (Rhino.Geometry.Arc) — The arc value from which to construct the new curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Insert.htm)

#### `public void Insert(int index, Circle circle)`

Inserts a line at a given index of this list.

**Parameters:**
- `index` (System.Int32) — A 0-based position in the list.
- `circle` (Rhino.Geometry.Circle) — The circle value from which to construct the new curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Insert_1.htm)

#### `public void Insert(int index, Ellipse ellipse)`

Inserts an ellipse at a given index of this list.

**Parameters:**
- `index` (System.Int32) — A 0-based position in the list.
- `ellipse` (Rhino.Geometry.Ellipse) — The ellipse value from which to construct the new curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Insert_2.htm)

#### `public void Insert(int index, IEnumerable<Point3d> polyline)`

Inserts a polyline at a given index of this list.

**Parameters:**
- `index` (System.Int32) — A 0-based position in the list.
- `polyline` (System.Collections.Generic.IEnumerable<Point3d>) — The polyline enumerable from which to construct a copy curve. This argument can be null, an array, a list or any enumerable set of Point3d.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Insert_4.htm)

#### `public void Insert(int index, Line line)`

Inserts a line at a given index of this list.

**Parameters:**
- `index` (System.Int32) — A 0-based position in the list.
- `line` (Rhino.Geometry.Line) — The line value from which to construct the new curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Insert_3.htm)

#### `public void Insert(int index, T item)`

Inserts an element into the List at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero-based index at which item should be inserted.
- `item` (T) — The object to insert. The value can be a null reference (Nothing in Visual Basic) for reference types.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Insert.htm)

#### `public void InsertRange(int index, IEnumerable<T> collection)`

Inserts the elements of a collection into the List at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero-based index at which the new elements should be inserted.
- `collection` (System.Collections.Generic.IEnumerable<T>) — The collection whose elements should be inserted into the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic), if type T is a reference type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_InsertRange.htm)

#### `public int LastIndexOf(T item)`

Searches for the specified object and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf.htm)

#### `public int LastIndexOf(T item, int index)`

Searches for the specified object and returns the zero-based index of the last occurrence within the range of elements in the List that extends from the first element to the specified index.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the backward search.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf_1.htm)

#### `public int LastIndexOf(T item, int index, int count)`

Searches for the specified object and returns the zero-based index of the last occurrence within the range of elements in the List that contains the specified number of elements and ends at the specified index.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the backward search.
- `count` (System.Int32) — The number of elements in the section to search.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf_2.htm)

#### `public int RemapIndex(int index)`

Remap an index in the infinite range onto the List index range.

**Parameters:**
- `index` (System.Int32) — Index to remap.

**Returns:** `Int32` — Remapped index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemapIndex.htm)

#### `public bool Remove(T item)`

Removes the first occurrence of a specific object from the List.

**Parameters:**
- `item` (T) — The object to remove from the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Boolean` — true if item is successfully removed; otherwise, false. This method also returns false if item was not found in the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Remove.htm)

#### `public int RemoveAll(Predicate<T> match)`

Removes the all the elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to remove.

**Returns:** `Int32` — The number of elements removed from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveAll.htm)

#### `public void RemoveAt(int index)`

Removes the element at the specified index of the List.

**Parameters:**
- `index` (System.Int32) — The zero-based index of the element to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveAt.htm)

#### `public int RemoveNulls()`

Removes all elements from the List that are null references (Nothing in Visual Basic). This function will not do anything if T is not a Reference type.

**Returns:** `Int32` — The number of nulls removed from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveNulls.htm)

#### `public void RemoveRange(int index, int count)`

Removes a range of elements from the List.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range of elements to remove.
- `count` (System.Int32) — The number of elements to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveRange.htm)

#### `public void Reverse()`

Reverses the order of the elements in the entire List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Reverse.htm)

#### `public void Reverse(int index, int count)`

Reverses the order of the elements in the specified range.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to reverse.
- `count` (System.Int32) — The number of elements in the range to reverse.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Reverse_1.htm)

#### `public void Sort()`

Sorts the elements in the entire List using the default comparer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort.htm)

#### `public void Sort(Comparison<T> comparison)`

Sorts the elements in the entire list using the specified comparer.

**Parameters:**
- `comparison` (System.Comparison<T>) — The System.Comparison(T) to use when comparing elements.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_2.htm)

#### `public void Sort(double[] keys)`

Sort this list based on a list of numeric keys of equal length. The keys array will not be altered.

**Remarks:** This function does not exist on the DotNET List class. David thought it was a good idea.

**Parameters:**
- `keys` (System.Double[]) — Numeric keys to sort with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_3.htm)

#### `public void Sort(IComparer<T> comparer)`

Sorts the elements in the entire list using the specified System.Comparison(T)

**Parameters:**
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements, or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T).Default.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_1.htm)

#### `public void Sort(int index, int count, IComparer<T> comparer)`

Sorts the elements in a range of elements in list using the specified comparer.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to sort.
- `count` (System.Int32) — The length of the range to sort.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements, or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T).Default.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_4.htm)

#### `public void Sort(int[] keys)`

Sort this list based on a list of numeric keys of equal length. The keys array will not be altered.

**Remarks:** This function does not exist on the DotNET List class. David thought it was a good idea.

**Parameters:**
- `keys` (System.Int32[]) — Numeric keys to sort with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_5.htm)

#### `public T[] ToArray()`

Constructs an array that contains all items in this list.

**Returns:** `T[]` — An array containing all items in this list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ToArray.htm)

#### `public bool Transform(Transform xform)`

Transform all the curves in this list. If at least a single transform failed this function returns false.

**Parameters:**
- `xform` (Rhino.Geometry.Transform) — Transformation to apply to all curves.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.CurveList.Transform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_CurveList_Transform.htm)

#### `public void TrimExcess()`

Sets the capacity to the actual number of elements in the List, if that number is less than a threshold value.

**Remarks:** This function differs from the DotNET implementation of List<T> since that one only trims the excess if the excess exceeds 10% of the list length.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_TrimExcess.htm)

#### `public bool TrueForAll(Predicate<T> match)`

Determines whether every element in the List matches the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions to check against the elements.

**Returns:** `Boolean` — true if every element in the List matches the conditions defined by the specified predicate; otherwise, false. If the list has no elements, the return value is true.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_TrueForAll.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of elements the internal data structure can hold without resizing.
- `Count` (Int32, get) — Gets the number of elements actually contained in the List.
- `First` (T, get/set) — Gets or sets the first item in the list. This is synonymous to calling List[0].
- `Item` (T, get/set) — Gets or sets the element at the specified index.
- `Last` (T, get/set) — Gets or sets the last item in the list. This is synonymous to calling List[Count-1].
- `NullCount` (Int32, get) — Gets the number of null references (Nothing in Visual Basic) in this list. If T is a ValueType, this property always return zero.

## IResizableList<T> (interface)

Provides the ability to resize a generic list by setting the Count property.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_IResizableList_1.htm)

### Properties
- `Count` (Int32, get/set) — Gets or sets the length of the list. This hides (Shadows in Vb.Net) the read-only Count property of the generic list.

## Point3dList (class)

Represents a list of Point3d.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_Point3dList.htm)

### Constructors
- `public Point3dList()` — Initializes a new empty list with default capacity.
- `public Point3dList(IEnumerable<Point3d> collection)` — Initializes a new point list by copying the values from another set.
- `public Point3dList(int initialCapacity)` — Initializes a new point list with a preallocated initial capacity.
- `public Point3dList(params Point3d[] initialPoints)` — Constructs a new point list from values in a point array.

### Methods
#### `public void Add(double x, double y, double z)`

Adds a Point3d to the end of the list with given x,y,z coordinates.

**Parameters:**
- `x` (System.Double) — The X coordinate.
- `y` (System.Double) — The Y coordinate.
- `z` (System.Double) — The Z coordinate.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_Add.htm)

#### `public void Add(T item)`

Adds an object to the end of the List.

**Parameters:**
- `item` (T) — Item to append.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Add.htm)

#### `public void AddRange(IEnumerable collection)`

Adds the elements of the specified collection to the end of the List.

**Parameters:**
- `collection` (System.Collections.IEnumerable) — The collection whose elements should be added to the end of the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic). Objects in collection which cannot be represented as T will throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AddRange_1.htm)

#### `public void AddRange(IEnumerable<T> collection)`

Adds the elements of the specified collection to the end of the List.

**Parameters:**
- `collection` (System.Collections.Generic.IEnumerable<T>) — The collection whose elements should be added to the end of the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic), if type T is a reference type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AddRange.htm)

#### `public ReadOnlyCollection<T> AsReadOnly()`

Constructs a read-only wrapper of this class.

**Returns:** `ReadOnlyCollection<T>` — A wrapper.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AsReadOnly.htm)

#### `public int BinarySearch(int index, int count, T item, IComparer<T> comparer)`

Searches the entire sorted List for an element using the specified comparer and returns the zero-based index of the element.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to search.
- `count` (System.Int32) — The length of the range to search.
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements. Or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T)::Default.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch.htm)

#### `public int BinarySearch(T item)`

Searches the entire sorted List for an element using the default comparer and returns the zero-based index of the element.

**Parameters:**
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch_1.htm)

#### `public int BinarySearch(T item, IComparer<T> comparer)`

Searches the entire sorted List for an element using the specified comparer and returns the zero-based index of the element.

**Parameters:**
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements. Or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T)::Default.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch_2.htm)

#### `public void Clear()`

Removes all elements from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Clear.htm)

#### `public int ClosestIndex(Point3d testPoint)`

Finds the index of the point that is closest to a test point in this list.

**Parameters:**
- `testPoint` (Rhino.Geometry.Point3d) — point to compare against.

**Returns:** `Int32` — index of closest point in the list on success. -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_ClosestIndex.htm)

#### `public static int ClosestIndexInList(IList<Point3d> list, Point3d testPoint)`

Finds the index of the point in a list of points that is closest to a test point.

**Parameters:**
- `list` (System.Collections.Generic.IList<Point3d>) — A list of points.
- `testPoint` (Rhino.Geometry.Point3d) — Point to compare against.

**Returns:** `Int32` — Index of closest point in the list on success or -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_ClosestIndexInList.htm)

#### `public static Point3d ClosestPointInList(IList<Point3d> list, Point3d testPoint)`

Finds the point in a list of points that is closest to a test point.

**Parameters:**
- `list` (System.Collections.Generic.IList<Point3d>) — A list of points.
- `testPoint` (Rhino.Geometry.Point3d) — Point to compare against.

**Returns:** `Point3d` — A point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_ClosestPointInList.htm)

#### `public bool Contains(T item)`

Determines whether an element is in the List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Boolean` — true if item is found in the List; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Contains.htm)

#### `public RhinoList<TOutput> ConvertAll<TOutput>(Converter<T, TOutput> converter)`

Aggregates all results of a conversion function over this table into a new list.

**Parameters:**
- `converter` (System.Converter<T,TOutput>) — A conversion function that can transform from T to TOutput.

**Returns:** `RhinoList<TOutput>` — The new list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ConvertAll__1.htm)

#### `public void CopyTo(int index, T[] array, int arrayIndex, int count)`

Copies a range of elements from the List to a compatible one-dimensional array, starting at the specified index of the target array.

**Parameters:**
- `index` (System.Int32) — The zero-based index in the source List at which copying begins.
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.
- `count` (System.Int32) — The number of elements to copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo.htm)

#### `public void CopyTo(T[] array)`

Copies the entire List to a compatible one-dimensional array, starting at the beginning of the target array.

**Parameters:**
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo_1.htm)

#### `public void CopyTo(T[] array, int arrayIndex)`

Copies the entire List to a compatible one-dimensional array, starting at the specified index of the target array.

**Parameters:**
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo_2.htm)

#### `public Point3dList Duplicate()`

Returns a deep copy of this point list instance.

**Returns:** `Point3dList` — The duplicated list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_Duplicate.htm)

#### `public override bool Equals(Object obj)`

Overrides the default object equality to compare lists by value.

**Parameters:**
- `obj` (System.Object)

**Returns:** `Boolean` — True is objects are exactly equal in value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_Equals_1.htm)

#### `public bool Equals(Point3dList other)`

Determines if the point lists are exactly equal.

**Parameters:**
- `other` (Rhino.Collections.Point3dList)

**Returns:** `Boolean` — True is objects are exactly equal in value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_Equals.htm)

#### `public bool Exists(Predicate<T> match)`

Determines whether the List contains elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to search for.

**Returns:** `Boolean` — true if the List contains one or more elements that match the conditions defined by the specified predicate; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Exists.htm)

#### `public T Find(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the first occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `T` — The first element that matches the conditions defined by the specified predicate, if found; otherwise, the default value for type T.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Find.htm)

#### `public RhinoList<T> FindAll(Predicate<T> match)`

Retrieves all the elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to search for.

**Returns:** `RhinoList<T>` — A ON_List(T) containing all the elements that match the conditions defined by the specified predicate, if found; otherwise, an empty ON_List(T).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindAll.htm)

#### `public int FindIndex(int startIndex, int count, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the range of elements in the List that extends from the specified index to the last element.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the search.
- `count` (System.Int32) — The number of elements in the section to search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex.htm)

#### `public int FindIndex(int startIndex, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex_1.htm)

#### `public int FindIndex(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex_2.htm)

#### `public T FindLast(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the last occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `T` — The last element that matches the conditions defined by the specified predicate, if found; otherwise, the default value for type T.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLast.htm)

#### `public int FindLastIndex(int startIndex, int count, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the backward search.
- `count` (System.Int32) — The number of elements in the section to search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex.htm)

#### `public int FindLastIndex(int startIndex, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the backward search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex_1.htm)

#### `public int FindLastIndex(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex_2.htm)

#### `public void ForEach(Action<T> action)`

Performs the specified action on each element of the List.

**Parameters:**
- `action` (System.Action<T>) — The Action(T) delegate to perform on each element of the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ForEach.htm)

#### `public IEnumerator<T> GetEnumerator()`

Constructs an enumerator that is capable of iterating over all items in this list.

**Returns:** `IEnumerator<T>` — The new enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_GetEnumerator.htm)

#### `public override int GetHashCode()`

Creates a hash code for this object.

**Returns:** `Int32` — An int that serves as an hash code. This is currently a XOR of all doubles, XORed in line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_GetHashCode.htm)

#### `public RhinoList<T> GetRange(int index, int count)`

Constructs a shallow copy of a range of elements in the source List.

**Parameters:**
- `index` (System.Int32) — The zero-based List index at which the range starts.
- `count` (System.Int32) — The number of elements in the range.

**Returns:** `RhinoList<T>` — A shallow copy of a range of elements in the source List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_GetRange.htm)

#### `public int IndexOf(T item)`

Searches for the specified object and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf.htm)

#### `public int IndexOf(T item, int index)`

Searches for the specified object and returns the zero-based index of the first occurrence within the range of elements in the List that extends from the specified index to the last element.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the search.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf_1.htm)

#### `public int IndexOf(T item, int index, int count)`

Searches for the specified object and returns the zero-based index of the first occurrence within the range of elements in the List that starts at the specified index and contains the specified number of elements.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the search.
- `count` (System.Int32) — The number of elements in the section to search.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf_2.htm)

#### `public void Insert(int index, T item)`

Inserts an element into the List at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero-based index at which item should be inserted.
- `item` (T) — The object to insert. The value can be a null reference (Nothing in Visual Basic) for reference types.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Insert.htm)

#### `public void InsertRange(int index, IEnumerable<T> collection)`

Inserts the elements of a collection into the List at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero-based index at which the new elements should be inserted.
- `collection` (System.Collections.Generic.IEnumerable<T>) — The collection whose elements should be inserted into the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic), if type T is a reference type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_InsertRange.htm)

#### `public int LastIndexOf(T item)`

Searches for the specified object and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf.htm)

#### `public int LastIndexOf(T item, int index)`

Searches for the specified object and returns the zero-based index of the last occurrence within the range of elements in the List that extends from the first element to the specified index.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the backward search.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf_1.htm)

#### `public int LastIndexOf(T item, int index, int count)`

Searches for the specified object and returns the zero-based index of the last occurrence within the range of elements in the List that contains the specified number of elements and ends at the specified index.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the backward search.
- `count` (System.Int32) — The number of elements in the section to search.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf_2.htm)

#### `public int RemapIndex(int index)`

Remap an index in the infinite range onto the List index range.

**Parameters:**
- `index` (System.Int32) — Index to remap.

**Returns:** `Int32` — Remapped index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemapIndex.htm)

#### `public bool Remove(T item)`

Removes the first occurrence of a specific object from the List.

**Parameters:**
- `item` (T) — The object to remove from the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Boolean` — true if item is successfully removed; otherwise, false. This method also returns false if item was not found in the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Remove.htm)

#### `public int RemoveAll(Predicate<T> match)`

Removes the all the elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to remove.

**Returns:** `Int32` — The number of elements removed from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveAll.htm)

#### `public void RemoveAt(int index)`

Removes the element at the specified index of the List.

**Parameters:**
- `index` (System.Int32) — The zero-based index of the element to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveAt.htm)

#### `public int RemoveNulls()`

Removes all elements from the List that are null references (Nothing in Visual Basic). This function will not do anything if T is not a Reference type.

**Returns:** `Int32` — The number of nulls removed from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveNulls.htm)

#### `public void RemoveRange(int index, int count)`

Removes a range of elements from the List.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range of elements to remove.
- `count` (System.Int32) — The number of elements to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveRange.htm)

#### `public void Reverse()`

Reverses the order of the elements in the entire List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Reverse.htm)

#### `public void Reverse(int index, int count)`

Reverses the order of the elements in the specified range.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to reverse.
- `count` (System.Int32) — The number of elements in the range to reverse.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Reverse_1.htm)

#### `public void SetAllX(double xValue)`

Set all the X values for the points to a single value

**Parameters:**
- `xValue` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_SetAllX.htm)

#### `public void SetAllY(double yValue)`

Set all the Y values for the points to a single value

**Parameters:**
- `yValue` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_SetAllY.htm)

#### `public void SetAllZ(double zValue)`

Set all the Z values for the points to a single value

**Parameters:**
- `zValue` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_SetAllZ.htm)

#### `public void Sort()`

Sorts the elements in the entire List using the default comparer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort.htm)

#### `public void Sort(Comparison<T> comparison)`

Sorts the elements in the entire list using the specified comparer.

**Parameters:**
- `comparison` (System.Comparison<T>) — The System.Comparison(T) to use when comparing elements.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_2.htm)

#### `public void Sort(double[] keys)`

Sort this list based on a list of numeric keys of equal length. The keys array will not be altered.

**Remarks:** This function does not exist on the DotNET List class. David thought it was a good idea.

**Parameters:**
- `keys` (System.Double[]) — Numeric keys to sort with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_3.htm)

#### `public void Sort(IComparer<T> comparer)`

Sorts the elements in the entire list using the specified System.Comparison(T)

**Parameters:**
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements, or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T).Default.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_1.htm)

#### `public void Sort(int index, int count, IComparer<T> comparer)`

Sorts the elements in a range of elements in list using the specified comparer.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to sort.
- `count` (System.Int32) — The length of the range to sort.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements, or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T).Default.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_4.htm)

#### `public void Sort(int[] keys)`

Sort this list based on a list of numeric keys of equal length. The keys array will not be altered.

**Remarks:** This function does not exist on the DotNET List class. David thought it was a good idea.

**Parameters:**
- `keys` (System.Int32[]) — Numeric keys to sort with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_5.htm)

#### `public T[] ToArray()`

Constructs an array that contains all items in this list.

**Returns:** `T[]` — An array containing all items in this list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ToArray.htm)

#### `public void Transform(Transform xform)`

Applies a transform to all the points in the list.

**Parameters:**
- `xform` (Rhino.Geometry.Transform) — Transform to apply.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_Point3dList_Transform.htm)

#### `public void TrimExcess()`

Sets the capacity to the actual number of elements in the List, if that number is less than a threshold value.

**Remarks:** This function differs from the DotNET implementation of List<T> since that one only trims the excess if the excess exceeds 10% of the list length.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_TrimExcess.htm)

#### `public bool TrueForAll(Predicate<T> match)`

Determines whether every element in the List matches the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions to check against the elements.

**Returns:** `Boolean` — true if every element in the List matches the conditions defined by the specified predicate; otherwise, false. If the list has no elements, the return value is true.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_TrueForAll.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — Even though this is a property, it is not a "fast" calculation. Every point is evaluated in order to get the bounding box of the list.
- `Capacity` (Int32, get/set) — Gets or sets the total number of elements the internal data structure can hold without resizing.
- `Count` (Int32, get) — Gets the number of elements actually contained in the List.
- `First` (T, get/set) — Gets or sets the first item in the list. This is synonymous to calling List[0].
- `Item` (T, get/set) — Gets or sets the element at the specified index.
- `Last` (T, get/set) — Gets or sets the last item in the list. This is synonymous to calling List[Count-1].
- `NullCount` (Int32, get) — Gets the number of null references (Nothing in Visual Basic) in this list. If T is a ValueType, this property always return zero.
- `X` (Point3dList.XAccess, get) — Returns an indexer with all X coordinates in this list.
- `Y` (Point3dList.YAccess, get) — Returns an indexer with all Y coordinates in this list.
- `Z` (Point3dList.ZAccess, get) — Returns an indexer with all Z coordinates in this list.

## Point3dList.XAccess (class)

Utility class for easy-access of x-components of points inside an ON_3dPointList.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_Point3dList_XAccess.htm)

### Properties
- `Item` (Double, get/set) — Gets or sets the x-coordinate of the specified point.

## Point3dList.YAccess (class)

Utility class for easy-access of x-components of points inside an ON_3dPointList.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_Point3dList_YAccess.htm)

### Properties
- `Item` (Double, get/set) — Gets or sets the y-coordinate of the specified point.

## Point3dList.ZAccess (class)

Utility class for easy-access of z-components of points inside an ON_3dPointList.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_Point3dList_ZAccess.htm)

### Properties
- `Item` (Double, get/set) — Gets or sets the z-coordinate of the specified point.

## RhinoList (class)

Provides helper methods to work with RhinoList<T> and other collections.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_RhinoList.htm)

### Methods
#### `public static IEnumerable<int[]> Point2dKNeighbors(IEnumerable<Point2d> hayPoints, IEnumerable<Point2d> needlePoints, int amount)`

Finds a certain amour of points in a list of single-precision 2D points that are the k-closest to a test point. This method searches needlePoints by computing all distances from each point cloud point and keeping a "short list".

**Parameters:**
- `hayPoints` (System.Collections.Generic.IEnumerable<Point2d>) — A point cloud to be searched.
- `needlePoints` (System.Collections.Generic.IEnumerable<Point2d>) — Points to search for.
- `amount` (System.Int32) — The required amount of closest neighbors to find.

**Returns:** `IEnumerable<Int32[]>` — An enumerable of arrays of indices; each array contains the indices for each of the needlePts.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_Point2dKNeighbors.htm)

#### `public static IEnumerable<int[]> Point2fKNeighbors(IEnumerable<Point2f> hayPoints, IEnumerable<Point2f> needlePoints, int amount)`

Finds a certain amour of points in a list of single-precision 2D points that are the k-closest to a test point. This method searches needlePoints by computing all distances from each point cloud point and keeping a "short list".

**Parameters:**
- `hayPoints` (System.Collections.Generic.IEnumerable<Point2f>) — A point cloud to be searched.
- `needlePoints` (System.Collections.Generic.IEnumerable<Point2f>) — Points to search for.
- `amount` (System.Int32) — The required amount of closest neighbors to find.

**Returns:** `IEnumerable<Int32[]>` — An enumerable of arrays of indices; each array contains the indices for each of the needlePts.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_Point2fKNeighbors.htm)

#### `public static IEnumerable<int[]> Point3dKNeighbors(IEnumerable<Point3d> hayPoints, IEnumerable<Point3d> needlePoints, int amount)`

Finds a certain amour of points in a list of 3D points that are the k-closest to a test point. This method searches needlePoints by computing all distances from each point cloud point and keeping a "short list". See RTree KNeighbors for alternatives.

**Parameters:**
- `hayPoints` (System.Collections.Generic.IEnumerable<Point3d>) — A point cloud to be searched.
- `needlePoints` (System.Collections.Generic.IEnumerable<Point3d>) — Points to search for.
- `amount` (System.Int32) — The required amount of closest neighbors to find.

**Returns:** `IEnumerable<Int32[]>` — An enumerable of arrays of indices; each array contains the indices for each of the needlePts.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_Point3dKNeighbors.htm)

#### `public static IEnumerable<int[]> Point3fKNeighbors(IEnumerable<Point3f> hayPoints, IEnumerable<Point3f> needlePoints, int amount)`

Finds a certain amour of points in a list of single-precision 3D points that are the k-closest to a test point. This method searches needlePoints by computing all distances from each point cloud point and keeping a "short list".

**Parameters:**
- `hayPoints` (System.Collections.Generic.IEnumerable<Point3f>) — A point cloud to be searched.
- `needlePoints` (System.Collections.Generic.IEnumerable<Point3f>) — Points to search for.
- `amount` (System.Int32) — The required amount of closest neighbors to find.

**Returns:** `IEnumerable<Int32[]>` — An enumerable of arrays of indices; each array contains the indices for each of the needlePts.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_Point3fKNeighbors.htm)

#### `public static IEnumerable<int[]> PointCloudKNeighbors(PointCloud pointcloud, IEnumerable<Point3d> needlePoints, int amount)`

Finds a certain amount of points in a list of 3D points that are the k-closest to a test point. This method searches needlePoints by computing all distances from each point cloud point and keeping a short list.

**Parameters:**
- `pointcloud` (Rhino.Geometry.PointCloud) — A point cloud to be searched.
- `needlePoints` (System.Collections.Generic.IEnumerable<Point3d>) — Points to search for.
- `amount` (System.Int32) — The required amount of closest neighbors to find.

**Returns:** `IEnumerable<Int32[]>` — An enumerable of arrays of indices; each array contains the indices for each of the needlePts.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_PointCloudKNeighbors.htm)

## RhinoList<T> (class)

Represents a list of generic data. This class is similar to System.Collections.Generic.List(T) but exposes a few more methods.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_RhinoList_1.htm)

### Constructors
- `public RhinoList()` — Initializes a new, empty list.
- `public RhinoList(IEnumerable<T> collection)` — Initializes this list as a shallow duplicate of another list, array or any other enumerable set of T.
- `public RhinoList(int initialCapacity)` — Initializes an empty list with a certain capacity.
- `public RhinoList(RhinoList<T> list)` — Initializes an new list by shallow duplicating another list.
- `public RhinoList(int amount, T defaultValue)` — Initializes a new list with a specified amount of values.

### Methods
#### `public void Add(T item)`

Adds an object to the end of the List.

**Parameters:**
- `item` (T) — Item to append.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Add.htm)

#### `public void AddRange(IEnumerable collection)`

Adds the elements of the specified collection to the end of the List.

**Parameters:**
- `collection` (System.Collections.IEnumerable) — The collection whose elements should be added to the end of the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic). Objects in collection which cannot be represented as T will throw exceptions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AddRange_1.htm)

#### `public void AddRange(IEnumerable<T> collection)`

Adds the elements of the specified collection to the end of the List.

**Parameters:**
- `collection` (System.Collections.Generic.IEnumerable<T>) — The collection whose elements should be added to the end of the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic), if type T is a reference type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AddRange.htm)

#### `public ReadOnlyCollection<T> AsReadOnly()`

Constructs a read-only wrapper of this class.

**Returns:** `ReadOnlyCollection<T>` — A wrapper.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_AsReadOnly.htm)

#### `public int BinarySearch(int index, int count, T item, IComparer<T> comparer)`

Searches the entire sorted List for an element using the specified comparer and returns the zero-based index of the element.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to search.
- `count` (System.Int32) — The length of the range to search.
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements. Or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T)::Default.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch.htm)

#### `public int BinarySearch(T item)`

Searches the entire sorted List for an element using the default comparer and returns the zero-based index of the element.

**Parameters:**
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch_1.htm)

#### `public int BinarySearch(T item, IComparer<T> comparer)`

Searches the entire sorted List for an element using the specified comparer and returns the zero-based index of the element.

**Parameters:**
- `item` (T) — The object to locate. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements. Or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T)::Default.

**Returns:** `Int32` — The zero-based index of item in the sorted List, if item is found; otherwise, a negative number that is the bitwise complement of the index of the next element that is larger than item or, if there is no larger element, the bitwise complement of Count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_BinarySearch_2.htm)

#### `public void Clear()`

Removes all elements from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Clear.htm)

#### `public bool Contains(T item)`

Determines whether an element is in the List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Boolean` — true if item is found in the List; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Contains.htm)

#### `public RhinoList<TOutput> ConvertAll<TOutput>(Converter<T, TOutput> converter)`

Aggregates all results of a conversion function over this table into a new list.

**Parameters:**
- `converter` (System.Converter<T,TOutput>) — A conversion function that can transform from T to TOutput.

**Returns:** `RhinoList<TOutput>` — The new list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ConvertAll__1.htm)

#### `public void CopyTo(int index, T[] array, int arrayIndex, int count)`

Copies a range of elements from the List to a compatible one-dimensional array, starting at the specified index of the target array.

**Parameters:**
- `index` (System.Int32) — The zero-based index in the source List at which copying begins.
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.
- `count` (System.Int32) — The number of elements to copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo.htm)

#### `public void CopyTo(T[] array)`

Copies the entire List to a compatible one-dimensional array, starting at the beginning of the target array.

**Parameters:**
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo_1.htm)

#### `public void CopyTo(T[] array, int arrayIndex)`

Copies the entire List to a compatible one-dimensional array, starting at the specified index of the target array.

**Parameters:**
- `array` (T[]) — The one-dimensional Array that is the destination of the elements copied from List. The Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_CopyTo_2.htm)

#### `public RhinoList<T> Duplicate()`

Returns a shallow copy of this instance. If the generic type is comprised of only value types (struct, enum, ptr), then the result will be a deep copy.

**Returns:** `RhinoList<T>` — The duplicated list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Duplicate.htm)

#### `public bool Exists(Predicate<T> match)`

Determines whether the List contains elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to search for.

**Returns:** `Boolean` — true if the List contains one or more elements that match the conditions defined by the specified predicate; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Exists.htm)

#### `public T Find(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the first occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `T` — The first element that matches the conditions defined by the specified predicate, if found; otherwise, the default value for type T.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Find.htm)

#### `public RhinoList<T> FindAll(Predicate<T> match)`

Retrieves all the elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to search for.

**Returns:** `RhinoList<T>` — A ON_List(T) containing all the elements that match the conditions defined by the specified predicate, if found; otherwise, an empty ON_List(T).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindAll.htm)

#### `public int FindIndex(int startIndex, int count, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the range of elements in the List that extends from the specified index to the last element.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the search.
- `count` (System.Int32) — The number of elements in the section to search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex.htm)

#### `public int FindIndex(int startIndex, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex_1.htm)

#### `public int FindIndex(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the first occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindIndex_2.htm)

#### `public T FindLast(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the last occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `T` — The last element that matches the conditions defined by the specified predicate, if found; otherwise, the default value for type T.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLast.htm)

#### `public int FindLastIndex(int startIndex, int count, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the backward search.
- `count` (System.Int32) — The number of elements in the section to search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex.htm)

#### `public int FindLastIndex(int startIndex, Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `startIndex` (System.Int32) — The zero-based starting index of the backward search.
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex_1.htm)

#### `public int FindLastIndex(Predicate<T> match)`

Searches for an element that matches the conditions defined by the specified predicate, and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the element to search for.

**Returns:** `Int32` — The zero-based index of the last occurrence of an element that matches the conditions defined by match, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_FindLastIndex_2.htm)

#### `public void ForEach(Action<T> action)`

Performs the specified action on each element of the List.

**Parameters:**
- `action` (System.Action<T>) — The Action(T) delegate to perform on each element of the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ForEach.htm)

#### `public IEnumerator<T> GetEnumerator()`

Constructs an enumerator that is capable of iterating over all items in this list.

**Returns:** `IEnumerator<T>` — The new enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_GetEnumerator.htm)

#### `public RhinoList<T> GetRange(int index, int count)`

Constructs a shallow copy of a range of elements in the source List.

**Parameters:**
- `index` (System.Int32) — The zero-based List index at which the range starts.
- `count` (System.Int32) — The number of elements in the range.

**Returns:** `RhinoList<T>` — A shallow copy of a range of elements in the source List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_GetRange.htm)

#### `public int IndexOf(T item)`

Searches for the specified object and returns the zero-based index of the first occurrence within the entire List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf.htm)

#### `public int IndexOf(T item, int index)`

Searches for the specified object and returns the zero-based index of the first occurrence within the range of elements in the List that extends from the specified index to the last element.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the search.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf_1.htm)

#### `public int IndexOf(T item, int index, int count)`

Searches for the specified object and returns the zero-based index of the first occurrence within the range of elements in the List that starts at the specified index and contains the specified number of elements.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the search.
- `count` (System.Int32) — The number of elements in the section to search.

**Returns:** `Int32` — The zero-based index of the first occurrence of item within the entire List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_IndexOf_2.htm)

#### `public void Insert(int index, T item)`

Inserts an element into the List at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero-based index at which item should be inserted.
- `item` (T) — The object to insert. The value can be a null reference (Nothing in Visual Basic) for reference types.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Insert.htm)

#### `public void InsertRange(int index, IEnumerable<T> collection)`

Inserts the elements of a collection into the List at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero-based index at which the new elements should be inserted.
- `collection` (System.Collections.Generic.IEnumerable<T>) — The collection whose elements should be inserted into the List. The collection itself cannot be a null reference (Nothing in Visual Basic), but it can contain elements that are a null reference (Nothing in Visual Basic), if type T is a reference type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_InsertRange.htm)

#### `public int LastIndexOf(T item)`

Searches for the specified object and returns the zero-based index of the last occurrence within the entire List.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf.htm)

#### `public int LastIndexOf(T item, int index)`

Searches for the specified object and returns the zero-based index of the last occurrence within the range of elements in the List that extends from the first element to the specified index.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the backward search.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf_1.htm)

#### `public int LastIndexOf(T item, int index, int count)`

Searches for the specified object and returns the zero-based index of the last occurrence within the range of elements in the List that contains the specified number of elements and ends at the specified index.

**Parameters:**
- `item` (T) — The object to locate in the List. The value can be a null reference (Nothing in Visual Basic) for reference types.
- `index` (System.Int32) — The zero-based starting index of the backward search.
- `count` (System.Int32) — The number of elements in the section to search.

**Returns:** `Int32` — The zero-based index of the last occurrence of item within the entire the List, if found; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_LastIndexOf_2.htm)

#### `public int RemapIndex(int index)`

Remap an index in the infinite range onto the List index range.

**Parameters:**
- `index` (System.Int32) — Index to remap.

**Returns:** `Int32` — Remapped index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemapIndex.htm)

#### `public bool Remove(T item)`

Removes the first occurrence of a specific object from the List.

**Parameters:**
- `item` (T) — The object to remove from the List. The value can be a null reference (Nothing in Visual Basic) for reference types.

**Returns:** `Boolean` — true if item is successfully removed; otherwise, false. This method also returns false if item was not found in the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Remove.htm)

#### `public int RemoveAll(Predicate<T> match)`

Removes the all the elements that match the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions of the elements to remove.

**Returns:** `Int32` — The number of elements removed from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveAll.htm)

#### `public void RemoveAt(int index)`

Removes the element at the specified index of the List.

**Parameters:**
- `index` (System.Int32) — The zero-based index of the element to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveAt.htm)

#### `public int RemoveNulls()`

Removes all elements from the List that are null references (Nothing in Visual Basic). This function will not do anything if T is not a Reference type.

**Returns:** `Int32` — The number of nulls removed from the List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveNulls.htm)

#### `public void RemoveRange(int index, int count)`

Removes a range of elements from the List.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range of elements to remove.
- `count` (System.Int32) — The number of elements to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_RemoveRange.htm)

#### `public void Reverse()`

Reverses the order of the elements in the entire List.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Reverse.htm)

#### `public void Reverse(int index, int count)`

Reverses the order of the elements in the specified range.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to reverse.
- `count` (System.Int32) — The number of elements in the range to reverse.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Reverse_1.htm)

#### `public void Sort()`

Sorts the elements in the entire List using the default comparer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort.htm)

#### `public void Sort(Comparison<T> comparison)`

Sorts the elements in the entire list using the specified comparer.

**Parameters:**
- `comparison` (System.Comparison<T>) — The System.Comparison(T) to use when comparing elements.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_2.htm)

#### `public void Sort(double[] keys)`

Sort this list based on a list of numeric keys of equal length. The keys array will not be altered.

**Remarks:** This function does not exist on the DotNET List class. David thought it was a good idea.

**Parameters:**
- `keys` (System.Double[]) — Numeric keys to sort with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_3.htm)

#### `public void Sort(IComparer<T> comparer)`

Sorts the elements in the entire list using the specified System.Comparison(T)

**Parameters:**
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements, or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T).Default.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_1.htm)

#### `public void Sort(int index, int count, IComparer<T> comparer)`

Sorts the elements in a range of elements in list using the specified comparer.

**Parameters:**
- `index` (System.Int32) — The zero-based starting index of the range to sort.
- `count` (System.Int32) — The length of the range to sort.
- `comparer` (System.Collections.Generic.IComparer<T>) — The IComparer(T) implementation to use when comparing elements, or a null reference (Nothing in Visual Basic) to use the default comparer Comparer(T).Default.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_4.htm)

#### `public void Sort(int[] keys)`

Sort this list based on a list of numeric keys of equal length. The keys array will not be altered.

**Remarks:** This function does not exist on the DotNET List class. David thought it was a good idea.

**Parameters:**
- `keys` (System.Int32[]) — Numeric keys to sort with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_Sort_5.htm)

#### `public T[] ToArray()`

Constructs an array that contains all items in this list.

**Returns:** `T[]` — An array containing all items in this list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_ToArray.htm)

#### `public void TrimExcess()`

Sets the capacity to the actual number of elements in the List, if that number is less than a threshold value.

**Remarks:** This function differs from the DotNET implementation of List<T> since that one only trims the excess if the excess exceeds 10% of the list length.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_TrimExcess.htm)

#### `public bool TrueForAll(Predicate<T> match)`

Determines whether every element in the List matches the conditions defined by the specified predicate.

**Parameters:**
- `match` (System.Predicate<T>) — The Predicate(T) delegate that defines the conditions to check against the elements.

**Returns:** `Boolean` — true if every element in the List matches the conditions defined by the specified predicate; otherwise, false. If the list has no elements, the return value is true.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_RhinoList_1_TrueForAll.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of elements the internal data structure can hold without resizing.
- `Count` (Int32, get) — Gets the number of elements actually contained in the List.
- `First` (T, get/set) — Gets or sets the first item in the list. This is synonymous to calling List[0].
- `Item` (T, get/set) — Gets or sets the element at the specified index.
- `Last` (T, get/set) — Gets or sets the last item in the list. This is synonymous to calling List[Count-1].
- `NullCount` (Int32, get) — Gets the number of null references (Nothing in Visual Basic) in this list. If T is a ValueType, this property always return zero.

## TransformObjectList (class)

A collection of Rhino object, grip objects, and the Rhino objects that owns the grips. Used by the TransformCommand and GetTransform classes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Collections_TransformObjectList.htm)

### Constructors
- `public TransformObjectList()` — Initializes a new instance of the TransformObjectList class

### Methods
#### `public void Add(ObjRef objref)`

Add an ObjRef to this list. Use this to add polyedges so the references are properly counted

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — [Missing <param name="objref"/> documentation for "M:Rhino.Collections.TransformObjectList.Add(Rhino.DocObjects.ObjRef)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_Add.htm)

#### `public void Add(RhinoObject rhinoObject)`

Add a RhinoObject to this list

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — [Missing <param name="rhinoObject"/> documentation for "M:Rhino.Collections.TransformObjectList.Add(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_Add_1.htm)

#### `public int AddObjects(GetObject go, bool allowGrips)`

Add objects to list with a GetObject

**Parameters:**
- `go` (Rhino.Input.Custom.GetObject) — Setup the GetObject, i.e. prompt, geometry filter, allow pre/post select before passing it as an argument.
- `allowGrips` (System.Boolean) — Specifically allow grips to be selected. if true, grips must also be included in geometry filter of the GetObject in order to be selected.

**Returns:** `Int32` — Number of objects selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_AddObjects.htm)

#### `public void Clear()`

Remove all elements from this list

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_Clear.htm)

#### `public void Dispose()`

Releases all resources used by the TransformObjectList

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the TransformObjectList and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_Finalize.htm)

#### `public BoundingBox GetBoundingBox(bool regularObjects, bool grips)`

Gets the bounding box of all of the objects that this list contains.

**Parameters:**
- `regularObjects` (System.Boolean) — true if any object except grips should be included; otherwise false.
- `grips` (System.Boolean) — true if grips should be included; otherwise false.

**Returns:** `BoundingBox` — Unset BoundingBox if this list is empty.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_GetBoundingBox.htm)

#### `public GripObject[] GripArray()`

Gets access to the internal GripObject array of the TransformObjectList object.

**Returns:** `GripObject[]` — An array of grip objects, or an empty array if there were no grip objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_GripArray.htm)

#### `public RhinoObject[] GripOwnerArray()`

Gets access to the internal GripOwner array of the TransformObjectList object.

**Returns:** `RhinoObject[]` — A n array of Rhino objects that are the owners of the grip objects the collection, or an empty array if there were no Rhino objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_GripOwnerArray.htm)

#### `public RhinoObject[] ObjectArray()`

Gets access to the internal RhinoObject array of the TransformObjectList object.

**Returns:** `RhinoObject[]` — An array of Rhino objects, or an empty array if there were no Rhino objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_ObjectArray.htm)

#### `public bool UpdateDisplayFeedbackTransform(Transform xform)`



**Parameters:**
- `xform` (Rhino.Geometry.Transform) — [Missing <param name="xform"/> documentation for "M:Rhino.Collections.TransformObjectList.UpdateDisplayFeedbackTransform(Rhino.Geometry.Transform)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Collections.TransformObjectList.UpdateDisplayFeedbackTransform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Collections_TransformObjectList_UpdateDisplayFeedbackTransform.htm)

### Properties
- `Count` (Int32, get) — Number of elements in this list
- `DisplayFeedbackEnabled` (Boolean, get/set) — 
- `GripCount` (Int32, get) — Number of elements in grip list
- `GripOwnerCount` (Int32, get) — Number of elements in grip owner list

