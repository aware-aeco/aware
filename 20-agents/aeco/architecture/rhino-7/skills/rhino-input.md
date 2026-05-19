---
name: rhino-rhino-input
description: This skill encodes the rhino 7.0 surface of the Rhino.Input namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RhinoGet, StringParser, StringParserSettings, GetBoxMode, GetResult, RhinoGet.BitmapFileTypes.
---

# Rhino.Input

Auto-generated from vendor docs for rhino 7.0. 6 types in this namespace.

## GetBoxMode (enum)

Enumerates all Box getter modes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_GetBoxMode.htm)

### Values
- `All` = `0` — All modes are allowed.
- `Corner` = `1` — The base rectangle is created by picking the two corner points.
- `ThreePoint` = `2` — The base rectangle is created by picking three points.
- `Vertical` = `3` — The base vertical rectangle is created by picking three points.
- `Center` = `4` — The base rectangle is created by picking a center point and a corner point.

## GetResult (enum)

Possible results from GetObject.Get(), GetPoint.Get(), etc...

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_GetResult.htm)

### Values
- `NoResult` = `0`
- `Cancel` = `1` — User wants to cancel current command.
- `Nothing` = `2` — User pressed enter - typically used to accept defaults.
- `Option` = `3` — User specified an option - call Option() to get option index.
- `Number` = `4` — User entered a real number - call Number() to get value.
- `Color` = `5` — User entered a color - call Color() to get value.
- `Undo` = `6` — User pressed undo.
- `Miss` = `7` — User clicked and missed.
- `Point` = `8` — User picked 3d point - call Point() to get 3d point.
- `Point2d` = `9` — User picked 2d window point in CRhinoGetPoint::Get2dPoint() call ON_2dPoint() to get the point and View() to get the view.
- `Line2d` = `10` — User picked a 2d line in CRhinoGetPoint::Get2dLine() call Line2d() to get the line and View() to get the view.
- `Rectangle2d` = `11` — User picked a 2d rectangle in CRhinoGetPoint::Get2dRectangle() call Rectangle2d() to get the rectangle and View() to get the view.
- `Object` = `12` — User selected an object - call Object() to get object.
- `String` = `13` — User typed a string - call String() to get the string.
- `CustomMessage` = `14` — A custom message was posted to the RhinoGet
- `Timeout` = `15` — The getter waited for the amount of time specified in RhinoGet::SetWaitDuration() and then gave up.
- `Circle` = `16` — call CRhinoGetCircle::GetCircle() to get the circle.
- `Plane` = `17` — call CRhinoGetPlane::GetPlane() to get the plane.
- `Cylinder` = `18` — call CRhinoGetCylinder::GetCylinder() to get the cylinder.
- `Sphere` = `19` — call CRhinoGetSphere::GetSphere() to get the sphere.
- `Angle` = `20` — call CRhinoGetAngle::Angle() to get the angle in radians (CRhinoGetAngle() returns this for typed number, too).
- `Distance` = `21` — call CRhinoGetDistance::Distance() to get the distance value.
- `Direction` = `22` — call CRhinoGetDirection::Direction() to get the direction vector.
- `Frame` = `23` — call CRhinoGetFrame::Frame() to get the frame that was picked.
- `User1` = `4294967295`
- `User2` = `4294967294`
- `User3` = `4294967293`
- `User4` = `4294967292`
- `User5` = `4294967291`
- `ExitRhino` = `268435455` — Stop now, do not cleanup, just return ASAP.

## RhinoGet (class)

Base class for GetObject, GetPoint, GetSphere, etc. You will never directly create a RhinoGet but you will use its member functions after calling GetObject::GetObjects(), GetPoint::GetPoint(), and so on. Provides tools to set command prompt, set command options, and specify if the "get" can optionally accept numbers, nothing (pressing enter), and undo.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_RhinoGet.htm)

### Methods
#### `public static Result Get2dRectangle(bool solidPen, out Rectangle rectangle, out RhinoView rectView)`

Gets a rectangle in view window coordinates.

**Parameters:**
- `solidPen` (System.Boolean) — If true, a solid pen is used for drawing while the user selects a rectangle. If false, a dotted pen is used for drawing while the user selects a rectangle.
- `rectangle` (System.Drawing.Rectangle) — user selected rectangle in window coordinates.
- `rectView` (Rhino.Display.RhinoView) — view that the user selected the window in.

**Returns:** `Result` — Success or Cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_Get2dRectangle.htm)

#### `public static Result GetAngle(string commandPrompt, Point3d basePoint, Point3d referencePoint, double defaultAngleRadians, out double angleRadians)`

Allows user to interactively pick an angle

