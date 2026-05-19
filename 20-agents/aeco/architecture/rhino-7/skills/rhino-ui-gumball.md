---
name: rhino-rhino-ui-gumball
description: This skill encodes the rhino 7.0 surface of the Rhino.UI.Gumball namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GumballDisplayConduit, GumballAppearanceSettings, GumballObject, GumballPickResult, GumballFrame, GumballMode, GumballScaleMode.
---

# Rhino.UI.Gumball

Auto-generated from vendor docs for rhino 7.0. 7 types in this namespace.

## GumballAppearanceSettings (class)

[Missing <summary> documentation for "T:Rhino.UI.Gumball.GumballAppearanceSettings"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Gumball_GumballAppearanceSettings.htm)

### Constructors
- `public GumballAppearanceSettings()` — Initializes a new instance of the GumballAppearanceSettings class

### Properties
- `ArcThickness` (Int32, get/set) — in pixels.
- `ArrowHeadLength` (Int32, get/set) — in pixels.
- `ArrowHeadWidth` (Int32, get/set) — in pixels.
- `AxisThickness` (Int32, get/set) — in pixels.
- `ColorMenuButton` (Color, get/set) — 
- `ColorX` (Color, get/set) — Default is Red.
- `ColorY` (Color, get/set) — Default is Green.
- `ColorZ` (Color, get/set) — Default is Blue.
- `FreeTranslate` (Int32, get/set) — When FreeTranslate is 1, the center translation control can be dragged in any direction and moves the object the gumball controls. When FreeTranslate is 2, the center translation control can be dragged in any direction and moves the object the gumball itself. The default value is 2.
- `MenuDistance` (Int32, get/set) — Distance of menu ball from center.
- `MenuEnabled` (Boolean, get/set) — When MenuEnabled is true, the menu "button" is drawn on the gumball. The default setting is true.
- `MenuSize` (Int32, get/set) — Radius of menu circle.
- `PlanarTranslationGripCorner` (Int32, get/set) — in pixels.
- `PlanarTranslationGripSize` (Int32, get/set) — in pixels.
- `Radius` (Int32, get/set) — in pixels.
- `RelocateEnabled` (Boolean, get/set) — When RelocateEnabled is true, the user can reposition the gumball by tapping the control key while dragging. Once the repostion drag is terminated by releasing the/ mouse button, ordinary editing resumes. The default setting is true.
- `RotateXEnabled` (Boolean, get/set) — When RotateX is true, the X rotation control is available. The default setting is true.
- `RotateYEnabled` (Boolean, get/set) — When RotateY is true, the Y rotation control is available. The default setting is true.
- `RotateZEnabled` (Boolean, get/set) — When RotateZ is true, the Z rotation control is available. The default setting is true.
- `ScaleGripSize` (Int32, get/set) — in pixels.
- `ScaleXEnabled` (Boolean, get/set) — When ScaleXEnabled is true, the X scale control is available. The default setting is true.
- `ScaleYEnabled` (Boolean, get/set) — When ScaleYEnabled is true, the Y scale control is available. The default setting is true.
- `ScaleZEnabled` (Boolean, get/set) — When ScaleZEnabled is true, the Z scale control is available. The default setting is true.
- `TranslateXEnabled` (Boolean, get/set) — TranslateXEnabled is true, the X axis translation control is available. The default setting is true.
- `TranslateXYEnabled` (Boolean, get/set) — When TranslateXY is true, the XY plane translation control is available in appropriate views. The default setting is true.
- `TranslateYEnabled` (Boolean, get/set) — TranslateYEnabled is true, the Y axis translation control is available. The default setting is true.
- `TranslateYZEnabled` (Boolean, get/set) — When TranslateYZ is true, the YZ plane translation control is available in appropriate views. The default setting is true.
- `TranslateZEnabled` (Boolean, get/set) — TranslateZEnabled is true, the Z axis translation control is available. The default setting is true.
- `TranslateZXEnabled` (Boolean, get/set) — When TranslateZX is true, the ZX plane translation control is available in appropriate views. The default setting is true.

## GumballDisplayConduit (class)

[Missing <summary> documentation for "T:Rhino.UI.Gumball.GumballDisplayConduit"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Gumball_GumballDisplayConduit.htm)

### Constructors
- `public GumballDisplayConduit()` — Initializes a new instance of the GumballDisplayConduit class

### Methods
#### `public void CheckShiftAndControlKeys()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_CheckShiftAndControlKeys.htm)

#### `public void Dispose()`

