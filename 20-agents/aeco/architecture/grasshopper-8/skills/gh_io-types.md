---
name: grasshopper-gh_io-types
description: This skill encodes the grasshopper 8.0 surface of the GH_IO.Types namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_Item, GH_BoundingBox, GH_Interval2D, GH_Interval1D, GH_Line, GH_Point2D, GH_Point3D, GH_Plane, and 3 more types.
---

# GH_IO.Types

Auto-generated from vendor docs for grasshopper 8.0. 11 types in this namespace.

## GH_BoundingBox (struct)

Represents a 3D bounding box, denoted by two points.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_BoundingBox.htm)

### Constructors
- `public GH_BoundingBox(GH_Point3D nMin, GH_Point3D nMax)` — Constructor.
- `public GH_BoundingBox(double Minx, double Miny, double Minz, double Maxx, double Maxy, double Maxz)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the box structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_BoundingBox_ToString.htm)

### Properties
- `Max` (GH_Point3D, get) — Maximum corner of box.
- `Min` (GH_Point3D, get) — Minimum corner of box.

## GH_Interval1D (struct)

Represents two double precision floating point values.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Interval1D.htm)

### Constructors
- `public GH_Interval1D(double na, double nb)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the Interval structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Interval1D_ToString.htm)

### Properties
- `a` (Double, get) — Start of interval.
- `b` (Double, get) — End of interval.

## GH_Interval2D (struct)

Represents two double precision domains.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Interval2D.htm)

