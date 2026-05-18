---
name: tekla-tekla-structures-drawing
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Drawing namespace — 349 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AlongLineOrWithLeaderLineAndParentObjectAlongPartPlacingType, AlongLineOrWithLeaderLinePlacingType, AlongLinePlacing, AlongLinePlacingType, AlongPartCenteredPlacingType, AngleDimension, AngleDimensionAttributes, Arc, and 341 more types.
---

# Tekla.Structures.Drawing

Auto-generated from vendor docs for tekla 2026.0. 349 types in this namespace.

## AlongLineOrWithLeaderLineAndParentObjectAlongPartPlacingType (class)

The AlongLineOrWithLeaderLineAndParentObjectAlongPartPlacingType class defines a placing type that places the object using the along line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line and rotated to be at the same angle as the connected part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1abcb5b7-e5df-cccf-642e-fef8e6c1d34c)

### Constructors
- `AlongLineOrWithLeaderLineAndParentObjectAlongPartPlacingType(...)` — Creates a new along line or with leader line and parent object along part placing type instance.

## AlongLineOrWithLeaderLinePlacingType (class)

The AlongLineOrWithLeaderLinePlacingType class defines a placing type that places the object using the along line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/71b65a9e-490c-8fad-d0f6-73e58516134a)

### Constructors
- `AlongLineOrWithLeaderLinePlacingType(...)` — Creates a new along line or with leader line placing type instance.

## AlongLinePlacing (class)

The AlongLinePlacing class defines a placing where the object is located and moved along a line defined by two points.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/add4e174-6dc4-3eeb-7b1f-2d013882190d)

### Constructors
- `AlongLinePlacing(...)` — Creates a new along line placing instance. The given two points define the line to be used for the placing.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/a2360f3f-0551-3a34-e20d-40e9168f3c6b)

### Properties
- `EndPoint` (object, get/set) — The end point of the line that the object is placed on.
- `StartPoint` (object, get/set) — The start point of the line that the object is placed on.

## AlongLinePlacingType (class)

The AlongLinePlacingType class defines a placing type that places the object using the along line placing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7c46b8e9-1452-4a67-55c3-ed71c540f360)

### Constructors
- `AlongLinePlacingType(...)` — Creates a new along line placing type instance.

## AlongPartCenteredPlacingType (class)

The AlongPartCenteredPlacingType class defines a placing type that places the object using the along line placing. The line used will be the part's edge in the center of that line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/297035d0-7383-a0c6-3c0a-c8f868b7834b)

### Constructors
- `AlongPartCenteredPlacingType(...)` — Creates a new along part centered placing type instance.

## AngleDimension (class)

The AngleDimension class defines an angle dimension between two lines. The dimension can be shown by a triangle or in degrees. You can also enter the triangle base length.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/31a8523d-decf-f1ff-75e6-b69b16d35454)

### Constructors
- `AngleDimension(...)` — Creates a new angle dimension instance using three points.
- `AngleDimension(...)` — Creates a new angle dimension instance using the origin and two vectors relative to the origin.
- `AngleDimension(...)` — Creates a new angle dimension instance using three points.
- `AngleDimension(...)` — Creates a new angle dimension instance using the origin and two vectors relative to the origin.

### Methods
#### `Delete(...)`

Deletes the dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/6ac8b2e0-7f6f-7136-8235-14825a618b59)

#### `GetAngle(...)`

Gets the angle value (in degrees).

[Docs](https://developer.tekla.com/topic/en/18/47/ac0311b8-f59b-bea7-5d4b-b9ac1f54d81f)

#### `GetDimensionSet(Boolean)(...)`

The angle dimension doesn't have a dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/899bcaed-102c-53b2-c66a-cd0d267802a9)

#### `GetDimensionSet.(...)`

The angle dimension doesn't have a dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/5800f661-0cf8-5da1-f904-35a9cba888b4)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the dimension into the database.

[Docs](https://developer.tekla.com/topic/en/18/47/a5df4b4e-4ac4-2680-8bcb-88288c60e35b)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/ed957c2a-89f3-f3e8-6d70-4e2f82dfa726)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the existing dimension in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/916cfccb-a9c3-ca2f-6cee-1a47ca4da98f)

#### `Select(...)`

Selects the angle dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/1d63d02b-8a0e-d10e-897f-74bf5c5a93a3)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the angle dimension attributes.
- `Distance` (object, get/set) — Gets or sets the distance (from the origin to the second point's direction). The distance is measured in paper millimeters.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `Origin` (object, get/set) — Gets or sets the origin of the dimension.
- `Point1` (object, get/set) — Gets or sets the first point.
- `Point2` (object, get/set) — Gets or sets the second point.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## AngleDimensionAttributes (class)

The AngleDimensionAttributes class contains the attributes for angle dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9eb486dc-e9c5-8674-7844-e74d53d50333)

### Constructors
- `AngleDimensionAttributes(...)` — Creates a new angle dimension attributes instance which loads standard attributes.
- `AngleDimensionAttributes(...)` — Creates a new angle dimension attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/d0a96bf2-2013-a0f5-9705-6155f631da34)

#### `LoadAttributes(...)`

Loads attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/5e011069-b9fc-983a-b591-62b41524349d)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the dimension.
- `Color` (object, get/set) — The color of the dimension line.
- `Format` (object, get/set) — The format attributes of the dimension.
- `Placing` (object, get/set) — The placing attributes of the dimension.
- `Text` (object, get/set) — The text attributes of the dimension.
- `TransparentBackground` (object, get/set) — The dimension text background transparency. (transparent/opaque)
- `TriangleBase` (object, get/set) — Gets or sets the triangle base length which controls the base dimension shown for bevel dimensions (only if Type is AngleTypes.Triangle).
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the dimension line.
- `Type` (object, get/set) — The type of the angle.

## AngleTypes (enum)

The different types for angle dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/62aa1446-baf4-d2f9-326e-a1a502f11e66)

### Values
- `AngleOnSide` = `1` — Shows the angle dimensions in degrees on the side.
- `AngleAtVertex` = `3` — Shows the angle dimensions in degrees at the angle vertex.
- `Triangle` = `2` — Shows the angle dimensions using a triangle. The triangle base length can be configured.
- `TriangleWithDegrees` = `4` — Shows the angle using a triangle, also shows the angle in degrees on one side of the triangle.
- `AngleOnSideGradian` = `5` — Shows the angle dimensions in gradians on the side.
- `AngleAtVertexGradian` = `6` — Shows the angle dimensions in gradians at the angle vertex.
- `TriangleWithGradians` = `7` — Shows the angle using a triangle, also shows the angle in gradians on one side of the triangle.

## Arc (class)

The Arc class defines an arc that is a two-point line with an optional curve radius. It can be created using either two points and a radius or three points.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/20ac2e4d-1c1f-23f7-1cfa-3d94717b1751)

### Constructors
- `Arc(...)` — Creates a new arc instance with the given geometrical arc and specified attributes.
- `Arc(...)` — Creates a new arc instance with standard attributes and the given two points and radius.
- `Arc(...)` — Creates a new arc instance with standard attributes and the given three points.
- `Arc(...)` — Creates a new arc instance with the given two points, radius and specified attributes.
- `Arc(...)` — Creates a new arc instance with the given three points and specified attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/3734c213-ded8-3f5c-dd47-cedf28799ca8)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetGeometricalArc(...)`

Gets the geometrical arc representation of the drawing arc.

[Docs](https://developer.tekla.com/topic/en/18/47/684dc064-85c8-57eb-4b4d-175617808018)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/a15ea585-270f-8896-74ad-5ac256c65daf)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/b1e1063f-cd00-d3f6-34dc-2001c8c9d182)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/c9153d29-df84-5ade-c88b-c3e74ea7d676)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/f1d7996c-e540-ec0b-8d9d-876cc941d79c)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the arc.
- `EndPoint` (object, get/set) — The end point of the arc.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `IsLargeArc` (object, get/set) — Whether the arc is a large arc or not.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Radius` (object, get/set) — The radius of the arc.
- `StartPoint` (object, get/set) — The start point of the arc.

## Arc.ArcAttributes (class)

The ArcAttributes class is the attributes class for the arc.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fe75912d-5edb-8618-bff7-26101e15bcca)

### Constructors
- `Arc.ArcAttributes(...)` — Creates a new default arc attributes instance that loads standard attributes.
- `Arc.ArcAttributes(...)` — Creates a new arc attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4487b421-12ff-7f14-e459-d8007696ea56)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/e57b493d-61b1-036c-333f-0c0a8610d197)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the open graphic object. The end points of an open graphic object can contain arrowheads.
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## ArrowheadAttributes (class)

The ArrowheadAttributes class handles attributes related to arrowheads of different objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/00085909-c077-e32b-79d7-fa0e543b5826)

### Constructors
- `ArrowheadAttributes(...)` — Creates a new default arrowhead attributes instance. The default values are ArrowheadPositions.None, ArrowheadTypes.FilledArrow, height 2.0 and width 1.0.
- `ArrowheadAttributes(...)` — Creates a new arrowhead attributes instance with the given parameters.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/1cf44e40-562e-14b3-74c4-d023a20009fa)

### Properties
- `ArrowPosition` (object, get/set) — The position of the arrowhead. The position is always ArrowheadPositions.Both for dimensions.
- `Head` (object, get/set) — The type of the arrowhead.
- `Height` (object, get/set) — The height of the arrowhead.
- `Width` (object, get/set) — The width of the arrowhead.

## ArrowheadPositions (enum)

The positions of the arrowheads on the line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/24eef0f3-f2ad-7860-664a-7bc488a78dfb)

### Values
- `None` = `0` — No arrowheads on the line.
- `Start` = `1` — One arrowhead at the start of the line.
- `End` = `2` — One arrowhead at the end of the line.
- `Both` = `3` — Arrowheads at both ends of the line.

## ArrowheadTypes (enum)

The arrowhead types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/102571cd-edbe-ea94-45dd-f3fd265f745d)

### Values
- `NoArrow` = `-1005` — No arrowhead.
- `FilledArrow` = `-1004` — The filled arrowhead.
- `LineArrow` = `-1003` — The line arrowhead.
- `CircleArrow` = `-1002` — The open circle arrowhead.
- `FilledCircleArrow` = `-1001` — The filled circle arrowhead.

## AssemblyDrawing (class)

The AssemblyDrawing class is for handling assembly drawings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c03052ca-3759-02d5-95d9-5d89b274e4ad)

### Constructors
- `AssemblyDrawing(...)` — Instantiates a new assembly drawing with standard attributes.
- `AssemblyDrawing(...)` — Instantiates a new assembly drawing with standard attributes and with a specified sheet number.
- `AssemblyDrawing(...)` — Instantiates a new assembly drawing with given attributes.
- `AssemblyDrawing(...)` — Instantiates a new assembly drawing with given attributes and with a specified sheet number.

### Methods
#### `CommitChanges(String)(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/92109855-5313-7446-bef4-ab6dde6875f1)

#### `CommitChanges.(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/bc26a611-bd6a-535a-4461-943b8bf5351c)

#### `Delete(...)`

Deletes the drawing. An active drawing cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/4bcff604-27c4-487f-bb7d-5d80a2d87d8b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetPlotFileName(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/fa70baa3-ad78-2e3c-88ca-0b77cb81cd21)

#### `GetPlotFileNameExt(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/6288f4e4-4baf-656d-57bb-c63dee354493)

#### `GetSheet(...)`

Gets the sheet of the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fe921886-00b9-57d3-0488-92bd25014c66)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `Insert(...)`

Inserts a new assembly drawing. Views are added according to the View Creation rules of the standard file. A drawing can be inserted only when there is no active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/d3f97421-4b6a-a687-ceac-77eea0f54736)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/6b1b3008-c9dd-5b97-0881-89ccc580b3e7)

#### `Modify(...)`

Updates the drawing object in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/4bc5636b-288e-b3b8-1378-d01cbed0622c)

#### `PlaceViews(...)`

Places views on the sheet. Computes the required size of the views to fit the parts, then places the views so that they fit on the sheet and do not overlap. Adjusts the sheet size if needed.

[Docs](https://developer.tekla.com/topic/en/18/47/1e1578e4-084f-21a9-3d1a-2b03140440ce)

#### `Select(...)`

Selects the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/0a879f8a-8245-2cd0-2b2c-7ba4842c26bc)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

### Properties
- `AssemblyIdentifier` (object, get/set) — The identifier of the model object assembly.
- `CommitMessage` (object, get/set) — Gets the commit message
- `CreationDate` (object, get/set) — The drawing creation date.
- `DrawingTypeStr` (object, get/set) — The type of the drawing
- `IsFrozen` (object, get/set) — Describes whether the drawing is frozen.
- `IsIssued` (object, get/set) — Describes whether the drawing is issued.
- `IsIssuedButModified` (object, get/set) — Describes whether the drawing is issued but modified since it was issued.
- `IsLocked` (object, get/set) — Describes whether the drawing is locked.
- `IsLockedBy` (object, get/set) — Gets the logged in username that set the drawing to locked
- `IsMasterDrawing` (object, get/set) — Describes whether the drawing is a master drawing (shown with a special symbol on the drawing list).
- `IsReadyForIssue` (object, get/set) — Describes whether the drawing is ready for issue.
- `IsReadyForIssueBy` (object, get/set) — Gets the logged in username that set the drawing Ready for issue
- `IssuingDate` (object, get/set) — The drawing issuing date.
- `Layout` (object, get/set) — Gets the drawing layout attributes.
- `Mark` (object, get/set) — The drawing mark.
- `ModificationDate` (object, get/set) — The drawing modification date.
- `Name` (object, get/set) — The name of the drawing.
- `OutputDate` (object, get/set) — The drawing output date. XS_DISABLE_DRAWING_PLOT_DATE controls whether this date is set.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SectionViewStartLabel` (object, get/set) — Gets the drawing attribute for start label of section view
- `SheetNumber` (object, get/set) — The sheet number of the drawing.
- `Title1` (object, get/set) — The first drawing title.
- `Title2` (object, get/set) — The second drawing title.
- `Title3` (object, get/set) — The third drawing title.
- `UpToDateStatus` (object, get/set) — Gets the drawing up to date status.

## AttributesBase (class)

The AttributesBase class is the class that all the main attributes that can load attribute files inherit from.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0bdac29d-037b-476a-e615-bac587510903)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/da11fa4f-a98d-c770-d0e2-f458bc3b82dc)

#### `LoadAttributes(...)`

Loads the attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/5597a575-45e2-aa83-6576-390f23af3e48)

## AutoSizeOptions (enum)

Options for controlling how the sheet size is calculated when the SizeDefinitionMode is AutoSize.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/84383201-3a42-4585-f98b-8f150ecfcb89)

### Values
- `CalculatedSizes` = `0` — Use "calculated size" values of the selected layout in the drawing.
- `FixedSizes` = `1` — Use "fixed size" values of the selected layout in the drawing.
- `CalculatedAndFixedSizes` = `2` — Use both "fixed size" and "calculated size" values of the selected layout in the drawing.

## BaseLinePlacing (class)

The BaseLinePlacing class defines a placing where the object is placed using a line as a reference. This will thus define the object's X-axis along that line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d8a3f20c-80e0-b943-0b1f-1ab47b62fcb5)

### Constructors
- `BaseLinePlacing(...)` — Creates a new base line placing instance. The given two points define the line to be used for the placing.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e9a8dbbb-56e4-be2a-1271-32945e7a22de)

### Properties
- `EndPoint` (object, get/set) — Sets the end point of the line that the object is using for the placement.
- `StartPoint` (object, get/set) — Sets the start point of the line that the object is using for the placement.

## BaseLinePlacingType (class)

The BaseLinePlacingType class defines a placing type that places the object using the base line placing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/31d87564-ac2c-c0ca-f3a6-b0a7ff9ef1a4)

### Constructors
- `BaseLinePlacingType(...)` — Creates a new base line placing type instance.

## BaseLineWithArrowAtEndPointPlacingType (class)

The BaseLineWithArrowAtEndPointPlacingType class defines a placing type that places the object using the base line placing. The line is drawn and will have an arrow at the end point.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5a0a60b1-8b68-6762-6183-f1a3e130a7e2)

### Constructors
- `BaseLineWithArrowAtEndPointPlacingType(...)` — Creates a new base line with arrow at end point placing type instance.

## BaseLineWithArrowAtStartPointPlacingType (class)

The BaseLineWithArrowAtStartPointPlacingType class defines a placing type that places the object using the base line placing. The line is drawn and will have an arrow at the start point.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/221c5978-602c-a603-92f8-78b46dc1ebb3)

### Constructors
- `BaseLineWithArrowAtStartPointPlacingType(...)` — Creates a new base line with arrow at start point placing type instance.

## Bolt (class)

The Bolt class contains methods related to bolts. A bolt is a drawing object derived from a model object. It represents a drawing bolt that has a reference to the actual bolt in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3ab42a97-20a2-22b1-8039-cc1d67963b41)

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/ff630972-9dfb-21b6-74b6-f689d2bed3b9)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/06305d37-4832-b9ef-d733-a2fc8aa663bd)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/56c04ba2-c989-a025-b5cf-ef3152c41785)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/07517741-6202-527b-9ed0-8c0207b2bff3)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/a2e18876-b63d-312e-9c0e-4da3fe203cfc)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the bolt. For more information see Bolt.BoltAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Bolt.BoltAttributes (class)

The BoltAttributes class is the attributes class for the bolt.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7edfee41-9b01-8791-1650-e4b2e91a169b)

### Constructors
- `Bolt.BoltAttributes(...)` — Creates a new default bolt attributes instance that loads standard attributes.
- `Bolt.BoltAttributes(...)` — Creates a new bolt attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/54817577-3916-4204-aee7-4746a827260f)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/544b8092-9dc3-ef98-1b98-6c8c619f4714)

### Properties
- `Color` (object, get/set) — The color of the bolt.
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the bolt
- `Representation` (object, get/set) — The representation type of the bolt.
- `SymbolContainsAxis` (object, get/set) — True if the bolt symbol contains an axis.
- `SymbolContainsHole` (object, get/set) — True if the bolt symbol contains a hole.
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the bolt.

## Bolt.Representation (enum)

The bolt representations.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d7923fba-2a16-e1e1-262f-82e3b4fe739c)

### Values
- `Solid` = `1` — The solid representation.
- `ExactSolid` = `9` — The exact solid representation.
- `Symbol` = `2` — The symbol representation.
- `Symbol2` = `4` — The second symbol representation.
- `Symbol3` = `5` — The third symbol representation.
- `DINSymbol` = `7` — The DIN symbol representation.
- `UserDefined` = `8` — The user defined representation.

## CannotCreateSectionViewFrom3dView (class)

The CannotCreateSectionViewFrom3dView class defines the exception that is thrown when a user tries to create a section view from a 3d view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4df15a75-29d2-f97f-fb2d-26edc10a4b2b)

### Constructors
- `CannotCreateSectionViewFrom3dView(...)` — Creates a new cannot create section view from 3d view exception instance.

## CannotDeleteActiveDrawingException (class)

The CannotDeleteActiveDrawingException class defines the exception that is thrown when a user tries to delete a drawing that is active.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ed998625-1913-2678-66f0-9e32917fb5d3)

### Constructors
- `CannotDeleteActiveDrawingException(...)` — Creates a new cannot delete active drawing exception instance.

## CannotGetAttributeForPluginDueToIncorrectTypeException (class)

The CannotGetAttributeForPluginDueToIncorrectTypeException class defines the exception that is thrown when Plugin.GetAttribute is called but the type is incorrect.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/baa4d03b-dd89-501c-d04c-6a8b6b415caa)

### Constructors
- `CannotGetAttributeForPluginDueToIncorrectTypeException(...)` — Creates a new CannotGetAttributeForPluginDueToIncorrectTypeException instance.

## CannotGetAttributeForPluginDueToInexistantFieldException (class)

The CannotGetAttributeForPluginDueToInexistantFieldException class defines the exception that is thrown when Plugin.GetAttribute is called but the field does not exist.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3d81a833-510c-0c73-42d2-02724799f341)

### Constructors
- `CannotGetAttributeForPluginDueToInexistantFieldException(...)` — Creates a new CannotGetAttributeForPluginDueToInexistantFieldException instance.

## CannotGetAttributeForPluginException (class)

The CannotGetAttributeForPluginException class defines the exception that is thrown when Plugin.GetAttribute is called but the attribute cannot be fetched.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3b434c42-0064-df39-c5b6-891eb1ded242)

### Constructors
- `CannotGetAttributeForPluginException(...)` — Creates a new CannotGetAttributeForPluginException instance.
- `CannotGetAttributeForPluginException(...)` — Creates a new CannotGetAttributeForPluginException instance.

## CannotInsertDrawingException (class)

The CannotInsertDrawingException class defines the exception that is thrown when a user tries to insert a drawing when another drawing is active.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/82d206f5-b423-ee30-ab9e-012751862593)

### Constructors
- `CannotInsertDrawingException(...)` — Creates a new cannot insert drawing exception instance.

## CannotLoadAttributesException (class)

The CannotLoadAttributesException class defines the exception that is thrown when the loading of attributes failed.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3e4c1779-ba8f-f298-49cd-bc239cabfa6c)

### Constructors
- `CannotLoadAttributesException(...)` — Creates a new cannot load attributes exception instance.

## CannotModifyNonActiveDrawingException (class)

The CannotModifyNonActiveDrawingException class defines the exception that is thrown when a user tries to modify a drawing that is not set as active.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4ab894ff-8e7a-f4cd-c372-f25135fb5c47)

### Constructors
- `CannotModifyNonActiveDrawingException(...)` — Creates a new cannot modify non-active drawing exception instance.

## CannotPerformOperationDrawingEditorMustBeClosedException (class)

The CannotPerformOperationDrawingEditorMustBeClosedException class defines the exception that is thrown when a user tries to do an operation which requires the drawing editor to be closed.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9f749962-f48a-a9fd-454a-17d708165ec3)

### Constructors
- `CannotPerformOperationDrawingEditorMustBeClosedException(...)` — Creates a new cannot perform operation drawing editor must be closed exception instance.

## CannotPerformOperationDrawingIsActiveException (class)

The CannotPerformOperationDrawingIsActiveException class defines the exception that is thrown when a user tries to do an operation which requires that the drawing is not active.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/68121f5f-165a-4167-1d2f-3e94360abb60)

### Constructors
- `CannotPerformOperationDrawingIsActiveException(...)` — Creates a new cannot perform operation drawing is active exception instance.

## CannotPerformOperationDrawingMustBeActiveException (class)

The CannotPerformOperationDrawingMustBeActiveException class defines the exception that is thrown when a user tries to do an operation which requires that the drawing is active.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/16b0a49f-98a0-95da-abec-823b740d8e7d)

### Constructors
- `CannotPerformOperationDrawingMustBeActiveException(...)` — Creates a new cannot perform operation drawing must be active exception instance.

## CannotPerformOperationDrawingNotUpToDateException (class)

The CannotPerformOperationDrawingNotUpToDateException class defines the exception that is thrown when a user tries to do an operation which requires that the drawing is active.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8b0e3916-a1a7-38fe-717e-487e8997f1b7)

### Constructors
- `CannotPerformOperationDrawingNotUpToDateException(...)` — Creates a new cannot perform operation drawing not up to date exception instance.

## CannotPerformOperationNumberingNotUpToDate (class)

The CannotPerformOperationNumberingNotUpToDate class defines the exception that is thrown when a user tries to do an operation which requires that numbering is up to date.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6cb682bc-5f77-e0e0-7573-25d4e40122a8)

### Constructors
- `CannotPerformOperationNumberingNotUpToDate(...)` — Creates a new cannot perform operation numbering not up to date exception instance.

## CannotSetAttributeForPluginDueToIncorrectTypeException (class)

The CannotSetAttributeForPluginDueToIncorrectTypeException class defines the exception that is thrown when Plugin.SetAttribute is called but the type is incorrect.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/788d7bc8-e2f4-45e4-5d97-dab2b698d816)

### Constructors
- `CannotSetAttributeForPluginDueToIncorrectTypeException(...)` — Creates a new CannotSetAttributeForPluginDueToIncorrectTypeException instance.

## CannotSetAttributeForPluginDueToInexistantFieldException (class)

The CannotSetAttributeForPluginDueToInexistantFieldException class defines the exception that is thrown when Plugin.SetAttribute is called but the field does not exist.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/dcf4f81a-e1c6-e81e-4551-cbc9184267c5)

### Constructors
- `CannotSetAttributeForPluginDueToInexistantFieldException(...)` — Creates a new CannotSetAttributeForPluginDueToInexistantFieldException instance.

## CannotSetAttributeForPluginException (class)

The CannotSetAttributeForPluginException class defines the exception that is thrown when Plugin.SetAttribute is called but the attribute cannot be set.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7b1a8e88-5ea9-24db-e3a6-7d310e170954)

### Constructors
- `CannotSetAttributeForPluginException(...)` — Creates a new CannotSetAttributeForPluginException instance.
- `CannotSetAttributeForPluginException(...)` — Creates a new CannotSetAttributeForPluginException instance.

## CastUnitDrawing (class)

The CastUnitDrawing class is for handling cast unit drawings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4bdbe4f7-156c-9d02-1b17-8b6423b2b501)

### Constructors
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing with standard attributes for a given cast unit.
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing with standard attributes and with a specified sheet number for a given cast unit.
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing with given attributes for a given cast unit.
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing with standard attributes for a given cast unit.
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing with given attributes and with a specified sheet number for a given cast unit.
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing by id with standard attributes and with a specified sheet number for a given cast unit.
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing with given attributes for a given cast unit.
- `CastUnitDrawing(...)` — Instantiates a new cast unit drawing by id with given attributes and with a specified sheet number for a given cast unit.

### Methods
#### `CommitChanges(String)(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/92109855-5313-7446-bef4-ab6dde6875f1)

#### `CommitChanges.(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/bc26a611-bd6a-535a-4461-943b8bf5351c)

#### `Delete(...)`

Deletes the drawing. An active drawing cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/4bcff604-27c4-487f-bb7d-5d80a2d87d8b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetPlotFileName(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/fa70baa3-ad78-2e3c-88ca-0b77cb81cd21)

#### `GetPlotFileNameExt(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/6288f4e4-4baf-656d-57bb-c63dee354493)

#### `GetSheet(...)`

Gets the sheet of the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fe921886-00b9-57d3-0488-92bd25014c66)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `Insert(...)`

Inserts a new cast unit drawing. Views are added according to the View Creation rules of the standard file. A drawing can be inserted only when there is no active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/6fd4fc68-eff2-0003-4586-1e96334393b2)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/6b1b3008-c9dd-5b97-0881-89ccc580b3e7)

#### `Modify(...)`

Updates the drawing object in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/805cea6a-4c0a-9a26-4dbf-3a0e712e5e70)

#### `PlaceViews(...)`

Places views on the sheet. Computes the required size of the views to fit the parts, then places the views so that they fit on the sheet and do not overlap. Adjusts the sheet size if needed.

[Docs](https://developer.tekla.com/topic/en/18/47/1e1578e4-084f-21a9-3d1a-2b03140440ce)

#### `Select(...)`

Selects the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/ae2c0f36-9460-e30f-63ce-02a01de323f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

### Properties
- `CastUnitById` (object, get/set) — Cast unit definition method. True means the cast unit definition method is by cast unit id. False means the cast unit definition method is by cast unit position.
- `CastUnitIdentifier` (object, get/set) — The identifier of the model object cast unit.
- `CommitMessage` (object, get/set) — Gets the commit message
- `CreationDate` (object, get/set) — The drawing creation date.
- `DrawingTypeStr` (object, get/set) — The type of the drawing
- `IsFrozen` (object, get/set) — Describes whether the drawing is frozen.
- `IsIssued` (object, get/set) — Describes whether the drawing is issued.
- `IsIssuedButModified` (object, get/set) — Describes whether the drawing is issued but modified since it was issued.
- `IsLocked` (object, get/set) — Describes whether the drawing is locked.
- `IsLockedBy` (object, get/set) — Gets the logged in username that set the drawing to locked
- `IsMasterDrawing` (object, get/set) — Describes whether the drawing is a master drawing (shown with a special symbol on the drawing list).
- `IsReadyForIssue` (object, get/set) — Describes whether the drawing is ready for issue.
- `IsReadyForIssueBy` (object, get/set) — Gets the logged in username that set the drawing Ready for issue
- `IssuingDate` (object, get/set) — The drawing issuing date.
- `Layout` (object, get/set) — Gets the drawing layout attributes.
- `Mark` (object, get/set) — The drawing mark.
- `ModificationDate` (object, get/set) — The drawing modification date.
- `Name` (object, get/set) — The name of the drawing.
- `OutputDate` (object, get/set) — The drawing output date. XS_DISABLE_DRAWING_PLOT_DATE controls whether this date is set.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SectionViewStartLabel` (object, get/set) — Gets the drawing attribute for start label of section view
- `SheetNumber` (object, get/set) — The sheet number of the drawing.
- `Title1` (object, get/set) — The first drawing title.
- `Title2` (object, get/set) — The second drawing title.
- `Title3` (object, get/set) — The third drawing title.
- `UpToDateStatus` (object, get/set) — Gets the drawing up to date status.

## CastUnitDrawing.CastUnitDrawingCreationType (enum)

Defines the Cast Unit Drawing Creation type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9c093d34-dd10-12ed-df8a-b676bf0e32f2)

### Values
- `CastUnitDrawingByPosition` = `0` — Creating the Cast Unit drawing by Position means we follow the position of the cast unit.
- `CastUnitDrawingById` = `1` — Creating the Cast Unit drawing by Id means we follow the id of the cast unit.

## Circle (class)

The Circle class defines the graphical object circle. A circle can also be hatched.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/50243dfc-c212-a805-ac2e-dee4eead998c)

### Constructors
- `Circle(...)` — Creates a new circle instance with a center point and a radius using standard attributes.
- `Circle(...)` — Creates a new circle instance with a center point and a radius using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/6df2855d-fd20-7168-74f9-4f264ec2f1ec)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/4946e483-66b0-abf8-9b3e-fc99403c1306)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/6ac2daad-bf08-ace1-c4c1-163249956b4d)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/cda4496c-372e-171e-c74e-f4c0df95bbdb)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/1a3943e8-f04f-13ba-f4a6-b484b32e0c57)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the circle.
- `CenterPoint` (object, get/set) — The center point of the circle.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Radius` (object, get/set) — The radius of the circle.

## Circle.CircleAttributes (class)

The CircleAttributes class is the attributes class of the circle.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1e037ad1-c3f9-cdf6-fb5d-3b6fcb191cc9)

### Constructors
- `Circle.CircleAttributes(...)` — Creates a new default circle attributes instance that loads standard attributes.
- `Circle.CircleAttributes(...)` — Creates a new circle attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/72984e43-0196-1930-e922-544f93db56ef)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/bd0ae190-2c49-55b6-41f9-ba7ff3e3be49)

### Properties
- `BehindModelObjects` (object, get/set) — Gets or sets whether the closed graphic object is drawn behind model objects
- `Hatch` (object, get/set) — The hatch attributes of the closed graphic object (the closed graphic object is hatchable).
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## ClosedGraphicObject (class)

The ClosedGraphicObject class is an abstract base class for all closed graphic objects (the circle, the cloud, the rectangular cloud, the polygon and the rectangle).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/745b2585-5546-ebce-f1bf-4e43cf118ce3)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## ClosedGraphicObject.ClosedGraphicObjectAttributes (class)

The ClosedGraphicObjectAttributes class is the attributes class for the closed graphic object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/97faf0f7-b5ad-77df-42a3-50ed1b6c5a6b)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/da11fa4f-a98d-c770-d0e2-f458bc3b82dc)

#### `LoadAttributes(...)`

Loads the attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/5597a575-45e2-aa83-6576-390f23af3e48)

### Properties
- `BehindModelObjects` (object, get/set) — Gets or sets whether the closed graphic object is drawn behind model objects
- `Hatch` (object, get/set) — The hatch attributes of the closed graphic object (the closed graphic object is hatchable).
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## Cloud (class)

The Cloud class defines a cloud that is a polygon with a specific size set for the bulges.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/57002cc6-3302-3201-156a-658298f9a70f)

### Constructors
- `Cloud(...)` — Creates a new cloud instance with a point list using standard attributes.
- `Cloud(...)` — Creates a new cloud instance with a point list using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/91ea91a7-8608-68ff-c7d9-723d35795fae)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/7ec820e4-0e70-66e8-1fba-784f69adfa57)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/f51059ef-fd04-ba2f-d627-79f4c75d9998)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/cd83b70d-a722-d035-4a15-6ceeda0e7ef2)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/fb50ee22-685e-15b4-b4f7-f09dc9be4429)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcWidth` (object, get/set) — The arc width to use for the cloud base. The property usage follows the below legacy cloud creation rules. In case of positive value then that value will be used. In case of zero value then the positive value of the XS_ARC_WIDTH_OF_CLOUD advance option is used and in case that also the advance option value is 0 then the default legacy 10 value is used. In case of negative value the value is transformed and its positive value is used.
- `Attributes` (object, get/set) — The attributes for the cloud.
- `Bulge` (object, get/set) — The bulge (curve) to use (a bulge is the ratio between the height and width of an object).
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `Points` (object, get/set) — The list of points for the cloud. The points you insert will specify the corner points of the cloud. After you insert the object, you will get every cloud point back.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Cloud.CloudAttributes (class)

The CloudAttributes class is the attributes class for the cloud.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c0fcd085-a3df-192b-9afe-5328bbc2b9d3)

### Constructors
- `Cloud.CloudAttributes(...)` — Creates a new default cloud attributes instance that loads standard attributes.
- `Cloud.CloudAttributes(...)` — Creates a new cloud attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/d146ade3-346c-b427-ffcf-4ca9c035c862)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/08b8a7b2-6e98-ee0a-854d-aeb994de3f31)

### Properties
- `BehindModelObjects` (object, get/set) — Gets or sets whether the closed graphic object is drawn behind model objects
- `Hatch` (object, get/set) — The hatch attributes of the closed graphic object (the closed graphic object is hatchable).
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## CloudBase (class)

The CloudBase class is an abstract base class for all cloud objects (the cloud and the rectangular cloud).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2243156d-e909-9f38-ce77-2528985a1a7e)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcWidth` (object, get/set) — The arc width to use for the cloud base. The property usage follows the below legacy cloud creation rules. In case of positive value then that value will be used. In case of zero value then the positive value of the XS_ARC_WIDTH_OF_CLOUD advance option is used and in case that also the advance option value is 0 then the default legacy 10 value is used. In case of negative value the value is transformed and its positive value is used.
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Bulge` (object, get/set) — The bulge (curve) to use (a bulge is the ratio between the height and width of an object).
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## ColorSelection (class)

This class provides methods to show the color palette for drawing colors and hatch background colors.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6def533d-91c7-8220-4dc7-66f666250c25)

### Methods
#### `ShowColorPalette(...)`

Shows the color palette for drawing colors.

[Docs](https://developer.tekla.com/topic/en/18/47/c3181ad1-7bcf-0aaf-f5bb-fc6371bd4567)

#### `ShowColorPaletteForHatchBackground(...)`

Shows the color palette for hatch background colors.

[Docs](https://developer.tekla.com/topic/en/18/47/4981902e-df62-64ad-71e1-432376fcbd38)

## Connection (class)

The Connection class contains methods related to connections. A connection is a drawing object derived from a model object. It represents a drawing connection that has a reference to the actual connection in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4d614ec4-28d4-f693-d727-7821825cbc54)

### Methods
#### `Delete(...)`

Deletes the drawing connection from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/f6ec2a51-f9a0-8833-ed72-cca89146ef33)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/7b83a813-1750-37d4-a676-a39e76c9aab3)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/be433a07-6340-1730-e7f6-d8be8bb1b6ac)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/5a7ff134-0b18-c287-0949-44e8bdc6a5f3)

#### `Select(...)`

Selects the drawing connection from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/23e6034e-c98c-0a55-57e5-d099487e21a1)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## ContainerElement (class)

The ContainerElement class defines container elements. A container element is a mark element that has a frame and contains other mark elements. A container element may also contain container elements to achieve nested frames.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a922aaee-1a0a-7083-3f39-496dff2e4641)

### Constructors
- `ContainerElement(...)` — Initializes a new instance of the ContainerElement class

### Methods
#### `Add(...)`

Adds a new element base to the end of the list.

[Docs](https://developer.tekla.com/topic/en/18/47/01ce5ad6-2d97-4dba-7a04-6459a4476c13)

#### `Clear(...)`

Removes all elements from the container instance.

[Docs](https://developer.tekla.com/topic/en/18/47/3ebf4c11-e7c8-6443-e915-da0b4ce362f8)

#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/8c2b52ff-4f86-62b0-0f43-3ea9f9abbc41)

#### `CopyTo(...)`

Copies the elements of the ICollection to an Array, starting at a particular Array index.

[Docs](https://developer.tekla.com/topic/en/18/47/25d92102-0435-213f-88b4-75f7e22a74fc)

#### `GetEnumerator(...)`

Returns an enumerator that iterates through a collection.

[Docs](https://developer.tekla.com/topic/en/18/47/fcecd7e0-80e1-bce3-3c16-0132ee1f7d1e)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/a7f3aff9-b3d9-5fb4-3044-7937d3719ed7)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/970e9be2-ffcc-8465-8a29-8f5d872deb90)

### Properties
- `Count` (object, get/set) — Gets the number of elements contained in the ICollection.
- `Frame` (object, get/set) — The frame of the container element.
- `IsSynchronized` (object, get/set) — Gets a value indicating whether access to the ICollection is synchronized (thread safe).
- `SyncRoot` (object, get/set) — Gets an object that can be used to synchronize access to the ICollection.

## ContainerView (class)

The ContainerView class contains container views that contain other drawing objects, including other views and container views. The drawing sheet is a container view. Container views do not contain drawing representations of model objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b688c316-8997-c314-172a-f0663ad1def1)

### Methods
#### `Delete(...)`

Deletes the view and its children objects from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/c7b5ef15-0c29-6a2a-ef92-99fb8e0ed0b3)

#### `GetAllObjects(.Type.)(...)`

Gets all the objects and their children objects in the view that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/d9638840-5e65-49e6-cadb-2e2a8b5423fd)

#### `GetAllObjects(Type)(...)`

Gets all the objects and their children objects in the view that are of a certain type.

[Docs](https://developer.tekla.com/topic/en/18/47/d8756000-215b-c1e0-49c7-d36dc245bec8)

#### `GetAllObjects.(...)`

Gets all the objects and their children objects in the view.

[Docs](https://developer.tekla.com/topic/en/18/47/6c8f0774-654d-498b-93d5-904797569fd5)

#### `GetAllViews(...)`

Gets all the views that are placed on the container view and their children views.

[Docs](https://developer.tekla.com/topic/en/18/47/e39c78bb-231b-ae59-0503-be6a015de47c)

#### `GetAxisAlignedBoundingBox(...)`

The view size in paper coordinates.

[Docs](https://developer.tekla.com/topic/en/18/47/c51d1265-7cc9-ca4c-86b0-d76fe62d8be8)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetModelObjects(...)`

Gets the drawing model objects based on given model object identifier. If used from sheet model objects from all views are returned.

[Docs](https://developer.tekla.com/topic/en/18/47/c59be515-bde2-42ef-2e58-fa8a4e0173ba)

#### `GetObjects(.Type.)(...)`

Gets the objects in the view that are of certain types. Does not return child objects (single dimensions, grid lines, views).

[Docs](https://developer.tekla.com/topic/en/18/47/e64fef01-3761-6798-cd3f-d5e5d006a37f)

#### `GetObjects.(...)`

Gets the objects in the view. Does not return child objects (single dimensions, grid lines, views).

[Docs](https://developer.tekla.com/topic/en/18/47/bd691388-4a5f-d33f-3364-d516c037cb21)

#### `GetOriginalDrawing(...)`

Gets the drawing on which the view was originally created. @see DrawingObject.GetDrawing() for retrieving the current drawing the object resides on.

[Docs](https://developer.tekla.com/topic/en/18/47/b34f08ba-6799-5511-dd98-8654ffbc1ebd)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `GetViews(...)`

Gets all the views that are placed on the container view.

[Docs](https://developer.tekla.com/topic/en/18/47/23495154-dc7d-df55-608c-a96542f311b7)

#### `Insert(...)`

Inserts the view to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/c5735e20-f3b0-3147-4879-3e1e42c30d82)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/f11de6f6-8c6b-74f6-9234-1d8ebd76a197)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the view in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/883ce75d-2048-41ab-05ba-96ffe20220aa)

#### `Select(...)`

Selects the view from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/4630a41b-4e54-0f11-a21d-eed889061760)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `ExtremaCenter` (object, get/set) — Gets the view extrema center.
- `FrameOrigin` (object, get/set) — The vector from the view origin to the frame origin.
- `Height` (object, get/set) — The height of the view frame in paper coordinates.
- `IsSheet` (object, get/set) — Gets a value indicating whether the container view is the drawing's sheet.
- `Origin` (object, get/set) — The view origin coordinates in the sheet.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Width` (object, get/set) — The width of the view frame in paper coordinates.

## CurvedDimensionBase (class)

The CurvedDimensionBase class is the base class for radial and orthogonal curved dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/49102592-8fe6-b5a3-9a31-c2d6df27404c)

### Methods
#### `Delete(...)`

Deletes the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/0fe9e404-f4b0-1b76-9e4a-e4c0a5e7367c)

#### `GetDimensionSet(Boolean)(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/c6436216-db95-d799-f218-b48db2bb953e)

#### `GetDimensionSet.(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/397b6ec6-423b-5262-4f0e-b1a277a5315c)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the dimension to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/1cf2f1c7-2de0-5af6-0bf4-3743aa15e0ec)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/a0d6dd2a-b634-2442-8ff1-05f858036979)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

The single dimension cannot be modified.

[Docs](https://developer.tekla.com/topic/en/18/47/b90fa807-4bb2-1206-ee8c-d4cdd4499ed9)

#### `Select(...)`

Selects the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/fef12af9-74d4-5ba8-21bc-bd644e554812)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcPoint1` (object, get/set) — Gets the first arc point.
- `ArcPoint2` (object, get/set) — Gets the second arc point.
- `ArcPoint3` (object, get/set) — Gets the third arc point.
- `Attributes` (object, get/set) — Gets or sets the curved dimension set base attributes.
- `Distance` (object, get/set) — Gets the distance.
- `EndPoint` (object, get/set) — The end point of the dimension.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — The start point of the dimension.

## CurvedDimensionOrthogonal (class)

The CurvedDimensionOrthogonal class defines a curved dimension with orthogonal reference lines.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/acd27e09-3557-4ceb-9755-c78b11ec20bb)

### Constructors
- `CurvedDimensionOrthogonal(...)` — Creates a new curved dimension with orthogonal reference lines using default attributes.
- `CurvedDimensionOrthogonal(...)` — Creates a new curved dimension with orthogonal reference lines.

### Methods
#### `Delete(...)`

Deletes the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/0fe9e404-f4b0-1b76-9e4a-e4c0a5e7367c)

#### `GetDimensionSet(Boolean)(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/5b00bfdf-f6a1-c3a8-577b-518f67fcabd9)

#### `GetDimensionSet.(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/397b6ec6-423b-5262-4f0e-b1a277a5315c)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the dimension to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/1cf2f1c7-2de0-5af6-0bf4-3743aa15e0ec)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c279d670-d75c-1dc7-a0c2-684adf985d71)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

The single dimension cannot be modified.

[Docs](https://developer.tekla.com/topic/en/18/47/b90fa807-4bb2-1206-ee8c-d4cdd4499ed9)

#### `Select(...)`

Selects the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/fef12af9-74d4-5ba8-21bc-bd644e554812)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcPoint1` (object, get/set) — Gets the first arc point.
- `ArcPoint2` (object, get/set) — Gets the second arc point.
- `ArcPoint3` (object, get/set) — Gets the third arc point.
- `Attributes` (object, get/set) — Gets or sets the curved dimension set orthogonal attributes.
- `Distance` (object, get/set) — Gets the distance.
- `EndPoint` (object, get/set) — The end point of the dimension.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — The start point of the dimension.

## CurvedDimensionRadial (class)

The CurvedDimensionRadial class defines a curved dimension with radial reference points. The dimension text on the line can be either a distance or an angle value.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/061b94cd-8971-4993-a437-736478c0a124)

### Constructors
- `CurvedDimensionRadial(...)` — Creates a new curved dimension with radial reference points using standard attributes.
- `CurvedDimensionRadial(...)` — Creates a new curved dimension with radial reference points.

### Methods
#### `Delete(...)`

Deletes the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/0fe9e404-f4b0-1b76-9e4a-e4c0a5e7367c)

#### `GetDimensionSet(Boolean)(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/42d0cdfc-a52f-74e8-948d-f41699e2507e)

#### `GetDimensionSet.(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/397b6ec6-423b-5262-4f0e-b1a277a5315c)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the dimension to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/1cf2f1c7-2de0-5af6-0bf4-3743aa15e0ec)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/7e844a5e-ab51-0820-ee74-9f36a0182106)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

The single dimension cannot be modified.

[Docs](https://developer.tekla.com/topic/en/18/47/b90fa807-4bb2-1206-ee8c-d4cdd4499ed9)

#### `Select(...)`

Selects the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/fef12af9-74d4-5ba8-21bc-bd644e554812)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcPoint1` (object, get/set) — Gets the first arc point.
- `ArcPoint2` (object, get/set) — Gets the second arc point.
- `ArcPoint3` (object, get/set) — Gets the third arc point.
- `Attributes` (object, get/set) — Gets or sets the curved dimension set base attributes.
- `Distance` (object, get/set) — Gets the distance.
- `EndPoint` (object, get/set) — The end point of the dimension.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — The start point of the dimension.

## CurvedDimensionSetBase (class)

The CurvedDimensionSetBase class is the base class for curved dimension sets which are defined by an arc and a point list.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/006bfc8c-7cbe-5fcf-136e-6584734699cb)

### Methods
#### `AddToDimensionSet(...)`

Adds a dimension set to the current dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/c6ae7b87-49aa-8bb4-456a-61e0f37e1637)

#### `Delete(...)`

Deletes the dimension set and its child dimensions from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/7530db16-3569-f9c9-cd5d-f2d5bae026c8)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjects(.Type.)(...)`

Gets the children straight dimensions of the current straight dimension set that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/0ec2481b-08c7-d5e3-41e4-291bfbb31bbc)

#### `GetObjects.(...)`

Gets the children straight dimensions of the current straight dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/a5b00ccb-a302-c9bd-b940-6b64af7bccdc)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Curved dimensions are created using the curved dimension set handler.

[Docs](https://developer.tekla.com/topic/en/18/47/f1c0f423-4134-c6bd-da4e-60d35f2accbe)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the existing dimension set in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/922d79ff-78bb-3f5f-2721-e9e1cb0b1f14)

#### `Select(...)`

Selects the dimension set from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/9f3d0ef6-b4fd-6b13-b7e5-6a3713410b6a)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcPoint1` (object, get/set) — Gets the first arc point.
- `ArcPoint2` (object, get/set) — Gets the second arc point.
- `ArcPoint3` (object, get/set) — Gets the third arc point.
- `Attributes` (object, get/set) — Gets or sets the dimension set attributes.
- `DimensionPoints` (object, get/set) — Gets a list of the dimension points.
- `Distance` (object, get/set) — Gets the distance. The distance is measured in paper millimeters.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## CurvedDimensionSetBase.CurvedDimensionSetBaseAttributes (class)

The CurvedDimensionSetBaseAttributes class contains the attributes for the curved dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0f753e4c-b5ed-87df-f138-16e63f6dd96b)

### Constructors
- `CurvedDimensionSetBase.CurvedDimensionSetBaseAttributes(...)` — Creates a new curved dimension set base attributes instance which loads standard attributes.
- `CurvedDimensionSetBase.CurvedDimensionSetBaseAttributes(...)` — Creates a new curved dimension set base attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/da11fa4f-a98d-c770-d0e2-f458bc3b82dc)

#### `LoadAttributes(...)`

Loads the attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/5597a575-45e2-aa83-6576-390f23af3e48)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the dimension.
- `Color` (object, get/set) — The color of the dimension line.
- `Format` (object, get/set) — The format attributes of the dimension.
- `Placing` (object, get/set) — The placing attributes of the dimension.
- `Text` (object, get/set) — The text attributes of the dimension.
- `TransparentBackground` (object, get/set) — The dimension text background transparency. (transparent/opaque)
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the dimension line.

## CurvedDimensionSetHandler (class)

The CurvedDimensionSetHandler class contains operations for creating curved dimension sets.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3b6dc735-5bad-9707-8004-f93b4f170e3b)

### Constructors
- `CurvedDimensionSetHandler(...)` — Initializes a new instance of the CurvedDimensionSetHandler class

### Methods
#### `CreateCurvedDimensionSetOrthogonal(ViewBase, Point, Point, Point, PointList, Double)(...)`

Creates a curved dimension set with orthogonal reference lines from the specified points and inserts it to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/af2e21ff-e20c-e3ee-7e1f-4f8ce296d084)

#### `CreateCurvedDimensionSetOrthogonal(ViewBase, Point, Point, Point, PointList, Double, CurvedDimensionSetOrthogonal.CurvedDimensionSetOrthogonalAttributes)(...)`

Creates a curved dimension set with orthogonal reference lines from the specified points and inserts it to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/9ce6fb41-f2ea-6d9d-7a18-34a2644ba4ed)

#### `CreateCurvedDimensionSetRadial(ViewBase, Point, Point, Point, PointList, Double)(...)`

Creates a curved dimension set from the specified points and inserts it to the database. Uses the default attributes.

[Docs](https://developer.tekla.com/topic/en/18/47/a1fb93d0-4b06-952e-ce28-7e5c314d3a4a)

#### `CreateCurvedDimensionSetRadial(ViewBase, Point, Point, Point, PointList, Double, CurvedDimensionSetRadial.CurvedDimensionSetRadialAttributes)(...)`

Creates a curved dimension set from the specified points and inserts it to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/67a02edd-e008-051a-d444-09f0edd8d1ca)

## CurvedDimensionSetOrthogonal (class)

The CurvedDimensionSetOrthogonal class defines a curved dimension set with orthogonal reference lines.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/db6f6bf2-87e3-5f87-61f1-e8ecd39666d7)

### Methods
#### `AddToDimensionSet(...)`

Adds a dimension set to the current dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/c6ae7b87-49aa-8bb4-456a-61e0f37e1637)

#### `Delete(...)`

Deletes the dimension set and its child dimensions from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/7530db16-3569-f9c9-cd5d-f2d5bae026c8)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjects(.Type.)(...)`

Gets the children straight dimensions of the current straight dimension set that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/0ec2481b-08c7-d5e3-41e4-291bfbb31bbc)

#### `GetObjects.(...)`

Gets the children straight dimensions of the current straight dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/a5b00ccb-a302-c9bd-b940-6b64af7bccdc)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Curved dimensions are created using the curved dimension set handler.

[Docs](https://developer.tekla.com/topic/en/18/47/f1c0f423-4134-c6bd-da4e-60d35f2accbe)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/98076ae0-ce6d-7234-641b-ee956ef68b23)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the existing dimension set in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/922d79ff-78bb-3f5f-2721-e9e1cb0b1f14)

#### `Select(...)`

Selects the dimension set from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/9f3d0ef6-b4fd-6b13-b7e5-6a3713410b6a)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcPoint1` (object, get/set) — Gets the first arc point.
- `ArcPoint2` (object, get/set) — Gets the second arc point.
- `ArcPoint3` (object, get/set) — Gets the third arc point.
- `Attributes` (object, get/set) — The curved dimension set orthogonal attributes.
- `DimensionPoints` (object, get/set) — Gets a list of the dimension points.
- `Distance` (object, get/set) — Gets the distance. The distance is measured in paper millimeters.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## CurvedDimensionSetOrthogonal.CurvedDimensionSetOrthogonalAttributes (class)

The CurvedDimensionSetOrthogonalAttributes class contains the attributes for managing all the attributes of the curved dimension with orthogonal reference lines.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/16c36036-7daa-fd1f-f952-c51698a43051)

### Constructors
- `CurvedDimensionSetOrthogonal.CurvedDimensionSetOrthogonalAttributes(...)` — Creates a new curved dimension set orthogonal attributes instance which loads standard attributes.
- `CurvedDimensionSetOrthogonal.CurvedDimensionSetOrthogonalAttributes(...)` — Creates a new curved dimension set orthogonal attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/ab0e47b8-1142-af1e-1c0d-d1588ece7aa1)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/cb80bc08-15ad-ac9a-ceeb-471936a6140e)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the dimension.
- `Color` (object, get/set) — The color of the dimension line.
- `Format` (object, get/set) — The format attributes of the dimension.
- `Placing` (object, get/set) — The placing attributes of the dimension.
- `Text` (object, get/set) — The text attributes of the dimension.
- `TransparentBackground` (object, get/set) — The dimension text background transparency. (transparent/opaque)
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the dimension line.

## CurvedDimensionSetRadial (class)

The CurvedDimensionSetRadial class defines a curved dimension set with radial reference lines.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/26636fc7-1a06-d8ab-537f-8724b9e485d2)

### Methods
#### `AddToDimensionSet(...)`

Adds a dimension set to the current dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/c6ae7b87-49aa-8bb4-456a-61e0f37e1637)

#### `Delete(...)`

Deletes the dimension set and its child dimensions from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/7530db16-3569-f9c9-cd5d-f2d5bae026c8)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjects(.Type.)(...)`

Gets the children straight dimensions of the current straight dimension set that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/0ec2481b-08c7-d5e3-41e4-291bfbb31bbc)

#### `GetObjects.(...)`

Gets the children straight dimensions of the current straight dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/a5b00ccb-a302-c9bd-b940-6b64af7bccdc)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Curved dimensions are created using the curved dimension set handler.

[Docs](https://developer.tekla.com/topic/en/18/47/f1c0f423-4134-c6bd-da4e-60d35f2accbe)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/51b56141-5bea-8c94-2588-f3a218485795)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the existing dimension set in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/922d79ff-78bb-3f5f-2721-e9e1cb0b1f14)

#### `Select(...)`

Selects the dimension set from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/9f3d0ef6-b4fd-6b13-b7e5-6a3713410b6a)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcPoint1` (object, get/set) — Gets the first arc point.
- `ArcPoint2` (object, get/set) — Gets the second arc point.
- `ArcPoint3` (object, get/set) — Gets the third arc point.
- `Attributes` (object, get/set) — The curved dimension set radial attributes.
- `DimensionPoints` (object, get/set) — Gets a list of the dimension points.
- `Distance` (object, get/set) — Gets the distance. The distance is measured in paper millimeters.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## CurvedDimensionSetRadial.CurvedDimensionSetRadialAttributes (class)

The CurvedDimensionSetRadialAttributes class contains the attributes for managing all the attributes of the curved dimension with radial reference lines.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/93e176d8-7aa3-89da-3e36-7968e250ef86)

### Constructors
- `CurvedDimensionSetRadial.CurvedDimensionSetRadialAttributes(...)` — Creates a new curved dimension set radial attributes instance which loads standard attributes.
- `CurvedDimensionSetRadial.CurvedDimensionSetRadialAttributes(...)` — Creates a new curved dimension set radial attributes instance which loads standard attributes. If it set to true attributes won't be lodaded.
- `CurvedDimensionSetRadial.CurvedDimensionSetRadialAttributes(...)` — Creates a new curved dimension set radial attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/9f71819e-59e9-7575-c9ec-e020b97186bc)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/f22edb8d-d2ae-0aee-3a63-1acfb794bd69)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the dimension.
- `Color` (object, get/set) — The color of the dimension line.
- `CurvedDimensionType` (object, get/set) — The curved dimension type.
- `DimensionType` (object, get/set) — The dimension type.
- `Format` (object, get/set) — The format attributes of the dimension.
- `Placing` (object, get/set) — The placing attributes of the dimension.
- `Text` (object, get/set) — The text attributes of the dimension.
- `TransparentBackground` (object, get/set) — The dimension text background transparency. (transparent/opaque)
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the dimension line.

## CurvedSectionMark (class)

The CurvedSectionMark class defines a drawing object that illustrates a curved section in a specific view. In contrast to straight section mark, the curved section mark can only be inserted as a result of creating a curved section view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d2f96be0-9634-5832-1ef0-03cd0f8aa8dd)

### Methods
#### `Delete(...)`

Deletes the section mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/17865574-7d79-82df-c3a4-d568c75af535)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the section mark into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/00d180af-43bf-d119-0ca7-0324343b45cc)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c611857a-38a1-630f-474b-bad570dfa501)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the section mark in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/9b9a2de8-9c42-3170-e7fc-a25d5eeb89fd)

#### `Select(...)`

Selects the section mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/888e2a8c-21c6-7e2c-cac9-af4948c11008)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the section mark.
- `LeftPoint` (object, get/set) — The starting point of the left symbol of the section mark.
- `MiddlePoint` (object, get/set) — The middle arc point of a curved section mark.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `RightPoint` (object, get/set) — The starting point of the right symbol of the section mark.

## CustomLineType (class)

The CustomLineType class defines the custom line types of the drawing objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5029cf06-1d52-5391-2f65-1760cf3721bf)

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/03a890bb-7796-54c5-907f-51890915a07a)

#### `Equals(...)`

Returns true if the current object and the given object are equal.

[Docs](https://developer.tekla.com/topic/en/18/47/d0b0011c-d08d-40ac-fdfc-1c8ccd83d064)

#### `GetHashCode(...)`

Returns a hash code for the object.

[Docs](https://developer.tekla.com/topic/en/18/47/60bfd567-f4d8-1eb5-eece-d8d9e16be107)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/508fe885-1e18-ed11-81c5-ea7963144e85)

#### `ToString(...)`

Returns the LineTypes information as a string.

[Docs](https://developer.tekla.com/topic/en/18/47/33edfbad-ab7c-406a-fec4-5e932e3ea8aa)

### Properties
- `Description` (object, get/set) — The Description of the custom line type (ASCII representation of the line).
- `LineDescription` (object, get/set) — The description of the custom line type. Description uses the following format: List of line segments. Positive value gives length of filled line segment and negative value gives length of empty line segment. 0 means a dot.
- `Name` (object, get/set) — The Name of the custom line type.

## CustomLineTypeCatalog (class)

The CustomLineTypeCatalog provides accessors to fetch the possible custom line types of drawing objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b16ef3b5-7c9a-f06f-2fa7-e3d8bc687323)

### Methods
#### `Get(Int32)(...)`

Gets the CustomLineType with the specified ID.

[Docs](https://developer.tekla.com/topic/en/18/47/fa20f456-d445-b0cd-9c15-6329e9b23ecc)

#### `Get(String)(...)`

Gets the CustomLineType with the specified Name.

[Docs](https://developer.tekla.com/topic/en/18/47/67d83194-02c4-4ea0-60cf-a2c822e7299b)

#### `Get.(...)`

Gets all the available CustomLineType's.

[Docs](https://developer.tekla.com/topic/en/18/47/79c1407b-4793-aae6-b25d-78b4ce92167c)

## DPMPrinterAttributes (class)

The DPMPrinterAttributes class contains the attributes for controlling the printing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/898f2ba9-b17f-44cf-92b7-08949a658c8e)

### Constructors
- `DPMPrinterAttributes(...)` — Creates a new PrintAttributes instance.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/0474757e-8df3-02dd-72e3-de832c6ada64)

### Properties
- `ColorMode` (object, get/set) — Output color mode.
- `NumberOfCopies` (object, get/set) — The number of copies of each drawing to print. Only applicable for printing to a printer.
- `OpenFileWhenFinished` (object, get/set) — Whether to open file when finished.
- `Orientation` (object, get/set) — The print orientation type.
- `OutputFileName` (object, get/set) — Output file name and path. Applicable in PDF printing.
- `OutputType` (object, get/set) — The printing output type.
- `PaperSize` (object, get/set) — The paper size name.
- `PrinterName` (object, get/set) — The printer name.
- `PrintToMultipleSheet` (object, get/set) — Whether to print to multiple sheets.
- `ScaleFactor` (object, get/set) — The print scale factor.
- `ScalingMethod` (object, get/set) — The print scaling method.

## DatabaseObject (class)

The DatabaseObject class is a base class for all drawing objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/81125e32-9440-5ec1-cf53-ca6a7d0f84ea)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

### Properties
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## DetailMark (class)

The DetailMark class defines a drawing object that illustrates a detail in a specific view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/23ec1cfa-34c1-4ed5-2313-46518a3b1bed)

### Constructors
- `DetailMark(...)` — Creates a new detail mark instance with the given positions.
- `DetailMark(...)` — Creates a new detail mark instance with the given positions and attributes.

### Methods
#### `Delete(...)`

Deletes the detail mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/05a27feb-fd82-2314-ac98-da8663e7e7f1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the detail mark into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/d84d3293-9933-28f3-6fc2-fad038672d69)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/a22e5d37-ff49-f897-1744-4a05a5e53561)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the detail mark in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/4394bb26-fbac-ebe2-657a-eef9ec160d11)

#### `Select(...)`

Selects the detail mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/7ab7f3a5-4a07-0059-c816-48e7fea29a77)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the detail mark.
- `BoundaryPoint` (object, get/set) — The boundary point of the mark.
- `CenterPoint` (object, get/set) — The center point of the mark.
- `LabelPoint` (object, get/set) — The label point of the mark.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## DetailMark.DetailMarkAttributes (class)

The DetailMarkAttributes class is the attributes class for detail marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/59e6e3c9-3096-fa15-6c87-984312b000dd)

### Constructors
- `DetailMark.DetailMarkAttributes(...)` — Creates a new default DetailMarkAttributes instance that loads standard attributes.
- `DetailMark.DetailMarkAttributes(...)` — Creates a new DetailMarkAttributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/604b5a6c-d054-ecfe-1a32-06b0a8b7be63)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/7c3e3b03-d742-f29f-31f8-4f949e810d91)

### Properties
- `BoundaryLineColor` (object, get/set) — Obsolete.
- `BoundaryLineType` (object, get/set) — Obsolete.
- `BoundaryShape` (object, get/set) — The detail mark boundary shape.
- `BoundingLine` (object, get/set) — Gets or sets the line type.
- `MarkName` (object, get/set) — The detail mark name.
- `MarkSymbolAttributes` (object, get/set) — The detail mark symbol attributes.
- `MarkSymbolColor` (object, get/set) — 
- `TagsAttributes` (object, get/set) — The attributes for detail mark tags.
- `TrueMarkSymbolColor` (object, get/set) — The true color (rgb enabled) of the detail mark symbol.

## DetailMark.DetailMarkAttributes.DetailBoundaryShape (enum)

The boundary shapes of the detail mark.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/980abf7f-e806-2baf-41e7-ed51ce84ad3b)

### Values
- `None` = `1` — No shape.
- `Circular` = `2` — The circular shape.
- `Rectangular` = `3` — The rectangular shape.

## DetailMarkSymbolAttributes (class)

The DetailMarkSymbolAttributes class is for detail mark attributes related to the shape of the connecting symbol.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/82122653-7463-1d72-9991-af73d284f9f4)

### Constructors
- `DetailMarkSymbolAttributes(...)` — Initializes a new instance of the DetailMarkSymbolAttributes class

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c5232b59-b0cc-d2d6-6ba6-26c04856847f)

### Properties
- `Shape` (object, get/set) — The shape of the mark symbol.
- `Size` (object, get/set) — The size of the mark symbol.

## DetailMarkTagsAttributes (class)

The DetailMarkTagsAttributes class contains attributes for detail mark tags.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/82466df4-79d8-486e-cdbe-c58c38a329c1)

### Constructors
- `DetailMarkTagsAttributes(...)` — Creates a new mark tags attributes instance.
- `DetailMarkTagsAttributes(...)` — Creates a new mark tags attributes instance.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/14044383-871a-2de6-0df0-017a86ee81de)

### Properties
- `TagA1` (object, get/set) — The tag A1 of a mark.
- `TagA2` (object, get/set) — The tag A2 of a mark.
- `TagA3` (object, get/set) — The tag A3 of a mark.
- `TagA4` (object, get/set) — The tag A4 of a mark.
- `TagA5` (object, get/set) — The tag A5 of a mark.

## DimensionBase (class)

The DimensionBase class is the common base class for all the different dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/18ba1bb5-72bc-8166-7e10-836d1ec334a8)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDimensionSet(Boolean)(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/c6436216-db95-d799-f218-b48db2bb953e)

#### `GetDimensionSet.(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/8508b7b5-5f8f-c2e4-fe1f-4b4e391f3143)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## DimensionLink (class)

The DimensionLink class contains methods related to dimension links. A dimension link is a link between two dimension sets.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6cb5fe2c-46ef-fd0c-94b8-47fafe4473e8)

### Constructors
- `DimensionLink(...)` — Creates a new DimensionLink instance.

### Methods
#### `Delete(...)`

Unlinks the dimensions in the current drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/a10d3ec0-91e6-f06e-6295-b82991528039)

#### `GetDimension1(...)`

Gets the first one of the linked dimensions.

[Docs](https://developer.tekla.com/topic/en/18/47/8f040785-d546-466c-a3de-b1691bda702b)

#### `GetDimension2(...)`

Gets the second one of the linked dimensions.

[Docs](https://developer.tekla.com/topic/en/18/47/8d364e73-1940-37f0-5891-afcc5b93068d)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Links the dimensions in the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/50c5ca49-79d2-3b0f-4304-fa0497d5958b)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e7ed0f4c-d443-85a3-3b19-d307cf2aec21)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/e96ea46d-77f7-5525-c592-95f66d67d610)

#### `Select(...)`

Selects the drawing DimensionLink from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/2e399c35-c3f2-af2b-e441-a31ad54be5c2)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## DimensionSetBase (class)

The DimensionSetBase class is the common base class for all the different dimension sets. The dimension set base cannot be instantiated.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e0a486f0-f2b7-ee59-3589-ecaa5ae93e81)

### Methods
#### `AddToDimensionSet(...)`

Adds a dimension set to the current dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/c6ae7b87-49aa-8bb4-456a-61e0f37e1637)

#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjects(.Type.)(...)`

Gets the children straight dimensions of the current straight dimension set that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/0ec2481b-08c7-d5e3-41e4-291bfbb31bbc)

#### `GetObjects.(...)`

Gets the children straight dimensions of the current straight dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/a5b00ccb-a302-c9bd-b940-6b64af7bccdc)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The dimension set attributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## DimensionSetBaseAttributes (class)

The DimensionSetBaseAttributes class is the base class for all dimension set attributes. The class contains attributes that are common to all dimension sets.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c618d777-d2a2-5505-c866-dd12e531fa33)

### Constructors
- `DimensionSetBaseAttributes(...)` — Creates a new default dimension set base attributes instance that loads standard attributes.
- `DimensionSetBaseAttributes(...)` — Creates a new dimension set base attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/da11fa4f-a98d-c770-d0e2-f458bc3b82dc)

#### `LoadAttributes(...)`

Loads the attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/5597a575-45e2-aa83-6576-390f23af3e48)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the dimension.
- `Color` (object, get/set) — The color of the dimension line.
- `Format` (object, get/set) — The format attributes of the dimension.
- `Placing` (object, get/set) — The placing attributes of the dimension.
- `Text` (object, get/set) — The text attributes of the dimension.
- `TransparentBackground` (object, get/set) — The dimension text background transparency. (transparent/opaque)
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the dimension line.

## DimensionSetBaseAttributes.CombineFormats (enum)

The combined dimension formats.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/28b0d248-7c2b-9615-1140-d9f538025098)

### Values
- `Off` = `0` — No combining.
- `NumberOfItemsTimesLength` = `1` — The number of items times the dimension length. E.g. 3*60.
- `NumberOfItemsTimesLengthEqualsResult` = `2` — The number of items times the dimension length equals the result. E.g. 3*60=180.

## DimensionSetBaseAttributes.CombinedDimensionAttributes (class)

The CombinedDimensionAttributes class handles attributes related to the combination of identical parts of dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1f848894-eb94-d830-b612-4e021e78f6ed)

### Constructors
- `DimensionSetBaseAttributes.CombinedDimensionAttributes(...)` — Creates a new combined dimension attributes instance. The defaults combine a minimum of three identical dimensions to be shown as the number of items times the length of the dimension.
- `DimensionSetBaseAttributes.CombinedDimensionAttributes(...)` — Creates a new combined dimension attributes instance with the given parameters.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/08efea1e-c0ac-8c7c-65e6-b5a6448d8347)

### Properties
- `Format` (object, get/set) — The combined dimension format.
- `MinimumNumberToCombine` (object, get/set) — The minimum number of dimensions to be combined.

## DimensionSetBaseAttributes.CurvedDimensionTypes (enum)

The type of the curved dimension.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/39532c2f-a5f7-6501-f040-7d65feaf3235)

### Values
- `Distance` = `0` — The distance of the points is displayed.
- `Angle` = `1` — The angle is displayed.

## DimensionSetBaseAttributes.DimensionExaggerationAttributes (class)

The DimensionExaggerationAttributes class handles attributes related to the exaggeration of dimension lines.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ed104708-bf83-237c-9c78-d8dac70012af)

### Constructors
- `DimensionSetBaseAttributes.DimensionExaggerationAttributes(...)` — Creates a new dimension exaggeration attributes instance. The default is to have exaggeration disabled.
- `DimensionSetBaseAttributes.DimensionExaggerationAttributes(...)` — Creates a new dimension exaggeration attributes instance with the given parameters.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/835f5a19-1bf6-17fd-f6e5-d3605e638016)

### Properties
- `ExaggerationDirection` (object, get/set) — The ExaggerationDirection.
- `ExaggerationEnabled` (object, get/set) — Exaggeration is enabled if the value is true.
- `ExaggerationHeight` (object, get/set) — The ExaggerationHeight.
- `ExaggerationOrigin` (object, get/set) — The ExaggerationOrigin.
- `ExaggerationPosition` (object, get/set) — The ExaggerationPosition.
- `ExaggerationWidth` (object, get/set) — The ExaggerationWidth.

## DimensionSetBaseAttributes.DimensionFormatAttributes (class)

The DimensionFormatAttributes class controls the precision, format and unit of the dimension.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4d479d66-b47c-fc8f-6223-3abe21e7f513)

### Constructors
- `DimensionSetBaseAttributes.DimensionFormatAttributes(...)` — Creates a new dimension format attributes instance. By default the format rounds the dimension value to the nearest whole and shows it with no decimals. The unit is selected automatically.
- `DimensionSetBaseAttributes.DimensionFormatAttributes(...)` — Creates a new dimension format attributes instance with the given parameters.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/72852c3f-ae80-ead6-2076-fe9e89d03971)

### Properties
- `Format` (object, get/set) — The format of the dimension.
- `Precision` (object, get/set) — The precision of the dimension.
- `Unit` (object, get/set) — Gets or sets the unit of the dimension. Changing this attribute does not have any effect if the dimension measures an angle.
- `UseDigitGrouping` (object, get/set) — Gets or sets a value indicating whether digit grouping should be used or not.

## DimensionSetBaseAttributes.DimensionPlacingAttributes (class)

The DimensionPlacingAttributes class controls the placing of the dimension.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0c652346-d920-5590-e777-e1edf097c620)

### Constructors
- `DimensionSetBaseAttributes.DimensionPlacingAttributes(...)` — Creates a new dimension placing attributes instance. The placing is set to the free placing.
- `DimensionSetBaseAttributes.DimensionPlacingAttributes(...)` — Creates a new dimension placing attributes instance with the given parameters.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/986ea414-b9bc-87b6-0ae1-6992b84173e6)

### Properties
- `Direction` (object, get/set) — The placing direction.
- `Distance` (object, get/set) — The placing distance attributes.
- `Placing` (object, get/set) — The placing type.

## DimensionSetBaseAttributes.DimensionTextAttributes (class)

The DimensionTextAttributes class controls the dimension value representation.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/561bf2b6-be3f-4ceb-1ecc-e3423a4d8e4e)

### Constructors
- `DimensionSetBaseAttributes.DimensionTextAttributes(...)` — Creates a new dimension text attributes instance. By default the default font is used, no frame is drawn and the value text is above the dimension line.
- `DimensionSetBaseAttributes.DimensionTextAttributes(...)` — Creates a new dimension text attributes instance with the given parameters.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/992e4a4e-49c4-07b6-2ff5-f3206535255c)

### Properties
- `Font` (object, get/set) — The font attributes.
- `Frame` (object, get/set) — The frame type of the dimension value text.
- `TextPlacing` (object, get/set) — The place of the dimension value text.

## DimensionSetBaseAttributes.DimensionTextPlacings (enum)

The placing types of the dimension value text.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f1b82bd1-7225-0d37-9278-34fa42dd0fef)

### Values
- `AboveDimensionLine` = `0` — The dimension value text is placed above the dimension line.
- `OnDimensionLine` = `1` — The dimension value text is placed on the dimension line.

## DimensionSetBaseAttributes.DimensionTypes (enum)

The dimension types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e83fe43a-91d6-39aa-cd37-17afd7d57023)

### Values
- `Relative` = `1` — The point to point dimensions.
- `Absolute` = `2` — The dimensions from a common starting point.
- `RelativeAndAbsolute` = `3` — The combination of point to point and a common start point.
- `USAbsolute` = `4` — The dimensions from a common starting point, which includes a running dimension mark (RD).
- `USAbsolute2` = `16` — The dimensions from a common starting point, which includes a running dimension mark (RD), but short dimensions are changed to point to point dimensions.
- `AbsoluteWithShortRelatives` = `35` — The dimensions from a common starting point, but short dimensions are changed to point to point dimensions.
- `AbsoluteWithAllRelativesAbove` = `99` — The combination of point to point and a common start point, but the point to point dimensions are placed above the absolute.
- `Elevation` = `8` — Creates an elevation dimension at a picked point.

## DimensionSetBaseAttributes.DimensionValueFormats (enum)

The dimension value formats.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/45e98101-6ab6-c31e-1fc3-97f9cdc23128)

### Values
- `NoDecimals` = `0` — The value is with no decimals.
- `OneOptionalDecimal` = `1` — The value is with one decimal if the value is not whole.
- `OneDecimal` = `2` — The value is with one decimal.
- `TwoOptionalDecimals` = `3` — The value is with two decimals if the value is not whole.
- `TwoDecimals` = `4` — The value is with two decimals.
- `ThreeOptionalDecimals` = `5` — The value is with three decimals if the value is not whole.
- `ThreeDecimals` = `6` — The value is with three decimals.
- `RationalPart` = `7` — The value is with a rational part.
- `SuperscriptEnding` = `8` — The value is with superscript ending.

## DimensionSetBaseAttributes.DimensionValuePrecisions (enum)

The value precisions for the dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f40fd295-4398-7fdc-849b-41a1f75b545d)

### Values
- `Whole` = `1` — The value is rounded to the nearest whole (1).
- `OnePerTwo` = `2` — The value is rounded to the nearest half (1/2).
- `OnePerThree` = `3` — The value is rounded to the nearest third (1/3).
- `OnePerFour` = `4` — The value is rounded to the nearest fourth (1/4).
- `OnePerEight` = `8` — The value is rounded to the nearest eighth (1/8).
- `OnePerSixteen` = `16` — The value is rounded to the nearest sixteenth (1/16).
- `OnePerThirtytwo` = `32` — The value is rounded to the nearest thirtysecond (1/32).
- `OnePerTen` = `10` — The value is rounded to one decimal (1/10).
- `OnePerHundred` = `100` — The value is rounded to two decimals (1/100).
- `OnePerThousand` = `1000` — The value is rounded to three decimals (1/1000).
- `TwoAndHalfPerOne` = `-25` — Value is rounded to nearest 2.5mm
- `FivePerOne` = `-50` — Value is rounded to nearest 5mm
- `TenPerOne` = `-100` — Value is rounded to nearest 10mm

## DimensionSetBaseAttributes.DimensionValueUnits (enum)

The units usable with the dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5a830ac6-5c02-ae18-226d-39080e9b07f0)

### Values
- `Automatic` = `0` — The unit of the value is selected automatically.
- `Millimeter` = `1` — The value is shown in millimeters.
- `Centimeter` = `2` — The value is shown in centimeters.
- `Meter` = `3` — The value is shown in meters.
- `Inch` = `4` — The value is shown in inches.

## DimensionSetBaseAttributes.ExaggerationDirections (enum)

The direction of the exaggeration.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5e0ede78-e2b7-43c8-7d69-c7f8b7f8d112)

### Values
- `LeftDown` = `1` — Left / Down.
- `RightUp` = `2` — Right / Up.
- `Both` = `3` — Both Left/Down and Right / Up.

## DimensionSetBaseAttributes.ExaggerationOrigins (enum)

The origin of the exaggeration.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bd6376a4-a1fa-6119-c7fd-d82be4ed70c7)

### Values
- `NearDimensionLine` = `1` — Start the exaggeration close to the dimension line.
- `NearMeasuredObject` = `2` — Start the exaggeration as soon as possible as seen from the measured part.

## DimensionSetBaseAttributes.ExtensionLineTypes (enum)

The extension line types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8afb849b-02f7-669f-c75e-ae42040fd4cf)

### Values
- `No` = `0` — The extension lines are visible.
- `Yes` = `1` — The extension lines of a dimension are not visible.
- `NotOnGridlines` = `2` — The extension lines are not visible when the lines are on a grid.

## DimensionSetBaseAttributes.FrameTypes (enum)

The frame types around the dimension values.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/64bbf0bb-5fe1-97d6-6bd5-11eb46ce9a07)

### Values
- `None` = `0` — No frame.
- `Rectangle` = `1` — The frame is a rectangle.
- `Underline` = `2` — The frame is an underline.
- `RoundedRectangle` = `3` — The frame is a rectangle with rounded ends.
- `SharpenedRectangle` = `8` — The frame is a rectangle with sharpened ends.

## DimensionSetBaseAttributes.Placings (enum)

The placing types of the objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/666145f2-0e7a-4353-fff7-0b2204d43a2b)

### Values
- `Free` = `0` — The placing of the object is free.
- `Fixed` = `1` — The placing of the object is fixed.

## DimensionSetBaseAttributes.ShortDimensionTypes (enum)

The position of the short dimension value.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/270e753d-29d1-436c-87f3-b61c61e63d66)

### Values
- `Inside` = `0` — The dimension value is set between the dimension lines when the dimension is too short.
- `Outside` = `1` — The dimension value is moved outside of the dimension when the dimension is too short.

## DotPrintAreaType (enum)

The print area type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2b25de02-ff60-ace9-bb0e-0ad759056350)

### Values
- `EntireDrawing` = `0` — Print entire drawing.
- `VisibleArea` = `1` — Print visible area.

## DotPrintColor (enum)

Output color mode.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/edd92030-a090-d4ab-ee00-61bcb093e92b)

### Values
- `Color` = `0` — Color mode.
- `BlackAndWhite` = `1` — Black and white mode.
- `GreyScale` = `2` — Grey scale mode.

## DotPrintOrientationType (enum)

The print orientation type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d8bbec36-04d7-9dd9-d9f3-3e498c83d2be)

### Values
- `Auto` = `0` — Auto orientation.
- `Landscape` = `1` — Landscape orientation.
- `Portrait` = `2` — Portrait orientation.

## DotPrintOutputType (enum)

The printer output type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/46b40759-ff8d-bf63-02bb-28853dec8d85)

### Values
- `Printer` = `0` — Printer output type.
- `PDF` = `2` — PDF output type.
- `Plot` = `3` — Plot output type.
- `Image` = `4` — Image png output type.

## DotPrintPaperSize (enum)

The paper size.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0f3d2d1b-e790-9d33-4d98-0745acb9c270)

### Values
- `Auto` = `0` — Printer output type.
- `A0` = `1` — A0 paper size.
- `A1` = `2` — A1 paper size.
- `A2` = `3` — A2 paper size.
- `A3` = `4` — A3 paper size.
- `A4` = `5` — A4 paper size.
- `A5` = `6` — A5 paper size.

## DotPrintScalingType (enum)

The print scaling type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c4113e81-eef4-b513-2830-ad91a6cd6df4)

### Values
- `Auto` = `0` — Auto scale.
- `Scale` = `1` — Use defined scale.

## DotPrintToMultipleSheet (enum)

The print to multiple sheet type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2937328b-0757-f390-dd0a-dfc66159f212)

### Values
- `Off` = `0` — Off.
- `LeftToRightAndTopToBottom` = `1` — Left to right and top to bottom.
- `BottomToTopAndRightToLeft` = `2` — Bottom to top and right to left.

## Drawing (class)

The Drawing class represents a drawing in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7b690fe6-8004-2c5d-aa07-f60af8ea16f8)

### Methods
#### `CommitChanges(String)(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/92109855-5313-7446-bef4-ab6dde6875f1)

#### `CommitChanges.(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/bc26a611-bd6a-535a-4461-943b8bf5351c)

#### `Delete(...)`

Deletes the drawing. An active drawing cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/4bcff604-27c4-487f-bb7d-5d80a2d87d8b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetIdsOfUnnumberedDrawings(...)`

Returns the list of identifiers of those drawings in the input that are NOT up to date regarding numbering in the drawing

[Docs](https://developer.tekla.com/topic/en/18/47/bc5c0f50-d50e-92ca-83ef-390151680205)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetPlotFileName(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/fa70baa3-ad78-2e3c-88ca-0b77cb81cd21)

#### `GetPlotFileNameExt(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/6288f4e4-4baf-656d-57bb-c63dee354493)

#### `GetSheet(...)`

Gets the sheet of the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fe921886-00b9-57d3-0488-92bd25014c66)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/6b1b3008-c9dd-5b97-0881-89ccc580b3e7)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `PlaceViews(...)`

Places views on the sheet. Computes the required size of the views to fit the parts, then places the views so that they fit on the sheet and do not overlap. Adjusts the sheet size if needed.

[Docs](https://developer.tekla.com/topic/en/18/47/1e1578e4-084f-21a9-3d1a-2b03140440ce)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

### Properties
- `CommitMessage` (object, get/set) — Gets the commit message
- `CreationDate` (object, get/set) — The drawing creation date.
- `DrawingTypeStr` (object, get/set) — The type of the drawing
- `IsFrozen` (object, get/set) — Describes whether the drawing is frozen.
- `IsIssued` (object, get/set) — Describes whether the drawing is issued.
- `IsIssuedButModified` (object, get/set) — Describes whether the drawing is issued but modified since it was issued.
- `IsLocked` (object, get/set) — Describes whether the drawing is locked.
- `IsLockedBy` (object, get/set) — Gets the logged in username that set the drawing to locked
- `IsMasterDrawing` (object, get/set) — Describes whether the drawing is a master drawing (shown with a special symbol on the drawing list).
- `IsReadyForIssue` (object, get/set) — Describes whether the drawing is ready for issue.
- `IsReadyForIssueBy` (object, get/set) — Gets the logged in username that set the drawing Ready for issue
- `IssuingDate` (object, get/set) — The drawing issuing date.
- `Layout` (object, get/set) — Gets the drawing layout attributes.
- `Mark` (object, get/set) — The drawing mark.
- `ModificationDate` (object, get/set) — The drawing modification date.
- `Name` (object, get/set) — The name of the drawing.
- `OutputDate` (object, get/set) — The drawing output date. XS_DISABLE_DRAWING_PLOT_DATE controls whether this date is set.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SectionViewStartLabel` (object, get/set) — Gets the drawing attribute for start label of section view
- `Title1` (object, get/set) — The first drawing title.
- `Title2` (object, get/set) — The second drawing title.
- `Title3` (object, get/set) — The third drawing title.
- `UpToDateStatus` (object, get/set) — Gets the drawing up to date status.

## DrawingColors (enum)

The colors used in Tekla.Structures.Drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/51916278-1622-9c21-d757-f9f8392746e9)

### Values
- `Invisible` = `152` — The light gray color: the "invisible color". Light gray is visible only on the screen and not in printed or exported drawings.
- `Black` = `153` — The black color.
- `NewLine1` = `154` — The brown color.
- `NewLine2` = `155` — The green color.
- `NewLine3` = `156` — The dark blue color.
- `NewLine4` = `157` — The forest green color.
- `NewLine5` = `158` — The orange color.
- `NewLine6` = `159` — The gray color.
- `Red` = `160` — The red color.
- `Green` = `161` — The bright green color.
- `Blue` = `162` — The blue color.
- `Cyan` = `163` — The turquoise color.
- `Yellow` = `164` — The yellow color.
- `Magenta` = `165` — The magenta color.
- `Gray30` = `130` — The Gray30 rgb(0.3, 0.3, 0.3) color.
- `Gray50` = `131` — The Gray50 rgb(0.5, 0.5, 0.5) color.
- `Gray70` = `132` — The Gray70 rgb(0.7, 0.7, 0.7) color.
- `Gray90` = `133` — The Gray90 rgb(0.9, 0.9, 0.9) color.

## DrawingEnumerator (class)

The DrawingEnumerator class provides the means to iterate through drawing instances. Drawing enumerators are generated by the drawing handler.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8c1e808a-2926-44d1-6bc3-cfb442a77ea6)

### Methods
#### `GetEnumerator(...)`

Allows the usage of the foreach statement with DrawingObjectEnumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/6d49ab9f-b229-3e1a-f471-83fe41ddd3aa)

#### `GetSize(...)`

Returns the total amout of items.

[Docs](https://developer.tekla.com/topic/en/18/47/1b1d3fb7-f3cb-3fbe-97f7-19fc5d437b48)

#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/4f07c4bf-0de8-7040-8a2a-883fb0b8db66)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/8b7ad0a7-646c-0763-6b98-9c49984e0d55)

### Properties
- `Current` (object, get/set) — Gets the current drawing.

## DrawingEnumeratorBase (class)

The DrawingEnumeratorBase class is a base class for DrawingObjectEnumerator and DrawingEnumerator.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/965d8de8-7450-4374-7000-018074df3b54)

### Methods
#### `GetEnumerator(...)`

Allows the usage of the foreach statement with DrawingObjectEnumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/6d49ab9f-b229-3e1a-f471-83fe41ddd3aa)

#### `GetSize(...)`

Returns the total amout of items.

[Docs](https://developer.tekla.com/topic/en/18/47/1b1d3fb7-f3cb-3fbe-97f7-19fc5d437b48)

#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/4f07c4bf-0de8-7040-8a2a-883fb0b8db66)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/8b7ad0a7-646c-0763-6b98-9c49984e0d55)

### Properties
- `AutoFetch` (object, get/set) — Indicates that the objects are fetched from the drawing when the enumerator is created. Object information is therefore not anymore fetched when 'Current' item is asked from the enumerator. Property value is used for all enumerators in application

## DrawingHandler (class)

The DrawingHandler class initializes the interface from a .NET application to Tekla Structures. This object must be created before any actions can be performed on Tekla Structures drawings. When this object is created, it is possible to ask the current active drawing in Tekla Structures, get a list of drawings or create a new drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/99085c56-78ab-d559-3ae6-e0cae84b3742)

### Constructors
- `DrawingHandler(...)` — Creates a new drawing handler instance, a "handle" to Tekla.Structures.Drawing. When a drawing handler object is created, it is possible to ask the current active drawing in Tekla Structures, get a list of drawings or create a new drawing.

### Methods
#### `CloseActiveDrawing(Boolean)(...)`

Closes the drawing editor.

[Docs](https://developer.tekla.com/topic/en/18/47/3b137222-7773-704a-d875-fbdbb1257ca6)

#### `CloseActiveDrawing.(...)`

Closes the drawing editor.

[Docs](https://developer.tekla.com/topic/en/18/47/28a51c2a-f66c-7516-857a-c28c7e8c0737)

#### `GetActiveDrawing(...)`

Returns an instance of the active drawing that is currently open in Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/b81bdfd4-7251-6dd2-3220-840143e6cbbf)

#### `GetConnectionStatus(...)`

Returns true if a proper connection to the Tekla Structures process has been established. A proper connection to Tekla Stuctures in the drawing API needs Tekla Structures up and running and a model open. Currently, there's no way to re-establish the connection.

[Docs](https://developer.tekla.com/topic/en/18/47/0f81f45b-d947-0af9-7dcd-5f808f6b77a1)

#### `GetDrawingObjectSelector(...)`

Gets a drawing object selector. With a drawing object selector drawing objects can be selected and highlighted in the drawing editor. A drawing object selector also provides a list of currently selected drawing objects.

[Docs](https://developer.tekla.com/topic/en/18/47/697aadf2-9bb6-4f35-f6a7-ad6f07afb121)

#### `GetDrawingSelector(...)`

Gets a drawing selector. With a drawing selector the list of selected drawings can be gotten.

[Docs](https://developer.tekla.com/topic/en/18/47/0d2838dc-fc44-9ca6-1160-9199c9a61c86)

#### `GetDrawings(...)`

Gets all the drawings from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/168596db-1134-c430-5061-c5e7bfd74137)

#### `GetMessageExecutionStatus(...)`

Gets the current message execution mode for the application.

[Docs](https://developer.tekla.com/topic/en/18/47/98f090d6-61bf-b7c3-e882-8ce74bfb4e94)

#### `GetModelObjectIdentifiers(...)`

Gets model object identifiers of the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/10273ee5-31be-5058-48b0-b54a684f60e7)

#### `GetPicker(...)`

Gets a picker for picking points and objects in a drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/4f0314ad-dfb2-e486-4936-0fb4e680e628)

#### `IsAnyDrawingOpen(...)`

Returns information if any drawing is open.

[Docs](https://developer.tekla.com/topic/en/18/47/bc85671d-e8ca-5537-83a4-251f188816e0)

#### `IssueDrawing(...)`

Issues the drawing if the drawing is not issued or was previously issued but has been modified since. This is the same as pressing Issue on the drawing list for a selected drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/c3d672fd-1551-264f-6ef0-845fb7c0ed8f)

#### `PrintDrawing(Drawing, DPMPrinterAttributes)(...)`

Prints the drawing using the given printer attributes.

[Docs](https://developer.tekla.com/topic/en/18/47/794a682c-1c9e-09e2-61a9-a190a3c6a7a0)

#### `PrintDrawing(Drawing, DPMPrinterAttributes, String)(...)`

Prints the drawing to file using the given printer attributes and name of the output file. The given output file overrides the printer attributes file settings.

[Docs](https://developer.tekla.com/topic/en/18/47/9892c982-a3a0-7c7f-be45-d0c0ecf5e011)

#### `PrintDrawing(Drawing, PrintAttributes)(...)`

Prints the drawing using the given printer instance. NOTE! The drawing cannot be active, otherwise printing fails.

[Docs](https://developer.tekla.com/topic/en/18/47/6d90ef48-e76c-7b90-97ea-f0f0305f356f)

#### `PrintDrawing(Drawing, PrintAttributes, String)(...)`

Prints the drawing to file using the given printer instance. NOTE! The drawing cannot be active, otherwise printing fails.

[Docs](https://developer.tekla.com/topic/en/18/47/8769190b-de6e-2268-846a-3fe3ccbfc6f1)

#### `PrintDrawings(IEnumerable.Drawing., DPMPrinterAttributes, String)(...)`

Prints the list of drawings using the given printer attributes.

[Docs](https://developer.tekla.com/topic/en/18/47/18cfb3c1-0804-3486-ae62-da6e8a3c357e)

#### `PrintDrawings(List.Drawing., DPMPrinterAttributes)(...)`

Prints the list of drawings using the given printer attributes.

[Docs](https://developer.tekla.com/topic/en/18/47/8bbf61cb-24e0-da11-a4ba-e187133b4c55)

#### `SaveActiveDrawing(...)`

Saves the currently open drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/22861cc2-b284-efde-02ff-824947336e92)

#### `SetActiveDrawing(Drawing)(...)`

Sets the active drawing that is currently open in Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/3dab6952-0b6b-9b3a-9d77-48c406290215)

#### `SetActiveDrawing(Drawing, Boolean)(...)`

Sets the active drawing that is currently open in Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/11e9c711-1f1d-a057-d26b-abec310ddafe)

#### `SetActiveDrawing(Drawing, Boolean, Boolean)(...)`

Sets the active drawing that is currently open in Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/106a2ced-348c-3323-21f7-61ffff722d8c)

#### `SetMessageExecutionStatus(...)`

Sets the message execution mode for the application.

[Docs](https://developer.tekla.com/topic/en/18/47/588ddcdb-534e-5cd4-3069-1cdb2f8a20f9)

#### `UnissueDrawing(...)`

Unissues the drawing. This is the same as pressing Unissue on the drawing list for a selected drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fa82a99e-ce89-e906-6d43-28e189ed48d3)

#### `UpdateDrawing(...)`

Updates the drawing. This is the same as pressing Update on the drawing list for a selected drawing. NOTE! The drawing cannot be active, otherwise the operation fails. NOTE! Numbering must be executed before this operation.

[Docs](https://developer.tekla.com/topic/en/18/47/e6815606-40e5-189f-d6c1-5192dc16f9ad)

## DrawingHandler.MessageExecutionModeEnum (enum)

The message execution modes of Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d857408b-12df-e503-3481-cbed0e9cc540)

### Values
- `INSTANT` = `0` — Executes messages after each database operation. All objects appear instantly on the screen when using the INSTANT mode.
- `BY_COMMIT` = `1` — Executes messages only when Drawing.CommitChanges() is called.

## DrawingHatchColors (enum)

The hatch colors used in Tekla.Structures.Drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e2841abe-9fbd-c6ae-74c1-c88b193cd767)

### Values
- `Invisible` = `152` — The light gray color: the "invisible color". Light gray is visible only on the screen and not in printed or exported drawings.
- `Black` = `153` — The black color.
- `NewLine1` = `154` — The brown color.
- `NewLine2` = `155` — The green color.
- `NewLine3` = `156` — The dark blue color.
- `NewLine4` = `157` — The forest green color.
- `NewLine5` = `158` — The orange color.
- `NewLine6` = `159` — The gray color.
- `Red` = `160` — The red color.
- `Green` = `161` — The bright green color.
- `Blue` = `162` — The blue color.
- `Cyan` = `163` — The turquoise color.
- `Yellow` = `164` — The yellow color.
- `Magenta` = `165` — The magenta color.
- `Special` = `120` — Obsolete. The special color (deprecated). Defined with advanced options XS_HATCH_SPECIAL_COLOR_R, XS_HATCH_SPECIAL_COLOR_G and XS_HATCH_SPECIAL_COLOR_B.
- `Gray30` = `130` — The Gray30 rgb(0.3, 0.3, 0.3) color.
- `Gray50` = `131` — The Gray50 rgb(0.5, 0.5, 0.5) color.
- `Gray70` = `132` — The Gray70 rgb(0.7, 0.7, 0.7) color.
- `Gray90` = `133` — The Gray90 rgb(0.9, 0.9, 0.9) color.

## DrawingLink (class)

The DrawingLink class defines a drawing link object in a drawing. When a drawing link is double clicked, the target drawing is opened.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d0fdf906-a890-dba2-03a8-7209a743f5ea)

### Constructors
- `DrawingLink(...)` — Creates a new drawing link instance using standard attributes.
- `DrawingLink(...)` — Creates a new drawing link instance using the given attributes.
- `DrawingLink(...)` — Creates a new drawing link instance of the given size.
- `DrawingLink(...)` — Creates a new drawing link instance of the given size.

### Methods
#### `Delete(...)`

Deletes the link.

[Docs](https://developer.tekla.com/topic/en/18/47/707bfed6-0ca4-cee2-cab5-335cb191a0fb)

#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/992fb6af-b657-ed89-b5bf-9a8f8854710a)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/bec56125-ac08-53b1-3304-39ba38dd409a)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the link.

[Docs](https://developer.tekla.com/topic/en/18/47/8dfe8f76-a35f-e6d0-b9d5-a0addf3582cd)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/bf6353c0-0cb2-e54f-920d-91dfd8607f03)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the link.

[Docs](https://developer.tekla.com/topic/en/18/47/e93780fd-6262-8155-e032-0d203b99ce58)

#### `MoveObjectRelative(...)`

Moves the object according to the given move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/504de256-3dc9-1f96-6eae-9baf92e5371d)

#### `Resize(...)`

Resizes the object. The scaling type is changed to ScaleToFit.

[Docs](https://developer.tekla.com/topic/en/18/47/9e3a70e8-d294-bdb3-b272-aee8ea698090)

#### `Select(...)`

Selects the link.

[Docs](https://developer.tekla.com/topic/en/18/47/b6b8e7b1-2c5b-9458-8809-492f3e05379d)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle of the link.
- `Attributes` (object, get/set) — Gets or sets the link attributes.
- `InsertionPoint` (object, get/set) — Gets or sets the insertion point of the link.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Target` (object, get/set) — Gets or sets the target drawing.
- `Text` (object, get/set) — Gets or sets the link text that is visible on the screen.

## DrawingObject (class)

The DrawingObject class is an abstract base class for all the objects in the drawing. All drawing objects have a database identifier and belong to some view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2056df24-dbe8-2cf2-1342-1a13dc0ccacd)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## DrawingObject.NoAttributes (class)

The NoAttributes class defines attributes for objects that do not contain any attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7227e500-8f42-5762-0143-d3eaf00e8c77)

### Constructors
- `DrawingObject.NoAttributes(...)` — Creates a new default no attributes instance.
- `DrawingObject.NoAttributes(...)` — Creates a new no attributes instance. No attributes are loaded from the attribute file as it is not possible to load attributes for objects of this type.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/8cab66ba-5115-4e23-78e2-6e663ff74211)

#### `LoadAttributes(...)`

It is not possible to load attributes for objects of this type because the objects have no attributes.

[Docs](https://developer.tekla.com/topic/en/18/47/6c1c9d6b-ed8f-13d0-5f6d-6564881fc82c)

## DrawingObjectEnumerator (class)

The DrawingObjectEnumerator class provides the means to iterate through drawing object instances. Drawing object enumerators are generated by the container view and they contain drawing objects that are children of the container view instance.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/98a25b0f-b809-6c62-1499-f5ebc6cfc72a)

### Methods
#### `GetEnumerator(...)`

Allows the usage of the foreach statement with DrawingObjectEnumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/6d49ab9f-b229-3e1a-f471-83fe41ddd3aa)

#### `GetSize(...)`

Returns the total amout of items.

[Docs](https://developer.tekla.com/topic/en/18/47/1b1d3fb7-f3cb-3fbe-97f7-19fc5d437b48)

#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/4f07c4bf-0de8-7040-8a2a-883fb0b8db66)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/8b7ad0a7-646c-0763-6b98-9c49984e0d55)

### Properties
- `Current` (object, get/set) — Gets the current drawing object.

## DrawingUpToDateStatus (enum)

The drawing up to date status.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f3ce4eac-1589-801a-b1e2-5009757ab10f)

### Values
- `DrawingIsUpToDate` = `1` — The drawing is up to date.
- `PartsWereModified` = `2` — Some parts in the drawing were modified since the last update.
- `DrawingIsUpToDateButMayNeedChecking` = `4` — The drawing is up to date but may need checking.
- `IncludedDrawingModified` = `5` — The included drawing was modified.
- `OriginalPartDeleted` = `6` — The original part from which the drawing was created was deleted since the last update.
- `AllPartsDeleted` = `7` — All parts were deleted.
- `NumberOfPartsInNumberingSeriesIncreased` = `8` — The number of parts in the numbering series has increased since the last update.
- `NumberOfPartsInNumberingSeriesDecreased` = `9` — The number of parts in the numbering series has decreased since the last update.
- `DrawingWasCloned` = `10` — The drawing was cloned from another drawing.
- `DrawingWasUpdated` = `11` — The drawing was updated.
- `CopiedViewChanged` = `12` — The copied view changed.
- `DrawingWasSplitted` = `13` — The drawing was splitted (cloned) during numbering.
- `MovedViewDeleted` = `14` — The moved view was deleted.
- `MovedViewLabelChanged` = `15` — The moved view symbol has changed.
- `DrawingWasClonedFromCloud` = `16` — The drawing was cloned from another drawing in cloud service.

## DwgObject (class)

The DwgObject class defines a DWG or DXF object that can be inserted into a drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/13e76fb2-edc3-613b-0827-af240fe12df6)

### Constructors
- `DwgObject(...)` — Creates a new DwgObject instance with the given parameters.
- `DwgObject(...)` — Creates a new DwgObject instance using the specified attributes.
- `DwgObject(...)` — Creates a new DwgObject instance with the given parameters.
- `DwgObject(...)` — Creates a new DwgObject instance using the specified attributes.

### Methods
#### `Delete(...)`

Deletes the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/5dc2f662-fa9f-431d-d6c4-c8b9fd7cb063)

#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/92532071-8507-973e-5e7c-303633870edc)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/2057db75-e1b4-d5cc-5ba4-5849af223043)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/faa4a0d5-3e63-d983-f5a2-625481575fb4)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/75685c67-29ad-955a-15b8-38e2558bdd89)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/4dc862b3-4968-e29c-83bf-a84a18c47e84)

#### `MoveObjectRelative(...)`

Moves the object according to the given move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/96087af0-e3e1-88dc-7d10-2aa68bcfe8d8)

#### `Resize(...)`

Resizes the object. The scaling type is changed to ScaleToFit.

[Docs](https://developer.tekla.com/topic/en/18/47/58cefb6f-d78a-089e-e16e-8dc0bdf1bf09)

#### `Select(...)`

Selects the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/fc0aa530-1e2b-0062-db7b-21f4ff2306d1)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle (in degrees).
- `Attributes` (object, get/set) — Gets or sets the attributes.
- `FileName` (object, get/set) — Gets or sets the file name defining the embedded object.
- `InsertionPoint` (object, get/set) — Gets or sets the insertion point.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Size` (object, get/set) — Gets or sets the size of the frame of the embedded object.

## EdgeChamfer (class)

The EdgeChamfer class contains methods related to edge chamfers. An edge chamfer is a drawing object derived from a model object. It represents a drawing chamfer that has a reference to the actual chamfer in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f7f0a579-7515-055a-d571-f5450caf7d20)

### Methods
#### `Delete(...)`

Deletes the drawing chamfer from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/22dd240e-6db8-71ff-602a-f559d4f878e6)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/ba732712-c7a1-9206-1fb3-edf45cd11541)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/7eed5c19-b890-7ad5-9fb6-dcf7b0bb1527)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing chamfer.

[Docs](https://developer.tekla.com/topic/en/18/47/e6250bdf-3ca8-97cc-e7b1-ef43c8a6f3a7)

#### `Select(...)`

Selects the drawing chamfer from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/785e823c-b45f-a48d-0be0-f71ee35fa88c)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the edge chamfer. For more information see EdgeChamfer.EdgeChamferAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## EdgeChamfer.EdgeChamferAttributes (class)

The EdgeChamferAttributes class is the attributes class for the edge chamfer.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/21329c5d-35e7-6835-32de-03df1084b0a0)

### Constructors
- `EdgeChamfer.EdgeChamferAttributes(...)` — Creates a new default edge chamfer attributes instance that loads standard attributes.
- `EdgeChamfer.EdgeChamferAttributes(...)` — Creates a new edge chamfer attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/0788720f-031d-788b-92e8-a85d95a2701c)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/96a9aab9-8841-803d-8bc5-7c0688350dff)

### Properties
- `VisibleLines` (object, get/set) — The line type of the chamfer.

## ElementBase (class)

The ElementBase class is the abstract base class for all mark elements.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/912b7474-052f-6236-b82b-fee837c57e62)

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/020dbce5-3810-0ebb-8cc3-66b14d32d46c)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/813c9266-4b18-84d8-87fd-205f9178ec3e)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/85182d02-6b31-f730-dfbb-ee91936b7d42)

## EmbeddedObjectAttributes (class)

The EmbeddedObjectAttributes class contains attributes for embedded objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/596ae9f6-fcde-e827-a806-61f33418be18)

### Constructors
- `EmbeddedObjectAttributes(...)` — Initializes a new instance of the EmbeddedObjectAttributes class that loads standard attributes.
- `EmbeddedObjectAttributes(...)` — Initializes a new instance of the EmbeddedObjectAttributes class that loads given attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/f8df975c-3c1c-23b9-96cb-2bfe8ae31ac6)

#### `LoadAttributes(...)`

Loads the specified attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/27635ad9-d4d6-ad93-a827-16cb3992936c)

### Properties
- `Frame` (object, get/set) — Gets or sets the frame.
- `Scaling` (object, get/set) — Gets or sets the scaling type.
- `XScale` (object, get/set) — Gets or sets the X scaling value. Changing this property will set the scaling type to ScaleXY.
- `YScale` (object, get/set) — Gets or sets the Y scaling value. Changing this property will set the scaling type to ScaleXY.

## EmbeddedObjectBase (class)

The EmbeddedObjectBase class defines an embedded object in a drawing. The object refers to another file in the file system and its drawing representation is updated when a drawing is opened.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/06a56769-d7c5-f846-e1fd-eca7788d10ca)

### Methods
#### `Delete(...)`

Deletes the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/5dc2f662-fa9f-431d-d6c4-c8b9fd7cb063)

#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/92532071-8507-973e-5e7c-303633870edc)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/2057db75-e1b4-d5cc-5ba4-5849af223043)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/faa4a0d5-3e63-d983-f5a2-625481575fb4)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/75685c67-29ad-955a-15b8-38e2558bdd89)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/4dc862b3-4968-e29c-83bf-a84a18c47e84)

#### `MoveObjectRelative(...)`

Moves the object according to the given move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/96087af0-e3e1-88dc-7d10-2aa68bcfe8d8)

#### `Resize(...)`

Resizes the object. The scaling type is changed to ScaleToFit.

[Docs](https://developer.tekla.com/topic/en/18/47/58cefb6f-d78a-089e-e16e-8dc0bdf1bf09)

#### `Select(...)`

Selects the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/fc0aa530-1e2b-0062-db7b-21f4ff2306d1)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle (in degrees).
- `Attributes` (object, get/set) — Gets or sets the attributes.
- `FileName` (object, get/set) — Gets or sets the file name defining the embedded object.
- `InsertionPoint` (object, get/set) — Gets or sets the insertion point.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Size` (object, get/set) — Gets or sets the size of the frame of the embedded object.

## EmbeddedObjectFrame (class)

The EmbeddedObjectFrame class defines the frame object for embedded objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/69279c2a-ae72-e8eb-62dc-ad7dbe3cc0d3)

### Constructors
- `EmbeddedObjectFrame(...)` — Creates a new embedded object frame instance with the specified line type and color.
- `EmbeddedObjectFrame(...)` — Creates a new embedded object frame instance with the specified line type and user defined RGB color.

### Methods
#### `GetAxisAlignedBoundingBox(...)`

Returns the bounding box of the frame in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/3d004ed7-9902-c6c8-6805-4c056268a763)

#### `GetObjectAlignedBoundingBox(...)`

Returns the bounding box of the frame in the local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/c896a504-9410-e2c9-bbda-c8d671479c0b)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/1182e099-00d7-f716-55c6-ebdb3397cbba)

### Properties
- `Color` (object, get/set) — Obsolete.
- `Line` (object, get/set) — Gets or sets the line type of the frame.
- `LineType` (object, get/set) — Obsolete.
- `Type` (object, get/set) — Gets the frame type (rectangular).

## EmbeddedObjectScalingOptions (enum)

The options for controlling the scaling of the objects in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/795b7d71-f3a0-1d23-342f-a65be670084e)

### Values
- `ScaleX` = `1` — Scales the object in the X direction.
- `ScaleXY` = `3` — Scales the object in the X and Y directions.
- `ScaleToFit` = `4` — Scales the object to fit in the frame in the Y direction.
- `ScaleBestFit` = `8` — The object is scaled inside the frame, still maintaining its original aspect ratio.

## Events (class)

The Events class allows the user to register event listeners for drawing events.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/70b96e37-c90a-f11f-1554-a4bf7c4147e4)

### Constructors
- `Events(...)` — Constructs a new instance of Events.

### Methods
#### `Dispose(...)`

Disposes of the Events instance.

[Docs](https://developer.tekla.com/topic/en/18/47/4d79d261-bcf1-d034-f6d8-6c413402a2a8)

#### `InitializeLifetimeService(...)`

Initializes the lifetime service.

[Docs](https://developer.tekla.com/topic/en/18/47/d68de3c6-70b9-635e-aeef-074e2c0ee453)

#### `Register(...)`

Registers the instance to listen to the specified events.

[Docs](https://developer.tekla.com/topic/en/18/47/b196e5b4-cac8-a0dd-94f3-2f351076174a)

#### `UnRegister(...)`

Unregisters the instance from listening to events.

[Docs](https://developer.tekla.com/topic/en/18/47/aaab4b70-a62a-2f51-bca6-3926ab0b156b)

### Events
#### `DrawingChanged` (`EventHandler`)

**Signature:** `event EventHandler DrawingChanged`

The DrawingChanged event is raised when the drawing has been changed and the database is committed.

[Docs](https://developer.tekla.com/topic/en/18/47/5fbe8494-e106-fdcc-720a-a513e1d87046)

#### `DrawingDeleted` (`EventHandler`)

**Signature:** `event EventHandler DrawingDeleted`

The DrawingDeleted event is raised when a drawing has been deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/3a3213f5-aa9f-9c69-5263-4d36fb12851b)

#### `DrawingInserted` (`EventHandler`)

**Signature:** `event EventHandler DrawingInserted`

The DrawingInserted event is raised when a drawing has been inserted.

[Docs](https://developer.tekla.com/topic/en/18/47/735b2e7d-95df-fb8a-a7aa-ce30800afa0b)

#### `DrawingReadyForIssuingChange` (`EventHandler`)

**Signature:** `event EventHandler DrawingReadyForIssuingChange`

The DrawingReadyForIssuingChange event is raised just after a drawing is marked or unmarked ready for issuing.

[Docs](https://developer.tekla.com/topic/en/18/47/ea59f224-4b9c-d9b2-ecb9-3e333a3a6cbe)

#### `DrawingStatusChanged` (`EventHandler`)

**Signature:** `event EventHandler DrawingStatusChanged`

The DrawingStatusChanged event is raised when the drawing status has changed.

[Docs](https://developer.tekla.com/topic/en/18/47/9006d864-225b-0144-4e0e-88c243fe1224)

#### `DrawingUpdated` (`EventHandler`)

**Signature:** `event EventHandler DrawingUpdated`

The DrawingUpdated event is raised when the drawing has been inserted, deleted or modified, and the database is committed.

[Docs](https://developer.tekla.com/topic/en/18/47/7b3864f6-9c92-e5d7-e611-eadcc58ae121)

## Events.DrawingDeletedDelegate (delegate)

The delegate to use for drawing deletion.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/435456fd-f193-48af-e90d-ae45600e2388)

## Events.DrawingInsertedDelegate (delegate)

The delegate to use for drawing insertion.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d133bbe2-02e5-c3b6-2847-be49f0c1d10f)

## Events.DrawingReadyForIssuingChangeDelegate (delegate)

The delegate to use for drawing ready for issuing change event.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5ca7a195-665c-d788-ce74-a36f21063a7f)

## Events.DrawingStatusChangedDelegate (delegate)

The delegate to use for drawing status change.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f6dd9476-1c9e-539b-f889-af972c6b43c1)

## Events.DrawingUpdateTypeEnum (enum)

The types of drawing update event.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/91548dcb-f7f5-b43e-fbf8-4040e94bf6d9)

### Values
- `INSERTED` = `0` — Drawing created
- `MODIFIED` = `1` — Drawing modified
- `DELETED` = `2` — Drawing deleted

## Events.DrawingUpdatedDelegate (delegate)

The delegate to use for drawing updated (insertion, deletion or modification).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/354f8066-5b51-c720-b519-1439296c0375)

## FontAttributes (class)

The FontAttributes class contains the basic attributes for fonts.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f0ae6595-19c8-88da-371c-e104b7d07388)

### Constructors
- `FontAttributes(...)` — Creates a new font attributes instance. The default color is black, the height is 2.5, the font name is Arial and the values of italic and bold are false.
- `FontAttributes(...)` — Creates a new font attributes instance with the given parameters.
- `FontAttributes(...)` — Creates a new font attributes instance with the given parameters.

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/8b7bcbf8-c379-13d6-9c02-0043bde2fbc4)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e240af18-28a0-7eba-1bff-de03c0f16c47)

### Properties
- `Bold` (object, get/set) — Whether the font is bold.
- `Color` (object, get/set) — The color of the font.
- `Height` (object, get/set) — The height of the font (in millimeters).
- `Italic` (object, get/set) — Whether the font is italic.
- `Name` (object, get/set) — The name of the font.
- `TrueColor` (object, get/set) — The true color (rgb enabled) of the font.

## FormatTypes (enum)

The available format types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ce6c34a2-171f-aad5-b117-917f18799552)

### Values
- `Automatic` = `0` — Automatic, chooses the most appropriate format for the current environment.
- `WholeNumber` = `1` — As shown on the dialog: ###. This format will not show fractions or decimals.
- `OneDecimalIfValidDecimal` = `2` — As shown on the dialog: ###.[#]. This format will not show fractions or decimals, unless the rounded decimal part is a non zero value.
- `OneDecimal` = `3` — As shown on the dialog: ###.#. This format will always show the value with one decimal.
- `TwoDecimalsIfValidDecimals` = `4` — As shown on the dialog: ###.[##]. This format will not show fractions or decimals, unless the rounded decimal part is a non zero value.
- `TwoDecimals` = `5` — As shown on the dialog: ###.##. This format will always show the value with two decimals.
- `ThreeDecimalsIfValidDecimals` = `6` — As shown on the dialog: ###.[###]. This format will not show fractions or decimals, unless the rounded decimal part is a non zero value.
- `ThreeDecimals` = `7` — As shown on the dialog: ###.###. This format will always show the value with three decimals.
- `Fractional` = `8` — As shown on the dialog: ### #/#. This format will always show the value with fractions.

## Frame (class)

The Frame class defines a frame that can be given to objects, including texts. A frame is a box around the object. You can specify the color and shape of the box.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f031cc4a-bd67-e91d-c143-8fdc4c580b35)

### Constructors
- `Frame(...)` — Creates a new frame instance with the specified frame type and color.
- `Frame(...)` — Creates a new frame instance with the specified frame type and RGB color.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/443b9742-2894-2ba3-92a8-0889c193cd2b)

#### `GetAxisAlignedBoundingBox(...)`

Returns the bounding box of the frame in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/26c9ea8e-b0f0-2043-4f8e-87b9af27d476)

#### `GetObjectAlignedBoundingBox(...)`

Returns the bounding box of the frame in the local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/9b7d3ea6-fe6a-1e44-1b7d-8545ffc17e94)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/867b0d3f-25bd-f1c4-a8b8-16f9b77c3cd1)

### Properties
- `Color` (object, get/set) — The frame's color.
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the frame.
- `Type` (object, get/set) — The frame's type (square, round, dashed...).

## FrameTypes (enum)

The available frame styles around the object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a054014a-abc5-d549-300d-00be30967f5e)

### Values
- `None` = `0` — No frame.
- `Rectangular` = `1` — The rectangular frame.
- `Line` = `2` — A line under the object.
- `Round` = `3` — The frame with rounded ends.
- `Circle` = `4` — The circular frame.
- `Diamond` = `5` — The frame looking like a diamond.
- `Hexagon` = `6` — The hexagonally shaped frame.
- `Triangle` = `7` — The triangular frame.
- `Sharpened` = `8` — The frame with sharpened ends.

## GADrawing (class)

The GADrawing class is for handling general arrangement drawings in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fde93858-3caf-30b8-9473-62392b72a083)

### Constructors
- `GADrawing(...)` — Instantiates a new GA drawing using standard attributes.
- `GADrawing(...)` — Instantiates a new GA drawing with the given attributes.
- `GADrawing(...)` — Instantiates a new GA drawing with the given name and attribute file.
- `GADrawing(...)` — Instantiates a new GA drawing with attributes. Loads the specified attribute file and overrides with the given layout attributes.

### Methods
#### `CommitChanges(String)(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/92109855-5313-7446-bef4-ab6dde6875f1)

#### `CommitChanges.(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/bc26a611-bd6a-535a-4461-943b8bf5351c)

#### `Delete(...)`

Deletes the drawing. An active drawing cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/4bcff604-27c4-487f-bb7d-5d80a2d87d8b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetPlotFileName(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/fa70baa3-ad78-2e3c-88ca-0b77cb81cd21)

#### `GetPlotFileNameExt(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/6288f4e4-4baf-656d-57bb-c63dee354493)

#### `GetSheet(...)`

Gets the sheet of the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fe921886-00b9-57d3-0488-92bd25014c66)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `Insert(...)`

Inserts an empty general arrangement drawing. Views must be added separately. A drawing can be inserted only when there is no active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/4c112d39-1b7a-b470-8dbd-5d649ca17b53)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/6b1b3008-c9dd-5b97-0881-89ccc580b3e7)

#### `Modify(...)`

Applies the changes made to the drawing attributes.

[Docs](https://developer.tekla.com/topic/en/18/47/5907ed8e-4fb4-9aff-c374-6bf3b73fb003)

#### `PlaceViews(...)`

Places views on the sheet. Computes the required size of the views to fit the parts, then places the views so that they fit on the sheet and do not overlap. Adjusts the sheet size if needed.

[Docs](https://developer.tekla.com/topic/en/18/47/1e1578e4-084f-21a9-3d1a-2b03140440ce)

#### `Select(...)`

Selects the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/c55c986a-b3d9-5f73-567c-d020a31938e2)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

### Properties
- `CommitMessage` (object, get/set) — Gets the commit message
- `CreationDate` (object, get/set) — The drawing creation date.
- `DrawingTypeStr` (object, get/set) — The type of the drawing
- `IsFrozen` (object, get/set) — Describes whether the drawing is frozen.
- `IsIssued` (object, get/set) — Describes whether the drawing is issued.
- `IsIssuedButModified` (object, get/set) — Describes whether the drawing is issued but modified since it was issued.
- `IsLocked` (object, get/set) — Describes whether the drawing is locked.
- `IsLockedBy` (object, get/set) — Gets the logged in username that set the drawing to locked
- `IsMasterDrawing` (object, get/set) — Describes whether the drawing is a master drawing (shown with a special symbol on the drawing list).
- `IsReadyForIssue` (object, get/set) — Describes whether the drawing is ready for issue.
- `IsReadyForIssueBy` (object, get/set) — Gets the logged in username that set the drawing Ready for issue
- `IssuingDate` (object, get/set) — The drawing issuing date.
- `Layout` (object, get/set) — Gets the drawing layout attributes.
- `Mark` (object, get/set) — The drawing mark.
- `ModificationDate` (object, get/set) — The drawing modification date.
- `Name` (object, get/set) — The name of the drawing.
- `OutputDate` (object, get/set) — The drawing output date. XS_DISABLE_DRAWING_PLOT_DATE controls whether this date is set.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SectionViewStartLabel` (object, get/set) — Gets the drawing attribute for start label of section view
- `Title1` (object, get/set) — The first drawing title.
- `Title2` (object, get/set) — The second drawing title.
- `Title3` (object, get/set) — The third drawing title.
- `UpToDateStatus` (object, get/set) — Gets the drawing up to date status.

## GenericAttributesBase (class)

The GenericAttributesBase class is a base class that shared attributes among many classes use.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/07cd6d10-6a6c-93d8-9f20-1f82869159fe)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/1fc87d83-33b8-7e4e-7803-dda26a0aa35a)

## GraphicObject (class)

The GraphicObject class is the abstract base class for all graphic objects (the arc, the circle, the cloud, the rectangular cloud, the line, the polygon, the polyline and the rectangle).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3e9d2a0d-5def-596a-819a-77a9515b447d)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## GraphicObject.GraphicObjectAttributes (class)

The GraphicObjectAttributes class is the attributes class of the graphic object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/769dd789-cb81-512f-c2da-32d369b422ad)

### Constructors
- `GraphicObject.GraphicObjectAttributes(...)` — Creates a new graphic object attributes instance.
- `GraphicObject.GraphicObjectAttributes(...)` — Creates a new graphic object attributes instance using the given attribute file's filename.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/da11fa4f-a98d-c770-d0e2-f458bc3b82dc)

#### `LoadAttributes(...)`

Loads the attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/5597a575-45e2-aa83-6576-390f23af3e48)

### Properties
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## GraphicObjectHatchAttributes (class)

The GraphicObjectHatchAttributes class contains hatch attributes for graphic object hatches.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/20d326bf-d289-3d7a-92e3-2f88973713fd)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/42743022-d347-9ac1-942e-7a6990e9bb83)

### Properties
- `Angle` (object, get/set) — The angle in which to draw the hatch.
- `BackgroundColor` (object, get/set) — The background color of the hatch.
- `Color` (object, get/set) — The color of the hatch.
- `DrawBackgroundColor` (object, get/set) — Sets whether the background color will be enabled or disabled for the hatch. See also: BackgroundColor.
- `FactorType` (object, get/set) — The type of the hatch factor.
- `Name` (object, get/set) — The name of the hatch. If an empty string is given as hatch name it will become "None".
- `OffsetX` (object, get/set) — The offset from the origin in the X-direction.
- `OffsetY` (object, get/set) — The offset from the origin in the Y-direction.
- `ScaleX` (object, get/set) — The scale value of the hatch in the X-direction.
- `ScaleY` (object, get/set) — The scale value of the hatch in the Y-direction.
- `TrueBackgroundColor` (object, get/set) — The true color (rgb enabled) of the hatch background.
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the hatch.

## Grid (class)

The Grid class contains methods related to grids. A drawing grid is inserted/removed using the view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/23c3017b-ef9e-8774-e16c-32ce800fae0c)

### Methods
#### `Delete(...)`

Grids cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/80ba0c5a-50c5-f50c-9191-430ef57b767b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjects(.Type.)(...)`

Retrieves a DrawingObjectEnumerator for all the grid lines of the grid that are of certain TypeFilter.

[Docs](https://developer.tekla.com/topic/en/18/47/58a490eb-40f4-d826-7a4a-19840a7ddd4c)

#### `GetObjects.(...)`

Retrieves a DrawingObjectEnumerator for all the grid lines of the grid.

[Docs](https://developer.tekla.com/topic/en/18/47/ad9dbd39-6586-982a-23b1-44e1c81c9c47)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/0ce24ec2-3572-d116-dd66-c0f195687da5)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/b0e2e402-61ed-365a-43b4-d05e5422440f)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/07fd113e-6165-837b-c95b-6cc73dab339d)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/b4835593-716d-68c6-d6df-f0216f788c08)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the grid. For more information see Grid.GridAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Grid.GridAttributes (class)

The GridAttributes class is the attributes class for the grid.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4d60e400-344e-9cc9-f842-f5c9abd32819)

### Constructors
- `Grid.GridAttributes(...)` — Creates a new default grid attributes instance that loads standard attributes.
- `Grid.GridAttributes(...)` — Creates a new grid attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/8c08ec6f-c6cd-22d9-ece4-8f6dca8a1339)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/edf66ccf-da28-90ec-e685-81c2a4649c9a)

### Properties
- `DrawOnlyTextLabelsNotGrids` (object, get/set) — Set as true if you don't want to draw the grids, only show the grids' text labels.
- `DrawTextAtBottomOfGrid` (object, get/set) — Set as true if you want to show the grid's text labels at the bottom of the grid.
- `DrawTextAtLeftOfGrid` (object, get/set) — Set as true if you want to show the grid's text labels at the left of the grid.
- `DrawTextAtRightOfGrid` (object, get/set) — Set as true if you want to show the grid's text labels at the right of the grid.
- `DrawTextAtTopOfGrid` (object, get/set) — Set as true if you want to show the grid's text labels at the top of the grid.
- `Font` (object, get/set) — The grid's font attributes.
- `Frame` (object, get/set) — The frame attributes of the grid's labels.
- `Line` (object, get/set) — The grid's line attributes.
- `OffsetAtEndOfLine` (object, get/set) — The offset for the end of the grid lines.
- `OffsetAtStartOfLine` (object, get/set) — The offset for the beginning of the grid lines.

## GridLine (class)

The GridLine class contains methods related to grid lines. A drawing grid line is always a child of a grid object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/51956a62-d0c7-3ab5-ab2e-550761cc4ffe)

### Methods
#### `Delete(...)`

GridLines cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/cd867df9-8de5-9115-f880-e0dde1600895)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

GridLines cannot be inserted.

[Docs](https://developer.tekla.com/topic/en/18/47/0a39da47-e22f-fbf5-c833-ca5dca075071)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4a583617-e79d-adbb-1c3c-0e3a0fe803a2)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/fad0c838-84cd-8ce6-1e3c-acc305cb7e9e)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/8b5b0170-7f4e-4f2c-552d-4d8182120381)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the grid line. For more information see GridLine.GridLineAttributes.
- `EndLabel` (object, get/set) — The end of the grid line, contains the grid label and position information. For more information see GridLine.GridLabel.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartLabel` (object, get/set) — The beginning of the grid line, contains the grid label and position information. For more information see GridLine.GridLabel.

## GridLine.GridLabel (class)

The GridLabel class contains the grid labels of the grid lines, the text label and the insertion points.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/dc9cc3a9-7a1e-3320-de0f-a3cd60aad2d5)

### Constructors
- `GridLine.GridLabel(...)` — Creates a new default grid label instance.

### Methods
#### `GetAxisAlignedBoundingBox(...)`

The bounding box of the grid line label (rectangle format, axis aligned).

[Docs](https://developer.tekla.com/topic/en/18/47/1d86e9d9-3f79-b151-6763-2e07f536c676)

#### `GetAxisAlignedBoundingBoxOfTextLabel(...)`

The bounding box of the grid line label's text (rectangle format, axis aligned).

[Docs](https://developer.tekla.com/topic/en/18/47/398b10cf-8189-74dc-700f-6bbdc6064484)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/44eb35f7-0343-00cd-c859-1b58302232d6)

### Properties
- `CenterPoint` (object, get/set) — The center point of the grid label.
- `FrameHeight` (object, get/set) — The height of the grid label frame.
- `FrameWidth` (object, get/set) — The width of the grid label frame.
- `GridLabelPoint` (object, get/set) — The grid line's label point. If the value of GridLabelPoint differs from the OffsetGridPoint value, a leader line will be created to the GridLabelPoint position. You can remove the leader line by setting GridLabelPoint to the GridPoint value (or by setting the GridLabelPoint anywhere on the grid line, but then the offset will change).
- `GridLabelText` (object, get/set) — The grid line's label text.
- `GridPoint` (object, get/set) — The grid line point, without offsets taken into consideration (model grid line coordinates).
- `OffsetGridPoint` (object, get/set) — The grid line point where the grid line actually ends, with offsets applied.
- `TextHeight` (object, get/set) — The height of the grid label's text frame.
- `TextWidth` (object, get/set) — The width of the grid label's text frame.

## GridLine.GridLineAttributes (class)

The GridLineAttributes class is the attributes class for the grid line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/00d87eb4-0b57-590a-3618-375be64d212d)

### Constructors
- `GridLine.GridLineAttributes(...)` — Creates a new default grid line attributes instance that loads standard attributes.
- `GridLine.GridLineAttributes(...)` — Creates a new grid line attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/31a350a0-9c84-12c6-74fa-9c2394fbc007)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/056276ed-908e-fa37-4d76-528c0bcd4a9f)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `DrawOnlyTextLabelsNotGridLines` (object, get/set) — Set as true if you don't want to draw the grid lines, only show the grid lines' text labels.
- `DrawTextAtEndOfGridLine` (object, get/set) — Set as true if you want to show the grid line's text label at the end of the line.
- `DrawTextAtStartOfGridLine` (object, get/set) — Set as true if you want to show the grid line's text label at the start of the line.
- `Font` (object, get/set) — The grid line's font attributes.
- `Frame` (object, get/set) — The frame attributes of the grid line's label.
- `Line` (object, get/set) — The grid line's line attributes.
- `OffsetAtEndOfLine` (object, get/set) — The offset for the end of the grid line.
- `OffsetAtStartOfLine` (object, get/set) — The offset for the beginning of the grid line.

## HatchAttributes (class)

The HatchAttributes base class, inherited by ModelObjectHatchAttributes and GraphicObjectHatchAttributes, contains hatch attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f9241cbb-e95d-32d9-1575-c794bf55d860)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/1fc87d83-33b8-7e4e-7803-dda26a0aa35a)

### Properties
- `Angle` (object, get/set) — The angle in which to draw the hatch.
- `BackgroundColor` (object, get/set) — The background color of the hatch.
- `Color` (object, get/set) — The color of the hatch.
- `DrawBackgroundColor` (object, get/set) — Sets whether the background color will be enabled or disabled for the hatch. See also: BackgroundColor.
- `FactorType` (object, get/set) — The type of the hatch factor.
- `Name` (object, get/set) — The name of the hatch. If an empty string is given as hatch name it will become "None".
- `ScaleX` (object, get/set) — The scale value of the hatch in the X-direction.
- `ScaleY` (object, get/set) — The scale value of the hatch in the Y-direction.
- `TrueBackgroundColor` (object, get/set) — The true color (rgb enabled) of the hatch background.
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the hatch.

## HatchLine (class)

The HatchLine class defines a graphical object representing a line with definable width and hatch fill.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/80bd6f3f-2132-0bc6-080d-8fd0a361c5c2)

### Constructors
- `HatchLine(...)` — Creates a new hatch line instance with a point list using standard attributes.
- `HatchLine(...)` — Creates a new hatch line instance with a point list using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/bef590b5-529a-64a5-b569-08f59acc5eb4)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/8ee4be91-5d3e-53d4-ded7-9e7c0b4ad131)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c9154c52-cf63-59a0-4347-ea578b45b8e0)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/cfa160f5-dfef-0112-9b9b-17341017beba)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/9695cc6e-0a13-1c46-dfa5-9b2dbdc37fa7)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the hatch line.
- `BaseLinePoints` (object, get/set) — The list of points for the hatch line.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## HatchLine.HatchLineAttributes (class)

The HatchLineAttributes class is the attributes class of the hatch line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d4b14e7a-0c10-2132-de30-99a72d7152b9)

### Constructors
- `HatchLine.HatchLineAttributes(...)` — Creates a new default hatch line attributes instance that loads standard attributes.
- `HatchLine.HatchLineAttributes(...)` — Creates a new hatch line attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/73fb5abf-1c5d-cf75-f3b9-0d69491ba188)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/8642d92b-52a5-117c-a5ba-4fb54cc1d9de)

### Properties
- `Alignment` (object, get/set) — Gets or sets the alignment of the hatch line.
- `BehindModelObjects` (object, get/set) — Gets or sets whether the closed graphic object is drawn behind model objects
- `Hatch` (object, get/set) — The hatch attributes of the closed graphic object (the closed graphic object is hatchable).
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.
- `Scale` (object, get/set) — Gets or sets the scale factor for the hatch line.
- `ShowJointLines` (object, get/set) — Gets or sets a value indicating whether to show joint lines of the hatch line.
- `ShowOuterLines` (object, get/set) — Gets or sets a value indicating whether to show outer lines of the hatch line.
- `Width` (object, get/set) — Gets or sets the width of the hatch line.

## HatchLine.HatchLineAttributes.Alignments (enum)

Alignment options for the hatch line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4697a65a-962c-2019-6f34-f3927b45c3d5)

### Values
- `Center` = `0` — Specifies that the hatch line is center aligned.
- `LeftEdge` = `1` — Specifies that the hatch line is left edge aligned.
- `RightEdge` = `2` — Specifies that the hatch line is right edge aligned.

## Hideable (class)

Accesses the information if this object is hidden or not or if it should be.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/adf9e63b-0e5b-ebab-0181-098602515f7d)

### Constructors
- `Hideable(...)` — Initializes a new instance of the Hideable class

### Methods
#### `HideFromDrawing(...)`

Hides this object from the whole drawing. Calling Modify on the object is required to hide the object.

[Docs](https://developer.tekla.com/topic/en/18/47/7feb0562-a85c-0504-cc94-ac7c4da2a769)

#### `HideFromDrawingView(...)`

Hides this object from the drawing view. Calling Modify on the object is required to hide the object.

[Docs](https://developer.tekla.com/topic/en/18/47/8bf4d843-e050-fc65-78c4-7e4eb98b11f3)

#### `ShowInDrawing(...)`

Shows this object in the whole drawing. Calling Modify on the object is required to show the object.

[Docs](https://developer.tekla.com/topic/en/18/47/ad4c979a-4777-72b6-bff0-f9f01d86bfc2)

#### `ShowInDrawingView(...)`

Shows this object in the drawing view. Calling Modify on the object is required to show the object.

[Docs](https://developer.tekla.com/topic/en/18/47/155059b7-6252-a8db-5e4c-7324b6442530)

### Properties
- `IsHidden` (object, get/set) — Retrieves the information if this object is hidden or not.

## Hideable.HiddenFlags (enum)

The flag for keeping track of objects hidden status.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/aebce6d6-f892-1132-4837-b50f34eb2fe5)

### Values
- `NotHidden` = `0` — Not hidden.
- `HiddenBySelf` = `1` — Hidden by self (meaning nothing else is hiding it).
- `HiddenByParent` = `2` — Hidden by parent (the objects parent is hidden and thus this object is also hidden).

## Hideable.ShouldBeHiddenFlags (enum)

The flag for keeping track if object should be hidden.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b2667dac-9ffe-f073-4d30-08a3983132fd)

### Values
- `ShouldNotBeHidden` = `0` — Should not be hidden, default value.
- `HideFromDrawingView` = `1` — To be hidden from drawing view.
- `HideFromDrawing` = `2` — To be hidden from drawing.
- `ShowInDrawingView` = `3` — To be shown in drawing view.
- `ShowInDrawing` = `4` — To be shown in drawing.

## HyperLink (class)

The HyperLink class defines a hyperlink object in a drawing. When a hyperlink is double clicked, the default browser is launched.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/58821f09-5e89-e43d-574c-0277abbafa24)

### Constructors
- `HyperLink(...)` — Creates a new hyperlink instance using standard attributes.
- `HyperLink(...)` — Creates a new hyperlink instance using the given attributes.
- `HyperLink(...)` — Creates a new hyperlink instance using standard attributes.
- `HyperLink(...)` — Creates a new hyperlink instance using the given attributes.

### Methods
#### `Delete(...)`

Deletes the link.

[Docs](https://developer.tekla.com/topic/en/18/47/c4def3cb-f070-f8fb-c1e5-6cb2aaff70e4)

#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/992fb6af-b657-ed89-b5bf-9a8f8854710a)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/bec56125-ac08-53b1-3304-39ba38dd409a)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the link.

[Docs](https://developer.tekla.com/topic/en/18/47/6ae37690-a986-1fa8-a70e-5a612096a569)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/b541bdee-a1e7-d6e2-dd9b-029ba40f995a)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the link.

[Docs](https://developer.tekla.com/topic/en/18/47/a91f2568-1ef6-849f-ace4-75507605058f)

#### `MoveObjectRelative(...)`

Moves the object according to the given move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/504de256-3dc9-1f96-6eae-9baf92e5371d)

#### `Resize(...)`

Resizes the object. The scaling type is changed to ScaleToFit.

[Docs](https://developer.tekla.com/topic/en/18/47/9e3a70e8-d294-bdb3-b272-aee8ea698090)

#### `Select(...)`

Selects the link.

[Docs](https://developer.tekla.com/topic/en/18/47/772b3930-5ac8-f6c7-0bb9-dd27038fea26)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle of the link.
- `Attributes` (object, get/set) — Gets or sets the link attributes.
- `InsertionPoint` (object, get/set) — Gets or sets the insertion point of the link.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Target` (object, get/set) — Gets or sets the target.
- `Text` (object, get/set) — Gets or sets the link text that is visible on the screen.

## IAxisAlignedBoundingBox (interface)

The IAxisAlignedBoundingBox interface is implemented by objects that have an axis aligned bounding box.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/be873f02-ea02-6703-2a50-35389c9b66e4)

### Methods
#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/f330d78c-7cd8-6e65-9af5-3ffae3df133a)

## IEvents (interface)

Interface for the Events class allows the user to register event listeners for Drawings events. This interface allows the Marshaling of the Events class with Trimble.Remoting

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9b0213b1-c8bf-7f15-b67f-6cadf5155f2a)

### Methods
#### `Register(...)`

Registers the instance to listen to the specified events.

[Docs](https://developer.tekla.com/topic/en/18/47/a29c697c-fd29-90b1-22a1-186419feb09e)

#### `UnRegister(...)`

Unregisters the instance from listening to events.

[Docs](https://developer.tekla.com/topic/en/18/47/33683b4f-4add-0b6d-6743-3adb3e5088ef)

### Events
#### `DrawingChanged` (`EventHandler`)

**Signature:** `event EventHandler DrawingChanged`

The DrawingChanged event is raised when the drawing has been changed and the database is committed.

[Docs](https://developer.tekla.com/topic/en/18/47/23af96ae-5aa6-bcc0-fbcf-a9aa902e36b1)

#### `DrawingDeleted` (`EventHandler`)

**Signature:** `event EventHandler DrawingDeleted`

The DrawingDeleted event is raised when a drawing has been deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/ea943c62-cf02-34f0-bbb3-233020f2c45f)

#### `DrawingInserted` (`EventHandler`)

**Signature:** `event EventHandler DrawingInserted`

The DrawingInserted event is raised when a drawing has been inserted.

[Docs](https://developer.tekla.com/topic/en/18/47/54d4e38c-ded8-dacc-103a-88c89f396649)

#### `DrawingReadyForIssuingChange` (`EventHandler`)

**Signature:** `event EventHandler DrawingReadyForIssuingChange`

The DrawingReadyForIssuingChange event is raised just after a drawing is marked or unmarked ready for issuing.

[Docs](https://developer.tekla.com/topic/en/18/47/c543e1bb-632f-9a9f-ee65-56c2aad7163d)

#### `DrawingStatusChanged` (`EventHandler`)

**Signature:** `event EventHandler DrawingStatusChanged`

The DrawingStatusChanged event is raised when the drawing status has changed.

[Docs](https://developer.tekla.com/topic/en/18/47/df0a1a2f-1cd2-f22f-077b-573f4fe8992a)

#### `DrawingUpdated` (`EventHandler`)

**Signature:** `event EventHandler DrawingUpdated`

The DrawingUpdated event is raised when the drawing has been inserted, deleted or modified, and the database is committed.

[Docs](https://developer.tekla.com/topic/en/18/47/9dce710b-0dca-4fca-7026-3d03cea42217)

## IHasChildren (interface)

The IHasChildren interface is implemented by objects that have child objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c8afa60b-3959-4d13-a047-c2a2b6450171)

### Methods
#### `GetObjects(.Type.)(...)`

Gets the children of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/7b2c4ae3-77ab-3ff9-2e07-a39e04badc8f)

#### `GetObjects.(...)`

Gets the children of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/e548034d-415e-3fc3-844f-780bc5d2e7c2)

## IHideable (interface)

The IHideable interface is used for each object that can be hidden in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0c4dfdaf-aa13-bb97-dc11-6f890f917d57)

### Properties
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.

## IIsEqual (interface)

The IIsEqual interface is for comparing objects using the function IsEqual().

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5269446d-386e-3171-79e1-c70d903bdb9d)

### Methods
#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/f3ea1fd0-1de5-e6c6-e153-fd78b0c22e96)

## IMovableRelative (interface)

The IMovableRelative interface is implemented by objects that can be moved.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/dd7c9522-fae4-f292-0602-24131a70b77e)

### Methods
#### `MoveObjectRelative(...)`

Moves the current object using the move vector. The move vector can be calculated as in the following example.

[Docs](https://developer.tekla.com/topic/en/18/47/36bb86bc-dd7e-cfd1-9918-1d6962524d59)

## IObjectAlignedBoundingBox (interface)

The IObjectAlignedBoundingBox interface is implemented by objects that have an object aligned bounding box.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c7cc1640-1aad-5955-defb-1e12e0708cac)

### Methods
#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/06142c5d-cfd3-40c9-493e-9f860a626281)

## IPlacing (interface)

The IPlacing interface is for objects that can have different placings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/560a76d0-1e87-ff6f-4905-ed9bfb167eb8)

### Methods
#### `GetObjects(.Type.)(...)`

Gets the children of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/7b2c4ae3-77ab-3ff9-2e07-a39e04badc8f)

#### `GetObjects.(...)`

Gets the children of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/e548034d-415e-3fc3-844f-780bc5d2e7c2)

### Properties
- `Placing` (object, get/set) — The current placing of the object. See PlacingTypes for different placing options.

## IPreferredPlacing (interface)

The IPreferredPlacing interface is for objects that can have different preferred placing types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/31880a9b-0bd9-17f0-de16-102b1edabb0a)

### Properties
- `PreferredPlacing` (object, get/set) — The type of placing the object should use. See PreferredPlacingTypes for different types of placing.

## IRelatedObjects (interface)

The IRelatedObjects interface is implemented by objects that have related objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/418d8ffc-af61-6647-0028-9b76311ea27f)

### Methods
#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01f07387-e7a7-4074-792a-d94e64785249)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/003820ed-f0fb-31db-bd7b-9e29af26c9ea)

## IResizeable (interface)

The IResizeable interface is implemented by objects that can be resized.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/dccac9b4-8457-057f-9198-c0a6633523fa)

### Methods
#### `Resize(...)`

Resizes the object to the specified size, overriding possible scaling settings.

[Docs](https://developer.tekla.com/topic/en/18/47/427f75fb-07cb-ce16-16bb-f2de80b5e716)

## Image (class)

The Image class defines an image that can be inserted into a drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/362655f7-c664-d22f-3246-90d4d910cdaa)

### Constructors
- `Image(...)` — Initializes a new instance of the Image class with the given parameters.
- `Image(...)` — Initializes a new instance of the Image class using the specified attributes.
- `Image(...)` — Initializes a new instance of the Image class with the given parameters.
- `Image(...)` — Initializes a new instance of the Image class using the specified attributes.

### Methods
#### `Delete(...)`

Deletes the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/5dc2f662-fa9f-431d-d6c4-c8b9fd7cb063)

#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/92532071-8507-973e-5e7c-303633870edc)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/2057db75-e1b4-d5cc-5ba4-5849af223043)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/faa4a0d5-3e63-d983-f5a2-625481575fb4)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/75685c67-29ad-955a-15b8-38e2558bdd89)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/4dc862b3-4968-e29c-83bf-a84a18c47e84)

#### `MoveObjectRelative(...)`

Moves the object according to the given move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/96087af0-e3e1-88dc-7d10-2aa68bcfe8d8)

#### `Resize(...)`

Resizes the object. The scaling type is changed to ScaleToFit.

[Docs](https://developer.tekla.com/topic/en/18/47/58cefb6f-d78a-089e-e16e-8dc0bdf1bf09)

#### `Select(...)`

Selects the embedded object.

[Docs](https://developer.tekla.com/topic/en/18/47/fc0aa530-1e2b-0062-db7b-21f4ff2306d1)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle (in degrees).
- `Attributes` (object, get/set) — Gets or sets the attributes.
- `FileName` (object, get/set) — Gets or sets the file name defining the embedded object.
- `InsertionPoint` (object, get/set) — Gets or sets the insertion point.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Size` (object, get/set) — Gets or sets the size of the frame of the embedded object.

## IncludeRevisionMarkEnum (enum)

Options for including a revision info in file name

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bc567ae8-40ba-d12f-6a83-d4f85959ccd7)

### Values
- `Never` = `0` — Never include revision info
- `Always` = `1` — Always include revision info
- `ByFormatString` = `2` — Include revision info if it's present in the format string, otherwise not

## InsidePartAlongPartOrWithLeaderLinePlacingType (class)

The InsidePartAlongPartOrWithLeaderLinePlacingType class defines a placing type that places the object using the base line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line. The base line used is in the middle of the part, aligned with the part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2f6e6f1d-f5c8-a4a5-3af3-804fb15825b2)

### Constructors
- `InsidePartAlongPartOrWithLeaderLinePlacingType(...)` — Creates a new inside part along part or with leader line placing type instance.

## InsidePartAlongPartPlacingType (class)

The InsidePartAlongPartPlacingType class defines a placing type that places the object using the base line placing. The base line used is in the middle of the part, aligned with the part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a372b6c0-8b52-bfb8-a605-da56b1d601e8)

### Constructors
- `InsidePartAlongPartPlacingType(...)` — Creates a new inside part along part placing type instance.

## InsidePartHorizontalOrWithLeaderLinePlacingType (class)

The InsidePartHorizontalOrWithLeaderLinePlacingType class defines a placing type that places the object using the base line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line. The base line used is in the middle of the part, aligned with the part. In addition the object will be rotated to be horizontal in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0746e3cd-c2cf-6fef-b019-e19d35518c7f)

### Constructors
- `InsidePartHorizontalOrWithLeaderLinePlacingType(...)` — Creates a new inside part horizontal or with leader line placing type instance.

## InsidePartHorizontalPlacingType (class)

The InsidePartHorizontalPlacingType class defines a placing type that places the object using the base line placing. The base line used is in the middle of the part, aligned with the part. In addition the object will be rotated to be horizontal in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/99cc2760-105a-bb05-e486-17b846a7d172)

### Constructors
- `InsidePartHorizontalPlacingType(...)` — Creates a new inside part horizontal placing type instance.

## IntList (class)

The IntList class defines a type safe list of integers.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2f040240-80e1-d7ed-7bc9-e99324180ca9)

### Constructors
- `IntList(...)` — Initializes a new instance of the IntList class

### Methods
#### `Add(...)`

Adds a new integer to the end of the string list.

[Docs](https://developer.tekla.com/topic/en/18/47/047ae331-ab3f-f133-9ad9-44c9e916afbd)

#### `Contains(...)`

Checks if the given integer is in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/a497d712-0f41-327b-f730-0028d83be6c8)

#### `GetRange(...)`

Gets a sub-list of the integer list.

[Docs](https://developer.tekla.com/topic/en/18/47/06faae13-73e4-b5ca-3f59-f42c12381516)

#### `IndexOf(Int32)(...)`

Gets the index of the specified integer.

[Docs](https://developer.tekla.com/topic/en/18/47/2cccec74-0b25-169f-ddbd-11b9360323ad)

#### `IndexOf(Int32, Int32)(...)`

Gets the index of the specified integer between the index to start the search from and the end of the list.

[Docs](https://developer.tekla.com/topic/en/18/47/925dae1d-c39e-7149-1d58-f8bcac32e8be)

#### `IndexOf(Int32, Int32, Int32)(...)`

Gets the index of the specified integer between the index to start the search from and startIndex + count.

[Docs](https://developer.tekla.com/topic/en/18/47/4ffca6bf-7601-19f3-5313-fc8749dfbdf9)

#### `Insert(...)`

Inserts the integer to the specified index.

[Docs](https://developer.tekla.com/topic/en/18/47/c256012d-a9b7-0165-7a1c-f7a2c104147c)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/426ae003-7107-e1f8-06f1-8c6e8dd833ee)

#### `LastIndexOf(Int32)(...)`

Searches for the last occurence of the integer in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/2aebe3ab-2bba-4fc2-4e3f-137c6589cb0f)

#### `LastIndexOf(Int32, Int32)(...)`

Searches for the last occurence of the integer in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/688d011b-6613-c514-9478-de9a30dc8deb)

#### `LastIndexOf(Int32, Int32, Int32)(...)`

Searches for the last occurence of the integer in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/9fa1a7a6-6810-cb91-91eb-27c93769b7d1)

#### `Remove(...)`

Removes the first occurence of the integer from the list.

[Docs](https://developer.tekla.com/topic/en/18/47/cd8e0284-12af-e850-bf04-e00168f6fe16)

#### `RemoveRange(...)`

Removes a range of items starting from the given index.

[Docs](https://developer.tekla.com/topic/en/18/47/29b15dbf-aae0-4823-6696-87243ba20ad3)

#### `ToArray(...)`

Copies the integers to an integer array.

[Docs](https://developer.tekla.com/topic/en/18/47/8abbc579-908a-e6d0-c687-133300d69e76)

### Properties
- `Item` (object, get/set) — Gets or sets the integer at the specified index.

## InvalidAttributesForOperationException (class)

The InvalidAttributesForOperationException class defines the exception that is thrown when an object's attributes are set to something that is not valid for the object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/13384446-c2d4-0dc5-5a3e-e88fe4fcfe8a)

### Constructors
- `InvalidAttributesForOperationException(...)` — Creates a new exception instance.

## InvalidPluginPickerInputException (class)

The InvalidPluginPickerInputException class defines the exception that is thrown when UI.Picker is used during a Plugin.Insert call but the previously assigned PluginPickerInput does not correspond to the next UI.Picker call.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/502ebf7f-f4ca-c868-7642-373b66416eb9)

### Constructors
- `InvalidPluginPickerInputException(...)` — Creates a new InvalidPluginPickerInputException instance.

## InvalidTypeException (class)

The InvalidTypeException class defines the exception that is thrown when a type was not one of the allowed ones.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6bf4fcbe-a96d-9809-629d-b9491f452c57)

### Constructors
- `InvalidTypeException(...)` — Creates a new InvalidTypeException instance.

## LayoutAttributes (class)

The LayoutAttributes class contains attributes for controlling the drawing layout.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1850def0-b58c-df22-0db5-e61ee3c34036)

### Constructors
- `LayoutAttributes(...)` — Creates a new layout attributes instance that loads standard attributes.
- `LayoutAttributes(...)` — Creates a new layout attributes instance that loads drawing layout attributes from the specified attribute file.
- `LayoutAttributes(...)` — Creates a new layout attributes instance that loads standard attributes.
- `LayoutAttributes(...)` — Creates a new layout attributes instance that loads drawing layout attributes from the specified attribute file.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/628efe7a-02bf-5d5c-0e53-cf5464e9e70d)

#### `LoadAttributes(...)`

Loads drawing layout attributes from the specified attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/be6c4b9e-a9a4-b5c7-e99c-a88c1d33ebcc)

### Properties
- `AutoSizeOptions` (object, get/set) — Gets or sets the auto size options that control the drawing sheet size. Setting this property will change the SizeDefinitionMode to AutoSize.
- `ListHiddenObjectsInTemplates` (object, get/set) — Gets or sets the property for controlling whether to list hidden objects in templates.
- `SheetSize` (object, get/set) — Gets or sets the sheet size. Setting this property will change the SizeDefinitionMode to SpecifiedSize.
- `SizeDefinitionMode` (object, get/set) — Gets or sets the size definition mode.

## LeaderLine (class)

The LeaderLine class defines a leader line attached to a parent object, for example a Text or a Mark. The LeaderLine can be modified to add/remove elbow points or to change where it is pointing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f7503a7c-8599-f10f-cd3f-d9632596af1e)

### Methods
#### `Delete(...)`

A leader line cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/55dc81ea-f9f5-b49a-af01-22b286c1b307)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

A leader line cannot be inserted.

[Docs](https://developer.tekla.com/topic/en/18/47/3179c628-5248-ef8c-b8ae-6ee695921cbe)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/a367edcc-e56e-96f7-a644-a662a8185df3)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the leader line in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/c1072353-6212-6e01-ff12-093ef915d5fa)

#### `Select(...)`

Selects the leader line from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/3e793cdb-ed27-2002-961f-aa4392536e2a)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `ElbowPoints` (object, get/set) — The leader line's elbow points (middle points).
- `EndPoint` (object, get/set) — The end point of the leader line (set by the parent object).
- `LeaderLineType` (object, get/set) — Gets the type of the leader line. For a list of the available leader line types see LeaderLine.LeaderLineTypes.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — The starting point of the leader line.

## LeaderLine.LeaderLineTypes (enum)

The different types of leader lines available.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7cf6ff97-74c7-7dab-6c0b-1daf30c10428)

### Values
- `Undefined` = `0` — No type defined yet (this means that the leader line isn't selected yet).
- `NormalLeaderLine` = `1` — The standard leader line type. Its start point and elbow points can be freely modified. The end point is read-only information.
- `SupportLeaderLine` = `2` — The support leader line type, used in reinforcement marks with parallel leader lines. All leader line information is read-only.
- `ExtensionLeaderLine` = `3` — The type for the extension leader lines (enabled by the usage of an advanced option). All leader line information is read-only.

## LeaderLineAndParentObjectAlongPartPlacingType (class)

The LeaderLineAndParentObjectAlongPartPlacingType class defines a placing type that places the object with a leader line and rotated to be at the same angle as the connected part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/91b99dcd-589f-a08c-b38a-6aa79c43ff21)

### Constructors
- `LeaderLineAndParentObjectAlongPartPlacingType(...)` — Creates a new leader line and parent object along part placing type instance.

## LeaderLinePlacing (class)

The LeaderLinePlacing class defines a placing where the object is placed attached to a leader line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8d9ed204-c63f-3704-6012-4a401c2cd950)

### Constructors
- `LeaderLinePlacing(...)` — Creates a new leader line placing instance. The given point defines the start point of the leader line.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/9eceac7d-2672-4279-d808-b365ecc3e2c7)

### Properties
- `StartPoint` (object, get/set) — The start point of the leader line (if multiple leader lines exist, only returns the first).

## LeaderLinePlacingType (class)

The LeaderLinePlacingType class defines a placing type that places the object with a leader line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e8d1a85a-a4b7-79e7-22f9-99906e34409f)

### Constructors
- `LeaderLinePlacingType(...)` — Creates a new leader line placing type instance.

## LevelMark (class)

The LevelMark class defines a drawing object that is displayed as a level mark in a drawing. See the Tekla Structures help for more information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fce74c34-2166-28c3-f4db-792a8ecdff6c)

### Constructors
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the default standard attributes.
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the given attributes.
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the given attributes.
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the given attributes.
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the given attributes.
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the given attributes.
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the given attributes.
- `LevelMark(...)` — Initializes a new instance of the LevelMark class using the given attributes.

### Methods
#### `Delete(...)`

Deletes the level mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/b10336fd-b459-a877-0e4a-dcb65e25769e)

#### `GetAxisAlignedBoundingBox(...)`

Returns the axis aligned bounding box of the level mark (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/87100edf-a1b5-af48-a4fc-1c4d7e8264d4)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Returns the bounding box of the level mark (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/3f4fc5f4-2b3a-d410-68ea-e7dd6af1d96f)

#### `GetObjects(.Type.)(...)`

Gets the children objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/6345f693-40cb-39a3-cd7d-2ca032df473b)

#### `GetObjects.(...)`

Gets the children objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/e697d436-dbc5-daf0-5f23-441f0ed1d3f0)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the level mark into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/098b8330-bcfa-6dda-40dc-836e08cd3692)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/fa26702e-29e2-512c-b1cd-23c502ea0aa2)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the level mark in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/d730928a-7f22-1a4b-351d-4830dd26f25e)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/274e6d27-c54b-31cd-8cc5-30075d19d665)

#### `Select(...)`

Selects the level mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/6e4eabea-c3ae-b9ec-5759-2c93154afc98)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the level mark attributes.
- `BasePoint` (object, get/set) — Gets or sets the level mark base point.
- `Hideable` (object, get/set) — Gets the information if this object is hidden or not or if it should be.
- `InsertionPoint` (object, get/set) — Gets or sets the level mark insertion point.
- `ModelObjectIdentifier` (object, get/set) — Gets the model object identifier
- `Placing` (object, get/set) — Gets or sets the current placing of the object. See the placing types for different placing options.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SubType` (object, get/set) — Gets or sets the subtype of the level mark.

## LevelMark.LevelMarkAttributes (class)

The LevelMarkAttributes class contains the attributes for the level mark.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ac4e8119-c21c-dd94-2a5c-f7537c74525d)

### Constructors
- `LevelMark.LevelMarkAttributes(...)` — Initializes a new instance of the LevelMark.LevelMarkAttributes class that loads standard attributes.
- `LevelMark.LevelMarkAttributes(...)` — Initializes a new instance of the LevelMark.LevelMarkAttributes class that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/6a5f89ee-75fb-acf8-f90a-70311f0a3c83)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/61650b6f-be32-1dab-3b09-8256f7189601)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle of the level mark, in degrees.
- `ArrowHead` (object, get/set) — Gets or sets the arrow attributes for the level mark.
- `Font` (object, get/set) — Gets or sets the font of the level mark text.
- `Frame` (object, get/set) — Gets or sets the frame attributes for the level mark.
- `PlacingAttributes` (object, get/set) — Gets or sets the placing attributes that the object should use. Using these you can specify whether the object is automatically arranged in the drawing or not.
- `Postfix` (object, get/set) — Gets or sets text after the level value.
- `Prefix` (object, get/set) — Gets or sets the text before the level value.
- `TextHidden` (object, get/set) — Gets or sets a value indicating whether the numeric values are visible or hidden.
- `TransparentBackground` (object, get/set) — Gets or sets a value indicating whether the transparency of the background of the level mark text is enabled.
- `Unit` (object, get/set) — Gets or sets the unit attributes.
- `UseGrouping` (object, get/set) — Gets or sets a value indicating whether to use different grouping options to represent the level mark dimensions.
- `UsePositiveSignForPositiveLevels` (object, get/set) — Gets or sets a value indicating whether a + character in front of positive level value.

## LevelMark.LevelMarkType (enum)

Defines options about how the level mark is shown.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2f6865a2-2311-b832-fc45-e2d34079958b)

### Values
- `NoArrowNoLeaderLine` = `0` — Level mark is shown without an arrow and a leader line.
- `ArrowWithoutLeaderLine` = `1` — Level mark is shown with an arrow but without a leader line.
- `InclinedLeaderLine` = `2` — Level mark is shown with an inclined leader line but without an arrow.
- `OrthogonalLeaderLine` = `3` — Level mark is shown with an orthogonal leader line and with an arrow.

## Line (class)

The Line class defines a line that is a two-point line with an optional bulge (curve).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/363f7e93-fecf-3cde-bf2b-d3d7e69612f2)

### Constructors
- `Line(...)` — Creates a new line instance with two points and no bulge (straight line) using standard attributes.
- `Line(...)` — Creates a new line instance with two points and a bulge using standard attributes.
- `Line(...)` — Creates a new line instance with two points and no bulge (straight line) using the given attributes.
- `Line(...)` — Creates a new line instance with two points and a bulge using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/4b20fa91-4db1-9a9b-f9dd-0623bf3e92e0)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/53766bd6-4de8-9ace-b8a7-6829e76de287)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/0d740f84-4cb9-bb44-55e0-5c72f7dd92a5)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/06acda7f-52ec-3928-119f-4bee6ce133ff)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/950fbd22-f825-a413-2662-93bb43c15dea)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the line.
- `Bulge` (object, get/set) — The bulge (curve) to use for the line (a bulge is the ratio between the height and width of an object).
- `EndPoint` (object, get/set) — The end point of the line.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — The start point of the line.

## Line.LineAttributes (class)

The LineAttributes class is the attributes class of the line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e6365158-0bd4-42cd-8068-a8f8c605c955)

### Constructors
- `Line.LineAttributes(...)` — Creates a new default line attributes instance that loads standard attributes.
- `Line.LineAttributes(...)` — Creates a new line attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/d743bcbf-c959-1831-0386-c9fbee6f9810)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/317028db-7b96-a1a1-35e0-5cd89c50c6e5)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the open graphic object. The end points of an open graphic object can contain arrowheads.
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## LineTypeAttributes (class)

The LineTypeAttributes class contains basic attributes for lines.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7f1413c6-c244-9485-97d7-4825df1f5e31)

### Constructors
- `LineTypeAttributes(...)` — Creates a new line type attributes instance.
- `LineTypeAttributes(...)` — Creates a new line type attributes instance with the given line type and color.
- `LineTypeAttributes(...)` — Creates a new line type attributes instance with the given line type and color.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c39f2c11-8b12-201f-711c-ed81d31727d7)

### Properties
- `Color` (object, get/set) — The color of the line (see DrawingColors for options).
- `TrueColor` (object, get/set) — The true color (rgb enabled) of the line.
- `Type` (object, get/set) — The type of the line (see LineTypes for options).

## LineTypes (class)

The LineTypes class defines the possible line types that can be used by the drawing objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d03ed4aa-3509-cc8a-04ef-9b56ae3fab9d)

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/1ed967ac-f07e-0cc1-c3c8-c2dba440dcc8)

#### `Custom(...)`

The Custom line type defines a line that is drawn using one of the custom line types, as specified by the CustomLineTypeName.

[Docs](https://developer.tekla.com/topic/en/18/47/14e7a45f-c8ba-cddd-f71f-768adacfea19)

#### `Equals(...)`

Returns true if the current object and the given object are equal.

[Docs](https://developer.tekla.com/topic/en/18/47/b4e0cd18-0e45-28f4-dc2a-4ce9399f142c)

#### `GetHashCode(...)`

Returns a hash code for the object.

[Docs](https://developer.tekla.com/topic/en/18/47/e76f3aa2-80f7-e0a1-47da-d7b8e484fbcf)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/50b5e14f-4a13-4df3-f496-0fe6c740a6af)

#### `ToString(...)`

Returns the LineTypes information as a string.

[Docs](https://developer.tekla.com/topic/en/18/47/33edfbad-ab7c-406a-fec4-5e932e3ea8aa)

## LineTypesEnum (enum)

The line types (the presentation of the line).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/74868e4d-6ded-4dee-92eb-c69c3abb316c)

### Values
- `UndefinedLine` = `0` — The line type is undefined.
- `SolidLine` = `1` — The solid line (default).
- `DashedLine` = `2` — The dashed line (-----).
- `SlashedLine` = `3` — The slashed line (- - -).
- `DashDot` = `4` — The dash dotted line (--.--.).
- `DottedLine` = `5` — The dotted line (.....).
- `DashDoubleDot` = `6` — The dash double dotted line (-..-..-).
- `SlashDash` = `7` — The slash dash line (-- - -- -).
- `Custom` = `9999` — The custom line type (it will also transport the custom line type name)

## LinkAttributes (class)

The LinkAttributes class contains the attributes for controlling the appearance of a link.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4fe70624-5381-eb78-a7a4-17802129c21d)

### Constructors
- `LinkAttributes(...)` — Creates a new link attributes instance using standard attributes.
- `LinkAttributes(...)` — Creates a new link attributes instance loading the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/d0248c23-25fb-914d-c803-44c1ad907136)

#### `LoadAttributes(...)`

Loads the link frame attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/9aa04884-118a-a851-bc6c-be135647aa0d)

### Properties
- `Font` (object, get/set) — Gets or sets the font attributes.
- `Frame` (object, get/set) — Gets or sets the link frame attributes.
- `IsUnderlined` (object, get/set) — Defines whether the link is underlined or not.
- `Scaling` (object, get/set) — Gets or sets the link scaling options.

## LinkBase (class)

The LinkBase class is the base class for links.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/86f36190-bcf2-9e2e-8917-295c91b15cde)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/992fb6af-b657-ed89-b5bf-9a8f8854710a)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/bec56125-ac08-53b1-3304-39ba38dd409a)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `MoveObjectRelative(...)`

Moves the object according to the given move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/504de256-3dc9-1f96-6eae-9baf92e5371d)

#### `Resize(...)`

Resizes the object. The scaling type is changed to ScaleToFit.

[Docs](https://developer.tekla.com/topic/en/18/47/9e3a70e8-d294-bdb3-b272-aee8ea698090)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle of the link.
- `Attributes` (object, get/set) — Gets or sets the link attributes.
- `InsertionPoint` (object, get/set) — Gets or sets the insertion point of the link.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Text` (object, get/set) — Gets or sets the link text that is visible on the screen.

## LinkFrameAttributes (class)

The LinkFrameAttributes class contains the attributes for controlling the appearance of a link frame.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/46352982-8c81-4d2f-b58d-05ef4466176f)

### Constructors
- `LinkFrameAttributes(...)` — Initializes a new instance of the LinkFrameAttributes class

### Methods
#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box of the frame in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/a41d1798-1556-f561-f40b-79a9118b48f5)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box of the frame in the local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/6fe41cdb-f28b-9253-d9dc-8fe9a419471c)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/b2e4ca25-cc6b-5b25-7ac6-37e67fe95ef6)

### Properties
- `Color` (object, get/set) — Obsolete.
- `Line` (object, get/set) — Gets or sets the line type.
- `LineType` (object, get/set) — Obsolete.

## Mark (class)

The Mark class represents a mark object that contains a single mark. Several mark objects can be merged into one mark set.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/27fcc7fe-3bc1-d692-2b30-83c4fe213fb1)

### Constructors
- `Mark(...)` — Constructs a Mark that will be attached to the specified ModelObject upon Insert.

### Methods
#### `Delete(...)`

Calls the system to delete the mark attached to a model object. After this operation, only associative notes can be attached to that model object.

[Docs](https://developer.tekla.com/topic/en/18/47/e6aad9bc-bff0-99b1-7612-de5aaf232658)

#### `GetAxisAlignedBoundingBox(...)`

Returns the axis aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/7d4a86d5-1579-545d-35e8-fbdf5a9cb282)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Returns the object aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/7c35501a-be1b-5b35-69e5-70cd3010ff93)

#### `GetObjects(.Type.)(...)`

Gets the children objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/75151c19-9f40-2e6f-8da8-8691595f4ea1)

#### `GetObjects.(...)`

Gets the children objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/b1bf3deb-7012-dcc6-8bf4-6edc9b5cad4f)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/b6576364-5892-1074-c9e1-7e5ea96dabc6)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/76a4951b-0899-1dd4-52dd-7d5766023883)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/6ee73645-4f04-ab3e-18e6-25239b5902d4)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/b5a520e8-01a4-3e66-58d3-1689d451a17b)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/422f9924-8f5a-ee8a-450a-b6bfc1cb703a)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the mark attributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `InsertionPoint` (object, get/set) — The insertion point of the mark.
- `IsAssociativeNote` (object, get/set) — Gets a value indicating whether the mark is an associative note.
- `Placing` (object, get/set) — The current placing of the object. See the placing types for different placing options.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Mark.MarkAttributes (class)

The MarkAttributes class defines the mark attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4dede4c2-574c-e381-baef-66b0c36c9fdf)

### Constructors
- `Mark.MarkAttributes(...)` — Creates a new default mark attributes instance that loads standard attributes.
- `Mark.MarkAttributes(...)` — Creates a new mark attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/48571435-fb53-ae1c-8991-522c0f153733)

#### `LoadAttributes(...)`

Loads attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/bc73e5ae-f88b-8971-2354-196cf03b0255)

### Properties
- `Angle` (object, get/set) — The actual angle of the mark, in degrees. Can differ from the rotation angle set by the user for marks, e.g. 45-degree column marks in GA drawings or when placing mark along part edges.
- `ArrowHead` (object, get/set) — The arrowhead of the leader line(s).
- `Content` (object, get/set) — The mark content as a list of elements.
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `Frame` (object, get/set) — The frame attributes of the mark's labels.
- `PlacingAttributes` (object, get/set) — The placing attributes that the object should use. Using these you can specify whether the object is automatically arranged in the drawing or not.
- `PreferredPlacing` (object, get/set) — The type of placing the object should use. See the preferred placing types for different types of placing.
- `RotationAngle` (object, get/set) — The mark rotation angle set by the user. Unit is degrees.
- `TextAlignment` (object, get/set) — The mark text alignment.
- `TransparentBackground` (object, get/set) — The mark text background transparency. (transparent/opaque)

## MarkBase (class)

The MarkBase class contains general mark information and can be selected and modified with the general attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4cf94af5-de25-6ca8-bb63-f11c9f2198f3)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetAxisAlignedBoundingBox(...)`

Returns the axis aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/7d4a86d5-1579-545d-35e8-fbdf5a9cb282)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Returns the object aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/7c35501a-be1b-5b35-69e5-70cd3010ff93)

#### `GetObjects(.Type.)(...)`

Gets the children objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/75151c19-9f40-2e6f-8da8-8691595f4ea1)

#### `GetObjects.(...)`

Gets the children objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/b1bf3deb-7012-dcc6-8bf4-6edc9b5cad4f)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/b5a520e8-01a4-3e66-58d3-1689d451a17b)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the mark base attributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `InsertionPoint` (object, get/set) — The insertion point of the mark.
- `IsAssociativeNote` (object, get/set) — Gets a value indicating whether the mark is an associative note.
- `Placing` (object, get/set) — The current placing of the object. See the placing types for different placing options.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## MarkBase.MarkBaseAttributes (class)

The MarkBaseAttributes class is the attributes class for the mark base.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0a855c11-4d75-8d03-91e3-450ddbd154f4)

### Constructors
- `MarkBase.MarkBaseAttributes(...)` — Creates a new default mark base attributes instance that loads standard attributes.
- `MarkBase.MarkBaseAttributes(...)` — Creates a new mark base attributes instance with the given attributes file's filename.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/da11fa4f-a98d-c770-d0e2-f458bc3b82dc)

#### `LoadAttributes(...)`

Loads the attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/5597a575-45e2-aa83-6576-390f23af3e48)

### Properties
- `Angle` (object, get/set) — The actual angle of the mark, in degrees. Can differ from the rotation angle set by the user for marks, e.g. 45-degree column marks in GA drawings or when placing mark along part edges.
- `ArrowHead` (object, get/set) — The arrowhead of the leader line(s).
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `Frame` (object, get/set) — The frame attributes of the mark's labels.
- `PlacingAttributes` (object, get/set) — The placing attributes that the object should use. Using these you can specify whether the object is automatically arranged in the drawing or not.
- `PreferredPlacing` (object, get/set) — The type of placing the object should use. See the preferred placing types for different types of placing.
- `RotationAngle` (object, get/set) — The mark rotation angle set by the user. Unit is degrees.
- `TextAlignment` (object, get/set) — The mark text alignment.
- `TransparentBackground` (object, get/set) — The mark text background transparency. (transparent/opaque)

## MarkBase.MarkBaseAttributes.TextAlignOptions (enum)

Enumeration of the possible text alignment values.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b95d051a-03df-2ce9-2ed6-62caf2b0d0df)

### Values
- `Left` = `0` — Text is left-aligned.
- `Center` = `1` — Text is center-aligned.
- `Right` = `2` — Text is right-aligned.

## MarkSet (class)

The MarkSet class defines mark set objects that contain several mark objects merged to one mark set.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1e43b65c-6268-c52d-9a74-006c8b9d692d)

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/d28ec2a8-8cfd-553a-142d-44b07213f90a)

#### `GetAxisAlignedBoundingBox(...)`

Returns the axis aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/7d4a86d5-1579-545d-35e8-fbdf5a9cb282)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Returns the object aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/7c35501a-be1b-5b35-69e5-70cd3010ff93)

#### `GetObjects(.Type.)(...)`

Gets the children objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/75151c19-9f40-2e6f-8da8-8691595f4ea1)

#### `GetObjects.(...)`

Gets the children objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/b1bf3deb-7012-dcc6-8bf4-6edc9b5cad4f)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Mark sets cannot be inserted.

[Docs](https://developer.tekla.com/topic/en/18/47/4d8d1f90-84ed-7aef-b216-d1128333adaa)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/a6c5d089-00ce-3e81-18b9-bc860ac6fbc1)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/a5b76ae6-3768-d127-81f4-b119058af8fd)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/b5a520e8-01a4-3e66-58d3-1689d451a17b)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/733fbbe3-1d9f-0ce8-00d8-541828adff9e)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the mark set attributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `InsertionPoint` (object, get/set) — The insertion point of the mark.
- `IsAssociativeNote` (object, get/set) — Gets a value indicating whether the mark is an associative note.
- `Placing` (object, get/set) — The current placing of the object. See the placing types for different placing options.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## MarkSet.MarkSetAttributes (class)

The MarkSetAttributes class is the attribute class for the mark set.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/02a61a8d-5979-873a-a818-588030444c16)

### Constructors
- `MarkSet.MarkSetAttributes(...)` — Initializes a new instance of the MarkSet.MarkSetAttributes class

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/52043a5a-0ded-dae1-433e-b78dbf2bd68a)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file. Not implemented yet.

[Docs](https://developer.tekla.com/topic/en/18/47/000e97d7-16e6-56b8-9ede-6153fecbc722)

### Properties
- `Angle` (object, get/set) — The actual angle of the mark, in degrees. Can differ from the rotation angle set by the user for marks, e.g. 45-degree column marks in GA drawings or when placing mark along part edges.
- `ArrowHead` (object, get/set) — The arrowhead of the leader line(s).
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `Frame` (object, get/set) — The frame attributes of the mark's labels.
- `PlacingAttributes` (object, get/set) — The placing attributes that the object should use. Using these you can specify whether the object is automatically arranged in the drawing or not.
- `PreferredPlacing` (object, get/set) — The type of placing the object should use. See the preferred placing types for different types of placing.
- `RotationAngle` (object, get/set) — The mark rotation angle set by the user. Unit is degrees.
- `TextAlignment` (object, get/set) — The mark text alignment.
- `TransparentBackground` (object, get/set) — The mark text background transparency. (transparent/opaque)

## MarkSymbolShape (enum)

The mark symbol shapes for the detail mark.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/94c5a91d-8f13-ba70-192b-ea34fcbbc28c)

### Values
- `None` = `1` — No shape.
- `Circular` = `2` — The circular shape.
- `SlashedCircular` = `3` — The slashed circular shape.
- `Custom` = `4` — The custom shape.

## ModelObject (class)

The ModelObject class is an abstract class for all model objects in the drawing side. The model objects of the drawing side contain identifiers of the actual model objects in the model database. The model objects appear and disappear based on current view properties (view size, settings).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/415b2552-1c37-d8ba-b1bd-59a60b8f0783)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## ModelObjectHatchAttributes (class)

The ModelObjectHatchAttributes class contains hatch attributes for model object hatches.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/50587d82-2278-88b6-491b-0a4c58015d81)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4d971521-0161-e151-2ecc-52e2351a380c)

### Properties
- `Angle` (object, get/set) — The angle in which to draw the hatch.
- `AutomaticScaling` (object, get/set) — Specifies whether automatic scaling is enabled. If disabled, custom scale values will be used (see ScaleX and ScaleY).
- `BackgroundColor` (object, get/set) — The background color of the hatch.
- `Color` (object, get/set) — The color of the hatch.
- `DrawBackgroundColor` (object, get/set) — Sets whether the background color will be enabled or disabled for the hatch. See also: BackgroundColor.
- `FactorType` (object, get/set) — The type of the hatch factor.
- `Name` (object, get/set) — The name of the hatch. If an empty string is given as hatch name it will become "None".
- `ScaleX` (object, get/set) — The scale value of the hatch in the X-direction.
- `ScaleY` (object, get/set) — The scale value of the hatch in the Y-direction.
- `TrueBackgroundColor` (object, get/set) — The true color (rgb enabled) of the hatch background.
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the hatch.

## MultiDrawing (class)

The MultiDrawing class is for handling multidrawings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fc3ba5fd-4ff0-8683-dcb2-929e61446a07)

### Methods
#### `CommitChanges(String)(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/92109855-5313-7446-bef4-ab6dde6875f1)

#### `CommitChanges.(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/bc26a611-bd6a-535a-4461-943b8bf5351c)

#### `Delete(...)`

Deletes the drawing. An active drawing cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/4bcff604-27c4-487f-bb7d-5d80a2d87d8b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetPlotFileName(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/fa70baa3-ad78-2e3c-88ca-0b77cb81cd21)

#### `GetPlotFileNameExt(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/6288f4e4-4baf-656d-57bb-c63dee354493)

#### `GetSheet(...)`

Gets the sheet of the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fe921886-00b9-57d3-0488-92bd25014c66)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `Insert(...)`

Not supported.

[Docs](https://developer.tekla.com/topic/en/18/47/9af81525-8235-df90-aca4-ed5c418145ad)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/6b1b3008-c9dd-5b97-0881-89ccc580b3e7)

#### `Modify(...)`

Applies the changes made to the drawing attributes.

[Docs](https://developer.tekla.com/topic/en/18/47/108368a4-a79b-0076-9549-a5a10ee9b6cc)

#### `PlaceViews(...)`

Places views on the sheet. Computes the required size of the views to fit the parts, then places the views so that they fit on the sheet and do not overlap. Adjusts the sheet size if needed.

[Docs](https://developer.tekla.com/topic/en/18/47/1e1578e4-084f-21a9-3d1a-2b03140440ce)

#### `Select(...)`

Selects the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/e47ae2a5-70bb-1ea5-b601-af7dee02c7d6)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

### Properties
- `CommitMessage` (object, get/set) — Gets the commit message
- `CreationDate` (object, get/set) — The drawing creation date.
- `DrawingTypeStr` (object, get/set) — The type of the drawing
- `IsFrozen` (object, get/set) — Describes whether the drawing is frozen.
- `IsIssued` (object, get/set) — Describes whether the drawing is issued.
- `IsIssuedButModified` (object, get/set) — Describes whether the drawing is issued but modified since it was issued.
- `IsLocked` (object, get/set) — Describes whether the drawing is locked.
- `IsLockedBy` (object, get/set) — Gets the logged in username that set the drawing to locked
- `IsMasterDrawing` (object, get/set) — Describes whether the drawing is a master drawing (shown with a special symbol on the drawing list).
- `IsReadyForIssue` (object, get/set) — Describes whether the drawing is ready for issue.
- `IsReadyForIssueBy` (object, get/set) — Gets the logged in username that set the drawing Ready for issue
- `IssuingDate` (object, get/set) — The drawing issuing date.
- `Layout` (object, get/set) — Gets the drawing layout attributes.
- `Mark` (object, get/set) — The drawing mark.
- `ModificationDate` (object, get/set) — The drawing modification date.
- `Name` (object, get/set) — The name of the drawing.
- `OutputDate` (object, get/set) — The drawing output date. XS_DISABLE_DRAWING_PLOT_DATE controls whether this date is set.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SectionViewStartLabel` (object, get/set) — Gets the drawing attribute for start label of section view
- `Title1` (object, get/set) — The first drawing title.
- `Title2` (object, get/set) — The second drawing title.
- `Title3` (object, get/set) — The third drawing title.
- `UpToDateStatus` (object, get/set) — Gets the drawing up to date status.

## NewLineElement (class)

The NewLineElement class defines a line feed between the desired elements to create multi-row marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/23d83e95-d85b-f88e-fea5-e05dabf2ddba)

### Constructors
- `NewLineElement(...)` — Initializes a new instance of the NewLineElement class

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/a51519a3-5dcd-cefe-2d19-9b39d166e895)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/eeef7e0e-64f0-a0ac-5971-fdf7830dacd4)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/61dd3ba4-f0f0-a376-9305-fa512aaddf20)

## NormalLineType (class)

The NormalLineType class defines the non-custom line types that can be used by the drawing objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c9aca42d-ba9e-3f0b-6a12-e49e2ea9ab3d)

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/21c01554-a250-befd-9455-5c733dfb448c)

#### `Equals(...)`

Returns true if the current object and the given object are equal.

[Docs](https://developer.tekla.com/topic/en/18/47/aa24fb70-37d2-a1f5-f78c-cca72c8e38bd)

#### `GetHashCode(...)`

Returns a hash code for the object.

[Docs](https://developer.tekla.com/topic/en/18/47/9f419242-f76c-753a-bab8-8ca3145178ab)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/89683344-0ce4-1b8c-1dd2-0122a8d8d67c)

#### `ToString(...)`

Returns the LineTypes information as a string.

[Docs](https://developer.tekla.com/topic/en/18/47/33edfbad-ab7c-406a-fec4-5e932e3ea8aa)

## OpenGraphicObject (class)

The OpenGraphicObject class is an abstract base class for all open graphic objects (the arc, the line and the polyline).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d8fbe75c-2a62-42eb-88f3-ad6f879d38d8)

### Methods
#### `Delete(...)`

Deletes the instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/f13a6c17-3cc9-d9cb-5393-7ef5795760c1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the object instance into the database. The inserted instance must be of a real type.

[Docs](https://developer.tekla.com/topic/en/18/47/020bf502-50ea-ba9f-bf32-a167db34fe82)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the instance in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/22491355-41a3-d4b9-ff88-842fe212aaf0)

#### `Select(...)`

Selects the object instance from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/44b1746f-df01-19ef-b98f-d577d5dd0435)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## OpenGraphicObject.OpenGraphicObjectAttributes (class)

The OpenGraphicObjectAttributes class is the attributes class of the open graphic object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c5d70471-238f-783d-4b33-c7dcb02251ce)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/da11fa4f-a98d-c770-d0e2-f458bc3b82dc)

#### `LoadAttributes(...)`

Loads the attributes from the specified file.

[Docs](https://developer.tekla.com/topic/en/18/47/5597a575-45e2-aa83-6576-390f23af3e48)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the open graphic object. The end points of an open graphic object can contain arrowheads.
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## Part (class)

The Part class contains methods related to parts. A part is a drawing object derived from a model object. It represents a drawing part that has a reference to the actual part in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/85faee90-0121-460f-f6fe-65d1f54a33d3)

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/c212a075-6030-fcde-99ad-6fca006f705f)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/14dab0e5-3935-9c07-8007-7a4851b7a8e7)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/3b5d86d1-728b-2e32-3188-2ebb793d03f8)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/c3c5d362-caef-6f42-756e-a0536e23cccc)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/440ce9f7-d175-ada6-626e-974336f53952)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the part. For more information see Part.PartAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Part.PartAttributes (class)

The PartAttributes class is the attributes class for the part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/08d50d99-aaab-1778-91d0-0deab85b3dff)

### Constructors
- `Part.PartAttributes(...)` — Creates a new default part attributes instance that loads standard attributes.
- `Part.PartAttributes(...)` — Creates a new part attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/471a1278-b484-0fca-5b96-d7755761df7a)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/1dfc4ac7-a2b3-bf89-87af-1fc429293cb0)

### Properties
- `CenterLine` (object, get/set) — The line type of the center line.
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `DrawCenterLine` (object, get/set) — True if the center line is drawn.
- `DrawChamfers` (object, get/set) — True if chamfers are drawn.
- `DrawConnectingSideMarks` (object, get/set) — True if connecting side marks are drawn.
- `DrawHiddenLines` (object, get/set) — True if hidden lines are drawn.
- `DrawOrientationMark` (object, get/set) — True if the orientation mark is drawn.
- `DrawOwnHiddenLines` (object, get/set) — True if own hidden lines are drawn.
- `DrawPopMarks` (object, get/set) — True if pop marks are drawn.
- `DrawReferenceLine` (object, get/set) — True if the reference line is drawn.
- `FaceHatch` (object, get/set) — The hatch type of the face hatch.
- `HiddenLines` (object, get/set) — The line type of hidden lines.
- `OwnHiddenLines` (object, get/set) — The line type of own hidden lines.
- `PartialProfileLength` (object, get/set) — Gets or sets the partial profile length.
- `PartialProfileOffset` (object, get/set) — Gets or sets the partial profile offset.
- `ReferenceLine` (object, get/set) — The line type of the reference line.
- `Representation` (object, get/set) — The representation of the part.
- `SectionFaceHatch` (object, get/set) — The hatch type of the section face hatch.
- `SectionLines` (object, get/set) — The line type of section lines.
- `SymbolOffset` (object, get/set) — The symbol offset.
- `VisibleLines` (object, get/set) — The line type of visible lines.

## Part.Representation (enum)

The part representations.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7c7b3c48-8df3-697d-d3d0-a3e06f4c467c)

### Values
- `Outline` = `1` — The outline representation.
- `Symbol` = `2` — The symbol representation.
- `WorkshopForm` = `4` — The workshop form representation.
- `Exact` = `8` — The exact representation.
- `BoundingBox` = `16` — The bounding box representation.
- `BaseBox` = `32` — The base box representation.
- `SolidWithPartialProfile` = `64` — The solid with partial profile representation.

## PickerInput (class)

The PickerInput class instances can be provided as predefined results of requests through the methods of the UI.Picker class.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/af20a192-0a70-c008-9821-8a87646a472e)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/1f670998-82b1-3842-37c0-deaa48e7fc95)

## PickerInputInterrupt (class)

The PickerInputInterrupt class represents the interruption of picking by the user (e.g. by pressing Escape while requested to pick points). Used for values asked by any UI.Picker.Pick* method where an interrupt happens by the user.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9294b17b-788d-fdf8-9238-bbd7d9b0c2d3)

### Constructors
- `PickerInputInterrupt(...)` — Creates a new PickerInputInterrupt instance representing the interruption of picking by the user.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/dec8c103-42e2-6d17-dc9b-0947f96a89a7)

## PickerInputNPoints (class)

The PickerInputNPoints class represents a pick of several points in a view. Used for values asked by UI.Picker.PickPoints(StringList Prompts, out PointList PickedPoints, out ViewBase PickedView) and UI.Picker.PickPoints(int NumberOfPicks, StringList Prompts, out PointList PickedPoints, out ViewBase PickedView).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3ffa5a86-d245-1c64-7261-2cff99d38314)

### Constructors
- `PickerInputNPoints(...)` — Creates a new PickerInputNPoints instance representing N points selected in a view.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/0a458ed1-3080-d293-0f63-3e6346bb244b)

### Properties
- `PickedPoints` (object, get/set) — Gets or sets the list of picked points in the view coordinates.
- `PickedView` (object, get/set) — The view within which the pick occurred.

## PickerInputObject (class)

The PickerInputObject class represents a pick of an object in a view. Used for values asked by UI.Picker.PickObject(string Prompt, out DrawingObject PickedObject, out ViewBase PickedView).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7a15973c-357f-775e-2360-55a2ae153fb6)

### Constructors
- `PickerInputObject(...)` — Creates a new PickerInputObject instance representing the pick of a drawing object in a view.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/875ad65f-678f-edae-4205-17361f71f304)

### Properties
- `PickedObject` (object, get/set) — Gets or sets the picked object in the view.
- `PickedView` (object, get/set) — The view within which the pick occurred.

## PickerInputObjectAndAPoint (class)

The PickerInputObjectAndAPoint class represents a pick of an object in a view. It provides both the picked object and the point used to pick the object. Used for values asked by UI.Picker.PickObject(string Prompt, out DrawingObject PickedObject, out ViewBase PickedView, out Point PickedPoint).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e112c940-87e5-a405-7aff-c753132e3fd2)

### Constructors
- `PickerInputObjectAndAPoint(...)` — Creates a new PickerInputObjectAndAPoint instance representing the pick of a drawing object and the coordinates of the point used to pick the object.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/239a901f-aec1-7b2a-f509-0cdf9bae0c16)

### Properties
- `PickedObject` (object, get/set) — Gets or sets the picked object in the view.
- `PickedPoint` (object, get/set) — Gets or sets the picked point in the view coordinates.
- `PickedView` (object, get/set) — The view within which the pick occurred.

## PickerInputPoint (class)

The PickerInputPoint class represents a pick of one point in a view. Used for values asked by UI.Picker.PickPoint(string Prompt, out Point PickedPoint, out ViewBase PickedView).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a0988e3c-a83d-a2b0-cd04-6a6b0228d72b)

### Constructors
- `PickerInputPoint(...)` — Creates a new PickerInputPoint instance with one point selected in a view.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/2e213537-2c72-6afb-177b-a04c8b70b27a)

### Properties
- `PickedPoint` (object, get/set) — The picked point in the view coordinates.
- `PickedView` (object, get/set) — The view within which the pick occurred.

## PickerInputPointsWithinAView (class)

The PickerInputPointsWithinAView class is the base class of PickerInput instances that represent a pick of one or more points in a view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2bc856da-4f9c-8c14-5cdc-dff8e131c667)

### Constructors
- `PickerInputPointsWithinAView(...)` — Creates a new PickerInputPointsWithinAView instance representing N points selected in a view.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/0a458ed1-3080-d293-0f63-3e6346bb244b)

### Properties
- `PickedView` (object, get/set) — The view within which the pick occurred.

## PickerInputThreePoints (class)

The PickerInputThreePoints class represents a pick of three points in a view. Used for values asked by UI.Picker.PickThreePoints(string FirstPrompt, string SecondPrompt, string ThirdPrompt, out Point FirstPickedPoint, out Point SecondPickedPoint, out Point ThirdPickedPoint, out ViewBase PickedView).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2629dcea-675a-313d-5374-4318480be672)

### Constructors
- `PickerInputThreePoints(...)` — Creates a new PickerInputThreePoints instance with one point selected in a view.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/27544458-03bd-3973-26f7-e0599800aa4e)

### Properties
- `FirstPickedPoint` (object, get/set) — The first picked point in the view coordinates.
- `PickedView` (object, get/set) — The view within which the pick occurred.
- `SecondPickedPoint` (object, get/set) — The second picked point in the view coordinates.
- `ThirdPickedPoint` (object, get/set) — The third picked point in the view coordinates.

## PickerInputTwoPoints (class)

The PickerInputTwoPoints class represents a pick of two points in a view. Used for values asked by UI.Picker.PickTwoPoints(string FirstPrompt, string SecondPrompt, out Point FirstPickedPoint, out Point SecondPickedPoint, out ViewBase PickedView).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8d4a2bd3-5128-a0f1-1ad1-b1ddeb70c717)

### Constructors
- `PickerInputTwoPoints(...)` — Creates a new PickerInputTwoPoints instance with two points selected in a view.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c5b40b5c-1721-87c1-3ebe-b9719f042bda)

### Properties
- `FirstPickedPoint` (object, get/set) — The first picked point in the view coordinates.
- `PickedView` (object, get/set) — The view within which the pick occurred.
- `SecondPickedPoint` (object, get/set) — The second picked point in the view coordinates.

## PickerInputWithinAView (class)

The PickerInputWithinAView class is the base class of PickerInput instances that represent a valid pick of point(s) or object(s) in a view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ddd636f6-9030-6f6f-cbaa-384cdf839526)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/5b0aca25-a5f4-0eca-8817-8b0e912a5229)

### Properties
- `PickedView` (object, get/set) — The view within which the pick occurred.

## PickerInterruptedException (class)

The PickerInterruptedException class defines the exception that is thrown when a user interrupts the picker.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a6d20ff1-fe2d-6ade-2eec-3a729b104222)

### Constructors
- `PickerInterruptedException(...)` — Creates a new picker interrupted exception instance.

## PlacingAttributes (class)

The PlacingAttributes class contains the basic attributes for determining an object's placing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/22655d18-229c-1261-e042-94b9a0bc6cd7)

### Constructors
- `PlacingAttributes(...)` — Creates a new placing attributes instance. The default parameters are: IsFixed = true, default PlacingDistanceAttributes, default PlacingQuarterAttributes.
- `PlacingAttributes(...)` — Creates a new placing quarter attributes instance with the given parameters.

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/8cd456cb-5f60-4c00-94fd-ce8f161a4c33)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/5788119b-9502-d9c7-a144-ca2179d1b643)

### Properties
- `IsFixed` (object, get/set) — Defines whether the object is using fixed or free placing. If set to true, it means that fixed placing is enabled, and the object will not be rearranged in the drawing automatically. If set to false, then free placing is enabled, and the object will be placed in the most appropriate location near the original location.
- `PlacingDistance` (object, get/set) — Gets or sets the placing distance attributes.
- `PlacingQuarter` (object, get/set) — Gets or sets the placing quarter attributes.

## PlacingBase (class)

The PlacingBase class is the base class for placing types. If you set a user defined Placing to an object it will override the PreferredPlacingType set in the object's attributes. Please note that not all objects accept all types. If you try to use the wrong type it will not be accepted (Insert, Modify will fail). (It is strongly recommended to use the PreferredPlacingTypes to assign new Placings to objects.)

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cef90e8e-aad5-4ce3-8efe-9d223a56b4e1)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/15f0b136-8c99-9174-4b69-90dcd3963d00)

## PlacingDirectionAttributes (class)

The PlacingDirectionAttributes class contains the basic attributes for placing directions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d1a2b9c5-012d-c737-abc6-d5b7bac840ff)

### Constructors
- `PlacingDirectionAttributes(...)` — Creates a new placing direction attributes instance. The positive and negative directions are set to false by default.
- `PlacingDirectionAttributes(...)` — Creates a new placing direction attributes instance.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/9505e7d9-339c-5f35-f0bb-e40e256106e9)

### Properties
- `Negative` (object, get/set) — The negative direction of placing.
- `Positive` (object, get/set) — The positive direction of placing.

## PlacingDistanceAttributes (class)

The PlacingDistanceAttributes class contains the basic attributes for placing distances.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/63b1eb62-5a40-d347-0a8a-46e21aa63e49)

### Constructors
- `PlacingDistanceAttributes(...)` — Initializes a new instance of the PlacingDistanceAttributes class. The default values are a 0.0 search margin and a 0.0 minimal distance.
- `PlacingDistanceAttributes(...)` — Creates a new placing distance attributes instance with the given search marginal and minimal distance.
- `PlacingDistanceAttributes(...)` — Creates a new placing distance attributes instance with the given search margin, minimal and maximal distance.

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/b6d8d2ac-5201-c26c-4db8-c01199cdcdd4)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/f6c990a4-f8d1-b02b-2b83-2f794879511e)

### Properties
- `MaximalDistance` (object, get/set) — Gets or sets the maximal distance for placing.
- `MinimalDistance` (object, get/set) — Gets or sets the minimal distance for placing.
- `SearchMargin` (object, get/set) — Gets or sets the search marginal for placing.

## PlacingQuarterAttributes (class)

The PlacingQuarterAttributes class contains the basic attributes for placing quarters.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c71a59d4-6836-e9f2-a51e-09c64d4ed72d)

### Constructors
- `PlacingQuarterAttributes(...)` — Creates a new placing quarter attributes instance. The default parameters are: TopLeft = true, TopRight = true, BottomLeft = true, BottomRight = true.
- `PlacingQuarterAttributes(...)` — Creates a new placing quarter attributes instance with the given parameters.

### Methods
#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/3528770e-cd45-9205-73d7-b00c3e84a270)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c39a275d-9cb2-f970-34d2-f2fcf3beacb2)

### Properties
- `BottomLeft` (object, get/set) — Defines whether the object can search for a suitable location in the bottom left direction.
- `BottomRight` (object, get/set) — Defines whether the object can search for a suitable location in the bottom right direction.
- `TopLeft` (object, get/set) — Defines whether the object can search for a suitable location in the top left direction.
- `TopRight` (object, get/set) — Defines whether the object can search for a suitable location in the top right direction.

## PlacingTypes (class)

The PlacingTypes class defines the different available placing types. Please note that not all objects accept all types. If you try to use the wrong type it will not be accepted (Insert, Modify will fail).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/38d18df2-bc23-3978-7a9f-cb48e7d91ae3)

### Methods
#### `AlongLinePlacing(...)`

The object is located and moved along a line defined by two points.

[Docs](https://developer.tekla.com/topic/en/18/47/0a360cc9-c148-aba4-f799-70c87d9cee9d)

#### `BaseLinePlacing(...)`

The object is placed using a line as a reference. This will thus define the object's X-axis along that line.

[Docs](https://developer.tekla.com/topic/en/18/47/350d8df2-b3ab-8e63-0a45-5e25d7eef0a5)

#### `LeaderLinePlacing(...)`

The object is placed attached to a leader line.

[Docs](https://developer.tekla.com/topic/en/18/47/db02754b-9b64-ac06-1603-6ed790b6d33f)

#### `PointPlacing(...)`

The object is placed at one point (the insertion point) without any additional placing options.

[Docs](https://developer.tekla.com/topic/en/18/47/a01a7987-8b62-391d-46ce-1746577b2c25)

## Plugin (class)

The Plugin class represents a drawing plug-in.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4b3cdf4c-0872-f97b-d56e-5209179f1fca)

### Constructors
- `Plugin(...)` — Creates a new plug-in instance.

### Methods
#### `ApplyStandardValues(...)`

Applies the values of the specified file as the default values for the dialog belonging to this plug-in. This will affect each Insert call of the plug-ins with the same name.

[Docs](https://developer.tekla.com/topic/en/18/47/6c51b9d0-7066-d66d-27cd-9d428d3d1d72)

#### `Delete(...)`

Deletes the plug-in from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/84960371-6f53-b251-560c-d1a016ff3245)

#### `GetDoubleAttribute(...)`

Gets the value of the specified double attribute of the plug-in.

[Docs](https://developer.tekla.com/topic/en/18/47/8647b360-ba26-7611-8cd0-2050ad19102b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntAttribute(...)`

Gets the value of the specified int attribute of the plug-in.

[Docs](https://developer.tekla.com/topic/en/18/47/a5b6c7c9-984f-0a3a-5d3b-fbbfd3294aa2)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetPluginInput(...)`

Retrieves the input defined by the plug-in after insertion.

[Docs](https://developer.tekla.com/topic/en/18/47/88ed2c7f-ecf5-d0d9-eade-1086903a24d2)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringAttribute(...)`

Gets the value of the specified string attribute of the plug-in.

[Docs](https://developer.tekla.com/topic/en/18/47/72af6d07-6045-1a47-376f-e03201f45b8e)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the plug-in into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/b0e118a0-f4da-3596-c616-08a5eef1a456)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e78bf0b7-77d4-d79d-221d-40ecc01d135e)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `LoadStandardValues(...)`

Sets the values from the specified file to this plug-in.

[Docs](https://developer.tekla.com/topic/en/18/47/eed2c1f4-ccc7-2fbb-0a37-f060a84bdd24)

#### `Modify(...)`

Modifies the plug-in in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/3bae715a-a26b-6c5b-c595-ab84a1d09c38)

#### `Select(...)`

Selects the plug-in from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/a891f7b9-5cbf-385f-fbba-470a10659e73)

#### `SetAttribute(String, Double)(...)`

Sets the specified attribute of the plug-in to the given value. NOTE! Only works on already inserted plug-ins.

[Docs](https://developer.tekla.com/topic/en/18/47/f105a4d9-b048-d920-d62f-9e7aa942b851)

#### `SetAttribute(String, Int32)(...)`

Sets the specified attribute of the plug-in to the given value. NOTE! Only works on already inserted plug-ins.

[Docs](https://developer.tekla.com/topic/en/18/47/65959646-4ec3-c048-f3c7-19baf38dd810)

#### `SetAttribute(String, String)(...)`

Sets the specified attribute of the plug-in to the given value. NOTE! Only works on already inserted plug-ins.

[Docs](https://developer.tekla.com/topic/en/18/47/9cc2e355-1a58-ca88-6375-6d86b8bf01da)

#### `SetPickerInput(...)`

Defines the sequence of inputs to use in place of interactive picks when the plug-in is inserted to a drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/37bc4b37-0684-8e0f-1792-9712b732790e)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

#### `TryGetAttribute(String, Double.)(...)`

Gets the value of the specified double attribute of the plug-in.

[Docs](https://developer.tekla.com/topic/en/18/47/90f39e86-2f82-c971-61af-61736228eb01)

#### `TryGetAttribute(String, Int32.)(...)`

Gets the value of the specified int attribute of the plug-in.

[Docs](https://developer.tekla.com/topic/en/18/47/136b2adb-3657-772c-ba38-f6fce33d1b50)

#### `TryGetAttribute(String, String.)(...)`

Gets the value of the specified string attribute of the plug-in.

[Docs](https://developer.tekla.com/topic/en/18/47/ba108b08-e074-f5c1-beb1-b8ad48eba1b3)

#### `TrySetAttribute(String, Double)(...)`

Sets the specified attribute of the plug-in to the given value. NOTE! Only works on already inserted plug-ins.

[Docs](https://developer.tekla.com/topic/en/18/47/63d39896-bda4-c016-124d-51178e81773e)

#### `TrySetAttribute(String, Int32)(...)`

Sets the specified attribute of the plug-in to the given value. NOTE! Only works on already inserted plug-ins.

[Docs](https://developer.tekla.com/topic/en/18/47/c50b08a0-129e-d671-adf3-40ae248dc941)

#### `TrySetAttribute(String, String)(...)`

Sets the specified attribute of the plug-in to the given value. NOTE! Only works on already inserted plug-ins.

[Docs](https://developer.tekla.com/topic/en/18/47/f0d5baa8-6f9c-d8fd-103e-ebfbdefb4fb0)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `Name` (object, get/set) — Gets the name of the plug-in.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## PluginPickerInput (class)

The PluginPickerInput class represents a predefined set of picker inputs that can be provided to a plug-in object. It replaces the interactive selection of points and/or objects by the user when the plug-in is inserted into a drawing through the API.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1fbe56ff-0633-90a5-ef51-20307ee497cb)

### Constructors
- `PluginPickerInput(...)` — Initializes a new instance of the PluginPickerInput class with an empty list of picker inputs.
- `PluginPickerInput(...)` — Initializes a new instance of the PluginPickerInput class which contains a predefined sequence of inputs.

### Methods
#### `Add(...)`

Appends a picker input to the plug-in input.

[Docs](https://developer.tekla.com/topic/en/18/47/2e8405d4-bef2-bbf7-c056-e1f6c63c8352)

#### `AddRange(...)`

Appends several picker inputs to the picker input sequence.

[Docs](https://developer.tekla.com/topic/en/18/47/e5b2b352-1609-2d51-f946-15a3d19593a8)

## PointList (class)

The PointList class defines a type safe point list.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/46ef3374-513f-103b-3d95-13a94c678f33)

### Constructors
- `PointList(...)` — Initializes a new instance of the PointList class

### Methods
#### `Add(...)`

Adds a new point to the end of the point list.

[Docs](https://developer.tekla.com/topic/en/18/47/0a85dfdc-b822-8a8f-e229-363ef592a753)

#### `AddRange(...)`

Appends all points from another point list to the end of this point list.

[Docs](https://developer.tekla.com/topic/en/18/47/99fd5ef6-df1d-d958-1935-606fee669fab)

#### `Contains(...)`

Checks if the given point is in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/e3ffabed-f260-de57-7187-1052ca293554)

#### `GetRange(...)`

Gets a part of the point list.

[Docs](https://developer.tekla.com/topic/en/18/47/5f47ab27-bd7e-8777-dae0-2c3e9cb12ed6)

#### `IndexOf(Point)(...)`

Gets the index of the specified point.

[Docs](https://developer.tekla.com/topic/en/18/47/275ccccd-031e-39e6-2e60-b3335e1aea1d)

#### `IndexOf(Point, Int32)(...)`

Gets the index of the specified point between the index to start the search from and the end of the list.

[Docs](https://developer.tekla.com/topic/en/18/47/71ec7df3-2212-b7f7-05eb-a1fd3eed7b9c)

#### `IndexOf(Point, Int32, Int32)(...)`

Gets the index of the specified point between the index to start the search from and startIndex + count.

[Docs](https://developer.tekla.com/topic/en/18/47/b1382631-8c95-1737-6763-124469f375f7)

#### `Insert(...)`

Inserts the point to the specified index.

[Docs](https://developer.tekla.com/topic/en/18/47/74fe90d5-80cf-09d3-5a76-e136dfe6277e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/862def26-cc3f-96a8-629e-892bf2833081)

#### `LastIndexOf(Point)(...)`

Searches for the last occurence of the point in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/4e7b3fe7-4ed2-7d42-b3d2-933d9450e458)

#### `LastIndexOf(Point, Int32)(...)`

Searches for the last occurence of the point in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/2d941eaf-2701-40aa-61f3-4390403322ab)

#### `LastIndexOf(Point, Int32, Int32)(...)`

Searches for the last occurence of the point in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/6d9bd143-8a6e-c3f8-4763-a97b3052057e)

#### `Remove(...)`

Removes the first occurence of the point from the list.

[Docs](https://developer.tekla.com/topic/en/18/47/7742080a-5946-fc98-472b-25baddad1e51)

#### `RemoveRange(...)`

Removes a range of items starting from the given index.

[Docs](https://developer.tekla.com/topic/en/18/47/53f1d876-8c57-2b85-a646-25903eff3636)

#### `ToArray(...)`

Copies the points to a point array.

[Docs](https://developer.tekla.com/topic/en/18/47/5a62ed18-25a0-73ff-ab35-2044f5a47004)

### Properties
- `Item` (object, get/set) — Gets or sets the point at the specified index.

## PointPlacing (class)

The PointPlacing class defines a placing where the object is placed at one point (the insertion point) without any additional placing options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2b95dd6f-0941-805a-9f6b-07976e857327)

### Constructors
- `PointPlacing(...)` — Creates a new point placing instance. The value used for the placing is the parent object's insertion point in this case.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/be2242f1-aa49-fbdb-fc2e-ac7b35b4f910)

## PointPlacingType (class)

The PointPlacingType class defines a placing type where the object is placed at one point (the insertion point) without any additional placing options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a6e0ca36-fefa-9b75-fd24-f764eab84ec0)

### Constructors
- `PointPlacingType(...)` — Creates a new point placing type instance.

## Polygon (class)

The Polygon class defines a polygon that is a multipoint line which is connected. It can also have a bulge (curved lines).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f171c5a5-0d65-7d63-7a2d-0d8428e631f8)

### Constructors
- `Polygon(...)` — Creates a new polygon instance with the point list using standard attributes.
- `Polygon(...)` — Creates a new polygon instance with the point list using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/8c7f3928-e132-9c57-e19d-2cc34136bff1)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/55938604-040a-7876-68cf-83d0d3cdde98)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/65f33b85-15a7-c49f-0665-2a225d828d0c)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/3aa93603-d749-5e86-4b7c-e22f9ab0bd64)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/e541de4f-9464-df87-31df-644f422a25a5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the polygon.
- `Bulge` (object, get/set) — The bulge (curve) to use for the polygon (a bulge is the ratio between the height and width of an object).
- `Bulges` (object, get/set) — The list of individual bulges for the polygon.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `Points` (object, get/set) — The list of points for the polygon.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Polygon.PolygonAttributes (class)

The PolygonAttributes class is the attributes class of the polygon.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/562f0e90-6ddc-aa05-5066-0a8e0e80742c)

### Constructors
- `Polygon.PolygonAttributes(...)` — Creates a new default polygon attributes instance that loads standard attributes.
- `Polygon.PolygonAttributes(...)` — Creates a new polygon attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/911a092e-34a8-d822-fc21-ffda11bcd033)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/1a4be117-bc05-ff96-b7a7-b26a0989425f)

### Properties
- `BehindModelObjects` (object, get/set) — Gets or sets whether the closed graphic object is drawn behind model objects
- `Hatch` (object, get/set) — The hatch attributes of the closed graphic object (the closed graphic object is hatchable).
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## Polyline (class)

The Polyline class defines a polyline that is a multipoint line. It can also have a bulge (curved lines).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/028cf6a3-19e6-0c68-13c6-1668b0d42b2a)

### Constructors
- `Polyline(...)` — Creates a new polyline instance with the point list using standard attributes.
- `Polyline(...)` — Creates a new polyline instance with the point list using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/01d06297-5791-dcbd-6c4f-292a123479c7)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/ce2541e9-896e-dea2-a975-d3957f4f8262)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/9a1d36c8-594b-4d55-d0e9-95a36a8d0bc0)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/6739e2ec-0163-74e2-73f0-f9ca6184f173)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/0095c0f9-9d44-c772-28d0-179d093086b9)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the polyline.
- `Bulge` (object, get/set) — The bulge (curve) to use for the polyline (a bulge is the ratio between the height and width of an object).
- `Bulges` (object, get/set) — The list of individual bulges for the polygon.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `Points` (object, get/set) — The list of points for the polyline.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Polyline.PolylineAttributes (class)

The PolylineAttributes class is the attributes class of the polyline.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8682dcd0-7945-40e4-ebbd-54033e315020)

### Constructors
- `Polyline.PolylineAttributes(...)` — Creates a new default polyline attributes instance that loads standard attributes.
- `Polyline.PolylineAttributes(...)` — Creates a new polyline attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/404136cb-bdc9-a35f-5793-d0412c18b305)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/cb4bb2c3-a745-825c-37b5-446de54da1cf)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the open graphic object. The end points of an open graphic object can contain arrowheads.
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## PourBreak (class)

The PourBreak class contains methods related to pour breaks. A pour break is a drawing object derived from a model object. It represents a drawing pour break that has a reference to the actual pour break in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2edc2f18-1091-31bb-9a18-c68a6023cbb6)

### Methods
#### `Delete(...)`

Deletes the drawing pour break from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/a2da9872-7a7e-665e-3dee-deb784bac633)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/cc4a52ca-845a-5e33-9121-1a02c8eccce2)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/8b21dbd6-a0fe-2d7e-efac-b798e90dec9c)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing pour break.

[Docs](https://developer.tekla.com/topic/en/18/47/c47bca17-822d-3e0e-f71e-6d469dfede14)

#### `Select(...)`

Selects the drawing pour break from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/39f525b1-051e-590d-6a27-2008bd2864b5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the pour break. For more information see PourBreak.PourBreakAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## PourBreak.PourBreakAttributes (class)

The PourBreakAttributes class is the attributes class for the pour break.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4fa0263e-e4ce-f9b7-70f4-2da5d9551389)

### Constructors
- `PourBreak.PourBreakAttributes(...)` — Creates a new default pour break attributes instance that loads standard attributes.
- `PourBreak.PourBreakAttributes(...)` — Creates a new pour break attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/b994540c-2208-45a6-70d7-2f5fee66233d)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/50ff8c5e-d2a4-3fad-ed02-c8f4e9cc691b)

### Properties
- `DrawHiddenLines` (object, get/set) — Indicates whether hidden lines are used to draw the pour break or not.
- `HiddenLines` (object, get/set) — The hidden line type of the pour break.
- `VisibleLines` (object, get/set) — The visible line type of the pour break.

## PourObject (class)

The PourObject class contains methods related to pours. A pour is a drawing object derived from a model object. It represents a drawing pour that has a reference to the actual pour in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a58d5b8e-3427-6697-8087-f549172008ce)

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/fc96d962-ef43-320b-4e61-aa2431b5d9b5)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/c921dbe6-8e41-1c16-bf2b-f30e003a634e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e4b9d653-77f5-bc0a-081c-d2e1f8c833fe)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/84f08d85-b6cb-0b6c-c830-17580415c3c0)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/18e70902-d622-606e-70a4-07b6ff928176)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the pour. For more information see PourObject.PourAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## PourObject.PourAttributes (class)

The PourAttributes class is the attributes class for the pour.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5b4159f0-d8d8-3643-a751-bc4a891d8816)

### Constructors
- `PourObject.PourAttributes(...)` — Creates a new default pour attributes instance that loads standard attributes.
- `PourObject.PourAttributes(...)` — Creates a new pour attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/11186dbb-413d-d09a-a4fe-e5a4173f2644)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/c2ca65bc-9ea4-e06c-494a-f22dbcd642e9)

### Properties
- `DrawChamfers` (object, get/set) — True if chamfers are drawn.
- `DrawHiddenLines` (object, get/set) — True if hidden lines are drawn.
- `DrawOwnHiddenLines` (object, get/set) — True if own hidden lines are drawn.
- `FaceHatch` (object, get/set) — The hatch type of the face hatch.
- `HiddenLines` (object, get/set) — The line type of hidden lines.
- `OwnHiddenLines` (object, get/set) — The line type of own hidden lines.
- `SectionFaceHatch` (object, get/set) — The hatch type of the section face hatch.
- `SectionLines` (object, get/set) — The line type of section lines.
- `VisibleLines` (object, get/set) — The line type of visible lines.

## PreferredMarkPlacingTypes (class)

The PreferredMarkPlacingTypes class contains the available preferred mark placing types. These are allowed to be used for marks. However, there are different mark types and not all types can be used by all marks. If you try to use the wrong type it will not be accepted (Insert, Modify will fail).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8e8d4861-e6de-1baf-ba97-9ee03154eed1)

### Methods
#### `AlongLineOrWithLeaderLineAndParentObjectAlongPartPlacingType(...)`

The object is placed using the along line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line and rotated to be at the same angle as the connected part.

[Docs](https://developer.tekla.com/topic/en/18/47/6d3540d9-24e3-6269-a47b-001bf7d8bc4f)

#### `AlongLineOrWithLeaderLinePlacingType(...)`

The object is placed using the along line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line.

[Docs](https://developer.tekla.com/topic/en/18/47/1601449f-48f3-fe2b-3370-7a3754f21808)

#### `AlongLinePlacingType(...)`

The object is placed using the along line placing.

[Docs](https://developer.tekla.com/topic/en/18/47/5f098f43-2054-cd59-c87d-4e1b4f870dee)

#### `AlongPartCenteredPlacingType(...)`

The object is placed using the along line placing. The line used will be the part's edge in the center of that line.

[Docs](https://developer.tekla.com/topic/en/18/47/a972a34f-74e0-7329-39e3-c9d1370159fd)

#### `InsidePartAlongPartOrWithLeaderLinePlacingType(...)`

The object is placed using the base line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line. The base line used is in the middle of the part, aligned with the part.

[Docs](https://developer.tekla.com/topic/en/18/47/376611d0-8168-9b78-18f4-4d2a822964d8)

#### `InsidePartAlongPartPlacingType(...)`

The object is placed using the base line placing. The base line used is in the middle of the part, aligned with the part.

[Docs](https://developer.tekla.com/topic/en/18/47/df64f61b-ae1e-b113-57bd-64dbf50c7266)

#### `InsidePartHorizontalOrWithLeaderLinePlacingType(...)`

The object is placed using the base line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line. The base line used is in the middle of the part, aligned with the part. In addition the object will be rotated to be horizontal in the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/1eb7852e-30f6-18b3-5a18-c04ede39612c)

#### `InsidePartHorizontalPlacingType(...)`

The object is placed using the base line placing. The base line used is in the middle of the part, aligned with the part. In addition the object will be rotated to be horizontal in the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/d10771d3-a18f-4a93-0d8f-fe215dad4d4e)

#### `LeaderLineAndParentObjectAlongPartPlacingType(...)`

The object is placed with a leader line and rotated to be at the same angle as the connected part.

[Docs](https://developer.tekla.com/topic/en/18/47/9ee993af-0f5b-196f-30da-475c8ad0f470)

#### `LeaderLinePlacingType(...)`

The object is placed with a leader line.

[Docs](https://developer.tekla.com/topic/en/18/47/0b63f11c-400b-e9ce-ad61-3211f61e4c03)

#### `PointPlacingType(...)`

The object is placed at one point (the insertion point) without any additional placing options.

[Docs](https://developer.tekla.com/topic/en/18/47/67efdc2d-3657-5658-5c5c-c58d93d569a0)

## PreferredPlacingTypeBase (class)

The PreferredPlacingTypeBase class is the base class for the preferred placing type. You can set the PreferredPlacingType in the object's attributes to specify how your object should be placed in the drawing. Please note that not all objects accept all types. If you try to use the wrong type it will not be accepted (Insert, Modify will fail). To use valid types you should use the named defaults (PreferredTextPlacingTypes for Text, PreferredMarkPlacingTypes for Mark, PreferredSymbolPlacingTypes for Symbol).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b87c3562-b4be-a343-1589-2b1dd093c416)

## PreferredPlacingTypes (class)

The PreferredPlacingTypes class contains all the available preferred placing types. Please note that not all objects accept all types. If you try to use the wrong type it will not be accepted (Insert, Modify will fail).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3a23c62f-126c-7cf8-6fc8-dd677ef0e771)

### Methods
#### `AlongLineOrWithLeaderLineAndParentObjectAlongPartPlacingType(...)`

The object is placed using the along line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line and rotated to be at the same angle as the connected part.

[Docs](https://developer.tekla.com/topic/en/18/47/bee6a26b-0753-ce5e-6094-2c2ab92af624)

#### `AlongLineOrWithLeaderLinePlacingType(...)`

The object is placed using the along line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line.

[Docs](https://developer.tekla.com/topic/en/18/47/5c0ba66e-4043-2ba5-21e7-42b7f8d97f66)

#### `AlongLinePlacingType(...)`

The object is placed using the along line placing.

[Docs](https://developer.tekla.com/topic/en/18/47/209f1964-7118-c59b-9d46-16c0857a0d6f)

#### `AlongPartCenteredPlacingType(...)`

The object is placed using the along line placing. The line used will be the part's edge in the center of that line.

[Docs](https://developer.tekla.com/topic/en/18/47/4541a2da-3a77-d728-12f8-79577e4de568)

#### `BaseLinePlacingType(...)`

The object is placed using the base line placing.

[Docs](https://developer.tekla.com/topic/en/18/47/d371f81f-ae53-4a09-151a-15029ae269e1)

#### `BaseLineWithArrowAtEndPointPlacingType(...)`

The object is placed using the base line placing. The line is drawn and will have an arrow at the end point.

[Docs](https://developer.tekla.com/topic/en/18/47/9a715018-be1b-2ead-04f2-e9a4767617d1)

#### `BaseLineWithArrowAtStartPointPlacingType(...)`

The object is placed using the base line placing. The line is drawn and will have an arrow at the start point.

[Docs](https://developer.tekla.com/topic/en/18/47/a36b8468-3ccd-2124-730b-74dcc5b7e525)

#### `InsidePartAlongPartOrWithLeaderLinePlacingType(...)`

The object is placed using the base line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line. The base line used is in the middle of the part, aligned with the part.

[Docs](https://developer.tekla.com/topic/en/18/47/5cc32179-76b9-c9b0-a698-3aa793e708ff)

#### `InsidePartAlongPartPlacingType(...)`

The object is placed using the base line placing. The base line used is in the middle of the part, aligned with the part.

[Docs](https://developer.tekla.com/topic/en/18/47/dd4622b7-338d-4a27-3216-9ece3e1bf71a)

#### `InsidePartHorizontalOrWithLeaderLinePlacingType(...)`

The object is placed using the base line placing as the first option. If there is no room for the first option the fallback option will be used. The fallback option is placing the object with a leader line. The base line used is in the middle of the part, aligned with the part. In addition the object will be rotated to be horizontal in the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/05d26601-283a-3f96-f09a-be94c66cd378)

#### `InsidePartHorizontalPlacingType(...)`

The object is placed using the base line placing. The base line used is in the middle of the part, aligned with the part. In addition the object will be rotated to be horizontal in the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/6cad7a00-579c-2841-df66-524041167410)

#### `LeaderLineAndParentObjectAlongPartPlacingType(...)`

The object is placed with a leader line and rotated to be at the same angle as the connected part.

[Docs](https://developer.tekla.com/topic/en/18/47/578f0a0c-be70-c95b-a244-c170facfaea9)

#### `LeaderLinePlacingType(...)`

The object is placed with a leader line.

[Docs](https://developer.tekla.com/topic/en/18/47/965de8e1-bf3c-0b5e-a0d4-05882aefd9f6)

#### `PointPlacingType(...)`

The object is placed at one point (the insertion point) without any additional placing options.

[Docs](https://developer.tekla.com/topic/en/18/47/f2ded56b-313b-6507-9f9a-f704858f9ffa)

## PreferredSymbolPlacingTypes (class)

The PreferredSymbolPlacingTypes class contains the available preferred symbol placing types. These are allowed to be used for symbols.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c5a4ef62-4d55-0c45-dd84-a1195de6c4ab)

### Methods
#### `AlongLinePlacingType(...)`

The object is placed using the along line placing.

[Docs](https://developer.tekla.com/topic/en/18/47/49eecfdf-0020-ec6e-1c64-f56f000450df)

#### `LeaderLinePlacingType(...)`

The object is placed with a leader line.

[Docs](https://developer.tekla.com/topic/en/18/47/bf6e2c24-f843-b1f9-b78c-c12464250f15)

#### `PointPlacingType(...)`

The object is placed at one point (the insertion point) without any additional placing options.

[Docs](https://developer.tekla.com/topic/en/18/47/e175ecf4-37c0-2e1c-ca51-eb68fa11a7da)

## PreferredTextPlacingTypes (class)

The PreferredTextPlacingTypes class contains the available preferred text placing types. These are allowed to be used for texts.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0bc47111-6564-81c1-ad98-fc364b505948)

### Methods
#### `AlongLinePlacingType(...)`

The object is placed using the along line placing.

[Docs](https://developer.tekla.com/topic/en/18/47/9ead849f-1453-42a5-0f13-5e9f8df3ac74)

#### `BaseLinePlacingType(...)`

The object is placed using the base line placing.

[Docs](https://developer.tekla.com/topic/en/18/47/2b53974c-ff30-db37-1ba7-52e9c809fb38)

#### `BaseLineWithArrowAtEndPointPlacingType(...)`

The object is placed using the base line placing. The line is drawn and will have an arrow at the end point.

[Docs](https://developer.tekla.com/topic/en/18/47/b92c2238-7348-9bb3-75ff-2387f54b4bbe)

#### `BaseLineWithArrowAtStartPointPlacingType(...)`

The object is placed using the base line placing. The line is drawn and will have an arrow at the start point.

[Docs](https://developer.tekla.com/topic/en/18/47/ee8b2d0d-35bb-9776-0997-a63d6ce56fce)

#### `LeaderLinePlacingType(...)`

The object is placed with a leader line.

[Docs](https://developer.tekla.com/topic/en/18/47/d4eda65a-3cb9-3867-d7ce-14a885a7a730)

#### `PointPlacingType(...)`

The object is placed at one point (the insertion point) without any additional placing options.

[Docs](https://developer.tekla.com/topic/en/18/47/bf401d7e-1f2c-f14b-b6e6-717c99b594c4)

## PrintAttributes (class)

The PrintAttributes class contains the attributes for controlling the printing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1cba3ab8-d44e-01ee-d0f2-fbab61eb2a3d)

### Constructors
- `PrintAttributes(...)` — Creates a new PrintAttributes instance.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4e37a7ae-9c14-2009-7793-1a9928692792)

### Properties
- `NumberOfCopies` (object, get/set) — The number of copies in printing.
- `Orientation` (object, get/set) — The print orientation type.
- `PrintArea` (object, get/set) — The print area type.
- `PrinterInstance` (object, get/set) — The name of the printer instance. Can be fetched from the printer catalog.
- `PrintToMultipleSheet` (object, get/set) — Whether to print to multiple sheets.
- `Scale` (object, get/set) — The scale factor.
- `ScalingType` (object, get/set) — The scale type.

## PropertyElement (class)

The PropertyElement class may represent any of the many different mark element types, such as the profile, the material and the name. The property is defined by the name and it is converted into a textual value before the mark is drawn.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/91af96df-1eb0-e9ac-31c9-33a3bd2abbdc)

### Constructors
- `PropertyElement(...)` — Creates a new property element instance with the given property element type.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/a3b5281a-031c-19d2-f42b-65a64fe2d6db)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/825d769f-e545-be31-92b9-85a86bb9a3ac)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/db24463c-811c-73fb-56bf-2fa2c4b07f34)

### Properties
- `Font` (object, get/set) — The font of the property.
- `Name` (object, get/set) — The property name.
- `PropertyType` (object, get/set) — The type of the property element.
- `Value` (object, get/set) — The current value of the property.

## PropertyElement.PropertyElementType (class)

The PropertyElementType class represents the type of a property element.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/941ed10f-6559-4a0b-08c3-cbcd3d7ae447)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/983806d4-2c2a-7a71-8ae0-5d28bfae9edc)

## PropertyElement.PropertyElementType.BoltMarkPropertyElementTypes (class)

The BoltMarkPropertyElementTypes class represents the available property element types for bolt marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d36be319-423a-418a-3c8a-1f543e775621)

### Methods
#### `AssemblyType(...)`

Returns the assembly type property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/fa58b9ca-8a4c-431f-2e41-76d37565dc4d)

#### `BoltDiameter(...)`

Returns the bolt diameter property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/90e0b789-a192-6099-6c3b-3e73b1b76c2c)

#### `BoltLength(...)`

Returns the bolt length property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/360c3c18-1583-c92e-7a7f-7423a41b20fc)

#### `CenterToCenterDistance(...)`

Returns the center-to-center distance property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/c1cfe04f-975b-26e0-ea8a-ba11f34a7c80)

#### `Countersunk(...)`

Returns the countersunk property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/967ab431-a43f-b22c-20b9-8a277273874a)

#### `FullName(...)`

Returns the full name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/cb2cb655-5c21-e6e9-8d57-f362e0bde84b)

#### `GageOfOutstandingLeg(...)`

Returns the gage of the outstanding leg property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/c5f92a8f-2229-4c48-50bf-4f471fa21c10)

#### `HoleDepth(...)`

Returns the hole depth property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/aad3fee4-cd74-0635-4547-f20caca809a1)

#### `HoleDiameter(...)`

Returns the hole diameter property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/9282d00e-0091-af22-bf04-2870117afc93)

#### `Material(...)`

Returns the material property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/81ac6787-2693-8077-ec78-cafc3a8291d8)

#### `NumberOfBolts(...)`

Returns the number of bolts property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/5604c3f6-8ed6-b1fc-49e7-29a44f19c9aa)

#### `ShortName(...)`

Returns the short name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/d0af3a94-0fc2-5a2b-5a2f-16577c5f4a82)

#### `Size(...)`

Returns the size property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/e17d5d99-83fe-d50b-f8f8-b06f2be7d0d3)

#### `SlotHeight(...)`

Returns the slot height property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/5d8a5e44-df26-9629-3680-14a15d35f95a)

#### `SlotLength(...)`

Returns the slot length property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/d61f76ec-e763-e674-cdea-4bcc9973fb3c)

#### `SlotLengthX(...)`

Returns the slot length in the X direction property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/2cea2514-1c06-56dd-890c-811ebd24c9dc)

#### `SlotLengthY(...)`

Returns the slot length in the Y direction property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/1eedc4a3-fd20-83b4-82f9-c66a382b308a)

#### `Standard(...)`

Returns the standard property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/80542dfa-937c-2fd5-c16c-41e17f6c6e48)

## PropertyElement.PropertyElementType.ChamferMarkPropertyElementTypes (class)

The ChamferMarkPropertyElementTypes class represents the available property element types for chamfer marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/231f44a3-29c7-5980-9df0-29f5296d5987)

### Methods
#### `DX(...)`

Returns the change in the X direction property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/3e6d779e-10b6-33c1-df38-d198f6b996a9)

#### `DY(...)`

Returns the change in the Y direction property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/2069df56-220d-7226-f530-32218e664b54)

#### `Length(...)`

Returns the length property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/f81511c6-d604-52e2-aa1c-4f657fbe20ec)

#### `Name(...)`

Returns the name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/3d9eff2b-955a-6ac2-d9ab-ee5667787f28)

## PropertyElement.PropertyElementType.ConnectionMarkPropertyElementTypes (class)

The ConnectionMarkPropertyElementTypes class represents the available property element types for connection marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f89115c8-c097-a5c8-04aa-391b320e0464)

### Methods
#### `Code(...)`

Returns the code property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/8d2376c6-6901-71a9-ba28-27f292306f7e)

#### `ConnectionError(...)`

Returns the connection error property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/6f8bdf3b-4d59-8fe0-1d7e-fe4e9a71da8c)

#### `ConnectionNumber(...)`

Returns the connection number property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/741b1f03-5c3a-ddcb-1717-06ff94a530cf)

#### `DSTVCode(...)`

Returns the DSTV code property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/bf801068-1b27-64b9-330a-58fc573599a3)

#### `Group(...)`

Returns the group property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/088f22c9-5c1a-5c16-1265-9aabaf1ae15c)

#### `Name(...)`

Returns the name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/6eba9c44-0042-f5f2-9ca6-6c9c4901c5dd)

#### `RunningNumber(...)`

Returns the running number property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/6edb1485-4fa1-af78-6cce-6f6ff5b9f89b)

## PropertyElement.PropertyElementType.DetailMarkPropertyElementTypes (class)

The DetailMarkPropertyElementTypes class represents the available property element types for detail marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a1e6ee33-224e-ebd1-9bcf-d6f0188d8ca9)

### Methods
#### `DetailName(...)`

Returns the detail name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7373f53a-629c-b92a-9cd5-f67b18334854)

#### `SourceDrawingName(...)`

Returns the source drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/745bc5f2-3d1f-fadd-f39a-10a0752bfaeb)

#### `SourceDrawingNameWhenMoved(...)`

Returns the source drawing name when moved property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7035319d-1e22-adc1-3a62-1815fd21de69)

## PropertyElement.PropertyElementType.DetailViewLabelMarkPropertyElementTypes (class)

The DetailViewLabelMarkPropertyElementTypes class represents the available property element types for detail view label marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5745efac-809b-be8c-b39b-9b92abb0ac2e)

### Methods
#### `DetailName(...)`

Returns the detail name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7bd6ac56-ff80-3f6a-35d7-1a9f6f79298a)

#### `DrawingName(...)`

Returns the drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/8eebf810-7b54-cb4c-9866-a3494fb206d5)

#### `Scale(...)`

Returns the scale property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/9848474b-543b-692c-cb2e-63ad562b1448)

#### `SourceDrawingName(...)`

Returns the source drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/1e57bf7b-e2b8-6ba8-859c-340fa7d23ed8)

#### `SourceDrawingNameWhenMoved(...)`

Returns the source drawing name when moved property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/ea2257ad-e829-0e9f-926b-33f93bbfacbc)

## PropertyElement.PropertyElementType.MergedMarkPropertyElementTypes (class)

The MergedMarkPropertyElementTypes class represents the available property element types for merged marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d6a251ce-4150-4159-9c56-e9edf31af3eb)

### Methods
#### `BlockPrefix(...)`

Returns the block prefix property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/4ee576e6-627e-4ef0-0237-96210ab2f571)

#### `DistancesBetweenGroups(...)`

Returns the distances between groups property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/abb53463-74f5-2082-f65f-a22b569f8a76)

#### `SingleMarkContent(...)`

Returns the single mark content property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/d0ce8586-96a1-6fa1-ac45-f19c2d1e0392)

#### `SymbolSeparatingBlocksInMarks(...)`

Returns the symbol separating blocks in marks property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/ad717bd5-04a5-8531-0633-aafef9ef4aa2)

## PropertyElement.PropertyElementType.PartMarkPropertyElementTypes (class)

The PartMarkPropertyElementTypes class represents the available property element types for part marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a05d6a56-8938-bc56-9d19-3f600e238718)

### Methods
#### `AssemblyPosition(...)`

Returns the assembly position property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/afad0710-1da5-130b-05f8-68b575327119)

#### `Camber(...)`

Returns the camber property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/0709b579-58a2-2914-da29-fd19389c3995)

#### `CenterToCenterDistance(...)`

Returns the center-to-center distance property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/665f5d5e-b516-32ad-d490-5f2dc76a0ff6)

#### `Class(...)`

Returns the class property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/0ffff76e-2d03-690f-b528-ce6c798b7802)

#### `FaceDirection(...)`

Returns the face direction property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/e64a5d5c-5276-1ab7-0d0b-a6977d26bc57)

#### `Finish(...)`

Returns the finish property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/3b0ce7f8-7b74-b5df-c423-d8d0183c6221)

#### `FittingsNsFs(...)`

Returns the fittings (NS/FS) property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/e3c678fb-43fb-0885-9331-43c01e7bbc17)

#### `GageOfOutstandingLeg(...)`

Returns the gage of the outstanding leg property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/61ec229a-0d26-3ce7-c406-3274a9b455aa)

#### `Length(...)`

Returns the length property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7c5bc94f-d334-2d99-c083-4110db02a883)

#### `Material(...)`

Returns the material property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/b58eb34e-d398-abb4-5361-6716d387c8ad)

#### `Name(...)`

Returns the name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/aa2a98cc-12f0-db4c-24f9-4f6e7ea22171)

#### `PartPosition(...)`

Returns the part position property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/bdf77fd5-57f5-7c9a-183c-e4c95ec10a78)

#### `Profile(...)`

Returns the profile property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7ac95960-913b-a58b-d824-71168994785d)

#### `RotationAngle(...)`

Returns the rotation angle property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/bc9a1c05-9270-b3b6-0bca-cf11eafd1a6b)

#### `Size(...)`

Returns the size property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/3c1f8f4d-9b6b-7c4e-c497-cf505fafa371)

## PropertyElement.PropertyElementType.PourMarkPropertyElementTypes (class)

The PourMarkPropertyElementTypes class represents the available property element types for pour marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f8361d11-b852-50ae-74a5-659e139eabbe)

### Methods
#### `Class(...)`

Returns the class property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/ac02e677-57de-9fb0-dd34-ea05ff4139db)

#### `Material(...)`

Returns the material property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/43ffe354-2bd0-465c-2aba-3f85dacb1e76)

#### `Name(...)`

Returns the name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/05d664e1-603c-2754-3acf-d17cc4e66b26)

#### `PourConcreteMixture(...)`

Returns the contrete mixture property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/901b05ba-44a0-88e0-771f-a708ddb43cf7)

#### `PourNumber(...)`

Returns the pour number property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/e0d82f72-a13a-c81e-c979-1c58f4e9ff46)

#### `PourType(...)`

Returns the pour type property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/f31607b7-4ff5-07a1-37e1-7b530c219265)

## PropertyElement.PropertyElementType.ReinforcementMarkPropertyElementTypes (class)

The ReinforcementMarkPropertyElementTypes class represents the available property element types for reinforcement marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/20d7b27d-98cf-1dd2-786e-e3ac03f57a47)

### Methods
#### `Cc(...)`

Returns the cc property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/25d784fd-f266-2316-bde3-534c2c8ccfba)

#### `CcExact(...)`

Returns the cc exact property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/b2d55752-7107-1281-16f8-16c3ad7b1a82)

#### `CcMax(...)`

Returns the cc max property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/e555640a-388d-5850-bf59-4f18420ba3f2)

#### `CcMin(...)`

Returns the cc min property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/68cf364a-e493-be7a-f968-c84557a0929b)

#### `CcTarget(...)`

Returns the cc target property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/41d768a1-e016-7304-ca20-0f74ac732b1d)

#### `Class(...)`

Returns the class property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/a355828b-d10f-c990-9002-011ca430e7e6)

#### `Diameter(...)`

Returns the diameter property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/cba228ca-ad32-a07a-65dd-e3b2f39a36c6)

#### `Grade(...)`

Returns the grade property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/6e9fd147-2b42-8b5e-16d4-e9ba46f5a950)

#### `Length(...)`

Returns the length property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/4da1b491-7248-f7b2-c926-149284d4729e)

#### `Name(...)`

Returns the name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/d3f30557-d52e-26af-8012-72c56c8f02ad)

#### `Number(...)`

Returns the number property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7e089ba6-08e3-a1d8-7594-9978c73958b3)

#### `Position(...)`

Returns the position property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/f16ffc9b-8090-7d2a-beeb-a0ee7c53b9e0)

#### `Shape(...)`

Returns the shape property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/516d97a3-5a6f-e7af-7579-b61c1f8a096b)

#### `Weight(...)`

Returns the weight property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/5687b005-c253-547c-42d7-b264b33f9879)

## PropertyElement.PropertyElementType.ReinforcementMeshMarkPropertyElementTypes (class)

The ReinforcementMeshMarkPropertyElementTypes class represents the available property element types for reinforcement mesh marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2f8635ce-f545-33dc-f58b-70b35a9b455c)

### Methods
#### `CcCrossing(...)`

Returns the crossing cc property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/6ad30b28-de86-d336-f472-e262c2191c78)

#### `CcExactCrossing(...)`

Returns the crossing cc exact property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7c809512-05fa-c152-d302-fb120582c76d)

#### `CcExactLongitudinal(...)`

Returns the longitudinal cc exact property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/09bebe43-a812-1d83-0249-8b399041ed1d)

#### `CcLongitudinal(...)`

Returns the longitudinal cc property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/815cb488-adf6-ff57-e235-79f5ce15522a)

#### `CcMaxCrossing(...)`

Returns the crossing cc max property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/68df9c09-dd6c-31d9-cf80-57eb7be1d631)

#### `CcMaxLongitudinal(...)`

Returns the longitudinal cc max property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/5e7769c0-b17f-bbc5-611a-65a367d820c2)

#### `CcMinCrossing(...)`

Returns the crossing cc min property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/9c69ea45-9915-c206-529d-7d6d56330a70)

#### `CcMinLongitudinal(...)`

Returns the longitudinal cc min property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/48ba2e4f-ea34-13f9-663e-b8859f62d661)

#### `Class(...)`

Returns the class property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/e3e2ecbe-98d8-41a3-b2c5-a77e924e5df8)

#### `DiameterCrossing(...)`

Returns the crossing diameter property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/b9247e49-d325-2c4b-799d-edf40f24ed74)

#### `DiameterLongitudinal(...)`

Returns the longitudinal diameter property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/24f66cc3-dbe9-ccf1-6674-415fc3a44431)

#### `Grade(...)`

Returns the grade property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/3bd15925-bab8-0842-f4ce-3718e0bb7b8d)

#### `MeshLength(...)`

Returns the mesh length property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/b620f7bc-342a-747c-2d2e-b0576f3c5141)

#### `MeshWidth(...)`

Returns the mesh width property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/2d6c9c68-c1a2-fb6c-8a73-0a7888a4b0d6)

#### `Name(...)`

Returns the name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/94b1f110-4233-628e-c141-2bb5fe34f8aa)

#### `Number(...)`

Returns the number property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/0f6df021-0c8b-d454-8b32-c4accc8e5f25)

#### `Position(...)`

Returns the position property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/fcf99a1f-e7d8-b1e9-4621-b995829fbb66)

#### `Shape(...)`

Returns the shape property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/2c21a5fe-3920-fba9-1729-37591400ab92)

#### `Size(...)`

Returns the size property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/85b41a8a-0d8e-b1e9-0973-474d542d3f6b)

#### `Weight(...)`

Returns the weight property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/4e538ce0-ecd8-f281-b56f-fed2c51a0dd4)

## PropertyElement.PropertyElementType.SectionMarkPropertyElementTypes (class)

The SectionMarkPropertyElementTypes class represents the available property element types for section marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/925e11ae-1b4a-6dd7-26aa-dada89a429be)

### Methods
#### `SectionName(...)`

Returns the section name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/b55080ac-ccb8-e84c-0182-f7a4ef2b7570)

#### `SourceDrawingName(...)`

Returns the source drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/8a6e4d37-9aaf-2b24-fcbe-d8a116c08ef4)

#### `SourceDrawingNameWhenMoved(...)`

Returns the source drawing name when moved property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/0d097acf-f84d-dcfe-e999-01bcce19d2f5)

## PropertyElement.PropertyElementType.SectionViewLabelMarkPropertyElementTypes (class)

The SectionViewLabelMarkPropertyElementTypes class represents the available property element types for section view label marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1d48c8b5-a2c0-d9d0-2631-24b8da41f12e)

### Methods
#### `DrawingName(...)`

Returns the drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/c11fef10-152f-340a-e90d-c5d0e1eac26e)

#### `Scale(...)`

Returns the scale property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/d7004d07-3aed-741d-dc7f-a0e5aa16e285)

#### `SectionName(...)`

Returns the section name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/7e76335b-275e-81d3-7fa7-bb5dd5086eed)

#### `SourceDrawingName(...)`

Returns the source drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/18a04dac-66be-9b22-7c96-c92dd80ada06)

#### `SourceDrawingNameWhenMoved(...)`

Returns the source drawing name when moved property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/c4366fbd-79a1-d3ff-5b7d-8e57960943e5)

## PropertyElement.PropertyElementType.SurfacingMarkPropertyElementTypes (class)

The SurfacingMarkPropertyElementTypes class represents the available property element types for surfacing marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/217d2470-7a29-0688-957c-d8dbb02bc45b)

### Methods
#### `Class(...)`

Returns the class property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/cb114bbf-965d-29eb-8c94-699ef7495e6b)

#### `Code(...)`

Returns the code property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/c7f205af-dfa1-3448-e391-4b4050e758df)

#### `Material(...)`

Returns the material property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/571e386b-5303-3ada-1454-b59db0e1fe46)

#### `Name(...)`

Returns the name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/68ff7a5f-2f98-1812-31b7-7f62d7e51ac0)

#### `SurfaceTreatmentName(...)`

Returns the surface treatment name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/8b345135-4c8d-1e27-760f-635a5c3f69a6)

## PropertyElement.PropertyElementType.ViewLabelMarkPropertyElementTypes (class)

The ViewLabelMarkPropertyElementTypes class represents the available property element types for view label marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/abbe2390-240f-cf69-a2af-b793252af8ac)

### Methods
#### `DrawingName(...)`

Returns the drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/13caf612-86cc-eb11-f6d3-2fabb1146280)

#### `Scale(...)`

Returns the scale property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/48f65244-1845-9db7-f2ce-ce61ee46d316)

#### `SourceDrawingName(...)`

Returns the source drawing name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/5098a39e-ebd9-339a-7883-1bdebf72a8cc)

#### `SourceDrawingNameWhenMoved(...)`

Returns the source drawing name when moved property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/46e40cc1-f08f-a39b-45ce-27c61b2c3d8a)

#### `ViewName(...)`

Returns the view name property element type.

[Docs](https://developer.tekla.com/topic/en/18/47/f2045126-e266-a18c-9b9b-adc29b83bdfe)

## RadiusDimension (class)

The RadiusDimension class defines a radius dimension that displays the radius of the given arc.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cd974d85-701f-9b61-e4bb-6955d7efa816)

### Constructors
- `RadiusDimension(...)` — Creates a new radius dimension instance using three points and the distance.
- `RadiusDimension(...)` — Creates a new radius dimension instance with the attributes, three points and the distance.

### Methods
#### `Delete(...)`

Deletes the dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/35f59670-a535-3441-b6d3-bfd79adb418c)

#### `GetDimensionSet(Boolean)(...)`

Radius dimensions don't have dimension sets.

[Docs](https://developer.tekla.com/topic/en/18/47/bb7ae1f7-a0be-f198-69ca-33f905958f5d)

#### `GetDimensionSet.(...)`

Radius dimensions don't have dimension sets.

[Docs](https://developer.tekla.com/topic/en/18/47/8f7eb088-aace-b802-9371-e18d7f8f262b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the dimension into the database.

[Docs](https://developer.tekla.com/topic/en/18/47/e9b43533-00eb-a0fd-5e97-7d60db74f20e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/6094cdbd-6fe1-adbd-ebc1-2b38fa12b553)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the existing dimension in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/12a81116-02ec-6164-e22e-4a497b8bd46e)

#### `Select(...)`

Selects the radius dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/3d8646d2-2755-c05b-c5e1-05fea49bf272)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `ArcPoint1` (object, get/set) — Gets the first arc point.
- `ArcPoint2` (object, get/set) — Gets the second arc point.
- `ArcPoint3` (object, get/set) — Gets the third arc point.
- `Attributes` (object, get/set) — The radius dimension attributes.
- `Distance` (object, get/set) — Gets or sets the distance.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## RadiusDimensionAttributes (class)

The RadiusDimensionAttributes class contains attributes to manage all the attributes of a straight dimension.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/63e95acf-3561-9d38-0ba6-172ff088e445)

### Constructors
- `RadiusDimensionAttributes(...)` — Creates a new radius dimension attributes instance which loads standard attributes.
- `RadiusDimensionAttributes(...)` — Creates a new radius dimension attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/f5601fad-2842-6f5d-6f0e-a2726866153e)

#### `LoadAttributes(...)`

Loads attributes from the specified attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/553f5f0a-be30-616c-aefc-f9e2ec0fbe85)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the dimension.
- `Color` (object, get/set) — The color of the dimension line.
- `Format` (object, get/set) — The format attributes of the dimension.
- `Placing` (object, get/set) — The placing attributes of the dimension.
- `Text` (object, get/set) — The text attributes of the dimension.
- `TransparentBackground` (object, get/set) — The dimension text background transparency. (transparent/opaque)
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the dimension line.

## Rectangle (class)

The Rectangle class defines a rectangle that is a box made of four lines with orthogonal angles.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/54ce7e2f-93ea-3eeb-ea30-cdc6e5bf453b)

### Constructors
- `Rectangle(...)` — Creates a new rectangle instance with two points using standard attributes. Please note that creating a rectangle using two points will re-calculate the start point to correspond with positive widths and heights.
- `Rectangle(...)` — Creates a new rectangle instance with the start point and the defined width and height using standard attributes.
- `Rectangle(...)` — Creates a new rectangle instance with two points using the given attributes. Please note that creating a rectangle using two points will re-calculate the start point to correspond with positive widths and heights.
- `Rectangle(...)` — Creates a new rectangle instance with the start point and the defined width and height using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/724d7fa7-8819-dfac-4c9c-700264d7ef79)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/7375cc49-4069-3d42-049c-842d1919d682)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/1872c312-d5ed-2f86-a094-9076a0d97b31)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/9f602580-e659-9f52-e664-58c98882f003)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/804f1153-35e7-4c6f-eef0-f3851e625d8d)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — The angle of the rectangle. The rectangle rotates around its start point.
- `Attributes` (object, get/set) — The attributes of the rectangle.
- `EndPoint` (object, get/set) — The end point of the rectangle.
- `Height` (object, get/set) — The height of the rectangle. Changing the height changes the end point with the height.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — The start point of the rectangle. Changing this will move the rectangle.
- `Width` (object, get/set) — The width of the rectangle. Changing the width changes the end point with the width.

## Rectangle.RectangleAttributes (class)

The RectangleAttributes class is the attributes class of the rectangle.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d916e102-2fc3-f888-8399-dc94fc4cfebf)

### Constructors
- `Rectangle.RectangleAttributes(...)` — Creates a new default rectangle attributes instance that loads standard attributes.
- `Rectangle.RectangleAttributes(...)` — Creates a new rectangle attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/6e4b5fa1-be4f-1840-4fd0-de810b7e10bf)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/9e964ef1-34b9-4ffb-33fe-480366a83dd1)

### Properties
- `BehindModelObjects` (object, get/set) — Gets or sets whether the closed graphic object is drawn behind model objects
- `Bulge` (object, get/set) — The bulge value for the rectangle.
- `Hatch` (object, get/set) — The hatch attributes of the closed graphic object (the closed graphic object is hatchable).
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## RectangleBoundingBox (class)

The RectangleBoundingBox class represents a bounding box for an object, i.e. the surrounding box of the object. The bounding box contains the calculated corner points of the box. The rectangle bounding box is inherited from the axis-aligned bounding box. The AABB also contains methods for getting the center point of the box.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/93efcf40-92db-3e7d-2cb4-5e220d916f5b)

### Methods
#### `Collide(...)`

Checks if the current axis-aligned 3d bounding box collides with another given axis-aligned 3d bounding box. Both axis-aligned 3d bounding boxes need to be in the same coordinate system or in the same workplane, so that they are defined using the same axes.

[Docs](https://developer.tekla.com/topic/en/18/47/057b4309-6593-9b8a-581e-8233518f597a)

#### `CreateRectangleBoundingBox(...)`

Creates a new RectangleBoundingBox with the given values.

[Docs](https://developer.tekla.com/topic/en/18/47/c57f052a-b758-e28b-434d-8090532e341b)

#### `GetCenterPoint(...)`

Returns the geometric center point of the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/015fb65b-ad44-3e33-e891-0d9351e340cf)

#### `GetCornerPoints(...)`

Returns all corner points of the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/5f6f6ca4-13aa-d956-047a-f84112c028bb)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/d4a6968c-2054-eb16-a2bb-5fc4573e2a28)

#### `IsInside(LineSegment)(...)`

Checks if the given line segment is inside the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/31e46219-4fba-b1da-4b5b-2ab276eef609)

#### `IsInside(Point)(...)`

Checks if the given point is inside the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/29c49520-1316-8e73-c29e-f4c1dd49711d)

#### `ToString(...)`

Returns the rectangle bounding box values as a string.

[Docs](https://developer.tekla.com/topic/en/18/47/cc7026d3-a7d7-7763-b336-e2b914e42252)

### Properties
- `AngleToAxis` (object, get/set) — The angle of the bounding box in relation to the axis (the coordinate system's X-plane).
- `Height` (object, get/set) — The height of the bounding box.
- `LowerLeft` (object, get/set) — The lower left corner point of the bounding box.
- `LowerRight` (object, get/set) — The lower right corner point of the bounding box.
- `MaxPoint` (object, get/set) — The maximum point of the axis-aligned 3d bounding box.
- `MinPoint` (object, get/set) — The minimum point of the axis-aligned 3d bounding box.
- `UpperLeft` (object, get/set) — The upper left corner point of the bounding box.
- `UpperRight` (object, get/set) — The upper right corner point of the bounding box.
- `Width` (object, get/set) — The width of the bounding box.

## RectangularCloud (class)

The RectangularCloud class defines a rectangular cloud that is a rectangle with a specific size set for the bulges.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/588d418f-7707-420b-88d1-a905e0f07898)

### Constructors
- `RectangularCloud(...)` — Creates a new rectangle cloud instance with two points using standard attributes. Please note that creating a rectangular cloud using two points will re-calculate the start point to correspond with positive widths and heights.
- `RectangularCloud(...)` — Creates a new RectangularCloud instance with the start point and the defined width and height using standard attributes.
- `RectangularCloud(...)` — Creates a new rectangle cloud instance with two points using the given attributes. Please note that creating a rectangular cloud using two points will re-calculate the start point to correspond with positive widths and heights.
- `RectangularCloud(...)` — Creates a new rectangular cloud instance with the start point and the defined width and height using the given attributes.

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/9c36efa2-c4b5-6d8f-2426-f1548eaaf58b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Calls the system to insert the new object.

[Docs](https://developer.tekla.com/topic/en/18/47/828d8081-8b9b-4bdf-6c13-59fc1adf4c88)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/3b6f0c72-746a-9831-0c3e-77ec22245e45)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/474acb2f-833c-0592-aa8a-8a245c2bae8f)

#### `Select(...)`

Calls the system to select and retrieve the object.

[Docs](https://developer.tekla.com/topic/en/18/47/31d731bf-ed27-bffc-8e4d-93456a15d88b)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — The angle of the rectangular cloud. The rectangular cloud rotates around its start point.
- `ArcWidth` (object, get/set) — The arc width to use for the cloud base. The property usage follows the below legacy cloud creation rules. In case of positive value then that value will be used. In case of zero value then the positive value of the XS_ARC_WIDTH_OF_CLOUD advance option is used and in case that also the advance option value is 0 then the default legacy 10 value is used. In case of negative value the value is transformed and its positive value is used.
- `Attributes` (object, get/set) — The attributes for the rectangular cloud.
- `Bulge` (object, get/set) — The bulge (curve) to use (a bulge is the ratio between the height and width of an object).
- `EndPoint` (object, get/set) — The end point of the rectangular cloud.
- `Height` (object, get/set) — The height of the rectangular cloud. Changing the height changes the end point with the height.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — The start point of the rectangular cloud. Changing this will move the rectangular cloud.
- `Width` (object, get/set) — The width of the rectangular cloud. Changing the width changes the end point with the width.

## RectangularCloud.RectangularCloudAttributes (class)

The RectangularCloudAttributes class is the attributes class for the rectangular cloud.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/884bba0b-003b-00b2-91b4-7ec98cbbf103)

### Constructors
- `RectangularCloud.RectangularCloudAttributes(...)` — Creates a new default rectangular cloud attributes instance that loads standard attributes.
- `RectangularCloud.RectangularCloudAttributes(...)` — Creates a new rectangular cloud attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/ac4fbd86-0f83-5897-ad39-73d63f3f3763)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/8dc2696f-f991-96de-3820-4baa9d2f2d8f)

### Properties
- `BehindModelObjects` (object, get/set) — Gets or sets whether the closed graphic object is drawn behind model objects
- `Hatch` (object, get/set) — The hatch attributes of the closed graphic object (the closed graphic object is hatchable).
- `Line` (object, get/set) — The line attributes of the graphic object. A graphic object always consists of lines.

## ReferenceModel (class)

The ReferenceModel class represents a reference model object in a drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/469f1282-a2f2-a9f7-cb79-df1bc3bd3792)

### Methods
#### `Delete(...)`

Reference model objects cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/641abecd-b11e-5d5d-2694-0cd605a8518f)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Reference model objects cannot be inserted.

[Docs](https://developer.tekla.com/topic/en/18/47/51a8a741-169e-4316-b71f-4fb555b0db0f)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/7454d120-ba92-73e7-3acc-a06090469dab)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the reference model object.

[Docs](https://developer.tekla.com/topic/en/18/47/bb47810a-8b6f-6eed-f4ee-a5440c74390a)

#### `Select(...)`

Selects the reference model object.

[Docs](https://developer.tekla.com/topic/en/18/47/090655e8-4326-9a39-fe78-1765f749e68e)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the reference model object attributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## ReferenceModel.ReferenceModelAttributes (class)

The ReferenceModelAttributes class contains attributes for reference model objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/78daa97e-0ebb-8713-3df7-ea124fc0a6db)

### Constructors
- `ReferenceModel.ReferenceModelAttributes(...)` — Creates a new default reference model attributes instance that loads standard attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/cfe1ac5d-1be1-1e60-c42f-15ecd038ca6f)

#### `LoadAttributes(...)`

Loads the specified attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/b59bcad8-760c-a158-f842-39989dc788b3)

### Properties
- `Color` (object, get/set) — Obsolete.
- `Line` (object, get/set) — Gets or sets the line type.
- `LineType` (object, get/set) — Obsolete.

## ReinforcementBase (class)

The ReinforcementBase class contains methods related to reinforcements. A reinforcement is a drawing object derived from a model object. It represents a drawing reinforcement that has a reference to the actual reinforcement in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/159fff52-3638-7a00-c559-67ae63e8a579)

### Methods
#### `Delete(...)`

Deletes the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/5417940b-7449-1720-f9d1-b0b4732c0764)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the drawing reinforcement into the current drawing database. NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/d2e3043f-83c3-bd2a-3f21-d0c15160a88e)

#### `IsEqual(...)`

Compares the current object with the given object. Checks every field of both objects for equality.

[Docs](https://developer.tekla.com/topic/en/18/47/ef079c64-f745-35e7-e5c0-4b2c2bc93d02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing reinforcement in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/1ffbb56b-6e9e-56be-7169-effee37049e8)

#### `Select(...)`

Selects the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/09aa70c0-44c3-0951-01c6-b88f00a82d98)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The common attributes of all reinforcements. For more information see ReinforcementBase.ReinforcementBaseAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## ReinforcementBase.HookedEndSymbolTypes (enum)

The available hooked end symbol types. See Tekla Structures help, "Reinforcement appearance", for more information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8e4956a2-6d10-7e8e-7aa9-fd0338c4ef15)

### Values
- `NoHook` = `1` — No hook symbol.
- `HalfHook` = `2` — The half hook symbol.
- `FullHook` = `3` — The full hook symbol.

## ReinforcementBase.ReinforcementBaseAttributes (class)

The ReinforcementBaseAttributes class is the attributes class for the reinforcement.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/184f289b-bf2a-1406-6387-c9db0de79a39)

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/787be2aa-0991-a803-fcd1-9548946d766e)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/d0b91e0d-69ce-a357-cb15-716ca34687ce)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `HiddenLines` (object, get/set) — The line type of the hidden lines of the reinforcement.
- `HideLinesHiddenByPart` (object, get/set) — Defines whether lines hidden by parts should be hidden or not.
- `HideLinesHiddenByReinforcement` (object, get/set) — Defines whether lines hidden by reinforcements should be hidden or not.
- `ReinforcementRepresentation` (object, get/set) — The representation of reinforcing bars.
- `VisibleLines` (object, get/set) — The line type of the visible lines of the reinforcement.

## ReinforcementBase.ReinforcementGroupAttributes (class)

The ReinforcementGroupAttributes class is the base attributes class for ReinforcementGroup.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/864c99b0-8739-7a28-32a8-4d4bd7334287)

### Constructors
- `ReinforcementBase.ReinforcementGroupAttributes(...)` — Creates a new default reinforcement attributes instance that loads standard attributes.
- `ReinforcementBase.ReinforcementGroupAttributes(...)` — Creates a new reinforcement attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/846ae007-a932-b8e1-933b-a423311c1f91)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/d0b91e0d-69ce-a357-cb15-716ca34687ce)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `HiddenLines` (object, get/set) — The line type of the hidden lines of the reinforcement.
- `HideLinesHiddenByPart` (object, get/set) — Defines whether lines hidden by parts should be hidden or not.
- `HideLinesHiddenByReinforcement` (object, get/set) — Defines whether lines hidden by reinforcements should be hidden or not.
- `HookedEndSymbolType` (object, get/set) — Defines how the hooked ends of reinforcing bars should look like.
- `ReinforcementRepresentation` (object, get/set) — The representation of reinforcing bars.
- `ReinforcementVisibility` (object, get/set) — Defines which bars in the group appear in drawings.
- `StraightEndSymbolType` (object, get/set) — Defines how the straight ends of reinforcing bars should look like.
- `VisibleLines` (object, get/set) — The line type of the visible lines of the reinforcement.

## ReinforcementBase.ReinforcementMeshAttributes (class)

The ReinforcementMeshAttributes class is the base attributes class for ReinforcementMesh.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cf7408d5-abf8-bc64-7087-ae2389369349)

### Constructors
- `ReinforcementBase.ReinforcementMeshAttributes(...)` — Creates a new default reinforcement attributes instance that loads standard attributes.
- `ReinforcementBase.ReinforcementMeshAttributes(...)` — Creates a new default reinforcement attributes instance that loads standard attributes. If it set to true attributes won't be lodaded.
- `ReinforcementBase.ReinforcementMeshAttributes(...)` — Creates a new reinforcement attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/73beb5c4-187c-b116-b9d1-24415cf0bf65)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/d0b91e0d-69ce-a357-cb15-716ca34687ce)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `HiddenLines` (object, get/set) — The line type of the hidden lines of the reinforcement.
- `HideLinesHiddenByPart` (object, get/set) — Defines whether lines hidden by parts should be hidden or not.
- `HideLinesHiddenByReinforcement` (object, get/set) — Defines whether lines hidden by reinforcements should be hidden or not.
- `MeshReinforcementSymbolIndex` (object, get/set) — Defines the index for the mesh symbol to be used. The mesh symbol appears in the middle of the mesh line when ReinforcementRepresentation == OutLine. The index starts from 0 and corresponds to the symbol in the file mesh.sym.
- `MeshReinforcementSymbolSize` (object, get/set) — Defines the size of the reinforcement mesh symbol.
- `MeshReinforcementVisibilityCrossing` (object, get/set) — Defines which crossing bars in the mesh appear in drawings.
- `MeshReinforcementVisibilityLongitudinal` (object, get/set) — Defines which longitudinal wires in the mesh appear in drawings.
- `ReinforcementRepresentation` (object, get/set) — The representation of reinforcing bars.
- `VisibleLines` (object, get/set) — The line type of the visible lines of the reinforcement.

## ReinforcementBase.ReinforcementRepresentationTypes (enum)

The available reinforcement representation types. See Tekla Structures help, "Reinforcement appearance", for more information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/16b9e0af-60a4-8a44-1cbf-bb7be95cc3dd)

### Values
- `SingleLine` = `1` — A single line with radiused bends.
- `DoubleLine` = `2` — The outline of a bar with radiused bends.
- `Filled` = `3` — A solid bar with radiused bends.
- `Stick` = `4` — A single line without radiused bends.
- `OutLine` = `5` — Shows the shape of the mesh using an outline rectangle or polygon, and a diagonal line. Only applies to reinforcement meshes.
- `DoubleLineFilledEnds` = `6` — The outline of a bar with radiused bends and filled bar ends.
- `SingleLineFilledEnds` = `7` — A single line with radiused bends and filled bar ends.
- `OutlineIgnoreHoles` = `8` — Shows the shape of the mesh using an outline rectangle or polygon, and a diagonal line. It ignores the holes in the meshes. Only applies to reinforcement meshes.

## ReinforcementBase.ReinforcementSetGroupAttributes (class)

The ReinforcementSetGroupAttributes class is the base attributes class for ReinforcementSetGroup.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a614dcaa-9005-9783-8b6a-0b22e4eca438)

### Constructors
- `ReinforcementBase.ReinforcementSetGroupAttributes(...)` — Initializes a new instance of the ReinforcementBase.ReinforcementSetGroupAttributes class.
- `ReinforcementBase.ReinforcementSetGroupAttributes(...)` — Initializes a new instance of the ReinforcementBase.ReinforcementSetGroupAttributes class that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/b9fb0f90-b2dc-ffe2-dbe8-84f72fe7c086)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/d0b91e0d-69ce-a357-cb15-716ca34687ce)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `HiddenLines` (object, get/set) — The line type of the hidden lines of the reinforcement.
- `HideLinesHiddenByPart` (object, get/set) — Defines whether lines hidden by parts should be hidden or not.
- `HideLinesHiddenByReinforcement` (object, get/set) — Defines whether lines hidden by reinforcements should be hidden or not.
- `HookedEndSymbolType` (object, get/set) — Gets or sets how the hooked ends of reinforcing bars should look like.
- `ReinforcementRepresentation` (object, get/set) — The representation of reinforcing bars.
- `ReinforcementVisibility` (object, get/set) — Gets or sets which bars in the group appear in drawings.
- `StraightEndSymbolType` (object, get/set) — Gets or sets how the straight ends of reinforcing bars should look like.
- `VisibleLines` (object, get/set) — The line type of the visible lines of the reinforcement.

## ReinforcementBase.ReinforcementSingleAttributes (class)

The ReinforcementSingleAttributes class is the base attributes class for ReinforcementSingle.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/106ed442-a9b1-c122-82ad-3540f21968b2)

### Constructors
- `ReinforcementBase.ReinforcementSingleAttributes(...)` — Creates a new default reinforcement attributes instance that loads standard attributes.
- `ReinforcementBase.ReinforcementSingleAttributes(...)` — Creates a new reinforcement attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4258603a-024a-5fb5-624c-f1e15e693581)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/d0b91e0d-69ce-a357-cb15-716ca34687ce)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `HiddenLines` (object, get/set) — The line type of the hidden lines of the reinforcement.
- `HideLinesHiddenByPart` (object, get/set) — Defines whether lines hidden by parts should be hidden or not.
- `HideLinesHiddenByReinforcement` (object, get/set) — Defines whether lines hidden by reinforcements should be hidden or not.
- `HookedEndSymbolType` (object, get/set) — Defines how the hooked ends of reinforcing bars should look like.
- `ReinforcementRepresentation` (object, get/set) — The representation of reinforcing bars.
- `ReinforcementVisibility` (object, get/set) — Defines which bars in the group appear in drawings.
- `StraightEndSymbolType` (object, get/set) — Defines how the straight ends of reinforcing bars should look like.
- `VisibleLines` (object, get/set) — The line type of the visible lines of the reinforcement.

## ReinforcementBase.ReinforcementStrandAttributes (class)

The ReinforcementStrandAttributes class is the base attributes class for ReinforcementStrand.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a5f43557-02c7-1312-8b14-e3def23c5741)

### Constructors
- `ReinforcementBase.ReinforcementStrandAttributes(...)` — Creates a new default reinforcement attributes instance that loads standard attributes.
- `ReinforcementBase.ReinforcementStrandAttributes(...)` — Creates a new default reinforcement attributes instance that loads standard attributes. If it set to true attributes won't be lodaded.
- `ReinforcementBase.ReinforcementStrandAttributes(...)` — Creates a new reinforcement attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e0f41474-dfc7-ca16-fde9-adcbdd9a2add)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/d0b91e0d-69ce-a357-cb15-716ca34687ce)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `HiddenLines` (object, get/set) — The line type of the hidden lines of the reinforcement.
- `HideLinesHiddenByPart` (object, get/set) — Defines whether lines hidden by parts should be hidden or not.
- `HideLinesHiddenByReinforcement` (object, get/set) — Defines whether lines hidden by reinforcements should be hidden or not.
- `HookedEndSymbolType` (object, get/set) — Defines how the hooked ends of reinforcing bars should look like.
- `ReinforcementRepresentation` (object, get/set) — The representation of reinforcing bars.
- `ReinforcementVisibility` (object, get/set) — Defines which bars in the group appear in drawings.
- `StraightEndSymbolType` (object, get/set) — Defines how the straight ends of reinforcing bars should look like.
- `VisibleLines` (object, get/set) — The line type of the visible lines of the reinforcement.

## ReinforcementBase.ReinforcementVisibilityTypes (enum)

The available reinforcement visibility types. Only applies to bar groups and meshes. See Tekla Structures help, "Setting the visibility of reinforcement in drawings", for more information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4b3131b4-003c-4de6-41ce-787cc77fb8e4)

### Values
- `All` = `1` — Shows all bars in a group or mesh.
- `First` = `2` — Only shows the first bar in the group or mesh.
- `Last` = `3` — Only shows the last bar in the group or mesh.
- `FirstAndLast` = `4` — Shows the first and last bar in the group or mesh.
- `OneInTheMiddle` = `5` — Shows one bar in the middle of the group or mesh.
- `TwoInTheMiddle` = `6` — Shows two bars in the middle of the group or mesh.
- `Customized` = `7` — Indicates that you have specified the location of the only visible reinforcing bar. See "Adjusting reinforcing bars" in the Tekla Structures help for more information.

## ReinforcementBase.StraightEndSymbolTypes (enum)

The available straight end symbol types. See Tekla Structures help, "Reinforcement appearance", for more information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/291c577a-03d1-f01b-e1ec-64975aab8399)

### Values
- `NoSymbol` = `1` — No straight end symbol.
- `HooksOnInnerSide` = `2` — Draws half hooks on the reinforcement's inner side.
- `HooksOnOuterSide` = `3` — Draws half hooks on the reinforcement's outer side.
- `HookOnReinforcementStartOuterSide` = `4` — Draws a half hook on the reinforcement's outer side in the reinforcement's start point, and another half hook on the inner side in the end point.
- `HookOnReinforcementStartInnerSide` = `5` — Draws a half hook on the reinforcement's inner side in the reinforcement's start point, and another half hook on the outer side in the end point.
- `FullHooks` = `6` — Draws full hooks in the reinforcement's end points.

## ReinforcementGroup (class)

The ReinforcementGroup class defines a reinforcement group in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c7ac15f0-687c-db18-d381-d6eabd263707)

### Methods
#### `Delete(...)`

Deletes the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/5417940b-7449-1720-f9d1-b0b4732c0764)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the drawing reinforcement into the current drawing database. NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/d2e3043f-83c3-bd2a-3f21-d0c15160a88e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/9d557296-3b9a-53ea-bdfc-b73073a9cfc0)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing reinforcement in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/1ffbb56b-6e9e-56be-7169-effee37049e8)

#### `Select(...)`

Selects the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/09aa70c0-44c3-0951-01c6-b88f00a82d98)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the reinforcement. For more information see ReinforcementBase.ReinforcementGroupAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `ReinforcementCustomPosition` (object, get/set) — Defines the location of the reinforcement bar when using ReinforcementVisibility == ReinforcementVisibilityTypes.Customized. This location is a value between 0.0 and 1.0 and it defines the proportion from the reinforcement's start point.

## ReinforcementMesh (class)

The ReinforcementMesh class defines a reinforcement mesh in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9e75aa3d-1757-77f9-bd72-af97984fbdee)

### Methods
#### `Delete(...)`

Deletes the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/5417940b-7449-1720-f9d1-b0b4732c0764)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the drawing reinforcement into the current drawing database. NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/d2e3043f-83c3-bd2a-3f21-d0c15160a88e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/77384ed5-452c-89fc-ddee-dd92cdd7e0eb)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing reinforcement in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/1ffbb56b-6e9e-56be-7169-effee37049e8)

#### `Select(...)`

Selects the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/09aa70c0-44c3-0951-01c6-b88f00a82d98)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the reinforcement. For more information see ReinforcementBase.ReinforcementMeshAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `ReinforcementCustomPositionCrossing` (object, get/set) — Defines the location of the crossing bar when using MeshReinforcementVisibilityCrossing == ReinforcementVisibilityTypes.Customized. This location is a value between 0.0 and 1.0 and it defines the proportion from the mesh's start point.
- `ReinforcementCustomPositionLongitudinal` (object, get/set) — Defines the location of the longitudinal wire when using ReinforcementVisibility == ReinforcementVisibilityTypes.Customized. This location is a value between 0.0 and 1.0 and it defines the proportion from the reinforcement's start point.

## ReinforcementPulloutElement (class)

The ReinforcementPulloutElement class illustrates the shape and dimensions of a reinforcing bar in a drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/952320a4-528c-1bc2-b9da-e4eaf2c29ebd)

### Constructors
- `ReinforcementPulloutElement(...)` — Creates a new reinforcement pull-out element instance.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/4832526c-92cd-e1e1-5081-96cb7c97339d)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/499b4a65-9122-8014-eb20-cb8c7f90d748)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4c93af0e-c5f6-9da0-a4ed-bfeb2dc34331)

### Properties
- `AutomaticScaling` (object, get/set) — Gets or sets a value indicating whether the pull-out is scaled automatically or to a specified value.
- `BendingAngle` (object, get/set) — Gets or sets a value indicating whether to show the pull-out bending angles.
- `BendingRadius` (object, get/set) — Gets or sets a value indicating whether to show the pull-out bending radii.
- `CouplersEndAnchorSymbols` (object, get/set) — Gets or sets a value indicating whether to show coupler/anchor symbols or not.
- `CouplersScale` (object, get/set) — Gets or sets the scale of coupler/anchor symbols.
- `Dimensioning` (object, get/set) — Gets or sets a value indicating whether to show the pull-out dimensions.
- `EndSymbolType` (object, get/set) — Gets or sets the type of the ending marks.
- `Exaggeration` (object, get/set) — Gets or sets a value indicating whether the pull-out is exaggerated.
- `Font` (object, get/set) — Gets or sets the font of the dimension.
- `RotationAxis` (object, get/set) — Gets or sets the rotation of the pull-out.
- `ScaleX` (object, get/set) — Gets or sets the scaling of the pull-out in the X direction.
- `ScaleY` (object, get/set) — Gets or sets the scaling of the pull-out in the Y direction.

## ReinforcementPulloutElement.EndSymbols (enum)

The shape of the bar ends in the pull-out.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fd664343-fc64-7eb6-3d15-3d523ee0498b)

### Values
- `NONE` = `1` — Obsolete. No end symbols.
- `None` = `1` — No end symbols.
- `SINGLE` = `2` — Obsolete. A single end symbol arrow.
- `Single` = `2` — A single end symbol arrow.
- `BOTH` = `3` — Obsolete. An arrow to both directions.
- `Both` = `3` — An arrow to both directions.

## ReinforcementPulloutElement.Rotation (enum)

The rotation of the pull-out.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/406dd8c3-f13c-b1f7-f468-d4fbe369036a)

### Values
- `Automatic` = `0` — Automatic rotation.
- `Plane` = `1` — Plane rotation.
- `Global` = `2` — Global 3D rotation.

## ReinforcementSetGroup (class)

The ReinforcementSetGroup class defines a reinforcement set group in the drawing. A model rebar set generates one or more groups of rebars for drawing purposes and each group is represented in a drawing as a reinforcement set group. ModelIdentifier is always zero for the ReinforcementSetGroup.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7a692049-f2d2-daa4-af1d-0fa3d6c7df8b)

### Methods
#### `Delete(...)`

Deletes the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/5417940b-7449-1720-f9d1-b0b4732c0764)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetModelIdentifiers(...)`

Gets the model identifiers of the single rebars in this reinforcement set group.

[Docs](https://developer.tekla.com/topic/en/18/47/1fb35643-b806-ac06-2bec-0753fdb9817a)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the drawing reinforcement into the current drawing database. NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/d2e3043f-83c3-bd2a-3f21-d0c15160a88e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/8c8190b3-e696-d01d-62a5-c37afbd2a0ac)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing reinforcement in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/1ffbb56b-6e9e-56be-7169-effee37049e8)

#### `Select(...)`

Selects the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/09aa70c0-44c3-0951-01c6-b88f00a82d98)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the reinforcement. For more information see ReinforcementBase.ReinforcementSetGroupAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `ReinforcementCustomPosition` (object, get/set) — Gets or sets the location of the reinforcement bar when using ReinforcementVisibility == ReinforcementVisibilityTypes.Customized. This location is a value between 0.0 and 1.0 and it defines the proportion from the reinforcement's start point.

## ReinforcementSingle (class)

The ReinforcementSingle class defines a single (as in not part of a group) reinforcement bar in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bf5a9fba-dc64-8081-43be-9d41beff590c)

### Methods
#### `Delete(...)`

Deletes the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/5417940b-7449-1720-f9d1-b0b4732c0764)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the drawing reinforcement into the current drawing database. NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/d2e3043f-83c3-bd2a-3f21-d0c15160a88e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/d63f1736-e54c-6d7d-f34b-3bc02af8d179)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing reinforcement in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/1ffbb56b-6e9e-56be-7169-effee37049e8)

#### `Select(...)`

Selects the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/09aa70c0-44c3-0951-01c6-b88f00a82d98)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the reinforcement. For more information see ReinforcementBase.ReinforcementSingleAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `ReinforcementCustomPosition` (object, get/set) — Defines the location of the reinforcement bar when using ReinforcementVisibility == ReinforcementVisibilityTypes.Customized. This location is a value between 0.0 and 1.0 and it defines the proportion from the reinforcement's start point.

## ReinforcementStrand (class)

The ReinforcementStrand class defines a reinforcement mesh in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2aaf7ceb-4300-8754-4e50-035d31aee6b3)

### Methods
#### `Delete(...)`

Deletes the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/5417940b-7449-1720-f9d1-b0b4732c0764)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the drawing reinforcement into the current drawing database. NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/d2e3043f-83c3-bd2a-3f21-d0c15160a88e)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/b3857fa0-d243-6e78-017f-1c3ae494565d)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the drawing reinforcement in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/1ffbb56b-6e9e-56be-7169-effee37049e8)

#### `Select(...)`

Selects the drawing reinforcement from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/09aa70c0-44c3-0951-01c6-b88f00a82d98)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the reinforcement. For more information see ReinforcementBase.ReinforcementStrandAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `ReinforcementCustomPosition` (object, get/set) — Defines the location of the reinforcement bar when using ReinforcementVisibility == ReinforcementVisibilityTypes.Customized. This location is a value between 0.0 and 1.0 and it defines the proportion from the reinforcement's start point.

## ReportTemplateElement (class)

The ReportTemplateElement class represents a template in the Tekla Structures template library.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0a6690c4-2eca-a4d2-427f-e044282fc4c3)

### Constructors
- `ReportTemplateElement(...)` — Creates a new report template element instance with the default template.
- `ReportTemplateElement(...)` — Creates a new report template element instance with the given template.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/9e4ac224-4776-92ab-a8ac-25a05d262bc6)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/89abb5d5-b23f-ede9-1455-76d6c947e420)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/f6040361-088b-3a59-e2d2-82f7bd06fea4)

### Properties
- `Template` (object, get/set) — The template.

## ScalingOptions (enum)

The options for controlling the scaling of the objects in the drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9665449d-ff04-7985-ad81-52330c936e19)

### Values
- `NoScaling` = `0` — Does not scale the object.
- `ScaleToFit` = `4` — Scales the object to fit in the frame in the Y direction.

## SectionMark (class)

The SectionMark class defines a drawing object that illustrates a straight section in a specific view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/85ff4eb8-4c7e-c72f-4de6-59843ac0138a)

### Constructors
- `SectionMark(...)` — Creates a new section mark instance with the given positions.
- `SectionMark(...)` — Creates a new section mark instance with the given positions and attributes.

### Methods
#### `Delete(...)`

Deletes the section mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/17865574-7d79-82df-c3a4-d568c75af535)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the section mark into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/00d180af-43bf-d119-0ca7-0324343b45cc)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/3135efc4-7739-3450-27a6-554f8f1e841c)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the section mark in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/9b9a2de8-9c42-3170-e7fc-a25d5eeb89fd)

#### `Select(...)`

Selects the section mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/888e2a8c-21c6-7e2c-cac9-af4948c11008)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the section mark.
- `LeftPoint` (object, get/set) — The starting point of the left symbol of the section mark.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `RightPoint` (object, get/set) — The starting point of the right symbol of the section mark.

## SectionMarkBase (class)

The SectionMarkBase class is a base class for both the SectionMark class and the CurvedSectionMark class.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d6b75dbc-26e3-6c2e-f64c-c6634d7533e7)

### Methods
#### `Delete(...)`

Deletes the section mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/17865574-7d79-82df-c3a4-d568c75af535)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the section mark into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/00d180af-43bf-d119-0ca7-0324343b45cc)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/3135efc4-7739-3450-27a6-554f8f1e841c)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the section mark in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/9b9a2de8-9c42-3170-e7fc-a25d5eeb89fd)

#### `Select(...)`

Selects the section mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/888e2a8c-21c6-7e2c-cac9-af4948c11008)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the section mark.
- `LeftPoint` (object, get/set) — The starting point of the left symbol of the section mark.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `RightPoint` (object, get/set) — The starting point of the right symbol of the section mark.

## SectionMarkBase.SectionMarkAttributes (class)

The SectionMarkAttributes class is the attributes class for section marks.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bc3ac46a-11ed-0b50-182d-2f8fbddb2763)

### Constructors
- `SectionMarkBase.SectionMarkAttributes(...)` — Creates a new default SectionMarkAttributes instance that loads standard attributes.
- `SectionMarkBase.SectionMarkAttributes(...)` — Creates a new SectionMarkAttributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/6e982ee0-53e8-1f44-d19b-5c335d6e4462)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/a6910abe-d01a-b867-6c65-66f7f1f34fa9)

### Properties
- `LeftSymbol` (object, get/set) — The symbol to use on the left side of the section mark.
- `LineColor` (object, get/set) — The color of the section mark line (see DrawingColors for options).
- `LineLength` (object, get/set) — The length of the section mark lines.
- `LineLengthOffset` (object, get/set) — The offset from the end of the line on both sides of the section mark.
- `LineWidth` (object, get/set) — The width of the section mark lines.
- `LineWidthOffsetLeft` (object, get/set) — The offset from the start of the line on the left side of the section mark.
- `LineWidthOffsetRight` (object, get/set) — The offset from the start of the line on the right side of the section mark.
- `MarkName` (object, get/set) — The section mark name.
- `RightSymbol` (object, get/set) — The symbol to use on the right side of the section mark.
- `SymbolColor` (object, get/set) — The color of the mark symbols (see DrawingColors for options).
- `TagsAttributes` (object, get/set) — The tags A1-A5 in the section mark attributes.
- `TrueLineColor` (object, get/set) — The true color (rgb enabled) of the line.
- `TrueSymbolColor` (object, get/set) — The true color (rgb enabled) of the symbol.

## SectionMarkBase.SectionMarkSymbol (class)

The SectionMarkSymbol class is for section mark attributes related to the shape of the connecting symbol.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b08c53a3-44f2-e297-c1ab-1c5b86fa7110)

### Constructors
- `SectionMarkBase.SectionMarkSymbol(...)` — Creates a new section mark symbol instance.
- `SectionMarkBase.SectionMarkSymbol(...)` — Creates a new section mark symbol instance with the given shape, size and position.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/109f4eec-fc10-4b25-930b-3d95fb9d173b)

### Properties
- `Position` (object, get/set) — The position of the section mark symbol.
- `Shape` (object, get/set) — The shape of the section mark symbol.
- `Size` (object, get/set) — The size of the section mark symbol.

## SectionMarkBase.SectionMarkSymbol.SymbolShapes (enum)

The possible shapes of the section mark symbol.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0a5890e1-e6db-7652-7fad-2dbfd157e0fa)

### Values
- `None` = `1` — No symbol.
- `SymbolShape1` = `101` — The triangle with black and white halves.
- `SymbolShape2` = `102` — The arrow line pointing to an orthogonal straight line.
- `SymbolShape3` = `103` — The line with half of an arrow end.
- `SymbolShape4` = `104` — The rectangle shape with very small height.
- `SymbolShape5` = `105` — The triangle and circle shapes mixed. The circle partly hides the triangle.
- `SymbolShape6` = `106` — The triangle and circle shapes mixed. The triangle is on top.
- `SymbolShape7` = `107` — The triangle shape without content.
- `SymbolShape8` = `108` — The triangle shape filled in black.
- `Custom` = `4` — The custom shape.

## SectionMarkBase.SectionMarkTagAttributes (class)

The SectionMarkTagAttributes class contains attributes for section mark tags.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/36943ef9-e37b-1238-0997-d786e733b781)

### Constructors
- `SectionMarkBase.SectionMarkTagAttributes(...)` — Creates a new section mark tag attributes instance.
- `SectionMarkBase.SectionMarkTagAttributes(...)` — Creates a new section mark tag attributes instance with the given information.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/6fc31695-b20b-b0f1-010f-0627515c22eb)

### Properties
- `Location` (object, get/set) — The location of the mark tag.
- `Offset` (object, get/set) — The offset of the mark tag.
- `ShowOnSide` (object, get/set) — The side(s) on which the tag is shown.
- `TagContent` (object, get/set) — The tag content.
- `TagRotation` (object, get/set) — The policy for the tag rotation.

## SectionMarkBase.SectionMarkTagAttributes.TagShowOnSide (enum)

The values that determine on which side(s) the mark tag text is shown.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/600a0cdd-702c-b984-b35e-5310be105fad)

### Values
- `ShowOnBothSides` = `1` — Show on both sides.
- `ShowOnLeftSideOnly` = `2` — Show only on the left side.
- `ShowOnRightSideOnly` = `3` — Show only on the right side.

## SectionMarkBase.SectionMarkTagAttributes.TagTextRotation (enum)

The mark tag text rotation values.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a8e4a0bf-8d73-5f8a-1cf5-3f86921a4d8e)

### Values
- `AlwaysRotate` = `1` — Always rotate.
- `AlwaysHorizontal` = `2` — Always horizontal.
- `DoNotRotateVertically` = `3` — Do not rotate vertically.
- `AlwaysOrthogonal` = `4` — Always orthogonal.

## SectionMarkBase.SectionMarkTagsAttributes (class)

The SectionMarkTagsAttributes class contains attributes for section mark tags.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6a08c09a-d36f-31ec-a859-ff21cf495ba3)

### Constructors
- `SectionMarkBase.SectionMarkTagsAttributes(...)` — Creates a new mark tags attributes instance.
- `SectionMarkBase.SectionMarkTagsAttributes(...)` — Creates a new mark tags attributes instance.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/5c46c46d-9f88-cbe4-8231-2039e196b773)

### Properties
- `TagA1` (object, get/set) — The tag A1 of a mark.
- `TagA2` (object, get/set) — The tag A2 of a mark.
- `TagA3` (object, get/set) — The tag A3 of a mark.
- `TagA4` (object, get/set) — The tag A4 of a mark.
- `TagA5` (object, get/set) — The tag A5 of a mark.

## SinglePartDrawing (class)

The SinglePartDrawing class is for handling single part drawings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a20d3504-8756-95d2-a1e5-e4813005771d)

### Constructors
- `SinglePartDrawing(...)` — Instantiates a new single part drawing with standard attributes.
- `SinglePartDrawing(...)` — Instantiates a new single part drawing with standard attributes and with a specified sheet number.
- `SinglePartDrawing(...)` — Instantiates a new single part drawing with given attributes.
- `SinglePartDrawing(...)` — Instantiates a new single part drawing with standard attributes and with a specified sheet number.

### Methods
#### `CommitChanges(String)(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/92109855-5313-7446-bef4-ab6dde6875f1)

#### `CommitChanges.(...)`

Commits the changes made to the drawings database so far. Commit inserts a kind of save-point into the database and the save-point will be retrieved if the user does undo. Commit also executes all messages. Please note that you need to also to save the drawings database to persists the changes to disk.

[Docs](https://developer.tekla.com/topic/en/18/47/bc26a611-bd6a-535a-4461-943b8bf5351c)

#### `Delete(...)`

Deletes the drawing. An active drawing cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/47/4bcff604-27c4-487f-bb7d-5d80a2d87d8b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetPlotFileName(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/fa70baa3-ad78-2e3c-88ca-0b77cb81cd21)

#### `GetPlotFileNameExt(...)`

Get plot file name.

[Docs](https://developer.tekla.com/topic/en/18/47/6288f4e4-4baf-656d-57bb-c63dee354493)

#### `GetSheet(...)`

Gets the sheet of the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fe921886-00b9-57d3-0488-92bd25014c66)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `Insert(...)`

Inserts a new single part drawing. Views are added according to the View Creation rules of the standard file. A drawing can be inserted only when there is no active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/dfe6c343-3aa8-1e53-01cf-dcd9c6622f02)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/6b1b3008-c9dd-5b97-0881-89ccc580b3e7)

#### `Modify(...)`

Updates the drawing object in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/0e24ea81-402d-8821-e2d9-86c95a78c834)

#### `PlaceViews(...)`

Places views on the sheet. Computes the required size of the views to fit the parts, then places the views so that they fit on the sheet and do not overlap. Adjusts the sheet size if needed.

[Docs](https://developer.tekla.com/topic/en/18/47/1e1578e4-084f-21a9-3d1a-2b03140440ce)

#### `Select(...)`

Selects the drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/cd990fe6-97b8-366f-859d-d80435f7bc96)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

### Properties
- `CommitMessage` (object, get/set) — Gets the commit message
- `CreationDate` (object, get/set) — The drawing creation date.
- `DrawingTypeStr` (object, get/set) — The type of the drawing
- `IsFrozen` (object, get/set) — Describes whether the drawing is frozen.
- `IsIssued` (object, get/set) — Describes whether the drawing is issued.
- `IsIssuedButModified` (object, get/set) — Describes whether the drawing is issued but modified since it was issued.
- `IsLocked` (object, get/set) — Describes whether the drawing is locked.
- `IsLockedBy` (object, get/set) — Gets the logged in username that set the drawing to locked
- `IsMasterDrawing` (object, get/set) — Describes whether the drawing is a master drawing (shown with a special symbol on the drawing list).
- `IsReadyForIssue` (object, get/set) — Describes whether the drawing is ready for issue.
- `IsReadyForIssueBy` (object, get/set) — Gets the logged in username that set the drawing Ready for issue
- `IssuingDate` (object, get/set) — The drawing issuing date.
- `Layout` (object, get/set) — Gets the drawing layout attributes.
- `Mark` (object, get/set) — The drawing mark.
- `ModificationDate` (object, get/set) — The drawing modification date.
- `Name` (object, get/set) — The name of the drawing.
- `OutputDate` (object, get/set) — The drawing output date. XS_DISABLE_DRAWING_PLOT_DATE controls whether this date is set.
- `PartIdentifier` (object, get/set) — The identifier of the model object part.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SectionViewStartLabel` (object, get/set) — Gets the drawing attribute for start label of section view
- `SheetNumber` (object, get/set) — The sheet number of the drawing.
- `Title1` (object, get/set) — The first drawing title.
- `Title2` (object, get/set) — The second drawing title.
- `Title3` (object, get/set) — The third drawing title.
- `UpToDateStatus` (object, get/set) — Gets the drawing up to date status.

## Size (class)

The Size class contains the width and height properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7a8f20c3-67f0-3571-d11c-1ca129c00738)

### Constructors
- `Size(...)` — Creates a new size instance with zero width and height.
- `Size(...)` — Creates a new size instance with the specified width and height.

### Methods
#### `ToString(IFormatProvider, String)(...)`

Returns a string that represents this instance (in fractional inches if advanced option XS_USE_ONLY_INCHES_IN_SHEET_SIZES set to true).

[Docs](https://developer.tekla.com/topic/en/18/47/1b4b2d55-5e98-707a-b193-91ecb9428738)

#### `ToString(String)(...)`

Returns a string that represents this instance (in fractional inches if advanced option XS_USE_ONLY_INCHES_IN_SHEET_SIZES set to true).

[Docs](https://developer.tekla.com/topic/en/18/47/9731ad65-0f62-db52-cba4-b742413b369b)

#### `ToString.(...)`

Returns a string that represents this instance (in fractional inches if advanced option XS_USE_ONLY_INCHES_IN_SHEET_SIZES set to true).

[Docs](https://developer.tekla.com/topic/en/18/47/1ef52bd4-9074-7210-56ff-6c3cf681ca06)

### Properties
- `Height` (object, get/set) — Gets or sets the height value of the size instance.
- `Width` (object, get/set) — Gets or sets the width value of the size instance.

## SizeDefinitionMode (enum)

Controls the size definition mode of the drawing sheet. Set the mode to SizeDefinitionMode.SpecifiedSize if you want to force the sheet to a certain size.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ac0235fb-263e-d1b5-334b-283dec46145e)

### Values
- `AutoSize` = `0` — The smallest possible drawing sheet is automatically calculated.
- `SpecifiedSize` = `1` — Fixed values are used to control the sheet size.

## SpaceElement (class)

The SpaceElement class defines a space between the desired elements.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/51836ed0-86ad-cb8b-ea56-eb5f2a8645c9)

### Constructors
- `SpaceElement(...)` — Initializes a new instance of the SpaceElement class

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/397b8b91-583d-295c-801a-b2847d807561)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/b8551ee4-d132-407b-2f23-0da21a35b230)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/ca9411db-9480-72cc-fc6c-b0a27767c50d)

## StraightDimension (class)

The StraightDimension class defines straight dimensions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/473fa702-2679-7d63-f53a-da67153f8c3e)

### Constructors
- `StraightDimension(...)` — Initializes a new instance of the StraightDimension class using "standard" attributes.
- `StraightDimension(...)` — Initializes a new instance of the StraightDimension class.

### Methods
#### `Delete(...)`

Deletes the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/4245f304-e8dd-4749-3eeb-4411721f478e)

#### `GetDimensionSet(Boolean)(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/815002cc-f8bb-e90c-4768-f44d44a95d74)

#### `GetDimensionSet.(...)`

Gets the dimension set the current dimension belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/29ac1286-a4b2-3119-7f4f-4d050ad1cc2f)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the straight dimension to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/016b4366-3683-aa10-e34c-726500bab2e5)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/76b1a3e9-efb3-0c77-d961-bd750536c0bb)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the existing straight dimension in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/eaf36c9f-5c2e-feb6-cb26-731b6b267759)

#### `Select(...)`

Selects the straight dimension from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/2f490645-5dd8-308e-b157-8167042329e1)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the dimension's attributes.
- `Distance` (object, get/set) — Gets or sets the distance of the dimension line from the start point of the dimension.
- `EndPoint` (object, get/set) — Gets or sets the end point of the dimension.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `StartPoint` (object, get/set) — Gets or sets the start point of the dimension.
- `UpDirection` (object, get/set) — Gets or sets the direction from the dimension points to the dimension line.
- `Value` (object, get/set) — Gets the content of the dimension mark.

## StraightDimension.StraightDimensionAttributes (class)

The StraightDimensionAttributes class contains the attributes of the straight dimension.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c95a6a34-4067-25fa-8453-60b87f64c66f)

### Constructors
- `StraightDimension.StraightDimensionAttributes(...)` — Initializes a new instance of the StraightDimension.StraightDimensionAttributes class

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/cb6f2f50-1a59-6848-90c1-f19b5adcc489)

#### `LoadAttributes(...)`

The attributes can't be loaded.

[Docs](https://developer.tekla.com/topic/en/18/47/3725a98d-a2e9-323f-4362-4123b635085f)

### Properties
- `DimensionValuePostfix` (object, get/set) — The postfix for the dimension's value.
- `DimensionValuePrefix` (object, get/set) — The prefix for the dimension's value.
- `Exaggeration` (object, get/set) — The exaggeration attributes of the dimension.
- `MiddleLowerTag` (object, get/set) — The dimension tag placed under the dimension value.

## StraightDimensionSet (class)

The StraightDimensionSet class defines a straight dimension set.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/749836b2-8af0-c2d5-6c6d-9967c7cb2f20)

### Methods
#### `AddToDimensionSet(...)`

Adds a dimension set to the current dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/c6ae7b87-49aa-8bb4-456a-61e0f37e1637)

#### `Delete(...)`

Deletes the dimension set and its child dimensions from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/7223e007-2a6f-dead-55b2-5710a0990ae7)

#### `GetAllExcludePartsAccordingToFilter(...)`

Retrieves all the possible valid filters for ExcludePartsAccordingToFilter.

[Docs](https://developer.tekla.com/topic/en/18/47/92561556-1f17-2311-fc6e-fbb7bf74020c)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjects(.Type.)(...)`

Gets the children straight dimensions of the current straight dimension set that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/0ec2481b-08c7-d5e3-41e4-291bfbb31bbc)

#### `GetObjects.(...)`

Gets the children straight dimensions of the current straight dimension set.

[Docs](https://developer.tekla.com/topic/en/18/47/a5b00ccb-a302-c9bd-b940-6b64af7bccdc)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Dimension sets cannot be inserted into the database.

[Docs](https://developer.tekla.com/topic/en/18/47/49e573ba-c27a-3730-4059-4064670a6e69)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/acff1b2d-7d45-08e6-724e-46a0e7141381)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the existing dimension set in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/745b3504-198a-6ac1-47ba-d681f11c6bf2)

#### `Select(...)`

Selects the straight dimension set from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/292334f3-57c9-a259-b617-7ef70a610e5b)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The straight dimension set attributes.
- `Distance` (object, get/set) — The distance from the first dimension point to the dimension line. The distance is measured in paper millimeters.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `LeftTagLineOffset` (object, get/set) — The offset of the left tag line. The offset is measured in paper millimeters.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `RightTagLineOffset` (object, get/set) — The offset of the right tag line. The offset is measured in paper millimeters.

## StraightDimensionSet.StraightDimensionSetAttributes (class)

The StraightDimensionSetAttributes class contains the attributes for managing all the attributes of the straight dimension.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f7cd1b5b-2dba-2eee-cf5d-198239da028b)

### Constructors
- `StraightDimensionSet.StraightDimensionSetAttributes(...)` — Creates a default straight dimension set attributes instance that loads standard attributes.
- `StraightDimensionSet.StraightDimensionSetAttributes(...)` — Creates a straight dimension set attributes instance that loads the specified attributes.
- `StraightDimensionSet.StraightDimensionSetAttributes(...)` — Creates a default straight dimension set attributes instance that loads standard attributes.
- `StraightDimensionSet.StraightDimensionSetAttributes(...)` — Creates a straight dimension set attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/1af5065a-8f4d-e2a0-196f-d21a07480b5e)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/9aa1604e-5949-3a5a-2c9e-85648d2384b4)

### Properties
- `Arrowhead` (object, get/set) — The arrowhead attributes of the dimension.
- `Color` (object, get/set) — The color of the dimension line.
- `CombinedDimension` (object, get/set) — Combines the attributes of the dimension.
- `DimensionType` (object, get/set) — The dimension type.
- `Exaggeration` (object, get/set) — The exaggeration attributes of the dimension.
- `ExcludePartsAccordingToFilter` (object, get/set) — Sets the filter to use for excluding parts from the tags. You may query the possible filters using StraightDimensionSet.GetAllExcludePartsAccordingToFilter()
- `ExtensionLine` (object, get/set) — The extension line presentation of the dimension.
- `Format` (object, get/set) — The format attributes of the dimension.
- `IncludePartCountInTag` (object, get/set) — Gets or sets whether the part count is included in at least one of the tag position, or set part count flag for all tag positions.
- `IncludePartCountInTagA` (object, get/set) — Gets or sets a value indicating whether the part count is included in the tag A.
- `IncludePartCountInTagB` (object, get/set) — Gets or sets a value indicating whether the part count is included in the tag B.
- `IncludePartCountInTagC` (object, get/set) — Gets or sets a value indicating whether the part count is included in the tag C.
- `IncludePartCountInTagD` (object, get/set) — Gets or sets a value indicating whether the part count is included in the tag D.
- `IncludePartCountInTagE` (object, get/set) — Gets or sets a value indicating whether the part count is included in the tag E.
- `IncludePartCountInTagF` (object, get/set) — Gets or sets a value indicating whether the part count is included in the tag F.
- `IncludePartCountInTagG` (object, get/set) — Gets or sets a value indicating whether the part count is included in the tag G.
- `LeftLowerTag` (object, get/set) — The left lower dimension tag.
- `LeftMiddleTag` (object, get/set) — The left middle dimension tag.
- `LeftUpperTag` (object, get/set) — The left upper dimension tag.
- `Placing` (object, get/set) — The placing attributes of the dimension.
- `RightLowerTag` (object, get/set) — The right lower dimension tag.
- `RightMiddleTag` (object, get/set) — The right middle dimension tag.
- `RightUpperTag` (object, get/set) — The right upper dimension tag.
- `ShortDimension` (object, get/set) — How short dimensions are shown.
- `Text` (object, get/set) — The text attributes of the dimension.
- `TransparentBackground` (object, get/set) — The dimension text background transparency. (transparent/opaque)
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the dimension line.

## StraightDimensionSetHandler (class)

The StraightDimensionSetHandler class contains operations for creating and modifying straight dimension sets.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/13408e9f-2759-ac34-b4b2-30597ce5e73f)

### Constructors
- `StraightDimensionSetHandler(...)` — Initializes a new instance of the StraightDimensionSetHandler class

### Methods
#### `CreateDimensionSet(ViewBase, PointList, Vector, Double)(...)`

Creates straight dimensions and combines them to one straight dimension set. Returns null if fails.

[Docs](https://developer.tekla.com/topic/en/18/47/506ac2af-0156-0c5f-3531-063cbe11a51d)

#### `CreateDimensionSet(ViewBase, PointList, Vector, Double, Double, Double)(...)`

Creates straight dimensions and combines them to one straight dimension set. Returns null if fails.

[Docs](https://developer.tekla.com/topic/en/18/47/56bf2ebf-e514-45d2-121c-c41d1f8644b0)

#### `CreateDimensionSet(ViewBase, PointList, Vector, Double, Double, Double, StraightDimensionSet.StraightDimensionSetAttributes)(...)`

Creates straight dimensions and combines them to one straight dimension set. Returns null if fails.

[Docs](https://developer.tekla.com/topic/en/18/47/392a2299-5b18-4fe2-09bc-43365daf5494)

#### `CreateDimensionSet(ViewBase, PointList, Vector, Double, StraightDimensionSet.StraightDimensionSetAttributes)(...)`

Creates straight dimensions and combines them to one straight dimension set. Returns null if fails.

[Docs](https://developer.tekla.com/topic/en/18/47/5ce3babb-67f5-6f3a-e078-4b40adb04967)

## StringList (class)

The StringList class defines a type safe string collection.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a6cf4d7f-3cda-d1ed-79ab-ac5ead707295)

### Constructors
- `StringList(...)` — Creates a new string list instance.
- `StringList(...)` — Creates a new string list instance with the given capacity.

### Methods
#### `Add(...)`

Adds a new string to the end of the string list.

[Docs](https://developer.tekla.com/topic/en/18/47/fc0068a0-ffb4-23a5-8a95-e02ec5836e20)

#### `Contains(...)`

Checks if the string is in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/7712ff56-f320-f4d0-6f98-900066e3c931)

#### `GetRange(...)`

Gets a part of the string list.

[Docs](https://developer.tekla.com/topic/en/18/47/eea5d9d5-8b63-8bb6-ccc6-0231864fd401)

#### `IndexOf(String)(...)`

Gets the index of the specified string.

[Docs](https://developer.tekla.com/topic/en/18/47/d13aab21-684a-72f7-2c2e-bda2deae6bbe)

#### `IndexOf(String, Int32)(...)`

Gets the index of the specified string between the index to start the search from and the end of the list.

[Docs](https://developer.tekla.com/topic/en/18/47/717ab436-5319-c27c-b63f-65db9a5425a7)

#### `IndexOf(String, Int32, Int32)(...)`

Gets the index of the specified string between the index to start the search from and startIndex + count.

[Docs](https://developer.tekla.com/topic/en/18/47/87ab9055-6834-3f94-0d08-56a5ed1c1183)

#### `Insert(...)`

Inserts the string to the specified index.

[Docs](https://developer.tekla.com/topic/en/18/47/be01405b-81c0-615a-0ffb-c002f21d4d0a)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/13fa4807-e6f2-2068-254c-6667ca2f9346)

#### `LastIndexOf(String)(...)`

Searches for the last occurence of the string in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/df56c09b-a83f-109d-cf47-79e93127f7f0)

#### `LastIndexOf(String, Int32)(...)`

Searches for the last occurence of the string in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/9fc51a21-bcdb-ebb9-c2a9-d93434d5aedf)

#### `LastIndexOf(String, Int32, Int32)(...)`

Searches for the last occurence of the string in the list.

[Docs](https://developer.tekla.com/topic/en/18/47/717161f2-b30e-d521-f960-b3d39925ae92)

#### `Remove(...)`

Removes the first occurence of the string from the list.

[Docs](https://developer.tekla.com/topic/en/18/47/29585d51-29fa-8ec1-51b4-a6d0ccc053b3)

#### `RemoveRange(...)`

Removes a range of items starting from the given index.

[Docs](https://developer.tekla.com/topic/en/18/47/46812b00-070a-9e4a-adf1-e306fe85d657)

#### `ToArray(...)`

Copies the strings to a string array.

[Docs](https://developer.tekla.com/topic/en/18/47/6d363dd2-aa3c-ca8d-bc36-c873fee76eb3)

### Properties
- `Item` (object, get/set) — Gets or sets the string at the specified index.

## Surfacing (class)

The Surfacing class contains methods related to surfacings. A surfacing is a drawing object derived from a model object. It represents a drawing surfacing that has a reference to the actual surfacing in the model database.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/95313b14-458b-8b48-d281-5cca62d1122a)

### Methods
#### `Delete(...)`

Calls the system to delete the object.

[Docs](https://developer.tekla.com/topic/en/18/47/8d48ebe2-b01e-a283-c751-823e1d6ea115)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED.

[Docs](https://developer.tekla.com/topic/en/18/47/3d270e77-3151-64d5-ff86-e250992ecf79)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4866e397-bea6-2d94-7def-e4517d8540c1)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the object.

[Docs](https://developer.tekla.com/topic/en/18/47/f63f36ef-cca6-2ba7-b23b-507641217bc5)

#### `Select(...)`

Selects the drawing surfacing from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/82420b4e-d59c-a99a-aa1e-7064e0441b7d)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the surfacing. For more information see Surfacing.SurfacingAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Surfacing.Representation (enum)

The surfacing representations.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/12021b92-4283-acf8-69db-3025a78584fa)

### Values
- `Outline` = `1` — The outline representation.
- `Symbol` = `2` — The symbol representation.
- `WorkshopForm` = `4` — The workshop form representation.
- `Exact` = `8` — The exact representation.
- `BoundingBox` = `16` — The bounding box representation.
- `BaseBox` = `32` — The base box representation.

## Surfacing.SurfacingAttributes (class)

The SurfacingAttributes class is the attributes class for the surfacing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b0b71097-ef0e-95a2-5a44-398c01b6f009)

### Constructors
- `Surfacing.SurfacingAttributes(...)` — Creates a new default surfacing attributes instance that loads standard attributes.
- `Surfacing.SurfacingAttributes(...)` — Creates a new surfacing attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/4b4ee56c-622d-74d8-99f1-bcf616d3fb67)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/53def60f-8695-9925-0755-4fc288c1bb65)

### Properties
- `DrawHiddenLines` (object, get/set) — True if hidden lines are drawn.
- `DrawOwnHiddenLines` (object, get/set) — True if own hidden lines are enabled.
- `HiddenLines` (object, get/set) — The line type of hidden lines.
- `Representation` (object, get/set) — The representation type of the surfacing.
- `ShowPattern` (object, get/set) — True if the pattern is shown.
- `VisibleLines` (object, get/set) — The line type of visible lines.

## Symbol (class)

The Symbol class defines a drawing object that is displayed as a symbol in a drawing. Symbols are located in the environment directory under the folder named "symbols" in .sym files that can contain up to 255 symbols. Symbol files can be created with SymEd. See the Tekla Structures help for more information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/dc665636-f4f7-758f-1e69-414de20de61c)

### Constructors
- `Symbol(...)` — Creates a new symbol instance using the default symbol information and standard attributes.
- `Symbol(...)` — Creates a new symbol instance using the default symbol information, standard attributes and the given placing.
- `Symbol(...)` — Creates a new symbol instance using standard attributes.
- `Symbol(...)` — Creates a new symbol instance using standard attributes.
- `Symbol(...)` — Creates a new symbol instance using the given attributes.
- `Symbol(...)` — Creates a new symbol instance using the given attributes and placing.

### Methods
#### `Delete(...)`

Deletes the symbol from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/88becf47-bf4b-6f7b-509e-961f0a747456)

#### `GetAxisAlignedBoundingBox(...)`

Returns the bounding box of the symbol (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/733aed46-70c1-2a60-a37d-b127a4d74e1b)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Returns the bounding box of the symbol (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/d222a75a-6250-447b-2904-44b9617ec57b)

#### `GetObjects(.Type.)(...)`

Gets the children objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/bda97323-4f4d-38e2-0164-17919a253bd8)

#### `GetObjects.(...)`

Gets the children objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/dccf780b-3136-ff9b-60cc-c26c9730906c)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the symbol into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/0a2bef10-775f-af99-b185-57ae86f59c6c)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/53a34438-cdeb-849c-584f-d43d12fbd6ee)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the symbol in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/92ae3489-79fe-69f4-cba1-9172b953e605)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/a3ebbc30-658d-6fe4-2bd9-f5c02c74d183)

#### `Select(...)`

Selects the symbol from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/01ddae45-65fd-959e-0c60-d354137381f4)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the symbol attributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `InsertionPoint` (object, get/set) — Gets or sets the symbol insertion point.
- `Placing` (object, get/set) — The current placing of the object. See the placing types for different placing options.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `SymbolInfo` (object, get/set) — Gets or sets the symbol information.

## Symbol.SymbolAttributes (class)

The SymbolAttributes class contains the attributes for the symbol.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b10faf4e-f57b-f8ce-e4b7-6127b94600e7)

### Constructors
- `Symbol.SymbolAttributes(...)` — Creates a new default symbol attributes instance that loads standard attributes.
- `Symbol.SymbolAttributes(...)` — Creates a symbol attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/6ccd8468-c5e8-7d1e-0de9-78423f575142)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/7bbea6d2-d775-8e1a-ad4b-4fae564d6b74)

### Properties
- `Angle` (object, get/set) — The angle of the symbol. Cannot be changed for an editor-created symbol when the symbol is created "along line".
- `Color` (object, get/set) — The symbol color. Does not have an effect on symbols where the color is already defined in the symbol file.
- `Frame` (object, get/set) — The frame for the symbol.
- `Height` (object, get/set) — The height of the symbol.
- `PreferredPlacing` (object, get/set) — The type of placing the object should use. See the preferred placing types for different types of placing.
- `TrueColor` (object, get/set) — The true color (RGB enabled) of the symbol. Does not have an effect on symbols where the color is already defined in the symbol file.

## SymbolElement (class)

The SymbolElement class represents a symbol in the Tekla Structures symbol library.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ec0e7aaa-df5c-48a8-bded-69228d9b9b11)

### Constructors
- `SymbolElement(...)` — Creates a new symbol element instance with the default symbol.
- `SymbolElement(...)` — Creates a new symbol element instance with the given symbol.
- `SymbolElement(...)` — Creates a new symbol element instance with the given symbol, color and height.
- `SymbolElement(...)` — Creates a new symbol element instance with the given symbol, the RGB color and height.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/1d7480df-5842-f41e-3eac-d5a8d5570b7b)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/016571de-9ea1-5da5-8812-d3fe8683e142)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/af6fb1a7-fa73-5b8e-d06d-94f761310e69)

### Properties
- `Color` (object, get/set) — This property is the symbol color.
- `Height` (object, get/set) — The symbol height (mm).
- `Symbol` (object, get/set) — The symbol.
- `TrueColor` (object, get/set) — The true color (rgb enabled) of the symbol.

## SymbolInfo (class)

The SymbolInfo class holds symbol information: the symbol file and the symbol index in the symbol file.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c9fe0d20-c02c-e062-a86e-ee0f3f1beb5a)

### Constructors
- `SymbolInfo(...)` — Creates a new symbol info instance.
- `SymbolInfo(...)` — Creates a new symbol info instance.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/93299518-9929-1413-753b-a96c1aa722bc)

### Properties
- `SymbolFile` (object, get/set) — Gets or sets the symbol file. E.g. "xsteel".
- `SymbolIndex` (object, get/set) — Gets or sets the the symbol index. Throws an ArgumentOutOfRangeException if the symbol index is not in the range from 0 to 255.

## SymbolLibrary (class)

The SymbolLibrary class is for handling symbol libraries.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/83f5e488-a939-d16f-cb31-7b27f36d8886)

### Methods
#### `GetSymbolLibraries(Boolean)(...)`

Gets the names of the available symbol files in the current Tekla Structures environment, in the "symbols" directory, without the suffix.

[Docs](https://developer.tekla.com/topic/en/18/47/d0213ef2-9232-62f2-9c58-ba7b53e5f454)

#### `GetSymbolLibraries.(...)`

Gets the names of the available symbol files in the current Tekla Structures environment, in the "symbols" directory, without the suffix.

[Docs](https://developer.tekla.com/topic/en/18/47/1a52bc25-6891-60fb-c459-d8531a91f5ae)

## TagLocation (enum)

The mark tag location values.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cf4a5079-333d-3f02-24ca-d86be19552f1)

### Values
- `AboveLine` = `1` — The mark tag is above the line.
- `BelowLine` = `2` — The mark tag is below the line.
- `MiddleLevelOfSymbol` = `3` — The mark tag is in the middle level of the mark symbol.
- `AboveSymbolCenterLine` = `4` — The mark tag is above the center line of the mark symbol.
- `BelowSymbolCenterLine` = `5` — The mark tag is below the center line of the mark symbol.
- `CustomRelativeToSymbol` = `6` — The mark tag is in a custom location relative to the mark symbol.

## TeklaDrawingColor (struct)

The TeklaDrawingColor struct contains a compact storage for drawing color data This struct is intended to be an "all-in-one" solution for OpenAPI users interacting with drawing colors. It supports old enums and new user defined RGB colors. The essential principle of this struct is that once an object of this struct is created (i.e. you have one of these objects instantiated), it contains valid RGB information, as well as a valid enum value in the case of a standard color. This means users will never have to run checks to see if their object has a valid RGB value. After all, if they receive this "TrueColor" object, they expect we can provide them RGB information. To support this functionality, we need to make sure that there is RGB information present for every single object of this type that is created. All objects constructed with RGB values are guaranteed to have this, as well as the ordinary standard drawing colors with values between 0 and 255. Other indices are forbidden, and will cause an error upon construction attempt. Given that the constructor of this struct can throw an exception, the struct should never be used as a "middleman", meaning it should never be used to pass color information from one place to another within the codebase: int color -> TeklaDrawingColor -> int color.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6e7f1080-8544-0c94-f532-8ab243698da9)

### Constructors
- `TeklaDrawingColor(...)` — Default constructor for the struct. Assigns color to standard black.
- `TeklaDrawingColor(...)` — Construct a TeklaDrawingColor instance by passing in an RGB System.Drawing.Color value. This is the safe non-throwing constructor to use.
- `TeklaDrawingColor(...)` — Construct a TeklaDrawingColor instance by passing in a Tekla standard color enum value. Use of this constructor is discouraged, please use TeklaDrawingColor(Color RGB)
- `TeklaDrawingColor(...)` — Construct a TeklaDrawingColor instance by passing in a Tekla standard color hatch enum value. Use only for hatches.

### Properties
- `DrawingColorEnum` (object, get/set) — Enum for the drawing color
- `RGBColor` (object, get/set) — Red, Green, Blue representation of the color.

## TeklaStructuresDrawingsApplicationException (class)

The TeklaStructuresDrawingsApplicationException class defines the base class for all exceptions that can be thrown while using Tekla.Structures.Drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/03b4176e-9f3e-ae2e-c94a-18404d14ce7d)

### Constructors
- `TeklaStructuresDrawingsApplicationException(...)` — Creates a new exception instance.

## TemplateInfo (class)

The TemplateInfo class holds template information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/874545f9-0b80-904b-71b8-5a002ac6fcd8)

### Constructors
- `TemplateInfo(...)` — Creates a new template info instance.
- `TemplateInfo(...)` — Creates a new template info instance with the given template file name.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/bd278648-b27d-782e-56a4-9a48782ae669)

### Properties
- `Name` (object, get/set) — Gets or sets the name of the template file. E.g. "revision".

## Text (class)

The Text class defines a drawing object that can hold a text string and is shown in a drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a0397063-758c-2266-20c3-a70083fc302c)

### Constructors
- `Text(...)` — Creates a new text instance with the given text string.
- `Text(...)` — Creates a new text instance with the given text string using a specific placing type.
- `Text(...)` — Creates a new text instance with the given text string and text attributes.
- `Text(...)` — Creates a new text instance with the given text string and text attributes using a specific placing type.

### Methods
#### `Delete(...)`

Deletes the text from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/8046ec4d-7495-401b-36bf-ab19112abb3f)

#### `GetAxisAlignedBoundingBox(...)`

Returns the axis aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/4c9d4d1f-8fb7-255f-803c-e935b084fbde)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Returns the object aligned bounding box of the text (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/df12c0cc-3b69-b142-0716-7c196694f0db)

#### `GetObjects(.Type.)(...)`

Gets the children objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/2e30931a-cc02-c1f4-2a59-27f44ff519b2)

#### `GetObjects.(...)`

Gets the children objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/59774d78-87e8-44c4-a0c8-d38d85c81bd5)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the text into the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/f0c01941-938a-df07-c1e3-23bcd0ce72fc)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/73d1e9ce-2460-b66c-4107-0c4ef2adb354)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the text in the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/fb30fcb4-25c1-d1e8-1896-83757c015bb2)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/5c3f5b5b-afdb-a6d9-ce4a-b03f13f88f14)

#### `Select(...)`

Selects the text from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/38543e63-4b92-66c8-b912-12d53361d233)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes for the text.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `InsertionPoint` (object, get/set) — The insertion point of the text.
- `IsRichText` (object, get/set) — Gets a value indicating whether the text contains rich text formatting (RTF). Returns false if the text is plain text or an empty string.
- `Placing` (object, get/set) — The current placing of the object. See the placing types for different placing options.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `ResizeBehavior` (object, get/set) — The text resize behavior.
- `TextString` (object, get/set) — The contents of the text.

## Text.TextAttributes (class)

The TextAttributes class is the attributes class for the text.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/67e8c418-3157-dad6-7057-82f1e31c42c0)

### Constructors
- `Text.TextAttributes(...)` — Creates a new default text attributes instance that loads standard attributes.
- `Text.TextAttributes(...)` — Creates a new text attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/7ebbe26b-4b27-5290-0d70-7489b2a11e00)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/8b850f20-a238-96c8-db85-d1ff5e368993)

### Properties
- `Alignment` (object, get/set) — The text object's alignment (left, center, right).
- `Angle` (object, get/set) — The angle of the text frame.
- `ArrowHead` (object, get/set) — The arrowhead of the leader line.
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `Font` (object, get/set) — The text object's font settings.
- `Frame` (object, get/set) — The frame for the text.
- `PlacingAttributes` (object, get/set) — The placing attributes that the object should use. Using these you can specify whether the object is automatically arranged in the drawing or not.
- `PreferredPlacing` (object, get/set) — The type of placing the object should use. See the preferred placing types for different types of placing.
- `RulerWidth` (object, get/set) — Gets or sets the width in paper coordinates of the ruler to be used in word wrapping.
- `TransparentBackground` (object, get/set) — Gets or sets a value indicating whether the background transparency is enabled for texts.
- `UseWordWrapping` (object, get/set) — Gets or sets a value indicating whether to [use word wrapping].

## TextAlignment (enum)

The alignment of the text.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/730bcec8-2bb6-29dd-7010-4d5e0f27d0ad)

### Values
- `Left` = `0` — The left aligned text.
- `Center` = `1` — The center aligned text.
- `Right` = `2` — The right aligned text.
- `LeaderLine` = `3` — The text is aligned to leader line direction

## TextElement (class)

The TextElement class defines a user-defined text in a mark content.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/71636693-ebad-f893-1ca0-95a85d63a091)

### Constructors
- `TextElement(...)` — Creates a new text element instance with the given textual value.
- `TextElement(...)` — Creates a new text element instance with the given textual value and font.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/765f3e69-9834-0f36-e506-3d9fec1cbe12)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/185c6e84-13cd-8f89-7cb7-a52913ce103f)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/542e3a8d-9da6-f6fd-07ab-d14a76a05ad2)

### Properties
- `Font` (object, get/set) — The font of the text element.
- `Value` (object, get/set) — The textual value of the text element.

## TextFile (class)

The TextFile class defines a text file object in a drawing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d6c4338c-95d8-e22d-c7cd-44d9ca5daef5)

### Constructors
- `TextFile(...)` — Creates a new text file instance using standard attributes.
- `TextFile(...)` — Creates a new text file instance using standard attributes.
- `TextFile(...)` — Creates a new text file instance using specific attributes.
- `TextFile(...)` — Creates a new text file instance using specific attributes.

### Methods
#### `Delete(...)`

Deletes the text file object.

[Docs](https://developer.tekla.com/topic/en/18/47/f347e9e6-839f-9c4b-5a0b-5e4f2154beb7)

#### `GetAxisAlignedBoundingBox(...)`

Gets the bounding box for the current object in the global axis coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/92532071-8507-973e-5e7c-303633870edc)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Gets the bounding box for the current object in its local coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/2057db75-e1b4-d5cc-5ba4-5849af223043)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the text file object.

[Docs](https://developer.tekla.com/topic/en/18/47/ed1ca5cb-8833-4168-bcf1-b78d3b026b05)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e56bc80a-1a2e-98c3-8aca-ae891dca89d1)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the text file object.

[Docs](https://developer.tekla.com/topic/en/18/47/0d484d2f-7c6f-3855-7948-bce9338cf683)

#### `MoveObjectRelative(...)`

Moves the object according to the given move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/96087af0-e3e1-88dc-7d10-2aa68bcfe8d8)

#### `Resize(...)`

Resizes the object. The scaling type is changed to ScaleToFit.

[Docs](https://developer.tekla.com/topic/en/18/47/58cefb6f-d78a-089e-e16e-8dc0bdf1bf09)

#### `Select(...)`

Selects the text file object.

[Docs](https://developer.tekla.com/topic/en/18/47/a05c74ca-f8ed-483e-b815-cd5c88f6f699)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Angle` (object, get/set) — Gets or sets the angle (in degrees).
- `Attributes` (object, get/set) — Gets or sets the text file attributes.
- `FileName` (object, get/set) — Gets or sets the file name defining the embedded object.
- `InsertionPoint` (object, get/set) — Gets or sets the insertion point.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Size` (object, get/set) — Gets or sets the size of the frame of the embedded object.

## TextFile.TextFileAttributes (class)

The TextFileAttributes class contains the attributes for the text file object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c547587c-07dc-1a38-ccf3-592042bf2205)

### Constructors
- `TextFile.TextFileAttributes(...)` — Creates a new text file attributes instance that loads standard attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c631729a-82b1-c6c1-ed34-e077d86a0b6d)

#### `LoadAttributes(...)`

Loads the specified attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/ffaa1763-eb69-ab96-d557-fb060ce8b690)

### Properties
- `Font` (object, get/set) — Gets or sets the font.
- `Frame` (object, get/set) — Gets or sets the frame.
- `Line` (object, get/set) — Gets or sets the line type.
- `LineType` (object, get/set) — Obsolete.
- `Scaling` (object, get/set) — Gets or sets the scaling.
- `XScale` (object, get/set) — Gets or sets the X scaling value. Changing this property will set the scaling type to ScaleXY.
- `YScale` (object, get/set) — Gets or sets the Y scaling value. Changing this property will set the scaling type to ScaleXY.

## TextResizeBehavior (enum)

Text resize behavior.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cbd758f9-0513-bc56-aee5-ac7f8d15558d)

### Values
- `AllDirections` = `0` — When the text is modified, resizing happens in all directions. In this case, the insertion point will remain the same.
- `BottomRight` = `1` — When the text is modified, resizing happens in bottom and right directions. In this case, the insertion point will be modified.

## UnitAttributes (class)

The UnitAttributes class handles attributes related to units and formatting of values.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7c59d106-b059-1d79-d697-33133265f971)

### Constructors
- `UnitAttributes(...)` — Creates a new default UnitAttributes attributes instance. The default values are Units.Automatic, FormatTypes.Automatic.
- `UnitAttributes(...)` — Creates a new UnitAttributes attributes instance with the given parameters.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/a99f0c77-a698-90f2-307d-556ec55a9881)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e5dea66a-b241-1e2a-6275-085f3184c73d)

### Properties
- `Format` (object, get/set) — Gets or sets the format to use.
- `Unit` (object, get/set) — Gets or sets the unit to use.

## Units (enum)

The available unit types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a050ba82-07ea-f06f-3467-eaab7b462ca5)

### Values
- `Automatic` = `0` — Automatic, chooses the most appropriate unit for the current environment.
- `Millimeters` = `1` — Show the value in millimeters.
- `Centimeters` = `2` — Show the value in centimeters.
- `Meters` = `3` — Show the value in meters.
- `FeetAndInches` = `4` — Show the value in feet and inches.
- `CentimetersOrMeters` = `5` — Show the value in centimeters or meters whichever is more appropriate.
- `Inches` = `6` — Show the value in inches.
- `Feet` = `7` — Show the value in feet.

## UserDefinedElement (class)

The UserDefinedElement class represents a user-defined attribute specified by its name. A user-defined element is converted into a textual value before it is drawn.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d2294ddd-0810-2f6c-e414-9e64867bc2e1)

### Constructors
- `UserDefinedElement(...)` — Creates a new user-defined element instance with the given name.
- `UserDefinedElement(...)` — Creates a new user-defined element instance with the given name and font.

### Methods
#### `Clone(...)`

Clones the object.

[Docs](https://developer.tekla.com/topic/en/18/47/9d8c81d8-26c1-6ef9-1130-663591375ba2)

#### `GetUnformattedString(...)`

Returns the content as a string without formatting.

[Docs](https://developer.tekla.com/topic/en/18/47/2471ba47-3a7d-b112-a1f7-1999248324b3)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/7267437d-5400-eab8-2d2c-0f7bb7aacea1)

### Properties
- `Font` (object, get/set) — The font of the user-defined attribute.
- `Name` (object, get/set) — The name of the user-defined attribute.
- `Unit` (object, get/set) — Gets or sets the unit attributes.
- `Value` (object, get/set) — The value of the user-defined attribute.

## View (class)

The View class contains methods related to views. Views are views that contain other drawing objects, excluding other views. Views also contain drawing representations of model objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c677c912-dc7d-0e16-cc44-433195eee9d2)

### Constructors
- `View(...)` — Creates a new view instance with the given coordinate systems and part list.
- `View(...)` — Creates a new view instance with the given coordinate systems and model area.
- `View(...)` — Creates a new view instance with the given coordinate systems, part list and attributes file's filename.
- `View(...)` — Creates a new view instance with the given coordinate systems, model area and attributes file's filename.

### Methods
#### `Create3dView(Drawing, CoordinateSystem, CoordinateSystem, AABB, Point, View.ViewAttributes, View.)(...)`

Generates a new 3d view on the sheet of the given active drawing using given view plane and display plane.

[Docs](https://developer.tekla.com/topic/en/18/47/55ff2f8d-a8ab-0d68-0d86-92bb423cb8b5)

#### `Create3dView(Drawing, Point, View.ViewAttributes, View.)(...)`

Generates a new 3d view on the sheet of the given active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/b76cddf7-6db5-800f-66f1-9c0e522c6753)

#### `CreateBackView(...)`

Generates a new back view on the sheet of the given active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/0a133867-4bc0-c354-4ba1-ffa7b7d46596)

#### `CreateBottomView(...)`

Generates a new bottom view on the sheet of the given active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/4a06f3ff-0fac-dfd8-c683-5035e2e9255d)

#### `CreateCurvedSectionView(...)`

Generates a new curved section view on the sheet of the active drawing and the associated section mark for the specified view.

[Docs](https://developer.tekla.com/topic/en/18/47/96f0d083-d484-cea9-3fc0-e534f94a4fd1)

#### `CreateDetailView(...)`

Generates a new detail view on the sheet of the active drawing and the associated detail mark for the specified view.

[Docs](https://developer.tekla.com/topic/en/18/47/39279a5c-e5a3-ea02-8bdc-c8d8c7c3cea2)

#### `CreateFrontView(...)`

Generates a new front view on the sheet of the given active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/fefcfa36-befa-a3b7-2147-e6b7c6b21c54)

#### `CreateModelView(...)`

Generates a new model view on the sheet of the given active drawing using given model view id.

[Docs](https://developer.tekla.com/topic/en/18/47/5d36926b-48a0-48a2-9b51-7e9dd48c3a26)

#### `CreateSectionView(...)`

Generates a new section view on the sheet of the active drawing and the associated section mark for the specified view.

[Docs](https://developer.tekla.com/topic/en/18/47/524fe216-b878-33be-8db7-035a7da252f1)

#### `CreateTopView(...)`

Generates a new top view on the sheet of the given active drawing.

[Docs](https://developer.tekla.com/topic/en/18/47/5893bec2-ade0-9949-35de-f2f52db65e9d)

#### `Delete(...)`

Deletes the view and its children objects from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/c7b5ef15-0c29-6a2a-ef92-99fb8e0ed0b3)

#### `GetAllObjects(.Type.)(...)`

Gets all the objects and their children objects in the view that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/d9638840-5e65-49e6-cadb-2e2a8b5423fd)

#### `GetAllObjects(Type)(...)`

Gets all the objects and their children objects in the view that are of a certain type.

[Docs](https://developer.tekla.com/topic/en/18/47/d8756000-215b-c1e0-49c7-d36dc245bec8)

#### `GetAllObjects.(...)`

Gets all the objects and their children objects in the view.

[Docs](https://developer.tekla.com/topic/en/18/47/6c8f0774-654d-498b-93d5-904797569fd5)

#### `GetAxisAlignedBoundingBox(...)`

The view size in paper coordinates.

[Docs](https://developer.tekla.com/topic/en/18/47/c51d1265-7cc9-ca4c-86b0-d76fe62d8be8)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetModelObjects(Identifier)(...)`

Gets the drawing model objects based on given model object identifier. If used from sheet model objects from all views are returned.

[Docs](https://developer.tekla.com/topic/en/18/47/c59be515-bde2-42ef-2e58-fa8a4e0173ba)

#### `GetModelObjects.(...)`

Gets the model objects that are in the view.

[Docs](https://developer.tekla.com/topic/en/18/47/ec48c4dd-923d-2eae-4243-5454aa07e042)

#### `GetObjects(.Type.)(...)`

Gets the objects in the view that are of certain types. Does not return child objects (single dimensions, grid lines, views).

[Docs](https://developer.tekla.com/topic/en/18/47/e64fef01-3761-6798-cd3f-d5e5d006a37f)

#### `GetObjects.(...)`

Gets the objects in the view. Does not return child objects (single dimensions, grid lines, views).

[Docs](https://developer.tekla.com/topic/en/18/47/bd691388-4a5f-d33f-3364-d516c037cb21)

#### `GetOriginalDrawing(...)`

Gets the drawing on which the view was originally created. @see DrawingObject.GetDrawing() for retrieving the current drawing the object resides on.

[Docs](https://developer.tekla.com/topic/en/18/47/b34f08ba-6799-5511-dd98-8654ffbc1ebd)

#### `GetPossibleFollowedObjects(...)`

Get the possible rule objects for the section view. Note: this is currently only supported for section views in fabrication drawings.

[Docs](https://developer.tekla.com/topic/en/18/47/7e69cee3-8259-76b4-d8a7-98787e27aed4)

#### `GetPreferredFollowedObject(...)`

Gets the preferred followed object for the view, if it's supported. Note: this is currently only supported for section views in fabrication drawings.

[Docs](https://developer.tekla.com/topic/en/18/47/dba90d2a-d451-1160-dd06-f8f8b1e77d02)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `GetVisibleAreaRestrictionBoxes(...)`

Gets the restriction boxes that represent the visible areas of the view.

[Docs](https://developer.tekla.com/topic/en/18/47/398218d2-fd27-947f-5c2c-c5285109d398)

#### `Insert(...)`

Inserts the view to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/c5735e20-f3b0-3147-4879-3e1e42c30d82)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/7054fc86-ce4c-0668-7ddd-1cfaae8e81bb)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the view in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/883ce75d-2048-41ab-05ba-96ffe20220aa)

#### `RotateViewOnAxisX(...)`

Rotate the view on the X axis.

[Docs](https://developer.tekla.com/topic/en/18/47/5773a49d-274a-6f1a-a489-53fd7f7b8f0f)

#### `RotateViewOnAxisY(...)`

Rotate the view on the Y axis.

[Docs](https://developer.tekla.com/topic/en/18/47/c9e316b0-ee4f-8c1b-a859-672fa91f2f1f)

#### `RotateViewOnAxisZ(...)`

Rotate the view on the Z axis.

[Docs](https://developer.tekla.com/topic/en/18/47/7daeae28-0a89-b6a5-cd69-1973544a1298)

#### `RotateViewOnDrawingPlane(...)`

Rotate the view on the drawings plane. This works just like the user interface command "Rotate view".

[Docs](https://developer.tekla.com/topic/en/18/47/7448119c-d5c0-391c-1ee6-fdc2aa42fcf6)

#### `Select(...)`

Selects the view from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/4630a41b-4e54-0f11-a21d-eed889061760)

#### `SetPreferredFollowedObject(...)`

Sets the preferred rule object for the section view based on model object ID or Guid. Note: this is currently only supported for section views in fabrication drawings.

[Docs](https://developer.tekla.com/topic/en/18/47/7dcc461e-02c7-91d8-82fd-91f519443e17)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The view attributes.
- `DisplayCoordinateSystem` (object, get/set) — The coordinate system of the display plane. This can be used to transform global points from the model that is to be inserted in the view.
- `ExtremaCenter` (object, get/set) — Gets the view extrema center.
- `FrameOrigin` (object, get/set) — The vector from the view origin to the frame origin.
- `Height` (object, get/set) — The height of the view frame in paper coordinates.
- `IsSheet` (object, get/set) — Gets a value indicating whether the container view is the drawing's sheet.
- `Name` (object, get/set) — The name of the view.
- `Origin` (object, get/set) — The view origin coordinates in the sheet.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `RestrictionBox` (object, get/set) — The axis aligned bounding box that represents the volume of the view.
- `ViewCoordinateSystem` (object, get/set) — The coordinate system of the view plane.
- `ViewType` (object, get/set) — The view type information.
- `Width` (object, get/set) — The width of the view frame in paper coordinates.

## View.HorizontalLabelPosition (enum)

The horizontal label position types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2b927757-222e-2465-f880-2410862df005)

### Values
- `CenteredByViewFrame` = `1` — The label is centered by the view frame.
- `CenteredByViewRestrictionBox` = `2` — The label is centered by the view restriction box.
- `LeftAlignedByViewFrame` = `3` — The label is left aligned by the view frame.
- `RightAlignedByViewFrame` = `4` — The label is right aligned by the view frame.
- `LeftAlignByViewRestrictionBox` = `5` — The label is left aligned by the view restriction box.
- `RightAlignByViewRestrictionBox` = `6` — The label is right aligned by the view restriction box.

## View.LabelLineLengthType (enum)

The label line length types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/96db0a18-547a-5b83-dadc-dd300725426f)

### Values
- `Minimal` = `1` — The label is of minimal possible length according to the label text.
- `Custom` = `2` — The label is of custom length.

## View.ShorteningCutPartType (enum)

The vertical label position types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bdeb3e98-0558-a3d5-d224-2b8f0a6eb242)

### Values
- `BothDirections` = `1` — The shortening is done to both x and y directions.
- `X_Direction` = `2` — The shortening is done only to x direction.
- `Y_Direction` = `3` — The shortening is done only to y direction.

## View.VerticalLabelPosition (enum)

The vertical label position types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0f88254d-66dc-5106-8c22-03dbfd891b58)

### Values
- `Top` = `1` — The label is located on the top of the view.
- `Bottom` = `2` — The label is located on the bottom of the view.

## View.ViewAttributes (class)

The ViewAttributes class contains the attributes for the view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/37966d35-61b2-f198-c207-c709508a943a)

### Constructors
- `View.ViewAttributes(...)` — Creates a new view attributes instance that loads standard attributes.
- `View.ViewAttributes(...)` — Creates a new view attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/88f7ad53-6e09-7bf7-e523-1b30b1fc152b)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/9e31a4e7-f908-e01d-d4b0-99f0582dc4f6)

### Properties
- `DatumLevel` (object, get/set) — The specified datum level. If ViewPlaneDatumPointForElevations is set to true when DatumLevel is set, the value of ViewPlaneDatumPointForElevations will change to false.
- `FixedViewPlacing` (object, get/set) — Defines whether the view placing is fixed (true) or free (false).
- `LabelPositionHorizontal` (object, get/set) — The horizontal position of the view label.
- `LabelPositionVertical` (object, get/set) — The vertical position of the view label.
- `LocationBy` (object, get/set) — Gets or sets the location by attribute. Value is ViewAttributes.LocationByModelOrigin, ViewAttributes.LocationByProjectBasePoint or guid of a Basepoint.
- `MarkSymbolAttributes` (object, get/set) — The view mark symbol attributes.
- `MarkSymbolColor` (object, get/set) — The color of the mark symbol (see DrawingColors for options).
- `PartialProfileLength` (object, get/set) — Gets or sets the partial profile length.
- `PartialProfileOffset` (object, get/set) — Gets or sets the partial profile offset
- `PourView` (object, get/set) — Defines whether the view is pour view or not.
- `ReflectedView` (object, get/set) — Defines whether the view is reflected or not.
- `Scale` (object, get/set) — The view scale.
- `Shortening` (object, get/set) — Gets or sets the shortening attributes for the view.
- `ShowPartOpeningsOrRecessSymbol` (object, get/set) — Defines whether to show the openings/recess symbol or not.
- `TagsAttributes` (object, get/set) — The attributes for view mark tags.
- `TrueMarkSymbolColor` (object, get/set) — The true color (rgb enabled) of the mark symbol.
- `UndeformedView` (object, get/set) — Defines whether the view is undeformed or not.
- `UnfoldedView` (object, get/set) — Defines whether the view is unfolded or not.
- `ViewExtensionForNeighbourParts` (object, get/set) — The view extension for neighbour parts.
- `ViewPlaneDatumPointForElevations` (object, get/set) — Defines whether the datum point for elevations is view plane based (true) or specified (false). When set to false, the value of DatumLevel is used. When set to true, DatumLevel will be set to 0.0.

## View.ViewMarkSymbolAttributes (class)

The ViewMarkSymbolAttributes class contains symbol attributes for view mark tags.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2db7869f-e77d-a9a1-c2fa-f1f737208f33)

### Constructors
- `View.ViewMarkSymbolAttributes(...)` — Creates a new mark symbol attributes instance.
- `View.ViewMarkSymbolAttributes(...)` — Creates a new mark symbol attributes instance with the given size, shape, line length type and line length.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/55a27e06-da69-f6ab-6337-1b71216f247d)

### Properties
- `LineLength` (object, get/set) — The label line length in the case of custom length.
- `LineLengthType` (object, get/set) — The label line length type.
- `Shape` (object, get/set) — The shape of the mark symbol.
- `Size` (object, get/set) — The size of the mark symbol.

## View.ViewMarkTagAttributes (class)

The ViewMarkTagAttributes class contains attributes for view mark tags.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d14a53c7-82c7-69b3-101e-81403379a9df)

### Constructors
- `View.ViewMarkTagAttributes(...)` — Creates a new mark tag attributes instance.
- `View.ViewMarkTagAttributes(...)` — Creates a new mark tag attributes instance with the given offset, location, alignment and content.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/aa40044e-5b47-ea87-43b9-db5751e4c0aa)

### Properties
- `Location` (object, get/set) — The location of the mark tag.
- `Offset` (object, get/set) — The offset of the mark tag.
- `TagAlignment` (object, get/set) — The tag text alignment.
- `TagContent` (object, get/set) — The tag content.

## View.ViewMarkTagsAttributes (class)

The ViewMarkTagsAttributes class contains attributes for view mark tags.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9066f94e-5bf0-17fa-79ab-5e44c3c90ba7)

### Constructors
- `View.ViewMarkTagsAttributes(...)` — Creates a new mark tags attributes instance.
- `View.ViewMarkTagsAttributes(...)` — Creates a new mark tags attributes instance.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/5d087974-45d9-02b4-ea01-e21fa5153ffd)

### Properties
- `TagA1` (object, get/set) — The tag A1 of a mark.
- `TagA2` (object, get/set) — The tag A2 of a mark.
- `TagA3` (object, get/set) — The tag A3 of a mark.
- `TagA4` (object, get/set) — The tag A4 of a mark.
- `TagA5` (object, get/set) — The tag A5 of a mark.

## View.ViewShorteningAttributes (class)

The ViewShorteningAttributes class sets the shortening attributes for the view. If CutParts is set to true, the other values are applied. If it is false, none of the other values matter.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/17b1c7e5-f3c2-f128-ba48-d25e5c1618b4)

### Constructors
- `View.ViewShorteningAttributes(...)` — Creates a new default view shortening attributes instance.
- `View.ViewShorteningAttributes(...)` — Creates a new view shortening attributes instance with the given parameters.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/d5db5013-c321-f2b7-34fe-17024157b078)

### Properties
- `CutParts` (object, get/set) — Defines whether cut parts are enabled (true) or disabled (false).
- `CutPartType` (object, get/set) — Defines cut part type. Not used if CutParts is false.
- `CutSkewParts` (object, get/set) — Defines whether cut skewed parts are enabled (true) or disabled (false).
- `MinimumLength` (object, get/set) — The minimum cut part length.
- `Offset` (object, get/set) — The space between the cut parts.

## View.ViewTypes (enum)

The possible types of views.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e2c4286c-cf79-8fb8-cad3-db5fa4ea7030)

### Values
- `UnknownViewType` = `0` — The view type is not known.
- `FrontView` = `1` — Front side view of the part.
- `TopView` = `2` — Top side view of the part.
- `BackView` = `3` — Back side view of the part.
- `BottomView` = `4` — Bottom side view of the part.
- `EndView` = `5` — End side view of the part.
- `SectionView` = `6` — Section view of the part.
- `ModelView` = `7` — Model view of the part.
- `DetailView` = `8` — Detail view of the part.
- `_3DView` = `9` — 3d view of the part.

## ViewBase (class)

The ViewBase class is a base class for both the ContainerView class and the View class.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/474d3089-26da-d62c-0797-bcae0f714220)

### Methods
#### `Delete(...)`

Deletes the view and its children objects from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/c7b5ef15-0c29-6a2a-ef92-99fb8e0ed0b3)

#### `GetAllObjects(.Type.)(...)`

Gets all the objects and their children objects in the view that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/d9638840-5e65-49e6-cadb-2e2a8b5423fd)

#### `GetAllObjects(Type)(...)`

Gets all the objects and their children objects in the view that are of a certain type.

[Docs](https://developer.tekla.com/topic/en/18/47/d8756000-215b-c1e0-49c7-d36dc245bec8)

#### `GetAllObjects.(...)`

Gets all the objects and their children objects in the view.

[Docs](https://developer.tekla.com/topic/en/18/47/6c8f0774-654d-498b-93d5-904797569fd5)

#### `GetAxisAlignedBoundingBox(...)`

The view size in paper coordinates.

[Docs](https://developer.tekla.com/topic/en/18/47/c51d1265-7cc9-ca4c-86b0-d76fe62d8be8)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetModelObjects(...)`

Gets the drawing model objects based on given model object identifier. If used from sheet model objects from all views are returned.

[Docs](https://developer.tekla.com/topic/en/18/47/c59be515-bde2-42ef-2e58-fa8a4e0173ba)

#### `GetObjects(.Type.)(...)`

Gets the objects in the view that are of certain types. Does not return child objects (single dimensions, grid lines, views).

[Docs](https://developer.tekla.com/topic/en/18/47/e64fef01-3761-6798-cd3f-d5e5d006a37f)

#### `GetObjects.(...)`

Gets the objects in the view. Does not return child objects (single dimensions, grid lines, views).

[Docs](https://developer.tekla.com/topic/en/18/47/bd691388-4a5f-d33f-3364-d516c037cb21)

#### `GetOriginalDrawing(...)`

Gets the drawing on which the view was originally created. @see DrawingObject.GetDrawing() for retrieving the current drawing the object resides on.

[Docs](https://developer.tekla.com/topic/en/18/47/b34f08ba-6799-5511-dd98-8654ffbc1ebd)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

Inserts the view to the database.

[Docs](https://developer.tekla.com/topic/en/18/47/c5735e20-f3b0-3147-4879-3e1e42c30d82)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e8224dea-c111-7c34-9e53-9ab222787de0)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the view in the database.

[Docs](https://developer.tekla.com/topic/en/18/47/883ce75d-2048-41ab-05ba-96ffe20220aa)

#### `Select(...)`

Selects the view from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/4630a41b-4e54-0f11-a21d-eed889061760)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — Gets or sets the attributes of the current object. Only attributes of the same type as the object are allowed.
- `ExtremaCenter` (object, get/set) — Gets the view extrema center.
- `FrameOrigin` (object, get/set) — The vector from the view origin to the frame origin.
- `Height` (object, get/set) — The height of the view frame in paper coordinates.
- `IsSheet` (object, get/set) — Gets a value indicating whether the container view is the drawing's sheet.
- `Origin` (object, get/set) — The view origin coordinates in the sheet.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).
- `Width` (object, get/set) — The width of the view frame in paper coordinates.

## ViewMarkBasicSymbolAttributes (class)

The ViewMarkBasicSymbolAttributes class contains basic attributes for mark symbols.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b01382aa-3575-dbde-21b4-23bf6c496c8b)

### Constructors
- `ViewMarkBasicSymbolAttributes(...)` — Creates a new mark symbol attributes instance.
- `ViewMarkBasicSymbolAttributes(...)` — Creates a new mark symbol attributes instance with the given size and shape.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/c5232b59-b0cc-d2d6-6ba6-26c04856847f)

### Properties
- `Shape` (object, get/set) — The shape of the mark symbol.
- `Size` (object, get/set) — The size of the mark symbol.

## ViewMarkBasicTagAttributes (class)

The ViewMarkBasicTagAttributes class contains basic attributes for view mark tags.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/115cdf5c-c4ca-707c-761d-a174175a4682)

### Constructors
- `ViewMarkBasicTagAttributes(...)` — Creates a new mark tag attributes instance.
- `ViewMarkBasicTagAttributes(...)` — Creates a new mark tag attributes instance with the given offset, location and content.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/f39f9dcb-d8ad-b6c2-0199-cabb488c493f)

### Properties
- `Location` (object, get/set) — The location of the mark tag.
- `Offset` (object, get/set) — The offset of the mark tag.
- `TagContent` (object, get/set) — The tag content.

## Weld (class)

The Weld class represents weldings in drawing views that refer to actual welds in the model database. It has properties and methods to control the graphical representation of welding lines. Instances of Weld class cannot be inserted with code. These appear in drawings when views are created/updated. To modify its graphical representation, attributes of Weld class can use .welo setting files.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8e62e9d2-d28d-f0e5-c421-9e0a8cd05a97)

### Methods
#### `Delete(...)`

Deletes the drawing weld from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/3b14113a-d783-ae0f-779d-9ce582c93f33)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED. Welding lines that refer to welds in the model database appear in views when drawing views are created/updated.

[Docs](https://developer.tekla.com/topic/en/18/47/4fa6cc65-61e2-4add-5bec-bf24a57d603d)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/9921493c-247e-6bca-1015-9d408f795f4c)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Calls the system to update/modify the weld. Class can use .welo setting files to modify graphical representation of the welding lines.

[Docs](https://developer.tekla.com/topic/en/18/47/f2ae6b8f-42bf-b1ae-13aa-72cf76351cbc)

#### `Select(...)`

Selects the drawing weld from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/e0fd9cc1-b347-c052-9d47-413f3851b66b)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the weld. For more information see Weld.WeldAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `ModelIdentifier` (object, get/set) — The database identifier of the model object in the model database. By using this identifier, it is possible to select the actual object in the Tekla Structures model.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## Weld.Representation (enum)

The weld representations.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ddf07234-73d3-b433-f405-508278135d49)

### Values
- `Outline` = `1` — The outline solid representation.
- `Path` = `2` — The path representation.

## Weld.WeldAttributes (class)

The WeldAttributes class is the attributes class for the weld.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/624aa0ff-13ca-e29c-8b7e-91338f265e8a)

### Constructors
- `Weld.WeldAttributes(...)` — Creates a new default weld attributes instance that loads standard attributes.
- `Weld.WeldAttributes(...)` — Creates a new weld attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/324f5eed-802f-1847-a2e3-e205fae092d3)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/610dd09a-8447-6f6d-22bf-45c35819209f)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the weld
- `DrawHiddenLines` (object, get/set) — True if hidden lines are drawn.
- `DrawOwnHiddenLines` (object, get/set) — True if own hidden lines are drawn.
- `HiddenLines` (object, get/set) — The line type of hidden lines.
- `Representation` (object, get/set) — The representation of the weld.
- `VisibleLines` (object, get/set) — The line type of visible lines.

## WeldMark (class)

The WeldMark class defines a drawing object that illustrates a weld mark in a specific view. WeldMark class refers to welding marks that appear when creating/updating drawing views. In this case, ModelIdentifier has a value. WeldMark class also refers to weld marks created with drawing editor UI. In this case, ModelIdentifier value is zero. Graphical representation of welding marks (line and text properties) can be modified using .wel setting files. The welding mark settting files (.wls) cannot be used to modify instances of WeldMark class.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/dae803f9-fb07-c6d7-ad0f-df44a120a6e4)

### Methods
#### `Delete(...)`

Deletes the weld mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/0aa79b98-4590-4547-2c66-949037ffba8e)

#### `GetAxisAlignedBoundingBox(...)`

Returns the axis aligned bounding box of the weld mark (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/8bb50ca2-6dea-a3fb-5118-79cf400644be)

#### `GetDoubleUserProperties(Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b25f0e6b-ab5c-154a-a45c-4f5868a20599)

#### `GetDoubleUserProperties(List.String., Dictionary.String, Double..)(...)`

Retrieves all double user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/7f327fc5-14b5-f534-da39-b68d3f87124a)

#### `GetDrawing(...)`

Gets the drawing where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/4e9beef3-2188-ab45-9f07-dbde351915bf)

#### `GetIntegerUserProperties(Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/20a7d099-ebee-b016-ffc7-0181a1219405)

#### `GetIntegerUserProperties(List.String., Dictionary.String, Int32..)(...)`

Retrieves all integer user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/b485bd81-5ccf-68e4-0453-caea6b1d0dde)

#### `GetObjectAlignedBoundingBox(...)`

Returns the object aligned bounding box of the weld mark (rectangle format).

[Docs](https://developer.tekla.com/topic/en/18/47/ddbc4ec3-4fff-620f-fd09-e743318185b6)

#### `GetObjects(.Type.)(...)`

Gets the children objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/4f27ba0a-4880-f09e-d734-82fea5b72200)

#### `GetObjects.(...)`

Gets the children objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/41fc1a14-869d-313a-134b-6abba011a1e9)

#### `GetRelatedObjects(.Type.)(...)`

Gets the related objects of the current object that are of certain types.

[Docs](https://developer.tekla.com/topic/en/18/47/01d5e137-fa4b-92d7-328b-69c974dc7415)

#### `GetRelatedObjects.(...)`

Gets the related objects of the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/50820387-cee1-637a-318e-fb3e06780149)

#### `GetStringUserProperties(Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/a0039213-2e79-912f-5bdf-5e6dd417c092)

#### `GetStringUserProperties(List.String., Dictionary.String, String..)(...)`

Retrieves all string user properties for the object with the given list of names.

[Docs](https://developer.tekla.com/topic/en/18/47/474a80dc-539a-2734-4b5c-1ea52f86e750)

#### `GetUserProperty(String, Double.)(...)`

Gets a double property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/203a39af-db6b-e317-5de3-f0301c1642f2)

#### `GetUserProperty(String, Int32.)(...)`

Gets an integer property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/f773aabe-fe4e-dba3-bf33-f6f927441a85)

#### `GetUserProperty(String, String.)(...)`

Gets a string property from the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5335a30d-ac20-6231-5b02-22e14c6980a7)

#### `GetView(...)`

Gets the view where the drawing object is.

[Docs](https://developer.tekla.com/topic/en/18/47/ed3c08d7-c75f-2e18-ee97-fe2e096f29b5)

#### `Insert(...)`

NOT YET IMPLEMENTED. Welding marks connected with welds in the model database appear when drawing views are created/updated. Weld marks created with drawing editor UI cannot be inserted with API code.

[Docs](https://developer.tekla.com/topic/en/18/47/133d93c8-9325-f294-6a36-a307848ff516)

#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/620f4174-d0c0-3b3a-4d5e-78da6e205edd)

#### `IsSameDatabaseObject(...)`

Returns true if the current object and the given object are referencing the same database object.

[Docs](https://developer.tekla.com/topic/en/18/47/675fe6a8-2f1f-6e66-07f4-768bc3a1c74b)

#### `Modify(...)`

Modifies the weld mark in the current drawing database. Attributes can be modified with .wel setting files.

[Docs](https://developer.tekla.com/topic/en/18/47/bdb342be-7a5a-1962-3079-b741c7e90428)

#### `MoveObjectRelative(...)`

Moves the object using the move vector.

[Docs](https://developer.tekla.com/topic/en/18/47/041e9576-0600-15ac-8339-bc2cdb274ba4)

#### `Select(...)`

Selects the weld mark from the current drawing database.

[Docs](https://developer.tekla.com/topic/en/18/47/052f7f53-8715-a861-fb57-dbeaa8688eca)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/995715f7-c86c-86d0-359f-76dc4d14197a)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/88a5c4aa-fb12-e341-ee9e-fcd3284fab68)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the drawing object. The object has to be in a drawing before the method can be used.

[Docs](https://developer.tekla.com/topic/en/18/47/633cfaf7-22e7-1bb1-6596-053d78749c52)

#### `ToString(...)`

Changes the string presentation of the drawing object.

[Docs](https://developer.tekla.com/topic/en/18/47/23e70841-4b0d-9dc2-7a9b-f3b6d7eed19f)

### Properties
- `Attributes` (object, get/set) — The attributes of the weld mark. For more information see WeldMark.WeldMarkAttributes.
- `Hideable` (object, get/set) — Accesses the information if this object is hidden or not or if it should be.
- `InsertionPoint` (object, get/set) — The insertion point of the mark.
- `ModelIdentifier` (object, get/set) — When instance refers to welding mark related to a weld in model database, ModelIdentifier has a value. When instance refers to weld mark created with drawing editor UI, value is zero.
- `QueryReturnValue` (object, get/set) — Status information about the latest database operation (select, insert, modify, delete).

## WeldMark.SeamVisibilityAttributes (class)

The SeamVisibilityAttributes class is the attributes class for the weld seam visibility.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/853a2750-92ba-db30-78e3-543bbe6914d1)

### Constructors
- `WeldMark.SeamVisibilityAttributes(...)` — Creates a new weld seam visibility attributes instance.
- `WeldMark.SeamVisibilityAttributes(...)` — Creates a new weld seam visibility attributes instance with the given attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/e1d49834-4506-32f5-7fe4-0dab996df6af)

### Properties
- `ShowAngle` (object, get/set) — Gets or sets a value indicating whether the seam angle is visible.
- `ShowContour` (object, get/set) — Gets or sets a value indicating whether the seam contour is visible.
- `ShowEffectiveThroat` (object, get/set) — Gets or sets a value indicating whether the seam effective throat is visible.
- `ShowFinish` (object, get/set) — Gets or sets a value indicating whether the seam finish is visible.
- `ShowLength` (object, get/set) — Gets or sets a value indicating whether the seam length is visible.
- `ShowPitch` (object, get/set) — Gets or sets a value indicating whether the seam pitch is visible.
- `ShowPrefix` (object, get/set) — Gets or sets a value indicating whether the seam prefix is visible.
- `ShowRootOpening` (object, get/set) — Gets or sets a value indicating whether the seam root opening is visible.
- `ShowSize` (object, get/set) — Gets or sets a value indicating whether the seam size is visible.
- `ShowType` (object, get/set) — Gets or sets a value indicating whether the seam type is visible.

## WeldMark.WeldMarkAttributes (class)

The WeldMarkAttributes class is the attributes class for the weld mark attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/98722d9e-764f-601c-d375-077400a3052c)

### Constructors
- `WeldMark.WeldMarkAttributes(...)` — Creates a new default weld mark attributes instance that loads standard attributes.
- `WeldMark.WeldMarkAttributes(...)` — Creates a new weld mark attributes instance that loads the specified attributes.

### Methods
#### `IsEqual(...)`

Compares the current object with an object of the same type.

[Docs](https://developer.tekla.com/topic/en/18/47/15f7400e-a8c0-6cef-0991-aa99d4daa923)

#### `LoadAttributes(...)`

Loads the attributes from the attribute file.

[Docs](https://developer.tekla.com/topic/en/18/47/9f018710-63bd-27fd-fa0f-8e8e34de85ab)

### Properties
- `CustomPresentation` (object, get/set) — Custom presentation GUID to be applied to the object type
- `Font` (object, get/set) — The weld mark font settings.
- `SeamAbove` (object, get/set) — The visibility of the seam above parameters.
- `SeamBelow` (object, get/set) — The visibility of the seam below parameters.
- `ShowEdgeAround` (object, get/set) — Gets or sets visibility of the weld mark edge/around property.
- `ShowReferenceText` (object, get/set) — Gets or sets visibility of the weld mark reference text.
- `ShowWeldNumber` (object, get/set) — The visibility of the weld number.
- `ShowWorkshopSite` (object, get/set) — Gets or sets visibility of the weld mark workshop/site property.
- `TransparentBackground` (object, get/set) — Controls the text background transparency (transparent/opaque).