**Parameters:**
- `commandPrompt` (System.String) — if null, a default prompt will be displayed
- `basePoint` (Rhino.Geometry.Point3d) — [Missing <param name="basePoint"/> documentation for "M:Rhino.Input.RhinoGet.GetAngle(System.String,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Double,System.Double@)"]
- `referencePoint` (Rhino.Geometry.Point3d) — [Missing <param name="referencePoint"/> documentation for "M:Rhino.Input.RhinoGet.GetAngle(System.String,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Double,System.Double@)"]
- `defaultAngleRadians` (System.Double) — [Missing <param name="defaultAngleRadians"/> documentation for "M:Rhino.Input.RhinoGet.GetAngle(System.String,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Double,System.Double@)"]
- `angleRadians` (System.Double) — [Missing <param name="angleRadians"/> documentation for "M:Rhino.Input.RhinoGet.GetAngle(System.String,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Double,System.Double@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetAngle(System.String,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Double,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetAngle.htm)

#### `public static Result GetArc(out Arc arc)`



**Parameters:**
- `arc` (Rhino.Geometry.Arc) — [Missing <param name="arc"/> documentation for "M:Rhino.Input.RhinoGet.GetArc(Rhino.Geometry.Arc@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetArc(Rhino.Geometry.Arc@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetArc.htm)

#### `public static Result GetBool(string prompt, bool acceptNothing, string offPrompt, string onPrompt, ref bool boolValue)`

Easy to use Boolean getter.

**Parameters:**
- `prompt` (System.String) — Command prompt.
- `acceptNothing` (System.Boolean) — If true, the user can press enter.
- `offPrompt` (System.String) — The 'false/off' message.
- `onPrompt` (System.String) — The 'true/on' message.
- `boolValue` (System.Boolean) — Default Boolean value set to this and returned here.

**Returns:** `Result` — The getter result based on user choice. Commands.Result.Success - got value.Commands.Result.Nothing - user pressed enter.Commands.Result.Cancel - user canceled value getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetBool.htm)

#### `public static Result GetBox(out Box box)`

Asks the user to select a Box in the viewport.

**Parameters:**
- `box` (Rhino.Geometry.Box) — If the result is Success, this parameter will be filled out.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetBox.htm)

#### `public static Result GetBox(out Box box, GetBoxMode mode, Point3d basePoint, string prompt1, string prompt2, string prompt3)`

Asks the user to select a Box in the viewport.

**Parameters:**
- `box` (Rhino.Geometry.Box) — If the result is Success, this parameter will be filled out.
- `mode` (Rhino.Input.GetBoxMode) — A particular "get box" mode, or All.
- `basePoint` (Rhino.Geometry.Point3d) — Optional base point. Supply Point3d.Unset if you don't want to use this.
- `prompt1` (System.String) — Optional first prompt. Supply null to use the default prompt.
- `prompt2` (System.String) — Optional second prompt. Supply null to use the default prompt.
- `prompt3` (System.String) — Optional third prompt. Supply null to use the default prompt.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetBox_1.htm)

#### `public static Result GetBoxWithCounts(int xMin, ref int xCount, int yMin, ref int yCount, int zMin, ref int zCount, out Point3d[] corners)`

Gets a 3d box with prompts for counts in X, Y and Z directions.

**Parameters:**
- `xMin` (System.Int32) — Minimum value allowed for count in the x direction.
- `xCount` (System.Int32) — Count in the x direction.
- `yMin` (System.Int32) — Minimum value allowed for count in the y direction.
- `yCount` (System.Int32) — Count in the y direction.
- `zMin` (System.Int32) — Minimum value allowed for count in the z direction.
- `zCount` (System.Int32) — Count in the z direction.
- `corners` (Rhino.Geometry.Point3d[]) — corners of the bottom rectangle in counter-clockwise order, followed by top rectangle.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetBoxWithCounts.htm)

#### `public static Result GetCircle(out Circle circle)`



**Parameters:**
- `circle` (Rhino.Geometry.Circle) — [Missing <param name="circle"/> documentation for "M:Rhino.Input.RhinoGet.GetCircle(Rhino.Geometry.Circle@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetCircle(Rhino.Geometry.Circle@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetCircle.htm)

#### `public static Result GetColor(string prompt, bool acceptNothing, ref Color color)`

Easy to use color getter.

**Remarks:** If you need options or more advanced user interface, then use GetColor class.

**Parameters:**
- `prompt` (System.String) — Command prompt.
- `acceptNothing` (System.Boolean) — If true, the user can press enter.
- `color` (System.Drawing.Color) — Color value returned here. also used as default color.

**Returns:** `Result` — Commands.Result.Success - got color.Commands.Result.Nothing - user pressed enter.Commands.Result.Cancel - user cancel color getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetColor.htm)

#### `public static string GetFileName(GetFileNameMode mode, string defaultName, string title, Object parent)`



**Parameters:**
- `mode` (Rhino.Input.Custom.GetFileNameMode) — [Missing <param name="mode"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object)"]
- `defaultName` (System.String) — [Missing <param name="defaultName"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object)"]
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object)"]
- `parent` (System.Object) — [Missing <param name="parent"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetFileName.htm)