### Constructors
- `public GH_Interval2D(GH_Interval1D nu, GH_Interval1D nv)` — Constructor.
- `public GH_Interval2D(double nu0, double nu1, double nv0, double nv1)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the two-dimensional Interval structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Interval2D_ToString.htm)

### Properties
- `u` (GH_Interval1D, get) — One dimensional component in U direction.
- `v` (GH_Interval1D, get) — One dimensional component in V direction.

## GH_Item (class)

Represents a single data item in a chunk.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Item.htm)

### Constructors
- `public GH_Item(string item_name, GH_BoundingBox item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Interval1D item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Interval2D item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Line item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Plane item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Point2D item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Point3D item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Point4D item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, GH_Version item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, bool item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, byte item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, byte[] item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, DateTime item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, decimal item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, double item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, double[] item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, Bitmap item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, Color item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, Point item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, PointF item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, Rectangle item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, RectangleF item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, Size item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, SizeF item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, Guid item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, int item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, long item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, float item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, string item_data)` — Construct a new instance of GH_Item with the specified name and value.
- `public GH_Item(string item_name, int item_index, GH_BoundingBox item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Interval1D item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Interval2D item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Line item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Plane item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Point2D item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Point3D item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Point4D item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, GH_Version item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, bool item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, byte item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, byte[] item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, DateTime item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, decimal item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, double item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, double[] item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, Bitmap item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, Color item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, Point item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, PointF item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, Rectangle item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, RectangleF item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, Size item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, SizeF item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, Guid item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, int item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, long item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, float item_data)` — Construct a new instance of GH_Item with the specified name, index and value.
- `public GH_Item(string item_name, int item_index, string item_data)` — Construct a new instance of GH_Item with the specified name, index and value.

### Methods
#### `public static GH_Item CreateFrom(BinaryReader reader)`

Creates a new instance of GH_Item and sets the fields from a reader object.

**Parameters:**
- `reader` (System.IO.BinaryReader) — Reader object that defines the field data.

**Returns:** `GH_Item` — The constructed and read item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Item_CreateFrom.htm)

#### `public static GH_Item CreateFrom(XmlNode node)`

Creates a new instance of GH_Item and sets the fields from an Xml node object.

**Parameters:**
- `node` (System.Xml.XmlNode) — Xml node object that defines the field data.

**Returns:** `GH_Item` — The constructed and read item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Item_CreateFrom_1.htm)

#### `public void Read(BinaryReader reader)`

Deserialize this item from a binary stream.

**Parameters:**
- `reader` (System.IO.BinaryReader) — Reader to deserialize with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Item_Read.htm)

#### `public void Read(XmlNode node)`

Deserialize this item from an Xml node.

**Parameters:**
- `node` (System.Xml.XmlNode) — Xml node to serialize from.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Item_Read_1.htm)

#### `public override string ToString()`

Converts the struct into a human readable format.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Item_ToString.htm)

#### `public void Write(BinaryWriter writer)`

Serialize this item into a binary stream.

**Parameters:**
- `writer` (System.IO.BinaryWriter) — Writer to serialize with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Item_Write.htm)

#### `public void Write(XmlWriter writer)`

Serialize this item into an Xml stream.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — Writer to serialize with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Item_Write_1.htm)

### Properties
- `_bool` (Boolean, get) — Returns the internal data of this item cast to a Boolean. If the data is not stored as a Boolean, a conversion exception might be thrown.
- `_boundingbox` (GH_BoundingBox, get) — Returns the internal data of this item cast to a BoundingBox. If the data is not stored as a BoundingBox, a conversion exception might be thrown.
- `_byte` (Byte, get) — Returns the internal data of this item cast to a Byte. If the data is not stored as a Byte, a conversion exception might be thrown.
- `_bytearray` (Byte[], get) — Returns the internal data of this item cast to a Byte array. If the data is not stored as a Byte array, a conversion exception might be thrown.
- `_date` (DateTime, get) — Returns the internal data of this item cast to a DateTime. If the data is not stored as a DateTime, a conversion exception might be thrown.
- `_decimal` (Decimal, get) — Returns the internal data of this item cast to a Decimal. If the data is not stored as a Decimal, a conversion exception might be thrown.
- `_double` (Double, get) — Returns the internal data of this item cast to a Double. If the data is not stored as a Double, a conversion exception might be thrown.
- `_doublearray` (Double[], get) — Returns the internal data of this item cast to a Byte array. If the data is not stored as a Byte array, a conversion exception might be thrown.
- `_drawing_bitmap` (Bitmap, get) — Returns the internal data of this item cast to a Bitmap. If the data is not stored as a Bitmap, a conversion exception might be thrown.
- `_drawing_color` (Color, get) — Returns the internal data of this item cast to a Color. If the data is not stored as a Color, a conversion exception might be thrown.
- `_drawing_point` (Point, get) — Returns the internal data of this item cast to a Point. If the data is not stored as a Point, a conversion exception might be thrown.
- `_drawing_pointf` (PointF, get) — Returns the internal data of this item cast to a PointF. If the data is not stored as a PointF, a conversion exception might be thrown.
- `_drawing_rectangle` (Rectangle, get) — Returns the internal data of this item cast to a Rectangle. If the data is not stored as a Rectangle, a conversion exception might be thrown.
- `_drawing_rectanglef` (RectangleF, get) — Returns the internal data of this item cast to a RectangleF. If the data is not stored as a RectangleF, a conversion exception might be thrown.
- `_drawing_size` (Size, get) — Returns the internal data of this item cast to a Size. If the data is not stored as a Size, a conversion exception might be thrown.
- `_drawing_sizef` (SizeF, get) — Returns the internal data of this item cast to a SizeF. If the data is not stored as a SizeF, a conversion exception might be thrown.
- `_guid` (Guid, get) — Returns the internal data of this item cast to a Guid. If the data is not stored as a Guid, a conversion exception might be thrown.
- `_int32` (Int32, get) — Returns the internal data of this item cast to an Int32. If the data is not stored as an Int32, a conversion exception might be thrown.
- `_int64` (Int64, get) — Returns the internal data of this item cast to an Int64. If the data is not stored as an Int64, a conversion exception might be thrown.
- `_interval1d` (GH_Interval1D, get) — Returns the internal data of this item cast to an Interval1D. If the data is not stored as an Interval1D, a conversion exception might be thrown.
- `_interval2d` (GH_Interval2D, get) — Returns the internal data of this item cast to an Interval2D. If the data is not stored as an Interval2D, a conversion exception might be thrown.
- `_line` (GH_Line, get) — Returns the internal data of this item cast to a Line. If the data is not stored as a Line, a conversion exception might be thrown.
- `_plane` (GH_Plane, get) — Returns the internal data of this item cast to a Plane. If the data is not stored as a Plane, a conversion exception might be thrown.
- `_point2d` (GH_Point2D, get) — Returns the internal data of this item cast to a Point2D. If the data is not stored as a Point2D, a conversion exception might be thrown.
- `_point3d` (GH_Point3D, get) — Returns the internal data of this item cast to a Point3D. If the data is not stored as a Point3D, a conversion exception might be thrown.
- `_point4d` (GH_Point4D, get) — Returns the internal data of this item cast to a Point4D. If the data is not stored as a Point4D, a conversion exception might be thrown.
- `_single` (Single, get) — Returns the internal data of this item cast to a Single. If the data is not stored as a Single, a conversion exception might be thrown.
- `_string` (String, get) — Returns the internal data of this item cast to a String. If the data is not stored as a String, a conversion exception might be thrown.
- `_version` (GH_Version, get) — Returns the internal data of this item cast to a Version. If the data is not stored as a Version, a conversion exception might be thrown.
- `HasIndex` (Boolean, get) — Gets the index existence implication. The item is considered to have an index qualifier if the index value is larger than or equal to zero.
- `HasName` (Boolean, get) — Gets the name validity of this item. The item is considered to have an invalid name if string.IsNullOrEmpty(name)
- `HasType` (Boolean, get) — Gets the type set validity of this item. The item is considered to have a type if type != GH_Types.unset
- `Index` (Int32, get/set) — Gets or sets the index of an item. Typically, indices are set at construction and do not change. If you change indices after construction, you could corrupt an archive.
- `InternalData` (Object, get) — Retrieves the internal data of this item. No type casting is performed.
- `Name` (String, get/set) — Gets or sets the name of this item. Typically, names are set at construction and do not change. If you change names after construction, you could corrupt an archive.
- `Type` (GH_Types, get) — Gets the type of this item. Type flags are set during construction and cannot be altered.

## GH_Line (struct)

Represents a 3D line segment, denoted by start and endpoints.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Line.htm)

### Constructors
- `public GH_Line(GH_Point3D nA, GH_Point3D nB)` — Constructor.
- `public GH_Line(double Ax, double Ay, double Az, double Bx, double By, double Bz)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the line structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Line_ToString.htm)