Releases all resources used by the GumballDisplayConduit

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the GumballDisplayConduit and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_Finalize.htm)

#### `public bool PickGumball(PickContext pickContext, GetPoint getPoint)`



**Parameters:**
- `pickContext` (Rhino.Input.Custom.PickContext) — [Missing <param name="pickContext"/> documentation for "M:Rhino.UI.Gumball.GumballDisplayConduit.PickGumball(Rhino.Input.Custom.PickContext,Rhino.Input.Custom.GetPoint)"]
- `getPoint` (Rhino.Input.Custom.GetPoint) — [Missing <param name="getPoint"/> documentation for "M:Rhino.UI.Gumball.GumballDisplayConduit.PickGumball(Rhino.Input.Custom.PickContext,Rhino.Input.Custom.GetPoint)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballDisplayConduit.PickGumball(Rhino.Input.Custom.PickContext,Rhino.Input.Custom.GetPoint)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_PickGumball.htm)

#### `public void SetBaseGumball(GumballObject gumball)`

Contents of the gumball are copied to the base gumball of this class.

**Parameters:**
- `gumball` (Rhino.UI.Gumball.GumballObject) — The gumball source.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_SetBaseGumball.htm)

#### `public void SetBaseGumball(GumballObject gumball, GumballAppearanceSettings appearanceSettings)`

Contents of the gumball are copied to the base gumball of this class.

**Parameters:**
- `gumball` (Rhino.UI.Gumball.GumballObject) — The gumball source.
- `appearanceSettings` (Rhino.UI.Gumball.GumballAppearanceSettings) — The gumball appearance and behavior settings. Argument can be null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_SetBaseGumball_1.htm)

#### `public bool UpdateGumball(Point3d point, Line worldLine)`



**Parameters:**
- `point` (Rhino.Geometry.Point3d) — [Missing <param name="point"/> documentation for "M:Rhino.UI.Gumball.GumballDisplayConduit.UpdateGumball(Rhino.Geometry.Point3d,Rhino.Geometry.Line)"]
- `worldLine` (Rhino.Geometry.Line) — [Missing <param name="worldLine"/> documentation for "M:Rhino.UI.Gumball.GumballDisplayConduit.UpdateGumball(Rhino.Geometry.Point3d,Rhino.Geometry.Line)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballDisplayConduit.UpdateGumball(Rhino.Geometry.Point3d,Rhino.Geometry.Line)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballDisplayConduit_UpdateGumball.htm)

### Properties
- `BaseGumball` (GumballObject, get) — Starting location.
- `Enabled` (Boolean, get/set) — 
- `Gumball` (GumballObject, get) — 
- `GumballTransform` (Transform, get) — The gumball transformation is the transformation calculated by comparing the current gumball to the starting BaseGumball.
- `InRelocate` (Boolean, get) — 
- `PickResult` (GumballPickResult, get) — The inital mouse down event sets PickResult.
- `PreTransform` (Transform, get/set) — The pre-transform is a transformation that needs to be applied before the gumball transformation.
- `TotalTransform` (Transform, get) — The total transformation is GumballTransform * PreTransform.

## GumballFrame (struct)

[Missing <summary> documentation for "T:Rhino.UI.Gumball.GumballFrame"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Gumball_GumballFrame.htm)

### Properties
- `Plane` (Plane, get/set) — 
- `ScaleGripDistance` (Vector3d, get/set) — 
- `ScaleMode` (GumballScaleMode, get/set) — 

## GumballMode (enum)

Transformation modes for gumballs.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Gumball_GumballMode.htm)

### Values
- `None` = `0`
- `Menu` = `1` — Gumball menu button was picked.
- `TranslateFree` = `2` — Unconstrained translation.
- `TranslateX` = `3` — Translation along a single axis.
- `TranslateY` = `4` — Translation along a single axis.
- `TranslateZ` = `5` — Translation along a single axis.
- `TranslateXY` = `6` — Translation in a plane.
- `TranslateYZ` = `7` — Translation in a plane.
- `TranslateZX` = `8` — Translation in a plane.
- `ScaleX` = `9` — Scale along a single axis.
- `ScaleY` = `10` — Scale along a single axis.
- `ScaleZ` = `11` — Scale along a single axis.
- `ScaleXY` = `12` — Scale in a plane.
- `ScaleYZ` = `13` — Scale in a plane.
- `ScaleZX` = `14` — Scale in a plane.
- `RotateX` = `15` — Rotation around a single axis.
- `RotateY` = `16` — Rotation around a single axis.
- `RotateZ` = `17` — Rotation around a single axis.
- `ExtrudeX` = `18` — Extrusion along a single axis.
- `ExtrudeY` = `19` — Extrusion along a single axis.
- `ExtrudeZ` = `20` — Extrusion along a single axis.