#### `public static string GetFileName(GetFileNameMode mode, string defaultName, string title, Object parent, RhinoGet.BitmapFileTypes fileTypes)`



**Parameters:**
- `mode` (Rhino.Input.Custom.GetFileNameMode) — [Missing <param name="mode"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object,Rhino.Input.RhinoGet.BitmapFileTypes)"]
- `defaultName` (System.String) — [Missing <param name="defaultName"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object,Rhino.Input.RhinoGet.BitmapFileTypes)"]
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object,Rhino.Input.RhinoGet.BitmapFileTypes)"]
- `parent` (System.Object) — [Missing <param name="parent"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object,Rhino.Input.RhinoGet.BitmapFileTypes)"]
- `fileTypes` (Rhino.Input.RhinoGet.BitmapFileTypes) — [Missing <param name="fileTypes"/> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object,Rhino.Input.RhinoGet.BitmapFileTypes)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetFileName(Rhino.Input.Custom.GetFileNameMode,System.String,System.String,System.Object,Rhino.Input.RhinoGet.BitmapFileTypes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetFileName_1.htm)

#### `public static string GetFileNameScripted(GetFileNameMode mode, string defaultName)`



**Parameters:**
- `mode` (Rhino.Input.Custom.GetFileNameMode) — [Missing <param name="mode"/> documentation for "M:Rhino.Input.RhinoGet.GetFileNameScripted(Rhino.Input.Custom.GetFileNameMode,System.String)"]
- `defaultName` (System.String) — [Missing <param name="defaultName"/> documentation for "M:Rhino.Input.RhinoGet.GetFileNameScripted(Rhino.Input.Custom.GetFileNameMode,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetFileNameScripted(Rhino.Input.Custom.GetFileNameMode,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetFileNameScripted.htm)

#### `public static Result GetGrip(out GripObject grip, string prompt)`



**Parameters:**
- `grip` (Rhino.DocObjects.GripObject) — [Missing <param name="grip"/> documentation for "M:Rhino.Input.RhinoGet.GetGrip(Rhino.DocObjects.GripObject@,System.String)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Input.RhinoGet.GetGrip(Rhino.DocObjects.GripObject@,System.String)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetGrip(Rhino.DocObjects.GripObject@,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetGrip.htm)

#### `public static Result GetGrips(out GripObject[] grips, string prompt)`



**Parameters:**
- `grips` (Rhino.DocObjects.GripObject[]) — [Missing <param name="grips"/> documentation for "M:Rhino.Input.RhinoGet.GetGrips(Rhino.DocObjects.GripObject[]@,System.String)"]
- `prompt` (System.String) — [Missing <param name="prompt"/> documentation for "M:Rhino.Input.RhinoGet.GetGrips(Rhino.DocObjects.GripObject[]@,System.String)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetGrips(Rhino.DocObjects.GripObject[]@,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetGrips.htm)

#### `public static Result GetHelix(out NurbsCurve helix)`



**Parameters:**
- `helix` (Rhino.Geometry.NurbsCurve) — [Missing <param name="helix"/> documentation for "M:Rhino.Input.RhinoGet.GetHelix(Rhino.Geometry.NurbsCurve@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetHelix(Rhino.Geometry.NurbsCurve@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetHelix.htm)

#### `public static Result GetInteger(string prompt, bool acceptNothing, ref int outputNumber)`

Easy to use number getter.

**Parameters:**
- `prompt` (System.String) — command prompt.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `outputNumber` (System.Int32) — default number is set to this value and number value returned here.

**Returns:** `Result` — Commands.Result.Success - got number Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel number getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetInteger.htm)

#### `public static Result GetInteger(string prompt, bool acceptNothing, ref int outputNumber, int lowerLimit, int upperLimit)`

Easy to use number getter.

**Parameters:**
- `prompt` (System.String) — The command prompt.
- `acceptNothing` (System.Boolean) — If true, the user can press enter.
- `outputNumber` (System.Int32) — default number is set to this value and number value returned here.
- `lowerLimit` (System.Int32) — The minimum allowed value.
- `upperLimit` (System.Int32) — The maximum allowed value.

**Returns:** `Result` — Commands.Result.Success - got number Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel number getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetInteger_1.htm)

#### `public static Result GetLine(out Line line)`



**Parameters:**
- `line` (Rhino.Geometry.Line) — [Missing <param name="line"/> documentation for "M:Rhino.Input.RhinoGet.GetLine(Rhino.Geometry.Line@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetLine(Rhino.Geometry.Line@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetLine.htm)

#### `public static Result GetLinearDimension(out LinearDimension dimension)`



**Parameters:**
- `dimension` (Rhino.Geometry.LinearDimension) — [Missing <param name="dimension"/> documentation for "M:Rhino.Input.RhinoGet.GetLinearDimension(Rhino.Geometry.LinearDimension@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetLinearDimension(Rhino.Geometry.LinearDimension@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetLinearDimension.htm)

#### `public static Result GetMeshParameters(RhinoDoc doc, ref MeshingParameters parameters, ref int uiStyle)`

Asks the user to specify meshing parameters.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The active document
- `parameters` (Rhino.Geometry.MeshingParameters) — The initial meshing parameters. If successful, the updated meshing parameters are returned here.
- `uiStyle` (System.Int32) — The user interface style, where: 0 = simple dialog, 1 = details dialog, 2 = script or batch mode.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetMeshParameters.htm)

#### `public static Result GetMultipleObjects(string prompt, bool acceptNothing, GetObjectGeometryFilter filter, out ObjRef[] rhObjects)`

Easy to use object getter for getting multiple objects.

**Remarks:** If you need options or more advanced user interface, then use GetObject class.

**Parameters:**
- `prompt` (System.String) — command prompt.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `filter` (Rhino.Input.Custom.GetObjectGeometryFilter) — geometry filter to use when getting objects.
- `rhObjects` (Rhino.DocObjects.ObjRef[]) — result of the get. may be null.

**Returns:** `Result` — Commands.Result.Success - got object Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel object getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetMultipleObjects_1.htm)

#### `public static Result GetMultipleObjects(string prompt, bool acceptNothing, ObjectType filter, out ObjRef[] rhObjects)`

Easy to use object getter for getting multiple objects.

**Remarks:** If you need options or more advanced user interface, then use GetObject class.

**Parameters:**
- `prompt` (System.String) — command prompt.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `filter` (Rhino.DocObjects.ObjectType) — geometry filter to use when getting objects.
- `rhObjects` (Rhino.DocObjects.ObjRef[]) — result of the get. may be null.

**Returns:** `Result` — Commands.Result.Success - got object Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel object getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetMultipleObjects.htm)

#### `public static Result GetNumber(string prompt, bool acceptNothing, ref double outputNumber)`

Easy to use number getter.

**Parameters:**
- `prompt` (System.String) — command prompt.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `outputNumber` (System.Double) — default number is set to this value and number value returned here.

**Returns:** `Result` — Commands.Result.Success - got number Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel number getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetNumber.htm)