### Properties
- `A` (GH_Point3D, get) — Start of line.
- `B` (GH_Point3D, get) — End of line.

## GH_Plane (struct)

Represents a 3D plane system, defined by origin point and {X,Y} axis directions.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Plane.htm)

### Constructors
- `public GH_Plane(GH_Point3D nOrigin, GH_Point3D nXAxis, GH_Point3D nYAxis)` — Constructor.
- `public GH_Plane(double Ox, double Oy, double Oz, double Xx, double Xy, double Xz, double Yx, double Yy, double Yz)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the plane structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Plane_ToString.htm)

### Properties
- `Origin` (GH_Point3D, get) — Origin point of plane.
- `XAxis` (GH_Point3D, get) — X axis vector of plane.
- `YAxis` (GH_Point3D, get) — Y axis vector of plane.

## GH_Point2D (struct)

Represents a 2D point coordinate with double precision floating point components.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Point2D.htm)

### Constructors
- `public GH_Point2D(double nx, double ny)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the two-dimenionsional point structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Point2D_ToString.htm)

### Properties
- `x` (Double, get) — X coordinate of point.
- `y` (Double, get) — Y coordinate of point.

## GH_Point3D (struct)

Represents a 3D point coordinate with double precision floating point components.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Point3D.htm)

