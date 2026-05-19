---
name: allplan-nemall_python_geometry
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_Geometry namespace — 142 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AngleList, ApproximationSettings, Angle, Arc2D, Arc3D, Arc3DList, Arc2DList, Axis2D, and 134 more types.
---

# NemAll_Python_Geometry

Auto-generated from vendor docs for allplan 2024.0. 142 types in this namespace.

## Angle (class)

Representation class for angle in [rad].

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/)

### Constructors
- `Angle() | Angle(angle) | Angle(angle)` — Initialize

### Methods
#### `DegToRad(angleDeg)`

Convert angle from deg to rad Can be used for initialization of Angle class with deg angle. angle = Angle(Angle.DegToRad(45)); // angle will be 0.75[rad] (approx.)

**Remarks:** Convert angle from deg to rad Can be used for initialization of Angle class with deg angle. angle = Angle(Angle.DegToRad(45)); // angle will be 0.75[rad] (approx.)

**Parameters:**
- `angleDeg` (float) — Angle in deg

**Returns:** `float` — Angle in rad

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.DegToRad)

#### `FromDeg(angleDeg)`



**Parameters:**
- `angleDeg` (float) — Angle in degree

**Returns:** `Angle` — Angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.FromDeg)

#### `Get()`

Get angle as radian value.

**Remarks:** Get angle as radian value.

**Returns:** `float` — double as radian value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.Get)

#### `GetDeg()`

Get angle as degree value.

**Remarks:** Get angle as degree value.

**Returns:** `float` — double as degree value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.GetDeg)

#### `Normalize2Pi()`

Normalize the angle to a range of <0, 2PI>. This method is checked and set Angle to 0 while angle is out of range <-1e8, 1e8>. The algorithm isn't stable for angle out of this range.

**Remarks:** Normalize the angle to a range of <0, 2PI>. This method is checked and set Angle to 0 while angle is out of range <-1e8, 1e8>. The algorithm isn't stable for angle out of this range.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.Normalize2Pi)

#### `NormalizePi()`

Normalize the angle to a range of <-PI, PI>. This method is checked and set Angle to 0 while angle is out of range <-1e8, 1e8>. The algorithm isn't stable for angle out of this range.

**Remarks:** Normalize the angle to a range of <-PI, PI>. This method is checked and set Angle to 0 while angle is out of range <-1e8, 1e8>. The algorithm isn't stable for angle out of this range.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.NormalizePi)

#### `RadToDeg(angleRad)`

Convert angle from rad to deg

**Remarks:** Convert angle from rad to deg

**Parameters:**
- `angleRad` (float) — Angle in rad

**Returns:** `float` — Angle in rad

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.RadToDeg)

#### `RadToGrad(angleRad)`

Convert angle from rad to grad

**Remarks:** Convert angle from rad to grad

**Parameters:**
- `angleRad` (float) — Angle in rad

**Returns:** `float` — Angle in grad

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.RadToGrad)

#### `Set(angle) | Set(angle)`

Set angle as radian value.

**Remarks:** Set angle as radian value.

**Parameters:**
- `angle` (float) — angle which will be set.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.Set)

#### `SetDeg(angleDeg)`

Set angle as degree value.

**Remarks:** Set angle as degree value.

**Parameters:**
- `angleDeg` (float) — angle as degree value which will be set.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.SetDeg)

#### `__add__(angle) | __add__(angle)`

Addition operator

**Remarks:** Addition operator

**Parameters:**
- `angle` (Angle) — Angle which will be added

**Returns:** `Angle` — New angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.__add__)

#### `__eq__(angle)`

Comparison of angles. Be careful, this method work without tolerance!

**Remarks:** Comparison of angles. Be careful, this method work without tolerance!

**Parameters:**
- `angle` (Angle) — angle to be compared.

**Returns:** `bool` — True when angles are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.__eq__)

#### `__float__()`

Type conversion operator.

**Remarks:** Type conversion operator.

**Returns:** `float` — Angle as double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.__float__)

#### `__iadd__(angle)`

Addition assignment operator.

**Remarks:** Addition assignment operator.

**Parameters:**
- `angle` (Angle) — Angle which will be added.

**Returns:** `Angle` — Reference to Angle.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.__iadd__)

#### `__isub__(angle)`

Addition assignment operator.

**Remarks:** Addition assignment operator.

**Parameters:**
- `angle` (Angle) — Angle which will be added.

**Returns:** `Angle` — Reference to Angle.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.__isub__)

#### `__ne__(angle)`

Comparison of angles. Be careful, this method work without tolerance!

**Remarks:** Comparison of angles. Be careful, this method work without tolerance!

**Parameters:**
- `angle` (Angle) — angle to be compared.

**Returns:** `bool` — True when angles are not equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/#NemAll_Python_Geometry.Angle.__repr__)

### Properties
- `Deg` (float, get/set) — Get angle as degree value.
- `Rad` (float, get/set) — Get angle as radian value.

## AngleList (class)

List for Angle objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/)

### Constructors
- `AngleList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Angle) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Angle) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (AngleList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Angle` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Angle) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Angle) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (AngleList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AngleList/#NemAll_Python_Geometry.AngleList.extend)

## ApproximationSettings (class)

Class holding approximation options.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/)

### Constructors
- `ApproximationSettings(oType=eApproximationSettingsType.ASET_SEGMENTATION, value=30.0) | ApproximationSettings(options)` — Default constructor. Other members will be set to 0. Types can be found in GeometryEnums.h

### Methods
#### `GetDensity()`

Get Density value

**Remarks:** Get Density value

**Returns:** `float` — density value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.GetDensity)

#### `GetMaxAngle()`

Get Maximal Angle value

**Remarks:** Get Maximal Angle value

**Returns:** `Angle` — maximal angle value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.GetMaxAngle)

#### `GetMaxDistance()`

Get MaxDistance value

**Remarks:** Get MaxDistance value

**Returns:** `float` — const Reference to MaxDistance value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.GetMaxDistance)

#### `GetMaxLength()`

Get MaxLength value

**Remarks:** Get MaxLength value

**Returns:** `float` — const Reference to MaxLength value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.GetMaxLength)

#### `GetMinLength()`

Get Minimal Length value

**Remarks:** Get Minimal Length value

**Returns:** `float` — minimal length value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.GetMinLength)

#### `GetSegmentation()`

Get Segmentation value

**Remarks:** Get Segmentation value

**Returns:** `int` — const Reference to Segmentation value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.GetSegmentation)

#### `GetType()`

Get Type of the settings

**Remarks:** Get Type of the settings

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.GetType)

#### `IsBRepTesselation()`

Check whether the type of the settings is ASET_BREP_TESSELATION

**Remarks:** Check whether the type of the settings is ASET_BREP_TESSELATION

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.IsBRepTesselation)

#### `IsMaxDistance()`

Check whether the type of the settings is ASET_MAX_DISTANCE

**Remarks:** Check whether the type of the settings is ASET_MAX_DISTANCE

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.IsMaxDistance)

#### `IsMaxLength()`

Check whether the type of the settings is ASET_MAX_LENGTH

**Remarks:** Check whether the type of the settings is ASET_MAX_LENGTH

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.IsMaxLength)

#### `IsSegmentation()`

Check whether the type of the settings is ASET_SEGMENTATION

**Remarks:** Check whether the type of the settings is ASET_SEGMENTATION

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.IsSegmentation)

#### `SetBRepTesselation(density, maxAngle, minLength, maxLength)`

Set all settings for BRep3D tesselation.

**Remarks:** Set all settings for BRep3D tesselation.

**Parameters:**
- `density` (float) — new density value.
- `maxAngle` (Angle) — new maximal edge length value.
- `minLength` (float) — new minimal edge length value.
- `maxLength` (float) — new maximal angle value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.SetBRepTesselation)

#### `SetMaxDistance(maxDistance)`

Set MaxDistance value.

**Remarks:** Set MaxDistance value.

**Parameters:**
- `maxDistance` (float) — new MaxDistance value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.SetMaxDistance)

#### `SetMaxLength(maxLength)`

Set MaxLength value.

**Remarks:** Set MaxLength value.

**Parameters:**
- `maxLength` (float) — new MaxLength value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.SetMaxLength)

#### `SetSegmentation(segmentation)`

Set Segmentation.

**Remarks:** Set Segmentation.

**Parameters:**
- `segmentation` (int) — new Segmentation value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.SetSegmentation)

#### `__eq__(right)`

Operator ==

**Remarks:** Operator ==

**Parameters:**
- `right` (ApproximationSettings) — settings to compare

**Returns:** `bool` — true if settings are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.__eq__)

#### `__ne__(right)`

Operator !=

**Remarks:** Operator !=

**Parameters:**
- `right` (ApproximationSettings) — settings to compare

**Returns:** `bool` — true if settings are NOT equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ApproximationSettings/#NemAll_Python_Geometry.ApproximationSettings.__ne__)

## Arc2D (class)

Representation class for 2D arc.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/)

### Constructors
- `Arc2D() | Arc2D(center, minor, major, axisangle, startangle, endangle, counterClockwise=True) | Arc2D(center, radius, counterClockwise=True) | Arc2D(element)` — Initialize

### Methods
#### `Close()`

Close arc End angle will be adjusted to close arc.

**Remarks:** Close arc End angle will be adjusted to close arc.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.Close)

#### `EqualRef(arc)`

Test for equal reference points

**Remarks:** Test for equal reference points

**Parameters:**
- `arc` (Arc2D) — Arc which will be compared

**Returns:** `bool` — result as bool

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.EqualRef)

#### `GetAxisAngle()`

Get the axis angle Returns the the angle of the major axis

**Remarks:** Get the axis angle Returns the the angle of the major axis

**Returns:** `Angle` — axis angle.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetAxisAngle)

#### `GetCenter()`

Get center point

**Remarks:** Get center point

**Returns:** `Point2D` — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetCenter)

#### `GetCenterRel()`

Get center point in relative coordinate system.

**Remarks:** Get center point in relative coordinate system.

**Returns:** `Point2D` — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetCenterRel)

#### `GetDeltaAngle()`

Get difference between EndAngle and StartAngle

**Remarks:** Get difference between EndAngle and StartAngle

**Returns:** `Angle` — delta angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetDeltaAngle)

#### `GetEndAngle()`

Get end angle

**Remarks:** Get end angle

**Returns:** `Angle` — end angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetEndAngle)

#### `GetEndPoint()`

Get the end point in world coordinate system.

**Remarks:** Get the end point in world coordinate system.

**Returns:** `Point2D` — point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetEndPoint)

#### `GetEndRelPoint()`

Get the end point in relative coordinate system

**Remarks:** Get the end point in relative coordinate system

**Returns:** `Point2D` — constant point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetEndRelPoint)

#### `GetEndTangent()`

Get tangent vector at the end point of Arc

**Remarks:** Get tangent vector at the end point of Arc

**Returns:** `Vector2D` — Tangent vector (unit vector)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetEndTangent)

#### `GetMajorRadius()`

Get major radius

**Remarks:** Get major radius

**Returns:** `float` — major radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetMajorRadius)

#### `GetMinorRadius()`

Get minor radius

**Remarks:** Get minor radius

**Returns:** `float` — minor radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetMinorRadius)

#### `GetPoint(angle)`

Get point on Arc in world coordinate system Calculates the point at central angle in parameter angle on the Arc

**Remarks:** Get point on Arc in world coordinate system Calculates the point at central angle in parameter angle on the Arc

**Parameters:**
- `angle` (Angle) — central angle of the point

**Returns:** `Point2D` — point on Arc

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetPoint)

#### `GetRefPoint()`

Get reference point

**Remarks:** Get reference point

**Returns:** `Point2D` — reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetRefPoint)

#### `GetStartAngle()`

Get start angle

**Remarks:** Get start angle

**Returns:** `Angle` — start angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetStartAngle)

#### `GetStartPoint()`

Get the start point in world coordinate system.

**Remarks:** Get the start point in world coordinate system.

**Returns:** `Point2D` — point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetStartPoint)

#### `GetStartRelPoint()`

Get the start point in relative coordinate system

**Remarks:** Get the start point in relative coordinate system

**Returns:** `Point2D` — constant point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetStartRelPoint)

#### `GetStartTangent()`

Get tangent vector at the start point of Arc

**Remarks:** Get tangent vector at the start point of Arc

**Returns:** `Vector2D` — Tangent vector (unit vector)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.GetStartTangent)

#### `IsAngleOnArc(angle)`

Checks if the given angle lies on the arc

**Remarks:** Checks if the given angle lies on the arc

**Parameters:**
- `angle` (Angle) — angle to test

**Returns:** `bool` — Angle on arc true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.IsAngleOnArc)

#### `IsCircle()`

Test if ellipse is circle Tests if major radius equals minor radius

**Remarks:** Test if ellipse is circle Tests if major radius equals minor radius

**Returns:** `bool` — if ellipse is circle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.IsCircle)

#### `IsClosed()`

Check if arc is closed ( Full circle/ellipse )

**Remarks:** Check if arc is closed ( Full circle/ellipse )

**Returns:** `bool` — closed curve true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.IsClosed)

#### `IsCounterClockwise()`

Returns winding direction counterclockwise true/false

**Remarks:** Returns winding direction counterclockwise true/false

**Returns:** `bool` — Is counterclockwise true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.IsCounterClockwise)

#### `Reverse()`

Reverse of current arc Method reverse Arc, start with end angle and orientation.

**Remarks:** Reverse of current arc Method reverse Arc, start with end angle and orientation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.Reverse)

#### `SetAxisAngle(angle)`

Set the axis angle Set the angle of the major axis

**Remarks:** Set the axis angle Set the angle of the major axis

**Parameters:**
- `angle` (Angle) — axis angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetAxisAngle)

#### `SetCenter(center)`

Set center point

**Remarks:** Set center point

**Parameters:**
- `center` (Point2D) — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetCenter)

#### `SetCenterRel(center)`

Set center point in local coordinate system

**Remarks:** Set center point in local coordinate system

**Parameters:**
- `center` (Point2D) — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetCenterRel)

#### `SetCounterClockwise(ccw)`

Set the winding direction of the arc

**Remarks:** Set the winding direction of the arc

**Parameters:**
- `ccw` (bool) — winding counterclockwise true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetCounterClockwise)

#### `SetEndAngle(angle)`

Set end angle Set the end angle

**Remarks:** Set end angle Set the end angle

**Parameters:**
- `angle` (Angle) — end angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetEndAngle)

#### `SetEndPoint(endpoint)`

Set the end point in world coordinate system

**Remarks:** Set the end point in world coordinate system

**Parameters:**
- `endpoint` (Point2D) — constant 2D point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetEndPoint)

#### `SetMajorRadius(radius)`

Set major radius

**Remarks:** Set major radius

**Parameters:**
- `radius` (float) — major radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetMajorRadius)

#### `SetMinorRadius(radius)`

Set minor radius

**Remarks:** Set minor radius

**Parameters:**
- `radius` (float) — minor radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetMinorRadius)

#### `SetRefPoint(refPoint)`

Set reference point

**Remarks:** Set reference point

**Parameters:**
- `refPoint` (Point2D) — reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetRefPoint)

#### `SetStartAngle(angle)`

Set start angle Set the start angle

**Remarks:** Set start angle Set the start angle

**Parameters:**
- `angle` (Angle) — start angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetStartAngle)

#### `SetStartPoint(startpoint)`

Set the start point in world coordinate system

**Remarks:** Set the start point in world coordinate system

**Parameters:**
- `startpoint` (Point2D) — constant 2D point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.SetStartPoint)

#### `Supplement()`

Convert to supplementary arc Orientation of arc will not be changed.

**Remarks:** Convert to supplementary arc Orientation of arc will not be changed.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.Supplement)

#### `__eq__(arc)`

Comparison of arcs. Be careful, this method work without tolerance!

**Remarks:** Comparison of arcs. Be careful, this method work without tolerance!

**Parameters:**
- `arc` (Arc2D) — arc to be compared.

**Returns:** `bool` — True when arcs are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.__eq__)

#### `__mul__(matrix)`

Matrix transformation.

**Remarks:** Matrix transformation.

**Parameters:**
- `matrix` (Matrix2D) — transformation matrix.

**Returns:** `Arc2D` — transformed Arc2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2D/#NemAll_Python_Geometry.Arc2D.__repr__)

### Properties
- `AxisAngle` (Angle, get/set) — Get the axis angle Returns the the angle of the major axis
- `Center` (Point2D, get/set) — Get center point
- `CenterRel` (Point2D, get/set) — Get center point in relative coordinate system.
- `CounterClockwise` (bool, get/set) — Returns winding direction counterclockwise true/false
- `DeltaAngle` (Angle, get) — Get difference between EndAngle and StartAngle
- `EndAngle` (Angle, get/set) — Get end angle
- `EndPoint` (Point2D, get/set) — Get the end point in world coordinate system.
- `EndRelPoint` (Point2D, get) — Get the end point in relative coordinate system
- `MajorRadius` (float, get/set) — Get major radius
- `MinorRadius` (float, get/set) — Get minor radius
- `RefPoint` (Point2D, get/set) — Get reference point
- `StartAngle` (Angle, get/set) — Get start angle
- `StartPoint` (Point2D, get/set) — Get the start point in world coordinate system.
- `StartRelPoint` (Point2D, get) — Get the start point in relative coordinate system

## Arc2DList (class)

List for Arc2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/)

### Constructors
- `Arc2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Arc2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Arc2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (Arc2DList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Arc2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Arc2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Arc2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Arc2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc2DList/#NemAll_Python_Geometry.Arc2DList.extend)

## Arc3D (class)

Representation class for 3D arc.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/)

### Constructors
- `Arc3D() | Arc3D(center, xDir, normVector, minor, major, startAngle, deltaAngle) | Arc3D(center, xDir, normVector, minor, major, startAngle, endAngle, counterClockwise) | Arc3D(center, minor, major, startAngle, deltaAngle) | Arc3D(center, minor, major, startAngle, endAngle, counterClockwise) | Arc3D(placement, minor, major, startAngle, deltaAngle) | Arc3D(placement, minor, major, startAngle, endAngle, counterClockwise) | Arc3D(arc2D) | Arc3D(element)` — Initialize

### Methods
#### `Close()`

Close the arc. Set the delta angle to 2Pi, to close the arc

**Remarks:** Close the arc. Set the delta angle to 2Pi, to close the arc

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.Close)

#### `EqualRef(arc)`

Test for equal reference point

**Remarks:** Test for equal reference point

**Parameters:**
- `arc` (Arc3D) — Arc3D to compare with

**Returns:** `bool` — result as bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.EqualRef)

#### `GetCenter()`

Get center.

**Remarks:** Get center.

**Returns:** `Point3D` — Point3D center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetCenter)

#### `GetCenterRel()`

Get center in relative coordinate system

**Remarks:** Get center in relative coordinate system

**Returns:** `Point3D` — Point3D center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetCenterRel)

#### `GetDeltaAngle()`

Get the delta angle

**Remarks:** Get the delta angle

**Returns:** `Angle` — delta angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetDeltaAngle)

#### `GetEndAngle()`

Get end angle.

**Remarks:** Get end angle.

**Returns:** `Angle` — angle as const Angle.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetEndAngle)

#### `GetEndPoint()`

Get end point in world coordinates

**Remarks:** Get end point in world coordinates

**Returns:** `Point3D` — end point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetEndPoint)

#### `GetEndRelPoint()`

Get end point in relative coordinates

**Remarks:** Get end point in relative coordinates

**Returns:** `Point3D` — End Point in relative coordinates.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetEndRelPoint)

#### `GetLocalPoint(point)`

Get the local coordinates of a global world point

**Remarks:** Get the local coordinates of a global world point

**Parameters:**
- `point` (Point3D) — Global point

**Returns:** `Point3D` — point angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetLocalPoint)

#### `GetMajorAxis()`

Get the major axis The major axis is calculated according to the plane normal and the axis angle

**Remarks:** Get the major axis The major axis is calculated according to the plane normal and the axis angle

**Returns:** `Vector3D` — major axis as Vector3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetMajorAxis)

#### `GetMajorRadius()`

Get major radius.

**Remarks:** Get major radius.

**Returns:** `float` — major radius as const double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetMajorRadius)

#### `GetMinorAxis()`

Get the minor axis The minor axis is calculated via the cross product of major axis and plane normal

**Remarks:** Get the minor axis The minor axis is calculated via the cross product of major axis and plane normal

**Returns:** `Vector3D` — minor axis as Vector3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetMinorAxis)

#### `GetMinorRadius()`

Get minor radius.

**Remarks:** Get minor radius.

**Returns:** `float` — minor radius as const double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetMinorRadius)

#### `GetNormVector()`

Get normvector.

**Remarks:** Get normvector.

**Returns:** `Vector3D` — normvector as const Vector3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetNormVector)

#### `GetOrigin()`

Get Origin.

**Remarks:** Get Origin.

**Returns:** `Point3D` — Point3D Origin

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetOrigin)

#### `GetPoint(angle)`

Get point on arc with given angle in world coordinates

**Remarks:** Get point on arc with given angle in world coordinates

**Parameters:**
- `angle` (Angle) — central angle of the point

**Returns:** `Point3D` — point on arc as Point3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetPoint)

#### `GetPointAngle(point)`

Get the angle of a point

**Remarks:** Get the angle of a point

**Parameters:**
- `point` (Point3D) — Global point

**Returns:** `Angle` — point angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetPointAngle)

#### `GetPointRel(angle)`

Get point on arc with given angle in relative coordinates

**Remarks:** Get point on arc with given angle in relative coordinates

**Parameters:**
- `angle` (Angle) — central angle of the point

**Returns:** `Point3D` — point on arc as Point3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetPointRel)

#### `GetRefPlacement()`

Get the reference placement

**Remarks:** Get the reference placement

**Returns:** `AxisPlacement3D` — reference placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetRefPlacement)

#### `GetRefPlacementRel()`

Get the reference placement

**Remarks:** Get the reference placement

**Returns:** `AxisPlacement3D` — reference placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetRefPlacementRel)

#### `GetRefPoint()`

Get the reference point

**Remarks:** Get the reference point

**Returns:** `Point3D` — reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetRefPoint)

#### `GetStartAngle()`

Get start angle.

**Remarks:** Get start angle.

**Returns:** `Angle` — angle as const Angle.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetStartAngle)

#### `GetStartPoint()`

Get start point in world coordinates

**Remarks:** Get start point in world coordinates

**Returns:** `Point3D` — start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetStartPoint)

#### `GetStartRelPoint()`

Get start point in relative coordinates

**Remarks:** Get start point in relative coordinates

**Returns:** `Point3D` — start point in relative coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetStartRelPoint)

#### `GetXAxis()`

Get Major axis (X-Axis of the placement).

**Remarks:** Get Major axis (X-Axis of the placement).

**Returns:** `Vector3D` — Major axis as const Vector3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetXAxis)

#### `GetZAxis()`

Get Normal vector of the placement.

**Remarks:** Get Normal vector of the placement.

**Returns:** `Vector3D` — Normal vector as const Vector3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.GetZAxis)

#### `IsAngleOnArc(angle)`

Checks if the given angle lies on the arc

**Remarks:** Checks if the given angle lies on the arc

**Parameters:**
- `angle` (Angle) — angle to test

**Returns:** `bool` — Angle on arc true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.IsAngleOnArc)

#### `IsCircle()`

Check if minor radius equals major radius

**Remarks:** Check if minor radius equals major radius

**Returns:** `bool` — result of check as bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.IsCircle)

#### `IsClockwise()`

Returns winding direction clockwise true/false

**Remarks:** Returns winding direction clockwise true/false

**Returns:** `bool` — Is winding direction clockwise true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.IsClockwise)

#### `IsClosed()`

Check delta angle is 2pi

**Remarks:** Check delta angle is 2pi

**Returns:** `bool` — result of check as bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.IsClosed)

#### `IsCounterClockwise()`

Returns winding direction counterclockwise true/false

**Remarks:** Returns winding direction counterclockwise true/false

**Returns:** `bool` — Is winding direction counterclockwise true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.IsCounterClockwise)

#### `IsEpsilonClosed()`

Check delta angle is 2pi

**Remarks:** Check delta angle is 2pi

**Returns:** `bool` — result of check as bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.IsEpsilonClosed)

#### `IsValid()`

Checks if the arc is valid

**Remarks:** Checks if the arc is valid

**Returns:** `bool` — Validity true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.IsValid)

#### `Reverse()`

Reverse of current arc Method reverse Arc, start with end angle and orientation.

**Remarks:** Reverse of current arc Method reverse Arc, start with end angle and orientation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.Reverse)

#### `RotateAroundLocalZAxis(angle)`

Rotate the arc around the Z-axis (normal vector)

**Remarks:** Rotate the arc around the Z-axis (normal vector)

**Parameters:**
- `angle` (Angle) — Rotation angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.RotateAroundLocalZAxis)

#### `SetCenter(center)`

Set center point.

**Remarks:** Set center point.

**Parameters:**
- `center` (Point3D) — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetCenter)

#### `SetCenterRel(center)`

Set center point in local coordinate system

**Remarks:** Set center point in local coordinate system

**Parameters:**
- `center` (Point3D) — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetCenterRel)

#### `SetClockwise(cw) | SetClockwise()`

Set the winding direction of the arc

**Remarks:** Set the winding direction of the arc

**Parameters:**
- `cw` (bool) — winding clockwise true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetClockwise)

#### `SetCounterClockwise(ccw) | SetCounterClockwise()`

Set the winding direction of the arc

**Remarks:** Set the winding direction of the arc

**Parameters:**
- `ccw` (bool) — winding counterclockwise true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetCounterClockwise)

#### `SetDeltaAngle(deltaAngle)`

Set the delta angle. Set the delta angle

**Remarks:** Set the delta angle. Set the delta angle

**Parameters:**
- `deltaAngle` (Angle) — angle which will be set.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetDeltaAngle)

#### `SetEndAngle(angle) | SetEndAngle(angle, ccw)`

Set end angle. Set the end angle and normalize it to [0,2PI[

**Remarks:** Set end angle. Set the end angle and normalize it to [0,2PI[

**Parameters:**
- `angle` (Angle) — angle which will be set.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetEndAngle)

#### `SetEndPoint(endpoint)`

Set the end point of an object in world coordinates

**Remarks:** Set the end point of an object in world coordinates

**Parameters:**
- `endpoint` (Point3D) — New end point of curve

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetEndPoint)

#### `SetMajorRadius(radius)`

Set major radius.

**Remarks:** Set major radius.

**Parameters:**
- `radius` (float) — major radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetMajorRadius)

#### `SetMinorRadius(radius)`

Set minor radius.

**Remarks:** Set minor radius.

**Parameters:**
- `radius` (float) — minor radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetMinorRadius)

#### `SetNormVector(normalVec)`

Set the normvector Set the plane normal and normalizes it to unit vector

**Remarks:** Set the normvector Set the plane normal and normalizes it to unit vector

**Parameters:**
- `normalVec` (Vector3D) — Vector3D to be set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetNormVector)

#### `SetOrigin(center)`

Set Origin

**Remarks:** Set Origin

**Parameters:**
- `center` (Point3D) — Origin

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetOrigin)

#### `SetRefPlacement(refPlacement) | SetRefPlacement(center, xAxis, normalVec)`

Set the reference placement

**Remarks:** Set the reference placement

**Parameters:**
- `refPlacement` (AxisPlacement3D) — reference placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetRefPlacement)

#### `SetRefPlacementRel(refPlacement)`

Set the reference placement relative to the refPoint

**Remarks:** Set the reference placement relative to the refPoint

**Parameters:**
- `refPlacement` (AxisPlacement3D) — reference placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetRefPlacementRel)

#### `SetRefPoint(refPoint)`

Set the reference point

**Remarks:** Set the reference point

**Parameters:**
- `refPoint` (Point3D) — reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetRefPoint)

#### `SetStartAngle(angle) | SetStartAngle(angle, ccw)`

Set start angle Set the start angle and normalize it to [0,2PI[

**Remarks:** Set start angle Set the start angle and normalize it to [0,2PI[

**Parameters:**
- `angle` (Angle) — angle which will be set.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetStartAngle)

#### `SetStartPoint(startpoint)`

Set the start point of an object in world coordinates

**Remarks:** Set the start point of an object in world coordinates

**Parameters:**
- `startpoint` (Point3D) — New start point of curve

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.SetStartPoint)

#### `__eq__(arc)`

Comparison of arcs. Be careful, this method work without tolerance!

**Remarks:** Comparison of arcs. Be careful, this method work without tolerance!

**Parameters:**
- `arc` (Arc3D) — arc to be compared.

**Returns:** `bool` — True when arcs are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3D/#NemAll_Python_Geometry.Arc3D.__repr__)

### Properties
- `Center` (Point3D, get/set) — Get center.
- `CenterRel` (Point3D, get/set) — Get center in relative coordinate system
- `CounterClockwise` (bool, get/set) — Returns winding direction counterclockwise true/false
- `DeltaAngle` (Angle, get/set) — Get the delta angle
- `EndAngle` (Angle, get/set) — Get end angle.
- `EndPoint` (Point3D, get/set) — Get end point in world coordinates
- `EndRelPoint` (Point3D, get) — Get end point in relative coordinates
- `MajorRadius` (float, get/set) — Get major radius.
- `MinorRadius` (float, get/set) — Get minor radius.
- `NormVector` (Vector3D, get/set) — Get normvector.
- `Origin` (Point3D, get/set) — Get Origin.
- `RefPlacement` (AxisPlacement3D, get/set) — Get the reference placement
- `RefPlacementRel` (AxisPlacement3D, get/set) — Get the reference placement
- `RefPoint` (Point3D, get/set) — Get the reference point
- `StartAngle` (Angle, get/set) — Get start angle.
- `StartPoint` (Point3D, get/set) — Get start point in world coordinates
- `StartRelPoint` (Point3D, get) — Get start point in relative coordinates

## Arc3DList (class)

List for Arc3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/)

### Constructors
- `Arc3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Arc3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Arc3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (Arc3DList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Arc3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Arc3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Arc3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Arc3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Arc3DList/#NemAll_Python_Geometry.Arc3DList.extend)

## Axis2D (class)

Representation class for 2D Axis

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/)

### Constructors
- `Axis2D() | Axis2D(axisPoint, vector) | Axis2D(refPoint, axisPoint, vector) | Axis2D(line) | Axis2D(element)` — Initialize

### Methods
#### `GetAxisPoint()`

Get axis point in world coordinate system.

**Remarks:** Get axis point in world coordinate system.

**Returns:** `Point2D` — Axis point in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.GetAxisPoint)

#### `GetAxisPoint2()`

Get second axis point. Used world coordinate system.

**Remarks:** Get second axis point. Used world coordinate system.

**Returns:** `Point2D` — Second axis point in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.GetAxisPoint2)

#### `GetAxisRelPoint()`

Get axis point in local coordinate system

**Remarks:** Get axis point in local coordinate system

**Returns:** `Point2D` — Axis point in local coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.GetAxisRelPoint)

#### `GetAxisRelPoint2()`

Get second axis point. Used local coordinate system

**Remarks:** Get second axis point. Used local coordinate system

**Returns:** `Point2D` — Second axis point in local coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.GetAxisRelPoint2)

#### `GetRefPoint()`

Get reference point.

**Remarks:** Get reference point.

**Returns:** `Point2D` — Reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.GetRefPoint)

#### `GetVector()`

Get axis vector

**Remarks:** Get axis vector

**Returns:** `Vector2D` — Axis vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.GetVector)

#### `Set(axis) | Set(refPoint, axisPoint, vector)`

Set axis

**Remarks:** Set axis

**Parameters:**
- `axis` (Axis2D) — Axis which will be copied

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.Set)

#### `SetAxisPoint(point)`

Set axis point, used world coordinate system.

**Remarks:** Set axis point, used world coordinate system.

**Parameters:**
- `point` (Point2D) — Axis point in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.SetAxisPoint)

#### `SetAxisRelPoint(axisPoint)`

Set axis point, used local coordinate system.

**Remarks:** Set axis point, used local coordinate system.

**Parameters:**
- `axisPoint` (Point2D) — Axis point in local coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.SetAxisRelPoint)

#### `SetRefPoint(refPoint)`

Set reference point

**Remarks:** Set reference point

**Parameters:**
- `refPoint` (Point2D) — New reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.SetRefPoint)

#### `SetVector(vector)`

Set axis vector

**Remarks:** Set axis vector

**Parameters:**
- `vector` (Vector2D) — New axis vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.SetVector)

#### `__eq__(axis)`

Comparison of axes. Be careful, this method work without tolerance!

**Remarks:** Comparison of axes. Be careful, this method work without tolerance!

**Parameters:**
- `axis` (Axis2D) — axis to be compared.

**Returns:** `bool` — True when axes are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis2D/#NemAll_Python_Geometry.Axis2D.__repr__)

### Properties
- `AxisPoint` (Point2D, get/set) — Get axis point in world coordinate system.
- `AxisRefPoint` (Point2D, get/set) — Get reference point.
- `AxisRelPoint` (Point2D, get/set) — Get axis point in local coordinate system
- `AxisVector` (Vector2D, get/set) — Get axis vector

## Axis3D (class)

Representation class for 3D Axis

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/)

### Constructors
- `Axis3D() | Axis3D(axisPoint, vector) | Axis3D(refPoint, axisPoint, vector) | Axis3D(line) | Axis3D(element)` — Initialize

### Methods
#### `GetAxisPoint()`

Get axis point in world coordinate system.

**Remarks:** Get axis point in world coordinate system.

**Returns:** `Point3D` — Axis point in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.GetAxisPoint)

#### `GetAxisPoint2()`

Get second axis point. Used world coordinate system.

**Remarks:** Get second axis point. Used world coordinate system.

**Returns:** `Point3D` — Second axis point in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.GetAxisPoint2)

#### `GetAxisRelPoint()`

Get axis point in local coordinate system

**Remarks:** Get axis point in local coordinate system

**Returns:** `Point3D` — Axis point in local coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.GetAxisRelPoint)

#### `GetAxisRelPoint2()`

Get second axis point. Used local coordinate system

**Remarks:** Get second axis point. Used local coordinate system

**Returns:** `Point3D` — Second axis point in local coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.GetAxisRelPoint2)

#### `GetRefPoint()`

Get reference point.

**Remarks:** Get reference point.

**Returns:** `Point3D` — Reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.GetRefPoint)

#### `GetVector()`

Get axis vector

**Remarks:** Get axis vector

**Returns:** `Vector3D` — Axis vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.GetVector)

#### `Set(axis) | Set(refPoint, axisPoint, vector)`

Set axis

**Remarks:** Set axis

**Parameters:**
- `axis` (Axis3D) — Axis which will be copied

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.Set)

#### `SetAxisPoint(point)`

Set axis point, used world coordinate system.

**Remarks:** Set axis point, used world coordinate system.

**Parameters:**
- `point` (Point3D) — Axis point in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.SetAxisPoint)

#### `SetAxisRelPoint(axisPoint)`

Set axis point, used local coordinate system.

**Remarks:** Set axis point, used local coordinate system.

**Parameters:**
- `axisPoint` (Point3D) — Axis point in local coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.SetAxisRelPoint)

#### `SetRefPoint(refPoint)`

Set reference point

**Remarks:** Set reference point

**Parameters:**
- `refPoint` (Point3D) — New reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.SetRefPoint)

#### `SetVector(vector)`

Set axis vector

**Remarks:** Set axis vector

**Parameters:**
- `vector` (Vector3D) — New axis vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.SetVector)

#### `__eq__(axis)`

Comparison of axes. Be careful, this method work without tolerance!

**Remarks:** Comparison of axes. Be careful, this method work without tolerance!

**Parameters:**
- `axis` (Axis3D) — axis to be compared.

**Returns:** `bool` — True when axes are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Axis3D/#NemAll_Python_Geometry.Axis3D.__repr__)

### Properties
- `AxisPoint` (Point3D, get/set) — Get axis point in world coordinate system.
- `AxisRefPoint` (Point3D, get/set) — Get reference point.
- `AxisRelPoint` (Point3D, get/set) — Get axis point in local coordinate system
- `AxisVector` (Vector3D, get/set) — Get axis vector

## AxisPlacement2D (class)

Representation class for orthogonal axis placement

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/)

### Constructors
- `AxisPlacement2D() | AxisPlacement2D(refPoint, dirvector) | AxisPlacement2D(element)` — Initialize

### Methods
#### `GetDirection()`

Get direction

**Remarks:** Get direction

**Returns:** `Vector2D` — Vector2D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.GetDirection)

#### `GetRefPoint()`

Get the reference point

**Remarks:** Get the reference point

**Returns:** `Point2D` — Reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.GetRefPoint)

#### `GetYDirection()`

Get direction of local y-axis

**Remarks:** Get direction of local y-axis

**Returns:** `Vector2D` — Vector2D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.GetYDirection)

#### `IsValid()`

Check if the placement is valid

**Remarks:** Check if the placement is valid

**Returns:** `bool` — True if it is a valid placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.IsValid)

#### `SetDirection(dir)`

Set direction

**Remarks:** Set direction

**Parameters:**
- `dir` (Vector2D) — Vector2D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.SetDirection)

#### `SetRefPoint(refPoint)`

Set reference point

**Remarks:** Set reference point

**Parameters:**
- `refPoint` (Point2D) — New reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.SetRefPoint)

#### `__eq__(axis_placement)`

Comparison of axis placements. Be careful, this method work without tolerance!

**Remarks:** Comparison of axis placements. Be careful, this method work without tolerance!

**Parameters:**
- `axis_placement` (AxisPlacement2D) — axis placement to be compared.

**Returns:** `bool` — True when axis placements are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2D/#NemAll_Python_Geometry.AxisPlacement2D.__repr__)

### Properties
- `Direction` (Vector2D, get/set) — Get direction
- `RefPoint` (Point2D, get/set) — Get the reference point

## AxisPlacement2DList (class)

List for AxisPlacement2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/)

### Constructors
- `AxisPlacement2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (AxisPlacement2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (AxisPlacement2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (AxisPlacement2DList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `AxisPlacement2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (AxisPlacement2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (AxisPlacement2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (AxisPlacement2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement2DList/#NemAll_Python_Geometry.AxisPlacement2DList.extend)

## AxisPlacement3D (class)

Representation class for orthogonal axis placement in 3D space

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/)

### Constructors
- `AxisPlacement3D() | AxisPlacement3D(refPoint) | AxisPlacement3D(refPoint, xvector, zvector) | AxisPlacement3D(rotationAxis, rotationStart) | AxisPlacement3D(matrix) | AxisPlacement3D(element)` — Initialize

### Methods
#### `CalcGlobalPoint(point)`

Calculate global point from local coordinate system of placement 3D

**Remarks:** Calculate global point from local coordinate system of placement 3D

**Parameters:**
- `point` (Point3D) — local point 3D

**Returns:** `Point3D` — Point3D instance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.CalcGlobalPoint)

#### `CalcLocalPoint(point)`

Calculate local point in coordinate system of placement 3D

**Remarks:** Calculate local point in coordinate system of placement 3D

**Parameters:**
- `point` (Point3D) — global point 3D

**Returns:** `Point3D` — Point3D instance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.CalcLocalPoint)

#### `GetOrigin()`

Get origin

**Remarks:** Get origin

**Returns:** `Point3D` — Origin

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.GetOrigin)

#### `GetRotationMatrix()`

Get Rotation matrix given by placement 3D

**Remarks:** Get Rotation matrix given by placement 3D

**Returns:** `Matrix3D` — Matrix3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.GetRotationMatrix)

#### `GetTransformationMatrix()`

Get Transformation matrix given by placement 3D

**Remarks:** Get Transformation matrix given by placement 3D

**Returns:** `Matrix3D` — Matrix3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.GetTransformationMatrix)

#### `GetXDirection()`

Get x-direction

**Remarks:** Get x-direction

**Returns:** `Vector3D` — Vector3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.GetXDirection)

#### `GetYDirection()`

Get direction of local y-axis

**Remarks:** Get direction of local y-axis

**Returns:** `Vector3D` — Vector3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.GetYDirection)

#### `GetZDirection()`

Get z-direction

**Remarks:** Get z-direction

**Returns:** `Vector3D` — Vector3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.GetZDirection)

#### `IsValid()`

Check if the placement is valid

**Remarks:** Check if the placement is valid

**Returns:** `bool` — True if it is a valid placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.IsValid)

#### `RotateAroundLocalZAxis(angle)`

Rotate the placement around it's Z-axis

**Remarks:** Rotate the placement around it's Z-axis

**Parameters:**
- `angle` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.RotateAroundLocalZAxis)

#### `Set(origin, xDir, zDir)`

Set the placement

**Remarks:** Set the placement

**Parameters:**
- `origin` (Point3D) — New origin
- `xDir` (Vector3D) — New X-direction
- `zDir` (Vector3D) — New Z-direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.Set)

#### `SetOrigin(origin)`

Set origin

**Remarks:** Set origin

**Parameters:**
- `origin` (Point3D) — New origin

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.SetOrigin)

#### `SetXDirection(dir)`

Set x-direction

**Remarks:** Set x-direction

**Parameters:**
- `dir` (Vector3D) — Vector3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.SetXDirection)

#### `SetZDirection(dir)`

Set z-direction

**Remarks:** Set z-direction

**Parameters:**
- `dir` (Vector3D) — Vector3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.SetZDirection)

#### `__eq__(axis_placement)`

Comparison of axis placements. Be careful, this method work without tolerance!

**Remarks:** Comparison of axis placements. Be careful, this method work without tolerance!

**Parameters:**
- `axis_placement` (AxisPlacement3D) — axis placement to be compared.

**Returns:** `bool` — True when axis placements are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/#NemAll_Python_Geometry.AxisPlacement3D.__repr__)

### Properties
- `Origin` (Point3D, get/set) — Get origin
- `XDirection` (Vector3D, get/set) — Get x-direction
- `ZDirection` (Vector3D, get/set) — Get z-direction

## AxisPlacement3DList (class)

List for AxisPlacement3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/)

### Constructors
- `AxisPlacement3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (AxisPlacement3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (AxisPlacement3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (AxisPlacement3DList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `AxisPlacement3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (AxisPlacement3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (AxisPlacement3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (AxisPlacement3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3DList/#NemAll_Python_Geometry.AxisPlacement3DList.extend)

## BRep3D (class)

Representation class for 3D boundary representation.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/)

### Constructors
- `BRep3D() | BRep3D(brep)` — Initialize

### Methods
#### `AreFacesNaturallyTrimmed()`

Find if all faces are naturally trimmed by their surfaces

**Remarks:** Find if all faces are naturally trimmed by their surfaces

**Returns:** `bool` — bool (true = yes)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.AreFacesNaturallyTrimmed)

#### `CreateCone(cone, closed=True)`

Create BRep3D as cone

**Remarks:** Create BRep3D as cone

**Parameters:**
- `cone` (Cone3D) — cone data
- `closed` (bool) — whether to create solid or sheet body

**Returns:** `BRep3D` — created geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.CreateCone)

#### `CreateCuboid(placement, length, width, height)`

Create BRep3D as cuboid

**Remarks:** Create BRep3D as cuboid

**Parameters:**
- `placement` (AxisPlacement3D) — cuboid origin
- `length` (float) — length in its x axis
- `width` (float) — width in its y axis
- `height` (float) — height in its z axis

**Returns:** `BRep3D` — created geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.CreateCuboid)

#### `CreateCylinder(placement, radius, height, closedTop=True, closedBottom=True)`

Create Brep as cylinder

**Remarks:** Create Brep as cylinder

**Parameters:**
- `placement` (AxisPlacement3D) — axis placement
- `radius` (float) — cylinder radius
- `height` (float) — cylinder height
- `closedTop` (bool) — whether to close top of cylinder
- `closedBottom` (bool) — whether to close bottom of cylinder

**Returns:** `BRep3D` — created cone

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.CreateCylinder)

#### `CreateSphere(placement, radius)`

Create BRep3D as sphere

**Remarks:** Create BRep3D as sphere

**Parameters:**
- `placement` (AxisPlacement3D) — sphere origin
- `radius` (float) — sphere radius

**Returns:** `BRep3D` — created geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.CreateSphere)

#### `CreateWireBody(icurve_object)`

Create wire body from the curve as one edge

**Remarks:** Create wire body from the curve as one edge

**Parameters:**
- `icurve_object` (object) — curve for edge geometry

**Returns:** `BRep3D` — created BRep3D element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.CreateWireBody)

#### `DeleteFace(faceIndex)`

Delete face from brep

**Remarks:** Delete face from brep

**Parameters:**
- `faceIndex` (int) — index of face we want to delete

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.DeleteFace)

#### `DeleteFaces(faceIndices)`

Delete faces from brep

**Remarks:** Delete faces from brep

**Parameters:**
- `faceIndices` (VecULongList) — indices of faces to delete

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.DeleteFaces)

#### `GetAllFacesFlags()`

Get flags of the all faces.

**Remarks:** Get flags of the all faces.

**Returns:** `eGeometryErrorCode` — tuple(Error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetAllFacesFlags)

#### `GetEdgeCount()`

Get the edges count

**Remarks:** Get the edges count

**Returns:** `int` — size_t count of edges

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeCount)

#### `GetEdgeCurves()`

get all edge curves as BSpline3D

**Remarks:** get all edge curves as BSpline3D

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeCurves)

#### `GetEdgeFaceIndices(edge)`

Get faces containing this edge (if any)

**Remarks:** Get faces containing this edge (if any)

**Parameters:**
- `edge` (int) — desired edge index

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeFaceIndices)

#### `GetEdgeGeometry(edge)`

Get (trimmed) edge geometry as BSpline3D

**Remarks:** Get (trimmed) edge geometry as BSpline3D

**Parameters:**
- `edge` (int) — egde index

**Returns:** `object` — handle to geometry curve

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeGeometry)

#### `GetEdgeParametricCurves()`

get parametric curves of all edges

**Remarks:** get parametric curves of all edges

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeParametricCurves)

#### `GetEdgeParametricGeometry(edge)`

Get (trimmed) edge geometry as parametric curves

**Remarks:** Get (trimmed) edge geometry as parametric curves

**Parameters:**
- `edge` (int) — egde index

**Returns:** `object` — handle to geometry curve

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeParametricGeometry)

#### `GetEdgeVertexIndices(edge)`

Get vertex indices of the edge

**Remarks:** Get vertex indices of the edge

**Parameters:**
- `edge` (int) — edge index

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeVertexIndices)

#### `GetEdgeVertices(edge)`

get edge vertices

**Remarks:** get edge vertices

**Parameters:**
- `edge` (int) — edge index

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetEdgeVertices)

#### `GetExactTypeIsoCurves(ucount, vcount, planarfaces=False)`

Get iso curves from all faces

**Remarks:** Get iso curves from all faces

**Parameters:**
- `ucount` (int) — number of u-curves
- `vcount` (int) — number of v-curves
- `planarfaces` (bool) — include planar faces

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetExactTypeIsoCurves)

#### `GetFaceBoundaryCurves(face)`

Get face boundary curves, curves are trimmed

**Remarks:** Get face boundary curves, curves are trimmed

**Parameters:**
- `face` (int) — face index

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceBoundaryCurves)

#### `GetFaceCount()`

Get the faces count

**Remarks:** Get the faces count

**Returns:** `int` — size_t count of faces

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceCount)

#### `GetFaceEdgeNaturalTrimming(face, oedge)`

get edge trimming on this face

**Remarks:** get edge trimming on this face

**Parameters:**
- `face` (int) — face index
- `oedge` (OrientedEdge) — oriented edge

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceEdgeNaturalTrimming)

#### `GetFaceEdgeOrientation(face, edge)`

Get orientation of the edge in the given face

**Remarks:** Get orientation of the edge in the given face

**Parameters:**
- `face` (int) — face index
- `edge` (int) — edge index

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceEdgeOrientation)

#### `GetFaceEdges(faceIndex)`

Get face edges together with their orientations

**Remarks:** Get face edges together with their orientations

**Parameters:**
- `faceIndex` (int) — face index

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceEdges)

#### `GetFaceFlags(face)`

Get flags of the face.

**Remarks:** Get flags of the face.

**Parameters:**
- `face` (int) — face index

**Returns:** `int` — flags value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceFlags)

#### `GetFaceGeometry(face)`

Get surface geometry of the face

**Remarks:** Get surface geometry of the face

**Parameters:**
- `face` (int) — face index

**Returns:** `object` — surface geometry handle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceGeometry)

#### `GetFaceGeometryOrientation(face)`

Get surface geometry orientation of the face

**Remarks:** Get surface geometry orientation of the face

**Parameters:**
- `face` (int) — face index

**Returns:** `bool` — surface geometry orientation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceGeometryOrientation)

#### `GetFaceLoopsCount(face)`

Get count of loops of face given by index

**Remarks:** Get count of loops of face given by index

**Parameters:**
- `face` (int) — index of face

**Returns:** `int` — count of loops

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceLoopsCount)

#### `GetFaceUVBox(face)`

Get uv box of the face

**Remarks:** Get uv box of the face

**Parameters:**
- `face` (int) — selected face

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceUVBox)

#### `GetFaceVerticesIndices(faceIndex)`

Get indices of vertices for given face index

**Remarks:** Get indices of vertices for given face index

**Parameters:**
- `faceIndex` (int) — index of face

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetFaceVerticesIndices)

#### `GetIsoCurves(ucount, vcount, planarfaces=False)`

Get iso curves from all faces

**Remarks:** Get iso curves from all faces

**Parameters:**
- `ucount` (int) — number of u-curves
- `vcount` (int) — number of v-curves
- `planarfaces` (bool) — include planar faces

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetIsoCurves)

#### `GetParts()`

Get separated parts (continuos shells)

**Remarks:** Get separated parts (continuos shells)

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetParts)

#### `GetPartsCount()`

Get number of parts in this body

**Remarks:** Get number of parts in this body

**Returns:** `int` — number of parts

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetPartsCount)

#### `GetPlanarFaces()`

Get planar faces if this brep contains planar faces

**Remarks:** Get planar faces if this brep contains planar faces

**Returns:** `tuple[Plane3D, int]` — pairs of - surface geometry and surface index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetPlanarFaces)

#### `GetRefPoint()`

Get the reference point

**Remarks:** Get the reference point

**Returns:** `Point3D` — Constant reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetRefPoint)

#### `GetSilhouetteCurves(viewMatrix, bPerspective)`

Get silhouette curves of Brep

**Remarks:** Get silhouette curves of Brep

**Parameters:**
- `viewMatrix` (Matrix3D) — View matrix
- `bPerspective` (bool) — Flag if it is central projection or not (true / false)

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetSilhouetteCurves)

#### `GetSilhouetteVertices(viewMatrix, bPerspective)`

Get vertices (end points) of silhouette curves

**Remarks:** Get vertices (end points) of silhouette curves

**Parameters:**
- `viewMatrix` (Matrix3D) — view matrix
- `bPerspective` (bool) — Flag if view is central perspective (true / false)

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetSilhouetteVertices)

#### `GetVertex(vertexIndex)`

Get vertex geometry

**Remarks:** Get vertex geometry

**Parameters:**
- `vertexIndex` (int) — vertex index

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetVertex)

#### `GetVertexCount()`

Get the vertices count

**Remarks:** Get the vertices count

**Returns:** `int` — count of vertices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetVertexCount)

#### `GetVertexEdges(vertex)`

Get the edges containing given vertex

**Remarks:** Get the edges containing given vertex

**Parameters:**
- `vertex` (int) — Index of vertex

**Returns:** `eGeometryErrorCode` — tuple(Error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetVertexEdges)

#### `GetVertexFaceIndices(vertexIndex)`

Get indices of faces for given vertex index

**Remarks:** Get indices of faces for given vertex index

**Parameters:**
- `vertexIndex` (int) — index of vertex

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetVertexFaceIndices)

#### `GetVertices()`

Get all vertices from brep

**Remarks:** Get all vertices from brep

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetVertices)

#### `GetVirtualVertices()`

Get vertices from edges without vertex (get start points of edge curves)

**Remarks:** Get vertices from edges without vertex (get start points of edge curves)

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetVirtualVertices)

#### `GetVirtualVerticesCount()`

Get count of vertices (count of edges without vertex)

**Remarks:** Get count of vertices (count of edges without vertex)

**Returns:** `int` — count of vertices (edges without vertex)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.GetVirtualVerticesCount)

#### `HasPlanarFaces()`

Check if this brep contains planar faces

**Remarks:** Check if this brep contains planar faces

**Returns:** `bool` — bool true = yes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.HasPlanarFaces)

#### `InvertAllFacesFlags()`

Invert flags for the all faces

**Remarks:** Invert flags for the all faces

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.InvertAllFacesFlags)

#### `IsClosed()`

Check whether the B-rep is closed (closed shells)

**Remarks:** Check whether the B-rep is closed (closed shells)

**Returns:** `bool` — bool true = yes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsClosed)

#### `IsCone()`

Check if the b-rep is a cone

**Remarks:** Check if the b-rep is a cone

**Returns:** `bool` — tuple(bool true = yes,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsCone)

#### `IsEmpty()`

Returns true if the body is empty

**Remarks:** Returns true if the body is empty

**Returns:** `bool` — bool

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsEmpty)

#### `IsFaceNaturallyTrimmed(face)`

Find if the face is naturally trimmed by its surface

**Remarks:** Find if the face is naturally trimmed by its surface

**Parameters:**
- `face` (int) — face index

**Returns:** `bool` — bool (true = yes)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsFaceNaturallyTrimmed)

#### `IsPolyhedron()`

Check if the b-rep is a polyhedron

**Remarks:** Check if the b-rep is a polyhedron

**Returns:** `bool` — bool true = yes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsPolyhedron)

#### `IsSphere()`

Check if the b-rep is a sphere

**Remarks:** Check if the b-rep is a sphere

**Returns:** `bool` — tuple(bool true = yes,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsSphere)

#### `IsValid()`

check whether data is valid

**Remarks:** check whether data is valid

**Returns:** `bool` — bool true = is valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsValid)

#### `IsWire()`

Check if BRep is wire body. If has only edges.

**Remarks:** Check if BRep is wire body. If has only edges.

**Returns:** `bool` — true if BRep is wire.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.IsWire)

#### `PickEdge(rayPoint, rayVector, searchRadius)`

Pick the edge under cursor

**Remarks:** Pick the edge under cursor

**Parameters:**
- `rayPoint` (Point3D) — ray point
- `rayVector` (Vector3D) — ray vector
- `searchRadius` (float) — search radius

**Returns:** `bool` — tuple(true if any edge under cursor found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.PickEdge)

#### `PickEdges(rayPoint, rayVector, searchRadius)`

Pick the edges under cursor

**Remarks:** Pick the edges under cursor

**Parameters:**
- `rayPoint` (Point3D) — ray point
- `rayVector` (Vector3D) — ray vector
- `searchRadius` (float) — search radius

**Returns:** `bool` — tuple(true if any edge under cursor found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.PickEdges)

#### `PickFace(rayPoint, rayVector)`

Pick the face under cursor

**Remarks:** Pick the face under cursor

**Parameters:**
- `rayPoint` (Point3D) — ray point
- `rayVector` (Vector3D) — ray vector

**Returns:** `bool` — tuple(true if any face under cursor found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.PickFace)

#### `PickVertex(rayPoint, rayVector, searchRadius)`

Pick the vertex under cursor

**Remarks:** Pick the vertex under cursor

**Parameters:**
- `rayPoint` (Point3D) — ray point
- `rayVector` (Vector3D) — ray vector
- `searchRadius` (float) — search radius

**Returns:** `bool` — tuple(true if any vertex under cursor found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.PickVertex)

#### `ReadFromStream(sstream_str, transformationMatrix) | ReadFromStream(sstream_str, scale, translation, spaceminmax, visibleminmax)`

Read BRep3D from string

**Remarks:** Read BRep3D from string

**Parameters:**
- `sstream_str` (str) — input string
- `transformationMatrix` (Matrix3D) — Matrix to use for BRep transformation

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.ReadFromStream)

#### `Reverse()`

Reverse the orientation of BRep

**Remarks:** Reverse the orientation of BRep

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.Reverse)

#### `SetAllFacesFlags(flags)`

Set flags to the all faces.

**Remarks:** Set flags to the all faces.

**Parameters:**
- `flags` (int) — flags value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.SetAllFacesFlags)

#### `SetFaceFlags(face, flags)`

Set flags to the face.

**Remarks:** Set flags to the face.

**Parameters:**
- `face` (int) — face index
- `flags` (int) — flags value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.SetFaceFlags)

#### `SetRefPoint(refPoint)`

Set the reference point

**Remarks:** Set the reference point

**Parameters:**
- `refPoint` (Point3D) — New reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.SetRefPoint)

#### `WriteToStream()`

Write BRep3D to the text

**Remarks:** Write BRep3D to the text

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.WriteToStream)

#### `WriteToStreamSplitTransform()`

Write BRep3D to the text, get transform parameters and minmax

**Remarks:** Write BRep3D to the text, get transform parameters and minmax

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.WriteToStreamSplitTransform)

#### `__eq__(brep)`

Comparison of breps without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of breps without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `brep` (BRep3D) — Compared brep.

**Returns:** `object` — True when breps are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3D/#NemAll_Python_Geometry.BRep3D.__repr__)

### Properties
- `AllFacesFlags` (eGeometryErrorCode, get/set) — Get flags of the all faces.
- `RefPoint` (Point3D, get/set) — Get the reference point

## BRep3DBuilder (class)

Builder for BRep3D

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/)

### Constructors
- `BRep3DBuilder() | BRep3DBuilder(element)` — Initialize

### Methods
#### `AddEdge(edgeIdx, edgeSense, loopIdx) | AddEdge(curve_object, curveSense, edgeSense, loopIdx, precision)`

Add edge to loop

**Remarks:** Add edge to loop

**Parameters:**
- `edgeIdx` (int) — Index of already added edge
- `edgeSense` (bool) — Sense of edge
- `loopIdx` (int) — Index of loop to which edge will be added

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/#NemAll_Python_Geometry.BRep3DBuilder.AddEdge)

#### `AddFace(surface_object, sense)`

Add face

**Remarks:** Add face

**Parameters:**
- `surface_object` (object) — Geometry of surface
- `sense` (bool) — Sense of surface

**Returns:** `int` — Index of face

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/#NemAll_Python_Geometry.BRep3DBuilder.AddFace)

#### `AddLoop(faceIdx)`

Add loop

**Remarks:** Add loop

**Parameters:**
- `faceIdx` (int) — Index of face to which loop will be added

**Returns:** `int` — Index of face

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/#NemAll_Python_Geometry.BRep3DBuilder.AddLoop)

#### `AddVertex(point, edgeIdx, precision) | AddVertex(vertexIdx, edgeIdx)`

Add vertex to edge

**Remarks:** Add vertex to edge

**Parameters:**
- `point` (Point3D) — Geometry point
- `edgeIdx` (int) — Index of edge to which vertex will be added
- `precision` (float) — Vertex precision

**Returns:** `int` — Index of vertex

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/#NemAll_Python_Geometry.BRep3DBuilder.AddVertex)

#### `CheckLoop(loopIdx)`

Check whether loop topology is correct

**Remarks:** Check whether loop topology is correct

**Parameters:**
- `loopIdx` (int) — Index of loop to check

**Returns:** `bool` — Flag whether loop is topologically correct

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/#NemAll_Python_Geometry.BRep3DBuilder.CheckLoop)

#### `Complete()`

Complete topology and create BRep

**Remarks:** Complete topology and create BRep

**Returns:** `BRep3D` — Created BRep

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/#NemAll_Python_Geometry.BRep3DBuilder.Complete)

#### `Init(isSolid)`

Add body, region and shell

**Remarks:** Add body, region and shell

**Parameters:**
- `isSolid` (bool) — Flag whether body is solid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DBuilder/#NemAll_Python_Geometry.BRep3DBuilder.Init)

## BRep3DList (class)

List for BRep3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/)

### Constructors
- `BRep3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (BRep3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (BRep3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `BRep3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (BRep3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BRep3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (BRep3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BRep3DList/#NemAll_Python_Geometry.BRep3DList.extend)

## BSpline2D (class)

class for 2D (non uniform, rational) B-spline geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/)

### Constructors
- `BSpline2D() | BSpline2D(points, weights, knots, degree, isPeriodic) | BSpline2D(spline)` — Initialize

### Methods
#### `Clear()`

Clear data, getting invalid state

**Remarks:** Clear data, getting invalid state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.Clear)

#### `CreateLine2D(line)`

Create BSpline2D from Line2D

**Remarks:** Create BSpline2D from Line2D

**Parameters:**
- `line` (Line2D) — Input line.

**Returns:** `BSpline2D` — Created BSpline2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.CreateLine2D)

#### `Get()`

Return type: tuple(list(Point2D), list(float), list(float), int, bool)

**Remarks:** Return type: tuple(list(Point2D), list(float), list(float), int, bool)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.Get)

#### `GetDegree()`

Gets spline degree

**Remarks:** Gets spline degree

**Returns:** `int` — spline degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.GetDegree)

#### `GetKnots()`

Get knot vector

**Remarks:** Get knot vector

**Returns:** `VecDoubleList` — knot vector const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.GetKnots)

#### `GetWeights()`

Get control points weights

**Remarks:** Get control points weights

**Returns:** `VecDoubleList` — weights vector const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.GetWeights)

#### `IsClosed()`

Check if spline is closed ( first/last points are equal )

**Remarks:** Check if spline is closed ( first/last points are equal )

**Returns:** `bool` — closed spline true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.IsClosed)

#### `IsRational()`

Check if the spline is rational

**Remarks:** Check if the spline is rational

**Returns:** `bool` — bool true = rational

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.IsRational)

#### `IsValid()`

Check spline validity

**Remarks:** Check spline validity

**Returns:** `bool` — bool valid = true

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.IsValid)

#### `Reverse()`

Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

**Remarks:** Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.Reverse)

#### `Set(points, weights, knots, degree, isPeriodic)`

Get/Set functions

**Remarks:** Get/Set functions

**Parameters:**
- `points` (Point2DList)
- `weights` (VecDoubleList)
- `knots` (VecDoubleList)
- `degree` (int)
- `isPeriodic` (bool)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.Set)

#### `SetDegree(degree)`

Set spline degree

**Remarks:** Set spline degree

**Parameters:**
- `degree` (int) — desired degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.SetDegree)

#### `SetKnots(knots)`

Set knot vector

**Remarks:** Set knot vector

**Parameters:**
- `knots` (VecDoubleList) — knot vector to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.SetKnots)

#### `SetPeriodic(arg2)`

Set the periodic flag

**Remarks:** Set the periodic flag

**Parameters:**
- `flag` (object) — True if BSpline is periodic

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.SetPeriodic)

#### `SetWeights(weights)`

Set weights for control points

**Remarks:** Set weights for control points

**Parameters:**
- `weights` (VecDoubleList) — weights vector to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.SetWeights)

#### `__eq__(bspline)`

Comparison of bsplines without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of bsplines without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `bspline` (BSpline2D) — Compared bspline.

**Returns:** `object` — True when bsplines are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2D/#NemAll_Python_Geometry.BSpline2D.__repr__)

### Properties
- `Degree` (int, get/set) — Gets spline degree
- `IsPeriodic` (bool, get/set) — Check if the spline is periodic
- `Knots` (NemAll_Python_Utility.VecDoubleList, get/set) — Get knot vector
- `Weights` (NemAll_Python_Utility.VecDoubleList, get/set) — Get control points weights

## BSpline2DList (class)

List for BSpline2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/)

### Constructors
- `BSpline2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (BSpline2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (BSpline2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `BSpline2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (BSpline2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BSpline2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (BSpline2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline2DList/#NemAll_Python_Geometry.BSpline2DList.extend)

## BSpline3D (class)

class for 3D (non uniform, rational) B-spline geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/)

### Constructors
- `BSpline3D() | BSpline3D(points, weights, knots, degree, isPeriodic) | BSpline3D(spline)` — Initialize

### Methods
#### `Clear()`

Clear data, getting invalid state

**Remarks:** Clear data, getting invalid state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.Clear)

#### `Create(curve_object)`

Create BSpline from ICurve3D

**Remarks:** Create BSpline from ICurve3D

**Parameters:**
- `curve_object` (object) — Input curve

**Returns:** `BSpline3D` — Created BSpline3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.Create)

#### `CreateArc3D(arc)`

Create BSpline form Arc3D

**Remarks:** Create BSpline form Arc3D

**Parameters:**
- `arc` (Arc3D) — Input arc

**Returns:** `BSpline3D` — Created Bspline3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateArc3D)

#### `CreateBSpline(points, degree, isPeriodic)`

Create BSpline from control points

**Remarks:** Create BSpline from control points

**Parameters:**
- `points` (Point3DList) — Control points
- `degree` (int) — Degree of BSpline
- `isPeriodic` (bool) — true if BSpline should be smoothly closed

**Returns:** `BSpline3D` — BSpline object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateBSpline)

#### `CreateBSpline3DFrom2DCurves(directionCurve_object, elevationCurve_object, startHeight, chainPoints)`

Creates BSpline3D from direction 2D curve and elevation 2D curve

**Remarks:** Creates BSpline3D from direction 2D curve and elevation 2D curve

**Parameters:**
- `directionCurve_object` (object) — direction curve ( for X,Y coordinates)
- `elevationCurve_object` (object) — elevation curve (for Z coordinates)
- `startHeight` (float) — Z coordinate of new curve start point
- `chainPoints` (Point3DList) — list of chain points to match resulting curve

**Returns:** `eGeometryErrorCode` — tuple(eOK if successful,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateBSpline3DFrom2DCurves)

#### `CreateBSpline3DFromAxisAndGradient(directionCurve_object, elevationCurve_object, startHeight, chainPoints)`

Create BSpline3D curve from axis and gradient (all placed in XY plane)

**Remarks:** Create BSpline3D curve from axis and gradient (all placed in XY plane)

**Parameters:**
- `directionCurve_object` (object) — direction/axis curve
- `elevationCurve_object` (object) — elevation/gradient curve
- `startHeight` (float) — Z coordinate of new curve start point
- `chainPoints` (Point3DList) — list of chain points to match resulting curve

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateBSpline3DFromAxisAndGradient)

#### `CreateBSplineInterpolated(points, degree, isPeriodic)`

Create BSpline from interpolated points

**Remarks:** Create BSpline from interpolated points

**Parameters:**
- `points` (Point3DList) — Interpolated points
- `degree` (int) — Degree of BSpline
- `isPeriodic` (bool) — true if BSpline should be smoothly closed

**Returns:** `BSpline3D` — BSpline object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateBSplineInterpolated)

#### `CreateBSplineJoined(curves_object)`

Create BSpline by joining curves. Curves should meet by end points.

**Remarks:** Create BSpline by joining curves. Curves should meet by end points.

**Parameters:**
- `curves_object` (list) — Source curves

**Returns:** `BSpline3D` — BSpline object (empty, if joining failed)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateBSplineJoined)

#### `CreateLine3D(line)`

Create BSpline3D from Line3D

**Remarks:** Create BSpline3D from Line3D

**Parameters:**
- `line` (Line3D) — Input line.

**Returns:** `BSpline3D` — Created BSpline3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateLine3D)

#### `CreatePolyline3D(polyline3d)`

Create a BSpline out of a Polyline3D object

**Remarks:** Create a BSpline out of a Polyline3D object

**Parameters:**
- `polyline3d` (Polyline3D) — Polyline3D that will be used for conversion

**Returns:** `BSpline3D` — An instance of the created BSpline3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreatePolyline3D)

#### `CreateSpline(spline)`

Create BSpline from Spline3D

**Remarks:** Create BSpline from Spline3D

**Parameters:**
- `spline` (Spline3D) — Spline

**Returns:** `BSpline3D` — Created BSpline3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.CreateSpline)

#### `EvaluateEndPoint()`

Evaluate end point of bspline

**Remarks:** Evaluate end point of bspline

**Returns:** `tuple[eGeometryErrorCode, Point3D]` — evaluated end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.EvaluateEndPoint)

#### `EvaluateEndRelPoint()`

Evaluate relative end (last) point of bspline

**Remarks:** Evaluate relative end (last) point of bspline

**Returns:** `tuple[eGeometryErrorCode, Point3D]` — evaluated relative end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.EvaluateEndRelPoint)

#### `EvaluatePoint(param)`

Evaluate point of b-spline

**Remarks:** Evaluate point of b-spline

**Parameters:**
- `param` (float) — parameter to evaluate point on b-spline

**Returns:** `eGeometryErrorCode` — tuple(error code of evaluation,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.EvaluatePoint)

#### `EvaluatePointsWithTangents(parameters)`

Evaluate points and tangents of b-spline

**Remarks:** Evaluate points and tangents of b-spline

**Parameters:**
- `parameters` (VecDoubleList) — parameters to evaluate points and tangents on b-spline

**Returns:** `tuple[Point3D, Vector3D]` — vector of resulting points and tangents, if calculation failed for any parameter, empty vector is returned

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.EvaluatePointsWithTangents)

#### `EvaluateStartPoint()`

Evaluate start point of bspline

**Remarks:** Evaluate start point of bspline

**Returns:** `tuple[eGeometryErrorCode, Point3D]` — evaluated start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.EvaluateStartPoint)

#### `EvaluateStartRelPoint()`

Evaluate relative start point of bspline

**Remarks:** Evaluate relative start point of bspline

**Returns:** `tuple[eGeometryErrorCode, Point3D]` — evaluated relative start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.EvaluateStartRelPoint)

#### `Get()`

Return type: tuple(Point3DList, VecDoubleList, VecDoubleList, int, bool)

**Remarks:** Return type: tuple(Point3DList, VecDoubleList, VecDoubleList, int, bool)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.Get)

#### `GetDegree()`

Gets spline degree

**Remarks:** Gets spline degree

**Returns:** `int` — spline degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetDegree)

#### `GetInterpolatedPoints()`

Function returns interpolated points of b-spline

**Remarks:** Function returns interpolated points of b-spline

**Returns:** `Point3DList` — list of interpolated points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetInterpolatedPoints)

#### `GetInterpolatedPointsParameters()`

Function returns list of parameters for interpolated points of b-spline

**Remarks:** Function returns list of parameters for interpolated points of b-spline

**Returns:** `VecDoubleList` — list of parameters of interpolated points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetInterpolatedPointsParameters)

#### `GetInterval()`

Function returns the interval of bspline

**Remarks:** Function returns the interval of bspline

**Returns:** `bool` — tuple(true if success,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetInterval)

#### `GetKnotMultiplicities()`

Get knot vector and knot multiplicities

**Remarks:** Get knot vector and knot multiplicities

**Returns:** `VecDoubleList` — tuple(knot vector with unique values,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetKnotMultiplicities)

#### `GetKnots()`

Get knot vector

**Remarks:** Get knot vector

**Returns:** `VecDoubleList` — knot vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetKnots)

#### `GetParametersForDistances(distances)`

Find parameters for distances on b-spline

**Remarks:** Find parameters for distances on b-spline

**Parameters:**
- `distances` (VecDoubleList) — list of distances to calculate parameters for

**Returns:** `VecDoubleList` — list of parameters, if calculation failed for any distance, empty vector is returned

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetParametersForDistances)

#### `GetWeights()`

Get control points weights

**Remarks:** Get control points weights

**Returns:** `VecDoubleList` — weights vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.GetWeights)

#### `IsClosed()`

Check if spline is closed ( first/last points are equal )

**Remarks:** Check if spline is closed ( first/last points are equal )

**Returns:** `bool` — closed spline true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.IsClosed)

#### `IsEndClamped()`

Function returns whether the bspline is clamped at the end

**Remarks:** Function returns whether the bspline is clamped at the end

**Returns:** `bool` — true if clamped on end

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.IsEndClamped)

#### `IsLine()`

Check if bspline is line

**Remarks:** Check if bspline is line

**Returns:** `bool` — Returns true if it is line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.IsLine)

#### `IsPeriodicClosed()`

Returns whether the spline is periodic or closed in 1st degree BSpline with degree == 1 cannot be periodic, wherefore returns true if closed.

**Remarks:** Returns whether the spline is periodic or closed in 1st degree BSpline with degree == 1 cannot be periodic, wherefore returns true if closed.

**Returns:** `bool` — bool true = periodic or closed

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.IsPeriodicClosed)

#### `IsRational()`

Check if the spline is rational

**Remarks:** Check if the spline is rational

**Returns:** `bool` — bool true = rational

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.IsRational)

#### `IsStartClamped()`

Function returns whether the bspline is clamped at the start

**Remarks:** Function returns whether the bspline is clamped at the start

**Returns:** `bool` — true if clamped on start

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.IsStartClamped)

#### `IsValid()`

Check spline validity

**Remarks:** Check spline validity

**Returns:** `bool` — bool valid = true

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.IsValid)

#### `Reverse()`

Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

**Remarks:** Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.Reverse)

#### `Set(points, weights, knots, degree, isPeriodic=False)`

Get/Set functions

**Remarks:** Get/Set functions

**Parameters:**
- `points` (Point3DList)
- `weights` (VecDoubleList)
- `knots` (VecDoubleList)
- `degree` (int)
- `isPeriodic` (bool)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.Set)

#### `SetDegree(degree)`

Set spline degree

**Remarks:** Set spline degree

**Parameters:**
- `degree` (int) — desired degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.SetDegree)

#### `SetKnots(knots)`

Set knot vector

**Remarks:** Set knot vector

**Parameters:**
- `knots` (VecDoubleList) — knot vector to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.SetKnots)

#### `SetPeriodic(periodic)`

Set periodic flag

**Remarks:** Set periodic flag

**Parameters:**
- `periodic` (bool) — value of periodic flag

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.SetPeriodic)

#### `SetWeights(weights)`

Set weights for control points

**Remarks:** Set weights for control points

**Parameters:**
- `weights` (VecDoubleList) — weights vector to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.SetWeights)

#### `__eq__(bspline)`

Comparison of bsplines without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of bsplines without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `bspline` (BSpline3D) — Compared bspline.

**Returns:** `object` — True when bsplines are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3D/#NemAll_Python_Geometry.BSpline3D.__repr__)

### Properties
- `Degree` (int, get/set) — Gets spline degree
- `IsPeriodic` (bool, get/set) — Returns whether the spline is periodic Returns only given flag which was originally set
- `Knots` (NemAll_Python_Utility.VecDoubleList, get/set) — Get knot vector
- `Weights` (NemAll_Python_Utility.VecDoubleList, get/set) — Get control points weights

## BSpline3DList (class)

List for BSpline3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/)

### Constructors
- `BSpline3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (BSpline3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (BSpline3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `BSpline3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (BSpline3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BSpline3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (BSpline3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DList/#NemAll_Python_Geometry.BSpline3DList.extend)

## BSpline3DService (class)

3D BSpline service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/)

### Constructors
- `BSpline3DService(bSpline)` — Constructor

### Methods
#### `AddControlPoint(pointIdx, coords)`

add control point to B-Spline

**Remarks:** add control point to B-Spline

**Parameters:**
- `pointIdx` (int) — index, where new point will be stored
- `coords` (Point3D) — coordinates of point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.AddControlPoint)

#### `AddControlPointOnSegment(ray, coords)`

add control point to B-Spline's segment given by point

**Remarks:** add control point to B-Spline's segment given by point

**Parameters:**
- `ray` (Vector3D) — view vector
- `coords` (Point3D) — coordinates of point

**Returns:** `tuple` — segment index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.AddControlPointOnSegment)

#### `GetControlPointIndex(param)`

calculate control point index from parameter on B-Spline

**Remarks:** calculate control point index from parameter on B-Spline

**Parameters:**
- `param` (float) — parameter on BSpline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.GetControlPointIndex)

#### `InsertKnot(param, numInsertionsMultiplicity)`

Insert knot into bspline knot vector (compute new control points, preserve geometry)

**Remarks:** Insert knot into bspline knot vector (compute new control points, preserve geometry)

**Parameters:**
- `param` (float) — param to insert
- `numInsertionsMultiplicity` (int) — multiplicity of new knot

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.InsertKnot)

#### `IsValid()`

Check validity of service data

**Remarks:** Check validity of service data

**Returns:** `bool` — bool valid = true

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.IsValid)

#### `MoveStartPeriodic(startParam)`

Move start point in periodic BSpline

**Remarks:** Move start point in periodic BSpline

**Parameters:**
- `startParam` (float) — start value for knot interval

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.MoveStartPeriodic)

#### `PointModification(pointsIdx, moveVector, isInterpolated, hSet)`

move interpolated or control point of B-Spline

**Remarks:** move interpolated or control point of B-Spline

**Parameters:**
- `pointsIdx` (VecSizeTList) — point indexes
- `moveVector` (Vector3D) — move vector
- `isInterpolated` (bool) — true if wanted to handle interpolated points
- `hSet` (HealingSettings) — healing settings

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.PointModification)

#### `RefineKnots(knotvalues, knotMultiplicities)`

Refine knots (insert knots if necessary)

**Remarks:** Refine knots (insert knots if necessary)

**Parameters:**
- `knotvalues` (VecDoubleList) — knot values to refine
- `knotMultiplicities` (VecSizeTList) — knot multiplicities

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.RefineKnots)

#### `RemoveControlPoint(pointIdx)`

remove control point from B-Spline

**Remarks:** remove control point from B-Spline

**Parameters:**
- `pointIdx` (int) — point index to remove

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.RemoveControlPoint)

#### `SetControlPoint(pointIdx, newCoords)`

set coordinates of control point of B-Spline

**Remarks:** set coordinates of control point of B-Spline

**Parameters:**
- `pointIdx` (int) — point index
- `newCoords` (Point3D) — new coordinates of point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.SetControlPoint)

#### `SetDegree(degree)`

set degree of B-Spline

**Remarks:** set degree of B-Spline

**Parameters:**
- `degree` (int) — new degree for B-Spline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.SetDegree)

#### `SetInterpolatedPoint(pointIdx, newCoords)`

set coordinates of interpolated point of B-Spline

**Remarks:** set coordinates of interpolated point of B-Spline

**Parameters:**
- `pointIdx` (int) — point index
- `newCoords` (Point3D) — new coordinates of point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.SetInterpolatedPoint)

#### `SetPeriodic(periodic)`

set/unset periodic property of B-Spline

**Remarks:** set/unset periodic property of B-Spline

**Parameters:**
- `periodic` (bool) — it true, B-Spline will be periodic, otherwise B-Spline will be open

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSpline3DService/#NemAll_Python_Geometry.BSpline3DService.SetPeriodic)

## BSplineSurface3D (class)

3D bsplinesrfc class for 3D (non uniform, rational) B-spline surface geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/)

### Constructors
- `BSplineSurface3D() | BSplineSurface3D(points, weights, uknots, vknots, udegree, vdegree, isUPeriodic, isVPeriodic, isUClosed, isVClosed) | BSplineSurface3D(surface)` — Initialize

### Methods
#### `Clear()`

Clear data, getting invalid state

**Remarks:** Clear data, getting invalid state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.Clear)

#### `Get()`

Get all surface members

**Remarks:** Get all surface members

**Returns:** `tuple` — all control points,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.Get)

#### `GetCenterPoint()`

Get center point

**Remarks:** Get center point

**Returns:** `Point3D` — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetCenterPoint)

#### `GetNormalVector(uv)`

Evaluate normal vector for given parameters

**Remarks:** Evaluate normal vector for given parameters

**Parameters:**
- `uv` (Point2D) — uv paramaters

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetNormalVector)

#### `GetPoints()`

Get control points

**Remarks:** Get control points

**Returns:** `Point3DList` — all control points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetPoints)

#### `GetUDegree()`

Get surface u-degree

**Remarks:** Get surface u-degree

**Returns:** `int` — surface degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetUDegree)

#### `GetUKnots()`

Get u-knot vector

**Remarks:** Get u-knot vector

**Returns:** `VecDoubleList` — knot vector const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetUKnots)

#### `GetUPointsCount()`

Get control points count in u-direction

**Remarks:** Get control points count in u-direction

**Returns:** `int` — size_t control points count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetUPointsCount)

#### `GetVDegree()`

Get surface v-degree

**Remarks:** Get surface v-degree

**Returns:** `int` — surface degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetVDegree)

#### `GetVKnots()`

Get v-knot vector

**Remarks:** Get v-knot vector

**Returns:** `VecDoubleList` — knot vector const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetVKnots)

#### `GetVPointsCount()`

Get control points count in v-direction

**Remarks:** Get control points count in v-direction

**Returns:** `int` — size_t control points count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetVPointsCount)

#### `GetWeights()`

Get control points weights

**Remarks:** Get control points weights

**Returns:** `VecDoubleList` — weights vector const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.GetWeights)

#### `IsPlanar()`

Check if surface is planar

**Remarks:** Check if surface is planar

**Returns:** `bool` — bool true = yes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.IsPlanar)

#### `IsRational()`

Check if the surface is rational

**Remarks:** Check if the surface is rational

**Returns:** `bool` — bool true = rational

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.IsRational)

#### `IsUClosed()`

Check if the surface is closed in U direction

**Remarks:** Check if the surface is closed in U direction

**Returns:** `bool` — bool true = closed

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.IsUClosed)

#### `IsUPeriodic()`

Check if the surface is periodic in U direction

**Remarks:** Check if the surface is periodic in U direction

**Returns:** `bool` — bool true = periodic

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.IsUPeriodic)

#### `IsVClosed()`

Check if the surface is closed in V direction

**Remarks:** Check if the surface is closed in V direction

**Returns:** `bool` — bool true = closed

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.IsVClosed)

#### `IsVPeriodic()`

Check if the surface is periodic in V direction

**Remarks:** Check if the surface is periodic in V direction

**Returns:** `bool` — bool true = periodic

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.IsVPeriodic)

#### `IsValid()`

Check surface validity

**Remarks:** Check surface validity

**Returns:** `bool` — bool valid = true

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.IsValid)

#### `Set(points, weights, uknots, vknots, udegree, vdegree, isUPeriodic, isVPeriodic, isUClosed, isVClosed)`

Set all surface members

**Remarks:** Set all surface members

**Parameters:**
- `points` (Point3DList) — all control points
- `weights` (VecDoubleList) — all control points weights
- `uknots` (VecDoubleList) — knots in u direction
- `vknots` (VecDoubleList) — knots in v direction
- `udegree` (int) — degree in u direction
- `vdegree` (int) — degree in v direction
- `isUPeriodic` (bool) — periodic in u direction
- `isVPeriodic` (bool) — periodic in v direction
- `isUClosed` (bool) — closed in u direction
- `isVClosed` (bool) — closed in v direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.Set)

#### `SetUDegree(degree)`

Set surface v-degree

**Remarks:** Set surface v-degree

**Parameters:**
- `degree` (int) — desired degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.SetUDegree)

#### `SetUKnots(knots)`

Set u-knot vector

**Remarks:** Set u-knot vector

**Parameters:**
- `knots` (VecDoubleList) — knot vector to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.SetUKnots)

#### `SetVDegree(degree)`

Set surface v-degree

**Remarks:** Set surface v-degree

**Parameters:**
- `degree` (int) — desired degree

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.SetVDegree)

#### `SetVKnots(knots)`

Set v-knot vector

**Remarks:** Set v-knot vector

**Parameters:**
- `knots` (VecDoubleList) — knot vector to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.SetVKnots)

#### `SetWeights(weights)`

Set weights for control points

**Remarks:** Set weights for control points

**Parameters:**
- `weights` (VecDoubleList) — weights vector to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.SetWeights)

#### `__eq__(bsplinesrfc)`

Comparison of bsplinesrfcs without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of bsplinesrfcs without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `bsplinesrfc` (BSplineSurface3D) — Compared bsplinesrfc.

**Returns:** `object` — True when bsplinesrfcs are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3D/#NemAll_Python_Geometry.BSplineSurface3D.__repr__)

### Properties
- `Points` (None, get) — Get the points property :type: None
- `UClosed` (None, get) — Get and set the closed property :type: None
- `UDegree` (None, get) — Get and set the degree property :type: None
- `UKnots` (None, get) — Get and set the uknots property :type: None
- `UPeriodic` (None, get) — Get and set the periodic property :type: None
- `VClosed` (None, get) — Get and set the closed property :type: None
- `VDegree` (None, get) — Get and set the degree property :type: None
- `VKnots` (None, get) — Get and set the vknots property :type: None
- `VPeriodic` (None, get) — Get and set the periodic property :type: None
- `Weights` (None, get) — Get and set the weights property :type: None

## BSplineSurface3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.BSplineSurface3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/)

### Constructors
- `BSplineSurface3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (BSplineSurface3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (BSplineSurface3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `BSplineSurface3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (BSplineSurface3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BSplineSurface3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (BSplineSurface3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BSplineSurface3DList/#NemAll_Python_Geometry.BSplineSurface3DList.extend)

## BoundingBox2D (class)

2D boundingBox Representation class for 2D bounding box.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/)

### Constructors
- `BoundingBox2D() | BoundingBox2D(angle) | BoundingBox2D(min, max, angle) | BoundingBox2D(boundingBox2D) | BoundingBox2D(minMax2D)` — Initialize

### Methods
#### `Deflate(x, y) | Deflate(size)`

Deflate in x and y axis

**Remarks:** Deflate in x and y axis

**Parameters:**
- `x` (float) — deflate in X axis
- `y` (float) — deflate in Y axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.Deflate)

#### `Get()`

Get minimum point, maximum point and angle

**Remarks:** Get minimum point, maximum point and angle

**Returns:** `tuple` — minimum point in local coordinate system,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.Get)

#### `GetAngle()`

Get direction of X axis of new coordinate system

**Remarks:** Get direction of X axis of new coordinate system

**Returns:** `Angle` — angle direction of X axis of new coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetAngle)

#### `GetBoxPoint(arg2)`

Get world point of a dedicated location of the box

**Remarks:** Get world point of a dedicated location of the box

**Returns:** `Point2D` — Point2D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetBoxPoint)

#### `GetCenter()`

Get box center point in local coordinate system

**Remarks:** Get box center point in local coordinate system

**Returns:** `Point2D` — center point in local coordinates system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetCenter)

#### `GetCenterPoint()`

Get world point at center of box

**Remarks:** Get world point at center of box

**Returns:** `Point2D` — Point2D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetCenterPoint)

#### `GetHeight()`

Get height of box in local system

**Remarks:** Get height of box in local system

**Returns:** `float` — double

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetHeight)

#### `GetMax()`

Get maximum point in local coordinate system

**Remarks:** Get maximum point in local coordinate system

**Returns:** `Point2D` — maximum point in local coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetMax)

#### `GetMaxPoint()`

Get world point at maximum of box

**Remarks:** Get world point at maximum of box

**Returns:** `Point2D` — Point2D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetMaxPoint)

#### `GetMin()`

Get minimum point in local coordinate system

**Remarks:** Get minimum point in local coordinate system

**Returns:** `Point2D` — minimum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetMin)

#### `GetMinPoint()`

Get world point at maximum of box

**Remarks:** Get world point at maximum of box

**Returns:** `Point2D` — Point2D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetMinPoint)

#### `GetWidth()`

Get width of box in local system

**Remarks:** Get width of box in local system

**Returns:** `float` — double

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.GetWidth)

#### `Inflate(x, y) | Inflate(size)`

Inflate in x and y axis

**Remarks:** Inflate in x and y axis

**Parameters:**
- `x` (float) — inflate in X axis
- `y` (float) — inflate in Y axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.Inflate)

#### `IsContaining(point) | IsContaining(box) | IsContaining(minmax)`

Is point inside this box

**Remarks:** Is point inside this box

**Parameters:**
- `point` (Point2D) — point in world coordinate system

**Returns:** `bool` — true, if is inside, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.IsContaining)

#### `IsValid()`

Test if box is valid

**Remarks:** Test if box is valid

**Returns:** `bool` — true, if box valid, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.IsValid)

#### `Overlaps(box) | Overlaps(minmax)`

Does box overlap this box

**Remarks:** Does box overlap this box

**Parameters:**
- `box` (BoundingBox2D) — bounding box

**Returns:** `bool` — true, if boxes overlap, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.Overlaps)

#### `Reset(angle)`

Reset bounding box and set initial angle When bounding box is reseted, then min point is initialized to [DBL_MAX,DBL_MAX], max point to [-DBL_MAX,-DBL_MAX], This method is using when previous initialized bounding box start accept a new point values.

**Remarks:** Reset bounding box and set initial angle When bounding box is reseted, then min point is initialized to [DBL_MAX,DBL_MAX], max point to [-DBL_MAX,-DBL_MAX], This method is using when previous initialized bounding box start accept a new point values.

**Parameters:**
- `angle` (Angle) — Initial angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.Reset)

#### `Set(min, max, angle)`

Set minimum point, maximum point and angle

**Remarks:** Set minimum point, maximum point and angle

**Parameters:**
- `min` (Point2D) — minimum point in local coordinate system
- `max` (Point2D) — maximum point in local coordinate system
- `angle` (Angle) — angle between world X and local X coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.Set)

#### `SetAngle(angle)`

Set direction of X axis of new coordinate system

**Remarks:** Set direction of X axis of new coordinate system

**Parameters:**
- `angle` (Angle) — direction of X axis of new coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.SetAngle)

#### `SetHeight(height)`

Set height of box in local system. Computed from Min point

**Remarks:** Set height of box in local system. Computed from Min point

**Parameters:**
- `height` (float) — Height

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.SetHeight)

#### `SetMax(max)`

Set maximum point in local coordinate system

**Remarks:** Set maximum point in local coordinate system

**Parameters:**
- `max` (Point2D) — maximum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.SetMax)

#### `SetMin(min)`

Set minimum point

**Remarks:** Set minimum point

**Parameters:**
- `min` (Point2D) — minimum point in local coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.SetMin)

#### `SetWidth(width)`

Set width of box in local system. Computed from Min point

**Remarks:** Set width of box in local system. Computed from Min point

**Parameters:**
- `width` (float) — Width
- `-----------------------------------------------------------------------------` (object)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.SetWidth)

#### `__add__(boundingBox) | __add__(minMax)`

Expand BoundingBox2D box. Expands the BoundingBox2D box by the box given in parameter minmax

**Remarks:** Expand BoundingBox2D box. Expands the BoundingBox2D box by the box given in parameter minmax

**Parameters:**
- `boundingBox` (BoundingBox2D) — BoundingBox2D to be added

**Returns:** `object` — BoundingBox2D box.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.__add__)

#### `__eq__(boundingBox)`

Comparison of boundingBoxs without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of boundingBoxs without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `boundingBox` (BoundingBox2D) — Compared boundingBox.

**Returns:** `object` — True when boundingBoxs are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.__eq__)

#### `__getitem__(index)`

Get the corners of bounding box. This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get the corners of bounding box. This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — corner index (0-left bottom, 1-right bottom, 2-right top, 3-left top)

**Returns:** `Point2D` — corner as Point2D in world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.__getitem__)

#### `__iadd__(boundingBox) | __iadd__(minmax) | __iadd__(point)`

Expand BoundingBox2D box. Expands the BoundingBox2D box by the box given in parameter minmax

**Remarks:** Expand BoundingBox2D box. Expands the BoundingBox2D box by the box given in parameter minmax

**Parameters:**
- `boundingBox` (BoundingBox2D) — BoundingBox2D to be added

**Returns:** `object` — reference to BoundingBox2D box.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.__iadd__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2D/#NemAll_Python_Geometry.BoundingBox2D.__repr__)

### Properties
- `Angle` (None, get) — Get and set the angle property :type: None
- `Max` (None, get) — Get and set the max point property :type: None
- `Min` (None, get) — Get and set the min point property :type: None

## BoundingBox2DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.BoundingBox2DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/)

### Constructors
- `BoundingBox2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (BoundingBox2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (BoundingBox2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `BoundingBox2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (BoundingBox2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BoundingBox2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (BoundingBox2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/BoundingBox2DList/#NemAll_Python_Geometry.BoundingBox2DList.extend)

## CenterCalculus (class)

Class to calculate the center of an object

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/CenterCalculus/)

### Methods
#### `Calculate(line) | Calculate(line) | Calculate(polyline, edge) | Calculate(polyline, edge) | Calculate(polygon, bPlaneCenter, edge) | Calculate(polygon, bPlaneCenter, edge) | Calculate(arc, center) | Calculate(arc, center) | Calculate(spline, eps) | Calculate(spline, eps) | Calculate(spline, eps, bAreaCenter) | Calculate(spline, eps) | Calculate(clothoid, eps) | Calculate(path, eps, bAreaCenter) | Calculate(path, eps, bAreaCenter) | Calculate(geoObject, eps, bArcCenter, edge)`

Calculates the center of a Line2D

**Remarks:** Calculates the center of a Line2D

**Parameters:**
- `line` (Line2D) — Line2D on which to calculate the center

**Returns:** `tuple` — center calculated,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/CenterCalculus/#NemAll_Python_Geometry.CenterCalculus.Calculate)

## ChamferCalculus (class)

Class for chamfer calculation between two objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/)

### Methods
#### `Calculate(polyhedron, chamferWidth) | Calculate(polyhedron, edges, chamferWidth, propagation) | Calculate(brep, chamferWidth) | Calculate(brep, edges, chamferWidth, propagation)`

Calculate chamfer on all edges of given Polyhedron3D

**Remarks:** Calculate chamfer on all edges of given Polyhedron3D

**Parameters:**
- `polyhedron` (Polyhedron3D) — polyhedron to chamfer
- `chamferWidth` (float) — chamfer width

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/#NemAll_Python_Geometry.ChamferCalculus.Calculate)

#### `CalculateApplicableChamfers(line1, line2, intersectionPoint, chamferWidth) | CalculateApplicableChamfers(igeo1, igeo2, plane3D, intersectionPoint, chamferWidth) | CalculateApplicableChamfers(line1, line2, plane3D, intersectionPoint, chamferWidth) | CalculateApplicableChamfers(line, arc, chamferArc, bFirstElementIsLine) | CalculateApplicableChamfers(arc1, arc2, chamferArc)`

Calculates four applicable chamfer lines

**Remarks:** Calculates four applicable chamfer lines

**Parameters:**
- `line1` (Line2D) — Geometry of first chamfered line
- `line2` (Line2D) — Geometry of second chamfered line
- `intersectionPoint` (Point2D) — Intersection point of 2 chamfered lines
- `chamferWidth` (float) — Chamfer width

**Returns:** `Line2DList` — Calculated 4 applicable chamfer lines

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/#NemAll_Python_Geometry.ChamferCalculus.CalculateApplicableChamfers)

#### `CalculateChamferInsideOnePolyline(originalChamferLine, originalPolyLine, polySegment1, polySegment2)`

Calculate chamfer inside one polyline - 2 different segments of a polyline are to be chamfered

**Remarks:** Calculate chamfer inside one polyline - 2 different segments of a polyline are to be chamfered

**Parameters:**
- `originalChamferLine` (Line3D) — Geometry of chamfer line
- `originalPolyLine` (Polyline3D) — Geometry of chamfered polyline
- `polySegment1` (int) — First segment index of polyline
- `polySegment2` (int) — Second segment index of polyline

**Returns:** `Polyline3D` — Calculated chamfered polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/#NemAll_Python_Geometry.ChamferCalculus.CalculateChamferInsideOnePolyline)

#### `CalculateChamferLine(line1, line2, inputPoint) | CalculateChamferLine(igeo1, igeo2, plane3D, intersectionPoint, inputPoint) | CalculateChamferLine(line1, line2, plane3D, intersectionPoint, inputPoint) | CalculateChamferLine(line, arc, plane3D, intersectionPoint, inputPoint, bFirstElementIsLine) | CalculateChamferLine(arc1, arc2, plane3D, intersectionPoint, inputPoint)`

Calculates chamfer line

**Remarks:** Calculates chamfer line

**Parameters:**
- `line1` (Line2D) — Geometry of first chamfered line
- `line2` (Line2D) — Geometry of second chamfered line
- `inputPoint` (Point2D) — Point of click (through which the chamfer should run)

**Returns:** `Line2D` — Calculated chamfer line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/#NemAll_Python_Geometry.ChamferCalculus.CalculateChamferLine)

#### `CalculateChamferedLine(originalLine, intersectionPoint, chamferPoint) | CalculateChamferedLine(originalLine, plane3D, intersectionPoint, chamferPoint)`

Calculates chamfered line

**Remarks:** Calculates chamfered line

**Parameters:**
- `originalLine` (Line2D) — Line which is being chamfered
- `intersectionPoint` (Point2D) — Intersection point of 2 chamfered lines
- `chamferPoint` (Point2D) — Intersection point of chamfer line and (original) chamfered line

**Returns:** `Line2D` — Calculated chamfered line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/#NemAll_Python_Geometry.ChamferCalculus.CalculateChamferedLine)

#### `CalculateHalvingAngle(line1, line2, intersectionPoint)`

Calculates halving angle of 2 given lines in the global coordinate system

**Remarks:** Calculates halving angle of 2 given lines in the global coordinate system

**Parameters:**
- `line1` (Line2D) — Geometry of first line
- `line2` (Line2D) — Geometry of second line
- `intersectionPoint` (Point2D) — Intersection point of the 2 lines

**Returns:** `Angle` — Halving angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/#NemAll_Python_Geometry.ChamferCalculus.CalculateHalvingAngle)

#### `SwapLinePoints(line, point) | SwapLinePoints(line, point)`

Changes line orientation if given point lies on line's left side.

**Remarks:** Changes line orientation if given point lies on line's left side.

**Parameters:**
- `line` (Line2D) — line to be changed
- `point` (Point2D) — point to be evaluated

**Returns:** `tuple` — True if line orientation was changed, false if orientation was left intact,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ChamferCalculus/#NemAll_Python_Geometry.ChamferCalculus.SwapLinePoints)

## ClippedSweptSolid3D (class)

Representation class for solid created by extrusion of area with borders of a plane at the bottom and a plane at the top in world z - axis direction

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/)

### Constructors
- `ClippedSweptSolid3D() | ClippedSweptSolid3D(area, bottom, top) | ClippedSweptSolid3D(solid) | ClippedSweptSolid3D(refPoint, solid)` — Initialize

### Methods
#### `GetBottomPlane()`

Get bottom clipping plane

**Remarks:** Get bottom clipping plane

**Returns:** `Plane3D` — Plane3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.GetBottomPlane)

#### `GetRefPoint()`

Get the reference point

**Remarks:** Get the reference point

**Returns:** `Point3D` — Reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.GetRefPoint)

#### `GetSweptArea()`

Get Swept area

**Remarks:** Get Swept area

**Returns:** `PolygonalArea2D` — PolygonalArea2D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.GetSweptArea)

#### `GetTopPlane()`

Get top clipping plane

**Remarks:** Get top clipping plane

**Returns:** `Plane3D` — Plane3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.GetTopPlane)

#### `IsValid()`

Check if the Solid is valid

**Remarks:** Check if the Solid is valid

**Returns:** `bool` — True if it is a valid solid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.IsValid)

#### `SetBottomPlane(plane)`

Set bottom clipping plane

**Remarks:** Set bottom clipping plane

**Parameters:**
- `plane` (Plane3D) — Plane3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.SetBottomPlane)

#### `SetRefPoint(refPoint)`

Set reference point

**Remarks:** Set reference point

**Parameters:**
- `refPoint` (Point3D) — New reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.SetRefPoint)

#### `SetSweptArea(area)`

Set Swept area

**Remarks:** Set Swept area

**Parameters:**
- `area` (PolygonalArea2D) — PolygonalArea2D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.SetSweptArea)

#### `SetTopPlane(plane)`

Set top clipping plane

**Remarks:** Set top clipping plane

**Parameters:**
- `plane` (Plane3D) — Plane3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.SetTopPlane)

#### `__eq__(clippedSweptSolid)`

Comparison of clippedSweptSolids without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of clippedSweptSolids without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `clippedSweptSolid` (ClippedSweptSolid3D) — Compared clippedSweptSolid.

**Returns:** `object` — True when clippedSweptSolids are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.__eq__)

#### `__mul__(matrix)`

Matrix transformation

**Remarks:** Matrix transformation

**Parameters:**
- `matrix` (Matrix3D) — Transformation 3D matrix

**Returns:** `object` — Transformed ClippedSweptSolid3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.__mul__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3D/#NemAll_Python_Geometry.ClippedSweptSolid3D.__repr__)

### Properties
- `BottomPlane` (None, get) — Get and set the bottom plane as property :type: None
- `RefPoint` (None, get) — Get and set the reference point as property :type: None
- `SweptArea` (None, get) — Get and set the swept area as property :type: None
- `TopPlane` (None, get) — Get and set the top plane as property :type: None

## ClippedSweptSolid3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.ClippedSweptSolid3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/)

### Constructors
- `ClippedSweptSolid3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ClippedSweptSolid3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ClippedSweptSolid3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ClippedSweptSolid3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ClippedSweptSolid3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ClippedSweptSolid3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ClippedSweptSolid3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClippedSweptSolid3DList/#NemAll_Python_Geometry.ClippedSweptSolid3DList.extend)

## ClosedArea2D (class)

2D closed area Representation class for 2D geometry closed (path bounded) area

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/)

### Constructors
- `ClosedArea2D() | ClosedArea2D(area)` — Initialize

### Methods
#### `AddInnerCurve(innerpath)`

Add new inner curve

**Remarks:** Add new inner curve

**Parameters:**
- `innerpath` (Path2D) — New inner curve ( Path2D )

**Returns:** `bool` — true if the operation was successful

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.AddInnerCurve)

#### `Clear()`

Clear all the components of this Area

**Remarks:** Clear all the components of this Area

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.Clear)

#### `GetInnerCurve(index)`

Get the curve of given index

**Remarks:** Get the curve of given index

**Parameters:**
- `index` (int) — Index of the inner curve

**Returns:** `Path2D` — const reference to the curve ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.GetInnerCurve)

#### `GetInnerList()`

Get the list of the inner curves

**Remarks:** Get the list of the inner curves

**Returns:** `Path2DList` — reference to a vector that contains the inner curves ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.GetInnerList)

#### `GetOuterCurve()`

Get the outer curve of this area

**Remarks:** Get the outer curve of this area

**Returns:** `Path2D` — const reference to outer curve ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.GetOuterCurve)

#### `InnerCount()`

Get the count of inner curves

**Remarks:** Get the count of inner curves

**Returns:** `int` — count of inner curves

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.InnerCount)

#### `IsValid()`

Set Check the validity

**Remarks:** Set Check the validity

**Returns:** `bool` — true if this Area is valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.IsValid)

#### `SetOuterCurve(outerpath)`

Set new Outer curve ( bounds )

**Remarks:** Set new Outer curve ( bounds )

**Parameters:**
- `outerpath` (Path2D) — New outer curve ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.SetOuterCurve)

#### `__eq__(closedArea)`

Comparison of closedAreas without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of closedAreas without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `closedArea` (ClosedArea2D) — Compared closedArea.

**Returns:** `object` — True when closedAreas are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2D/#NemAll_Python_Geometry.ClosedArea2D.__repr__)

### Properties
- `InnerList` (None, get) — Get the inner path list as property :type: None
- `OuterCurve` (None, get) — Get and set the outer path as property :type: None

## ClosedArea2DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.ClosedArea2DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/)

### Constructors
- `ClosedArea2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ClosedArea2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ClosedArea2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ClosedArea2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ClosedArea2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ClosedArea2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ClosedArea2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea2DList/#NemAll_Python_Geometry.ClosedArea2DList.extend)

## ClosedArea3D (class)

3D closed area Representation class for 3D geometry area

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/)

### Constructors
- `ClosedArea3D() | ClosedArea3D(closedArea)` — Initialize

### Methods
#### `AddInnerCurve(innerpath)`

Add new inner curve

**Remarks:** Add new inner curve

**Parameters:**
- `innerpath` (Path2D) — New inner curve ( Path2D )

**Returns:** `bool` — true if the operation was successful

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.AddInnerCurve)

#### `Clear()`

Clear all the components of this Profile

**Remarks:** Clear all the components of this Profile

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.Clear)

#### `GetInnerCurve(index)`

Get the curve of given index

**Remarks:** Get the curve of given index

**Parameters:**
- `index` (int) — Index of the inner curve

**Returns:** `Path2D` — const reference to the curve ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.GetInnerCurve)

#### `GetInnerList()`

Get the list of the inner curves

**Remarks:** Get the list of the inner curves

**Returns:** `Path2DList` — reference to a vector that contains the inner curves ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.GetInnerList)

#### `GetOuterCurve()`

Get the outer curve of this profile

**Remarks:** Get the outer curve of this profile

**Returns:** `Path2D` — const reference to outer curve ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.GetOuterCurve)

#### `GetRefPlacement()`

Get axis placement

**Remarks:** Get axis placement

**Returns:** `AxisPlacement3D` — const reference to the reference axis placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.GetRefPlacement)

#### `InnerCount()`

Get the count of inner curves

**Remarks:** Get the count of inner curves

**Returns:** `int` — count of inner curves

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.InnerCount)

#### `IsValid()`

Set Check the validity

**Remarks:** Set Check the validity

**Returns:** `bool` — true if this Profile is valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.IsValid)

#### `SetOuterCurve(outerpath)`

Set new Outer curve ( bounds )

**Remarks:** Set new Outer curve ( bounds )

**Parameters:**
- `outerpath` (Path2D) — New outer curve ( Path2D )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.SetOuterCurve)

#### `SetRefPlacement(placement)`

Set axis placement

**Remarks:** Set axis placement

**Parameters:**
- `placement` (AxisPlacement3D) — Reference axis placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.SetRefPlacement)

#### `__eq__(closedArea)`

Comparison of closedAreas without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of closedAreas without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `closedArea` (ClosedArea3D) — Compared closedArea.

**Returns:** `object` — True when closedAreas are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3D/#NemAll_Python_Geometry.ClosedArea3D.__repr__)

### Properties
- `RefPlacement` (None, get) — Get and set the reference placement 3d as property :type: None

## ClosedArea3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.ClosedArea3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/)

### Constructors
- `ClosedArea3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ClosedArea3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ClosedArea3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ClosedArea3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ClosedArea3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ClosedArea3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ClosedArea3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedArea3DList/#NemAll_Python_Geometry.ClosedArea3DList.extend)

## ClosedAreaComposite2D (class)

2D closed area composite Representation class for 2D geometry closed (path bounded) area composite

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/)

### Constructors
- `ClosedAreaComposite2D() | ClosedAreaComposite2D(composite)` — Initialize

### Methods
#### `Add(area)`

Add new area

**Remarks:** Add new area

**Parameters:**
- `area` (ClosedArea2D) — New area

**Returns:** `bool` — true if the operation was successful

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.Add)

#### `Clear()`

Clear contents of this composite

**Remarks:** Clear contents of this composite

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.Clear)

#### `GetProfile(index)`

Get profile from list of profiles

**Remarks:** Get profile from list of profiles

**Parameters:**
- `index` (int) — Index of profile in vector of profiles

**Returns:** `ClosedArea2D` — reference to profile

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.GetProfile)

#### `GetProfileCount()`

Get count of profiles (areas)

**Remarks:** Get count of profiles (areas)

**Returns:** `int` — size_t - count of profiles (areas)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.GetProfileCount)

#### `GetProfileList()`

Get const reference to vector of profiles

**Remarks:** Get const reference to vector of profiles

**Returns:** `ClosedArea2DList` — const reference to vector of profiles

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.GetProfileList)

#### `IsEmpty()`

Check if this composite has any contents ( areas )

**Remarks:** Check if this composite has any contents ( areas )

**Returns:** `bool` — true if it is empty

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.IsEmpty)

#### `__eq__(closedAreaComposite)`

Comparison of closedAreaComposites without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of closedAreaComposites without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `closedAreaComposite` (ClosedAreaComposite2D) — Compared closedAreaComposite.

**Returns:** `object` — True when closedAreaComposites are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2D/#NemAll_Python_Geometry.ClosedAreaComposite2D.__repr__)

### Properties
- `ProfileList` (None, get) — Get the profile list as property :type: None

## ClosedAreaComposite2DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.ClosedAreaComposite2DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/)

### Constructors
- `ClosedAreaComposite2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ClosedAreaComposite2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ClosedAreaComposite2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ClosedAreaComposite2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ClosedAreaComposite2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ClosedAreaComposite2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ClosedAreaComposite2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite2DList/#NemAll_Python_Geometry.ClosedAreaComposite2DList.extend)

## ClosedAreaComposite3D (class)

3D closed area composite Representation class for 3D geometry profile composite

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/)

### Constructors
- `ClosedAreaComposite3D() | ClosedAreaComposite3D(composite)` — Initialize

### Methods
#### `Add(profile)`

Add new profile

**Remarks:** Add new profile

**Parameters:**
- `profile` (ClosedArea2D) — New profile

**Returns:** `bool` — true if the operation was successful

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.Add)

#### `Clear()`

Clear contents of this composite

**Remarks:** Clear contents of this composite

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.Clear)

#### `GetClosedAreaComposite()`

Get constant reference to composite of ClosedArea2D

**Remarks:** Get constant reference to composite of ClosedArea2D

**Returns:** `ClosedAreaComposite2D` — ClosedAreaComposite2D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.GetClosedAreaComposite)

#### `GetRefPlacement()`

Get axis placement

**Remarks:** Get axis placement

**Returns:** `AxisPlacement3D` — const reference to the reference axis placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.GetRefPlacement)

#### `IsEmpty()`

Check if this composite has any contents ( profiles )

**Remarks:** Check if this composite has any contents ( profiles )

**Returns:** `bool` — true if it is empty

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.IsEmpty)

#### `SetRefPlacement(placement)`

Set axis placement

**Remarks:** Set axis placement

**Parameters:**
- `placement` (AxisPlacement3D) — Reference axis placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.SetRefPlacement)

#### `__eq__(closedAreaComposite)`

Comparison of closedAreaComposites without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of closedAreaComposites without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `closedAreaComposite` (ClosedAreaComposite3D) — Compared closedAreaComposite.

**Returns:** `object` — True when closedAreaComposites are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3D/#NemAll_Python_Geometry.ClosedAreaComposite3D.__repr__)

### Properties
- `ClosedAreaComposite` (None, get) — Get the 2d closed area composite as property :type: None
- `RefPlacement` (None, get) — Get and set the reference placement 3d as property :type: None

## ClosedAreaComposite3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.ClosedAreaComposite3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/)

### Constructors
- `ClosedAreaComposite3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ClosedAreaComposite3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ClosedAreaComposite3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ClosedAreaComposite3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ClosedAreaComposite3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ClosedAreaComposite3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ClosedAreaComposite3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ClosedAreaComposite3DList/#NemAll_Python_Geometry.ClosedAreaComposite3DList.extend)

## Clothoid2D (class)

2D clothoid Representation class for 2D clothoid.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/)

### Constructors
- `Clothoid2D() | Clothoid2D(clothoid)` — Initialize

### Methods
#### `GetEndCurvature()`

Get end curvature.

**Remarks:** Get end curvature.

**Returns:** `float` — double end curvature of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetEndCurvature)

#### `GetEndPoint()`

Get end point in world coordinate system.

**Remarks:** Get end point in world coordinate system.

**Returns:** `Point2D` — point Point2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetEndPoint)

#### `GetEndRelPoint()`

Get the end point in relative coordinate system

**Remarks:** Get the end point in relative coordinate system

**Returns:** `Point2D` — constant reference to Point2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetEndRelPoint)

#### `GetIsReversed()`

is clothoid reversed

**Remarks:** is clothoid reversed

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetIsReversed)

#### `GetLength()`

Get length of clothoid.

**Remarks:** Get length of clothoid.

**Returns:** `float` — double length of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetLength)

#### `GetParallel()`

Get parallel of clothoid.

**Remarks:** Get parallel of clothoid.

**Returns:** `float` — double parallel of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetParallel)

#### `GetRefPoint()`

Get reference point in world coordinate system

**Remarks:** Get reference point in world coordinate system

**Returns:** `Point2D` — reference to point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetRefPoint)

#### `GetStartCurvature()`

Get start curvature.

**Remarks:** Get start curvature.

**Returns:** `float` — double start curvature of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetStartCurvature)

#### `GetStartPoint()`

Get start point in world coordinate system.

**Remarks:** Get start point in world coordinate system.

**Returns:** `Point2D` — point Point2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetStartPoint)

#### `GetStartRelPoint()`

Get the start point in relative coordinate system

**Remarks:** Get the start point in relative coordinate system

**Returns:** `Point2D` — constant reference to Point2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetStartRelPoint)

#### `GetStartVector()`

Get start vector.

**Remarks:** Get start vector.

**Returns:** `Vector2D` — const reference to Vector2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetStartVector)

#### `GetType()`

Get type of clothoid.

**Remarks:** Get type of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.GetType)

#### `Reverse()`

Reverse orientation of the Clothoid

**Remarks:** Reverse orientation of the Clothoid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.Reverse)

#### `Set(clothoid)`

Initialize clothoid from clothoid.

**Remarks:** Initialize clothoid from clothoid.

**Parameters:**
- `clothoid` (Clothoid2D) — Clothoid2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.Set)

#### `SetEndCurvature(curvature)`

Set end curvature of clothoid.

**Remarks:** Set end curvature of clothoid.

**Parameters:**
- `curvature` (float) — end curvature of clothoid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetEndCurvature)

#### `SetEndPoint(point)`

Set end point in world coordinate system.

**Remarks:** Set end point in world coordinate system.

**Parameters:**
- `point` (Point2D) — const reference to Point2D in World coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetEndPoint)

#### `SetEndRelPoint(point)`

Set end point in local coordinate system

**Remarks:** Set end point in local coordinate system

**Parameters:**
- `point` (Point2D) — Point2D in local coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetEndRelPoint)

#### `SetLength(length)`

Set length of clothoid.

**Remarks:** Set length of clothoid.

**Parameters:**
- `length` (float) — length of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetLength)

#### `SetParallel(parallel)`

Set parallel of clothoid.

**Remarks:** Set parallel of clothoid.

**Parameters:**
- `parallel` (float) — length of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetParallel)

#### `SetRefPoint(refPoint)`

Set reference point in world coordinate system

**Remarks:** Set reference point in world coordinate system

**Parameters:**
- `refPoint` (Point2D) — const reference to Point2D in World coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetRefPoint)

#### `SetReversed(flag)`

Set orientation.

**Remarks:** Set orientation.

**Parameters:**
- `flag` (bool) — true for reversed, false normal orientation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetReversed)

#### `SetStartCurvature(curvature)`

Set start curvature of clothoid.

**Remarks:** Set start curvature of clothoid.

**Parameters:**
- `curvature` (float) — start curvature of clothoid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetStartCurvature)

#### `SetStartPoint(point)`

Set start point in world coordinate system.

**Remarks:** Set start point in world coordinate system.

**Parameters:**
- `point` (Point2D) — const reference to Point2D in World coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetStartPoint)

#### `SetStartRelPoint(point)`

Set start point in local coordinate system

**Remarks:** Set start point in local coordinate system

**Parameters:**
- `point` (Point2D) — Point2D in local coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetStartRelPoint)

#### `SetStartVector(vec)`

Set start vector.

**Remarks:** Set start vector.

**Parameters:**
- `vec` (Vector2D) — const reference to Vector2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetStartVector)

#### `SetType(type)`

Set type of clothoid.

**Remarks:** Set type of clothoid.

**Parameters:**
- `type` (eClothoidType) — type of clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.SetType)

#### `__eq__(clothoid)`

Comparison of clothoids without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of clothoids without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `clothoid` (Clothoid2D) — Compared clothoid.

**Returns:** `object` — True when clothoids are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.__eq__)

#### `__mul__(matrix)`

Matrix transformation.

**Remarks:** Matrix transformation.

**Parameters:**
- `matrix` (Matrix2D) — transformation matrix.

**Returns:** `object` — Clothoid2D transformed clothoid.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.__mul__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2D/#NemAll_Python_Geometry.Clothoid2D.__repr__)

### Properties
- `EndCurvature` (None, get) — Get and set the end curvature property :type: None
- `EndPoint` (None, get) — Get and set the end point property :type: None
- `IsReversed` (None, get) — Get and set the reversed property :type: None
- `Length` (None, get) — Get and set the length property :type: None
- `Parallel` (None, get) — Get and set the parallel property :type: None
- `RefPoint` (None, get) — Get and set the end point property :type: None
- `StartCurvature` (None, get) — Get and set the start curvature property :type: None
- `StartPoint` (None, get) — Get and set the start point property :type: None
- `StartVector` (None, get) — Get and set the start vector property :type: None
- `Type` (None, get) — Get and set the type property :type: None

## Clothoid2DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Clothoid2DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/)

### Constructors
- `Clothoid2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Clothoid2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Clothoid2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Clothoid2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Clothoid2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Clothoid2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Clothoid2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Clothoid2DList/#NemAll_Python_Geometry.Clothoid2DList.extend)

## Comparison (class)

Class for comparison between two objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/)

### Methods
#### `AllPointsAreOneAxis(polyline, tolerance)`

Check if all points of polyline are on one straight line

**Remarks:** Check if all points of polyline are on one straight line

**Parameters:**
- `polyline` (Polyline2D) — Polyline
- `tolerance` (float) — Tolerance

**Returns:** `bool` — Result of the check

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.AllPointsAreOneAxis)

#### `Congruent(l1p1, l1p2, l2p1, l2p2) | Congruent(l1, l2)`

compare 2D points of two 2D lines if are congruent

**Remarks:** compare 2D points of two 2D lines if are congruent

**Parameters:**
- `l1p1` (Point2D) — the 1. point of the 1. line
- `l1p2` (Point2D) — the 2. point of the 1. line
- `l2p1` (Point2D) — the 1. point of the 2. line
- `l2p2` (Point2D) — the 2. point of the 2. line

**Returns:** `bool` — true if lines are congruent, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.Congruent)

#### `DeterminePosition(geoObject_object, point, tolerance) | DeterminePosition(line, point, tolerance) | DeterminePosition(axis, point, tolerance) | DeterminePosition(polyline, point, tolerance) | DeterminePosition(polyline, point, tolerance, segment) | DeterminePosition(arc, point, tolerance) | DeterminePosition(arc, point, tolerance) | DeterminePosition(clothoid, point, tolerance) | DeterminePosition(spline, point, tolerance) | DeterminePosition(polygon, point, tolerance) | DeterminePosition(polygon, line) | DeterminePosition(line, point, tolerance) | DeterminePosition(polyline, point, tolerance) | DeterminePosition(phed, point) | DeterminePosition(spline, point, tolerance) | DeterminePosition(spline, point, tolerance) | DeterminePosition(path, point, tolerance) | DeterminePosition(b, point, tolerance) | DeterminePosition(polygon1, polygon2) | DeterminePosition(line, minMax) | DeterminePosition(lines, minMax) | DeterminePosition(igeo_object, point, tolerance)`

Determine the relative position of a point to an object.

**Remarks:** Determine the relative position of a point to an object.

**Parameters:**
- `geoObject_object` (object) — IGeometry
- `point` (Point2D) — Point
- `tolerance` (float) — tolerance (if distance < tolerance, point on object)

**Returns:** `eComparisionResult` — eOnElement(between start and end points), eEqualToStartPoint, eEqualToEndPoint, eLeft, eRight, eAbove, eBelow

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.DeterminePosition)

#### `Equal(el1, el2) | Equal(el1, el2) | Equal(el1, el2) | Equal(el1, el2) | Equal(el1, el2) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(el1, el2, tol) | Equal(path1, path2, tol) | Equal(igeo1_object, igeo2_object, tol)`

Compare 2 doubles

**Remarks:** Compare 2 doubles

**Parameters:**
- `el1` (float) — element
- `el2` (float) — element

**Returns:** `bool` — true when equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.Equal)

#### `EqualCoordsRel(el1, el2) | EqualCoordsRel(el1, el2)`

compare 2 2D points using iqrkor

**Remarks:** compare 2 2D points using iqrkor

**Parameters:**
- `el1` (Point2D) — element
- `el2` (Point2D) — element

**Returns:** `bool` — true when equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.EqualCoordsRel)

#### `EqualDistance(el1, el2, tol) | EqualDistance(el1, el2, tol)`

compare 2 2D points using their distance with given tolerance

**Remarks:** compare 2 2D points using their distance with given tolerance

**Parameters:**
- `el1` (Point2D) — point
- `el2` (Point2D) — point
- `tol` (float) — tolerance

**Returns:** `bool` — true when equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.EqualDistance)

#### `EqualRel(el1, el2) | EqualRel(el1, el2, tol) | EqualRel(el1, el2) | EqualRel(el1, el2) | EqualRel(el1, el2) | EqualRel(el1, el2)`

compare 2 doubles using built-in relative tolerance

**Remarks:** compare 2 doubles using built-in relative tolerance

**Parameters:**
- `el1` (float) — element
- `el2` (float) — element

**Returns:** `bool` — true when equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.EqualRel)

#### `GetSegmentNumber(geoObject_object, segment, tolerance) | GetSegmentNumber(geoObject_object, point, tolerance) | GetSegmentNumber(geoObject_object, point, tolerance) | GetSegmentNumber(line, point, tolerance) | GetSegmentNumber(line, point, tolerance) | GetSegmentNumber(polyPoints, point, tolerance) | GetSegmentNumber(polygon, point, tolerance) | GetSegmentNumber(polyPoints, point, tolerance) | GetSegmentNumber(spline, point, tolerance) | GetSegmentNumber(line, segment, tolerance) | GetSegmentNumber(line, segment, tolerance) | GetSegmentNumber(polyPoints, segment, tolerance) | GetSegmentNumber(polyPoints, segment, tolerance) | GetSegmentNumber(geoObject_object, segment, tolerance)`

Determine the segment position of a point to a 2D geometry object

**Remarks:** Determine the segment position of a point to a 2D geometry object

**Parameters:**
- `geoObject_object` (object) — IGeometry
- `segment` (Line2D) — Line2D
- `tolerance` (float) — tolerance (if distance < tolerance, point on object)

**Returns:** `int` — segment number 0 is first element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.GetSegmentNumber)

#### `HitsElement(result)`

Check if comparison result hits element in some way

**Remarks:** Check if comparison result hits element in some way

**Parameters:**
- `result` (eComparisionResult) — the comparison result to check

**Returns:** `bool` — true, if the result indicates that the element is hit, otherwise false e.g. left, right, etc.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.HitsElement)

#### `IsInside(polygon, line) | IsInside(line, lineToCheck)`

Determine if the line is inside the polygon

**Remarks:** Determine if the line is inside the polygon

**Parameters:**
- `polygon` (Polygon2D) — Polygon
- `line` (Line2D) — Line

**Returns:** `bool` — true, if the line is inside the polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.IsInside)

#### `IsParallel(el1, el2) | IsParallel(lines) | IsParallel(el1, el2) | IsParallel(el1_object, el2_object)`

test 2 2D lines for parallel

**Remarks:** test 2 2D lines for parallel

**Parameters:**
- `el1` (Line2D) — element
- `el2` (Line2D) — element

**Returns:** `eComparisionResult` — tuple(eParallel, eAntiParallel, eNotParallel,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.IsParallel)

#### `Overlapped(el1, el2) | Overlapped(el1, el2, partiallyToo=False) | Overlapped(pp1, pp2) | Overlapped(pp1, pp2) | Overlapped(line1, line2) | Overlapped(line1, line2, partiallyToo=False) | Overlapped(arc1, arc2) | Overlapped(arc1, arc2) | Overlapped(phed1, phed2) | Overlapped(el1, el2) | Overlapped(geo1_object, geo2_object)`

compare of 2 Polyline 2D

**Remarks:** compare of 2 Polyline 2D

**Parameters:**
- `el1` (Polyline2D) — Polyline2D
- `el2` (Polyline2D) — Polyline2D

**Returns:** `bool` — true when overlapped, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.Overlapped)

#### `signum(a, tol)`

sgn function

**Remarks:** sgn function

**Parameters:**
- `a` (float) — Number of which to compute sign
- `tol` (float) — tolerance

**Returns:** `float` — 0, 1, -1

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Comparison/#NemAll_Python_Geometry.Comparison.signum)

## Cone3D (class)

3D cone

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/)

### Constructors
- `Cone3D() | Cone3D(cone) | Cone3D(refPlacement, radiusMajor, radiusMinor, apex) | Cone3D(radiusMajor, radiusMinor, apex)` — Initialize

### Methods
#### `GetApex()`

Get Apex of the Cone in the local coordinate system

**Remarks:** Get Apex of the Cone in the local coordinate system

**Returns:** `Point3D` — Reference to Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetApex)

#### `GetApexParent()`

Get Apex of the Cone in the parent coordinate system

**Remarks:** Get Apex of the Cone in the parent coordinate system

**Returns:** `Point3D` — Reference to Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetApexParent)

#### `GetCenter()`

Get Center of the Cone

**Remarks:** Get Center of the Cone

**Returns:** `Point3D` — Reference to Center.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetCenter)

#### `GetHeight()`

Get Height of the Cone

**Remarks:** Get Height of the Cone

**Returns:** `float` — Reference to Height of the Cone.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetHeight)

#### `GetLocalPlacement()`

Get Local Placement

**Remarks:** Get Local Placement

**Returns:** `AxisPlacement3D` — Reference to Local Placement.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetLocalPlacement)

#### `GetMajorRadius()`

Get Major Radius of the Cone

**Remarks:** Get Major Radius of the Cone

**Returns:** `float` — Reference to MajorRadius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetMajorRadius)

#### `GetMinMax()`

Get MinMax of the Cone

**Remarks:** Get MinMax of the Cone

**Returns:** `MinMax3D` — Reference to MinMax of the Cone.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetMinMax)

#### `GetMinorRadius()`

Get Minor Radius of the Cone

**Remarks:** Get Minor Radius of the Cone

**Returns:** `float` — Reference to MinorRadius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetMinorRadius)

#### `GetXAxis()`

Get X-Axis of the placement of the Cone

**Remarks:** Get X-Axis of the placement of the Cone

**Returns:** `Vector3D` — Reference to X-Axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetXAxis)

#### `GetZAxis()`

Get Z - axis of the placement of the Cone

**Remarks:** Get Z - axis of the placement of the Cone

**Returns:** `Vector3D` — Reference to Z-axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.GetZAxis)

#### `IsCircular()`

Circularity check for the Cone

**Remarks:** Circularity check for the Cone

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.IsCircular)

#### `IsOblique()`

Perpendicularity check for the Cone

**Remarks:** Perpendicularity check for the Cone

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.IsOblique)

#### `IsValid()`

Validity check for the Cone

**Remarks:** Validity check for the Cone

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.IsValid)

#### `SetApex(apex)`

Set Apex in the local coordinate system

**Remarks:** Set Apex in the local coordinate system

**Parameters:**
- `apex` (Point3D) — New Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.SetApex)

#### `SetApexParent(apex)`

Set Apex in the parent coordinate system

**Remarks:** Set Apex in the parent coordinate system

**Parameters:**
- `apex` (Point3D) — New Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.SetApexParent)

#### `SetCenter(center)`

Set center

**Remarks:** Set center

**Parameters:**
- `center` (Point3D) — New center.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.SetCenter)

#### `SetHeight(arg2)`

Set Height of the Cone

**Remarks:** Set Height of the Cone

**Parameters:**
- `height` (object) — New height.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.SetHeight)

#### `SetLocalPlacement(placement)`

Set Local Placement.

**Remarks:** Set Local Placement.

**Parameters:**
- `placement` (AxisPlacement3D) — Local Placement.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.SetLocalPlacement)

#### `SetMajorRadius(arg2)`

Set Major Radius of the Cone

**Remarks:** Set Major Radius of the Cone

**Parameters:**
- `radius` (object) — New major radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.SetMajorRadius)

#### `SetMinorRadius(arg2)`

Set Minor Radius of the Cone

**Remarks:** Set Minor Radius of the Cone

**Parameters:**
- `radius` (object) — New minor radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.SetMinorRadius)

#### `__eq__(cone)`

Comparison of cones without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of cones without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `cone` (Cone3D) — Compared cone.

**Returns:** `object` — True when cones are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3D/#NemAll_Python_Geometry.Cone3D.__repr__)

### Properties
- `Apex` (None, get) — Get and set the apex property :type: None
- `LocalPlacement` (None, get) — Get and set the local placement property :type: None
- `MajorRadius` (None, get) — Get and set the major radius property :type: None
- `MinorRadius` (None, get) — Get and set the minor radius property :type: None

## Cone3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Cone3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/)

### Constructors
- `Cone3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Cone3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Cone3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Cone3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Cone3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Cone3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Cone3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cone3DList/#NemAll_Python_Geometry.Cone3DList.extend)

## ConicalSurface3D (class)

3D conical surface

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/)

### Constructors
- `ConicalSurface3D() | ConicalSurface3D(placement, radius, angle) | ConicalSurface3D(surface)` — Initialize

### Methods
#### `Get()`

Get all surface members

**Remarks:** Get all surface members

**Returns:** `tuple` — placement of conical surface,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.Get)

#### `GetPlacement()`

returns axis placement of the conical surface

**Remarks:** returns axis placement of the conical surface

**Returns:** `AxisPlacement3D` — placement - point + axis vector + reference direction vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.GetPlacement)

#### `GetRadius()`

Returns the radius at the placement

**Remarks:** Returns the radius at the placement

**Returns:** `float` — radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.GetRadius)

#### `GetSemiAngle()`

Returns the value of semi angle of conical surface

**Remarks:** Returns the value of semi angle of conical surface

**Returns:** `Angle` — angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.GetSemiAngle)

#### `IsValid()`

Check surface validity

**Remarks:** Check surface validity

**Returns:** `bool` — bool valid = true

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.IsValid)

#### `Set(placement, radius, angle)`

Set all surface members

**Remarks:** Set all surface members

**Parameters:**
- `placement` (AxisPlacement3D) — placement of conical surface
- `radius` (float) — radius of conical surface in placement
- `angle` (Angle) — semi angle of conical surface

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.Set)

#### `SetPlacement(value)`

sets the position of conical surface

**Remarks:** sets the position of conical surface

**Parameters:**
- `value` (AxisPlacement3D) — placement - point + axis vector + reference direction vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.SetPlacement)

#### `SetRadius(value)`

Sets the radius at the placement

**Remarks:** Sets the radius at the placement

**Parameters:**
- `value` (float) — radius to be set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.SetRadius)

#### `SetSemiAngle(value)`

Sets the value of semi angle of conical surface

**Remarks:** Sets the value of semi angle of conical surface

**Parameters:**
- `value` (Angle) — angle to be set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.SetSemiAngle)

#### `__eq__(surface)`

Comparison of conical surfaces without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of conical surfaces without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `conical` (object) — surface Compared conical surface.

**Returns:** `object` — True when conical surfaces are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3D/#NemAll_Python_Geometry.ConicalSurface3D.__repr__)

### Properties
- `Placement` (None, get) — Get and set the placement property :type: None
- `Radius` (None, get) — Get and set the radius property :type: None
- `SemiAngle` (None, get) — Get and set the semi angle property :type: None

## ConicalSurface3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.ConicalSurface3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/)

### Constructors
- `ConicalSurface3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ConicalSurface3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ConicalSurface3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ConicalSurface3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ConicalSurface3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ConicalSurface3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ConicalSurface3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ConicalSurface3DList/#NemAll_Python_Geometry.ConicalSurface3DList.extend)

## Cuboid3D (class)

3D cuboid

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/)

### Constructors
- `Cuboid3D() | Cuboid3D(cuboid) | Cuboid3D(refPoint, startPoint, vec1, vec2, vec3)` — Initialize

### Methods
#### `GetBottomFacePolygon()`

Get the boundary polygon of bottom face of the cuboid

**Remarks:** Get the boundary polygon of bottom face of the cuboid

**Returns:** `Polygon3D` — Bottom face boundary polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetBottomFacePolygon)

#### `GetGroundPlane()`

Get ground plane ( ref point + Z-vector ).

**Remarks:** Get ground plane ( ref point + Z-vector ).

**Returns:** `Plane3D` — ground Plane3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetGroundPlane)

#### `GetHeight()`

Get height.

**Remarks:** Get height.

**Returns:** `float` — Size of Z-vector ( vector3 ).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetHeight)

#### `GetLength()`

Get length.

**Remarks:** Get length.

**Returns:** `float` — Size of X-vector ( vector1 ).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetLength)

#### `GetRefPoint()`

Get reference point.

**Remarks:** Get reference point.

**Returns:** `Point3D` — Reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetRefPoint)

#### `GetRelStartPoint()`

Get start point in relative coordinates.

**Remarks:** Get start point in relative coordinates.

**Returns:** `Point3D` — Constant reference to the start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetRelStartPoint)

#### `GetStartPoint()`

Get start point in world coordinates.

**Remarks:** Get start point in world coordinates.

**Returns:** `Point3D` — copy of start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetStartPoint)

#### `GetTopFacePolygon()`

Get the boundary polygon of top face of the cuboid

**Remarks:** Get the boundary polygon of top face of the cuboid

**Returns:** `Polygon3D` — Top face boundary polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetTopFacePolygon)

#### `GetVector(index)`

Get given vector.

**Remarks:** Get given vector.

**Parameters:**
- `index` (int) — Index of the vector.

**Returns:** `Vector3D` — m_Vec[index]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetVector)

#### `GetWidth()`

Get width.

**Remarks:** Get width.

**Returns:** `float` — Size of Y-vector ( vector2 ).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.GetWidth)

#### `Set(refPoint, startPoint, vec1, vec2, vec3) | Set(cuboid)`

Set the Cuboid.

**Remarks:** Set the Cuboid.

**Parameters:**
- `refPoint` (Point3D) — Reference point in world coordinate system.
- `startPoint` (Point3D) — Start point of cuboid.
- `vec1` (Vector3D) — X-vector.
- `vec2` (Vector3D) — Y-vector.
- `vec3` (Vector3D) — Z-vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.Set)

#### `SetHeight(height)`

Set height.

**Remarks:** Set height.

**Parameters:**
- `height` (float) — New size of Z-vector ( vector3 ).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.SetHeight)

#### `SetLength(length)`

Set length.

**Remarks:** Set length.

**Parameters:**
- `length` (float) — New size of X-vector ( vector1 ).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.SetLength)

#### `SetRefPoint(refPoint)`

Set reference point.

**Remarks:** Set reference point.

**Parameters:**
- `refPoint` (Point3D) — New reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.SetRefPoint)

#### `SetStartPoint(startPoint)`

Set start point.

**Remarks:** Set start point.

**Parameters:**
- `startPoint` (Point3D) — New start point ( in world coordinates ).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.SetStartPoint)

#### `SetVector(vec, index)`

Set the vector.

**Remarks:** Set the vector.

**Parameters:**
- `vec` (Vector3D) — New vector.
- `index` (int) — Index pf the vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.SetVector)

#### `SetWidth(width)`

Set width.

**Remarks:** Set width.

**Parameters:**
- `width` (float) — New size of Y-vector ( vector2 ).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.SetWidth)

#### `__eq__(cuboid)`

Comparison of cuboids without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of cuboids without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `cuboid` (Cuboid3D) — Compared cuboid.

**Returns:** `object` — True when cuboids are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3D/#NemAll_Python_Geometry.Cuboid3D.__repr__)

### Properties
- `HeightVector` (None, get) — Get and set the height vector property :type: None
- `LengthVector` (None, get) — Get and set the length vector property :type: None
- `RefPoint` (None, get) — Get and set the ref point property :type: None
- `StartPoint` (None, get) — Get and set the start point property :type: None
- `WidthVector` (None, get) — Get and set the width vector property :type: None

## Cuboid3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Cuboid3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/)

### Constructors
- `Cuboid3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Cuboid3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Cuboid3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Cuboid3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Cuboid3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Cuboid3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Cuboid3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cuboid3DList/#NemAll_Python_Geometry.Cuboid3DList.extend)

## Cylinder3D (class)

3D cylinder

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/)

### Constructors
- `Cylinder3D() | Cylinder3D(cylinder) | Cylinder3D(refPlacement, radiusMajor, radiusMinor, apex) | Cylinder3D(radiusMajor, radiusMinor, apex)` — Initialize

### Methods
#### `GetApex()`

Get Apex of the Cylinder in the local coordinate system

**Remarks:** Get Apex of the Cylinder in the local coordinate system

**Returns:** `Point3D` — Reference to Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetApex)

#### `GetApexParent()`

Get Apex of the Cylinder in the parent coordinate system

**Remarks:** Get Apex of the Cylinder in the parent coordinate system

**Returns:** `Point3D` — Reference to Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetApexParent)

#### `GetBottomCenter()`

Get bottom center

**Remarks:** Get bottom center

**Returns:** `Point3D` — center of bottom base

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetBottomCenter)

#### `GetCenter()`

Get Center of the Cylinder

**Remarks:** Get Center of the Cylinder

**Returns:** `Point3D` — Reference to Center.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetCenter)

#### `GetHeight()`

Get Height of the Cylinder

**Remarks:** Get Height of the Cylinder

**Returns:** `float` — Reference to Height of the Cylinder.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetHeight)

#### `GetLocalPlacement()`

Get Local Placement

**Remarks:** Get Local Placement

**Returns:** `AxisPlacement3D` — Reference to Local Placement.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetLocalPlacement)

#### `GetMajorRadius()`

Get Major Radius of the Cylinder

**Remarks:** Get Major Radius of the Cylinder

**Returns:** `float` — Reference to Major Radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetMajorRadius)

#### `GetMinMax()`

Get MinMax of the Cylinder

**Remarks:** Get MinMax of the Cylinder

**Returns:** `MinMax3D` — Reference to MinMax of the Cylinder.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetMinMax)

#### `GetMinorRadius()`

Get Minor Radius of the Cylinder

**Remarks:** Get Minor Radius of the Cylinder

**Returns:** `float` — Reference to Minor Radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetMinorRadius)

#### `GetSilhouetteLines(viewMatrix, bPerspective)`

Get silhouette lines of cylinder

**Remarks:** Get silhouette lines of cylinder

**Parameters:**
- `viewMatrix` (Matrix3D) — View matrix
- `bPerspective` (bool) — Flag if it is central projection or not (true / false)

**Returns:** `Line3DList` — silhouette lines

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetSilhouetteLines)

#### `GetTopCenter()`

Get top center

**Remarks:** Get top center

**Returns:** `Point3D` — center of top plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetTopCenter)

#### `GetXAxis()`

Get X-Axis of the placement of the Cylinder

**Remarks:** Get X-Axis of the placement of the Cylinder

**Returns:** `Vector3D` — Reference to X-Axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetXAxis)

#### `GetYAxis()`

Get Y-Axis of the placement of the Cylinder

**Remarks:** Get Y-Axis of the placement of the Cylinder

**Returns:** `Vector3D` — Reference to X-Axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetYAxis)

#### `GetZAxis()`

Get Z - axis of the placement of the Cylinder

**Remarks:** Get Z - axis of the placement of the Cylinder

**Returns:** `Vector3D` — Reference to Z-axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.GetZAxis)

#### `IsCircular()`

Circularity check of the Cylinder

**Remarks:** Circularity check of the Cylinder

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.IsCircular)

#### `IsOblique()`

Perpendicularity check of the Cylinder

**Remarks:** Perpendicularity check of the Cylinder

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.IsOblique)

#### `IsValid()`

Validity check of the Cylinder

**Remarks:** Validity check of the Cylinder

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.IsValid)

#### `SetApex(apex)`

Set Apex in the local coordinate system

**Remarks:** Set Apex in the local coordinate system

**Parameters:**
- `apex` (Point3D) — New Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.SetApex)

#### `SetApexParent(apex)`

Set Apex in the parent coordinate system

**Remarks:** Set Apex in the parent coordinate system

**Parameters:**
- `apex` (Point3D) — New Apex.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.SetApexParent)

#### `SetCenter(center)`

Set center

**Remarks:** Set center

**Parameters:**
- `center` (Point3D) — New center.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.SetCenter)

#### `SetHeight(arg2)`

Set Height of the Cylinder

**Remarks:** Set Height of the Cylinder

**Parameters:**
- `height` (object) — New height.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.SetHeight)

#### `SetLocalPlacement(placement) | SetLocalPlacement(center, xAxis, zAxis)`

Set Local Placement.

**Remarks:** Set Local Placement.

**Parameters:**
- `placement` (AxisPlacement3D) — Local Placement.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.SetLocalPlacement)

#### `SetMajorRadius(arg2)`

Set Major Radius of the Cylinder

**Remarks:** Set Major Radius of the Cylinder

**Parameters:**
- `radius` (object) — New major radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.SetMajorRadius)

#### `SetMinorRadius(arg2)`

Set Minor Radius of the Cylinder

**Remarks:** Set Minor Radius of the Cylinder

**Parameters:**
- `radius` (object) — New minor radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.SetMinorRadius)

#### `__eq__(cylinder)`

Comparison of cylinders without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of cylinders without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `cylinder` (Cylinder3D) — Compared cylinder.

**Returns:** `object` — True when cylinders are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3D/#NemAll_Python_Geometry.Cylinder3D.__repr__)

### Properties
- `Apex` (None, get) — Get and set the apex property :type: None
- `LocalPlacement` (None, get) — Get and set the local placement property :type: None
- `MajorRadius` (None, get) — Get and set the major radius property :type: None
- `MinorRadius` (None, get) — Get and set the minor radius property :type: None

## Cylinder3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Cylinder3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/)

### Constructors
- `Cylinder3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Cylinder3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Cylinder3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Cylinder3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Cylinder3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Cylinder3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Cylinder3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Cylinder3DList/#NemAll_Python_Geometry.Cylinder3DList.extend)

## DivisionPoints (class)

Class for DivisionPoints

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/)

### Constructors
- `DivisionPoints(path, sectionLength, eps) | DivisionPoints(geoObject, number, eps) | DivisionPoints(path, number, eps) | DivisionPoints(geoObject, sectionLength, eps)` — Constructor

### Methods
#### `Count()`

Get number of division points

**Remarks:** Get number of division points

**Returns:** `int` — number of division points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.Count)

#### `GetEndPoint()`

Get the end point

**Remarks:** Get the end point

**Returns:** `Point3D` — End Point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetEndPoint)

#### `GetEndPointAngle()`

Get the angle at the end point

**Remarks:** Get the angle at the end point

**Returns:** `Angle` — Angle at the end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetEndPointAngle)

#### `GetPoint(index)`

Get division point

**Remarks:** Get division point

**Parameters:**
- `index` (int) — index of the division point

**Returns:** `Point3D` — division point as Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetPoint)

#### `GetPointAngle(index)`

Get the angle of a division point

**Remarks:** Get the angle of a division point

**Parameters:**
- `index` (int) — index of the division point

**Returns:** `Angle` — angle as Angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetPointAngle)

#### `GetPointAngles()`

Get the angles of the division points

**Remarks:** Get the angles of the division points

**Returns:** `AngleList` — copy of the angles of the division points as vector of Angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetPointAngles)

#### `GetPoints()`

Get the calculated division points

**Remarks:** Get the calculated division points

**Returns:** `Point3DList` — copy of the division points as vector of Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetPoints)

#### `GetStartPoint()`

Get the start point

**Remarks:** Get the start point

**Returns:** `Point3D` — Start Point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetStartPoint)

#### `GetStartPointAngle()`

Get the angle at the start point

**Remarks:** Get the angle at the start point

**Returns:** `Angle` — Angle at the start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.GetStartPointAngle)

#### `IsSuccessful()`

Get the result of the computation

**Remarks:** Get the result of the computation

**Returns:** `bool` — Computation successful: true / false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/DivisionPoints/#NemAll_Python_Geometry.DivisionPoints.IsSuccessful)

## Ellipsoid3D (class)

3D ellipsoid

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/)

### Constructors
- `Ellipsoid3D() | Ellipsoid3D(ellipsoid) | Ellipsoid3D(refPlacement, radiusX, radiusY, radiusZ) | Ellipsoid3D(radiusX, radiusY, radiusZ)` — Initialize

### Methods
#### `GetCenter()`

Get Center of the Ellipsoid

**Remarks:** Get Center of the Ellipsoid

**Returns:** `Point3D` — Reference to Center.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetCenter)

#### `GetIsoLines(USegmentsCount, VSegmentsCount)`

Test whether the Ellipsoid is Sphere

**Remarks:** Test whether the Ellipsoid is Sphere

**Parameters:**
- `USegmentsCount` (int) — count of circles
- `VSegmentsCount` (int) — count of circles

**Returns:** `Arc3DList` — vector of circles

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetIsoLines)

#### `GetLocalPlacement()`

Get Local Placement

**Remarks:** Get Local Placement

**Returns:** `AxisPlacement3D` — Reference to Local Placement.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetLocalPlacement)

#### `GetSilhouetteContour(viewMatrix, bPerspective)`

Get silhouette circle

**Remarks:** Get silhouette circle

**Parameters:**
- `viewMatrix` (Matrix3D) — view matrix
- `bPerspective` (bool) — central perspective true/false

**Returns:** `Arc3D` — silhouette

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetSilhouetteContour)

#### `GetXAxis()`

Get X-Axis of the placement of the Ellipsoid

**Remarks:** Get X-Axis of the placement of the Ellipsoid

**Returns:** `Vector3D` — Reference to X-Axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetXAxis)

#### `GetXRadius()`

Get X Radius of the Ellipsoid

**Remarks:** Get X Radius of the Ellipsoid

**Returns:** `float` — Reference to X Radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetXRadius)

#### `GetYAxis()`

Get Y-Axis of the placement of the Ellipsoid

**Remarks:** Get Y-Axis of the placement of the Ellipsoid

**Returns:** `Vector3D` — Reference to X-Axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetYAxis)

#### `GetYRadius()`

Get Y Radius of the Ellipsoid

**Remarks:** Get Y Radius of the Ellipsoid

**Returns:** `float` — Reference to Y Radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetYRadius)

#### `GetZAxis()`

Get Z - axis of the placement of the Ellipsoid

**Remarks:** Get Z - axis of the placement of the Ellipsoid

**Returns:** `Vector3D` — Reference to Z-axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetZAxis)

#### `GetZRadius()`

Get Z Radius of the Ellipsoid

**Remarks:** Get Z Radius of the Ellipsoid

**Returns:** `float` — Reference to Z Radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.GetZRadius)

#### `IsSphere()`

Test whether the Ellipsoid is Sphere

**Remarks:** Test whether the Ellipsoid is Sphere

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.IsSphere)

#### `IsValid()`

Validity check of the Ellipsoid

**Remarks:** Validity check of the Ellipsoid

**Returns:** `bool` — true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.IsValid)

#### `SetCenter(center)`

Set center

**Remarks:** Set center

**Parameters:**
- `center` (Point3D) — New center.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.SetCenter)

#### `SetLocalPlacement(placement) | SetLocalPlacement(center, xAxis, zAxis)`

Set Local Placement.

**Remarks:** Set Local Placement.

**Parameters:**
- `placement` (AxisPlacement3D) — Local Placement.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.SetLocalPlacement)

#### `SetXRadius(rad)`

Set X Radius of the Ellipsoid

**Remarks:** Set X Radius of the Ellipsoid

**Parameters:**
- `rad` (float) — New radius.

**Returns:** `object` — Reference to X Radius.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.SetXRadius)

#### `SetYRadius(rad)`

Set Y Radius of the Ellipsoid

**Remarks:** Set Y Radius of the Ellipsoid

**Parameters:**
- `rad` (float) — New radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.SetYRadius)

#### `SetZRadius(rad)`

Set Z Radius of the Ellipsoid

**Remarks:** Set Z Radius of the Ellipsoid

**Parameters:**
- `rad` (float) — New radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.SetZRadius)

#### `__eq__(ellipsoid)`

Comparison of ellipsoids without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of ellipsoids without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `ellipsoid` (Ellipsoid3D) — Compared ellipsoid.

**Returns:** `object` — True when ellipsoids are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3D/#NemAll_Python_Geometry.Ellipsoid3D.__repr__)

### Properties
- `LocalPlacement` (None, get) — Get and set the local placement property :type: None
- `XRadius` (None, get) — Get and set the x radius property :type: None
- `YRadius` (None, get) — Get and set the y radius property :type: None
- `ZRadius` (None, get) — Get and set the z radius property :type: None

## Ellipsoid3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Ellipsoid3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/)

### Constructors
- `Ellipsoid3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Ellipsoid3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Ellipsoid3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Ellipsoid3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Ellipsoid3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Ellipsoid3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Ellipsoid3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Ellipsoid3DList/#NemAll_Python_Geometry.Ellipsoid3DList.extend)

## ExtrudedAreaSolid3D (class)

Solid created by extrusion of area by given direction vector

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/)

### Constructors
- `ExtrudedAreaSolid3D() | ExtrudedAreaSolid3D(solid) | ExtrudedAreaSolid3D(solid, refPoint)` — Initialize

### Methods
#### `GetDirection()`

Get direction for extrusion

**Remarks:** Get direction for extrusion

**Returns:** `Vector3D` — Vector3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.GetDirection)

#### `GetExtrudedArea()`

Get extruded area

**Remarks:** Get extruded area

**Returns:** `PolygonalArea3D` — PolygonalArea3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.GetExtrudedArea)

#### `GetRefPoint()`

Get the reference point

**Remarks:** Get the reference point

**Returns:** `Point3D` — Reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.GetRefPoint)

#### `IsValid()`

Check if the Solid is valid

**Remarks:** Check if the Solid is valid

**Returns:** `bool` — True if it is a valid solid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.IsValid)

#### `SetDirection(dir)`

Set direction for extrusion

**Remarks:** Set direction for extrusion

**Parameters:**
- `dir` (Vector3D) — Vector3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.SetDirection)

#### `SetExtrudedArea(area)`

Set extruded area

**Remarks:** Set extruded area

**Parameters:**
- `area` (PolygonalArea3D) — PolygonalArea3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.SetExtrudedArea)

#### `SetRefPoint(refPoint)`

Set reference point

**Remarks:** Set reference point

**Parameters:**
- `refPoint` (Point3D) — New reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.SetRefPoint)

#### `__eq__(extrudedAreaSolid)`

Comparison of extrudedAreaSolids without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of extrudedAreaSolids without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `extrudedAreaSolid` (ExtrudedAreaSolid3D) — Compared extrudedAreaSolid.

**Returns:** `object` — True when extrudedAreaSolids are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.__eq__)

#### `__mul__(matrix)`

Matrix transformation

**Remarks:** Matrix transformation

**Parameters:**
- `matrix` (Matrix3D) — Transformation 3D matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.__mul__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3D/#NemAll_Python_Geometry.ExtrudedAreaSolid3D.__repr__)

### Properties
- `Direction` (None, get) — Get and set the direction as property :type: None
- `ExtrudedArea` (None, get) — Get and set the extruded area as property :type: None
- `RefPoint` (None, get) — Get and set the reference point as property :type: None

## ExtrudedAreaSolid3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.ExtrudedAreaSolid3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/)

### Constructors
- `ExtrudedAreaSolid3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ExtrudedAreaSolid3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ExtrudedAreaSolid3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ExtrudedAreaSolid3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ExtrudedAreaSolid3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ExtrudedAreaSolid3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ExtrudedAreaSolid3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ExtrudedAreaSolid3DList/#NemAll_Python_Geometry.ExtrudedAreaSolid3DList.extend)

## FaceOffset (enum)

Face offset

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FaceOffset/)

### Constructors
- `FaceOffset(inputPolyhedron) | FaceOffset(inputPolyhedron, faceIndices) | FaceOffset(inputBRep) | FaceOffset(inputBRep, faceIndices)` — Constructor with polyhedron input geometry

### Methods
#### `IsResultBrep()`

returns if selected object has brep geometry

**Remarks:** returns if selected object has brep geometry

**Returns:** `bool` — true if brep

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FaceOffset/#NemAll_Python_Geometry.FaceOffset.IsResultBrep)

#### `Offset(offsetDistance, direction, useOffsetStepPierce, useOrthoVXSplit, punchDirection)`

Execute face offset

**Remarks:** Execute face offset

**Parameters:**
- `offsetDistance` (float) — distance for offset
- `direction` (eFaceOffsetDirection) — offset direction 0 - in face normal direction, 1 - in both directions, 2 - opposite to face normal
- `useOffsetStepPierce` (bool) — if to use offset step pierce
- `useOrthoVXSplit` (bool) — if to use OrthoVXSplit option
- `punchDirection` (Vector3D) — defined direction for offset (optional)

**Returns:** `tuple` — eOK if success,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FaceOffset/#NemAll_Python_Geometry.FaceOffset.Offset)

#### `Shell(offsetDistance, direction, useOffsetStepPierce, useOrthoVXSplit, punchDirection)`

Execute face shell

**Remarks:** Execute face shell

**Parameters:**
- `offsetDistance` (float) — distance for offset
- `direction` (eFaceOffsetDirection) — offset direction 0 - in face normal direction, 1 - in both directions, 2 - opposite to face normal
- `useOffsetStepPierce` (bool) — if to use offset step pierce
- `useOrthoVXSplit` (bool) — if to use OrthoVXSplit option
- `punchDirection` (Vector3D) — defined direction for shell (optional)

**Returns:** `tuple` — eOK if success,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FaceOffset/#NemAll_Python_Geometry.FaceOffset.Shell)

## FilletCalculus2D (class)

Class for fillet calculation between two 2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/)

### Constructors
- `FilletCalculus2D(line1, line2, r) | FilletCalculus2D(line, arc, r) | FilletCalculus2D(arc1, arc2, r) | FilletCalculus2D(arc, point) | FilletCalculus2D(line, point) | FilletCalculus2D(geoObj1, geoObj2, r)` — constructor

### Methods
#### `ClickedOnObject(line, clickedPoint, searchRadius) | ClickedOnObject(arc, clickedPoint, searchRadius)`

Check if click point is a point located on given line

**Remarks:** Check if click point is a point located on given line

**Parameters:**
- `line` (Line2D) — Line2D which will be checked
- `clickedPoint` (Point2D) — clicked point
- `searchRadius` (float) — search radius

**Returns:** `bool` — true if point is on the line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.ClickedOnObject)

#### `GetArcHelpConstructions()`

Get all arc help constructions

**Remarks:** Get all arc help constructions

**Returns:** `Arc2DList` — all arc help constructions

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.GetArcHelpConstructions)

#### `GetFilletType(line1, line2) | GetFilletType(line, arc) | GetFilletType(arc1, arc2) | GetFilletType(geometry1, geometry2)`

Get fillet type for two lines

**Remarks:** Get fillet type for two lines

**Parameters:**
- `line1` (Line2D) — first line
- `line2` (Line2D) — second line

**Returns:** `eFilletType` — eFilletType

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.GetFilletType)

#### `GetFillets()`

Get all possible fillets

**Remarks:** Get all possible fillets

**Returns:** `Arc2DList` — possible fillets

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.GetFillets)

#### `GetLineHelpConstructions()`

Get all line help constructions

**Remarks:** Get all line help constructions

**Returns:** `object` — all line help constructions

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.GetLineHelpConstructions)

#### `GetNearest(point)`

Get the nearest fillet to the point

**Remarks:** Get the nearest fillet to the point

**Parameters:**
- `point` (Point2D) — Point2D

**Returns:** `Arc2D` — nearest arc to the point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.GetNearest)

#### `SplitPolylineBySegment(polyline, segment)`

Split polyline by given segment

**Remarks:** Split polyline by given segment

**Parameters:**
- `polyline` (Polyline2D) — polyline which will be split by given segment
- `segment` (int) — segment where polyline will be split

**Returns:** `tuple` — first part of polyline,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.SplitPolylineBySegment)

#### `TrimByHelpConstruction(line, hcLine, intersections) | TrimByHelpConstruction(polyline, segment, hcLine, intersections) | TrimByHelpConstruction(arc, hcArc, intersections)`

Trim line by given help construction

**Remarks:** Trim line by given help construction

**Parameters:**
- `line` (Line2D) — line which will be trimmed
- `hcLine` (Line2D) — line help construction
- `intersections` (Point3DList) — intersection points

**Returns:** `Line2D` — line which will be trimmed

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.TrimByHelpConstruction)

#### `UpdateBySelectedHelpConstruction(polyline, segment, trimHelpConstruction, intersection) | UpdateBySelectedHelpConstruction(line, trimHelpConstruction, intersection)`

Update polyline by selected help construction

**Remarks:** Update polyline by selected help construction

**Parameters:**
- `polyline` (Polyline2D) — polyline which will be updated
- `segment` (Line2D) — selected segment
- `trimHelpConstruction` (object) — help construction data
- `intersection` (Point2D) — intersection

**Returns:** `Polyline2D` — polyline which will be updated

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.UpdateBySelectedHelpConstruction)

#### `UpdateGeometry(geoArc, fillet, selectedGeometry) | UpdateGeometry(geoPolyline, fillet, selectedGeometry1, selectedGeometry2) | UpdateGeometry(geoPolyline, fillet, selectedGeometry) | UpdateGeometry(geoPolyline, fillet, selectedGeometry1, selectedGeometry2, segmentCount) | UpdateGeometry(geoPolygon, fillet, segment1, segment2, segmentCount) | UpdateGeometry(geoSpline, fillet) | UpdateGeometry(geoLine, fillet, selectedGeometry)`

Update arc geometry

**Remarks:** Update arc geometry

**Parameters:**
- `geoArc` (Arc2D) — arc which will be updated
- `fillet` (Arc2D) — Arc2D
- `selectedGeometry` (object) — selected geometry

**Returns:** `Arc2D` — arc which will be updated

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus2D/#NemAll_Python_Geometry.FilletCalculus2D.UpdateGeometry)

## FilletCalculus3D (class)

Class for 3D fillet calculation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus3D/)

### Methods
#### `Calculate(line1, line2, radius) | Calculate(line1, line2) | Calculate(line1, endPoint, polyline, startLineIndex, endLineIndex, radius) | Calculate(polyhedron, edges, radius, propagation) | Calculate(brep, radius) | Calculate(brep, edges, radius, propagation)`

Calculate fillet between two coplanar 3d lines

**Remarks:** Calculate fillet between two coplanar 3d lines

**Parameters:**
- `line1` (Line3D) — first Line3D
- `line2` (Line3D) — second Line3D
- `radius` (float) — radius of fillet

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/FilletCalculus3D/#NemAll_Python_Geometry.FilletCalculus3D.Calculate)

## Functions (static-class)

Module-level functions of NemAll_Python_Geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/)

### Methods
#### `AddToMinMax(minMax, point) | AddToMinMax(minMax, point) | AddToMinMax(minMax, point) | AddToMinMax(minMax, point)`

Calculate 2D-Min- and Max Point of an 2D point Calculates the MinMax2D volume by the Point2D given in parameter point.

**Remarks:** Calculate 2D-Min- and Max Point of an 2D point Calculates the MinMax2D volume by the Point2D given in parameter point.

**Parameters:**
- `minMax` (MinMax2D) — where to ad min max box
- `point` (Point2D) — Point2D to be added

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.AddToMinMax)

#### `CalcAngle(l1) | CalcAngle(point1, point2)`

compute angle of 2D line and x axis

**Remarks:** compute angle of 2D line and x axis

**Parameters:**
- `l1` (Line2D) — 2D Line

**Returns:** `tuple` — result angle (normalized 2pi),

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcAngle)

#### `CalcArea(p1, p2, p3) | CalcArea(polygon) | CalcArea(polygon) | CalcArea(polygonalArea) | CalcArea(areaGeo, arcSegmentation, useArcSegmentation, armLength, riseValue) | CalcArea(areaGeo, arcSegmentation, useArcSegmentation, armLength, riseValue)`

compute area of a triangle

**Remarks:** compute area of a triangle

**Parameters:**
- `p1` (Point2D) — Point
- `p2` (Point2D) — Point
- `p3` (Point2D) — Point

**Returns:** `tuple` — area result area,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcArea)

#### `CalcLength(el) | CalcLength(vec) | CalcLength(polyline) | CalcLength(polygon) | CalcLength(polygon) | CalcLength(arc) | CalcLength(clothoid) | CalcLength(spline) | CalcLength(path) | CalcLength(line) | CalcLength(vec) | CalcLength(polyline) | CalcLength(arc) | CalcLength(path) | CalcLength(poly) | CalcLength(spline) | CalcLength(spline) | CalcLength(spline)`

calculate length of a 2D line.

**Remarks:** calculate length of a 2D line.

**Parameters:**
- `el` (Line2D) — 2D line.

**Returns:** `float` — double result.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcLength)

#### `CalcMass(elem) | CalcMass(elem) | CalcMass(elem) | CalcMass(elem) | CalcMass(elem) | CalcMass(elem) | CalcMass(elem) | CalcMass(elem)`

Calculate mass values

**Remarks:** Calculate mass values

**Parameters:**
- `elem` (Polyhedron3D) — Polyhedron3D

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcMass)

#### `CalcMinMax(point) | CalcMinMax(line) | CalcMinMax(polyline) | CalcMinMax(polygon) | CalcMinMax(arc) | CalcMinMax(spline) | CalcMinMax(spline) | CalcMinMax(spline) | CalcMinMax(spline) | CalcMinMax(clothoid) | CalcMinMax(path) | CalcMinMax(area) | CalcMinMax(area) | CalcMinMax(point) | CalcMinMax(line) | CalcMinMax(polyline) | CalcMinMax(pointCloud) | CalcMinMax(polygon) | CalcMinMax(arc) | CalcMinMax(polyhedron) | CalcMinMax(cuboid) | CalcMinMax(cylinder) | CalcMinMax(cone) | CalcMinMax(ellipsoid) | CalcMinMax(area) | CalcMinMax(solid) | CalcMinMax(solid) | CalcMinMax(geo) | CalcMinMax(geo)`

Calculate Min- and Max Point of an element. Calculates the MinMax2D box by the Point2D given in parameter point

**Remarks:** Calculate Min- and Max Point of an element. Calculates the MinMax2D box by the Point2D given in parameter point

**Parameters:**
- `point` (Point2D) — Point2D to be calculated

**Returns:** `MinMax2D` — tuple(MinMax2D,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcMinMax)

#### `CalcProjectedMinMax(geo, matrix)`

Calculate minmax of brep in given projection

**Remarks:** Calculate minmax of brep in given projection

**Parameters:**
- `geo` (BRep3D) — BRep3D to calculate minmax for
- `matrix` (Matrix3D) — projection minmax

**Returns:** `MinMax3D` — tuple(minmax 3D,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcProjectedMinMax)

#### `CalcSurface(elem)`

Calculate surface of BRep3D

**Remarks:** Calculate surface of BRep3D

**Parameters:**
- `elem` (BRep3D) — input BRep3D

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcSurface)

#### `CalcVolume(elem)`

Calculate volume of BRep3D

**Remarks:** Calculate volume of BRep3D

**Parameters:**
- `elem` (BRep3D) — input BRep3D

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalcVolume)

#### `CalculateNormal(brep, inputPnt)`

Calculate normal vector of BRep at point

**Remarks:** Calculate normal vector of BRep at point

**Parameters:**
- `brep` (BRep3D) — BRep
- `inputPnt` (Point3D) — input point

**Returns:** `Vector3D` — normal vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalculateNormal)

#### `CalculateSplineLengthToPoint(spline, splinePointIndex)`

Calculate spline length from spline start to defined spline point. This function returns correct result even if the various points of the spline have the same coordinates ( PointLocal(.....) function fails in that case ).

**Remarks:** Calculate spline length from spline start to defined spline point. This function returns correct result even if the various points of the spline have the same coordinates ( PointLocal(.....) function fails in that case ).

**Parameters:**
- `spline` (Spline2D) — 2D Spline
- `splinePointIndex` (int) — Spline point index

**Returns:** `float` — Length if everything is OK. -1.0 in case of error.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CalculateSplineLengthToPoint)

#### `Clipping(el1, el2)`

test 2D line and minmax box for clipping

**Remarks:** test 2D line and minmax box for clipping

**Parameters:**
- `el1` (Line2D) — 2D line
- `el2` (MinMax2D) — 2D minmax

**Returns:** `eBoolOpResult` — eInside (line inside the box), eOutside (line outside the box), eClip

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Clipping)

#### `Colliding(polygon1, polygon2) | Colliding(polygon, line) | Colliding(minMax, polygon) | Colliding(brep1, brep2)`

Test for collision between 2 geometry objects

**Remarks:** Test for collision between 2 geometry objects

**Parameters:**
- `polygon1` (Polygon2D) — the first polygon
- `polygon2` (Polygon2D) — the second polygon

**Returns:** `bool` — true when colliding, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Colliding)

#### `Convert3DRotation(axis, angle)`

Convert a 3D Rotation to series of 2D operations: scale, rotation, scale, rotation.

**Remarks:** Convert a 3D Rotation to series of 2D operations: scale, rotation, scale, rotation.

**Parameters:**
- `axis` (Axis3D) — 3D axis
- `angle` (Angle) — Angle of 3D rotation

**Returns:** `tuple` — The first scale in X axis,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Convert3DRotation)

#### `ConvertTo2D(line3D) | ConvertTo2D(polyline3D) | ConvertTo2D(axis3D) | ConvertTo2D(point3D) | ConvertTo2D(polygon3D) | ConvertTo2D(arc3D) | ConvertTo2D(spline3D) | ConvertTo2D(bspline3D) | ConvertTo2D(points3D) | ConvertTo2D(path3D) | ConvertTo2D(geo_object)`

Converts Line3D to Line2D

**Remarks:** Converts Line3D to Line2D

**Parameters:**
- `line3D` (Line3D) — Line3D to convert

**Returns:** `bool` — tuple(line2D is valid: true/false,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.ConvertTo2D)

#### `ConvertTo3D(line2D, zPlane=0) | ConvertTo3D(polyline2D, zPlane=0) | ConvertTo3D(axis2D, zPlane=0) | ConvertTo3D(spline2D, zPlane=0) | ConvertTo3D(arc2D, zPlane=0) | ConvertTo3D(bspline2D, zPlane=0) | ConvertTo3D(path2D, zPlane=0) | ConvertTo3D(points2D, zPlane=0) | ConvertTo3D(polygon, zPlane=0) | ConvertTo3D(clothoid2D, zPlane=0)`

Converts Line2D to Line3D

**Remarks:** Converts Line2D to Line3D

**Parameters:**
- `line2D` (Line2D) — Line2D to convert
- `zPlane` (float) — Z value

**Returns:** `bool` — tuple(line3D is valid: true/false,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.ConvertTo3D)

#### `ConvertToBSpline2D(arc3D) | ConvertToBSpline2D(spline3D)`

Converts Arc3D to BSpline2D

**Remarks:** Converts Arc3D to BSpline2D

**Parameters:**
- `arc3D` (Arc3D) — Arc3D to convert

**Returns:** `bool` — tuple(bspline2D is valid: true/false,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.ConvertToBSpline2D)

#### `CreateBRep3D(polyhedron) | CreateBRep3D(brep, faceindex)`

Create BRep3D by converting a polyhedron

**Remarks:** Create BRep3D by converting a polyhedron

**Parameters:**
- `polyhedron` (Polyhedron3D) — polyhedron to convert

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreateBRep3D)

#### `CreateFrustumOfPyramid(length, width, height, offset, bottomPlane)`

) -> tuple : Create a frustum of pyramid

**Remarks:** ) -> tuple : Create a frustum of pyramid

**Parameters:**
- `length` (float) — Length of the bottom
- `width` (float) — Width of the bottom
- `height` (float) — Height of the bottom
- `offset` (float) — Offset of the top
- `bottomPlane` (Plane3D) — Bottom plane

**Returns:** `tuple` — Error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreateFrustumOfPyramid)

#### `CreateLoftedBRep3D(outerProfiles_object, innerProfiles_object, closecaps, createprofileedges, linear, periodic)`

Create BRep3D by means of lofting through a set of profiles (incl. hole)

**Remarks:** Create BRep3D by means of lofting through a set of profiles (incl. hole)

**Parameters:**
- `outerProfiles_object` (list) — outer profiles to loft
- `innerProfiles_object` (list) — inner profiles to loft (represents hole in outer profile)
- `closecaps` (bool) — close brep if possible (all profiles are closed)
- `createprofileedges` (bool) — create profile edges and more surfaces or just create one surface
- `linear` (bool) — linear or cubic interpolation
- `periodic` (bool) — if the start profile is also to be the end profile

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreateLoftedBRep3D)

#### `CreatePatchBRep3D(curves_object)`

Create breps as patch from given closed 3D-curves

**Remarks:** Create breps as patch from given closed 3D-curves

**Parameters:**
- `curves_object` (list) — vector of 3D-curves

**Returns:** `eCreatePatchResult` — tuple(Result of patch creation,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePatchBRep3D)

#### `CreatePlanarBRep3D(profiles_object) | CreatePlanarBRep3D(icurve_object)`

Create brep as planar surface

**Remarks:** Create brep as planar surface

**Parameters:**
- `profiles_object` (list) — border of planar surface ( must be closed )

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePlanarBRep3D)

#### `CreatePlanarSurface(geometries_object)`

create planar brep

**Remarks:** create planar brep

**Parameters:**
- `geometries_object` (list) — input geometries pointers

**Returns:** `ePlanarSurfaceError` — tuple(surface error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePlanarSurface)

#### `CreatePolygon3D(polyhedron, faceIndex) | CreatePolygon3D(polygon2D, plane) | CreatePolygon3D(polyline)`

Creates a Polygon3D element from a Polyhedron face

**Remarks:** Creates a Polygon3D element from a Polyhedron face

**Parameters:**
- `polyhedron` (Polyhedron3D) — Polyhedron to get face from
- `faceIndex` (int) — Index of the face in polyhedron

**Returns:** `tuple` — eOK if successful, else eError,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePolygon3D)

#### `CreatePolygon3DFromIndex(arg2, arg3, arg4)`

Create a 3D polygon from a start and end polygon, an index and a division count

**Remarks:** Create a 3D polygon from a start and end polygon, an index and a division count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePolygon3DFromIndex)

#### `CreatePolyhedron(polyhedronType, verticesCount, edgeCount, faceCount, negativeOrientation) | CreatePolyhedron(solid) | CreatePolyhedron(solid) | CreatePolyhedron(cylinder, countOfSegments) | CreatePolyhedron(igeo, options) | CreatePolyhedron(brep, options) | CreatePolyhedron(polygon) | CreatePolyhedron(line) | CreatePolyhedron(lines) | CreatePolyhedron(polyline) | CreatePolyhedron(base, path) | CreatePolyhedron(base, refPoint, path) | CreatePolyhedron(polygon, bottomPlane, topPlane) | CreatePolyhedron(baseOutline, leftOffset, rightOffset, frontOffset, backOffset, bottomPlane, topPlane) | CreatePolyhedron(startPolygon, endPolygon)`

Create and initialize new Polyhedron3D

**Remarks:** Create and initialize new Polyhedron3D

**Parameters:**
- `polyhedronType` (PolyhedronType) — Polyhedron type - edges, faces or volume.
- `verticesCount` (int) — Count of expected vertices.
- `edgeCount` (int) — Count of expected edges.
- `faceCount` (int) — Count of expected faces.
- `negativeOrientation` (bool) — True for negative orientation.

**Returns:** `tuple` — Error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePolyhedron)

#### `CreatePolyline3D(polyhedron) | CreatePolyline3D(polyline2D) | CreatePolyline3D(polygon)`

Creates a Polyline3D element from a Polyhedron

**Remarks:** Creates a Polyline3D element from a Polyhedron

**Parameters:**
- `polyhedron` (Polyhedron3D) — Polyhedron to create line from

**Returns:** `tuple` — eOK if converted successful, else eError,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePolyline3D)

#### `CreatePolyline3DFromIndex(startPol, endPol, index, count)`

Create a 3D polyline from a start and end polyline, an index and a division count Args: startPol: Start polyline endPol: End polyline index: Division index count: Division count

**Remarks:** Create a 3D polyline from a start and end polyline, an index and a division count Args: startPol: Start polyline endPol: End polyline index: Division index count: Division count

**Returns:** `Polyline3D` — eOK if converted successful, else eError,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreatePolyline3DFromIndex)

#### `CreateRailSweptBRep3D(profiles_object, rails_object, closecaps, uniformScaling, railrotation) | CreateRailSweptBRep3D(profiles_object, rails_object, path_object, closecaps, uniformScaling, railrotation, proportionalVertexMatch)`

Create BRep3D by rail sweeping of profiles. Rails must start/ends on profiles start/end points

**Remarks:** Create BRep3D by rail sweeping of profiles. Rails must start/ends on profiles start/end points

**Parameters:**
- `profiles_object` (list) — profiles to sweep
- `rails_object` (list) — rails to control sweeping
- `closecaps` (bool) — if true, create closed solid if possible, if false just surface(s) will be created
- `uniformScaling` (bool) — use uniform scaling for profiles along the rails
- `railrotation` (bool) — use rotation the shape to maintain a constant angle with the rail

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreateRailSweptBRep3D)

#### `CreateRevolvedBRep3D(profiles_object, axis, rotationAngle, closecaps, numprofiles)`

Create brep as revolved body.

**Remarks:** Create brep as revolved body.

**Parameters:**
- `profiles_object` (list) — vector of profile curves
- `axis` (Axis3D) — axis of rotation
- `rotationAngle` (Angle) — angle of rotation
- `closecaps` (bool) — if true, create closed solid if possible, if false just surface(s) will be created
- `numprofiles` (int) — number of control profiles created along the created surfaces (division of surface parameter)

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreateRevolvedBRep3D)

#### `CreateSweptBRep3D(profiles_object, path_object, closecaps, railrotation, rotAxis=None, numprofiles=0) | CreateSweptBRep3D(profiles_object, path_object, closecaps, railrotation, rotAxis=None, numprofiles=0) | CreateSweptBRep3D(profile_object, path_object, railrotation, rotAxis=None)`

Create BRep3D by means of a vector of profiles extrusion along a path

**Remarks:** Create BRep3D by means of a vector of profiles extrusion along a path

**Parameters:**
- `profiles_object` (list) — profiles to extrude, if closed solid will be created, otherwise a surface only
- `path_object` (object) — path to extrude along
- `closecaps` (bool) — if true, create closed solid if possible, if false just surface(s) will be created
- `railrotation` (SweepRotationType) — if true, rotate profile along the path
- `rotAxis` (Optional[Vector3D]) — if set, profile will be rotated along this axis along the path
- `numprofiles` (int) — number of control profiles created along the created surfaces (divisionn of surface parameter)

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreateSweptBRep3D)

#### `CreateSweptPolyhedron3D(profiles, path, closecaps, railrotation, rotAxis)`

Create translation polyhedron from base polygon and path

**Remarks:** Create translation polyhedron from base polygon and path

**Parameters:**
- `profiles` (Polyline3DList) — profiles to sweep
- `path` (Polyline3D) — translation path
- `closecaps` (bool) — create closed volume (in case of closed profiles) or open shell only
- `railrotation` (bool) — rotate profiles along path - standard rotation = true (keep angle between profile and a path), rotation along z-axis vector = false
- `rotAxis` (Vector3D) — if set, profile will be rotated along this axis along the path

**Returns:** `tuple` — Error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CreateSweptPolyhedron3D)

#### `CutBrepWithPlane(original, plane)`

Cut Brep with plane - create two independent BReps

**Remarks:** Cut Brep with plane - create two independent BReps

**Parameters:**
- `original` (BRep3D) — original brep
- `plane` (Plane3D) — cutting plane

**Returns:** `bool` — tuple(true if plane cuts the brep. If the plane is above or below brep, it returns false, but one of the result brep is filled with original according to plane position - use brep.IsValid() to check.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CutBrepWithPlane)

#### `CutPolyhedronWithPlane(original, plane)`

Cut polyhedron with plane - create two independent polyhedra

**Remarks:** Cut polyhedron with plane - create two independent polyhedra

**Parameters:**
- `original` (Polyhedron3D) — original polyhedron
- `plane` (Plane3D) — cutting plane

**Returns:** `bool` — tuple(true if there is a cut, otherwise false, one of polyhedron above/below is filled when whole body is above/below the plane,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.CutPolyhedronWithPlane)

#### `DeletePolyhedronLastFace(polyhedron)`

Delete the last face of given polyhedron

**Remarks:** Delete the last face of given polyhedron

**Parameters:**
- `polyhedron` (Polyhedron3D) — Polyhedron3D

**Returns:** `object` — Error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.DeletePolyhedronLastFace)

#### `FindMinDistancePoint(point, bspline) | FindMinDistancePoint(point, brep)`

Find minimal distance point between Point3D and BSpline3D

**Remarks:** Find minimal distance point between Point3D and BSpline3D

**Parameters:**
- `point` (Point3D) — Point3D
- `bspline` (BSpline3D) — BSpline3D

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.FindMinDistancePoint)

#### `GetAbsoluteTolerance()`

get absolute tolerance

**Remarks:** get absolute tolerance

**Returns:** `float` — absolute tolerance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetAbsoluteTolerance)

#### `GetAllIntersections(geoObject1, geoObject2, eps, maxSolutions)`

Calculate all intersections of two geometry objects Intersection point must not be located on the elements, it can be outside

**Remarks:** Calculate all intersections of two geometry objects Intersection point must not be located on the elements, it can be outside

**Parameters:**
- `geoObject1` (object) — First object
- `geoObject2` (object) — Second object
- `eps` (float) — Tolerance
- `maxSolutions` (int) — Maximum number of solution (especially for splines and clothoids)

**Returns:** `tuple` — true, if an intersection was found, otherwise false,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetAllIntersections)

#### `GetAngleTolerance()`

Get angle tolerance This tolerance is using for angle comparison. Use it with Comparison::Equal(value, value, GetAngleTolerance())

**Remarks:** Get angle tolerance This tolerance is using for angle comparison. Use it with Comparison::Equal(value, value, GetAngleTolerance())

**Returns:** `Angle` — Tolerance using for angle comparison [rad]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetAngleTolerance)

#### `GetClosestIntersection(geometrie1, geometrie2, inputPoint, eps, bOnlyInsidePoints)`

Function to calculate the nearest intersection between two geometries to a reference point Intersection point must not be located on the elements, it can be outside

**Remarks:** Function to calculate the nearest intersection between two geometries to a reference point Intersection point must not be located on the elements, it can be outside

**Parameters:**
- `geometrie1` (object) — First object
- `geometrie2` (object) — Second object
- `inputPoint` (Point3D) — Reference Point / Cursor point
- `eps` (float) — Tolerance
- `bOnlyInsidePoints` (bool) — Return point must be on both objects

**Returns:** `tuple` — true, if an intersection was found, otherwise false,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetClosestIntersection)

#### `GetCurvatureTolerance()`

Get curvature tolerance This tolerance is using for curvature comparison. Use it with Comparison::Equal(value, value, GetCurvatureTolerance())

**Remarks:** Get curvature tolerance This tolerance is using for curvature comparison. Use it with Comparison::Equal(value, value, GetCurvatureTolerance())

**Returns:** `float` — Tolerance using for curvature comparison

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetCurvatureTolerance)

#### `GetCurveLengthTolerance()`

Get tolerance for curve length comparison This tolerance is using for length of curve comparison. Use it with Comparison::Equal(value, value, GetCurveLengthTolerance())

**Remarks:** Get tolerance for curve length comparison This tolerance is using for length of curve comparison. Use it with Comparison::Equal(value, value, GetCurveLengthTolerance())

**Returns:** `float` — Tolerance using for length of curve comparison [mm]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetCurveLengthTolerance)

#### `GetIntersectionPoints(geoObject1, geoObject2, eps)`

Function to calculate the nearest intersection between two geometries to a reference point Intersection point must not be located on the elements, it can be outside

**Remarks:** Function to calculate the nearest intersection between two geometries to a reference point Intersection point must not be located on the elements, it can be outside

**Parameters:**
- `geoObject1` (object) — First object
- `geoObject2` (object) — Second object
- `eps` (float) — Tolerance (optional)

**Returns:** `tuple` — true, if an intersection was found, otherwise false,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetIntersectionPoints)

#### `GetRelativeTolerance()`

get relative tolerance

**Remarks:** get relative tolerance

**Returns:** `float` — relative tolerance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetRelativeTolerance)

#### `GetRotationMatrix(zeroPoint, angle) | GetRotationMatrix(axis, angle) | GetRotationMatrix(line, angle)`

Creates 2D rotation matrix

**Remarks:** Creates 2D rotation matrix

**Parameters:**
- `zeroPoint` (Point2D) — Rotation point
- `angle` (Angle) — Rotation angle

**Returns:** `Matrix2D` — 2D rotation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GetRotationMatrix)

#### `GroundViewHiddenCalculation(brep) | GroundViewHiddenCalculation(poly)`

Hidden calculation with ground view of BRep

**Remarks:** Hidden calculation with ground view of BRep

**Parameters:**
- `brep` (BRep3D) — BRep

**Returns:** `eGeometryErrorCode` — tuple(Error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.GroundViewHiddenCalculation)

#### `ImprintProfileOnFaces(brep, targetFaces, profile_object)`

Imprint edges on brep

**Remarks:** Imprint edges on brep

**Parameters:**
- `brep` (BRep3D) — Brep
- `targetFaces` ([list[int] | VecUIntList]) — faces where to imprint edges
- `profile_object` (Union[Arc3D, BSpline3D, Line3D, Path3D, Polyline3D, Polygon3D, Spline3D]) — imprinted profile

**Returns:** `eGeometryErrorCode` — tuple(eOK if successful otherwise eError,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.ImprintProfileOnFaces)

#### `Intersect(bodies1_list, bodies2_list) | Intersect(el1, el2) | Intersect(el1, el2) | Intersect(brep1, brep2) | Intersect(brep, polyhedron)`

compute intersection of set of bodies

**Remarks:** compute intersection of set of bodies

**Parameters:**
- `bodies1_list` (PolyhedronTypesList) — list of bodies
- `bodies2_list` (PolyhedronTypesList) — list of bodies

**Returns:** `bool` — tuple(true when intersecting, otherwise false.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Intersect)

#### `IntersectRayBRep(rayPoint, rayVector, brep, bNegativPrefered)`

Intersection of Ray and BRep

**Remarks:** Intersection of Ray and BRep

**Parameters:**
- `rayPoint` (Point3D) — ray Point
- `rayVector` (Vector3D) — direction vector of ray
- `brep` (BRep3D) — BRep
- `bNegativPrefered` (bool) — if true -> return first solution when ray point is out of object

**Returns:** `int` — tuple(0 if no error occurs,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.IntersectRayBRep)

#### `IntersectRayPolyhedron(rayPoint, rayVector, polyhedron, flag, selFaces=None)`

Intersection of Ray and Polyhedron

**Remarks:** Intersection of Ray and Polyhedron

**Parameters:**
- `rayPoint` (Point3D) — ray Point
- `rayVector` (Vector3D) — direction vector of ray
- `polyhedron` (Polyhedron3D) — polyhedron
- `flag` (IntersectRayPolyhedronFlag) — flag for determining the "best" intersection
- `selFaces` (Optional[[list[int] | VecIntList]]) — optional list of faces used for intersection, if default value is used, all faces are checked

**Returns:** `int` — tuple(0 if no error occured,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.IntersectRayPolyhedron)

#### `Intersecting(el1, el2) | Intersecting(el1, el2, tol) | Intersecting(el1, el2, tol) | Intersecting(el1, el2) | Intersecting(el1, el2) | Intersecting(el1, el2) | Intersecting(el1, el2) | Intersecting(el1, el2) | Intersecting(el1, el2) | Intersecting(el1, el2) | Intersecting(el1, el2)`

test 2 2D minmaxes for intersection

**Remarks:** test 2 2D minmaxes for intersection

**Parameters:**
- `el1` (MinMax2D) — element
- `el2` (MinMax2D) — element

**Returns:** `bool` — true when intersecting, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Intersecting)

#### `IntersectingRel(el1, el2) | IntersectingRel(el1, el2)`

test 2 geo 2D minmaxes for intersection using built-in relative tolerance

**Remarks:** test 2 geo 2D minmaxes for intersection using built-in relative tolerance

**Parameters:**
- `el1` (MinMax2D) — element
- `el2` (MinMax2D) — element

**Returns:** `bool` — true when intersecting, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.IntersectingRel)

#### `IntersectionCalculus(el1, el2) | IntersectionCalculus(axis, line) | IntersectionCalculus(axis2D, axis3D) | IntersectionCalculus(axis2D, arc2D, eps) | IntersectionCalculus(axis2D, polyline) | IntersectionCalculus(axis2D, polygon) | IntersectionCalculus(axis2D, spline, eps, maxSolutions) | IntersectionCalculus(axis2D, bspline, eps, maxSolutions) | IntersectionCalculus(axis1, axis2) | IntersectionCalculus(axis, line) | IntersectionCalculus(axis, arc, eps) | IntersectionCalculus(axis, polyline) | IntersectionCalculus(axis, polygon) | IntersectionCalculus(axis, spline, eps, maxSolutions) | IntersectionCalculus(axis, spline, eps, maxSolutions) | IntersectionCalculus(axis, spline, eps, maxSolutions) | IntersectionCalculus(axis, plane, eps) | IntersectionCalculus(line, plane, eps) | IntersectionCalculus(axis, arc, eps) | IntersectionCalculus(line, arc, eps) | IntersectionCalculus(el1, el2) | IntersectionCalculus(line2D, line3D) | IntersectionCalculus(line2D, arc2D, eps) | IntersectionCalculus(line2D, polyline) | IntersectionCalculus(line2D, polygon) | IntersectionCalculus(line, bspline, eps, maxSolutions) | IntersectionCalculus(line2D, spline, eps, maxSolutions) | IntersectionCalculus(line1, line2) | IntersectionCalculus(line, arc, eps) | IntersectionCalculus(line, polyline) | IntersectionCalculus(line, polygon) | IntersectionCalculus(line, spline, eps, maxSolutions) | IntersectionCalculus(line, spline, eps, maxSolutions) | IntersectionCalculus(line, bspline, eps, maxSolutions) | IntersectionCalculus(arc1, arc2) | IntersectionCalculus(arc, polyline, eps) | IntersectionCalculus(arc, polygon, intersectionPnts) | IntersectionCalculus(arc, spline, eps, maxSolutions) | IntersectionCalculus(arc, bspline, eps, maxSolutions) | IntersectionCalculus(arc, arc2, eps, maxSolutions) | IntersectionCalculus(arc, spline, eps, maxSolutions) | IntersectionCalculus(arc, bspline, eps, maxSolutions) | IntersectionCalculus(polyline1, polyline2) | IntersectionCalculus(polyline, polygon) | IntersectionCalculus(polygon1, polygon2) | IntersectionCalculus(polyline, spline, eps, maxSolutions) | IntersectionCalculus(polyline, spline, eps, maxSolutions) | IntersectionCalculus(polyline, spline, eps, maxSolutions) | IntersectionCalculus(polygon, spline, eps, maxSolutions) | IntersectionCalculus(plane, line, eps) | IntersectionCalculus(plane, line, eps) | IntersectionCalculus(plane, arc, eps) | IntersectionCalculus(plane, spline, eps, maxSolutions) | IntersectionCalculus(spline1, spline2, eps, maxSolutions) | IntersectionCalculus(spline1, spline2, eps, maxSolutions) | IntersectionCalculus(spline1, spline2, eps, maxSolutions) | IntersectionCalculus(spline1, spline2, eps, maxSolutions) | IntersectionCalculus(spline1, spline2, eps, maxSolutions) | IntersectionCalculus(spline1, spline2, eps, maxSolutions) | IntersectionCalculus(spline1, spline2, eps, maxSolutions) | IntersectionCalculus(ele1, ele2, eps, maxSolutions)`

compute intersection of 2 2D axis

**Remarks:** compute intersection of 2 2D axis

**Parameters:**
- `el1` (Axis2D) — element
- `el2` (Axis2D) — element

**Returns:** `tuple` — true when intersecting, otherwise false.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.IntersectionCalculus)

#### `IntersectionCalculusEx(axis2D, arc2D, eps) | IntersectionCalculusEx(axis2D, polyline) | IntersectionCalculusEx(axis2D, polygon) | IntersectionCalculusEx(axis2D, clothoid2D, eps, maxSolutions) | IntersectionCalculusEx(axis, arc, eps) | IntersectionCalculusEx(axis, polyline) | IntersectionCalculusEx(axis, polygon) | IntersectionCalculusEx(line1, line2) | IntersectionCalculusEx(line2D, line3D) | IntersectionCalculusEx(line2D, arc2D, eps) | IntersectionCalculusEx(line2D, clothoid2D, eps, maxSolutions) | IntersectionCalculusEx(line2D, polyline) | IntersectionCalculusEx(line2D, polygon) | IntersectionCalculusEx(line2D, spline, eps, maxSolutions) | IntersectionCalculusEx(line2D, spline, eps, maxSolutions) | IntersectionCalculusEx(line2D, bspline, eps, maxSolutions) | IntersectionCalculusEx(line1, line2) | IntersectionCalculusEx(line, arc, eps) | IntersectionCalculusEx(line, arc, eps) | IntersectionCalculusEx(line, clothoid, eps, maxSolutions) | IntersectionCalculusEx(line, polyline) | IntersectionCalculusEx(line, polygon) | IntersectionCalculusEx(line, spline, eps, maxSolutions) | IntersectionCalculusEx(line, spline, eps, maxSolutions) | IntersectionCalculusEx(line, spline, eps, maxSolutions) | IntersectionCalculusEx(arc1, arc2) | IntersectionCalculusEx(arc, clothoid, eps, maxSolutions) | IntersectionCalculusEx(arc, polyline, eps) | IntersectionCalculusEx(arc, polygon, intersectionPnts) | IntersectionCalculusEx(arc, spline, eps, maxSolutions) | IntersectionCalculusEx(arc, spline, eps, maxSolutions) | IntersectionCalculusEx(arc1, arc2, eps, maxSolutions) | IntersectionCalculusEx(arc, spline, eps, maxSolutions) | IntersectionCalculusEx(arc, spline, eps, maxSolutions) | IntersectionCalculusEx(clothoid, polyline, eps, maxSolutions) | IntersectionCalculusEx(clothoid, polygon, eps, maxSolutions) | IntersectionCalculusEx(polyline1, polyline2) | IntersectionCalculusEx(polygon1, polygon2) | IntersectionCalculusEx(polyline, spline, eps, maxSolutions) | IntersectionCalculusEx(polygon, spline, eps, maxSolutions) | IntersectionCalculusEx(geoObject1, geoObject2, eps, maxSolutions, bOnlyInsidePoints)`

Calculate the intersection between a 2D axis and a 2D arc Intersection point must not be located on the elements, it can be outside emarks There are known precision issues with the background function lksplk

**Remarks:** Calculate the intersection between a 2D axis and a 2D arc Intersection point must not be located on the elements, it can be outside emarks There are known precision issues with the background function lksplk

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.IntersectionCalculusEx)

#### `IsCoplanar(geoVector_object) | IsCoplanar(igeo1_object, igeo2_object) | IsCoplanar(plane, geo_object) | IsCoplanar(plane, point) | IsCoplanar(plane, line) | IsCoplanar(plane1, plane2) | IsCoplanar(point1, point2, point3, point4) | IsCoplanar(line1, line2) | IsCoplanar(line1, line2)`

Check if the 3D geometry objects are coplanar and calculate plane

**Remarks:** Check if the 3D geometry objects are coplanar and calculate plane

**Parameters:**
- `geoVector_object` (list) — vector of object geometries

**Returns:** `bool` — tuple(True if objects are coplanar, false otherwise.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.IsCoplanar)

#### `MakeBoolean(el1, el2) | MakeBoolean(el1, el2) | MakeBoolean(el1, el2)`

make boolean operation between line and polygon

**Remarks:** make boolean operation between line and polygon

**Parameters:**
- `el1` (Line2D) — Line2D
- `el2` (Polygon2D) — Polygon2D

**Returns:** `eGeometryErrorCode` — tuple(GeoErrorCode,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.MakeBoolean)

#### `MakeIntersection(brep1, breps) | MakeIntersection(brep1, brep2) | MakeIntersection(brep, polyhedron) | MakeIntersection(el1, el2)`

Compute intersection of 2 breps

**Remarks:** Compute intersection of 2 breps

**Parameters:**
- `brep1` (BRep3D) — first brep
- `breps` (BRep3DList) — second brep

**Returns:** `eGeometryErrorCode` — tuple(eOK if intersection was successful,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.MakeIntersection)

#### `MakeSectionWithSurfaces(brep1, brep2)`

Compute section of brep and surface brep

**Remarks:** Compute section of brep and surface brep

**Parameters:**
- `brep1` (BRep3D) — first brep
- `brep2` (BRep3D) — section brep

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.MakeSectionWithSurfaces)

#### `MakeSubtraction(body_object, voidbodies_list) | MakeSubtraction(poly1, poly2) | MakeSubtraction(polyhed1, polyhed2) | MakeSubtraction(brep1, brep2) | MakeSubtraction(brep1, breps) | MakeSubtraction(poly1, poly2, intersection)`

make subtraction of 3D bodies from given body

**Remarks:** make subtraction of 3D bodies from given body

**Parameters:**
- `body_object` (Union[Polyhedron3D, Cylinder3D, ExtrudedAreaSolid3D, Line3D, Polyline3D, Polygon3D, ClippedSweptSolid3D]) — input body
- `voidbodies_list` (PolyhedronTypesList) — list of 3D bodies to subtract

**Returns:** `eGeometryErrorCode` — tuple(GeoErrorCode,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.MakeSubtraction)

#### `MakeUnion(bodies_list, voidbodies_list) | MakeUnion(poly1, poly2) | MakeUnion(polyhed1, polyhed2) | MakeUnion(bodies) | MakeUnion(brep1, brep2) | MakeUnion(brep1, breps) | MakeUnion(breps)`

make union of 3D bodies As input arbitrary elements which represents 3D is possible.

**Remarks:** make union of 3D bodies As input arbitrary elements which represents 3D is possible.

**Parameters:**
- `bodies_list` (PolyhedronTypesList) — list of 3D bodies
- `voidbodies_list` (PolyhedronTypesList) — list of 3D bodies to subtract

**Returns:** `eGeometryErrorCode` — tuple(GeoErrorCode,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.MakeUnion)

#### `Mirror(point, plane) | Mirror(point, axis) | Mirror(vec, plane) | Mirror(vec, axis) | Mirror(point, plane) | Mirror(point, axis) | Mirror(vec, plane) | Mirror(vec, axis) | Mirror(line, plane) | Mirror(line, axis) | Mirror(line, plane) | Mirror(line, axis) | Mirror(polyline, plane) | Mirror(polyline, axis) | Mirror(polygon, plane) | Mirror(polygon, axis) | Mirror(plane, axis) | Mirror(pla, plane) | Mirror(polygon, axis) | Mirror(polygon, axis) | Mirror(polygon, plane) | Mirror(polyline, plane) | Mirror(polyline, axis) | Mirror(polygon, plane) | Mirror(polygon, axis) | Mirror(spline, plane) | Mirror(spline, axis) | Mirror(spline, plane) | Mirror(spline, axis) | Mirror(bSpline, plane) | Mirror(bSpline, axis) | Mirror(bSpline, plane) | Mirror(bSpline, axis) | Mirror(arc, plane) | Mirror(arc, axis) | Mirror(arc, plane) | Mirror(arc, axis) | Mirror(path, plane) | Mirror(path, axis) | Mirror(path, plane) | Mirror(path, axis) | Mirror(area, plane) | Mirror(area, axis) | Mirror(area, plane) | Mirror(area, axis) | Mirror(angle, plane) | Mirror(angle, axis) | Mirror(polyhedron, plane) | Mirror(polyhedron, axis) | Mirror(solid, axis) | Mirror(solid, plane) | Mirror(solid, axis) | Mirror(clothoid, plane) | Mirror(clothoid, axis) | Mirror(placement, axis) | Mirror(placement, plane) | Mirror(placement, axis) | Mirror(placement, plane) | Mirror(cylinder, plane) | Mirror(cylinder, axis) | Mirror(cone, plane) | Mirror(cone, axis) | Mirror(ellipsoid, plane) | Mirror(ellipsoid, axis) | Mirror(brep, plane)`

Mirror point 3D

**Remarks:** Mirror point 3D

**Parameters:**
- `point` (Point3D) — mirroring point
- `plane` (Plane3D) — mirror plane

**Returns:** `Point3D` — mirrored point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Mirror)

#### `Move(box, translation) | Move(area, translation) | Move(plane, translation) | Move(arc2D, moveVector) | Move(arc3D, moveVector) | Move(axis2D, moveVector) | Move(axis3D, moveVector) | Move(axisPlacement2D, moveVector) | Move(axisPlacement3D, moveVector) | Move(clippedSweptSolid3D, vec) | Move(clothoid2D, moveVector) | Move(cuboid3D, moveVector) | Move(extrudedAreaSolid3D, moveVector) | Move(line2D, moveVector) | Move(line3D, moveVector) | Move(path2D, moveVector) | Move(path3D, moveVector) | Move(point2D, moveVector) | Move(point3D, moveVector) | Move(polygon2D, moveVector) | Move(polygon3D, moveVector) | Move(polygonalArea3D, moveVector) | Move(polyhedron3D, moveVector) | Move(polyline2D, moveVector) | Move(polyline3D, moveVector) | Move(area, moveVector) | Move(area, moveVector) | Move(cylinder3D, moveVector) | Move(ellipsoid3D, moveVector) | Move(cone3D, moveVector) | Move(spline2D, moveVector) | Move(spline3D, moveVector) | Move(bspline3D, moveVector) | Move(brep, moveVector)`

Move bounding box

**Remarks:** Move bounding box

**Parameters:**
- `box` (BoundingBox2D) — Moving bounding box
- `translation` (Vector2D) — Vector of translation

**Returns:** `BoundingBox2D` — Moved bounding box

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Move)

#### `MoveArc3DToZ0Plane(arc3D)`

Move the given 3D arc on the z = 0 plane and create a 2D fromn it

**Remarks:** Move the given 3D arc on the z = 0 plane and create a 2D fromn it

**Parameters:**
- `arc3D` (Arc3D) — Arc 3D

**Returns:** `eServiceResult` — tuple(NO_ERR or INVALID_GEOOBJECT,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.MoveArc3DToZ0Plane)

#### `MoveSpline3DToZ0Plane(spline3D)`

Move the given 3D arc on the z = 0 plane and create a 2D from it

**Remarks:** Move the given 3D arc on the z = 0 plane and create a 2D from it

**Parameters:**
- `spline3D` (Spline3D) — Spline 3D

**Returns:** `eServiceResult` — tuple(NO_ERR or INVALID_GEOOBJECT,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.MoveSpline3DToZ0Plane)

#### `Offset(dist, lineSrc) | Offset(point, lineSrc) | Offset(point, pathSrc) | Offset(dist, pathSrc, checkSegmentsOrientation) | Offset(point, arc2DSrc) | Offset(dist, arc2DSrc) | Offset(dist, spline2DSrc, checkSegmentsOrientation) | Offset(point, spline2DSrc) | Offset(rDistance, polyline2D, checkSegmentsOrientation) | Offset(point, polyline2DSrc) | Offset(point, dist, lineSrc, offsetPlane) | Offset(dist, offsetPlane, parallelsCount, polySrc) | Offset(point, offsetPlane, parallelsCount, polySrc) | Offset(point, arc3DSrc) | Offset(dist, arc3DSrc) | Offset(dist, spline3DSrc, plane, checkSegmentsOrientation) | Offset(point, spline3DSrc, plane) | Offset(inputPath, normVector, offsetDistance) | Offset(inputPath, inputPoint, rayVector, plane)`

Counts parallel to 2D-Line

**Remarks:** Counts parallel to 2D-Line

**Parameters:**
- `dist` (float) — Distance from the source
- `lineSrc` (Line2D) — Source element

**Returns:** `tuple` — eOK if successful,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Offset)

#### `OffsetCurve(inputCurves, normVector, offsetDistance, sortEdges)`

Offset 3D curves with given distance

**Remarks:** Offset 3D curves with given distance

**Parameters:**
- `inputCurves` (list) — input curves
- `normVector` (Vector3D) — normal vector of plane in that the offset will be calculated
- `offsetDistance` (float) — offset distance
- `sortEdges` (bool) — flag if to sort edges to original order

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.OffsetCurve)

#### `Polygonize(geometry_object, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(geometry_object, fromPoint, toPoint, arcSegmentation, useArcSegmentation, armLength, riseValue, eps) | Polygonize(path, fromPoint, toPoint, arcSegmentation, useArcSegmentation, armLength, riseValue, eps) | Polygonize(line, fromPoint, toPoint, eps) | Polygonize(polyline, fromPoint, toPoint, eps) | Polygonize(polygon, fromPoint, toPoint, eps) | Polygonize(arc, fromPoint, toPoint, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(arc, fromPoint, toPoint, count, bInscribed) | Polygonize(arc, countOfSegments) | Polygonize(spline, fromPoint, toPoint, arcSegmentation, useArcSegmentation, armLength, riseValue, eps) | Polygonize(bspline, fromPoint, toPoint, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(clothoid, fromPoint, toPoint, arcSegmentation, useArcSegmentation, armLength, riseValue, eps) | Polygonize(arc, countOfSegments) | Polygonize(arc, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(path, countOfSegments) | Polygonize(path, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(spline, linesPerSegment) | Polygonize(spline, minmaxRadius, pixelSize) | Polygonize(spline, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(spline, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(line) | Polygonize(line, fromOffset, toOffset) | Polygonize(clothoid, fromOffset, toOffset, arcSegmentation, useArcSegmentation, armLength, riseValue, eps) | Polygonize(polyline, fromOffset, toOffset, eps) | Polygonize(polygon, fromOffset, toOffset, eps) | Polygonize(arc, fromOffset, toOffset, arcSegmentation, useArcSegmentation, armLength, riseValue) | Polygonize(arc, fromOffset, toOffset, count, bInscribed) | Polygonize(spline, fromOffset, toOffset, arcSegmentation, useArcSegmentation, armLength, riseValue, eps) | Polygonize(path, fromOffset, toOffset, arcSegmentation, useArcSegmentation, armLength, riseValue, eps) | Polygonize(area, eps, arcSegmentation=360, useArcSegmentation=0, armLength=0.0, riseValue=0.0) | Polygonize(geometry_object, fromOffset, toOffset, arcSegmentation, useArcSegmentation, armLength, riseValue, eps)`

Polygonize whole curve

**Remarks:** Polygonize whole curve

**Parameters:**
- `geometry_object` (object) — Geometry object which will be polygonized
- `arcSegmentation` (int) — segment number
- `useArcSegmentation` (int) — if use segment polygonization, set to 0
- `armLength` (float) — max length of calculated segment, for rise value polygonization only
- `riseValue` (float) — rise value

**Returns:** `bool` — tuple(bool true = success,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Polygonize)

#### `PolygonizeEqually(arc, fromPoint, toPoint, count)`

Polygonize given arc always equally

**Remarks:** Polygonize given arc always equally

**Parameters:**
- `arc` (Arc2D) — Source arc
- `fromPoint` (Point2D) — Start point for polygonization
- `toPoint` (Point2D) — End point for polygonization
- `count` (int) — Count of segments

**Returns:** `bool` — tuple(bool (true = polygonization possible),

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.PolygonizeEqually)

#### `Rotate(point, angle) | Rotate(point, zeroPoint, angle) | Rotate(line, angle) | Rotate(line, zeroPoint, angle) | Rotate(arc, angle) | Rotate(arc, zeroPoint, angle) | Rotate(vec, angle) | Rotate(vec, axis, angle) | Rotate(polyline, zeroPoint, angle) | Rotate(polyline, angle) | Rotate(axis, angle) | Rotate(axis, zeroPoint, angle) | Rotate(axis, angle) | Rotate(axis, zeroPoint, angle) | Rotate(placement, angle) | Rotate(placement, rotAxis, angle) | Rotate(placement, zeroPoint, angle) | Rotate(clothoid, angle) | Rotate(clothoid, zeroPoint, angle) | Rotate(polygon, angle) | Rotate(polygon, zeroPoint, angle) | Rotate(area, angle) | Rotate(area, zeroPoint, angle) | Rotate(spline, angle) | Rotate(spline, zeroPoint, angle) | Rotate(point, angle) | Rotate(point, zeroPoint, angle) | Rotate(point, axis, angle) | Rotate(line, zeroPoint, angle) | Rotate(cylinder, angle) | Rotate(cylinder, zeroPoint, angle) | Rotate(cylinder, axis, angle) | Rotate(ellipsoid, angle) | Rotate(ellipsoid, zeroPoint, angle) | Rotate(ellipsoid, axis, angle) | Rotate(cone, angle) | Rotate(cone, zeroPoint, angle) | Rotate(cone, axis, angle) | Rotate(minmax, angle) | Rotate(brep, axis, angle) | Rotate(polyhedron, axis, angle)`

Rotate a point around zero point by an angle.

**Remarks:** Rotate a point around zero point by an angle.

**Parameters:**
- `point` (Point2D) — 2D Point
- `angle` (Angle) — Angle

**Returns:** `Point2D` — Resulting 2D Point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Rotate)

#### `SetAbsoluteTolerance(value)`

set absolute tolerance

**Remarks:** set absolute tolerance

**Parameters:**
- `value` (float) — new value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.SetAbsoluteTolerance)

#### `SetAngleTolerance(value)`

Set angle tolerance This tolerance is using for angle comparison.

**Remarks:** Set angle tolerance This tolerance is using for angle comparison.

**Parameters:**
- `value` (Angle) — New tolerance using for angle comparison [rad]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.SetAngleTolerance)

#### `SetCurvatureTolerance(value)`

Set curvature tolerance This tolerance is using for curvature comparison.

**Remarks:** Set curvature tolerance This tolerance is using for curvature comparison.

**Parameters:**
- `value` (float) — New tolerance using for curvature comparison

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.SetCurvatureTolerance)

#### `SetCurveLengthTolerance(value)`

Set curve length tolerance This tolerance is using for length of curve comparison.

**Remarks:** Set curve length tolerance This tolerance is using for length of curve comparison.

**Parameters:**
- `value` (float) — New tolerance using for length of curve comparison [mm]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.SetCurveLengthTolerance)

#### `SetRelativeTolerance(value)`

set relative tolerance

**Remarks:** set relative tolerance

**Parameters:**
- `value` (float) — new value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.SetRelativeTolerance)

#### `SetZValue(zValue, line)`

Set the the z value of a line

**Remarks:** Set the the z value of a line

**Parameters:**
- `zValue` (float) — zValue
- `line` (Line3D) — line

**Returns:** `object` — true, if the calculation was correct

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.SetZValue)

#### `Split(line2D, splitPoints, posTol) | Split(line3D, splitPoints, posTol) | Split(polyline2D, splitPoints, posTol) | Split(polyline3D, splitPoints, posTol) | Split(arc2D, splitPoints, posTol) | Split(arc3D, splitPoints, posTol) | Split(spline2D, splitPoints, posTol) | Split(spline3D, splitPoints, posTol) | Split(bspline2D, splitPoints, posTol) | Split(bspline3D, splitPoints, posTol) | Split(polyline2D, polygon2D, posTol) | Split(polygon, polyline, posTol, divideComponents) | Split(polygon, axis, posTol, divideComponents)`

Split a Line2D into multiple Line2D geometries by given split points

**Remarks:** Split a Line2D into multiple Line2D geometries by given split points

**Parameters:**
- `line2D` (Line2D) — The line to split
- `splitPoints` (Point3DList) — The given split points (e.g. intersection points onto the line)
- `posTol` (float) — Tolerance being used to determine the position of split points on the input curve

**Returns:** `tuple` — eSplitResult, SPLIT_OK on success,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Split)

#### `SplitPolygon3DToParts(polygon)`

Splits normalized Polygon3D into parts

**Remarks:** Splits normalized Polygon3D into parts

**Parameters:**
- `polygon` (Polygon3D) — Normalized 3D polygon

**Returns:** `tuple` — eOK if converted successful, else eError,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.SplitPolygon3DToParts)

#### `Tesselate(brep, density, maxAngle, minEdge, maxEdge) | Tesselate(brep, density, chord, maxAngle, minEdge, maxEdge)`

Tesselate b-rep with mesh triangles

**Remarks:** Tesselate b-rep with mesh triangles

**Parameters:**
- `brep` (BRep3D) — brep to be tesselated
- `density` (float) — density of resulting mesh (when 0, not used)
- `maxAngle` (float) — maximal angle between neighbor triangles in degrees (when 0, not used)
- `minEdge` (float) — minimal edge length (when 0, not used)
- `maxEdge` (float) — maximal edge length (when 0, not used)

**Returns:** `tuple` — error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Tesselate)

#### `Touching(line1, line2) | Touching(polygon1, polygon2) | Touching(polygon, polyline)`

Test for collision between 2 line2D objects

**Remarks:** Test for collision between 2 line2D objects

**Parameters:**
- `line1` (Line2D) — the first geometry object
- `line2` (Line2D) — the second geometry object

**Returns:** `bool` — true when in touch, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Touching)

#### `Transform(point, matrix) | Transform(line, matrix) | Transform(vec, matrix) | Transform(arc, matrix) | Transform(clothoid, matrix) | Transform(spline, matrix) | Transform(axis, matrix) | Transform(polyline, matrix) | Transform(path, matrix) | Transform(area, matrix) | Transform(area, matrix) | Transform(area, matrix) | Transform(point, matrix) | Transform(point, matrix) | Transform(el, matrix) | Transform(el, matrix) | Transform(vec, matrix) | Transform(vec, matrix) | Transform(arc, matrix) | Transform(arc, matrix) | Transform(cuboid, matrix) | Transform(cuboid, matrix) | Transform(plane, matrix) | Transform(plane, matrix) | Transform(polygon, matrix) | Transform(polygon, matrix) | Transform(polygon, matrix) | Transform(polyline, matrix) | Transform(polyline, matrix) | Transform(spline, matrix) | Transform(polyhedron, matrix) | Transform(polyhedron, matrix) | Transform(axis, matrix) | Transform(axis, matrix) | Transform(path, matrix) | Transform(path, matrix) | Transform(area, matrix) | Transform(area, matrix) | Transform(solid, matrix) | Transform(solid, matrix) | Transform(cylinder, matrix) | Transform(cylinder, matrix) | Transform(ellipsoid, matrix) | Transform(ellipsoid, matrix) | Transform(brep, matrix) | Transform(brep, matrix) | Transform(spline, matrix) | Transform(spline, matrix) | Transform(cone, matrix) | Transform(cone, matrix)`

Point2D matrix transformation.

**Remarks:** Point2D matrix transformation.

**Parameters:**
- `point` (Point2D) — 2D point to transform.
- `matrix` (Matrix2D) — Transformation Matrix.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/_functions/#NemAll_Python_Geometry.Transform)

## GeometryEdge (class)

Geometry edge Identification of any edge via point indices, not via point coordinates.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/)

### Constructors
- `GeometryEdge() | GeometryEdge(startIndex, endIndex) | GeometryEdge(edge) | GeometryEdge(edge)` — Initialize

### Methods
#### `GetEndIndex()`

Get end index

**Remarks:** Get end index

**Returns:** `int` — End index.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/#NemAll_Python_Geometry.GeometryEdge.GetEndIndex)

#### `GetStartIndex()`

Get start index

**Remarks:** Get start index

**Returns:** `int` — Start index.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/#NemAll_Python_Geometry.GeometryEdge.GetStartIndex)

#### `Set(edge)`

Initialize edge from old Allplan structure

**Remarks:** Initialize edge from old Allplan structure

**Parameters:**
- `edge` (Kanten_t) — Edge which will be copied.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/#NemAll_Python_Geometry.GeometryEdge.Set)

#### `SetEndIndex(index)`

Set end index Index is not checked, you set anything.

**Remarks:** Set end index Index is not checked, you set anything.

**Parameters:**
- `index` (int) — End index.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/#NemAll_Python_Geometry.GeometryEdge.SetEndIndex)

#### `SetStartIndex(index)`

Set start index Index is not checked, you set anything.

**Remarks:** Set start index Index is not checked, you set anything.

**Parameters:**
- `index` (int) — Start index.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/#NemAll_Python_Geometry.GeometryEdge.SetStartIndex)

#### `__eq__(edge)`

Comparison of edges without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of edges without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `edge` (GeometryEdge) — Compared edge.

**Returns:** `object` — True when edges are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/#NemAll_Python_Geometry.GeometryEdge.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdge/#NemAll_Python_Geometry.GeometryEdge.__repr__)

### Properties
- `EndIndex` (None, get) — Get and set the end index property :type: None
- `StartIndex` (None, get) — Get and set the start index property :type: None

## GeometryEdgeList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.GeometryEdgeList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/)

### Constructors
- `GeometryEdgeList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (GeometryEdge) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (GeometryEdge) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `GeometryEdge` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (GeometryEdge) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (GeometryEdge) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (GeometryEdgeList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/GeometryEdgeList/#NemAll_Python_Geometry.GeometryEdgeList.extend)

## HealingSettings (class)

Class holding Healing settings for elements.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/)

### Constructors
- `HealingSettings(oPhedType=ePolyhedronHealingSettings.HSET_TRIANGULATE, oPgonType=ePolygonHealingSettings.PHSET_NOMODIFY, weldVertices=False) | HealingSettings(hSet)` — Default constructor. Types can be found in GeometryEnums.h

### Methods
#### `GetPolygonHealingSettings()`

Get Healing settings for polygon

**Remarks:** Get Healing settings for polygon

**Returns:** `ePolygonHealingSettings` — const Reference to Polygon healing settings

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.GetPolygonHealingSettings)

#### `GetPolyhedronHealingSettings()`

Get Healing settings for polyhedron

**Remarks:** Get Healing settings for polyhedron

**Returns:** `ePolyhedronHealingSettings` — const Reference to Polyhedron healing settings

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.GetPolyhedronHealingSettings)

#### `IsPolygonNoModify()`

Check whether the type of the settings for polygon is PHSET_NOMODIFY

**Remarks:** Check whether the type of the settings for polygon is PHSET_NOMODIFY

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.IsPolygonNoModify)

#### `IsPolygonNormalize()`

Check whether the type of the settings for polygon is PHSET_NOMODIFY

**Remarks:** Check whether the type of the settings for polygon is PHSET_NOMODIFY

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.IsPolygonNormalize)

#### `IsPolyhedronNoModify()`

Check whether the type of the settings for polyhedron is HSET_NOMODIFY

**Remarks:** Check whether the type of the settings for polyhedron is HSET_NOMODIFY

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.IsPolyhedronNoModify)

#### `IsPolyhedronTilt()`

Check whether the type of the settings for polyhedron is HSET_TILT

**Remarks:** Check whether the type of the settings for polyhedron is HSET_TILT

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.IsPolyhedronTilt)

#### `IsPolyhedronTriangulate()`

Check whether the type of the settings for polyhedron is HSET_TRIANGULATE

**Remarks:** Check whether the type of the settings for polyhedron is HSET_TRIANGULATE

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.IsPolyhedronTriangulate)

#### `IsPolyhedronUnknown()`

Check whether the type of the settings for polyhedron is HSET_TRIANGULATE

**Remarks:** Check whether the type of the settings for polyhedron is HSET_TRIANGULATE

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.IsPolyhedronUnknown)

#### `IsWeldVerticesEnabled()`

Check if vertices welding is enabled

**Remarks:** Check if vertices welding is enabled

**Returns:** `bool` — true/false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.IsWeldVerticesEnabled)

#### `SetPolygonHealingSettings(pgonSet)`

Set healing settings for polygon.

**Remarks:** Set healing settings for polygon.

**Parameters:**
- `pgonSet` (ePolygonHealingSettings) — new value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.SetPolygonHealingSettings)

#### `SetPolyhedronHealingSettings(phedSet)`

Set healing settings for polyhedron.

**Remarks:** Set healing settings for polyhedron.

**Parameters:**
- `phedSet` (ePolyhedronHealingSettings) — new value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.SetPolyhedronHealingSettings)

#### `SetWeldVertices(enableWeld)`

Set vertices welding

**Remarks:** Set vertices welding

**Parameters:**
- `enableWeld` (bool) — Enable vertices welding

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HealingSettings/#NemAll_Python_Geometry.HealingSettings.SetWeldVertices)

## HiddenCalculationParameters (class)

Parameters of hidden calculation.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculationParameters/)

### Constructors
- `HiddenCalculationParameters() | HiddenCalculationParameters(param)` — Initialize

### Methods
#### `SetObserverMatrix(eyePoint, viewPoint)`

Calculates observer matrix from 2 points.

**Remarks:** Calculates observer matrix from 2 points.

**Parameters:**
- `eyePoint` (Point3D) — Eye point.
- `viewPoint` (Point3D) — View point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculationParameters/#NemAll_Python_Geometry.HiddenCalculationParameters.SetObserverMatrix)

### Properties
- `AdjacentEdgesMaxAngle` (Angle, get/set) — Adjacent edges max angle. Maximal angle in which adjacent edges will be joined.
- `GetHiddenLines` (bool, get/set) — Determines if get also hidden lines. If false only visible lines are produced during hidden calculation.
- `ObserverMatrix` (Matrix3D, get/set) — Observer matrix;

## HiddenCalculus (class)

Hidden calculation service implementation, user part.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/)

### Constructors
- `HiddenCalculus()` — Initialize

### Methods
#### `AddElement(elementGeometry_object, elenentMaterial, elementTag)`

Add element geometry to hidden world.

**Remarks:** Add element geometry to hidden world.

**Parameters:**
- `elementGeometry_object` (object) — Element geometry.
- `elenentMaterial` (HiddenMaterial) — Element material.
- `elementTag` (int) — Tag transferred over hidden calculation from element to line.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/#NemAll_Python_Geometry.HiddenCalculus.AddElement)

#### `Calculate()`

Runs hidden calculation. hrow Exception in case of internal error.

**Remarks:** Runs hidden calculation. hrow Exception in case of internal error.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/#NemAll_Python_Geometry.HiddenCalculus.Calculate)

#### `Configure(parameters)`

Sets parameters of calculation.

**Remarks:** Sets parameters of calculation.

**Parameters:**
- `parameters` (HiddenCalculationParameters) — New parameters.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/#NemAll_Python_Geometry.HiddenCalculus.Configure)

#### `GetLinesCount()`

Gets total count of lines calculated.

**Remarks:** Gets total count of lines calculated.

**Returns:** `int` — size_t Lines count.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/#NemAll_Python_Geometry.HiddenCalculus.GetLinesCount)

#### `GetResultLine(lineIndex)`

GetResultLine

**Remarks:** GetResultLine

**Parameters:**
- `lineIndex` (int) — Index of line.

**Returns:** `Line3D` — tuple(Line calculated by hidden itself,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/#NemAll_Python_Geometry.HiddenCalculus.GetResultLine)

#### `GetResultLineMaterial(lineIndex)`

Gets hidden material of element line is from.

**Remarks:** Gets hidden material of element line is from.

**Parameters:**
- `lineIndex` (int) — Index of line.

**Returns:** `HiddenMaterial` — HiddenMaterial Material.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/#NemAll_Python_Geometry.HiddenCalculus.GetResultLineMaterial)

#### `GetResultLineTag(lineIndex)`

Gets (reference to) tag hooked to element line is from.

**Remarks:** Gets (reference to) tag hooked to element line is from.

**Parameters:**
- `lineIndex` (int) — Index of line.

**Returns:** `int` — const TagType Reference to tag object.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenCalculus/#NemAll_Python_Geometry.HiddenCalculus.GetResultLineTag)

## HiddenMaterial (class)

Input element settings used by hidden calculation.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/HiddenMaterial/)

### Constructors
- `HiddenMaterial() | HiddenMaterial(source)` — Initialize

### Properties
- `ExtraSmooth` (bool, get/set) — Extra smooth flag for entry. Flag is usual for elements like round column. Adjacent edges of such element are optimized with constant angle (20 deg) ignoring angle set in hidden options (usual about 1deg).
- `MaterialNumber` (int, get/set) — So called material number of entry. Elements with same material numbers are threaded specially depending on settings of hidden calculation e.g suspended lines between them etc. Value 0 means no material set.

## IntersectRayPolyhedronFlag (enum)

Flag for determining the best result of ray-polyhedron intersection

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/IntersectRayPolyhedronFlag/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `IntersectRayPolyhedronFlag` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/IntersectRayPolyhedronFlag/#NemAll_Python_Geometry.IntersectRayPolyhedronFlag.__getitem__)

### Values
- `eNegativeIfNoPositive` = `1`
- `eNegativePreferred` = `2`
- `ePositiveOnly` = `0`
- `eSmallestLmabda` = `3`

## IntersectionRayBRep (class)

Intersection result of the ray - BRep3D intersection

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/IntersectionRayBRep/)

### Constructors
- `IntersectionRayBRep(element)` — Copy constructor

### Methods
#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/IntersectionRayBRep/#NemAll_Python_Geometry.IntersectionRayBRep.__repr__)

### Properties
- `FaceIdx` (int, get) — 
- `FaceNv` (Vector3D, get) — 
- `IntersectionPoint` (Point3D, get) — 

## IntersectionRayPolyhedron (class)

Intersection result of the ray - Polyhedron3D intersection

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/IntersectionRayPolyhedron/)

### Constructors
- `IntersectionRayPolyhedron(element)` — Copy constructor

### Methods
#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/IntersectionRayPolyhedron/#NemAll_Python_Geometry.IntersectionRayPolyhedron.__repr__)

### Properties
- `FaceIdx` (int, get/set) — face index of the face of the best intersection (if found, not unique) beginning from 0
- `FaceNv` (Vector3D, get/set) — normal vector of best-intersected face (if found, not unique)
- `IntersectionPoint` (Point3D, get/set) — intersection point (if found)
- `Lambda` (float, get/set) — ray parameter of returned intersection (if found)
- `RetCode` (int, get/set) — -1: only neg. inters. found

## Kanten_t (class)

old Allplan structur anf/end begins with 1

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Kanten_t/)

### Constructors
- `Kanten_t()` — Initialize

### Properties
- `anf` (None, get) — :type: None
- `end` (None, get) — :type: None

## Line2D (class)

Representation class for 2D line.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/)

### Constructors
- `Line2D() | Line2D(line) | Line2D(line3D) | Line2D(point1, point2) | Line2D(startPoint, vec) | Line2D(x1, y1, x2, y2) | Line2D(refPoint, point1, point2)` — Initialize

### Methods
#### `EqualRef(line)`

Test for equal points

**Remarks:** Test for equal points

**Parameters:**
- `line` (Line2D) — line for comparision

**Returns:** `bool` — True if points are equal else return false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.EqualRef)

#### `Extend(delta)`

Extend the line

**Remarks:** Extend the line

**Parameters:**
- `delta` (float) — size of extension

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.Extend)

#### `GetAngle()`

Get angle of line

**Remarks:** Get angle of line

**Returns:** `Angle` — Angle of the line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetAngle)

#### `GetCenterPoint()`

Get the center point in world coordinate system

**Remarks:** Get the center point in world coordinate system

**Returns:** `Point2D` — Center point in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetCenterPoint)

#### `GetCoords()`

Get coordinates in world coordinate.

**Remarks:** Get coordinates in world coordinate.

**Returns:** `float` — tuple(X coordinate of start point,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetCoords)

#### `GetEndPoint()`

Get the end point in world coordinate system

**Remarks:** Get the end point in world coordinate system

**Returns:** `Point2D` — point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetEndPoint)

#### `GetEndRelPoint()`

Get the end point in relative coordinate system.

**Remarks:** Get the end point in relative coordinate system.

**Returns:** `Point2D` — constant point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetEndRelPoint)

#### `GetRefPoint()`

Get the point.

**Remarks:** Get the point.

**Returns:** `Point2D` — constant point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetRefPoint)

#### `GetStartPoint()`

Get the start point in world coordinate system.

**Remarks:** Get the start point in world coordinate system.

**Returns:** `Point2D` — point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetStartPoint)

#### `GetStartRelPoint()`

Get the start point in relative coordinate system

**Remarks:** Get the start point in relative coordinate system

**Returns:** `Point2D` — constant point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetStartRelPoint)

#### `GetVector()`

Get the vector.

**Remarks:** Get the vector.

**Returns:** `Vector2D` — vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.GetVector)

#### `IsPoint()`

Check, whether the line is a point (start point equal end point)

**Remarks:** Check, whether the line is a point (start point equal end point)

**Returns:** `bool` — Line is a point: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.IsPoint)

#### `Reverse()`

Reverse orientation of the line

**Remarks:** Reverse orientation of the line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.Reverse)

#### `Set(line) | Set(x1, y1, x2, y2) | Set(point1, point2) | Set(refPoint, point1, point2)`

Initialize line from line.

**Remarks:** Initialize line from line.

**Parameters:**
- `line` (Line2D) — Line2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.Set)

#### `SetEndPoint(endPoint)`

Set end point in world coordinate system.

**Remarks:** Set end point in world coordinate system.

**Parameters:**
- `endPoint` (Point2D) — New start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.SetEndPoint)

#### `SetEndRelPoint(endPoint)`

Set end point in local coordinate system.

**Remarks:** Set end point in local coordinate system.

**Parameters:**
- `endPoint` (Point2D) — New start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.SetEndRelPoint)

#### `SetRefPoint(refPoint)`

Set point in world coordinate system. Coordinates of points will be recalculated with new point. Formula: m_Points[i] = m_RefPoint + m_Points[i] - refPoint

**Remarks:** Set point in world coordinate system. Coordinates of points will be recalculated with new point. Formula: m_Points[i] = m_RefPoint + m_Points[i] - refPoint

**Parameters:**
- `refPoint` (Point2D) — new point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.SetRefPoint)

#### `SetStartPoint(startPoint)`

Set start point in world coordinate system.

**Remarks:** Set start point in world coordinate system.

**Parameters:**
- `startPoint` (Point2D) — New start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.SetStartPoint)

#### `SetStartRelPoint(startPoint)`

Set start point in local coordinate system.

**Remarks:** Set start point in local coordinate system.

**Parameters:**
- `startPoint` (Point2D) — New start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.SetStartRelPoint)

#### `TrimEnd(ds)`

Trim line at the end

**Remarks:** Trim line at the end

**Parameters:**
- `ds` (float) — dimension value the line is modified
- `A` (object) — value >0 shortens the line
- `A` (object) — value <0 extend the line
- `If` (object) — the line is shorten larger than its actual length a exception is throw

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.TrimEnd)

#### `TrimStart(ds)`

Trim line at the start

**Remarks:** Trim line at the start

**Parameters:**
- `ds` (float) — dimension value the line is modified
- `A` (object) — value >0 shortens the line
- `A` (object) — value <0 extend the line
- `If` (object) — the line is shorten larger than its actual length a exception is throw

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.TrimStart)

#### `__eq__(line)`

Comparison of lines without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of lines without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `line` (Line2D) — Compared line.

**Returns:** `object` — True when lines are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.__eq__)

#### `__mul__(matrix)`

Matrix transformation.

**Remarks:** Matrix transformation.

**Parameters:**
- `matrix` (Matrix2D) — transformation matrix.

**Returns:** `object` — Line2D transformed line.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2D/#NemAll_Python_Geometry.Line2D.__repr__)

### Properties
- `EndPoint` (Point2D, get/set) — Get the end point in world coordinate system
- `EndRelPoint` (Point2D, get/set) — Get the end point in relative coordinate system.
- `RefPoint` (Point2D, get/set) — Get the point. Coordinates of points will be recalculated with new point. Formula: m_Points[i] = m_RefPoint + m_Points[i] - refPoint
- `StartPoint` (Point2D, get/set) — Get the start point in world coordinate system.
- `StartRelPoint` (Point2D, get/set) — Get the start point in relative coordinate system

## Line2DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Line2DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/)

### Constructors
- `Line2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Line2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Line2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Line2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Line2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Line2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Line2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line2DList/#NemAll_Python_Geometry.Line2DList.extend)

## Line3D (class)

Representation class for 3D line.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/)

### Constructors
- `Line3D() | Line3D(line2D) | Line3D(line) | Line3D(point1, point2) | Line3D(startPoint, vec) | Line3D(x1, y1, z1, x2, y2, z2) | Line3D(refPoint, startPoint, endPoint)` — Initialize

### Methods
#### `EqualRef(line)`

Test if points are equal.

**Remarks:** Test if points are equal.

**Parameters:**
- `line` (Line3D) — line for comparision

**Returns:** `bool` — bool

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.EqualRef)

#### `GetCenterPoint()`

Get the center point in world coordinate system

**Remarks:** Get the center point in world coordinate system

**Returns:** `Point3D` — Center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetCenterPoint)

#### `GetCoords()`

Get the coordinates. Get the coordinates in world coordinate system.

**Remarks:** Get the coordinates. Get the coordinates in world coordinate system.

**Returns:** `float` — tuple(X coordinate of start point,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetCoords)

#### `GetEndPoint()`

Get the end point. Get the end point in world coordinate system.

**Remarks:** Get the end point. Get the end point in world coordinate system.

**Returns:** `Point3D` — Point3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetEndPoint)

#### `GetEndRelPoint()`

Get the end point. Get the end point in relative coordinate system.

**Remarks:** Get the end point. Get the end point in relative coordinate system.

**Returns:** `Point3D` — Point3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetEndRelPoint)

#### `GetRefPoint()`

Get the point

**Remarks:** Get the point

**Returns:** `Point3D` — Point3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetRefPoint)

#### `GetStartPoint()`

Get the start point. Get the start point in world coordinate system.

**Remarks:** Get the start point. Get the start point in world coordinate system.

**Returns:** `Point3D` — Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetStartPoint)

#### `GetStartRelPoint()`

Get the start point. Get the start point in relative coordinate system.

**Remarks:** Get the start point. Get the start point in relative coordinate system.

**Returns:** `Point3D` — Point3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetStartRelPoint)

#### `GetVector()`

Get the vector from start to end point.

**Remarks:** Get the vector from start to end point.

**Returns:** `Vector3D` — Vector3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.GetVector)

#### `Is2DLine()`

Check, whether the line is a 2D line (both y coordinates are 0.)

**Remarks:** Check, whether the line is a 2D line (both y coordinates are 0.)

**Returns:** `bool` — Line is a 2D line: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.Is2DLine)

#### `IsPoint()`

Check, whether the line is a point (start point equal end point)

**Remarks:** Check, whether the line is a point (start point equal end point)

**Returns:** `bool` — Line is a point: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.IsPoint)

#### `Reverse()`

Reverse orientation of the Line

**Remarks:** Reverse orientation of the Line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.Reverse)

#### `Set(x1, y1, z1, x2, y2, z2) | Set(line) | Set(startPoint, endPoint) | Set(refPoint, startPoint, endPoint)`

Initialize from 6 doubles Set line points in world coordinate system

**Remarks:** Initialize from 6 doubles Set line points in world coordinate system

**Parameters:**
- `x1` (float) — X coordinate of start point
- `y1` (float) — Y coordinate of start point
- `z1` (float) — Z coordinate of start point
- `x2` (float) — X coordinate of end point
- `y2` (float) — Y coordinate of end point
- `z2` (float) — Z coordinate of end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.Set)

#### `SetEndPoint(endPoint)`

Set end point Set end point in world coordinate system.

**Remarks:** Set end point Set end point in world coordinate system.

**Parameters:**
- `endPoint` (Point3D) — End point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.SetEndPoint)

#### `SetEndRelPoint(endPoint)`

Set end point Set end point in Local coordinate system.

**Remarks:** Set end point Set end point in Local coordinate system.

**Parameters:**
- `endPoint` (Point3D) — End point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.SetEndRelPoint)

#### `SetRefPoint(refPoint)`

Set the point.

**Remarks:** Set the point.

**Parameters:**
- `refPoint` (Point3D) — Reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.SetRefPoint)

#### `SetStartPoint(startPoint)`

Set start point Set start point in world coordinate system.

**Remarks:** Set start point Set start point in world coordinate system.

**Parameters:**
- `startPoint` (Point3D) — Start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.SetStartPoint)

#### `SetStartRelPoint(startPoint)`

Set start point Set start point in Local coordinate system.

**Remarks:** Set start point Set start point in Local coordinate system.

**Parameters:**
- `startPoint` (Point3D) — Start point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.SetStartRelPoint)

#### `TrimEnd(ds)`

Trim line at the end

**Remarks:** Trim line at the end

**Parameters:**
- `ds` (float) — dimension value the line is modified
- `A` (object) — value >0 shortens the line
- `A` (object) — value <0 extend the line
- `If` (object) — the line is shorten larger than its actual length a exception is throw

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.TrimEnd)

#### `TrimStart(ds)`

Trim line at the start

**Remarks:** Trim line at the start

**Parameters:**
- `ds` (float) — dimension value the line is modified
- `A` (object) — value >0 shortens the line
- `A` (object) — value <0 extend the line
- `If` (object) — the line is shorten larger than its actual length a exception is throw

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.TrimStart)

#### `__eq__(line)`

Comparison of lines without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of lines without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `line` (Line3D) — Compared line.

**Returns:** `object` — True when lines are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.__eq__)

#### `__mul__(matrix) | __mul__(matrix)`

2D matrix transformation. Multiplies line start and end point with matrix.

**Remarks:** 2D matrix transformation. Multiplies line start and end point with matrix.

**Parameters:**
- `matrix` (Matrix2D) — 2D transformation Matrix

**Returns:** `object` — Line3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3D/#NemAll_Python_Geometry.Line3D.__repr__)

### Properties
- `EndPoint` (Point3D, get/set) — Get the end point. Get the end point in world coordinate system.
- `EndRelPoint` (Point3D, get/set) — Get the end point. Get the end point in relative coordinate system.
- `RefPoint` (Point3D, get/set) — Get the point
- `StartPoint` (Point3D, get/set) — Get the start point. Get the start point in world coordinate system.
- `StartRelPoint` (Point3D, get/set) — Get the start point. Get the start point in relative coordinate system.

## Line3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Line3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/)

### Constructors
- `Line3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Line3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Line3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Line3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Line3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Line3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Line3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Line3DList/#NemAll_Python_Geometry.Line3DList.extend)

## Matrix2D (class)

Representation class for 2D matrix

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/)

### Constructors
- `Matrix2D() | Matrix2D(matrix)` — Initialize

### Methods
#### `AddDimension()`

Create a 3D matrix from this 2D matrix

**Remarks:** Create a 3D matrix from this 2D matrix

**Returns:** `Matrix3D` — 3D Matrix from this matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.AddDimension)

#### `Determinant()`

Calculate determinant

**Remarks:** Calculate determinant

**Returns:** `float` — Determinant.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.Determinant)

#### `IsIdentity()`

Check to identity matrix

**Remarks:** Check to identity matrix

**Returns:** `bool` — Return true when is matrix identity, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.IsIdentity)

#### `Multiply(matrix)`

Multiple matrix with given matrix calling " A.Multiply(B) " is identical with " A *= B "

**Remarks:** Multiple matrix with given matrix calling " A.Multiply(B) " is identical with " A *= B "

**Parameters:**
- `matrix` (Matrix2D) — Matrix to be multiple with

**Returns:** `Matrix2D` — Product of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.Multiply)

#### `Reflection(axis)`

Reflection across a axis of given angle

**Remarks:** Reflection across a axis of given angle

**Parameters:**
- `axis` (Axis2D) — Reflection axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.Reflection)

#### `Reverse()`

Reverse matrix This method provide geometrical inverse matrix and can not be used with regular inverse 4x4 matrix calculations. Geometrical representation: Point3D = { Point3D * Matrix } * Matrix.Reverse()

**Remarks:** Reverse matrix This method provide geometrical inverse matrix and can not be used with regular inverse 4x4 matrix calculations. Geometrical representation: Point3D = { Point3D * Matrix } * Matrix.Reverse()

**Returns:** `bool` — True when operation is successful, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.Reverse)

#### `Rotation(point, angle)`

Rotate the matrix

**Remarks:** Rotate the matrix

**Parameters:**
- `point` (Point2D) — Point of rotation
- `angle` (Angle) — Angle of rotation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.Rotation)

#### `Scaling(scaleX, scaleY)`

Scale the matrix

**Remarks:** Scale the matrix

**Parameters:**
- `scaleX` (float) — Scale in X axis.
- `scaleY` (float) — Scale in Y axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.Scaling)

#### `SetIdentity()`

Initialize identity matrix

**Remarks:** Initialize identity matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.SetIdentity)

#### `SetReflection(axis)`

Initialize matrix only with reflection

**Remarks:** Initialize matrix only with reflection

**Parameters:**
- `axis` (Axis2D) — Reflection axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.SetReflection)

#### `SetRotation(point, angle)`

Initialize matrix only with rotation

**Remarks:** Initialize matrix only with rotation

**Parameters:**
- `point` (Point2D) — Point of rotation.
- `angle` (Angle) — Angle of rotation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.SetRotation)

#### `SetScaling(scaleX, scaleY)`

Initialize matrix only with scaling factors

**Remarks:** Initialize matrix only with scaling factors

**Parameters:**
- `scaleX` (float) — Scale in X axis.
- `scaleY` (float) — Scale in Y axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.SetScaling)

#### `SetTranslation(vec)`

Initialize matrix only with translation

**Remarks:** Initialize matrix only with translation

**Parameters:**
- `vec` (Vector2D) — Vector of translation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.SetTranslation)

#### `SetValue(index, value)`

Set the matrix element at a specified position Use this method when you don't want to catch exception by operator[].

**Remarks:** Set the matrix element at a specified position Use this method when you don't want to catch exception by operator[].

**Parameters:**
- `index` (int) — Position index <0..9>
- `value` (float) — Value for set

**Returns:** `bool` — True when operation successful (index is not out of range), otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.SetValue)

#### `Translate(vec)`

Translate the matrix

**Remarks:** Translate the matrix

**Parameters:**
- `vec` (Vector2D) — Vector of translation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.Translate)

#### `__add__(matrix)`

Matrix addition Formula: Result(new matrix) = A+B A is this matrix.

**Remarks:** Matrix addition Formula: Result(new matrix) = A+B A is this matrix.

**Parameters:**
- `matrix` (Matrix2D) — B matrix

**Returns:** `Matrix2D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__add__)

#### `__getitem__(index)`

Get the matrix element at a specified position This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get the matrix element at a specified position This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — Position index <0..9>

**Returns:** `float` — Returns an element at a specified position.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__getitem__)

#### `__iadd__(matrix)`

Matrix addition Formula: A += B or A = A+B A is this matrix.

**Remarks:** Matrix addition Formula: A += B or A = A+B A is this matrix.

**Parameters:**
- `matrix` (Matrix2D) — B matrix

**Returns:** `Matrix2D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__iadd__)

#### `__imul__(matrix)`

Matrix multiplication Formula: A = B or A = AB A is this matrix.

**Remarks:** Matrix multiplication Formula: A = B or A = AB A is this matrix.

**Parameters:**
- `matrix` (Matrix2D) — B matrix

**Returns:** `Matrix2D` — Product of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__imul__)

#### `__isub__(matrix)`

Matrix addition Formula: A -= B or A = A-B A is this matrix.

**Remarks:** Matrix addition Formula: A -= B or A = A-B A is this matrix.

**Parameters:**
- `matrix` (Matrix2D) — B matrix

**Returns:** `Matrix2D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__isub__)

#### `__mul__(matrix)`

Matrix multiplication Formula: Result(new matrix) = A*B A is this matrix.

**Remarks:** Matrix multiplication Formula: Result(new matrix) = A*B A is this matrix.

**Parameters:**
- `matrix` (Matrix2D) — B matrix

**Returns:** `Matrix2D` — Product of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__repr__)

#### `__sub__(matrix)`

Matrix addition Formula: Result(new matrix) = A-B A is this matrix.

**Remarks:** Matrix addition Formula: Result(new matrix) = A-B A is this matrix.

**Parameters:**
- `matrix` (Matrix2D) — B matrix

**Returns:** `Matrix2D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix2D/#NemAll_Python_Geometry.Matrix2D.__sub__)

## Matrix3D (class)

Representation class for 3D matrix

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/)

### Constructors
- `Matrix3D() | Matrix3D(proj) | Matrix3D(matrix) | Matrix3D(v00, v01, v02, v03, v10, v11, v12, v13, v20, v21, v22, v23, v30, v31, v32, v33) | Matrix3D(valuesinrows)` — Initialize

### Methods
#### `Determinant()`

Calculate determinant

**Remarks:** Calculate determinant

**Returns:** `float` — Determinant.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Determinant)

#### `GaussInvert()`

Inverse matrix by Gauss

**Remarks:** Inverse matrix by Gauss

**Returns:** `bool` — True when operation is successful, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GaussInvert)

#### `GetScaleX()`

Get scale X

**Remarks:** Get scale X

**Returns:** `float` — scale X

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetScaleX)

#### `GetScaleY()`

Get scale Y

**Remarks:** Get scale Y

**Returns:** `float` — scale Y

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetScaleY)

#### `GetScaleZ()`

Get scale Z

**Remarks:** Get scale Z

**Returns:** `float` — scale Z

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetScaleZ)

#### `GetScaling()`

Calculates the scaling factors from the matrix

**Remarks:** Calculates the scaling factors from the matrix

**Returns:** `float` — tuple(Scale in X axis,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetScaling)

#### `GetTranslationVector()`

Get translation part of a matrix

**Remarks:** Get translation part of a matrix

**Returns:** `Vector3D` — The vector of translation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetTranslationVector)

#### `GetVectorX()`

Get vector X

**Remarks:** Get vector X

**Returns:** `Vector3D` — Vector X

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetVectorX)

#### `GetVectorY()`

Get vector Y

**Remarks:** Get vector Y

**Returns:** `Vector3D` — Vector Y

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetVectorY)

#### `GetVectorZ()`

Get vector Z

**Remarks:** Get vector Z

**Returns:** `Vector3D` — Vector Z

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.GetVectorZ)

#### `IsIdentity()`

Check to identity matrix

**Remarks:** Check to identity matrix

**Returns:** `bool` — Return true when is matrix identity, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.IsIdentity)

#### `LaplaceTransform()`

Transformation matrix by Laplace

**Remarks:** Transformation matrix by Laplace

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.LaplaceTransform)

#### `Multiply(matrix)`

Matrix multiplication Formula: A = B or A = AB A is this matrix.

**Remarks:** Matrix multiplication Formula: A = B or A = AB A is this matrix.

**Parameters:**
- `matrix` (Matrix3D) — B matrix

**Returns:** `Matrix3D` — Product of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Multiply)

#### `ReduceZDimension()`

Create a 2D matrix from this 3D matrix

**Remarks:** Create a 2D matrix from this 3D matrix

**Returns:** `Matrix2D` — a 2D matrix from this 3D matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.ReduceZDimension)

#### `Reflection(plane)`

Reflection across a plane

**Remarks:** Reflection across a plane

**Parameters:**
- `plane` (Plane3D) — Reflection plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Reflection)

#### `Reverse()`

Reverse matrix This method provide geometrical inverse matrix and can not be used with regular inverse 4x4 matrix calculations. Geometrical representation: Point3D = { Point3D * Matrix } * Matrix.Reverse()

**Remarks:** Reverse matrix This method provide geometrical inverse matrix and can not be used with regular inverse 4x4 matrix calculations. Geometrical representation: Point3D = { Point3D * Matrix } * Matrix.Reverse()

**Returns:** `bool` — True when operation is successful, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Reverse)

#### `Rotation(line, angle)`

Rotate the matrix Rotate current matrix, not created new one.

**Remarks:** Rotate the matrix Rotate current matrix, not created new one.

**Parameters:**
- `line` (Line3D) — Axis of rotation.
- `angle` (Angle) — Angle of rotation.

**Returns:** `bool` — True when successful, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Rotation)

#### `Scaling(scaleX, scaleY, scaleZ)`

Scale the matrix Scale current matrix, not created new one.

**Remarks:** Scale the matrix Scale current matrix, not created new one.

**Parameters:**
- `scaleX` (float) — Scale in X axis.
- `scaleY` (float) — Scale in Y axis.
- `scaleZ` (float) — Scale in Z axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Scaling)

#### `SetIdentity()`

Initialize identity matrix

**Remarks:** Initialize identity matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetIdentity)

#### `SetProjection(proj)`

Create a matrix for the required projection Method throw THROW_GEO_EXCEPTION_INCORRECT_PARAMETERS_ geometry exception in case of invalid proj.

**Remarks:** Create a matrix for the required projection Method throw THROW_GEO_EXCEPTION_INCORRECT_PARAMETERS_ geometry exception in case of invalid proj.

**Parameters:**
- `proj` (eProjectionMatrixType) — The required projection

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetProjection)

#### `SetReflection(plane)`

Initialize matrix only with reflection

**Remarks:** Initialize matrix only with reflection

**Parameters:**
- `plane` (Plane3D) — Reflection plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetReflection)

#### `SetRotation(axis, angle) | SetRotation(startDirection, endDirection)`

Initialize matrix only with rotation

**Remarks:** Initialize matrix only with rotation

**Parameters:**
- `axis` (Line3D) — Axis of rotation.
- `angle` (Angle) — Angle of rotation.

**Returns:** `bool` — True when successful, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetRotation)

#### `SetScaling(scaleX, scaleY, scaleZ)`

Initialize matrix only with scaling factors

**Remarks:** Initialize matrix only with scaling factors

**Parameters:**
- `scaleX` (float) — Scale in X axis.
- `scaleY` (float) — Scale in Y axis.
- `scaleZ` (float) — Scale in Z axis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetScaling)

#### `SetTranslation(vec)`

Initialize matrix only with translation

**Remarks:** Initialize matrix only with translation

**Parameters:**
- `vec` (Vector3D) — Vector of translation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetTranslation)

#### `SetValue(index, value)`

Set the matrix element at a specified position Use this method when you don't want to catch exception by operator[].

**Remarks:** Set the matrix element at a specified position Use this method when you don't want to catch exception by operator[].

**Parameters:**
- `index` (int) — Position index <0..15>
- `value` (float) — Value for set

**Returns:** `bool` — True when operation successful (index is not out of range), otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetValue)

#### `SetValues(v00, v01, v02, v03, v10, v11, v12, v13, v20, v21, v22, v23, v30, v31, v32, v33)`

Sets each matrix-element

**Remarks:** Sets each matrix-element

**Parameters:**
- `v00` (float) — first row, first element
- `v01` (float) — first row, second element
- `v02` (float) — first row, third element
- `v03` (float) — first row, fourth element
- `v10` (float) — second row, first element
- `v11` (float) — second row, second element
- `v12` (float) — second row, third element
- `v13` (float) — second row, fourth element
- `v20` (float) — third row, first element
- `v21` (float) — third row, second element
- `v22` (float) — third row, third element
- `v23` (float) — third row, fourth element
- `v30` (float) — fourth row, first element
- `v31` (float) — fourth row, second element
- `v32` (float) — fourth row, third element
- `v33` (float) — fourth row, fourth element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.SetValues)

#### `Translate(vec)`

Translate the matrix

**Remarks:** Translate the matrix

**Parameters:**
- `vec` (Vector3D) — Vector of translation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Translate)

#### `Transpose()`

Transpose matrix All transform-services multiply transformation-matrix from the right side: If you need the result as it would be multiplication from the left side you need the transposed Matrix [x,y,z,1.0] x / 0, 1, 2, 3\ / 0, 1, 2, 3\ / x | 4, 5, 6, 7 | | 4, 5, 6, 7 | x | y | | 8, 9,10,11 | | 8, 9,10,11 | | z | \ 12,13,14,15 / \ 12,13,14,15 / .0/

**Remarks:** Transpose matrix All transform-services multiply transformation-matrix from the right side: If you need the result as it would be multiplication from the left side you need the transposed Matrix [x,y,z,1.0] x / 0, 1, 2, 3\ / 0, 1, 2, 3\ / x | 4, 5, 6, 7 | | 4, 5, 6, 7 | x | y | | 8, 9,10,11 | | 8, 9,10,11 | | z | \ 12,13,14,15 / \ 12,13,14,15 / .0/

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.Transpose)

#### `__add__(matrix)`

Matrix addition Formula: Result(new matrix) = A+B A is this matrix.

**Remarks:** Matrix addition Formula: Result(new matrix) = A+B A is this matrix.

**Parameters:**
- `matrix` (Matrix3D) — B matrix

**Returns:** `Matrix3D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__add__)

#### `__eq__(mat)`

Comparison of matrices without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of matrices without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `arc` (object) — Compared arc.

**Returns:** `bool` — True when matrices are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__eq__)

#### `__getitem__(index)`

Get the matrix element at a specified position This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get the matrix element at a specified position This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — Position index <0..15>

**Returns:** `float` — Returns an element at a specified position.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__getitem__)

#### `__iadd__(matrix)`

Matrix addition Formula: A += B or A = A+B A is this matrix.

**Remarks:** Matrix addition Formula: A += B or A = A+B A is this matrix.

**Parameters:**
- `matrix` (Matrix3D) — B matrix

**Returns:** `Matrix3D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__iadd__)

#### `__imul__(matrix)`

Matrix multiplication Formula: A = B or A = AB. A is this matrix.

**Remarks:** Matrix multiplication Formula: A = B or A = AB. A is this matrix.

**Parameters:**
- `matrix` (Matrix3D) — B matrix

**Returns:** `Matrix3D` — Product of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__imul__)

#### `__isub__(matrix)`

Matrix addition Formula: A -= B or A = A-B A is this matrix.

**Remarks:** Matrix addition Formula: A -= B or A = A-B A is this matrix.

**Parameters:**
- `matrix` (Matrix3D) — B matrix

**Returns:** `Matrix3D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__isub__)

#### `__mul__(matrix)`

Matrix multiplication Formula: Result(new matrix) = A*B A is this matrix.

**Remarks:** Matrix multiplication Formula: Result(new matrix) = A*B A is this matrix.

**Parameters:**
- `matrix` (Matrix3D) — B matrix

**Returns:** `Matrix3D` — Product of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__repr__)

#### `__sub__(matrix)`

Matrix addition Formula: Result(new matrix) = A-B A is this matrix.

**Remarks:** Matrix addition Formula: Result(new matrix) = A-B A is this matrix.

**Parameters:**
- `matrix` (Matrix3D) — B matrix

**Returns:** `Matrix3D` — Addition of matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/#NemAll_Python_Geometry.Matrix3D.__sub__)

## Matrix3DList (class)

List for Matrix3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/)

### Constructors
- `Matrix3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Matrix3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Matrix3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Matrix3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Matrix3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Matrix3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Matrix3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3DList/#NemAll_Python_Geometry.Matrix3DList.extend)

## MinMax2D (class)

Representation class for 2D MinMax box.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/)

### Constructors
- `MinMax2D() | MinMax2D(min, max) | MinMax2D(point) | MinMax2D(minmax) | MinMax2D(minmax)` — Initialize

### Methods
#### `Deflate(x, y) | Deflate(size)`

Deflate in x and y axis.

**Remarks:** Deflate in x and y axis.

**Parameters:**
- `x` (float)
- `y` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.Deflate)

#### `Get()`

Get minimum and maximum point

**Remarks:** Get minimum and maximum point

**Returns:** `Point2D` — tuple(minimum point,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.Get)

#### `GetCenter()`

Get box center point

**Remarks:** Get box center point

**Returns:** `Point2D` — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.GetCenter)

#### `GetMax()`

Get maximum point

**Remarks:** Get maximum point

**Returns:** `Point2D` — maximum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.GetMax)

#### `GetMin()`

Get minimum point

**Remarks:** Get minimum point

**Returns:** `Point2D` — minimum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.GetMin)

#### `GetSizeX()`

Get the size of the box in the X direction

**Remarks:** Get the size of the box in the X direction

**Returns:** `float` — delta X value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.GetSizeX)

#### `GetSizeY()`

Get the size of the box in the Y direction

**Remarks:** Get the size of the box in the Y direction

**Returns:** `float` — delta Y value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.GetSizeY)

#### `Inflate(x, y) | Inflate(size)`

ame Inflate and deflate minmax box Inflate in x and y axis.

**Remarks:** ame Inflate and deflate minmax box Inflate in x and y axis.

**Parameters:**
- `x` (float)
- `y` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.Inflate)

#### `IsContaining(box) | IsContaining(point)`

Is box inside this box

**Remarks:** Is box inside this box

**Parameters:**
- `box` (MinMax2D) — Potentially contained box

**Returns:** `bool` — true, if is inside, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.IsContaining)

#### `IsValid()`

Test if box is valid

**Remarks:** Test if box is valid

**Returns:** `bool` — true, if box valid, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.IsValid)

#### `Overlaps(box)`

Does box overlap this box

**Remarks:** Does box overlap this box

**Parameters:**
- `box` (MinMax2D) — Box

**Returns:** `bool` — true, if boxes overlap, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.Overlaps)

#### `Reset()`

Set min point to [DBL_MAX,DBL_MAX] and max point to [-DBL_MAX,-DBL_MAX]

**Remarks:** Set min point to [DBL_MAX,DBL_MAX] and max point to [-DBL_MAX,-DBL_MAX]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.Reset)

#### `Set(min, max)`

Set minimum and maximum point

**Remarks:** Set minimum and maximum point

**Parameters:**
- `min` (Point2D) — minimum point
- `max` (Point2D) — maximum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.Set)

#### `SetMax(max)`

Set maximum point

**Remarks:** Set maximum point

**Parameters:**
- `max` (Point2D) — maximum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.SetMax)

#### `SetMin(min)`

Set minimum point

**Remarks:** Set minimum point

**Parameters:**
- `min` (Point2D) — minimum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.SetMin)

#### `ToPolygon2D()`

Creates a 2D polygon from the minmax box the corners of MinMax.

**Remarks:** Creates a 2D polygon from the minmax box the corners of MinMax.

**Returns:** `bool` — tuple(true, if the polygon has been created successfully, otherwise false,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.ToPolygon2D)

#### `__add__(minmax)`

Expand MinMax2D box. Expands the MinMax2D box by the box given in parameter minmax

**Remarks:** Expand MinMax2D box. Expands the MinMax2D box by the box given in parameter minmax

**Parameters:**
- `minmax` (MinMax2D) — MinMax2D to be added

**Returns:** `MinMax2D` — minmax box.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.__add__)

#### `__eq__(minmax)`

Comparison of minmax objects without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of minmax objects without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `minmax` (MinMax2D) — Compared minmax.

**Returns:** `object` — True when minmax objects are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.__eq__)

#### `__getitem__(index)`

Get the corners of MinMax. Corner index (0-left bottom, 1-right bottom, 2-right top, 3-left top) This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get the corners of MinMax. Corner index (0-left bottom, 1-right bottom, 2-right top, 3-left top) This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int)

**Returns:** `Point2D` — corner as Point2D in world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.__getitem__)

#### `__iadd__(minmax) | __iadd__(point) | __iadd__(box)`

Expand MinMax2D box. Expands the MinMax2D box by the box given in parameter minmax

**Remarks:** Expand MinMax2D box. Expands the MinMax2D box by the box given in parameter minmax

**Parameters:**
- `minmax` (MinMax2D) — MinMax2D to be added

**Returns:** `MinMax2D` — minmax box.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.__iadd__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2D/#NemAll_Python_Geometry.MinMax2D.__repr__)

### Properties
- `Max` (Point2D, get/set) — Get maximum point
- `Min` (Point2D, get/set) — Get minimum point
- `SizeX` (float, get) — Get the size of the box in the X direction
- `SizeY` (float, get) — Get the size of the box in the Y direction

## MinMax2DList (class)

List for MinMax2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/)

### Constructors
- `MinMax2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (MinMax2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (MinMax2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `MinMax2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (MinMax2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (MinMax2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (MinMax2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax2DList/#NemAll_Python_Geometry.MinMax2DList.extend)

## MinMax3D (class)

Representation class for 3D MinMax box.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/)

### Constructors
- `MinMax3D() | MinMax3D(min, max) | MinMax3D(point) | MinMax3D(minmax)` — Initialize

### Methods
#### `Deflate(size) | Deflate(x, y, z)`

Deflate in x,y,z axis concurrently.

**Remarks:** Deflate in x,y,z axis concurrently.

**Parameters:**
- `size` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.Deflate)

#### `Get()`

Get minimum and maximum point

**Remarks:** Get minimum and maximum point

**Returns:** `Point3D` — tuple(minimum point,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.Get)

#### `GetCenter()`

Get box center point

**Remarks:** Get box center point

**Returns:** `Point3D` — center point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.GetCenter)

#### `GetMax()`

Get maximum point

**Remarks:** Get maximum point

**Returns:** `Point3D` — maximum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.GetMax)

#### `GetMin()`

Get minimum point

**Remarks:** Get minimum point

**Returns:** `Point3D` — minimum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.GetMin)

#### `GetSizeX()`

Get the size of the box in the X direction

**Remarks:** Get the size of the box in the X direction

**Returns:** `float` — delta X value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.GetSizeX)

#### `GetSizeY()`

Get the size of the box in the Y direction

**Remarks:** Get the size of the box in the Y direction

**Returns:** `float` — delta Y value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.GetSizeY)

#### `GetSizeZ()`

Get the size of the box in the Y direction

**Remarks:** Get the size of the box in the Y direction

**Returns:** `float` — delta Y value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.GetSizeZ)

#### `Inflate(x, y, z) | Inflate(size)`

ame Inflate and deflate minmax box Inflate in x, y and z axis.

**Remarks:** ame Inflate and deflate minmax box Inflate in x, y and z axis.

**Parameters:**
- `x` (float)
- `y` (float)
- `z` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.Inflate)

#### `IsContaining(box)`

Is box inside this box

**Remarks:** Is box inside this box

**Parameters:**
- `box` (MinMax3D) — Potentially contained box

**Returns:** `bool` — true, if is inside, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.IsContaining)

#### `IsValid()`

Test if box is valid

**Remarks:** Test if box is valid

**Returns:** `bool` — true, if box valid, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.IsValid)

#### `Overlaps(box)`

Does box overlap this box

**Remarks:** Does box overlap this box

**Parameters:**
- `box` (MinMax3D) — Box

**Returns:** `bool` — true, if boxes overlap, otherwise false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.Overlaps)

#### `Reset()`

Set min point to [DBL_MAX,DBL_MAX,DBL_MAX] and max point to [-DBL_MAX,-DBL_MAX,-DBL_MAX]

**Remarks:** Set min point to [DBL_MAX,DBL_MAX,DBL_MAX] and max point to [-DBL_MAX,-DBL_MAX,-DBL_MAX]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.Reset)

#### `Set(min, max)`

Set minimum and maximum point

**Remarks:** Set minimum and maximum point

**Parameters:**
- `min` (Point3D) — minimum point
- `max` (Point3D) — maximum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.Set)

#### `SetMax(max)`

Set maximum point

**Remarks:** Set maximum point

**Parameters:**
- `max` (Point3D) — maximum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.SetMax)

#### `SetMin(min)`

Set minimum point

**Remarks:** Set minimum point

**Parameters:**
- `min` (Point3D) — minimum point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.SetMin)

#### `__add__(minmax)`

Expand MinMax3D box. Expands the MinMax3D box by the box given in parameter minmax

**Remarks:** Expand MinMax3D box. Expands the MinMax3D box by the box given in parameter minmax

**Parameters:**
- `minmax` (MinMax3D) — MinMax3D to be added

**Returns:** `MinMax3D` — minmax box.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.__add__)

#### `__eq__(minmax)`

Comparison of minmax objects without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of minmax objects without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `minmax` (MinMax3D) — Compared minmax.

**Returns:** `object` — True when minmax objects are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.__eq__)

#### `__iadd__(minmax) | __iadd__(point)`

Expand MinMax3D box. Expands the MinMax3D box by the box given in parameter minmax

**Remarks:** Expand MinMax3D box. Expands the MinMax3D box by the box given in parameter minmax

**Parameters:**
- `minmax` (MinMax3D) — MinMax3D to be added

**Returns:** `MinMax3D` — minmax box.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.__iadd__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/MinMax3D/#NemAll_Python_Geometry.MinMax3D.__repr__)

### Properties
- `Max` (Point3D, get/set) — Get maximum point
- `Min` (Point3D, get/set) — Get minimum point
- `SizeX` (float, get) — Get the size of the box in the X direction
- `SizeY` (float, get) — Get the size of the box in the Y direction
- `SizeZ` (float, get) — Get the size of the box in the Y direction

## Offset3DPlane (enum)

Definition of plane on which offset of 3D elements will be calculated

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Offset3DPlane/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `Offset3DPlane` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Offset3DPlane/#NemAll_Python_Geometry.Offset3DPlane.__getitem__)

### Values
- `eNoPlane` = `0`
- `eXY` = `1`
- `eXZ` = `2`
- `eYZ` = `3`

## OrientedEdge (class)

Oriented edge Identification of any edge via edge handle and orientation (positive, negative).

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/)

### Constructors
- `OrientedEdge() | OrientedEdge(edgeHandle, positiveOrientation) | OrientedEdge(orientedEdge)` — Initialize

### Methods
#### `GetEdgeHandle()`

Get handle to the edge

**Remarks:** Get handle to the edge

**Returns:** `int` — Edge handle.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/#NemAll_Python_Geometry.OrientedEdge.GetEdgeHandle)

#### `HasPositiveOrientation()`

Get the flag of positive orientation.

**Remarks:** Get the flag of positive orientation.

**Returns:** `bool` — True when edge have positive orientation, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/#NemAll_Python_Geometry.OrientedEdge.HasPositiveOrientation)

#### `Set(orientedEdge) | Set(edgeHandle, positiveOrientation)`

Initialize edge from old Allplan structure

**Remarks:** Initialize edge from old Allplan structure

**Parameters:**
- `orientedEdge` (OrientedEdge) — Edge which will be copied.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/#NemAll_Python_Geometry.OrientedEdge.Set)

#### `SetEdgeHandle(edgeHandle)`

Set handle to the edge edgeHandle is not checked, you set anything.

**Remarks:** Set handle to the edge edgeHandle is not checked, you set anything.

**Parameters:**
- `edgeHandle` (int) — Handle to the edge which will be set.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/#NemAll_Python_Geometry.OrientedEdge.SetEdgeHandle)

#### `SetOrientation(positiveOrientation)`

Set orientation

**Remarks:** Set orientation

**Parameters:**
- `positiveOrientation` (bool) — Set true when orientation is positive, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/#NemAll_Python_Geometry.OrientedEdge.SetOrientation)

#### `__eq__(orientedEdge)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `orientedEdge` (OrientedEdge) — Edge to comparison

**Returns:** `object` — True when both edges are equal.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/#NemAll_Python_Geometry.OrientedEdge.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdge/#NemAll_Python_Geometry.OrientedEdge.__repr__)

### Properties
- `EdgeHandle` (None, get) — Get and set the edge handle property :type: None
- `Positive` (None, get) — Get and set the positive orientation property :type: None

## OrientedEdgeList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.OrientedEdgeList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/)

### Constructors
- `OrientedEdgeList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (OrientedEdge) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (OrientedEdge) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `OrientedEdge` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (OrientedEdge) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (OrientedEdge) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (OrientedEdgeList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/OrientedEdgeList/#NemAll_Python_Geometry.OrientedEdgeList.extend)

## Path (class)

(No description provided in vendor docs for NemAll_Python_Geometry.Path.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/)

### Methods
#### `Clear()`

Clear all geometries from path

**Remarks:** Clear all geometries from path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/#NemAll_Python_Geometry.Path.Clear)

#### `Count()`

Count of geometries stored in path

**Remarks:** Count of geometries stored in path

**Returns:** `int` — Count of geometries.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/#NemAll_Python_Geometry.Path.Count)

#### `GetElement(arg2)`

Get the element by index

**Remarks:** Get the element by index

**Parameters:**
- `index` (object) — Element index.

**Returns:** `object` — Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/#NemAll_Python_Geometry.Path.GetElement)

#### `IsEmpty()`

Tests whether no elements are present

**Remarks:** Tests whether no elements are present

**Returns:** `bool` — True for an empty, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/#NemAll_Python_Geometry.Path.IsEmpty)

#### `Remove(position)`

Remove geometry at the specified position

**Remarks:** Remove geometry at the specified position

**Parameters:**
- `position` (int) — Specified position of removed geometry.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/#NemAll_Python_Geometry.Path.Remove)

#### `Reverse()`

Reverse orientation of the PathElement

**Remarks:** Reverse orientation of the PathElement

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/#NemAll_Python_Geometry.Path.Reverse)

#### `__iter__()`

Get the iterator

**Remarks:** Get the iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path/#NemAll_Python_Geometry.Path.__iter__)

## Path2D (class)

Representation class for 2D path.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/)

### Constructors
- `Path2D() | Path2D(path)` — Initialize

### Methods
#### `Add(element) | Add(element_object)`

Add element into path Path2D

**Remarks:** Add element into path Path2D

**Parameters:**
- `element` (Path2D) — Path element to add

**Returns:** `eGeometryErrorCode` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.Add)

#### `GetEndPoint()`

Get path end point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Remarks:** Get path end point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Returns:** `Point2D` — const Point2D End point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.GetEndPoint)

#### `GetEndRelPoint()`

Get relative end (last) point of the path

**Remarks:** Get relative end (last) point of the path

**Returns:** `Point2D` — End point (relative)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.GetEndRelPoint)

#### `GetStartPoint()`

Get path start point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Remarks:** Get path start point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Returns:** `Point2D` — const Point2D Start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.GetStartPoint)

#### `GetStartRelPoint()`

Get relative start (first) point of the path

**Remarks:** Get relative start (first) point of the path

**Returns:** `Point2D` — Start point (relative)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.GetStartRelPoint)

#### `IsClosed()`

Check for closed path, it mean that first point is same as last point

**Remarks:** Check for closed path, it mean that first point is same as last point

**Returns:** `bool` — True when path is closed, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.IsClosed)

#### `IsItPossibleToAddElement(pGeometry_object)`

Check if the geometry can be added into path Path2D

**Remarks:** Check if the geometry can be added into path Path2D

**Parameters:**
- `pGeometry_object` (object) — Pointer to geometry to check

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.IsItPossibleToAddElement)

#### `IsValid()`

Validity check \warning Path doesn't have to be closed to be valid, you have to check this extra

**Remarks:** Validity check \warning Path doesn't have to be closed to be valid, you have to check this extra

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.IsValid)

#### `IsValidCurveType(pGeometry_object)`

Check if the geometry is supported by Path2D

**Remarks:** Check if the geometry is supported by Path2D

**Parameters:**
- `pGeometry_object` (object) — Pointer to geometry to check

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.IsValidCurveType)

#### `SetEndPoint(endpoint)`

Set end (last) point of the path

**Remarks:** Set end (last) point of the path

**Parameters:**
- `endpoint` (Point2D) — E point

**Returns:** `object` — none

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.SetEndPoint)

#### `SetStartPoint(startpoint)`

Set start (first) point of the path

**Remarks:** Set start (first) point of the path

**Parameters:**
- `startpoint` (Point2D) — Start point

**Returns:** `object` — none

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.SetStartPoint)

#### `__eq__(path)`

Comparison of paths without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of paths without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `path` (Path2D) — Compared path.

**Returns:** `object` — True when paths are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.__eq__)

#### `__iadd__(element) | __iadd__(element) | __iadd__(element) | __iadd__(element) | __iadd__(element) | __iadd__(element)`

Append operator

**Remarks:** Append operator

**Parameters:**
- `element` (Path2D) — Element which will be appended

**Returns:** `Path2D` — Reference to path.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.__iadd__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2D/#NemAll_Python_Geometry.Path2D.__repr__)

### Properties
- `EndPoint` (Point2D, get/set) — Get path end point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error
- `StartPoint` (Point2D, get/set) — Get path start point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

## Path2DList (class)

List for Path2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/)

### Constructors
- `Path2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Path2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Path2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Path2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Path2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Path2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Path2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path2DList/#NemAll_Python_Geometry.Path2DList.extend)

## Path3D (class)

Representation class for 3D path.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/)

### Constructors
- `Path3D() | Path3D(path) | Path3D(point1, point2) | Path3D(point1, point2, point3) | Path3D(point1, point2, point3, point4) | Path3D(point1, point2, point3, point4, point5)` — Initialize

### Methods
#### `Add(element) | Add(element_object)`

Add element into path Path3D

**Remarks:** Add element into path Path3D

**Parameters:**
- `element` (Path3D) — Path element to add

**Returns:** `eGeometryErrorCode` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.Add)

#### `DefinitionPoints()`

Extracts all definition points of the path.

**Remarks:** Extracts all definition points of the path.

**Returns:** `Polyline3D` — All definition points ( join points of sub curves in this path ) of this path as Polyline3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.DefinitionPoints)

#### `FirstDefinitionPoint()`

Extracts the first definition point of the path.

**Remarks:** Extracts the first definition point of the path.

**Returns:** `Point3D` — First definition point of this path as Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.FirstDefinitionPoint)

#### `GetEndPoint()`

Get path end point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Remarks:** Get path end point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Returns:** `Point3D` — const Point3D End point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.GetEndPoint)

#### `GetEndRelPoint()`

Get relative end (last) point of the path

**Remarks:** Get relative end (last) point of the path

**Returns:** `Point3D` — End point (relative)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.GetEndRelPoint)

#### `GetStartPoint()`

Get path start point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Remarks:** Get path start point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

**Returns:** `Point3D` — const Point3D Start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.GetStartPoint)

#### `GetStartRelPoint()`

Get relative start (first) point of the path

**Remarks:** Get relative start (first) point of the path

**Returns:** `Point3D` — Start point (relative)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.GetStartRelPoint)

#### `IsClosed()`

Check for closed path, it mean that first point is same as last point

**Remarks:** Check for closed path, it mean that first point is same as last point

**Returns:** `bool` — True when path is closed, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.IsClosed)

#### `IsItPossibleToAddElement(pGeometry_object) | IsItPossibleToAddElement(pGeometry_object, tolerance)`

Check if the geometry can be added into path Path3D

**Remarks:** Check if the geometry can be added into path Path3D

**Parameters:**
- `pGeometry_object` (object) — Pointer to geometry to check

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.IsItPossibleToAddElement)

#### `IsValid()`

Validity check \warning Path doesn't have to be closed to be valid, you have to check this extra

**Remarks:** Validity check \warning Path doesn't have to be closed to be valid, you have to check this extra

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.IsValid)

#### `IsValidCurveType(pGeometry_object)`

Check if the geometry is supported by Path3D

**Remarks:** Check if the geometry is supported by Path3D

**Parameters:**
- `pGeometry_object` (object) — Pointer to geometry to check

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.IsValidCurveType)

#### `SetEndPoint(endpoint)`

Set end (last) point of the path

**Remarks:** Set end (last) point of the path

**Parameters:**
- `endpoint` (Point3D) — E point

**Returns:** `object` — none

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.SetEndPoint)

#### `SetStartPoint(startpoint)`

Set start (first) point of the path

**Remarks:** Set start (first) point of the path

**Parameters:**
- `startpoint` (Point3D) — Start point

**Returns:** `object` — none

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.SetStartPoint)

#### `__eq__(path)`

Comparison of paths without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of paths without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `path` (Path3D) — Compared path.

**Returns:** `object` — True when paths are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.__eq__)

#### `__iadd__(element) | __iadd__(element) | __iadd__(element) | __iadd__(element) | __iadd__(element) | __iadd__(element)`

Append operator

**Remarks:** Append operator

**Parameters:**
- `element` (Path3D) — Element which will be appended

**Returns:** `Path3D` — Reference to path.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.__iadd__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3D/#NemAll_Python_Geometry.Path3D.__repr__)

### Properties
- `EndPoint` (Point3D, get/set) — Get path end point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error
- `StartPoint` (Point3D, get/set) — Get path start point Throw THROW_GEO_EXCEPTION_OUT_OF_RANGE_ if path is empty Throw THROW_GEO_EXCEPTION_GENERAL_ERROR_ if internal error

## Path3DList (class)

List for Path3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/)

### Constructors
- `Path3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Path3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Path3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Path3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Path3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Path3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Path3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Path3DList/#NemAll_Python_Geometry.Path3DList.extend)

## PathIterator (class)

Path iterator

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PathIterator/)

### Methods
#### `__next__()`

Get the next element

**Remarks:** Get the next element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PathIterator/#NemAll_Python_Geometry.PathIterator.__next__)

## Plane3D (class)

Representation class for 3D plane.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/)

### Constructors
- `Plane3D() | Plane3D(plane) | Plane3D(point, normalVector) | Plane3D(point1, point2, point3) | Plane3D(axis)` — Initialize

### Methods
#### `CalcPlaneVectors()`

Calc X and Y axis vectors for plane

**Remarks:** Calc X and Y axis vectors for plane

**Returns:** `Vector3D` — tuple(Vector of X axis,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.CalcPlaneVectors)

#### `GetPoint()`

Get 3D Plane reference point

**Remarks:** Get 3D Plane reference point

**Returns:** `Point3D` — Point3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.GetPoint)

#### `GetTransformationMatrix()`

Get transformation matrix for given plane 3D

**Remarks:** Get transformation matrix for given plane 3D

**Returns:** `Matrix3D` — Matrix3D const reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.GetTransformationMatrix)

#### `GetVector()`

Get the Normal Vector

**Remarks:** Get the Normal Vector

**Returns:** `Vector3D` — Vector3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.GetVector)

#### `Set(point, normalVector)`

Initialize Plane from point and vector

**Remarks:** Initialize Plane from point and vector

**Parameters:**
- `point` (Point3D) — New point.
- `normalVector` (Vector3D) — New vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.Set)

#### `SetPoint(point)`

Set reference point

**Remarks:** Set reference point

**Parameters:**
- `point` (Point3D) — New point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.SetPoint)

#### `SetVector(vec)`

Set the Normal Vector

**Remarks:** Set the Normal Vector

**Parameters:**
- `vec` (Vector3D) — New vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.SetVector)

#### `__eq__(plane)`

Comparison of planes without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of planes without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `plane` (Plane3D) — Compared plane.

**Returns:** `object` — True when planes are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.__eq__)

#### `__mul__(matrix) | __mul__(matrix)`

2D matrix transformation.

**Remarks:** 2D matrix transformation.

**Parameters:**
- `matrix` (Matrix2D) — 2D transformation Matrix

**Returns:** `Plane3D` — Copy of transformed 3D plane.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Plane3D/#NemAll_Python_Geometry.Plane3D.__repr__)

### Properties
- `Point` (Point3D, get/set) — Get 3D Plane reference point
- `Vector` (Vector3D, get/set) — Get the Normal Vector

## Point2D (class)

Representation class for 2D point

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/)

### Constructors
- `Point2D() | Point2D(point) | Point2D(point) | Point2D(refPoint, point) | Point2D(x, y)` — Initialize

### Methods
#### `GetCoords()`

Get copy of X,Y coordinates.

**Remarks:** Get copy of X,Y coordinates.

**Returns:** `float` — tuple(X coordinate of point,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.GetCoords)

#### `GetDistance(point)`

Get distance. Formula: Result(double) = |A-B|.

**Remarks:** Get distance. Formula: Result(double) = |A-B|.

**Parameters:**
- `point` (Point2D) — Point2D.

**Returns:** `float` — double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.GetDistance)

#### `IsZero()`

Check the coords [0.0,0.0]. If the coords are zero, the return value is true. If the coords aren't zero, the return value is false.

**Remarks:** Check the coords [0.0,0.0]. If the coords are zero, the return value is true. If the coords aren't zero, the return value is false.

**Returns:** `bool` — bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.IsZero)

#### `Set(point) | Set(x, y)`

Set the coordinate.

**Remarks:** Set the coordinate.

**Parameters:**
- `point` (Point2D) — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.Set)

#### `SetX(x)`

Set the coordinate.

**Remarks:** Set the coordinate.

**Parameters:**
- `x` (float) — coordinate.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.SetX)

#### `SetY(y)`

Set the coordinate.

**Remarks:** Set the coordinate.

**Parameters:**
- `y` (float) — coordinate.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.SetY)

#### `Values()`

Get copy of X,Y coordinates as python list.

**Remarks:** Get copy of X,Y coordinates as python list.

**Returns:** `list[float]` — X coordinate of point.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.Values)

#### `__add__(point) | __add__(vec)`

Point translation by point. Formula: Point(new) = Point(this) + Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector2D operand.

**Remarks:** Point translation by point. Formula: Point(new) = Point(this) + Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector2D operand.

**Parameters:**
- `point` (Point2D) — Point.

**Returns:** `Point2D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__add__)

#### `__eq__(point)`

Comparison of points without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of points without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `point` (Point2D) — Compared point.

**Returns:** `bool` — True when points are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__eq__)

#### `__iadd__(point)`

Point translation by point. Formula: Point(this) = Point(this) + Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector2D operand.

**Remarks:** Point translation by point. Formula: Point(this) = Point(this) + Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector2D operand.

**Parameters:**
- `point` (Point2D) — Point.

**Returns:** `Point2D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__iadd__)

#### `__idiv__(divider)`

Divide operator. This method is checked and throwing Geometry::Exception when divider is zero.

**Remarks:** Divide operator. This method is checked and throwing Geometry::Exception when divider is zero.

**Parameters:**
- `divider` (float) — Divider.

**Returns:** `Point2D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__idiv__)

#### `__isub__(point)`

Point translation by negative point Formula: Point(this) = Point(this) - Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector2D operand.

**Remarks:** Point translation by negative point Formula: Point(this) = Point(this) - Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector2D operand.

**Parameters:**
- `point` (Point2D) — Point.

**Returns:** `Point2D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__isub__)

#### `__mul__(matrix)`

Matrix transformation. Result = Point * matrix

**Remarks:** Matrix transformation. Result = Point * matrix

**Parameters:**
- `matrix` (Matrix2D) — Transformation Matrix.

**Returns:** `Point2D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__mul__)

#### `__ne__(point)`

Comparison of points without tolerance. Be careful, this method works without tolerance!

**Remarks:** Comparison of points without tolerance. Be careful, this method works without tolerance!

**Parameters:**
- `point` (Point2D) — Compared point.

**Returns:** `bool` — True when points are not equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__repr__)

#### `__sub__(vec) | __sub__(point)`

Move the point by reversed vector

**Remarks:** Move the point by reversed vector

**Parameters:**
- `vec` (Vector2D) — Vector

**Returns:** `Point2D` — New point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__sub__)

#### `__truediv__(divider)`

Divide operator.

**Remarks:** Divide operator.

**Parameters:**
- `divider` (float) — Divider.

**Returns:** `Point2D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/#NemAll_Python_Geometry.Point2D.__truediv__)

### Properties
- `X` (float, get/set) — Get the x coordinate.
- `Y` (float, get/set) — Get the y coordinate.

## Point2DList (class)

List for Point2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/)

### Constructors
- `Point2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Point2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Point2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Point2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Point2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Point2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Point2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2DList/#NemAll_Python_Geometry.Point2DList.extend)

## Point3D (class)

Representation class for 3D point

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/)

### Constructors
- `Point3D() | Point3D(point) | Point3D(point) | Point3D(refPoint, point) | Point3D(x, y, z)` — Initialize

### Methods
#### `GetCoords()`

Get copy of X,Y,Z coordinates

**Remarks:** Get copy of X,Y,Z coordinates

**Returns:** `float` — tuple(X coordinate of point,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.GetCoords)

#### `GetDistance(point)`

Get distance Formula: Result(double) = |A-B|.

**Remarks:** Get distance Formula: Result(double) = |A-B|.

**Parameters:**
- `point` (Point3D) — Point3D.

**Returns:** `float` — double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.GetDistance)

#### `IsZero()`

Check the coords [0.0, 0.0, 0.0] If the coords are zero, the return value is true. If the coords aren't zero, the return value is false.

**Remarks:** Check the coords [0.0, 0.0, 0.0] If the coords are zero, the return value is true. If the coords aren't zero, the return value is false.

**Returns:** `bool` — bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.IsZero)

#### `Set(point) | Set(x, y, z)`

Initialize from point 3D.

**Remarks:** Initialize from point 3D.

**Parameters:**
- `point` (Point3D) — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.Set)

#### `Values()`

Get copy of X,Y,Z coordinates as python list

**Remarks:** Get copy of X,Y,Z coordinates as python list

**Returns:** `list[float]` — X coordinate of point,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.Values)

#### `__add__(vec) | __add__(vec) | __add__(point)`

Move the point by vector 3D

**Remarks:** Move the point by vector 3D

**Parameters:**
- `vec` (Vector3D) — Vector

**Returns:** `Point3D` — New point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__add__)

#### `__eq__(point)`

Comparison of points without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of points without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `point` (Point3D) — Compared point.

**Returns:** `bool` — True when points are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__eq__)

#### `__iadd__(point)`

Point translation by point Formula: Point(this) = Point(this) + Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector3D operand.

**Remarks:** Point translation by point Formula: Point(this) = Point(this) + Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector3D operand.

**Parameters:**
- `point` (Point3D) — Point.

**Returns:** `Point3D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__iadd__)

#### `__idiv__(divider)`

Divide operator. This method is checked and throwing Geometry::Exception when divider is zero.

**Remarks:** Divide operator. This method is checked and throwing Geometry::Exception when divider is zero.

**Parameters:**
- `divider` (float) — Divider.

**Returns:** `Point3D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__idiv__)

#### `__isub__(point)`

Point translation by negative point Formula: Point(this) = Point(this) - Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector3D operand.

**Remarks:** Point translation by negative point Formula: Point(this) = Point(this) - Point This is not standard math operation and is implemented only as practical use case for point moving in %Allplan. In this case given operand point represent offset from Zero point. For standard move operation please use Service::Move method with Vector3D operand.

**Parameters:**
- `point` (Point3D) — Point.

**Returns:** `Point3D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__isub__)

#### `__mul__(matrix) | __mul__(matrix) | __mul__(scale)`

2D matrix transformation. Result = Point * matrix

**Remarks:** 2D matrix transformation. Result = Point * matrix

**Parameters:**
- `matrix` (Matrix2D) — 2D transformation Matrix

**Returns:** `Point3D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__mul__)

#### `__ne__(point)`

Comparison of points without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of points without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `point` (Point3D) — Compared point.

**Returns:** `bool` — True when points are not equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__repr__)

#### `__sub__(vec) | __sub__(vec) | __sub__(point)`

Move the point by reversed vector 3D

**Remarks:** Move the point by reversed vector 3D

**Parameters:**
- `vec` (Vector3D) — 3D vector

**Returns:** `Point3D` — New point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__sub__)

#### `__truediv__(divider)`

Divide operator.

**Remarks:** Divide operator.

**Parameters:**
- `divider` (float) — Divider.

**Returns:** `Point3D` — Point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3D/#NemAll_Python_Geometry.Point3D.__truediv__)

### Properties
- `X` (float, get/set) — Get the x coordinate
- `Y` (float, get/set) — Get the y coordinate
- `Z` (float, get/set) — Get the z coordinate

## Point3DList (class)

List for Point3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/)

### Constructors
- `Point3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Point3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Point3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Point3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Point3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Point3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Point3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point3DList/#NemAll_Python_Geometry.Point3DList.extend)

## PolyPoints2D (class)

The PolyPoints class is the base template class for all objects which store geometry as a vector of points (or another objects), specially for polyline, polygon and spline.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/)

### Methods
#### `Clear()`

Remove all points from vector.

**Remarks:** Remove all points from vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Clear)

#### `Count()`

Get count of points.

**Remarks:** Get count of points.

**Returns:** `int` — bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Count)

#### `Empty()`

Return true if no points, otherwise false.

**Remarks:** Return true if no points, otherwise false.

**Returns:** `bool` — bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Empty)

#### `EqualRef(polyPoints)`

Test if reference points are equal.

**Remarks:** Test if reference points are equal.

**Parameters:**
- `polyPoints` (PolyPoints2D) — constant the PolyPoints.

**Returns:** `bool` — Reference points are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.EqualRef)

#### `GetEndPoint()`

Get the end point in world coordinate system.

**Remarks:** Get the end point in world coordinate system.

**Returns:** `Point2D` — end point in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetEndPoint)

#### `GetEndRelPoint()`

Get the end point

**Remarks:** Get the end point

**Returns:** `Point2D` — end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetEndRelPoint)

#### `GetLastPoint()`

Get the last point in world coordinate system, data.

**Remarks:** Get the last point in world coordinate system, data.

**Returns:** `Point2D` — last point in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetLastPoint)

#### `GetPoint(index)`

Get point in world coordinate system, data. This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get point in world coordinate system, data. This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — point index.

**Returns:** `Point2D` — point point in world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetPoint)

#### `GetPointIndex(point)`

Get index of the given point

**Remarks:** Get index of the given point

**Parameters:**
- `point` (Point2D) — Searched point

**Returns:** `bool` — tuple(True if a point was found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetPointIndex)

#### `GetPointIndexes(point)`

Get indexes of the given point, in case that several points in the spline will have the same coordinates

**Remarks:** Get indexes of the given point, in case that several points in the spline will have the same coordinates

**Parameters:**
- `point` (Point2D) — Searched point

**Returns:** `bool` — tuple(True if at least one point was found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetPointIndexes)

#### `GetRefPoint()`

Get reference point.

**Remarks:** Get reference point.

**Returns:** `Point2D` — constant the reference point in the world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetRefPoint)

#### `GetRelPoint(index)`

Get point in Local coordinate system, no data. This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get point in Local coordinate system, no data. This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — point index.

**Returns:** `Point2D` — point constant the point at position index.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetRelPoint)

#### `GetStartPoint()`

Get the start point in world coordinate system.

**Remarks:** Get the start point in world coordinate system.

**Returns:** `Point2D` — start point in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetStartPoint)

#### `GetStartRelPoint()`

Get the start point

**Remarks:** Get the start point

**Returns:** `Point2D` — start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.GetStartRelPoint)

#### `Insert(polyPoints, position=18446744073709551615) | Insert(point, position=18446744073709551615)`

Insert vector of points at specific position. If return false then points weren't inserted.

**Remarks:** Insert vector of points at specific position. If return false then points weren't inserted.

**Parameters:**
- `polyPoints` (PolyPoints2D) — constant the PolyPoints.
- `position` (int) — position where points will be inserted.

**Returns:** `bool` — bool true if successful.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Insert)

#### `InsertRel(point, position=18446744073709551615)`

Insert relative point at specific position. Used local coordinates. If return false then points weren't Inserted.

**Remarks:** Insert relative point at specific position. Used local coordinates. If return false then points weren't Inserted.

**Parameters:**
- `point` (Point2D) — constant the Point.
- `position` (int) — position where points will be inserted.

**Returns:** `bool` — bool true if successful.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.InsertRel)

#### `Remove(position)`

Remove point from specific position. If return false then points weren't removed.

**Remarks:** Remove point from specific position. If return false then points weren't removed.

**Parameters:**
- `position` (int) — position of point which will be removed.

**Returns:** `bool` — Point removed: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Remove)

#### `RemoveLastPoint()`

Remove the last point

**Remarks:** Remove the last point

**Returns:** `bool` — Point removed: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.RemoveLastPoint)

#### `Reserve(newCount)`

Reserve container capacity

**Remarks:** Reserve container capacity

**Parameters:**
- `newCount` (int) — Expected size of container [count of points]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Reserve)

#### `Resize(newSize)`

Specifies a new size for the points vector.

**Remarks:** Specifies a new size for the points vector.

**Parameters:**
- `newSize` (int) — The new size of the points vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Resize)

#### `Reverse()`

Reverse the point order

**Remarks:** Reverse the point order

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.Reverse)

#### `SetEndPoint(endpoint)`

Set the end point in world coordinates

**Remarks:** Set the end point in world coordinates

**Parameters:**
- `endpoint` (Point2D) — new end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.SetEndPoint)

#### `SetPoint(point, index)`

Set point at given position in world coordinate system.

**Remarks:** Set point at given position in world coordinate system.

**Parameters:**
- `point` (Point2D) — point in the world coordinate system.
- `index` (int) — index of point which will be set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.SetPoint)

#### `SetRefPoint(refPoint)`

Set reference point in world coordinate system.

**Remarks:** Set reference point in world coordinate system.

**Parameters:**
- `refPoint` (Point2D) — reference point in the world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.SetRefPoint)

#### `SetRelPoint(point, index)`

Set point at given position in relative coordinate system.

**Remarks:** Set point at given position in relative coordinate system.

**Parameters:**
- `point` (Point2D) — point in the relative coordinate system.
- `index` (int) — index of point which will be set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.SetRelPoint)

#### `SetStartPoint(startpoint)`

Set the start point in world coordinates

**Remarks:** Set the start point in world coordinates

**Parameters:**
- `startpoint` (Point2D) — new start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.SetStartPoint)

#### `ToLineChain()`

Get polyline as a chain of lines composed from 2 points.

**Remarks:** Get polyline as a chain of lines composed from 2 points.

**Returns:** `Point2DList` — vector of lines composed from 2 points (start and end point of a line)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.ToLineChain)

#### `__getitem__(index)`

Get point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — point index.

**Returns:** `Point2D` — point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.__getitem__)

#### `__iadd__(point)`

Add point in world coordinates.

**Remarks:** Add point in world coordinates.

**Parameters:**
- `point` (Point2D) — adding point.

**Returns:** `PolyPoints2D` — New point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.__iadd__)

#### `__mul__(matrix)`

2D matrix transformation

**Remarks:** 2D matrix transformation

**Parameters:**
- `matrix` (Matrix2D) — 2D transformation matrix

**Returns:** `PolyPoints2D` — Transformed polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.__mul__)

#### `__setitem__(arg2, value)`

Set point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range. Args: index: Specified position pnt: Point

**Remarks:** Set point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range. Args: index: Specified position pnt: Point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints2D/#NemAll_Python_Geometry.PolyPoints2D.__setitem__)

### Properties
- `EndPoint` (Point2D, get/set) — Get the end point in world coordinate system.
- `EndRelPoint` (Point2D, get) — Get the end point
- `Points` (list[Point2D], get/set) — Get the point list
- `RefPoint` (Point2D, get/set) — Get reference point.
- `StartPoint` (Point2D, get/set) — Get the start point in world coordinate system.
- `StartRelPoint` (Point2D, get) — Get the start point

## PolyPoints3D (class)

The PolyPoints class is the base template class for all objects which store geometry as a vector of points (or another objects), specially for polyline, polygon and spline.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/)

### Methods
#### `Clear()`

Remove all points from vector.

**Remarks:** Remove all points from vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Clear)

#### `Count()`

Get count of points.

**Remarks:** Get count of points.

**Returns:** `int` — bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Count)

#### `Empty()`

Return true if no points, otherwise false.

**Remarks:** Return true if no points, otherwise false.

**Returns:** `bool` — bool.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Empty)

#### `EqualRef(polyPoints)`

Test if reference points are equal.

**Remarks:** Test if reference points are equal.

**Parameters:**
- `polyPoints` (PolyPoints3D) — constant the PolyPoints.

**Returns:** `bool` — Reference points are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.EqualRef)

#### `GetEndPoint()`

Get the end point in world coordinate system.

**Remarks:** Get the end point in world coordinate system.

**Returns:** `Point3D` — end point in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetEndPoint)

#### `GetEndRelPoint()`

Get the end point

**Remarks:** Get the end point

**Returns:** `Point3D` — end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetEndRelPoint)

#### `GetLastPoint()`

Get the last point in world coordinate system, data.

**Remarks:** Get the last point in world coordinate system, data.

**Returns:** `Point3D` — last point in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetLastPoint)

#### `GetPoint(index)`

Get point in world coordinate system, data. This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get point in world coordinate system, data. This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — point index.

**Returns:** `Point3D` — point point in world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetPoint)

#### `GetPointIndex(point)`

Get index of the given point

**Remarks:** Get index of the given point

**Parameters:**
- `point` (Point3D) — Searched point

**Returns:** `bool` — tuple(True if a point was found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetPointIndex)

#### `GetPointIndexes(point)`

Get indexes of the given point, in case that several points in the spline will have the same coordinates

**Remarks:** Get indexes of the given point, in case that several points in the spline will have the same coordinates

**Parameters:**
- `point` (Point3D) — Searched point

**Returns:** `bool` — tuple(True if at least one point was found,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetPointIndexes)

#### `GetRefPoint()`

Get reference point.

**Remarks:** Get reference point.

**Returns:** `Point3D` — constant the reference point in the world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetRefPoint)

#### `GetRelPoint(index)`

Get point in Local coordinate system, no data. This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get point in Local coordinate system, no data. This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — point index.

**Returns:** `Point3D` — point constant the point at position index.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetRelPoint)

#### `GetStartPoint()`

Get the start point in world coordinate system.

**Remarks:** Get the start point in world coordinate system.

**Returns:** `Point3D` — start point in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetStartPoint)

#### `GetStartRelPoint()`

Get the start point

**Remarks:** Get the start point

**Returns:** `Point3D` — start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.GetStartRelPoint)

#### `Insert(polyPoints, position=18446744073709551615) | Insert(point, position=18446744073709551615)`

Insert vector of points at specific position. If return false then points weren't inserted.

**Remarks:** Insert vector of points at specific position. If return false then points weren't inserted.

**Parameters:**
- `polyPoints` (PolyPoints3D) — constant the PolyPoints.
- `position` (int) — position where points will be inserted.

**Returns:** `bool` — bool true if successful.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Insert)

#### `InsertRel(point, position=18446744073709551615)`

Insert relative point at specific position. Used local coordinates. If return false then points weren't Inserted.

**Remarks:** Insert relative point at specific position. Used local coordinates. If return false then points weren't Inserted.

**Parameters:**
- `point` (Point3D) — constant the Point.
- `position` (int) — position where points will be inserted.

**Returns:** `bool` — bool true if successful.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.InsertRel)

#### `Remove(position)`

Remove point from specific position. If return false then points weren't removed.

**Remarks:** Remove point from specific position. If return false then points weren't removed.

**Parameters:**
- `position` (int) — position of point which will be removed.

**Returns:** `bool` — Point removed: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Remove)

#### `RemoveLastPoint()`

Remove the last point

**Remarks:** Remove the last point

**Returns:** `bool` — Point removed: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.RemoveLastPoint)

#### `Reserve(newCount)`

Reserve container capacity

**Remarks:** Reserve container capacity

**Parameters:**
- `newCount` (int) — Expected size of container [count of points]

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Reserve)

#### `Resize(newSize)`

Specifies a new size for the points vector.

**Remarks:** Specifies a new size for the points vector.

**Parameters:**
- `newSize` (int) — The new size of the points vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Resize)

#### `Reverse()`

Reverse the point order

**Remarks:** Reverse the point order

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.Reverse)

#### `SetEndPoint(endpoint)`

Set the end point in world coordinates

**Remarks:** Set the end point in world coordinates

**Parameters:**
- `endpoint` (Point3D) — new end point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.SetEndPoint)

#### `SetPoint(point, index)`

Set point at given position in world coordinate system.

**Remarks:** Set point at given position in world coordinate system.

**Parameters:**
- `point` (Point3D) — point in the world coordinate system.
- `index` (int) — index of point which will be set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.SetPoint)

#### `SetRefPoint(refPoint)`

Set reference point in world coordinate system.

**Remarks:** Set reference point in world coordinate system.

**Parameters:**
- `refPoint` (Point3D) — reference point in the world coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.SetRefPoint)

#### `SetRelPoint(point, index)`

Set point at given position in relative coordinate system.

**Remarks:** Set point at given position in relative coordinate system.

**Parameters:**
- `point` (Point3D) — point in the relative coordinate system.
- `index` (int) — index of point which will be set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.SetRelPoint)

#### `SetStartPoint(startpoint)`

Set the start point in world coordinates

**Remarks:** Set the start point in world coordinates

**Parameters:**
- `startpoint` (Point3D) — new start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.SetStartPoint)

#### `ToLineChain()`

Get polyline as a chain of lines composed from 2 points.

**Remarks:** Get polyline as a chain of lines composed from 2 points.

**Returns:** `Point3DList` — vector of lines composed from 2 points (start and end point of a line)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.ToLineChain)

#### `__getitem__(index)`

Get point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range.

**Remarks:** Get point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range.

**Parameters:**
- `index` (int) — point index.

**Returns:** `Point3D` — point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.__getitem__)

#### `__iadd__(point)`

Add point in world coordinates.

**Remarks:** Add point in world coordinates.

**Parameters:**
- `point` (Point3D) — adding point.

**Returns:** `PolyPoints3D` — New point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.__iadd__)

#### `__mul__(matrix)`

2D matrix transformation

**Remarks:** 2D matrix transformation

**Parameters:**
- `matrix` (Matrix2D) — 2D transformation matrix

**Returns:** `PolyPoints3D` — Transformed polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.__mul__)

#### `__setitem__(arg2, value)`

Set point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range. Args: index: Specified position pnt: Point

**Remarks:** Set point at position from index. Used world coordinates. This method is checked and throwing Geometry::Exception when index is out of range. Args: index: Specified position pnt: Point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyPoints3D/#NemAll_Python_Geometry.PolyPoints3D.__setitem__)

### Properties
- `EndPoint` (Point3D, get/set) — Get the end point in world coordinate system.
- `EndRelPoint` (Point3D, get) — Get the end point
- `Points` (list[Point3D], get/set) — Get the point list
- `RefPoint` (Point3D, get/set) — Get reference point.
- `StartPoint` (Point3D, get/set) — Get the start point in world coordinate system.
- `StartRelPoint` (Point3D, get) — Get the start point

## Polygon2D (class)

Representation class for 2D Polygon

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/)

### Constructors
- `Polygon2D() | Polygon2D(pntList) | Polygon2D(polygon)` — Initialize

### Methods
#### `CreateRectangle(leftBottom, rightTop)`

Create a rectangle

**Remarks:** Create a rectangle

**Parameters:**
- `leftBottom` (Point2D) — Left bottom point
- `rightTop` (Point2D) — Right top point

**Returns:** `Polygon2D` — Polygon of the rectangle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.CreateRectangle)

#### `GetSegments()`

Get polygon segments

**Remarks:** Get polygon segments

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.GetSegments)

#### `IsValid()`

Check if the polygon is valid ( has at least 3 points ) For additional point validation use Service::Validate.

**Remarks:** Check if the polygon is valid ( has at least 3 points ) For additional point validation use Service::Validate.

**Returns:** `bool` — true if is valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.IsValid)

#### `Normalize(normalizeType=DEFAULT_NORM_TYPE, extra_smooth=False)`

Normalize Polygon2D. Using huge old algorithm, adding points at line crossings, reorganizing the lines, correcting gaps, ... This method is checked and throwing Exception when error occurs.

**Remarks:** Normalize Polygon2D. Using huge old algorithm, adding points at line crossings, reorganizing the lines, correcting gaps, ... This method is checked and throwing Exception when error occurs.

**Parameters:**
- `normalizeType` (ePolygonNormalizeType) — type of Polygon2D normalization
- `extra_smooth` (bool) — Including extra smooth: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.Normalize)

#### `NormalizeNoThrow(normalizeType=DEFAULT_NORM_TYPE, extra_smooth=False)`

Normalize Polygon2D. Same as Normalize, but method doesn't throw exception, just return error code

**Remarks:** Normalize Polygon2D. Same as Normalize, but method doesn't throw exception, just return error code

**Parameters:**
- `normalizeType` (ePolygonNormalizeType) — type of Polygon2D normalization
- `extra_smooth` (bool) — Including extra smooth: true/false

**Returns:** `object` — Error code (eOK, eAllocError, eStructuralError)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.NormalizeNoThrow)

#### `Reverse()`

Reverse the point order in polygon, separatelly for every subpolygon

**Remarks:** Reverse the point order in polygon, separatelly for every subpolygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.Reverse)

#### `__eq__(polygon2)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `polygon2` (Polygon2D) — Second polygon

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.__eq__)

#### `__iadd__(point)`

Addition assignment operator

**Remarks:** Addition assignment operator

**Parameters:**
- `point` (Point2D) — Point which will be added

**Returns:** `Polygon2D` — Reference to polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.__iadd__)

#### `__ne__(polygon2)`

Not equal operator

**Remarks:** Not equal operator

**Parameters:**
- `polygon2` (Polygon2D) — Second polygon

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2D/#NemAll_Python_Geometry.Polygon2D.__repr__)

## Polygon2DList (class)

List for Polygon2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/)

### Constructors
- `Polygon2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Polygon2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Polygon2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Polygon2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Polygon2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Polygon2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Polygon2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon2DList/#NemAll_Python_Geometry.Polygon2DList.extend)

## Polygon3D (class)

Representation class for 3D Polygon

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/)

### Constructors
- `Polygon3D() | Polygon3D(pntList) | Polygon3D(polygon)` — Initialize

### Methods
#### `GetLines()`

Get edge lines of polygon

**Remarks:** Get edge lines of polygon

**Returns:** `Line3DList` — Edge lines of polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.GetLines)

#### `GetPlane()`

Calculate plane

**Remarks:** Calculate plane

**Returns:** `tuple[eGeometryErrorCode, Plane3D]` — Plane where polygon is

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.GetPlane)

#### `GetVertices()`

Get polygon vertices

**Remarks:** Get polygon vertices

**Returns:** `object` — polygon vertices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.GetVertices)

#### `InsertPolygon(polygon, position=-1)`

Insert a polygon into current one

**Remarks:** Insert a polygon into current one

**Parameters:**
- `polygon` (Polygon3D) — Polygon which will be inserted
- `position` (int) — Position where the polygon will be inserted

**Returns:** `bool` — true if insert was successful

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.InsertPolygon)

#### `IsValid()`

Check polygon validity

**Remarks:** Check polygon validity

**Returns:** `bool` — true = valid, false = not valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.IsValid)

#### `IsValidStatus()`

Check polygon validity

**Remarks:** Check polygon validity

**Returns:** `tuple` — true = valid, false = not valid,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.IsValidStatus)

#### `Normalize(normalizeType=DEFAULT_NORM_TYPE, extra_smooth=False)`

Normalization of 3d polygon

**Remarks:** Normalization of 3d polygon

**Parameters:**
- `normalizeType` (ePolygonNormalizeType) — Normalization type
- `extra_smooth` (bool) — Increase level of details

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.Normalize)

#### `NormalizeNoThrow(normalizeType=DEFAULT_NORM_TYPE, extra_smooth=False)`

Normalize Polygon3D. Same as Normalize, but method doesn't throw exception, just return error code

**Remarks:** Normalize Polygon3D. Same as Normalize, but method doesn't throw exception, just return error code

**Parameters:**
- `normalizeType` (ePolygonNormalizeType) — type of Polygon2D normalization
- `extra_smooth` (bool) — Including extra smooth: true/false

**Returns:** `object` — Error code (eOK, eAllocError, eStructuralError, eWrongShape)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.NormalizeNoThrow)

#### `Reverse()`

Reverse the point order in polygon, separately for every sub-polygon

**Remarks:** Reverse the point order in polygon, separately for every sub-polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.Reverse)

#### `__eq__(polygon2)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `polygon2` (Polygon3D) — Second polygon

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.__eq__)

#### `__iadd__(polygon) | __iadd__(point)`

Addition assignment operator

**Remarks:** Addition assignment operator

**Parameters:**
- `polygon` (Polygon3D) — Polygon which will be copied

**Returns:** `Polygon3D` — Reference to polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.__iadd__)

#### `__mul__(matrix)`

Matrix transformation

**Remarks:** Matrix transformation

**Parameters:**
- `matrix` (Matrix3D) — Transformation matrix

**Returns:** `Polygon3D` — Reference to polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.__mul__)

#### `__ne__(polygon2)`

Not equal operator

**Remarks:** Not equal operator

**Parameters:**
- `polygon2` (Polygon3D) — Second polygon

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3D/#NemAll_Python_Geometry.Polygon3D.__repr__)

## Polygon3DList (class)

List for Polygon3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/)

### Constructors
- `Polygon3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Polygon3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Polygon3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Polygon3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Polygon3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Polygon3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Polygon3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polygon3DList/#NemAll_Python_Geometry.Polygon3DList.extend)

## PolygonalArea (class)

(No description provided in vendor docs for NemAll_Python_Geometry.PolygonalArea.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/)

### Methods
#### `AppendEdge(edge)`

Append edge (appendPolygonEdge).

**Remarks:** Append edge (appendPolygonEdge).

**Parameters:**
- `edge` (GeometryEdge) — Appended edge.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.AppendEdge)

#### `GetComponentsCount()`

Get count of components.

**Remarks:** Get count of components.

**Returns:** `int` — Count of component.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetComponentsCount)

#### `GetEdge(edgeIndex)`

Get edge.

**Remarks:** Get edge.

**Parameters:**
- `edgeIndex` (int) — index of the edge.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetEdge)

#### `GetEdges()`

Get copy of all edges.

**Remarks:** Get copy of all edges.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetEdges)

#### `GetEdgesCount()`

Get count of edges.

**Remarks:** Get count of edges.

**Returns:** `int` — Count of edges.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetEdgesCount)

#### `GetLoopEndEdgeIndex(loopIndex)`

Get the index of last edge of a loop.

**Remarks:** Get the index of last edge of a loop.

**Parameters:**
- `loopIndex` (int) — index of loop.

**Returns:** `int` — Index of the last edge of the loop identified by the loopIndex value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetLoopEndEdgeIndex)

#### `GetLoopsCount()`

Get Count of loops.

**Remarks:** Get Count of loops.

**Returns:** `int` — Count of loops.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetLoopsCount)

#### `GetNeighborEdges(edgeIndex)`

Get neighbor edges.

**Remarks:** Get neighbor edges.

**Parameters:**
- `edgeIndex` (int) — edge index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetNeighborEdges)

#### `GetNextEdge(edgeIndex)`

Get next edge.

**Remarks:** Get next edge.

**Parameters:**
- `edgeIndex` (int) — edge index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetNextEdge)

#### `GetPlane()`

Get plane if polygon is plane.

**Remarks:** Get plane if polygon is plane.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetPlane)

#### `GetPrevEdge(edgeIndex)`

Get previous edge.

**Remarks:** Get previous edge.

**Parameters:**
- `edgeIndex` (int) — edge index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetPrevEdge)

#### `GetVector()`

Get Normal vector if polygon is plane.

**Remarks:** Get Normal vector if polygon is plane.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetVector)

#### `GetVerticesCount()`

Get count of vertices.

**Remarks:** Get count of vertices.

**Returns:** `int` — Count of vertices.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.GetVerticesCount)

#### `SetPlane(plane)`

Set plane.

**Remarks:** Set plane.

**Parameters:**
- `plane` (Plane3D) — plane which will be set.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.SetPlane)

#### `SetVector(vec)`

Set normal vector.

**Remarks:** Set normal vector.

**Parameters:**
- `vec` (Vector3D) — vector which will be set.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea/#NemAll_Python_Geometry.PolygonalArea.SetVector)

## PolygonalArea2D (class)

2D polygonal area class for 2D polygonal area geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/)

### Constructors
- `PolygonalArea2D() | PolygonalArea2D(verticesCount, edgesCount) | PolygonalArea2D(refPoint, verticesCount, edgesCount) | PolygonalArea2D(polygon)` — Initialize

### Methods
#### `AppendRelVertex(vertex)`

Append vertex in local coordinate system. Warning: method does not update edges.

**Remarks:** Append vertex in local coordinate system. Warning: method does not update edges.

**Parameters:**
- `vertex` (Point2D) — vertex in local coordinate system which will be appended.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.AppendRelVertex)

#### `AppendVertex(vertex)`

Append vertex in World coordinate system. Warning: method does not update edges.

**Remarks:** Append vertex in World coordinate system. Warning: method does not update edges.

**Parameters:**
- `vertex` (Point2D) — vertex in world coordinate system which will be appended.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.AppendVertex)

#### `EqualRef(polygon)`

Test for equal reference points.

**Remarks:** Test for equal reference points.

**Parameters:**
- `polygon` (PolygonalArea2D) — polygon for comparision.

**Returns:** `bool` — True if reference points are equal else return false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.EqualRef)

#### `GetEdgeVertices(edgeIndex)`

Get edge vertices.

**Remarks:** Get edge vertices.

**Parameters:**
- `edgeIndex` (int) — edge index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.GetEdgeVertices)

#### `GetPolygon()`

Get polygon. Old interface: pgon_to_punkte

**Remarks:** Get polygon. Old interface: pgon_to_punkte

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.GetPolygon)

#### `GetRefPoint()`

Get the reference point.

**Remarks:** Get the reference point.

**Returns:** `Point2D` — constant reference to point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.GetRefPoint)

#### `GetRelVertex(vertexIndex)`

Get vertex in Local coordinate system.

**Remarks:** Get vertex in Local coordinate system.

**Parameters:**
- `vertexIndex` (int) — vertex index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.GetRelVertex)

#### `GetVertex(vertexIndex)`

Get vertex in World coordinate system.

**Remarks:** Get vertex in World coordinate system.

**Parameters:**
- `vertexIndex` (int) — vertex index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.GetVertex)

#### `SetRefPoint(refPoint)`

Set reference point in world coordinate system. Coordinates of points will be recalculated with new reference point.

**Remarks:** Set reference point in world coordinate system. Coordinates of points will be recalculated with new reference point.

**Parameters:**
- `refPoint` (Point2D) — new reference point.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.SetRefPoint)

#### `__eq__(polygonalArea)`

Comparison of polygonalAreas without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of polygonalAreas without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `polygonalArea` (PolygonalArea2D) — Compared polygonalArea.

**Returns:** `object` — True when polygonalAreas are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.__eq__)

#### `__getitem__(vertexIndex)`

Get point at position from index. Used world coordinates.

**Remarks:** Get point at position from index. Used world coordinates.

**Parameters:**
- `vertexIndex` (int) — vertex index.

**Returns:** `Point2D` — Copy of vertex[vertexIndex] in world coordinates.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.__getitem__)

#### `__iadd__(polygonalarea) | __iadd__(polygon)`

Append polygon.

**Remarks:** Append polygon.

**Parameters:**
- `polygonalarea` (PolygonalArea2D) — polygonal area which will be appended.

**Returns:** `object` — Reference to PolygonalArea2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.__iadd__)

#### `__mul__(matrix)`

Matrix transformation.

**Remarks:** Matrix transformation.

**Parameters:**
- `matrix` (Matrix2D) — point index.

**Returns:** `object` — Transformed 2D polygonal area.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.__mul__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2D/#NemAll_Python_Geometry.PolygonalArea2D.__repr__)

### Properties
- `RefPoint` (None, get) — Get and set the reference point in world coordinate system as property :type: None

## PolygonalArea2DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.PolygonalArea2DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/)

### Constructors
- `PolygonalArea2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (PolygonalArea2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (PolygonalArea2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `PolygonalArea2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (PolygonalArea2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (PolygonalArea2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (PolygonalArea2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea2DList/#NemAll_Python_Geometry.PolygonalArea2DList.extend)

## PolygonalArea3D (class)

3D polygonal area class for 3D polygonal area geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/)

### Constructors
- `PolygonalArea3D() | PolygonalArea3D(verticesCount, edgesCount) | PolygonalArea3D(refPoint, verticesCount, edgesCount) | PolygonalArea3D(polygon)` — Initialize

### Methods
#### `AppendRelVertex(vertex)`

Append vertex in local coordinate system. Warning: method does not update edges.

**Remarks:** Append vertex in local coordinate system. Warning: method does not update edges.

**Parameters:**
- `vertex` (Point3D) — vertex in local coordinate system which will be appended.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.AppendRelVertex)

#### `AppendVertex(vertex)`

Append vertex in World coordinate system. Warning: method does not update edges.

**Remarks:** Append vertex in World coordinate system. Warning: method does not update edges.

**Parameters:**
- `vertex` (Point3D) — vertex in world coordinate system which will be appended.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.AppendVertex)

#### `EqualRef(polygon)`

Test for equal reference points.

**Remarks:** Test for equal reference points.

**Parameters:**
- `polygon` (PolygonalArea3D) — polygonal area for comparision.

**Returns:** `bool` — True if reference points are equal else return false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.EqualRef)

#### `GetEdgeVertices(edgeIndex)`

Get edge vertices.

**Remarks:** Get edge vertices.

**Parameters:**
- `edgeIndex` (int) — edge index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.GetEdgeVertices)

#### `GetPolygon()`

Get polygon. Returns empty polygon, if it has zero edges.

**Remarks:** Get polygon. Returns empty polygon, if it has zero edges.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.GetPolygon)

#### `GetRefPoint()`

Get the reference point.

**Remarks:** Get the reference point.

**Returns:** `Point3D` — constant reference to point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.GetRefPoint)

#### `GetRelVertex(vertexIndex)`

Get vertex in Local coordinate system.

**Remarks:** Get vertex in Local coordinate system.

**Parameters:**
- `vertexIndex` (int) — vertex index.

**Returns:** `tuple` — vertex in local coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.GetRelVertex)

#### `GetVertex(vertexIndex)`

Get vertex in World coordinate system.

**Remarks:** Get vertex in World coordinate system.

**Parameters:**
- `vertexIndex` (int) — vertex index.

**Returns:** `tuple` — Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.GetVertex)

#### `Reverse()`

Reverse the point order of boundary polygons of the area.

**Remarks:** Reverse the point order of boundary polygons of the area.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.Reverse)

#### `SetRefPoint(refPoint)`

Set reference point in world coordinate system. Coordinates of points will be recalculated with new reference point.

**Remarks:** Set reference point in world coordinate system. Coordinates of points will be recalculated with new reference point.

**Parameters:**
- `refPoint` (Point3D) — new reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.SetRefPoint)

#### `__eq__(polygonalArea)`

Comparison of polygonalAreas without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of polygonalAreas without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `polygonalArea` (PolygonalArea3D) — Compared polygonalArea.

**Returns:** `object` — True when polygonalAreas are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.__eq__)

#### `__getitem__(arg2)`

Get point at position from index. Used world coordinates.

**Remarks:** Get point at position from index. Used world coordinates.

**Parameters:**
- `vertexIndex` (object) — vertex index.

**Returns:** `Point3D` — Copy of vertex[vertexIndex] in world coordinates.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.__getitem__)

#### `__iadd__(polygonalarea) | __iadd__(polygon)`

Append polygon.

**Remarks:** Append polygon.

**Parameters:**
- `polygonalarea` (PolygonalArea3D) — polygonal area which will be appended.

**Returns:** `object` — Reference to PolygonalArea3D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.__iadd__)

#### `__mul__(matrix) | __mul__(matrix)`

2D matrix transformation.

**Remarks:** 2D matrix transformation.

**Parameters:**
- `matrix` (Matrix2D) — 2D transformation matrix.

**Returns:** `object` — Transformed 3D polygon area.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.__mul__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3D/#NemAll_Python_Geometry.PolygonalArea3D.__repr__)

### Properties
- `RefPoint` (None, get) — Get and set the reference point in world coordinate system as property :type: None

## PolygonalArea3DList (class)

(No description provided in vendor docs for NemAll_Python_Geometry.PolygonalArea3DList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/)

### Constructors
- `PolygonalArea3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (PolygonalArea3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (PolygonalArea3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `PolygonalArea3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (PolygonalArea3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (PolygonalArea3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (PolygonalArea3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolygonalArea3DList/#NemAll_Python_Geometry.PolygonalArea3DList.extend)

## Polyhedron3D (class)

Representation class for 3D polyhedron.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/)

### Constructors
- `Polyhedron3D() | Polyhedron3D(polyhedronType, verticesCount, edgesCount, facesCount, negativeOrientation) | Polyhedron3D(polyhedron)` — Initialize

### Methods
#### `AppendEdge(edge)`

Append edge Method throw exception if object is not initialized. Old interface: appendPolyederEdge

**Remarks:** Append edge Method throw exception if object is not initialized. Old interface: appendPolyederEdge

**Parameters:**
- `edge` (GeometryEdge) — Appended edge.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.AppendEdge)

#### `Clear() | Clear(verticesCount, edgesCount, facesCount, negativeOrientation)`

Clear all vertices, edges and faces Type and vertices,edges,faces preallocation size of polyhedron will be preserved

**Remarks:** Clear all vertices, edges and faces Type and vertices,edges,faces preallocation size of polyhedron will be preserved

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.Clear)

#### `CreateCuboid(p1, p2) | CreateCuboid(box) | CreateCuboid(placement, length, width, height) | CreateCuboid(length, width, height)`

static constructor for cuboid

**Remarks:** static constructor for cuboid

**Parameters:**
- `p1` (Point3D) — lower point of min/max box that states the cuboid's size
- `p2` (Point3D) — lower point of min/max box that states the cuboid's size

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.CreateCuboid)

#### `CreateFace(expectedEdges)`

Create face and append it to polyhedron Method throw exception if object is not initialized. Old interface: appendPolyederFace

**Remarks:** Create face and append it to polyhedron Method throw exception if object is not initialized. Old interface: appendPolyederFace

**Parameters:**
- `expectedEdges` (int) — Appended face.

**Returns:** `PolyhedronFace` — Created face.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.CreateFace)

#### `DeleteEdge(edgeHandle)`

Delete edge Method throw exception if object is not initialized. Old interface: deletePolyederEdge

**Remarks:** Delete edge Method throw exception if object is not initialized. Old interface: deletePolyederEdge

**Parameters:**
- `edgeHandle` (int) — Edge to delete

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.DeleteEdge)

#### `DeleteFace(faceIndex)`

Delete face at specified position Method throw exception in case of any error. Old interface: deleteFace

**Remarks:** Delete face at specified position Method throw exception in case of any error. Old interface: deleteFace

**Parameters:**
- `faceIndex` (int) — Position of the deleted face. Zero based.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.DeleteFace)

#### `DeleteFaces(faceIndices)`

Delete faces at specified positions Method throw exception in case of any error. Old interface: deleteFace

**Remarks:** Delete faces at specified positions Method throw exception in case of any error. Old interface: deleteFace

**Parameters:**
- `faceIndices` (VecSizeTList) — Positions of the deleted faces. Zero based.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.DeleteFaces)

#### `EqualRef(polyhedron)`

Test for equal reference point

**Remarks:** Test for equal reference point

**Parameters:**
- `polyhedron` (Polyhedron3D) — Tested polyhedron.

**Returns:** `bool` — True when both polyhedrons has the same reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.EqualRef)

#### `GetEdge(edgeHandle) | GetEdge(orientedEdge)`

Get edge at the specified position

**Remarks:** Get edge at the specified position

**Parameters:**
- `edgeHandle` (int) — Specified position of the edge.

**Returns:** `eGeometryErrorCode` — tuple(Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetEdge)

#### `GetEdgeVertices(orientedEdge)`

Get points on specified edge from specified face Method throw exception in case of any error.

**Remarks:** Get points on specified edge from specified face Method throw exception in case of any error.

**Parameters:**
- `orientedEdge` (OrientedEdge) — Oriented edge.

**Returns:** `eGeometryErrorCode` — tuple(Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetEdgeVertices)

#### `GetEdges()`

Get copy of all edges Old interface: getPolyederAllEdges

**Remarks:** Get copy of all edges Old interface: getPolyederAllEdges

**Returns:** `eGeometryErrorCode` — tuple(Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetEdges)

#### `GetEdgesCount()`

Get count of edges Method throw exception in case of any error. Count

**Remarks:** Get count of edges Method throw exception in case of any error. Count

**Returns:** `int` — Count of edges.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetEdgesCount)

#### `GetEdgesLines()`

Get copy of all edges as vector of lines

**Remarks:** Get copy of all edges as vector of lines

**Returns:** `eGeometryErrorCode` — tuple(Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetEdgesLines)

#### `GetEdgesOnFaceCount(faceIndex)`

Get count of edges at the specified face Method throw exception in case of any error. In case of error, return 0. Count

**Remarks:** Get count of edges at the specified face Method throw exception in case of any error. In case of error, return 0. Count

**Parameters:**
- `faceIndex` (int) — Specified position of the face.

**Returns:** `int` — Count if edges.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetEdgesOnFaceCount)

#### `GetFace(faceIndex)`

Get face at the specified position In case of error, method throw an exception.

**Remarks:** Get face at the specified position In case of error, method throw an exception.

**Parameters:**
- `faceIndex` (int) — Specified position of the face.

**Returns:** `PolyhedronFace` — Face.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetFace)

#### `GetFacesCount()`

Get count of faces Method throw exception in case of any error.

**Remarks:** Get count of faces Method throw exception in case of any error.

**Returns:** `int` — Count of faces.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetFacesCount)

#### `GetNormalVectorOfFace(faceIndex)`

Get normal vector of the face

**Remarks:** Get normal vector of the face

**Parameters:**
- `faceIndex` (int) — face index

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetNormalVectorOfFace)

#### `GetParts()`

Get separated parts (continuos shells)

**Remarks:** Get separated parts (continuos shells)

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetParts)

#### `GetPartsCount()`

Get number of parts in this polyhedron

**Remarks:** Get number of parts in this polyhedron

**Returns:** `int` — number of parts

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetPartsCount)

#### `GetRefPoint()`

Get the reference point

**Remarks:** Get the reference point

**Returns:** `Point3D` — Constant reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetRefPoint)

#### `GetRelVertex(index)`

Get relative vertex at specified position Method throw exception in case of any error. (usually out of range)

**Remarks:** Get relative vertex at specified position Method throw exception in case of any error. (usually out of range)

**Parameters:**
- `index` (int) — Specified position.

**Returns:** `eGeometryErrorCode` — tuple(Error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetRelVertex)

#### `GetType()`

Get polyhedron type In case of error, method return tEdges.

**Remarks:** Get polyhedron type In case of error, method return tEdges.

**Returns:** `PolyhedronType` — Type.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetType)

#### `GetVertex(vertexIndex)`

Get vertex at specified position Method throw exception in case of any error.

**Remarks:** Get vertex at specified position Method throw exception in case of any error.

**Parameters:**
- `vertexIndex` (int) — Specified position of vertex.

**Returns:** `eGeometryErrorCode` — tuple(Error code.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetVertex)

#### `GetVertices()`

Get copy of vertices in world coordinate system

**Remarks:** Get copy of vertices in world coordinate system

**Returns:** `Point3DList` — Vector with all vertices as Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetVertices)

#### `GetVerticesCount()`

Get count of vertices Method throw exception in case of any error. Count

**Remarks:** Get count of vertices Method throw exception in case of any error. Count

**Returns:** `int` — Count of vertices.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.GetVerticesCount)

#### `Heal()`

Heal polyhedron by splitting non-planar faces to triangles

**Remarks:** Heal polyhedron by splitting non-planar faces to triangles

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.Heal)

#### `Invert()`

Invert polyhedron from positive to negative or vice versa

**Remarks:** Invert polyhedron from positive to negative or vice versa

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.Invert)

#### `InvertWithFlagUnchanged()`

Invert polyhedron from positive to negative or vice versa, but keep flag unchanged

**Remarks:** Invert polyhedron from positive to negative or vice versa, but keep flag unchanged

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.InvertWithFlagUnchanged)

#### `IsNegative()`

Checking the negative orientation Method throw exception in case of any error. Old interface: getPolyederNega

**Remarks:** Checking the negative orientation Method throw exception in case of any error. Old interface: getPolyederNega

**Returns:** `bool` — True when negative orientation, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.IsNegative)

#### `IsValid()`

Checking validity Depends on polyhedron type

**Remarks:** Checking validity Depends on polyhedron type

**Returns:** `bool` — True when valid, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.IsValid)

#### `Normalize(normType)`

Normalize polyhedron

**Remarks:** Normalize polyhedron

**Parameters:**
- `normType` (int) — Type of normalization.

**Returns:** `object` — Error code.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.Normalize)

#### `ReadFromStream(sstream_str)`

Read Polyhedron3D from stream

**Remarks:** Read Polyhedron3D from stream

**Parameters:**
- `sstream_str` (str) — input stream

**Returns:** `object` — error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.ReadFromStream)

#### `RemapVertices(source)`

set vertex order as in source polyhedron

**Remarks:** set vertex order as in source polyhedron

**Parameters:**
- `source` (Polyhedron3D) — original cuboid

**Returns:** `bool` — success

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.RemapVertices)

#### `Set(refPoint, polyhedronWorld)`

Set polyhedron on reference point. Polyhedron is in world coordinate system

**Remarks:** Set polyhedron on reference point. Polyhedron is in world coordinate system

**Parameters:**
- `refPoint` (Point3D) — Reference point.
- `polyhedronWorld` (Polyhedron3D) — Polyhedron with vertices in World coordinate system.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.Set)

#### `SetRefPoint(refPoint)`

Set the reference point Be aware: All vertices will be recalculated on new reference point. This operation is very slow in case of many vertices. Use constructor for better and faster initialization object.

**Remarks:** Set the reference point Be aware: All vertices will be recalculated on new reference point. This operation is very slow in case of many vertices. Use constructor for better and faster initialization object.

**Parameters:**
- `refPoint` (Point3D) — New reference point.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.SetRefPoint)

#### `SetType(polyheronType)`

Get polyhedron type Method throw exception if polyhedron is not initialized. In case of incorrect polyhedronType, tEdges type will be set.

**Remarks:** Get polyhedron type Method throw exception if polyhedron is not initialized. In case of incorrect polyhedronType, tEdges type will be set.

**Parameters:**
- `polyheronType` (PolyhedronType) — Type of polyhedron.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.SetType)

#### `WriteToStream()`

Write Polyhedron3D to the stream

**Remarks:** Write Polyhedron3D to the stream

**Returns:** `eGeometryErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.WriteToStream)

#### `__eq__(polyhedron)`

Comparison of polyhedrons without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of polyhedrons without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `polyhedron` (Polyhedron3D) — Compared polyhedron.

**Returns:** `object` — True when polyhedrons are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.__eq__)

#### `__getitem__(index)`

Get vertex at the specified position from index. Used world coordinates Operator throw exception in case of any error.

**Remarks:** Get vertex at the specified position from index. Used world coordinates Operator throw exception in case of any error.

**Parameters:**
- `index` (int) — Specified position

**Returns:** `Point3D` — Vertex

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.__getitem__)

#### `__mul__(matrix)`

Matrix transformation of vertices

**Remarks:** Matrix transformation of vertices

**Parameters:**
- `matrix` (Matrix3D) — Transformation matrix.

**Returns:** `Polyhedron3D` — Transformed polyhedron.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3D/#NemAll_Python_Geometry.Polyhedron3D.__repr__)

### Properties
- `RefPoint` (Point3D, get/set) — Get the reference point Be aware: All vertices will be recalculated on new reference point. This operation is very slow in case of many vertices. Use constructor for better and faster initialization object.
- `Type` (PolyhedronType, get/set) — Get polyhedron type In case of error, method return tEdges. Get polyhedron type Method throw exception if polyhedron is not initialized. In case of incorrect polyhedronType, tEdges type will be set.

## Polyhedron3DBuilder (class)

3D polyhedron builder

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DBuilder/)

### Constructors
- `Polyhedron3DBuilder(polyhedron)` — Constructor

### Methods
#### `AppendVertex(vertex)`

Append new vertex

**Remarks:** Append new vertex

**Parameters:**
- `vertex` (Point3D) — Point in world coordinate system

**Returns:** `tuple` — eOK if success,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DBuilder/#NemAll_Python_Geometry.Polyhedron3DBuilder.AppendVertex)

#### `Complete()`

Finalize polyhedron Recalculate cached minmax of modified polyhedron

**Remarks:** Finalize polyhedron Recalculate cached minmax of modified polyhedron

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DBuilder/#NemAll_Python_Geometry.Polyhedron3DBuilder.Complete)

#### `SetVertex(vertexIndex, vertex)`

Set vertex at the given index

**Remarks:** Set vertex at the given index

**Parameters:**
- `vertexIndex` (int) — Index of updated vertex
- `vertex` (Point3D) — Point in world coordinate system

**Returns:** `object` — eOK if success

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DBuilder/#NemAll_Python_Geometry.Polyhedron3DBuilder.SetVertex)

## Polyhedron3DList (class)

List for Polyhedron3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/)

### Constructors
- `Polyhedron3DList() | Polyhedron3DList(ele) | Polyhedron3DList(eleList)` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Polyhedron3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Polyhedron3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (Polyhedron3DList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Polyhedron3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__getitem__)

#### `__iadd__(eleList)`

Add a list

**Remarks:** Add a list

**Parameters:**
- `eleList` (list) — Polyhedron3D list

**Returns:** `Polyhedron3DList` — Lists with the added elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__iadd__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Polyhedron3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Polyhedron3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.append)

#### `extend(iterable) | extend(eleList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Polyhedron3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyhedron3DList/#NemAll_Python_Geometry.Polyhedron3DList.extend)

## PolyhedronFace (class)

Polyhedron face All constructors are prohibited, only Polyhedron3D can instantiate this class. This behavior is dependant on old %Allplan architecture and can be changed.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/)

### Constructors
- `PolyhedronFace(face)` — Standard copy constructor

### Methods
#### `AppendEdge(edgeHandle, positiveOrientation) | AppendEdge(edge)`

Append edge into face

**Remarks:** Append edge into face

**Parameters:**
- `edgeHandle` (int) — Appended EdgeHandle.
- `positiveOrientation` (bool) — True for positive, false for negative orientation.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.AppendEdge)

#### `AppendEdges(edges)`

Append edges into face

**Remarks:** Append edges into face

**Parameters:**
- `edges` (OrientedEdgeList) — Vector of edges.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.AppendEdges)

#### `GetEdge(edgeIndex)`

Get the edge index on specified position Old interface: getFaceEdge

**Remarks:** Get the edge index on specified position Old interface: getFaceEdge

**Parameters:**
- `edgeIndex` (int) — Specified position in edge vector.

**Returns:** `tuple` — False when edgeIndex is out of range or when occurred another error, otherwise true.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.GetEdge)

#### `GetEdges()`

Get the edge index on specified position

**Remarks:** Get the edge index on specified position

**Returns:** `OrientedEdgeList` — Vector of oriented edges.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.GetEdges)

#### `GetEdgesCount()`

Get count of edges Old interface: getFaceEdgeCount

**Remarks:** Get count of edges Old interface: getFaceEdgeCount

**Returns:** `int` — Count of edges.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.GetEdgesCount)

#### `GetFlags()`

Get the flags of the face

**Remarks:** Get the flags of the face

**Returns:** `int` — Face flags.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.GetFlags)

#### `SetFlags(flags)`

Set the flags of the face

**Remarks:** Set the flags of the face

**Parameters:**
- `flags` (int) — Face flags.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.SetFlags)

#### `__getitem__(edgeIndex)`

Get the edge index on specified position Method can throw exception when index is out of range. Please use the GetEdgeIndex with return flag when you do not used exception handler. Old interface: getFaceEdge

**Remarks:** Get the edge index on specified position Method can throw exception when index is out of range. Please use the GetEdgeIndex with return flag when you do not used exception handler. Old interface: getFaceEdge

**Parameters:**
- `edgeIndex` (int) — Specified position in edge vector.

**Returns:** `OrientedEdge` — Edge index, not reference.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.__getitem__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronFace/#NemAll_Python_Geometry.PolyhedronFace.__repr__)

## PolyhedronType (enum)

Type of polyhedron visual representation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `PolyhedronType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/PolyhedronType/#NemAll_Python_Geometry.PolyhedronType.__getitem__)

### Values
- `tEdges` = `1`
- `tFaces` = `2`
- `tInvalid` = `0`
- `tVolume` = `3`

## Polyline2D (class)

Representation class for 2D Polyline.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/)

### Constructors
- `Polyline2D() | Polyline2D(pntList) | Polyline2D(polyline) | Polyline2D(points) | Polyline2D(polyline, skip, count) | Polyline2D(polygon)` — Initialize

### Methods
#### `GetLine(index)`

Extract a line

**Remarks:** Extract a line

**Parameters:**
- `index` (int) — Index of the line

**Returns:** `Line2D` — Line from the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.GetLine)

#### `GetLines()`

Get all lines from this polyline

**Remarks:** Get all lines from this polyline

**Returns:** `Line2DList` — count of extracted lines,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.GetLines)

#### `IsValid()`

Check if the polygon is valid ( has at least 2 points ) For additional point validation use Service::Validate.

**Remarks:** Check if the polygon is valid ( has at least 2 points ) For additional point validation use Service::Validate.

**Returns:** `bool` — true if is valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.IsValid)

#### `LineCount()`

Get the count of lines connecting the points

**Remarks:** Get the count of lines connecting the points

**Returns:** `int` — count of lines

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.LineCount)

#### `Reverse()`

Reverse orientation of the Polyline

**Remarks:** Reverse orientation of the Polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.Reverse)

#### `__eq__(polyline2)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `polyline2` (Polyline2D) — Second polyline

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.__eq__)

#### `__iadd__(polyline) | __iadd__(point)`

Addition assignment operator

**Remarks:** Addition assignment operator

**Parameters:**
- `polyline` (Polyline2D) — Polyline which will be added

**Returns:** `Polyline2D` — Reference to polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.__iadd__)

#### `__ne__(polyline2)`

Not equal operator

**Remarks:** Not equal operator

**Parameters:**
- `polyline2` (Polyline2D) — Second polyline

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2D/#NemAll_Python_Geometry.Polyline2D.__repr__)

## Polyline2DList (class)

List for Polyline2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/)

### Constructors
- `Polyline2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Polyline2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Polyline2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Polyline2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Polyline2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Polyline2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Polyline2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline2DList/#NemAll_Python_Geometry.Polyline2DList.extend)

## Polyline3D (class)

Representation class for 3D Polyline.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/)

### Constructors
- `Polyline3D() | Polyline3D(pntList) | Polyline3D(polyline) | Polyline3D(polyline, skip, count) | Polyline3D(points)` — Initialize

### Methods
#### `GetLine(index)`

Extract a line

**Remarks:** Extract a line

**Parameters:**
- `index` (int) — Index of the line

**Returns:** `Line3D` — indexed line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.GetLine)

#### `GetLines()`

get lines from polyline

**Remarks:** get lines from polyline

**Returns:** `Line3DList` — lines

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.GetLines)

#### `InsertPolyline(polyline, position=18446744073709551615)`

Insert another polyline at given position

**Remarks:** Insert another polyline at given position

**Parameters:**
- `polyline` (Polyline3D) — Polyline to be inserted
- `position` (int) — Position where the polyline has to be inserted

**Returns:** `bool` — true if successful

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.InsertPolyline)

#### `IsPlanar()`

Check if polyline is on one plane

**Remarks:** Check if polyline is on one plane

**Returns:** `bool` — tuple(True if polyline is on one plane,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.IsPlanar)

#### `IsValid()`

Check if the polygon is valid ( has at least 2 points ) For additional point validation use Service::Validate.

**Remarks:** Check if the polygon is valid ( has at least 2 points ) For additional point validation use Service::Validate.

**Returns:** `bool` — true if is valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.IsValid)

#### `LineCount()`

Get the count of lines connecting the points

**Remarks:** Get the count of lines connecting the points

**Returns:** `int` — count of lines

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.LineCount)

#### `Reverse()`

Reverse Polyline orientation

**Remarks:** Reverse Polyline orientation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.Reverse)

#### `__eq__(polyline2)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `polyline2` (Polyline3D) — Second polyline

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.__eq__)

#### `__iadd__(polyline) | __iadd__(point) | __iadd__(line)`

Additional assignment operator

**Remarks:** Additional assignment operator

**Parameters:**
- `polyline` (Polyline3D) — Polyline which will be added to current polyline

**Returns:** `Polyline3D` — Reference to polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.__iadd__)

#### `__mul__(matrix)`

Multiple Polyline with matrix

**Remarks:** Multiple Polyline with matrix

**Parameters:**
- `matrix` (Matrix3D) — Transformation matrix

**Returns:** `Polyline3D` — Transformed polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.__mul__)

#### `__ne__(polyline2)`

Not equal operator

**Remarks:** Not equal operator

**Parameters:**
- `polyline2` (Polyline3D) — Second polyline

**Returns:** `bool` — Polyline3D are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3D/#NemAll_Python_Geometry.Polyline3D.__repr__)

## Polyline3DList (class)

List for Polyline3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/)

### Constructors
- `Polyline3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Polyline3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Polyline3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Polyline3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Polyline3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Polyline3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Polyline3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Polyline3DList/#NemAll_Python_Geometry.Polyline3DList.extend)

## Spline2D (class)

Representation class for 2D spline.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/)

### Constructors
- `Spline2D() | Spline2D(spline) | Spline2D(pntList)` — Initialize

### Methods
#### `GetEndVector()`

Get end vector.

**Remarks:** Get end vector.

**Returns:** `Vector2D` — end vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.GetEndVector)

#### `GetStartVector()`

Get start vector.

**Remarks:** Get start vector.

**Returns:** `Vector2D` — start vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.GetStartVector)

#### `IsClosed()`

Check if spline is closed ( first/last points are equal )

**Remarks:** Check if spline is closed ( first/last points are equal )

**Returns:** `bool` — closed spline true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.IsClosed)

#### `Reverse()`

Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

**Remarks:** Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.Reverse)

#### `SetEndVector(vec)`

Set end vector.

**Remarks:** Set end vector.

**Parameters:**
- `vec` (Vector2D) — new end vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.SetEndVector)

#### `SetStartVector(vec)`

Set start vector.

**Remarks:** Set start vector.

**Parameters:**
- `vec` (Vector2D) — new start vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.SetStartVector)

#### `__eq__(spline)`

Comparison of splines without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of splines without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `spline` (Spline2D) — Compared spline.

**Returns:** `object` — True when splines are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.__eq__)

#### `__mul__(matrix)`

Multiple Spline with matrix.

**Remarks:** Multiple Spline with matrix.

**Parameters:**
- `matrix` (Matrix2D) — Transformation matrix.

**Returns:** `Spline2D` — Transformed spline.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.__mul__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2D/#NemAll_Python_Geometry.Spline2D.__repr__)

### Properties
- `EndVector` (Vector2D, get/set) — Get end vector.
- `StartVector` (Vector2D, get/set) — Get start vector.

## Spline2DList (class)

List for Spline2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/)

### Constructors
- `Spline2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Spline2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Spline2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Spline2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Spline2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Spline2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Spline2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline2DList/#NemAll_Python_Geometry.Spline2DList.extend)

## Spline3D (class)

class for 3D spline geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/)

### Constructors
- `Spline3D() | Spline3D(splinePoints) | Spline3D(spline) | Spline3D(spline, zPlane)` — Initialize

### Methods
#### `CalculateEndVector()`

Calculates end vector

**Remarks:** Calculates end vector

**Returns:** `Vector3D` — End vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.CalculateEndVector)

#### `CalculatePoint(param) | CalculatePoint(param, cpoints)`

Calculates point on spline

**Remarks:** Calculates point on spline

**Parameters:**
- `param` (float) — parameter of spline.

**Returns:** `Point3D` — Resulting point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.CalculatePoint)

#### `CalculateStartVector()`

Calculates start vector

**Remarks:** Calculates start vector

**Returns:** `Vector3D` — Start vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.CalculateStartVector)

#### `CreateClosedSpline(points)`

Create closed spline from given points

**Remarks:** Create closed spline from given points

**Parameters:**
- `points` (Point3DList) — Points

**Returns:** `Spline3D` — Spline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.CreateClosedSpline)

#### `GetControlPoints()`

Compute bezier control points

**Remarks:** Compute bezier control points

**Returns:** `GeoErrorCode` — tuple(error code,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.GetControlPoints)

#### `GetEndVector()`

Get end vector.

**Remarks:** Get end vector.

**Returns:** `Vector3D` — end vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.GetEndVector)

#### `GetStartVector()`

Get start vector.

**Remarks:** Get start vector.

**Returns:** `Vector3D` — start vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.GetStartVector)

#### `IsClosed()`

Check if spline is closed ( first/last points are equal )

**Remarks:** Check if spline is closed ( first/last points are equal )

**Returns:** `bool` — closed spline true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.IsClosed)

#### `IsCollinear()`

Function checks if the 3D spline is collinear - all control points are on same line

**Remarks:** Function checks if the 3D spline is collinear - all control points are on same line

**Returns:** `bool` — true if collinear

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.IsCollinear)

#### `IsPlanar()`

Function checks if the 3D spline is planar, if yes sets the plane

**Remarks:** Function checks if the 3D spline is planar, if yes sets the plane

**Returns:** `bool` — tuple(true if planar,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.IsPlanar)

#### `Reverse()`

Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

**Remarks:** Reverse of current spline Method reverse Spline using reverse from PolyPoints and swapping tangents.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.Reverse)

#### `SetEndVector(vec)`

Set end vector.

**Remarks:** Set end vector.

**Parameters:**
- `vec` (Vector3D) — new end vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.SetEndVector)

#### `SetStartVector(vec)`

Set start vector.

**Remarks:** Set start vector.

**Parameters:**
- `vec` (Vector3D) — new start vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.SetStartVector)

#### `__eq__(spline)`

Comparison of splines without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of splines without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `spline` (Spline3D) — Compared spline.

**Returns:** `object` — True when splines are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.__eq__)

#### `__iadd__(point)`

Addition assignment operator

**Remarks:** Addition assignment operator

**Parameters:**
- `point` (Point3D) — New Point3D which will be added to the spline

**Returns:** `Spline3D` — Reference to spline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.__iadd__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3D/#NemAll_Python_Geometry.Spline3D.__repr__)

### Properties
- `EndVector` (Vector3D, get/set) — Get end vector.
- `IsPeriodic` (bool, get/set) — Check if spline is periodic ( first/last points are equal + start and end tangents are equal)
- `StartVector` (Vector3D, get/set) — Get start vector.

## Spline3DList (class)

List for Spline3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/)

### Constructors
- `Spline3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Spline3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Spline3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Spline3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Spline3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Spline3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Spline3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Spline3DList/#NemAll_Python_Geometry.Spline3DList.extend)

## SweepRotationType (enum)

Type of the sweep rotation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/SweepRotationType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `SweepRotationType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/SweepRotationType/#NemAll_Python_Geometry.SweepRotationType.__getitem__)

### Values
- `Locked` = `0`
- `RailLocked` = `2`
- `Unlocked` = `1`

## TangentCalculus (class)

Tangent calculation at an element point

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TangentCalculus/)

### Methods
#### `Calculate(line) | Calculate(line) | Calculate(polyline, inputPnt) | Calculate(polyline, inputPnt) | Calculate(polygon, inputPnt) | Calculate(arc, inputPnt) | Calculate(arc, inputPnt) | Calculate(spline, inputPnt) | Calculate(spline, inputPnt) | Calculate(bspline, inputPnt) | Calculate(bspline, inputPnt) | Calculate(clothoid, inputPnt) | Calculate(path, inputPnt) | Calculate(brep, inputPnt) | Calculate(geoObject, inputPnt)`

Calculates tangent vector of Line2D

**Remarks:** Calculates tangent vector of Line2D

**Parameters:**
- `line` (Line2D) — Line2D on which to calculate tangent vector

**Returns:** `Vector3D` — tangent vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TangentCalculus/#NemAll_Python_Geometry.TangentCalculus.Calculate)

#### `CalculateNormal(brep, inputPnt)`

Calculates normal vector of BRep3D at given point

**Remarks:** Calculates normal vector of BRep3D at given point

**Parameters:**
- `brep` (BRep3D) — Brep body
- `inputPnt` (Point3D) — point on object for normal calculation

**Returns:** `Vector3D` — normal vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TangentCalculus/#NemAll_Python_Geometry.TangentCalculus.CalculateNormal)

## TransformCoord (class)

This class offers functions to transform a world point into the local coordinate system (PointLocal) defined by a geometry object or a vector of objects. Functions named PointGlobal will take a local point and project its coordinates into world coordinates.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TransformCoord/)

### Methods
#### `PointGlobal(path, localPoint, eps) | PointGlobal(geoObject, offset, eps) | PointGlobal(path, offset, eps) | PointGlobal(path, offset, eps) | PointGlobal(line, offset) | PointGlobal(line, localPoint) | PointGlobal(line, offset) | PointGlobal(polygon, offset) | PointGlobal(polyline, offset) | PointGlobal(polyline, offset) | PointGlobal(polygon, offset) | PointGlobal(arc, offset) | PointGlobal(arc, localPoint) | PointGlobal(arc, offset) | PointGlobal(arc, localPoint) | PointGlobal(clothoid, offset, eps) | PointGlobal(clothoid, localPoint, eps) | PointGlobal(spline, offset, eps) | PointGlobal(spline, localPoint, eps) | PointGlobal(spline, offset, eps) | PointGlobal(spline, localPoint, eps) | PointGlobal(spline, offset) | PointGlobal(geoObject, localPoint, eps)`

Transform the local coordinates to world coordinates.

**Remarks:** Transform the local coordinates to world coordinates.

**Parameters:**
- `path` (Path2D) — Path of 2D geometry objects
- `localPoint` (Point2D) — Local point with offset as x and distance as y
- `eps` (float) — Tolerance for clothoids and splines

**Returns:** `tuple` — Global point on the path,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TransformCoord/#NemAll_Python_Geometry.TransformCoord.PointGlobal)

#### `PointGlobalEx(clothoid, offset, eps)`

Transform the local coordinates to world coordinates. This method compute global point by offset related to axis of clothoid (clothoid with parallel=0). This method is faster and computing global point with higher precision. Use PointGlobalEx with PointLocalEx cooperation only. sa PointLocalEx, TransformClothoidLocalOffset

**Remarks:** Transform the local coordinates to world coordinates. This method compute global point by offset related to axis of clothoid (clothoid with parallel=0). This method is faster and computing global point with higher precision. Use PointGlobalEx with PointLocalEx cooperation only. sa PointLocalEx, TransformClothoidLocalOffset

**Parameters:**
- `clothoid` (Clothoid2D) — 2D Clothoid
- `offset` (float) — Distance from start point on axis curve
- `eps` (float) — Tolerance for clothoids and splines

**Returns:** `tuple` — Global point on the geometry by offset on axis curve,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TransformCoord/#NemAll_Python_Geometry.TransformCoord.PointGlobalEx)

#### `PointLocal(path, inputPnt, eps) | PointLocal(line, inputPnt) | PointLocal(line, inputPnt) | PointLocal(placement, inputPnt) | PointLocal(placement, inputPnt) | PointLocal(line, inputPnt) | PointLocal(polyhedron, inputPnt) | PointLocal(polyline, inputPnt) | PointLocal(polyline, inputPnt) | PointLocal(polyline, inputPnt) | PointLocal(polygon, inputPnt) | PointLocal(arc, inputPnt) | PointLocal(arc, inputPnt) | PointLocal(arc, inputPnt) | PointLocal(arc, inputPnt) | PointLocal(clothoid, inputPnt, eps) | PointLocal(clothoid, inputPnt, eps) | PointLocal(spline, inputPnt, eps) | PointLocal(spline, inputPnt, eps) | PointLocal(spline, inputPnt, eps) | PointLocal(spline, inputPnt, eps) | PointLocal(geoObject, inputPnt, eps)`

Transform the local coordinates to world coordinates.

**Remarks:** Transform the local coordinates to world coordinates.

**Parameters:**
- `path` (Path2D) — 2D Path
- `inputPnt` (Point3D) — Projection point
- `eps` (float) — Tolerance for clothoids and splines

**Returns:** `tuple` — Local point on the path,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TransformCoord/#NemAll_Python_Geometry.TransformCoord.PointLocal)

#### `PointLocalEx(clothoid, inputPnt, eps)`

Calculate the local coordinates on a 2D clothoid PointLocalEx compute local point related to axis of clothoid (clothoid with parallel=0). This method is faster and computing X coordinate with higher precision. Use PointGlobalEx with PointLocalEx cooperation only. PointGlobalEx, TransformClothoidLocalOffset

**Remarks:** Calculate the local coordinates on a 2D clothoid PointLocalEx compute local point related to axis of clothoid (clothoid with parallel=0). This method is faster and computing X coordinate with higher precision. Use PointGlobalEx with PointLocalEx cooperation only. PointGlobalEx, TransformClothoidLocalOffset

**Parameters:**
- `clothoid` (Clothoid2D) — 2D Clothoid
- `inputPnt` (Point2D) — Projection point as Point2D
- `eps` (float) — Tolerance

**Returns:** `tuple` — Axis local point on the 2D clothoid,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TransformCoord/#NemAll_Python_Geometry.TransformCoord.PointLocalEx)

#### `TransformClothoidLocalOffset(clothoid, offset, toAxis)`

Transform local offset between clothoid axis curve and parallel curve This function provides transformation between offset related to axis or parallel(real) curve. This function is useful when you have offset on parallel curve and want to get length of axis curve. If clothoid parameter Parallel is zero, then axis and parallel offsets are equal. Sample: calc real length of clothoid ndouble real_length = Service::Length(clot); double curve_length = TransformClothoidLocalOffset(clot, clot.Getlength(), false); real_length and curve_length are equal.

**Remarks:** Transform local offset between clothoid axis curve and parallel curve This function provides transformation between offset related to axis or parallel(real) curve. This function is useful when you have offset on parallel curve and want to get length of axis curve. If clothoid parameter Parallel is zero, then axis and parallel offsets are equal. Sample: calc real length of clothoid ndouble real_length = Service::Length(clot); double curve_length = TransformClothoidLocalOffset(clot, clot.Getlength(), false); real_length and curve_length are equal.

**Parameters:**
- `clothoid` (Clothoid2D) — 2D Clothoid
- `offset` (float) — Source offset
- `toAxis` (bool) — If true, then offset is offset on parallel curve and result will be offset on axis curve.

**Returns:** `float` — Offset on axis or parallel curve, dependent on toAxis.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/TransformCoord/#NemAll_Python_Geometry.TransformCoord.TransformClothoidLocalOffset)

## Vector2D (class)

Representation class for 2D Vector.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/)

### Constructors
- `Vector2D() | Vector2D(vec) | Vector2D(angle, length) | Vector2D(x, y) | Vector2D(startPoint, endPoint) | Vector2D(endPoint) | Vector2D(vec)` — Initialize

### Methods
#### `CrossProduct(vec)`

Cross(vector) product operator. Formula: Va = Va x Vb Va is this Vector

**Remarks:** Cross(vector) product operator. Formula: Va = Va x Vb Va is this Vector

**Parameters:**
- `vec` (Vector2D) — Vector(Vb).

**Returns:** `Vector3D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.CrossProduct)

#### `DotProduct(vec)`

Dot(sxcalar) product. Formula: S = Va . Vb = Va1 * Va2 + Vb1 * Vb2 Va is this Vector

**Remarks:** Dot(sxcalar) product. Formula: S = Va . Vb = Va1 * Va2 + Vb1 * Vb2 Va is this Vector

**Parameters:**
- `vec` (Vector2D) — Vector(Vb).

**Returns:** `float` — double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.DotProduct)

#### `GetAngle()`

Get vector angle.

**Remarks:** Get vector angle.

**Returns:** `Angle` — Angle in a range <0, 2*PI> (normalized 2pi).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.GetAngle)

#### `GetAngleSigned()`

Get vector angle. equivalent to zfno()

**Remarks:** Get vector angle. equivalent to zfno()

**Returns:** `Angle` — Angle in a range <-PI..0..PI>

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.GetAngleSigned)

#### `GetCoords()`

Get copy of X,Y coordinates.

**Remarks:** Get copy of X,Y coordinates.

**Returns:** `float` — tuple(X coordinate of vector,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.GetCoords)

#### `GetLength()`

Get vector length. Formula: Result(double) = ||Va|| Va is this vector

**Remarks:** Get vector length. Formula: Result(double) = ||Va|| Va is this vector

**Returns:** `float` — double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.GetLength)

#### `IsZero()`

Check the coords [0.0, 0.0] (binary comparison)

**Remarks:** Check the coords [0.0, 0.0] (binary comparison)

**Returns:** `bool` — Is zero? true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.IsZero)

#### `Normalize() | Normalize(length)`

Normalize vector. Formula: Vn(a1/||Va||, a2/||Va||) Va is this vector This method is checked and throwing a geometry exception when vector is zero.

**Remarks:** Normalize vector. Formula: Vn(a1/||Va||, a2/||Va||) Va is this vector This method is checked and throwing a geometry exception when vector is zero.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.Normalize)

#### `Orthogonal(counterClockwise=True)`

Compute orthogonal vector Method calculate orthogonal vector. Sample: vec = Vector2D(100., 0) vec.Orthogonal()

**Remarks:** Compute orthogonal vector Method calculate orthogonal vector. Sample: vec = Vector2D(100., 0) vec.Orthogonal()

**Parameters:**
- `counterClockwise` (bool) — Orientation of orthogonal vector

**Returns:** `Vector2D` — Orthogonal vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.Orthogonal)

#### `Reverse()`

Compute reversed vector Method calculate vector with reversed orientation.

**Remarks:** Compute reversed vector Method calculate vector with reversed orientation.

**Returns:** `Vector2D` — Reversed vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.Reverse)

#### `Set(vec) | Set(x, y) | Set(startPoint, endPoint)`

Initialize from vector 2D.

**Remarks:** Initialize from vector 2D.

**Parameters:**
- `vec` (Vector2D) — Vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.Set)

#### `Values()`

Get copy of X,Y coordinates as python list

**Remarks:** Get copy of X,Y coordinates as python list

**Returns:** `list[float]` — X coordinate of vector.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.Values)

#### `__add__(vec)`

Addition operator. Formula: Vc = Va + Vb Va is this Vector.

**Remarks:** Addition operator. Formula: Vc = Va + Vb Va is this Vector.

**Parameters:**
- `vec` (Vector2D) — Vector(Vb).

**Returns:** `Vector2D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__add__)

#### `__eq__(vec)`

Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `vec` (Vector2D) — Compared vector.

**Returns:** `bool` — True when points are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__eq__)

#### `__iadd__(vec)`

Addition assignment operator. Formula: Va = Va + Vb Va is this Vector.

**Remarks:** Addition assignment operator. Formula: Va = Va + Vb Va is this Vector.

**Parameters:**
- `vec` (Vector2D) — Vb Vector.

**Returns:** `Vector2D` — Reference to vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__iadd__)

#### `__idiv__(divider)`

Multiply the vector by a factor (scalar multiplication)

**Remarks:** Multiply the vector by a factor (scalar multiplication)

**Parameters:**
- `divider` (float) — scaling divider

**Returns:** `Vector2D` — New vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__idiv__)

#### `__imul__(matrix) | __imul__(factor)`

Matrix transformation. Formula: Vector(this) = Vector(this) * matrix

**Remarks:** Matrix transformation. Formula: Vector(this) = Vector(this) * matrix

**Parameters:**
- `matrix` (Matrix2D) — Transformation matrix.

**Returns:** `Vector2D` — Vector2D.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__imul__)

#### `__isub__(vec)`

Subtraction assignment operator. Formula: Va = Va - Vb Va is this Vector.

**Remarks:** Subtraction assignment operator. Formula: Va = Va - Vb Va is this Vector.

**Parameters:**
- `vec` (Vector2D) — Vb Vector.

**Returns:** `Vector2D` — Reference to vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__isub__)

#### `__mul__(vec) | __mul__(matrix) | __mul__(factor)`

Cross(vector) product operator. Formula: Vc = Va x Vb Va is this Vector

**Remarks:** Cross(vector) product operator. Formula: Vc = Va x Vb Va is this Vector

**Parameters:**
- `vec` (Vector2D) — Vector(Vb).

**Returns:** `Vector3D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__mul__)

#### `__ne__(vec)`

Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `vec` (Vector2D) — Compared vector.

**Returns:** `bool` — True when points are not equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__repr__)

#### `__sub__(vec)`

Subtraction operator. Formula: Vc = Va - Vb Va is this Vector

**Remarks:** Subtraction operator. Formula: Vc = Va - Vb Va is this Vector

**Parameters:**
- `vec` (Vector2D) — Vector(Vb).

**Returns:** `Vector2D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__sub__)

#### `__truediv__(divider)`

Multiply the vector by a factor (scalar multiplication)

**Remarks:** Multiply the vector by a factor (scalar multiplication)

**Parameters:**
- `divider` (float) — scaling divider

**Returns:** `Vector2D` — New vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2D/#NemAll_Python_Geometry.Vector2D.__truediv__)

### Properties
- `X` (float, get/set) — Get the X coordinate reference.
- `Y` (float, get/set) — Get the Y coordinate reference.

## Vector2DList (class)

List for Vector2D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/)

### Constructors
- `Vector2DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Vector2D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Vector2D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Vector2D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Vector2D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Vector2D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Vector2DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector2DList/#NemAll_Python_Geometry.Vector2DList.extend)

## Vector3D (class)

Representation class for 3D Vector.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/)

### Constructors
- `Vector3D() | Vector3D(vec) | Vector3D(vec) | Vector3D(x, y, z) | Vector3D(startPoint, endPoint) | Vector3D(endPoint)` — Initialize

### Methods
#### `CrossProduct(vec)`

Cross(vector) product operator. Formula: Va = Va x Vb Va is this Vector

**Remarks:** Cross(vector) product operator. Formula: Va = Va x Vb Va is this Vector

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.CrossProduct)

#### `DotProduct(vec)`

Dot(sxcalar) product. Formula: S = Va . Vb = Va1 * Va2 + Vb1 * Vb2 Va is this Vector

**Remarks:** Dot(sxcalar) product. Formula: S = Va . Vb = Va1 * Va2 + Vb1 * Vb2 Va is this Vector

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

**Returns:** `float` — double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.DotProduct)

#### `GetCoords()`

Get copy of X,Y,Z coordinates

**Remarks:** Get copy of X,Y,Z coordinates

**Returns:** `float` — tuple(X coordinate of vector,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.GetCoords)

#### `GetLength()`

Get vector length. Formula: Result(double) = ||Va|| Va is this vector

**Remarks:** Get vector length. Formula: Result(double) = ||Va|| Va is this vector

**Returns:** `float` — double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.GetLength)

#### `GetLengthSquare()`

Get vector length without square-root in calculation.

**Remarks:** Get vector length without square-root in calculation.

**Returns:** `float` — double.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.GetLengthSquare)

#### `IsZero()`

Check the coords [0.0, 0.0, 0.0] If the coords are zero, the return value is true. If the coords aren't zero, the return value is false.

**Remarks:** Check the coords [0.0, 0.0, 0.0] If the coords are zero, the return value is true. If the coords aren't zero, the return value is false.

**Returns:** `bool` — Is zero? true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.IsZero)

#### `Normal(vec)`

Does the same as Cross(vector) product but does not change the operands. Formula: Vc = Va x Vb Va is this Vector

**Remarks:** Does the same as Cross(vector) product but does not change the operands. Formula: Vc = Va x Vb Va is this Vector

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.Normal)

#### `Normalize() | Normalize(length)`

Normalize vector. Formula: Vn(a1/||Va||, a2/||Va||) Va is this vector This method is checked and throwing a geometry exception when vector is zero.

**Remarks:** Normalize vector. Formula: Vn(a1/||Va||, a2/||Va||) Va is this vector This method is checked and throwing a geometry exception when vector is zero.

**Returns:** `object` — Geometry error code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.Normalize)

#### `Project(vec)`

Projection operator. Formula: Vc = / . Va Va is this Vector

**Remarks:** Projection operator. Formula: Vc = / . Va Va is this Vector

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

**Returns:** `Vector3D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.Project)

#### `Reverse()`

Compute reversed vector Method calculate vector with reversed orientation.

**Remarks:** Compute reversed vector Method calculate vector with reversed orientation.

**Returns:** `Vector3D` — Reversed vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.Reverse)

#### `Set(vec) | Set(x, y, z) | Set(startPoint, endPoint)`

Initialize from vector 3D.

**Remarks:** Initialize from vector 3D.

**Parameters:**
- `vec` (Vector3D) — Vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.Set)

#### `Values()`

Get copy of X,Y,Z coordinates as python list

**Remarks:** Get copy of X,Y,Z coordinates as python list

**Returns:** `list[float]` — X coordinate of vector.,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.Values)

#### `__add__(vec)`

Addition operator. Formula: Vc = Va + Vb Va is this Vector.

**Remarks:** Addition operator. Formula: Vc = Va + Vb Va is this Vector.

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

**Returns:** `Vector3D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__add__)

#### `__eq__(vec)`

Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `vec` (Vector3D) — Compared vector.

**Returns:** `bool` — True when points are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__eq__)

#### `__iadd__(vec)`

Addition assignment operator. Formula: Va = Va + Vb Va is this Vector.

**Remarks:** Addition assignment operator. Formula: Va = Va + Vb Va is this Vector.

**Parameters:**
- `vec` (Vector3D) — Vb Vector.

**Returns:** `Vector3D` — Reference to vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__iadd__)

#### `__idiv__(divider)`

Multiply the vector by a factor (scalar multiplication)

**Remarks:** Multiply the vector by a factor (scalar multiplication)

**Parameters:**
- `divider` (float) — scaling divider

**Returns:** `Vector3D` — New vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__idiv__)

#### `__imul__(vec) | __imul__(factor) | __imul__(matrix) | __imul__(matrix)`

Cross(vector) product operator. Formula: Va = Va x Vb Va is this Vector

**Remarks:** Cross(vector) product operator. Formula: Va = Va x Vb Va is this Vector

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

**Returns:** `Vector3D` — Reference to the cross(vector) product of vectors.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__imul__)

#### `__isub__(vec)`

Subtraction assignment operator. Formula: Va = Va - Vb Va is this Vector.

**Remarks:** Subtraction assignment operator. Formula: Va = Va - Vb Va is this Vector.

**Parameters:**
- `vec` (Vector3D) — Vb Vector.

**Returns:** `Vector3D` — Reference to vector.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__isub__)

#### `__mul__(vec) | __mul__(factor) | __mul__(matrix) | __mul__(matrix)`

Cross(vector) product operator. Formula: Vc = Va x Vb Va is this Vector

**Remarks:** Cross(vector) product operator. Formula: Vc = Va x Vb Va is this Vector

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

**Returns:** `Vector3D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__mul__)

#### `__ne__(vec)`

Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Remarks:** Comparison of vectors without tolerance. Be careful, this method work without tolerance!

**Parameters:**
- `vec` (Vector3D) — Compared vector.

**Returns:** `bool` — True when points are not equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__repr__)

#### `__sub__(vec)`

Subtraction operator. Formula: Vc = Va - Vb Va is this Vector

**Remarks:** Subtraction operator. Formula: Vc = Va - Vb Va is this Vector

**Parameters:**
- `vec` (Vector3D) — Vector(Vb).

**Returns:** `Vector3D` — new Vector(Vc).

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__sub__)

#### `__truediv__(divider)`

Multiply the vector by a factor (scalar multiplication)

**Remarks:** Multiply the vector by a factor (scalar multiplication)

**Parameters:**
- `divider` (float) — scaling divider

**Returns:** `Vector3D` — New vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3D/#NemAll_Python_Geometry.Vector3D.__truediv__)

### Properties
- `X` (float, get/set) — Get the X coordinate reference.
- `Y` (float, get/set) — Get the Y coordinate reference.
- `Z` (float, get/set) — Get the Z coordinate reference.

## Vector3DList (class)

List for Vector3D objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/)

### Constructors
- `Vector3DList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (Vector3D) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (Vector3D) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `Vector3D` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (Vector3D) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (Vector3D) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (Vector3DList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Vector3DList/#NemAll_Python_Geometry.Vector3DList.extend)

## eApproximationSettingsType (enum)

Type of Approximation setting

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eApproximationSettingsType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eApproximationSettingsType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eApproximationSettingsType/#NemAll_Python_Geometry.eApproximationSettingsType.__getitem__)

### Values
- `ASET_BREP_TESSELATION` = `3`
- `ASET_MAX_DISTANCE` = `2`
- `ASET_MAX_LENGTH` = `1`
- `ASET_SEGMENTATION` = `0`

## eBoolOpResult (enum)

bool operation result

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eBoolOpResult/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eBoolOpResult` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eBoolOpResult/#NemAll_Python_Geometry.eBoolOpResult.__getitem__)

### Values
- `eClip` = `2`
- `eInside` = `0`
- `eOutside` = `1`

## eBoxPoint (enum)

Dedicated location of the box 4----8----3 | | 9 5 7 | | 1----6----2

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eBoxPoint/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eBoxPoint` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eBoxPoint/#NemAll_Python_Geometry.eBoxPoint.__getitem__)

### Values
- `eCenter` = `5`
- `eLeftBottom` = `1`
- `eLeftTop` = `4`
- `eMiddleBottom` = `6`
- `eMiddleLeft` = `9`
- `eMiddleRight` = `7`
- `eMiddleTop` = `8`
- `eRightBottom` = `2`
- `eRightTop` = `3`

## eClothoidType (enum)

Clothoid type identification

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eClothoidType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eClothoidType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eClothoidType/#NemAll_Python_Geometry.eClothoidType.__getitem__)

### Values
- `eClothoid` = `1`
- `eParabolaGeneral` = `3`
- `eParabolaQuadratic` = `2`

## eComparisionResult (enum)

Used for better identification of comparision functionality's result

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eComparisionResult/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eComparisionResult` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eComparisionResult/#NemAll_Python_Geometry.eComparisionResult.__getitem__)

### Values
- `eAbove` = `6`
- `eAntiParallel` = `1`
- `eBelow` = `7`
- `eCrossing` = `12`
- `eEqualToEndPoint` = `9`
- `eEqualToStartPoint` = `8`
- `eInside` = `10`
- `eLeft` = `4`
- `eNotParallel` = `2`
- `eOnElement` = `3`
- `eOutside` = `11`
- `eParallel` = `0`
- `eRight` = `5`
- `eUnknown` = `13`

## eCreatePatchResult (enum)

Result of the patch operation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eCreatePatchResult/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eCreatePatchResult` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eCreatePatchResult/#NemAll_Python_Geometry.eCreatePatchResult.__getitem__)

### Values
- `eCurvesAreSelfIntersecting` = `3`
- `eCurvesNotClosed` = `2`
- `eError` = `1`
- `eOK` = `0`

## eFilletErrorCode (enum)

Enumeration of error codes for Fillet

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eFilletErrorCode/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eFilletErrorCode` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eFilletErrorCode/#NemAll_Python_Geometry.eFilletErrorCode.__getitem__)

### Values
- `eERROR_LINES_ARENOT_COPLANAR` = `1`
- `eERROR_LINES_ARE_COLLINEAR` = `2`
- `eERROR_LINES_ARE_NOT_PARALLEL` = `6`
- `eERROR_LINES_ARE_PARALLEL` = `3`
- `eERROR_NO_FILLET_CREATED` = `5`
- `eERROR_ZEROO_LINE_LENGTH` = `4`
- `eNO_ERROR` = `0`

## eFilletType (enum)

Enumeration for the type of Fillet

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eFilletType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eFilletType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eFilletType/#NemAll_Python_Geometry.eFilletType.__getitem__)

### Values
- `FT_CC_CONCETRIC` = `33`
- `FT_CC_NO_INTERSECTION` = `32`
- `FT_CC_ONE_INTERSECTION` = `31`
- `FT_CC_TWO_INTERSECTION` = `30`
- `FT_KK_KURVE_ELEMENTS` = `40`
- `FT_KK_PARALLEL_KURVE_ELEMENTS` = `41`
- `FT_LC_NO_INTERSECTION` = `22`
- `FT_LC_ONE_INTERSECTION` = `21`
- `FT_LC_TWO_INTERSECTION` = `20`
- `FT_LL_INTERSECTION_AT_THE_END` = `11`
- `FT_LL_INTERSECTION_ON_LINE` = `10`
- `FT_LL_INTERSECTION_OUT_OF_LINES` = `12`
- `FT_LL_PARALLEL_LINES` = `13`
- `FT_UNKNOWN` = `0`

## eGeometryErrorCode (enum)

Geometry error codes

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eGeometryErrorCode/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eGeometryErrorCode` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eGeometryErrorCode/#NemAll_Python_Geometry.eGeometryErrorCode.__getitem__)

### Values
- `eAllocError` = `1234`
- `eError` = `1`
- `eInvalid3DLine` = `512`
- `eOK` = `0`
- `eOutOfRange` = `256`
- `eStructuralError` = `4`
- `eWarpedPolygonalFace` = `8`
- `eWrongShape` = `2`

## eHiddenCalculationResult (enum)

Result of hidden calculation for line.

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eHiddenCalculationResult/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eHiddenCalculationResult` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eHiddenCalculationResult/#NemAll_Python_Geometry.eHiddenCalculationResult.__getitem__)

### Values
- `eHidden` = `1`
- `eVisible` = `0`

## eLinePointIdentification (enum)

Start and end point identification

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eLinePointIdentification/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eLinePointIdentification` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eLinePointIdentification/#NemAll_Python_Geometry.eLinePointIdentification.__getitem__)

### Values
- `END_POINT` = `1`
- `START_POINT` = `0`

## ePlanarSurfaceError (enum)

Planar surface creation error

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePlanarSurfaceError/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ePlanarSurfaceError` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePlanarSurfaceError/#NemAll_Python_Geometry.ePlanarSurfaceError.__getitem__)

### Values
- `eCollinear` = `0`
- `eError` = `3`
- `eNotContinuous` = `2`
- `eNotPlanar` = `1`
- `eOK` = `4`

## ePolygonHealingSettings (enum)

Type of Healing settings for polygon

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePolygonHealingSettings/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ePolygonHealingSettings` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePolygonHealingSettings/#NemAll_Python_Geometry.ePolygonHealingSettings.__getitem__)

### Values
- `PHSET_NOMODIFY` = `0`
- `PHSET_NORMALIZE` = `1`

## ePolygonNormalizeType (enum)

type of Polygon2D normalization

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePolygonNormalizeType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ePolygonNormalizeType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePolygonNormalizeType/#NemAll_Python_Geometry.ePolygonNormalizeType.__getitem__)

### Values
- `DEFAULT_NORM_TYPE` = `0`
- `HATCHING_MEASUR_NORM_TYPE` = `2`
- `HATCHING_NORM_TYPE` = `3`
- `SIDEFACE_NORM_TYPE` = `1`
- `STARTPOINT_NORM_TYPE` = `4`

## ePolyhedronHealingSettings (enum)

Type of Healing settings for polyhedron

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePolyhedronHealingSettings/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ePolyhedronHealingSettings` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/ePolyhedronHealingSettings/#NemAll_Python_Geometry.ePolyhedronHealingSettings.__getitem__)

### Values
- `HSET_NOMODIFY` = `2`
- `HSET_TILT` = `1`
- `HSET_TRIANGULATE` = `0`
- `HSET_UNKNOWN` = `3`

## eProjectionMatrixType (enum)

Type of projection in Matrix3D

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eProjectionMatrixType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eProjectionMatrixType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eProjectionMatrixType/#NemAll_Python_Geometry.eProjectionMatrixType.__getitem__)

### Values
- `BOTTOM_2D` = `5`
- `FREE_3D` = `6`
- `FRONT_2D` = `2`
- `LEFT_2D` = `0`
- `REAR_2D` = `3`
- `RIGHT_2D` = `1`
- `TOP_2D` = `4`

## eServiceResult (enum)

service result

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eServiceResult/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eServiceResult` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eServiceResult/#NemAll_Python_Geometry.eServiceResult.__getitem__)

### Values
- `FAILED_TO_CONVERGE` = `17`
- `FAILED_TO_INVERT_MATRIX` = `16`
- `INVALID_CONE` = `13`
- `INVALID_CUBOID` = `11`
- `INVALID_CYLINDER` = `12`
- `INVALID_ELLIPSOID` = `14`
- `INVALID_GEOOBJECT` = `9`
- `INVALID_LINE` = `3`
- `INVALID_POINT` = `10`
- `INVALID_POINTCLOUD` = `18`
- `INVALID_POLYGON` = `5`
- `INVALID_POLYHEDRON` = `6`
- `INVALID_POLYLINE` = `4`
- `INVALID_SURFACE` = `15`
- `IS_ABOVE` = `19`
- `IS_BELOW` = `20`
- `IS_IN` = `21`
- `IS_UNKNOWN` = `22`
- `NEGATIVE_ORIENTATION` = `1`
- `NO_ERR` = `2`
- `PARALLEL_LINES` = `8`
- `POSITIVE_ORIENTATION` = `0`
- `ZERO_ANGLE` = `7`

## eSplitResult (enum)

Used for split operation's result

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eSplitResult/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eSplitResult` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eSplitResult/#NemAll_Python_Geometry.eSplitResult.__getitem__)

### Values
- `eSplitInvalidArgs` = `3`
- `eSplitInvalidGeometry` = `2`
- `eSplitNone` = `0`
- `eSplitOk` = `1`

## eSurfaceTrimParam (enum)

Trimming curve surface param for naturally trimmed surfaces

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eSurfaceTrimParam/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eSurfaceTrimParam` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eSurfaceTrimParam/#NemAll_Python_Geometry.eSurfaceTrimParam.__getitem__)

### Values
- `eTrimUMaxValue` = `3`
- `eTrimUMaxValueReverse` = `4`
- `eTrimUMinValue` = `1`
- `eTrimUMinValueReverse` = `2`
- `eTrimUndefined` = `0`
- `eTrimVMaxValue` = `7`
- `eTrimVMaxValueReverse` = `8`
- `eTrimVMinValue` = `5`
- `eTrimVMinValueReverse` = `6`

## eValidationStatusPolygon3D (enum)

Validation status of 3D Polygon

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eValidationStatusPolygon3D/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eValidationStatusPolygon3D` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/eValidationStatusPolygon3D/#NemAll_Python_Geometry.eValidationStatusPolygon3D.__getitem__)

### Values
- `VS_COLINEAR` = `0`
- `VS_NOT_COPLANAR` = `1`