#### `public static Result GetNumber(string prompt, bool acceptNothing, ref double outputNumber, double lowerLimit, double upperLimit)`

Easy to use number getter.

**Parameters:**
- `prompt` (System.String) — The command prompt.
- `acceptNothing` (System.Boolean) — If true, the user can press Enter.
- `outputNumber` (System.Double) — Default number is set to this value and the return number value is assigned to this variable during the call.
- `lowerLimit` (System.Double) — The minimum allowed value.
- `upperLimit` (System.Double) — The maximum allowed value.

**Returns:** `Result` — Commands.Result.Success - got number.Commands.Result.Nothing - user pressed enter.Commands.Result.Cancel - user cancel number getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetNumber_1.htm)

#### `public static Result GetOneObject(string prompt, bool acceptNothing, GetObjectGeometryFilter filter, out ObjRef objref)`

Easy to use object getter.

**Remarks:** If you need options or more advanced user interface, then use GetObject class.

**Parameters:**
- `prompt` (System.String) — command prompt.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `filter` (Rhino.Input.Custom.GetObjectGeometryFilter) — geometry filter to use when getting objects.
- `objref` (Rhino.DocObjects.ObjRef) — result of the get. may be null.

**Returns:** `Result` — Commands.Result.Success - got object Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel object getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetOneObject_1.htm)

#### `public static Result GetOneObject(string prompt, bool acceptNothing, ObjectType filter, out ObjRef rhObject)`

Easy to use object getter.

**Remarks:** If you need options or more advanced user interface, then use GetObject class.

**Parameters:**
- `prompt` (System.String) — command prompt.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `filter` (Rhino.DocObjects.ObjectType) — geometry filter to use when getting objects.
- `rhObject` (Rhino.DocObjects.ObjRef) — result of the get. may be null.

**Returns:** `Result` — Commands.Result.Success - got object Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel object getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetOneObject.htm)

#### `public static Result GetPlane(out Plane plane)`

Gets an oriented infinite plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The plane result.

**Returns:** `Result` — Commands.Result.Success - got plane.Commands.Result.Nothing - user pressed enter.Commands.Result.Cancel - user cancel number getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPlane.htm)

#### `public static Result GetPoint(string prompt, bool acceptNothing, out Point3d point)`

Gets a point coordinate from the document.

**Remarks:** If you need options or more advanced user interface, then use GetPoint class.

**Parameters:**
- `prompt` (System.String) — Prompt to display in command line during the operation.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `point` (Rhino.Geometry.Point3d) — point value returned here.

**Returns:** `Result` — Commands.Result.Success - got point Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel point getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPoint.htm)

#### `[ObsoleteAttribute("Use method that requires document")] public static Result GetPointOnMesh(Guid meshObjectId, string prompt, bool acceptNothing, out Point3d point)`

Gets a point constrained to an existing mesh in the document.