### Constructors
- `public GH_Point3D(double nx, double ny, double nz)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the three-dimenionsional point structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Point3D_ToString.htm)

### Properties
- `x` (Double, get) — X coordinate of point.
- `y` (Double, get) — Y coordinate of point.
- `z` (Double, get) — Z coordinate of point.

## GH_Point4D (struct)

Represents a 4D point coordinate with double precision floating point components.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Point4D.htm)

### Constructors
- `public GH_Point4D(double nx, double ny, double nz, double nw)` — Constructor.

### Methods
#### `public override string ToString()`

Converts this structure to a human-readable string.

**Returns:** `String` — A string representation of the four-dimenionsional point structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Point4D_ToString.htm)

### Properties
- `w` (Double, get) — W coordinate of point.
- `x` (Double, get) — X coordinate of point.
- `y` (Double, get) — Y coordinate of point.
- `z` (Double, get) — Z coordinate of point.

## GH_Types (enum)

Contains flags for all data types currently supported by GH_IO.dll

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Types.htm)

### Values
- `unset` = `0` — Represents an unset type. GH_Items with this type will throw exceptions during serialization.
- `gh_bool` = `1` — boolean
- `gh_byte` = `2` — byte
- `gh_int32` = `3` — 32 bit integer
- `gh_int64` = `4` — 64 bit integer
- `gh_single` = `5` — single precision floating point number
- `gh_double` = `6` — double precision floating point number
- `gh_decimal` = `7` — decimal number
- `gh_date` = `8` — date time structure
- `gh_guid` = `9` — 128 bit globally unique identifier
- `gh_string` = `10` — unicode string
- `gh_bytearray` = `20` — an array of bytes
- `gh_doublearray` = `21` — an array of doubles
- `gh_drawing_point` = `30` — gdi+ integer precision point
- `gh_drawing_pointf` = `31` — gdi+ single precision point
- `gh_drawing_size` = `32` — gdi+ integer precision size
- `gh_drawing_sizef` = `33` — gdi+ single precision size
- `gh_drawing_rectangle` = `34` — gdi+ integer precision rectangle
- `gh_drawing_rectanglef` = `35` — gdi+ single precision rectangle
- `gh_drawing_color` = `36` — gdi+ argb color
- `gh_drawing_bitmap` = `37` — gdi+ bitmap (png format bytestream)
- `gh_point2d` = `50` — double precision two-dimensional point
- `gh_point3d` = `51` — double precision three-dimensional point
- `gh_point4d` = `52` — double precision four-dimensional point
- `gh_interval1d` = `60` — double precision one-dimensional numeric interval
- `gh_interval2d` = `61` — double precision two-dimensional numeric interval
- `gh_line` = `70` — double precision three-dimensional line segment
- `gh_boundingbox` = `71` — double precision three-dimensional box volume
- `gh_plane` = `72` — double precision three-dimensional plane construct
- `gh_version` = `80` — version structure with major, minor and revision fields

## GH_Version (struct)

Basic version type. Contains Major, Minor and Revision fields.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_Types_GH_Version.htm)

### Constructors
- `public GH_Version(GH_Version other)` — Copy constructor. Duplicate an existing version structure.
- `public GH_Version(int v_major, int v_minor, int v_revision)` — Default constructor. Create a new version from specified fields.

### Methods
#### `public static bool operator ==(GH_Version A, GH_Version B)`

Compares two version structures for equality.

**Parameters:**
- `A` (GH_IO.Types.GH_Version) — Version to compare.
- `B` (GH_IO.Types.GH_Version) — Version to compare with.

**Returns:** `Boolean` — True if all version fields in A and B are equal.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_op_Equality.htm)

#### `public override bool Equals(Object obj)`

Performs value equality comparison.

**Parameters:**
- `obj` (System.Object) — Object to compare with. If obj is a null reference or not a GH_Version instance, false is returned.

**Returns:** `Boolean` — True if obj is a GH_Version instance which is equal to this one.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_Equals.htm)

#### `public override int GetHashCode()`

Returns the hash code for this instance.

**Returns:** `Int32` — A hash code for the current version object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_GetHashCode.htm)

#### `public static bool operator >(GH_Version A, GH_Version B)`

Compares versions for larger than relationships.

**Parameters:**
- `A` (GH_IO.Types.GH_Version) — Version to compare.
- `B` (GH_IO.Types.GH_Version) — Version to compare with.

**Returns:** `Boolean` — True if A is newer than B.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_op_GreaterThan.htm)

#### `public static bool operator >=(GH_Version A, GH_Version B)`

Compares versions for larger than or equal to relationships.

**Parameters:**
- `A` (GH_IO.Types.GH_Version) — Version to compare.
- `B` (GH_IO.Types.GH_Version) — Version to compare with.

**Returns:** `Boolean` — True if A is newer than or equal to B.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_op_GreaterThanOrEqual.htm)

#### `public static bool operator !=(GH_Version A, GH_Version B)`

Compares two version structures for inequality.

**Parameters:**
- `A` (GH_IO.Types.GH_Version) — Version to compare.
- `B` (GH_IO.Types.GH_Version) — Version to compare with.

**Returns:** `Boolean` — True if A is different from B.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_op_Inequality.htm)

#### `public static bool operator <(GH_Version A, GH_Version B)`

Compares versions for smaller than relationships.

**Parameters:**
- `A` (GH_IO.Types.GH_Version) — Version to compare.
- `B` (GH_IO.Types.GH_Version) — Version to compare with.

**Returns:** `Boolean` — True if A represents an older version than B.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_op_LessThan.htm)

#### `public static bool operator <=(GH_Version A, GH_Version B)`

Compares versions for smaller than or equal to relationships.

**Parameters:**
- `A` (GH_IO.Types.GH_Version) — Version to compare.
- `B` (GH_IO.Types.GH_Version) — Version to compare with.

**Returns:** `Boolean` — True if A is older than or equal to B.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_op_LessThanOrEqual.htm)

#### `public override string ToString()`

Default formatter for Version data: M.m.RRRR Revision section is padded with zeroes until it is at least 4 digits long.

**Returns:** `String` — A string represtation of the Version structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_Types_GH_Version_ToString.htm)

### Properties
- `major` (Int32, get) — Major component of version structure.
- `minor` (Int32, get) — Minor component of version structure.
- `revision` (Int32, get) — Revision component of version structure.