## GumballObject (class)

[Missing <summary> documentation for "T:Rhino.UI.Gumball.GumballObject"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Gumball_GumballObject.htm)

### Constructors
- `public GumballObject()` — Initializes a new instance of the GumballObject class

### Methods
#### `public void Dispose()`

Releases all resources used by the GumballObject

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the GumballObject and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_Finalize.htm)

#### `public bool SetFromArc(Arc arc)`



**Parameters:**
- `arc` (Rhino.Geometry.Arc) — [Missing <param name="arc"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromArc(Rhino.Geometry.Arc)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromArc(Rhino.Geometry.Arc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromArc.htm)

#### `public bool SetFromBoundingBox(BoundingBox boundingBox)`



**Parameters:**
- `boundingBox` (Rhino.Geometry.BoundingBox) — [Missing <param name="boundingBox"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromBoundingBox(Rhino.Geometry.BoundingBox)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromBoundingBox(Rhino.Geometry.BoundingBox)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromBoundingBox.htm)

#### `public bool SetFromBoundingBox(Plane frame, BoundingBox frameBoundingBox)`

Sets the gumball bounding box with respect to a frame.

**Parameters:**
- `frame` (Rhino.Geometry.Plane) — The frame.
- `frameBoundingBox` (Rhino.Geometry.BoundingBox) — Bounding box with respect to frame.

**Returns:** `Boolean` — true if input is valid and gumball is set. false if input is not valid. In this case, gumball is set to the default.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromBoundingBox_1.htm)

#### `public bool SetFromCircle(Circle circle)`



**Parameters:**
- `circle` (Rhino.Geometry.Circle) — [Missing <param name="circle"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromCircle(Rhino.Geometry.Circle)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromCircle(Rhino.Geometry.Circle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromCircle.htm)

#### `public bool SetFromCurve(Curve curve)`



**Parameters:**
- `curve` (Rhino.Geometry.Curve) — [Missing <param name="curve"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromCurve(Rhino.Geometry.Curve)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromCurve(Rhino.Geometry.Curve)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromCurve.htm)

#### `public bool SetFromEllipse(Ellipse ellipse)`



**Parameters:**
- `ellipse` (Rhino.Geometry.Ellipse) — [Missing <param name="ellipse"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromEllipse(Rhino.Geometry.Ellipse)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromEllipse(Rhino.Geometry.Ellipse)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromEllipse.htm)

#### `public bool SetFromExtrusion(Extrusion extrusion)`



**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — [Missing <param name="extrusion"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromExtrusion(Rhino.Geometry.Extrusion)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromExtrusion(Rhino.Geometry.Extrusion)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromExtrusion.htm)

#### `public bool SetFromHatch(Hatch hatch)`



**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromHatch(Rhino.Geometry.Hatch)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromHatch(Rhino.Geometry.Hatch)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromHatch.htm)

#### `public bool SetFromLight(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromLight(Rhino.Geometry.Light)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromLight(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromLight.htm)

#### `public bool SetFromLine(Line line)`



**Parameters:**
- `line` (Rhino.Geometry.Line) — [Missing <param name="line"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromLine(Rhino.Geometry.Line)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromLine(Rhino.Geometry.Line)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromLine.htm)

#### `public bool SetFromPlane(Plane plane)`



**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromPlane(Rhino.Geometry.Plane)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Gumball.GumballObject.SetFromPlane(Rhino.Geometry.Plane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballObject_SetFromPlane.htm)

### Properties
- `Frame` (GumballFrame, get/set) — 

## GumballPickResult (class)

[Missing <summary> documentation for "T:Rhino.UI.Gumball.GumballPickResult"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Gumball_GumballPickResult.htm)

### Methods
#### `public void SetToDefault()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Gumball_GumballPickResult_SetToDefault.htm)

### Properties
- `Mode` (GumballMode, get) — 

## GumballScaleMode (enum)

[Missing <summary> documentation for "T:Rhino.UI.Gumball.GumballScaleMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Gumball_GumballScaleMode.htm)

### Values
- `Independent` = `0`
- `XY` = `1`
- `YZ` = `2`
- `ZX` = `3`
- `XYZ` = `4`