**Parameters:**
- `meshObjectId` (System.Guid) — An ID of a mesh in the document.
- `prompt` (System.String) — Text prompt.
- `acceptNothing` (System.Boolean) — true if nothing else should be accepted.
- `point` (Rhino.Geometry.Point3d) — A point value will be assigned to this out parameter during this call.

**Returns:** `Result` — A command result based on user choice.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPointOnMesh_3.htm)

#### `[ObsoleteAttribute("Use version that takes a document.")] public static Result GetPointOnMesh(MeshObject meshObject, string prompt, bool acceptNothing, out Point3d point)`

Gets a point constrained to an existing mesh in the document.

**Parameters:**
- `meshObject` (Rhino.DocObjects.MeshObject) — An mesh object in the document.
- `prompt` (System.String) — Text prompt.
- `acceptNothing` (System.Boolean) — true if nothing else should be accepted.
- `point` (Rhino.Geometry.Point3d) — A point value will be assigned to this out parameter during this call.

**Returns:** `Result` — The command result based on user choice.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPointOnMesh.htm)

#### `public static Result GetPointOnMesh(RhinoDoc doc, Guid meshObjectId, string prompt, bool acceptNothing, out Point3d point)`

Gets a point constrained to an existing mesh in the document.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Document containing mesh object.
- `meshObjectId` (System.Guid) — An ID of a mesh in the document.
- `prompt` (System.String) — Text prompt.
- `acceptNothing` (System.Boolean) — true if nothing else should be accepted.
- `point` (Rhino.Geometry.Point3d) — A point value will be assigned to this out parameter during this call.

**Returns:** `Result` — A command result based on user choice.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPointOnMesh_2.htm)

#### `public static Result GetPointOnMesh(RhinoDoc doc, MeshObject meshObject, string prompt, bool acceptNothing, out Point3d point)`

Gets a point constrained to an existing mesh in the document.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document
- `meshObject` (Rhino.DocObjects.MeshObject) — An mesh object in the document.
- `prompt` (System.String) — Text prompt.
- `acceptNothing` (System.Boolean) — true if nothing else should be accepted.
- `point` (Rhino.Geometry.Point3d) — A point value will be assigned to this out parameter during this call.

**Returns:** `Result` — The command result based on user choice.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPointOnMesh_1.htm)

#### `public static Result GetPolygon(ref int numberSides, ref bool inscribed, out Polyline polyline)`



**Parameters:**
- `numberSides` (System.Int32) — [Missing <param name="numberSides"/> documentation for "M:Rhino.Input.RhinoGet.GetPolygon(System.Int32@,System.Boolean@,Rhino.Geometry.Polyline@)"]
- `inscribed` (System.Boolean) — [Missing <param name="inscribed"/> documentation for "M:Rhino.Input.RhinoGet.GetPolygon(System.Int32@,System.Boolean@,Rhino.Geometry.Polyline@)"]
- `polyline` (Rhino.Geometry.Polyline) — [Missing <param name="polyline"/> documentation for "M:Rhino.Input.RhinoGet.GetPolygon(System.Int32@,System.Boolean@,Rhino.Geometry.Polyline@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetPolygon(System.Int32@,System.Boolean@,Rhino.Geometry.Polyline@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPolygon.htm)

#### `public static Result GetPolyline(out Polyline polyline)`



**Parameters:**
- `polyline` (Rhino.Geometry.Polyline) — [Missing <param name="polyline"/> documentation for "M:Rhino.Input.RhinoGet.GetPolyline(Rhino.Geometry.Polyline@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetPolyline(Rhino.Geometry.Polyline@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPolyline.htm)

#### `public static Result GetPrintWindow(ref ViewCaptureSettings settings)`



**Parameters:**
- `settings` (Rhino.Display.ViewCaptureSettings) — [Missing <param name="settings"/> documentation for "M:Rhino.Input.RhinoGet.GetPrintWindow(Rhino.Display.ViewCaptureSettings@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetPrintWindow(Rhino.Display.ViewCaptureSettings@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetPrintWindow.htm)

#### `public static Result GetRectangle(GetBoxMode mode, Point3d firstPoint, IEnumerable<string> prompts, out Point3d[] corners)`

Gets a 3d rectangle made up of four points.

**Parameters:**
- `mode` (Rhino.Input.GetBoxMode) — A get box mode.
- `firstPoint` (Rhino.Geometry.Point3d) — The first corner used. Pass Point3d.Unset if you do not want to set this.
- `prompts` (System.Collections.Generic.IEnumerable<String>) — Optional prompts to display while getting points. May be null.
- `corners` (Rhino.Geometry.Point3d[]) — Corners of the rectangle in counter-clockwise order will be assigned to this out parameter during this call.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetRectangle_1.htm)

#### `public static Result GetRectangle(out Point3d[] corners)`

Gets a 3d rectangle.

**Parameters:**
- `corners` (Rhino.Geometry.Point3d[]) — corners of the rectangle in counter-clockwise order.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetRectangle.htm)

#### `public static Result GetRectangle(string firstPrompt, out Point3d[] corners)`

Gets a 3d rectangle.

**Parameters:**
- `firstPrompt` (System.String) — [Missing <param name="firstPrompt"/> documentation for "M:Rhino.Input.RhinoGet.GetRectangle(System.String,Rhino.Geometry.Point3d[]@)"]
- `corners` (Rhino.Geometry.Point3d[]) — corners of the rectangle in counter-clockwise order.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetRectangle_2.htm)

#### `public static Result GetRectangleWithCounts(int xMin, ref int xCount, int yMin, ref int yCount, out Point3d[] corners)`

Gets a 3d rectangle with prompts for counts in X and Y directions.

**Parameters:**
- `xMin` (System.Int32) — Minimum value allowed for count in the x direction.
- `xCount` (System.Int32) — Count in the x direction.
- `yMin` (System.Int32) — Minimum value allowed for count in the y direction.
- `yCount` (System.Int32) — Count in the y direction.
- `corners` (Rhino.Geometry.Point3d[]) — corners of the rectangle in counter-clockwise order.

**Returns:** `Result` — Commands.Result.Success if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetRectangleWithCounts.htm)

#### `public static Result GetSpiral(out NurbsCurve spiral)`



**Parameters:**
- `spiral` (Rhino.Geometry.NurbsCurve) — [Missing <param name="spiral"/> documentation for "M:Rhino.Input.RhinoGet.GetSpiral(Rhino.Geometry.NurbsCurve@)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.GetSpiral(Rhino.Geometry.NurbsCurve@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetSpiral.htm)

#### `public static Result GetString(string prompt, bool acceptNothing, ref string outputString)`

Easy to use string getter.

**Remarks:** If you need options or more advanced user interface, then use GetString class.

**Parameters:**
- `prompt` (System.String) — command prompt.
- `acceptNothing` (System.Boolean) — if true, the user can press enter.
- `outputString` (System.String) — default string set to this value and string value returned here.

**Returns:** `Result` — Commands.Result.Success - got string Commands.Result.Nothing - user pressed enter Commands.Result.Cancel - user cancel string getting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetString.htm)

#### `public static Result GetView(string commandPrompt, out RhinoView view)`

Allows the user to interactively pick a viewport.

**Parameters:**
- `commandPrompt` (System.String) — The command prompt during the request.
- `view` (Rhino.Display.RhinoView) — The view that the user picked. If the operation is successful, then this out parameter is assigned the correct view during this call.

**Returns:** `Result` — The result based on user choice.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_GetView.htm)

#### `public static bool InGet(RhinoDoc doc)`

Returns true if the document is current in a "Get" operation.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Input.RhinoGet.InGet(Rhino.RhinoDoc)"]

**Returns:** `Boolean` — true if a getter is currently active.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_InGet.htm)

#### `public static bool InGetObject(RhinoDoc doc)`

Returns true if currently in a GetObject.GetObjects()

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Input.RhinoGet.InGetObject(Rhino.RhinoDoc)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.InGetObject(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_InGetObject.htm)

#### `public static bool InGetPoint(RhinoDoc doc)`

Returns true if currently in a GetPoint.Get()

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Input.RhinoGet.InGetPoint(Rhino.RhinoDoc)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.RhinoGet.InGetPoint(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_InGetPoint.htm)

#### `public static string StringToCommandOptionName(string stringToConvert)`

Convert some arbitrary string value to a valid command option name removing any invalid characters.

**Parameters:**
- `stringToConvert` (System.String) — String to convert.

**Returns:** `String` — Returns null if the string is null or empty or if it contains nothing but invalid characters. If the converted string is one or more characters in length then the converted value is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_StringToCommandOptionName.htm)

#### `public static LocalizeStringPair StringToCommandOptionName(string englishString, string localizedString)`

Convert some arbitrary string value to a valid command option name removing any invalid characters.

**Parameters:**
- `englishString` (System.String) — English string to convert.
- `localizedString` (System.String) — Optional localized string to convert.

**Returns:** `LocalizeStringPair` — Returns null if the strings are null or empty or if they contain nothing but invalid characters. If the converted string is one or more characters in length then a LocalizeStringPair is returned characters the converted string values. If the localized string is null or empty then the English string is used as the localized value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_RhinoGet_StringToCommandOptionName_1.htm)

### Properties
- `AllBitmapFileTypes` (RhinoGet.BitmapFileTypes, get) — 

## RhinoGet.BitmapFileTypes (enum)

[Missing <summary> documentation for "T:Rhino.Input.RhinoGet.BitmapFileTypes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_RhinoGet_BitmapFileTypes.htm)

### Values
- `bmp` = `1`
- `jpg` = `2`
- `pcx` = `4`
- `png` = `8`
- `tif` = `16`
- `tga` = `32`

## StringParser (class)

Parse strings to numbers, distances and angles

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_StringParser.htm)

### Constructors
- `public StringParser()` — Initializes a new instance of the StringParser class

### Methods
#### `public static int ParseAngleExpession(string expression, int start_offset, int expression_length, StringParserSettings parse_settings_in, AngleUnitSystem output_angle_unit_system, out double value_out, ref StringParserSettings parse_results, ref AngleUnitSystem parsed_unit_system)`



**Parameters:**
- `expression` (System.String) — [Missing <param name="expression"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]
- `start_offset` (System.Int32) — [Missing <param name="start_offset"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]
- `expression_length` (System.Int32) — [Missing <param name="expression_length"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]
- `parse_settings_in` (Rhino.Input.StringParserSettings) — [Missing <param name="parse_settings_in"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]
- `output_angle_unit_system` (Rhino.AngleUnitSystem) — [Missing <param name="output_angle_unit_system"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]
- `value_out` (System.Double) — [Missing <param name="value_out"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]
- `parse_results` (Rhino.Input.StringParserSettings) — [Missing <param name="parse_results"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]
- `parsed_unit_system` (Rhino.AngleUnitSystem) — [Missing <param name="parsed_unit_system"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Input.StringParser.ParseAngleExpession(System.String,System.Int32,System.Int32,Rhino.Input.StringParserSettings,Rhino.AngleUnitSystem,System.Double@,Rhino.Input.StringParserSettings@,Rhino.AngleUnitSystem@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParser_ParseAngleExpession.htm)

#### `public static bool ParseAngleExpressionDegrees(string expression, out double angle_degrees)`



**Parameters:**
- `expression` (System.String) — [Missing <param name="expression"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpressionDegrees(System.String,System.Double@)"]
- `angle_degrees` (System.Double) — [Missing <param name="angle_degrees"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpressionDegrees(System.String,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.StringParser.ParseAngleExpressionDegrees(System.String,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParser_ParseAngleExpressionDegrees.htm)

#### `public static bool ParseAngleExpressionRadians(string expression, out double angle_radians)`



**Parameters:**
- `expression` (System.String) — [Missing <param name="expression"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpressionRadians(System.String,System.Double@)"]
- `angle_radians` (System.Double) — [Missing <param name="angle_radians"/> documentation for "M:Rhino.Input.StringParser.ParseAngleExpressionRadians(System.String,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Input.StringParser.ParseAngleExpressionRadians(System.String,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParser_ParseAngleExpressionRadians.htm)

#### `public static int ParseLengthExpession(string expression, int start_offset, int expression_length, StringParserSettings parse_settings_in, UnitSystem output_unit_system, out double value_out, ref StringParserSettings parse_results, ref UnitSystem parsed_unit_system)`

Parse a string for a length value. Expression can include complex expressions Most complex version of length parsing

**Parameters:**
- `expression` (System.String) — [In] The string to parse
- `start_offset` (System.Int32) — [In] Offset position in string to start parsing
- `expression_length` (System.Int32) — [In] Maximum length of string to parse. -1 means parse to a terminating character or end of string
- `parse_settings_in` (Rhino.Input.StringParserSettings) — [In] Determines what input will be parsed
- `output_unit_system` (Rhino.UnitSystem) — [In] Output value is returned in this unit system
- `value_out` (System.Double) — [Out] The length value result
- `parse_results` (Rhino.Input.StringParserSettings) — [Out] Describes the results of the parse operation
- `parsed_unit_system` (Rhino.UnitSystem) — [Out] If a unit system name was found in the string, it is returned here. The output value is in the unit system specified in output_unit_system

**Returns:** `Int32` — Returns the count of characters that were parsed or 0 if the operation was unsuccessful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParser_ParseLengthExpession_1.htm)

#### `public static int ParseLengthExpession(string expression, StringParserSettings parse_settings_in, UnitSystem output_unit_system, out double value_out)`

Parse a string for a length value. Expression can include complex expressions Simplest version of Length parsing

**Parameters:**
- `expression` (System.String) — [In] The string to parse
- `parse_settings_in` (Rhino.Input.StringParserSettings) — [In] Determines what input will be parsed
- `output_unit_system` (Rhino.UnitSystem) — [In] Output value is in this unit system
- `value_out` (System.Double) — [Out] The length value result

**Returns:** `Int32` — Count of characters parsed or 0 for failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParser_ParseLengthExpession.htm)

#### `public static int ParseNumber(string expression, int max_count, StringParserSettings settings_in, ref StringParserSettings settings_out, out double answer)`

Parse a string expression to get a number

**Parameters:**
- `expression` (System.String) — String to parse
- `max_count` (System.Int32) — Maximum number of characters to parse
- `settings_in` (Rhino.Input.StringParserSettings) — Determines what input will be parsed
- `settings_out` (Rhino.Input.StringParserSettings) — Reports the results of the parse operation
- `answer` (System.Double) — The number result of the parse operation

**Returns:** `Int32` — Count of characters in expression parsed if ParseNumber() returns 0, parse was unsuccessful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParser_ParseNumber.htm)

## StringParserSettings (class)

Parameters for parsing strings

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Input_StringParserSettings.htm)

### Constructors
- `public StringParserSettings()` — Initializes a new instance of the StringParserSettings class

### Methods
#### `public void Dispose()`

Releases all resources used by the StringParserSettings

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParserSettings_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the StringParserSettings and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParserSettings_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParserSettings_Finalize.htm)

#### `public void SetAllExpressionSettingsToFalse()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParserSettings_SetAllExpressionSettingsToFalse.htm)

#### `public void SetAllFieldsToFalse()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Input_StringParserSettings_SetAllFieldsToFalse.htm)

### Properties
- `DefaultAngleUnitSystem` (AngleUnitSystem, get/set) — 
- `DefaultLengthUnitSystem` (UnitSystem, get/set) — 
- `DefaultParseSettings` (StringParserSettings, get) — - The default settings parse just about everything in a reasonable way. - Any angle values with unspecified units will be treated as radians. Angles without units can be encountered while parsing formulas, lengths and points and need to be thoughtfully considered in most parsing situations.
- `ParseAddition` (Boolean, get/set) — 
- `ParseArcDegreesMinutesSeconds` (Boolean, get/set) — 
- `ParseArithmeticExpression` (Boolean, get/set) — 
- `ParseCommaAsDecimalPoint` (Boolean, get/set) — 
- `ParseCommaAsDigitSeparator` (Boolean, get/set) — 
- `ParseDAsExponentInScientificENotation` (Boolean, get/set) — 
- `ParseDivision` (Boolean, get/set) — 
- `ParseExplicitFormulaExpression` (Boolean, get/set) — 
- `ParseFeetInches` (Boolean, get/set) — 
- `ParseFullStopAsDecimalPoint` (Boolean, get/set) — 
- `ParseFullStopAsDigitSeparator` (Boolean, get/set) — 
- `ParseHyphenAsNumberDash` (Boolean, get/set) — 
- `ParseHyphenMinusAsNumberDash` (Boolean, get/set) — 
- `ParseIntegerDashFraction` (Boolean, get/set) — 
- `ParseLeadingWhiteSpace` (Boolean, get/set) — 
- `ParseMathFunctions` (Boolean, get/set) — 
- `ParseMultiplication` (Boolean, get/set) — 
- `ParsePairedParentheses` (Boolean, get/set) — 
- `ParsePi` (Boolean, get/set) — 
- `ParseRationalNumber` (Boolean, get/set) — 
- `ParseScientificENotation` (Boolean, get/set) — 
- `ParseSettingsDegrees` (StringParserSettings, get) — - The default settings parse just about everything in a reasonable way. - Any angle values with unspecified units will be treated as degrees.Angles without units can be encountered while parsing formulas, lengths and points and need to be thoughtfully considered in most parsing situations.
- `ParseSettingsDoubleNumber` (StringParserSettings, get) — - The double number settings parse and optional unary + or unary - and then parse a number that can be integer, decimal, or scientific e notation.
- `ParseSettingsEmpty` (StringParserSettings, get) — - ON_ParseSetting::FalseSettings has all parsing options set to false. - A common use of ON_ParseSettings FalseSettings is to initialize ON_ParseSettings classes that are used to report what happened during parsing.Any parsing results value set to true after parsing indicates that type of parsing occurred.
- `ParseSettingsIntegerNumber` (StringParserSettings, get) — - The integer settings parse and optional unary + or unary - and then parses one or more digits.Parsing stops after the last digit.
- `ParseSettingsRadians` (StringParserSettings, get) — - The default settings parse just about everything in a reasonable way. - Any angle values with unspecified units will be treated as radians.Angles without units can be encountered while parsing formulas, lengths and points and need to be thoughtfully considered in most parsing situations.
- `ParseSettingsRationalNumber` (StringParserSettings, get) — - The rational number settings parse and optional unary + or unary - and then parse one or more digits.If a rational number fraction bar follows the last digit in the numerator, then it is parsed and an integer denominator is parsed.The denominator cannot have a unary + or - preceding the digits.Parsing stops after the last digit in the denominator.
- `ParseSettingsRealNumber` (StringParserSettings, get) — - The real number settings parse and optional unary + or unary - and then parse a number that can be integer, decimal, scientific e notation or pi.
- `ParseSignificandDigitSeparators` (Boolean, get/set) — 
- `ParseSignificandFractionalPart` (Boolean, get/set) — 
- `ParseSignificandIntegerPart` (Boolean, get/set) — 
- `ParseSpaceAsDigitSeparator` (Boolean, get/set) — 
- `ParseSubtraction` (Boolean, get/set) — 
- `ParseSurveyorsNotation` (Boolean, get/set) — 
- `ParseUnaryMinus` (Boolean, get/set) — 
- `ParseUnaryPlus` (Boolean, get/set) — 
- `PreferedLocaleId` (UInt32, get/set) — 

